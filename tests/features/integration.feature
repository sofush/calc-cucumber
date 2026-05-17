@integration
Feature: Expression evaluation

	Scenario: Evaluate a complex postfix expression
		When I evaluate "5 1 2 + 4 * + 3 - 5"
		Then stack result should be [14, 5]

	Scenario: Evaluate expression with an error
		When I evaluate "1 0 /"
		Then the error message should contain "by zero"

	Scenario: Detect an unknown token
		When I evaluate "5 abc +"
		Then the error message should contain "Unknown token"
