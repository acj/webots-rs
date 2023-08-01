# Webots controllers in Rust

**Status**: experimental

This is a reference project that shows how to build controllers for the [Webots robot simulator](https://cyberbotics.com) using the Rust programming language.

## Getting Started

1. Download and install [Webots](https://cyberbotics.com) for your operating system
1. Install [Rust](https://www.rust-lang.org/learn/get-started) if you haven't already
1. Clone this repository
1. Run `make` to compile the Rust controller and copy it into place
1. Open the `sample_project/worlds/my_first_simulation.wbt` file in Webots
1. Run the simulation

You should see "The Rust controller has started" in the Webots console.

To make changes to the controller, you can edit `src/main.rs` and then run `make` again. You might need to reset the simulation (File > Reset Simulation) or restart Webots to use the updated code.

## How this works

At compile time, I use [bindgen](https://github.com/rust-lang/rust-bindgen) to convert a list of Webots C header files (see `wrapper.h`) into Rust structures and types. Those types form a bridge between the Rust-based controller code and the Webots C library that does the hard work of interacting with the simulation engine. See `build.rs` for more details.

## Contributing

Improvements are welcome. If you have an idea, please open an issue so that we can discuss it.

## TODO

- Improve API safety. While any of the Webots C functions can be called from Rust, many are marked as `unsafe` due to raw pointer usage. I have started wrapping a few of them in Rust functions (see `lib.rs`), but this approach doesn't scale very well given the size of the API.
- More example controllers
- Resolve "not FFI-safe" warnings somehow

## License

MIT
