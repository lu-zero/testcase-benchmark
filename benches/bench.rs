use std::env;

fn main() {
    env::args().skip(1).for_each(|argument| {
        println!("{}", argument);
        match argument.as_ref() {
            "--bench" => (),
            "--special-option" => (),
            "--help" => println!("fake bencher"),
            _ => panic!("unsupported option"),
        }
    });
}
