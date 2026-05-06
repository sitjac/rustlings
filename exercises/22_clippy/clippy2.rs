fn main() {
    let mut res = 42;
    let option = Some(12);
    // let option: Option<i32> = None;
    
    // Fix the Clippy lint.
    // cargo clippy --bin clippy2
    // warning: for loop over an `Option`. This is more readably written as an `if let` statement
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
