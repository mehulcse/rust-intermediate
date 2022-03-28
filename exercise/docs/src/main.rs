/// The exercise is in lib.rs this time!!!
fn main() {
    println!("Go look in lib.rs ;-)");

    let s = "S";
    let f = move || {
        println!("{}", s);
    };
    f();
    println!("{}", s);
}
