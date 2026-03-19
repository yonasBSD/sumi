
  # sumi 🌸

  I created sumi because I was tired of scrolling through messy terminal logs to find errors. I wanted a way to see my process output and errors separately, in a clean, dark-mode web dashboard—and I thought
  that would be pretty kul.

  It's a high-performance process monitor built with Rust that makes debugging feel like using a modern IDE. Hope you find it useful too! 🚀

  ![sumi preview](https://github.com/AslanLM/sumi/releases/download/v0.1.0/preview.gif)

  ---

  ## 🧩 Features

   * Side-by-Side View: Separate columns for stdout and stderr.
   * Real-time Streaming: Powered by WebSockets for zero-latency updates.
   * Minimalist Dark UI: A clean, focused interface for long debugging sessions.
   * Self-Contained: The entire web UI is embedded into a single, portable binary.
   * Zero Config: Works with any command out of the box.

  ---

  ## 🚀 Installation

> **From Source (Recommended)
  You need to have `Rust` (https://www.rust-lang.org/tools/install) installed on your system.**

   ### Clone the repository
   ```bash
   git clone https://github.com/AslanLM/sumi.git
   cd sumi
   ```
   
  ### Install it globally
  ```bash
   cargo install --path .
  ```

  This will install the sumi binary into your ~/.cargo/bin directory.

  ---

  ## 🎬 Usage

  1. Basic command monitoring:
  Just wrap any command with sumi:

    sumi npm run dev

  2. Monitoring a script:

    sumi ./my-script.sh

  3. Open the Dashboard:
  Once running, open your browser at:
   http://localhost:8080

  ---

  ## ⚙️ How it works

  sumi acts as a wrapper for your process. It intercepts every line of output and broadcasts it through a WebSocket server.

   * Backend: Built with Axum (https://github.com/tokio-rs/axum) and Tokio (https://tokio.rs/).
   * Frontend: Minimalist Vanilla JS and CSS Grid (embedded in the binary).
