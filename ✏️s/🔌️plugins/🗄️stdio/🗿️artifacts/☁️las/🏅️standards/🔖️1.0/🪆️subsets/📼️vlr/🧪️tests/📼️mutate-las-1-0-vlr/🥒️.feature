@capability-las-1-0-vlr-mutate
@oracle-las-1-0-any-mutate
@comparison-semantic-las-v1
@mutations-las-1.0-vlr
Feature: Apply every typed LAS 1.0 mutation to a real-world point cloud
  See ../🎩️mutate-las-1-0/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real point cloud
    Given the real input point cloud shared://🧪️pattern-sphere/🧊️.las
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params |
      | insert-vlr             | {"index": 1, "vlr": {"userId": "semio-test", "recordId": 9, "description": "inserted vlr", "data": "hello-vlr"}} |
      | remove-vlr             | {"index": 0} |
      | set-vlr-data           | {"index": 1, "data": "patched-provenance"} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real point cloud
    Given the real input point cloud shared://🧪️pattern-sphere/🧊️.las
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original point cloud
    Examples:
      | id                    | params |
      | insert-vlr             | {"index": 1, "vlr": {"userId": "semio-test", "recordId": 9, "description": "inserted vlr", "data": "hello-vlr"}} |
      | remove-vlr             | {"index": 0} |
      | set-vlr-data           | {"index": 1, "data": "patched-provenance"} |
