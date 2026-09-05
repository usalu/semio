@capability-avi-1-0-movi-mutate
@oracle-riff-avi-1-0-mutate-reader
@comparison-semantic-avi-v1
@mutations-avi-1.0-movi
Feature: Apply every typed AVI 1.0 mutation to a real-world video container
  See ../🎛️mutate-avi-1-0/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

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
      | insert-chunk         | {"streamIndex": 0, "index": 1, "chunk": {"fourcc": "00dc", "data": "ffd8ffe0", "keyframe": false}}                                                                                                                                                                                       |
      | remove-chunk         | {"streamIndex": 0, "index": 0}                                                                                                                                                                                                                                                            |
      | set-chunk-keyframe   | {"streamIndex": 0, "index": 0, "keyframe": false}                                                                                                                                                                                                                                         |
      | add-unknown-chunk    | {"index": 2, "item": {"fourcc": "XTRA", "data": "cafef00d"}}                                                                                                                                                                                                                              |
      | remove-unknown-chunk | {"index": 1}                                                                                                                                                                                                                                                                              |

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
      | insert-chunk         | {"streamIndex": 0, "index": 1, "chunk": {"fourcc": "00dc", "data": "ffd8ffe0", "keyframe": false}}                                                                                                                                                                                       |
      | remove-chunk         | {"streamIndex": 0, "index": 0}                                                                                                                                                                                                                                                            |
      | set-chunk-keyframe   | {"streamIndex": 0, "index": 0, "keyframe": false}                                                                                                                                                                                                                                         |
      | add-unknown-chunk    | {"index": 2, "item": {"fourcc": "XTRA", "data": "cafef00d"}}                                                                                                                                                                                                                              |
      | remove-unknown-chunk | {"index": 1}                                                                                                                                                                                                                                                                              |
