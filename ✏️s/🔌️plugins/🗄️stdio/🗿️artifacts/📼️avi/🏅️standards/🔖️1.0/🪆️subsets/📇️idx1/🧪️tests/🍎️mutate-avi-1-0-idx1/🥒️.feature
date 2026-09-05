@capability-avi-1-0-idx1-mutate
@oracle-riff-avi-1-0-mutate-reader
@comparison-semantic-avi-v1
@mutations-avi-1.0-idx1
Feature: Apply every typed AVI 1.0 mutation to a real-world video container
  See ../🐯️mutate-avi-1-0/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real video container
    Given the real input document shared://🎬️.avi
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                                                                                                                                                                    |
      | set-idx1-present     | {"idx1Present": false}                                                                                                                                                                                                                                                                    |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real video container
    Given the real input document shared://🎬️.avi
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                                                                                                                                                                    |
      | set-idx1-present     | {"idx1Present": false}                                                                                                                                                                                                                                                                    |
