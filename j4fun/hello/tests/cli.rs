use std::process::Command;

#[test]
fn works() {
    assert!(true)
}

#[test]
fn runs() {
    let mut cmd = Command::new("hello");
    let res = cmd.output();
    assert!(res.is_ok())
}