macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

macro_rules! calculateV1 {
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

macro_rules! calculateV2 {
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculateV1! { eval $e }
        calculateV1! { $(eval $es),+ } // FIXME: no rules expected the token `,`. Look into this...
    }};
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));

    calculateV1! {
        eval 1 + 2
    }

    calculateV1! {
        eval (1 + 2) * (3 / 4)
    }

    calculateV2! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
