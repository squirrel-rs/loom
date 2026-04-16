/// Imports
use crate::assert_eval;

#[test]
fn test_rt_hello_world() {
    assert_eval!(
        r#"
        println("Hello, world!");
        "#
    )
}

#[test]
fn test_rt_for() {
    assert_eval!(
        r#"
        let a = 3;
        for i in 0..100 {
            if i == 10 {
                break;
            }
            a += i;
        }
        println(str_of(a));
        "#
    )
}
