# shogun-rust

This is a Rust crate with bindings to the [Shogun](https://github.com/shogun-toolbox/shogun) machine learning framework.

Note: this crate is in very early development and only supports a very limited part of the Shogun library.<br>
Note: this is just a Rust wrapper for the shogun C++ library so the internals/API are not very Rust-like.

More information about the design can be found [here](https://gf712.github.io/programming/2020/05/28/shogun-rust.html).

# Build

Assumes you have shogun-static installed locally, as well as spdlog. If not found CMake will throw an error.

To build simply:
```bash
cargo build
```

And then from another crate:
```rust
extern crate shogun;
```

# Example

## Basic API
```rust
use shogun::shogun::{Kernel, Version};

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
        Err(msg) => println!("Failed to put value."),
        _ => (),
    }

    // and get
    match k.get("log_width") {
        Ok(value) => match value.downcast_ref::<f64>() {
            Some(fvalue) => println!("GaussianKernel::log_width: {}", fvalue),
            None => println!("GaussianKernel::log_width not of type f64"),
        },
        Err(msg) => panic!("{}", msg),
    }
}
```

## Training a Random Forest
```rust
let f_feats_train = File::read_csv("classifier_4class_2d_linear_features_train.dat".to_string())?;
let f_feats_test = File::read_csv("classifier_4class_2d_linear_features_test.dat".to_string())?;
let f_labels_train = File::read_csv("classifier_4class_2d_linear_labels_train.dat".to_string())?;
let f_labels_test = File::read_csv("classifier_4class_2d_linear_labels_test.dat".to_string())?;

let features_train = Features::from_file(&f_feats_train)?;
let features_test = Features::from_file(&f_feats_test)?;
let labels_train = Labels::from_file(&f_labels_train)?;
let labels_test = Labels::from_file(&f_labels_test)?;

let mut rand_forest = Machine::new("RandomForest")?;
let m_vote = CombinationRule::new("MajorityVote")?;

rand_forest.put("labels", &labels_train)?;
rand_forest.put("num_bags", &100)?;
rand_forest.put("combination_rule", &m_vote)?;
rand_forest.put("seed", &1)?;

rand_forest.train(&features_train)?;

let predictions = rand_forest.apply(&features_test)?;

let acc = Evaluation::new("MulticlassAccuracy")?;
rand_forest.put("oob_evaluation_metric", &acc)?;
let accuracy = acc.evaluate(&predictions, &labels_test)?;

println!("Model accuracy: {}", accuracy);
```