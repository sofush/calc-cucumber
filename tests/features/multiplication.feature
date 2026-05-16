@calculator
@multiplication
Feature: Multiplication

	Scenario: Multiply zero numbers
		When I do multiplication
		Then the calculator should hold 0 numbers
		Then there should be no result

	Scenario: Multiply one number
		Given the number <x>
		When I do multiplication
		Then the calculator should hold 1 numbers
		Then the result should be <x>

		Examples:
			| x  |
			| 1  |
			| 0  |
			| -1 |

	Scenario: Multiply two numbers
		Given the number <x>
		Given the number <y>
		When I do multiplication
		Then the calculator should hold 1 number
		Then the result should be <result>

		Examples:
			| x   | y  | result |
			| 10  | 2  | 20     |
			| -5  | -5 | 25     |

	Scenario: Multiply two numbers by zero
		Given the number <x>
		Given the number 0
		When I do multiplication
		Then the result should be 0

		Examples:
			| x   |
			| 1   |
			| 0   |
			| -1  |

	Scenario: Multiply three numbers
		Given the number <x>
		Given the number <y>
		Given the number <z>
		When I do multiplication
		Then the calculator should hold 2 number
		Then the result should be <first>
		When I do multiplication
		Then the calculator should hold 1 number
		Then the result should be <second>

		Examples:
			| x  | y  | z   | first | second |
			| 5  | 5  | -5  | -25   | -125   |

