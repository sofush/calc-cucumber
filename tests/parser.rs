use calc::parser::Token;
use cucumber::{
    World,
    gherkin::{self, Feature},
    then, when,
};
use std::str::FromStr;

#[derive(Debug, Default, World)]
struct ParserWorld {
    input: String,
    tokens: Vec<Token>,
}

#[when(regex = r#"I parse "([^"]+)""#)]
fn parse_input(world: &mut ParserWorld, input: String) {
    world.input = input.clone();
    world.tokens = calc::parser::parse(&input);
}

#[then("the tokens should be:")]
fn check_tokens(world: &mut ParserWorld, step: &gherkin::Step) {
    let table = step.table.as_ref().expect("expected data table");

    let expected = table
        .rows
        .iter()
        .map(|row| Token::from_str(&row[0]).map_err(|e| anyhow::anyhow!(e)))
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("the tokens should be able to be parsed");

    assert_eq!(world.tokens, expected);
}

#[tokio::main]
async fn main() {
    ParserWorld::filter_run("tests/features", |feature: &Feature, _, _| {
        feature.tags.iter().any(|tag| tag == "parser")
    })
    .await;
}
