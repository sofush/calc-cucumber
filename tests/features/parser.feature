@parser
Feature: Parsing postfix calculator tokens

  Scenario: Parse a single number
    When I parse "42"
    Then the tokens should be:
      | Number(42) |

  Scenario: Parse operators
    When I parse "+ - * /"
    Then the tokens should be:
      | Add      |
      | Subtract |
      | Multiply |
      | Divide   |

  Scenario: Parse mixed expression
    When I parse "10 20 +"
    Then the tokens should be:
      | Number(10) |
      | Number(20) |
      | Add        |

  Scenario: Parse unknown token
    When I parse "abc"
    Then the tokens should be:
      | Unknown(abc) |

  Scenario: Parse complex expression
    When I parse "5 3 + x /"
    Then the tokens should be:
      | Number(5)  |
      | Number(3)  |
      | Add        |
      | Unknown(x) |
      | Divide     |
