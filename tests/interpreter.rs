use assert_matches::assert_matches;
use std::str::FromStr as _;

use calc::{interpreter::Interpreter, parser::Token};
use cucumber::{
    World,
    gherkin::{self, Feature},
    given, then, when,
};

#[derive(Debug, Default, World)]
struct InterpreterWorld {
    interpreter: Interpreter,
    result: Option<anyhow::Result<()>>,
}

#[given(expr = "a new interpreter")]
fn create_interpreter(world: &mut InterpreterWorld) {
    world.interpreter = Interpreter::default();
}

#[when(regex = r#"the interpreter processes (.*)"#)]
fn input_token(world: &mut InterpreterWorld, input: String) {
    let token = Token::from_str(&input).expect("input should be a token");
    world.result = Some(world.interpreter.interpret(&token));
}

#[given("the interpreter has processed:")]
fn input_tokens(world: &mut InterpreterWorld, step: &gherkin::Step) {
    let table = step.table.as_ref().expect("expected data table");

    let input_tokens = table
        .rows
        .iter()
        .map(|row| Token::from_str(&row[0]).map_err(|e| anyhow::anyhow!(e)))
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("the tokens should be able to be parsed");

    for token in input_tokens {
        let res = world.interpreter.interpret(&token);

        if matches!(world.result, Some(Ok(_)) | None) {
            world.result = world.result.take().or(Some(res));
        }
    }
}

#[then(regex = r"stack result should be \[(.*)\]")]
fn check_stack(world: &mut InterpreterWorld, expected: String) {
    let expected: Vec<i32> = if expected.trim().is_empty() {
        vec![]
    } else {
        expected
            .split(',')
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect()
    };

    assert_eq!(world.interpreter.result(), expected);
}

#[then(regex = "an error should occur")]
fn check_for_error(world: &mut InterpreterWorld) {
    assert_matches!(world.result, Some(Err(_)));
}

#[then(regex = r#"the error message should contain "(.*)""#)]
fn check_for_error_message(world: &mut InterpreterWorld, expected: String) {
    let error = world
        .result
        .as_ref()
        .expect("Expected a result")
        .as_ref()
        .expect_err("Expected result to be an error");

    assert!(
        error.to_string().contains(&expected),
        "Expected error message to contain `{expected}`, got `{error}`"
    );
}

#[tokio::main]
async fn main() {
    InterpreterWorld::filter_run(
        "tests/features",
        |feature: &Feature, _, _| {
            feature.tags.iter().any(|tag| tag == "interpreter")
        },
    )
    .await;
}
