@integration
Feature: Expression evaluation

  Scenario: Evaluate a complex postfix expression
    When I evaluate "5 1 2 + 4 * + 3 -"
    Then the result should be [14]

  Scenario: Detect an unknown token
    When I evaluate "5 abc +"
    Then the error message should contain "Unknown token"
