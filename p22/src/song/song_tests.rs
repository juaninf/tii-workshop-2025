use super::*;

#[test]
fn test_print_song() {
    let expected_output = "On the first day of Christmas my true love sent to me";
    let nice_slice = &print_song()[0..53];
    assert_eq!(nice_slice, expected_output);
}
