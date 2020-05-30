use shogun::shogun::{File, Features, Machine, CombinationRule, Evaluation, Labels, SGObject, set_num_threads};

#[test]
fn random_forest() -> Result<(), String> {

    set_num_threads(1);

    let f_feats_train = File::read_csv("/home/gf712/shogun/build/examples/meta/data/classifier_4class_2d_linear_features_train.dat".to_string())?;
    let f_feats_test = File::read_csv("/home/gf712/shogun/build/examples/meta/data/classifier_4class_2d_linear_features_test.dat".to_string())?;
    let f_labels_train = File::read_csv("/home/gf712/shogun/build/examples/meta/data/classifier_4class_2d_linear_labels_train.dat".to_string())?;
    let f_labels_test = File::read_csv("/home/gf712/shogun/build/examples/meta/data/classifier_4class_2d_linear_labels_test.dat".to_string())?;

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
    println!("{}", rand_forest);

    let predictions = rand_forest.apply(&features_test)?;

    let acc = Evaluation::new("MulticlassAccuracy")?;
    let accuracy = acc.evaluate(&predictions, &labels_test)?;

    // there is an issue with reproducing results with functions with parallel loops
    // when called from Rust
    if accuracy > 0.7 {
        Ok(())
    } else {
        Err("Expected an accuracy of at least 0.7".to_string())
    }
}