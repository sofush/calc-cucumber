use calc::PostfixNotationCalculator;
use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
struct CalcWorld {
    calc: PostfixNotationCalculator,
}

#[given(regex = r"the number (\d+)")]
fn given_the_number(world: &mut CalcWorld, n: i32) {
    world.calc.push(n);
}

#[when("I do addition")]
fn add_numbers(world: &mut CalcWorld) {
    world.calc.add();
}

#[when("I do subtraction")]
fn subtract_numbers(world: &mut CalcWorld) {
    world.calc.subtract();
}

#[when("I do multiplication")]
fn multiply_numbers(world: &mut CalcWorld) {
    world.calc.multiply();
}

#[then(regex = r"the calculator should hold (\d+) numbers?")]
fn check_length(world: &mut CalcWorld, expected: usize) {
    assert_eq!(world.calc.stack().len(), expected);
}

#[then(regex = r"the result should be (\d+)")]
fn check_result(world: &mut CalcWorld, expected: i32) {
    let top_of_stack = world.calc.stack().last();
    assert_eq!(top_of_stack, Some(&expected));
}

#[tokio::main]
async fn main() {
    CalcWorld::run("tests/features").await;
}
