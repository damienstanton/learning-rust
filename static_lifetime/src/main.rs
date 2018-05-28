static NUMBER: i32 = 42;

// coerce_static_int: returns a ref to the given object where its static
// lifetime is coerced to the length of the given object
fn coerce_static_int<'a>(_: &'a i32) -> &'a i32 {
    &NUMBER
}

fn main() {
    {
        let lifetime_num = 100;
        let coerced_static = coerce_static_int(&lifetime_num);
        println!("Lifetime number: {}", coerced_static);
    }

    println!("Static number: {}", NUMBER);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lifetime_and_static() {
        {
            let lifetime_num = 100;
            let coerced_static = coerce_static_int(&lifetime_num);
            println!("Lifetime number: {}", coerced_static);
            assert_eq!(coerced_static, &NUMBER);
        }
    }
}
