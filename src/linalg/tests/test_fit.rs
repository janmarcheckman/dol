use crate::linalg::fit::fit;

#[test]
fn test_fit()
{
    assert_eq!(fit(3.0, 3.0, 4.0, 5.0, 6.0), 5.0);
    assert_eq!(fit(4.0, 3.0, 4.0, 5.0, 6.0), 6.0);
    assert_eq!(fit(3.5, 3.0, 4.0, 5.0, 6.0), 5.5);
}