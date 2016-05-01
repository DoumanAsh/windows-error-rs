extern crate windows_error;

use windows_error::*;

#[test]
fn win_error_test() {
    //check PartialEq
    assert!(WindowsError::new(0) == 0 as u16);
    assert!(WindowsError::new(0) == WindowsError::from(0));
    assert!(WindowsError::new(1) != WindowsError::from(0));

    let result = WindowsError::new(0);
    assert!("The operation completed successfully.".to_string() == result.errno_desc());
    println!("{}={}", &result, result.errno_desc());

    let result = WindowsError::new(1);
    assert!("Incorrect function.".to_string() == result.errno_desc());
    println!("{}={}", &result, result.errno_desc());

    let result = WindowsError::new(666);
    assert!("Unknown Error.".to_string() == result.errno_desc());
    println!("{}={}", &result, result.errno_desc());
}
