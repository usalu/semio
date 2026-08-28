@capability-pdf-1-4-a-mutate
@oracle-lopdf-pdf-1-4-a-mutate
@comparison-semantic-pdf-1-4-conformance-a-v1
@mutations-pdf-1-4-a
Feature: Apply every concrete PDF 1.4 A mutation to a real document
  Every scenario copies the committed 65-page thesis before changing it.
  Forward operations must change the independently read projection. Inverse scenarios call
  the subject's real concrete inverse planner, restore the full snapshot, and compare with lopdf.
  The page-list domain retains geometry and shown text, not PDF 1.7's object graph.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> on the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | set-page-text | {"text":"another abstract"} |
      | clear-page-text | {} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | set-page-text | {"text":"another abstract"} |
      | clear-page-text | {} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the document is fully decoded and re-encoded from its model
    Then the oracle and the subject agree on the semantic projection
