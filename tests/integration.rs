use calc::interpreter::Interpreter;
use cucumber::{World, gherkin::Feature, when};

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
        world.result = world.result.take().or(Some(res));
    }
}

#[then(regex = "the result should be ")]
fn provide_input(world: &mut IntegrationWorld, input: String) {
    let tokens = calc::parser::parse(&input);

    for token in &tokens {
        let res = world.interpreter.interpret(token);
        world.result = world.result.take().or(Some(res));
    }
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
