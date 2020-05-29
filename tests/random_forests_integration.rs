use shogun::shogun::{File, Features, Machine, CombinationRule, Evaluation, Labels};

#[test]
fn random_forest() -> Result<(), String> {
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

    let predictions = rand_forest.apply(&features_test)?;

    let acc = Evaluation::new("MulticlassAccuracy")?;
    rand_forest.put("oob_evaluation_metric", &acc)?;
    let accuracy = acc.evaluate(&predictions, &labels_test)?;

    assert_eq!(accuracy, 0.75);
    
    Ok(())
}