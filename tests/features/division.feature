@calculator
@division
Feature: Division

	Scenario: Divide zero numbers
		When I do division
		Then the calculator should hold 0 numbers
		Then there should be no result

	Scenario: Divide one number
		Given the number <x>
		When I do division
		Then the calculator should hold 1 numbers
		Then the result should be <x>

		Examples:
			| x  |
			| 1  |
			| 0  |
			| -1 |

	Scenario: Divide two numbers
		Given the number <x>
		Given the number <y>
		When I do division
		Then the calculator should hold 1 number
		Then the result should be <result>

		Examples:
			| x   | y  | result |
			| 10  | 2  | 5      |
			| -5  | -5 | 1      |

	Scenario: Divide two numbers by zero
		Given the number <x>
		Given the number 0
		When I do division
		Then there should be an error

		Examples:
			| x   |
			| 1   |
			| 0   |
			| -1  |

	Scenario: Divide three numbers
		Given the number <x>
		Given the number <y>
		Given the number <z>
		When I do division
		Then the calculator should hold 2 number
		Then the result should be <first>
		When I do division
		Then the calculator should hold 1 number
		Then the result should be <second>

		Examples:
			| x  | y  | z   | first | second |
			| 5  | 5  | -5  | -1    | -5     |

