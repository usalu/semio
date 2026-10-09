Feature: Typed Dimensional Expressions
  Formulas are closed expression trees over numbers, lengths, angles, areas, volumes, booleans and text. Every implementation agrees on
  their text syntax, on the dimension of every node, on the value in SI base units and on the order in which named parameters resolve.

  Background:
    Given the committed fixtures under "🧫️fixtures"
    And the Python reference that parses each Python rendering with the standard "ast" module

  Scenario: Text Syntax Parses to the Same Tree as Python's Own Grammar
    When every natural-language source with a token-level Python translation is parsed by both
    Then the trees are identical, so precedence and associativity match

  Scenario Outline: Operator Precedence and Associativity
    When "<source>" is parsed and printed
    Then the canonical text is "<canonical>"
    And parsing the canonical text returns the same tree
    Examples:
      | source           | canonical        |
      | 1 + 2 * 3        | 1 + 2 * 3        |
      | (1 + 2) * 3      | (1 + 2) * 3      |
      | 2 ^ 3 ^ 2        | 2 ^ 3 ^ 2        |
      | (2 ^ 3) ^ 2      | (2 ^ 3) ^ 2      |
      | -2 ^ 2           | -2 ^ 2           |
      | 2 ^ -1           | 2 ^ (-1)         |
      | not a = b        | not a = b        |
      | 2.4m + 90mm      | 2.4 m + 90 mm    |
      | 45° + 30 deg     | 45 deg + 30 deg  |

  Scenario: Syntax Errors Carry a Code and a Character Span
    When every erroneous source of the syntax fixtures is parsed
    Then each failure has the committed code, start and end

  Scenario Outline: Dimensions Follow the Physical Quantities
    Given the parameters of the kind fixtures
    When the kind of "<source>" is inferred
    Then the result is "<outcome>"
    Examples:
      | source       | outcome                       |
      | w * h        | area                          |
      | area / w     | length                        |
      | w ^ 3        | volume                        |
      | area ^ 0.5   | length                        |
      | atan2(w, h)  | angle                         |
      | w + a        | mixed-kinds at the root       |
      | w ^ 4        | exponent at the root          |
      | sqrt(w)      | operand-kind at the root      |

  Scenario: Every Kind Case Is Reproduced
    When the kind of every case is inferred by the implementation and by the Python reference
    Then both report the same kind or the same first error with the same node path

  Scenario: Values Are Computed in SI Base Units
    When every evaluation case is evaluated by the implementation and by the Python reference
    Then both report the same kind and a value within a relative 1e-12, or the same error code and node path

  Scenario Outline: Tolerant Comparison and Lazy Logic
    Given the fixed environment of the evaluation fixtures
    When "<source>" is evaluated
    Then the result is <result>
    Examples:
      | source                        | result |
      | 0.1 + 0.2 = 0.3               | true   |
      | 0.1 + 0.2 < 0.3               | false  |
      | 1 m = 1.0000001 m             | false  |
      | zero != 0 and 1 / zero > 2    | false  |
      | zero = 0 or 1 / zero > 2      | true   |

  Scenario: Parameter Sets Resolve in a Deterministic Layered Order
    When every parameter case is resolved by the implementation and by the Python reference using networkx
    Then both list the same evaluation order, the same values and the same errors

  Scenario: Cycles Fail Their Members and Block Their Dependants
    Given parameters that reference each other in a cycle and parameters that depend on that cycle
    When the set is resolved
    Then every member of the cycle fails with the sorted member list
    And every dependant fails naming its first failed dependency by name
    And independent parameters still resolve

  Scenario: Overrides Replace Formulas and an Unknown Override Is Rejected
    Given a parameter set and overrides of existing and missing parameters
    When the set is resolved
    Then dependants see the overriding formula and the missing override fails with "unknown-override"
