# 🦀 **Jiggler Rustle**

A tiny 🧰 Rust-powered companion that gently reminds your machine you're still around 👀. Perfect for those deep focus sessions where stepping away is not really an option... or at least, that's what the system thinks 😏.

Cross-platform. Lightweight. Practically invisible. Just a little... rustle now and then. 🌬️🖱️

---

## 📦 Installation

### From crates.io (Recommended)

```bash
cargo install jiggler
```

After installation, you can run `jiggler` from anywhere in your terminal!

### From Source

```bash
git clone https://github.com/andrese03/jiggler_rustle.git
cd jiggler_rustle
cargo install --path jiggler-cli
```

## 🚀 Usage

Once installed, you can use `jiggler` like any other command-line tool:

```bash
# Basic usage - jiggles mouse every 60 seconds
jiggler

# Custom interval (every 30 seconds)
jiggler --interval 30

# Use key press instead of mouse movement
jiggler --key-mode

# Run once and exit
jiggler --once

# Show help
jiggler --help
```

### Command Options

- `-i, --interval <SECONDS>`: Set interval between actions (default: 60)
- `-k, --key-mode`: Use key press instead of mouse movement
- `-o, --once`: Run once and exit
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## 📦 Project Structure

- **jiggler-api/** – The core library with all the jiggly logic
- **jiggler-cli/** – Command-line tool to make your system wiggle (published as `jiggler`)

## ✨ Features

- Keeps your computer awake by simulating mouse or keyboard activity
- Works everywhere (thanks to [enigo](https://crates.io/crates/enigo))
- Async magic with [tokio](https://crates.io/crates/tokio)
- So light, you'll forget it's running
- Cross-platform: Windows, macOS, Linux
- Safe key presses that won't interfere with your work

## 📋 Examples

```bash
# Keep system awake with mouse movements every 2 minutes
jiggler --interval 120

# Use keyboard mode for systems where mouse movement isn't ideal
jiggler --key-mode --interval 45

# Test the tool with a single action
jiggler --once --key-mode
```

## 🛠️ Development

- Rust 1.56+ (edition 2021)
- Clone, hack, and `cargo build` to your heart's content

### Library Usage

Want to add a little rustle to your own project? Just add this to your `Cargo.toml`:

```toml
[dependencies]
jiggler-api = "0.1.0"
```

## 📜 License

This project is licensed under the MIT License — free for personal, open source, and commercial use (with attribution).

See the LICENSE file for full terms.

## 🌐 Repository

[github.com/andrese03/jiggler_rustle](https://github.com/andrese03/jiggler_rustle)
