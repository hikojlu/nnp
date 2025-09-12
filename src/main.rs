use nnp::Nnp;
use std::env;

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    match args.next().unwrap().as_str() {
        "long" | "l" => println!(
            "{}",
            Nnp::from(args.next().unwrap().parse::<usize>().unwrap())
        ),
        "short" | "s" => println!(
            "{}",
            Nnp::from(args.next().unwrap().parse::<usize>().unwrap()).short()
        ),
        "int" | "dec" | "i" | "d" => println!(
            "{}",
            usize::from(Nnp::try_from(args.collect::<Vec<_>>().join(" ").as_str()).unwrap())
        ),
        _ => panic!("???"),
    }
}
