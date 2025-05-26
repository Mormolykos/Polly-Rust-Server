use axum::{
    extract::{Json, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
    body::Bytes,
};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_polly::{
    Client as PollyClient,
    types::{Engine, OutputFormat, TextType, VoiceId},
};
use dotenv::dotenv;
use serde::Deserialize;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct SpeakPayload {
    text: String,
    #[serde(default)]
    ssml: bool,
    #[serde(default)]
    voice: Option<String>,
}

#[derive(Clone)]
struct AppState {
    polly: PollyClient,
    default_voice: VoiceId,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    if let Ok(r) = env::var("AWS_POLLY_REGION") {
        env::set_var("AWS_REGION", &r);
    }

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_conf = aws_config::from_env().region(region_provider).load().await;
    let polly = PollyClient::new(&shared_conf);

    let default_voice = env::var("POLLY_VOICE_ID").unwrap_or_else(|_| "Ruth".into());
    let default_voice = VoiceId::from(default_voice.as_str());

    let state = AppState { polly, default_voice };
    let app = Router::new()
        .route("/speak", post(handle_speak))
        .nest_service("/", ServeDir::new("."))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "5003".into());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    println!("🚀 Polly TTS Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await.expect("bind failed");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_speak(
    State(state): State<AppState>,
    Json(payload): Json<SpeakPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 1) Build the Polly request
    let mut req = state
        .polly
        .synthesize_speech()
        .engine(Engine::Neural)
        .output_format(OutputFormat::Mp3)  // switched to MP3
        .voice_id(payload.voice.as_deref().map(VoiceId::from).unwrap_or(state.default_voice.clone()))
        .text(payload.text.clone());

    if payload.ssml {
        req = req.text_type(TextType::Ssml);
    } else {
        req = req.text_type(TextType::Text);
    }

    // 2) Send to Polly
    let resp = req.send().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Polly error: {}", e)))?;

    // 3) Collect the MP3 stream
    let aggregated = resp.audio_stream.collect().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Stream error: {}", e)))?;
    let mp3_bytes = aggregated.into_bytes();

    // 4) Return as HTTP response
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("audio/mpeg"));
    Ok((StatusCode::OK, headers, Bytes::from(mp3_bytes)))
}
