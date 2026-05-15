use assert_matches::assert_matches;
use calc::PostfixNotationCalculator;
use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
struct CalcWorld {
    calc: PostfixNotationCalculator,
    result: Option<anyhow::Result<()>>,
}

#[given(regex = r"the number (-?\d+)")]
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

#[when("I do division")]
fn divide_numbers(world: &mut CalcWorld) {
    world.result = Some(world.calc.divide());
}

#[then(regex = r"the calculator should hold (\d+) numbers?")]
fn check_length(world: &mut CalcWorld, expected: usize) {
    assert_eq!(world.calc.stack().len(), expected);
}

#[then(regex = r"the result should be (-?\d+)")]
fn check_result(world: &mut CalcWorld, expected: i32) {
    let top_of_stack = world.calc.stack().last();
    assert_eq!(top_of_stack, Some(&expected));
}

#[then(regex = r"there should be no result")]
fn check_no_result(world: &mut CalcWorld) {
    let top_of_stack = world.calc.stack().last();
    assert_eq!(top_of_stack, None);
}

#[then(regex = r"there should be an error")]
fn check_error(world: &mut CalcWorld) {
    assert_matches!(world.result, Some(Err(_)));
}

#[tokio::main]
async fn main() {
    CalcWorld::run("tests/features").await;
}
