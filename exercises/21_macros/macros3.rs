// macro in a different module cannot be used without importing it.
// Use the `macro_use` attribute to import the macro from the `macros` module.
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
