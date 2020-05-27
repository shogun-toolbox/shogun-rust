use shogun_rust::shogun::{Distance, Kernel, Machine, Version};

fn main() {
    let version = Version::new();
    println!("Shogun version {}", version.main_version().unwrap());

    let rf = Machine::new("RandomForest").unwrap();
    println!("{}", rf);

    let gaussian = Kernel::new("GaussianKernel").unwrap();
    println!("{}", gaussian);
    match gaussian.get("log_width") {
        Ok(value) => match value.downcast_ref::<f64>() {
            Some(fvalue) => println!("log_width: {}", fvalue),
            None => println!("log_width not f64"),
        },
        Err(msg) => panic!(msg),
    }

    match gaussian.get("cache_size") {
        Ok(value) => match value.downcast_ref::<i32>() {
            Some(fvalue) => println!("cache_size: {}", fvalue),
            None => println!("cache_size not i32"),
        },
        Err(msg) => panic!(msg),
    }

    match gaussian.get("m_distance") {
        Ok(value) => match value.downcast_ref::<Distance>() {
            Some(fvalue) => println!("m_distance: {}", fvalue),
            None => println!("m_distance not Distance"),
        },
        Err(msg) => panic!(msg),
    }

    match Machine::new("RandomForests") {
        Ok(_) => println!("All good"),
        Err(msg) => println!("ShogunException: {}", msg),
    }
}
