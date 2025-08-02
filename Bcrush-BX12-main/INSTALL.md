````markdown
# Installation

To build and run **Bcrush + BX12**, follow these steps:

## Prerequisites

- **Rust** (latest stable version recommended)  
  Install from: https://rustup.rs

- **Python 3.10+** (for auxiliary tools or testing, if needed)  
  Install from: https://www.python.org/

## Build Steps

1. Clone the repository:

```bash
git clone https://github.com/8BiTentertaiment/bx12.git
cd bx12
````

2. Build the project using Cargo:

```bash
cargo build --release
```

The compiled binaries will appear in the `target/release/` directory.

## Run

Once built, run the executable:

```bash
./target/release/bcrush
```

Or, if BX12 wrapper is ready:

```bash
./target/release/bx12
```

## Notes

* ASCII rendering is terminal-based. Use a console that supports UTF-8 and ANSI colors.
* Windows users: Prefer **Windows Terminal** or **cmder**.
* Linux/macOS: any modern terminal should work.
* This project is under active development. Features may change.
