use shogun::shogun::{Distance, Kernel, Machine, Version, Features, File, CombinationRule, set_num_threads, Labels, Evaluation};
use ndarray::arr2;

fn main() -> Result<(), String> {
    let version = Version::new();
    println!("Shogun version {}", version.main_version()?);

    let rf = Machine::new("RandomForest")?;
    println!("{}", rf);

    let mut gaussian = Kernel::new("GaussianKernel")?;
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
        Err(_) => println!("Failed to put value."),
        _ => (),
    }

    match gaussian.put("log_width", &1) {
        Err(_) => println!("Failed to put value."),
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

    let features1 = Features::from_array(&array1)?;
    let features2 = Features::from_array(&array2)?;

    gaussian.init(&features1, &features2)?;

    println!("{}", gaussian);

    Ok(())
}
