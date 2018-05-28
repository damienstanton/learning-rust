use std::ops;

struct Foo;
struct Bar;

#[derive(Debug, PartialEq)]
struct FooBar;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn foobar_addition() {
        assert_eq!(FooBar, Foo + Bar)
    }
}
