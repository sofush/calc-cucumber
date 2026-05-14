Feature: Addition

  Scenario: Add two positive numbers
    Given the number 5
    Given the number 3
    When I do addition
    Then the calculator should hold 1 number
    Then the result should be 8
