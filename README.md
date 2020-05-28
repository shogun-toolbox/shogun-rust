# shogun-rust

This is a Rust crate with bindings to the [Shogun](https://github.com/shogun-toolbox/shogun) machine learning framework.

Note: this crate is in very early development and only supports a very limited part of the Shogun library.

More information about the design can be found [here](https://gf712.github.io/programming/2020/05/28/shogun-rust.html).

# Example
```rust
use shogun_rust::shogun::{Kernel, Version};

fn main() {
    let version = Version::new();
    println!("Shogun version {}", version.main_version().unwrap());

    // shogun-rust supports Shogun's factory functions
    let k = match Kernel::new("GaussianKernel") {
        Ok(obj) => obj,
        Err(msg) => {
            panic!("No can do: {}", msg);
        },
    };

    // also supports put
    match k.put("log_width", &1.0) {
        Some(msg) => println!("Failed to put value."),
        _ => (),
    }

    // and get
    match k.get("log_width") {
        Ok(value) => match value.downcast_ref::<f64>() {
            Some(fvalue) => println!("GaussianKernel::log_width: {}", fvalue),
            None => println!("GaussianKernel::log_width not of type f64"),
        },
        Err(msg) => panic!(msg),
    }
}
```