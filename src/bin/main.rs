use shogun_rust::shogun::{Version, Machine};

fn main() {
    let version = Version::new();
    println!("Shogun version {}", version.main_version().unwrap());

    let rf = Machine::new("RandomForest");
    println!("{}", rf);
}