use brute_force::data::read_file;
use std::fs;

#[test]
fn reads_minimal_json_object_of_arrays() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("mini.json");
    fs::write(
        &file,
        r#"{"Open":[1.0,2.0], "Close":[1.5,2.5], "Buy":[true,false]}"#,
    ).unwrap();

    let ds = read_file(&file).expect("should parse");
    assert_eq!(ds.len, 2);
    assert!(ds.columns.contains_key("Open"));
    assert!(ds.columns.contains_key("Close"));
    assert!(ds.columns.contains_key("Buy"));
}
