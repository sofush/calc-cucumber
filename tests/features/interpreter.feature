@interpreter
Feature: Interpreter evaluates postfix notation tokens
  Background:
    Given a new interpreter

  Scenario: Push a number onto the stack
    When the interpreter processes Number(5)
    Then the stack result should be [5]

  Scenario: Add two numbers
    Given the interpreter has processed:
      | Number(2)  |
      | Number(3)  |
    When the interpreter processes Add
    Then the stack result should be [5]

  Scenario: Subtract two numbers
    Given the interpreter has processed:
      | Number(10) |
      | Number(4)  |
    When the interpreter processes Subtract
    Then the stack result should be [6]

  Scenario: Multiply two numbers
    Given the interpreter has processed:
      | Number(6)  |
      | Number(7)  |
    When the interpreter processes Multiply
    Then the stack result should be [42]

  Scenario: Divide two numbers
    Given the interpreter has processed:
      | Number(20) |
      | Number(5)  |
    When the interpreter processes Divide
    Then the stack result should be [4]

  Scenario: Divide by zero returns an error
    Given the interpreter has processed:
      | Number(10) |
      | Number(0)  |
    When the interpreter processes Divide
    Then an error should occur
    And the error message should contain "by zero"

  Scenario: Unknown token returns an error
    When the interpreter processes Unknown(foo)
    Then an error should occur
    And the error message should contain "Unknown token `foo`."

  Scenario: Multiple operations are evaluated correctly
    Given the interpreter has processed:
      | Number(5)   |
      | Number(3)   |
      | Add         |
      | Number(2)   |
      | Multiply    |
    Then the stack result should be [16]

  Scenario: Stack preserves multiple values
    Given the interpreter has processed:
      | Number(1)  |
      | Number(2)  |
      | Number(3)  |
    Then the stack result should be [1, 2, 3]
