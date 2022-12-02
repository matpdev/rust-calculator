pub mod functions;
pub mod constants;

#[test]
fn wrong_input() {
    let entry_data = "-".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = "1 -".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = "1".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = ".".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = "stop_war".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = "LEROLERO".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
}

#[test]
fn add() {
    let entry_data = "1.022 + 3.009".to_string();
    assert_eq!("4.031", &functions::entry_parser(entry_data));
    let entry_data = "-1 + -2".to_string();
    assert_eq!("-3", &functions::entry_parser(entry_data));
}
#[test]
fn div() {
    let entry_data = "1.022 ÷ 3.009".to_string();
    assert_eq!("0.3396", &functions::entry_parser(entry_data));
    let entry_data = "6 ÷ -2".to_string();
    assert_eq!("-3", &functions::entry_parser(entry_data))
}