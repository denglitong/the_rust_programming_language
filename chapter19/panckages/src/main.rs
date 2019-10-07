use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct AnotherPancakes;

fn main() {
    Pancakes::hello_macro();
    AnotherPancakes::hello_macro();
}
