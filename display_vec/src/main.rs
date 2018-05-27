/// ## Quick note on `?` vs `try!`
/// The `?` syntax is the preferred method of indicating a "Result" return value
/// Old Rust code might still use the `try!` syntax.
/// New, good:
/// ```
/// write!(f, "{}", value)?;
/// ```
/// Old, bad:
/// ```
/// try!(write!(f, "{}", value));
/// ```
use std::fmt;

// List with a vector
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (i, v) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", i, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v)
}
