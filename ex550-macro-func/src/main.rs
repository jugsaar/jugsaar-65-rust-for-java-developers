macro_rules! calculate {
    (eval $e:expr) => {
        calculate! {
            eval usize, $e
        }
    };

    (eval $t:ty, $e:expr) => {
        {
            let val: $t = $e; // $t = usize Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

#[rustfmt::skip]
fn main() {
    calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }

    calculate! {
        eval f64, (1. + 2.) * (3. / 4.)
    }
}
