use std::borrow::Cow;

use owo_colors::OwoColorize;

fn foo() {}

fn main() {
    foo();
    let a = 42u32;
    let s = Cow::from(String::new());

    let _r: &str = &s;
    
    println!("{a:>10}");
    println!("what is going on here?!");

    println!(
        "{:<10}, {:}",
        "Hello".bold(),
        "world!".green().strikethrough()
    );
}
