// error-pattern: unresolved name: a

mod m1 {
    mod a { }
}

fn main(args: vec[str]) { log m1::a; }