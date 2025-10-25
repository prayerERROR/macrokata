////////// DO NOT CHANGE BELOW HERE /////////
fn print_success() {
    println!("Yay, the if statement worked.");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `if_any!()` macro.
macro_rules! if_any {
    ($($cond:expr),+; $func:block) => {
        if $($cond)||+ $func
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    if_any!(false, 0 == 1, true; {
        print_success();
    })
}
