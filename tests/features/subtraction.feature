@calculator
@subtraction
Feature: Subtraction

	Scenario: Subtract zero numbers
		When I do subtraction
		Then the calculator should hold 0 numbers
		Then there should be no result

	Scenario: Subtract one number
		Given the number <x>
		When I do subtraction
		Then the calculator should hold 1 number
		Then the result should be <x>

		Examples:
			| x  |
			| 1  |
			| 0  |
			| -1 |

	Scenario: Subtract two numbers
		Given the number <x>
		Given the number <y>
		When I do subtraction
		Then the calculator should hold 1 number
		Then the result should be <result>

		Examples:
			| x   | y  | result |
			| 2   | 5  | -3     |
			| 10  | -3 | 13     |

	Scenario: Subtract three numbers
		Given the number <x>
		Given the number <y>
		Given the number <z>
		When I do subtraction
		Then the calculator should hold 2 numbers
		Then the result should be <first>
		When I do subtraction
		Then the calculator should hold 1 number
		Then the result should be <second>

		Examples:
			| x  | y  | z   | first | second |
			| 5  | 5  | -5  | 10    | -5     |

