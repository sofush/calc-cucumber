@calculator
@addition
Feature: Addition

	Scenario: Add zero numbers
		When I do addition
		Then the calculator should hold 0 numbers
		Then there should be no result

	Scenario: Add one number
		Given the number <x>
		When I do addition
		Then the calculator should hold 1 number
		Then the result should be <x>

		Examples:
			| x  |
			| 1  |
			| 0  |
			| -1 |

	Scenario: Add two numbers
		Given the number <x>
		Given the number <y>
		When I do addition
		Then the calculator should hold 1 number
		Then the result should be <result>

		Examples:
			| x           | y           | result                  |
			# Max og min-værdier
			| 2147483647  | 0           | 2147483647              |
			| -2147483648 | 0           | -2147483648             |
			# Addition op til max og min-værdier
			| 2147483646  | 1           | 2147483647              |
			| -2147483647 | -1          | -2147483648             |
			# Overflow og underflow
			| 2147483647  | 1           | 2147483648              |
			| -2147483648 | -1          | -2147483649             |
			# Specielle værdier
			| 0           | 0           | 0                       |

	Scenario: Add two decimal numbers
		Given the number 10.5
		Given the number 10.5
		When I do addition
		Then there should be an error

	Scenario: Add three numbers
		Given the number <x>
		Given the number <y>
		Given the number <z>
		When I do addition
		Then the calculator should hold 2 numbers
		Then the result should be <first>
		When I do addition
		Then the calculator should hold 1 number
		Then the result should be <second>

		Examples:
			| x  | y  | z  | first | second |
			| 1  | 2  | 3  | 5     | 6      |

