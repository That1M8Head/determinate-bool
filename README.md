# Determinate - `bool` with superpowers

## Overview

This is a Rust library to provide a type, `Determinate<T>`, which is like bool,
but with additional features. It provides a simple enum, Determinate, with
variants for True, False, and Indeterminate, allowing you to express conditions
with or without associated data.

## Features

* **Simple Representation:** Easily express true, false, or indeterminate
conditions in your Rust code.
* **Flexible Usage:** Use associated data with the True variant or represent
false or indeterminate conditions without the need for additional information.
* **Display Formatting:** The std::fmt::Display trait has been implemented for
convenient printing and debugging.

## Installation

To make things easy, navigate to your project and run:

```bash
cargo add determinate-bool
```

This will automagically add the Determinate library to your project.

## Example

```rust
use determinate_bool::Determinate;

fn main() {
    let result_default = Determinate::new();
    println!("Default: {}", result_default);

    let result_true = Determinate::from(true);
    println!("True: {}", result_true);

    let result_false = Determinate::from(false);
    println!("False: {}", result_false);
    
    let explicit_determinate = Determinate::True("This is specific!");
    println!("Explicit determinate: {}", explicit_determinate);

    let explicit_indeterminate = Determinate::<()>::Indeterminate;
    println!("Explicit indeterminate: {}", explicit_indeterminate);

    println!("{}", explicit_determinate.unwrap());
}
```

## API Reference

* `Determinate::new()` - Create a new `Determinate::Indeterminate`.
* `Determinate::from(from_bool)` - Create a `Determinate` instance from a
`bool`. If `from_bool` is true, the associated value is set to 0.

## License

GNU GPL v3.0 or later.
