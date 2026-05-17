use assert_matches::assert_matches;
use calc::interpreter::Interpreter;
use cucumber::{World, gherkin::Feature, then, when};

#[derive(Debug, Default, World)]
struct IntegrationWorld {
    interpreter: Interpreter,
    result: Option<anyhow::Result<()>>,
}

#[when(regex = "I evaluate \"(.*)\"")]
fn provide_input(world: &mut IntegrationWorld, input: String) {
    let tokens = calc::parser::parse(&input);

    for token in &tokens {
        let res = world.interpreter.interpret(token);

        if res.is_err() {
            world.result = Some(res);
        }
    }
}

#[then(regex = r"stack result should be \[(.*)\]")]
fn check_stack(world: &mut IntegrationWorld, expected: String) {
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

#[then(regex = r#"the error message should contain "(.*)""#)]
fn check_for_error_message(world: &mut IntegrationWorld, expected: String) {
    assert_matches!(world.result, Some(Err(_)));

    let error = world
        .result
        .as_ref()
        .expect("Expected a result")
        .as_ref()
        .expect_err("Value should be an error");

    assert!(
        error.to_string().contains(&expected),
        "Expected error message to contain `{expected}`, got `{error}`"
    );
}

#[tokio::main]
async fn main() {
    IntegrationWorld::filter_run(
        "tests/features",
        |feature: &Feature, _, _| {
            feature.tags.iter().any(|tag| tag == "integration")
        },
    )
    .await;
}
