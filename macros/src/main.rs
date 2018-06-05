macro_rules! ok {
    () => {
        println!("OK");
    };
}

fn main() {
    ok!();
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_ok() {
    //     assert_eq!("OK", ok!());
    // }
}
