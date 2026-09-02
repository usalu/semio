@capability-gif-89a-comment-mutate
@oracle-gif-89a-extension-reader
@comparison-semantic-gif-89a-extension-v1
@mutations-gif-89a-comment
Feature: Apply every typed GIF 89a mutation to a real-world animation
  See ../mutate-gif-89a/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real animation
    Given the real input document asset://🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🧪️dancing/🖼️.gif
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                          | params                                                                   |
      | insert-comment              | {"index":0,"text":"oracle mutation test"}                                |
      | remove-comment              | {"index":0}                                                              |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real animation
    Given the real input document asset://🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🧪️dancing/🖼️.gif
    When the <id> mutation is applied and its computed inverse is applied back
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the original semantic projection is recovered
    Examples:
      | id                          | params                                                                   |
      | insert-comment              | {"index":0,"text":"oracle mutation test"}                                |
      | remove-comment              | {"index":0}                                                              |
