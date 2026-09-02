@capability-dxf-r12-blocks-mutate
@oracle-dxf-crate-r12-mutate-reader
@comparison-semantic-dxf-r12-v1
@mutations-dxf-r12-blocks
Feature: Apply every typed DXF R12 mutation to a real-world drawing
  See ../mutate-dxf-r12/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                 | params                                                                                                                                          |
      | insert-block       | {"index": 1, "name": "BENCH_MARK", "basePoint": [0, 0, 0], "entities": [{"entityKind": "line", "layer": "0", "start": [0, 0, 0], "end": [100, 0, 0]}]} |
      | remove-block       | {"index": 1}                                                                                                                                     |
      | set-block          | {"index": 0, "name": "SHELTER_POST", "basePoint": [0, 0, 0], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 20}]} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                 | params                                                                                                                                          |
      | insert-block       | {"index": 1, "name": "BENCH_MARK", "basePoint": [0, 0, 0], "entities": [{"entityKind": "line", "layer": "0", "start": [0, 0, 0], "end": [100, 0, 0]}]} |
      | remove-block       | {"index": 1}                                                                                                                                     |
      | set-block          | {"index": 0, "name": "SHELTER_POST", "basePoint": [0, 0, 0], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 20}]} |
