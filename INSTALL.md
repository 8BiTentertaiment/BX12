# Installation Guide

## Requirements

Before installing **Bcrush + BX12**, make sure you have the following installed:

- **Rust (Stable)**  
  Install via official script:  
  ```bash
  curl https://sh.rustup.rs -sSf | sh
  rustup default stable
Cargo (comes with Rust)

A Unix-like shell (e.g. Bash, Zsh, or Git Bash on Windows)

Windows users: You can use WSL, Git Bash, or a terminal in VS Code.

Build Instructions
Clone the repository:

bash
Копировать
Редактировать
git clone https://github.com/yourusername/bcrush.git
cd bcrush
Build the project:

bash
Копировать
Редактировать
cargo build --release
Run the project:

bash
Копировать
Редактировать
cargo run
Optional (for BX12 compatibility layer)
BX12 integration is under development.

In the future, you’ll be able to compile and use the DX12 wrapper via:

bash
Копировать
Редактировать
cargo build -p bx12_wrapper
Notes
This is a console-rendered graphical engine using ASCII output.

Performance and visual accuracy may depend on terminal configuration.

Contributions are welcome — see CONTRIBUTING.md (optional file).

© 2025 8BiT_entertainment
Licensed under MPL-2.0
