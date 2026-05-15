use calc::interpreter::Interpreter;
use std::io::Write as _;

fn main() -> anyhow::Result<()> {
    loop {
        if let Err(e) = do_calculation() {
            println!("Error: {e}");
        }
    }
}

fn do_calculation() -> anyhow::Result<()> {
    print!("> ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let mut interpreter = Interpreter::default();

    for token in calc::parser::parse(&input) {
        interpreter.interpret(&token)?;
    }

    let numbers = interpreter.result();

    match numbers.len() {
        0 => println!("0"),
        1 => println!("{}", numbers.last().unwrap()),
        n => {
            print!("[");
            numbers.iter().take(n - 1).for_each(|x| print!("{x},"));
            print!("{}", numbers.last().unwrap());
            print!("]");
        }
    }

    Ok(())
}
