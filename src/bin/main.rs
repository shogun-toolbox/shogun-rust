use shogun_rust::shogun::{Distance, Kernel, Machine, Version, Features};
use ndarray::arr2;

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

    match gaussian.put("log_width", &1.0) {
        Err(msg) => println!("Failed to put value."),
        _ => (),
    }

    match gaussian.put("log_width", &1) {
        Err(msg) => println!("Failed to put value."),
        _ => (),
    }
        
    match gaussian.get("log_width") {
        Ok(value) => match value.downcast_ref::<f64>() {
            Some(fvalue) => println!("log_width: {}", fvalue),
            None => println!("log_width not f64"),
        },
        Err(msg) =>panic!("{}", msg),
    }

    let array1 = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let array2 = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let features = Features::from_array(&array1).unwrap();
    println!("{}", features);

    match features.put("feature_matrx", &array2) {
        Err(msg) => panic!("{}", msg),
        _ => (),
    }

    println!("{}", features);
}
