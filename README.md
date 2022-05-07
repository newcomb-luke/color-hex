# color-hex
A small Rust crate that supplies procedural macros to convert hex strings into RGB/A colors at compile time.

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/newcomb-luke/color-hex/Rust%20CI)

![Crates.io](https://img.shields.io/crates/v/color-hex)

## Documentation

[Documentation with examples](https://docs.rs/color-hex) The documentation includes a comprehensive description of the syntax supported for parsing hex colors.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
color_hex = "0.2.0"
```

Here is an example of converting a direct "HTML style" hex color string to an array:

```Rust
use color_hex::color_from_hex;

fn main() {
    let color = color_from_hex!("#2d2d2d");

    println!("Color: {:x?}", color);
}
```

## License

Licensed under the [MIT license](http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
