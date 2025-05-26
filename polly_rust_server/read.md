# Polly Rust Server

**Production-ready Rust server for Amazon Polly TTS integration.**  
Built for both local and cloud deployment — zero compilation errors, no warnings.  
This binary is a diamond: stable, fast, and already used in production.

---

## 🚀 Features

- Converts plain text or SSML to **neural Amazon Polly voices**
- Returns audio in **WAV format**
- Accepts POST requests with dynamic voice switching, pitch, and language
- Modular `.env` file for flexible configuration
- Supports **SSML** input (refer to official Amazon Polly documentation)
- Designed specifically for **neural voice output**, not standard voices
- Production-stable: **zero warnings, clean build**

---

## 📦 Included

- `main.rs` – the core of the Polly TTS server
- `Cargo.toml` – dependencies and build config
- `.env` – defines keys, port, default voice, and region

---

## 🛠️ Setup

1. Clone or download the repository
2. Open your terminal and navigate to the folder where `src` exists
3. Open the `.env` file and set your values:
    - `AWS_ACCESS_KEY_ID`
    - `AWS_SECRET_ACCESS_KEY`
    - `AWS_REGION`
    - `PORT` (e.g. `5000`)
    - `DEFAULT_VOICE` (e.g. `Ruth`)
4. Build with:
    ```
    cargo build --release
    ```
5. After compilation:
    ```
    cargo run --release
    ```

---

## ❤️ Support This Project

If this server helped you or saved you time, you can support development here:  
**[Donate via Stripe](https://buy.stripe.com/cNi5kF3VJ3CVfF5d4X5Rm04)**

---




