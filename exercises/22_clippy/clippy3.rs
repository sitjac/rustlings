// Here are some more easy Clippy fixes so you can see its utility.
// TODO: Fix all the Clippy lints.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // error: this call to `unwrap()` will always panic
    if let Some(value) = my_option {
        println!("{}", value);
    }

    // error: possibly missing a comma here
    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    // error: emptying a vector with `resize`
    my_vec.clear();
    println!("This Vec is empty, see? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // error: this looks like you are trying to swap `value_a` and `value_b`
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
