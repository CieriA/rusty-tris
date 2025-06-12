# Rust Tris with SDL2
A Tris clone written in Rust using the SDL2 graphics library.

## Features
- Pass-to-play game with 2 players
- GUI with SDL2

## Requirements
- [**Rust**](#rust-installation) (stable)
- [**SDL2**](#sdl2-and-sdl2_ttf-installation) — graphics and input
- [**SDL2_ttf**](#sdl2-and-sdl2_ttf-installation) — texts

### Rust installation
Install Rust via [rustup](https://rustup.rs)

### SDL2 and SDL2_ttf installation

#### Windows
Use [vcpkg](https://github.com/microsoft/vcpkg) or manually install SDL2 and SDL2_ttf developer packages.
Make sure the .ddl files are in your `PATH` or project folder at runtime.

#### macOS (using Homebrew)
```bash
brew install sdl2 sdl2_ttd
```

#### Linux
```bash
sudo apt install libsdl2-dev libsdl2-ttf-dev
```

## Building and running the project
Clone the repository and build in release mode:
```bash
git clone https://github.com/CieriA/rusty-tris
cd rusty-tetris
cargo build --release
```

To run the game:
```bash
cargo run --release
```

## Game controls
- Left mouse click — place the piece

## Development notes
This project didn't need to be a `workspace` because of its simplicity.

This project uses the following crates:
- sdl2

TO regenerate documentation locally:
```bash
cargo doc --open
```

## License
This project is licensed under the ISC License. For more info see the [LICENSE](LICENSE) file.
