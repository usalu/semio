@capability-dxf-r12-entities-mutate
@oracle-dxf-crate-r12-mutate-reader
@comparison-semantic-dxf-r12-v1
@mutations-dxf-r12-entities
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
      | insert-entity      | {"index": 2, "entityKind": "circle", "layer": "0", "center": [1200, 100, 0], "radius": 30}                                                      |
      | remove-entity      | {"index": 3}                                                                                                                                     |
      | set-entity         | {"index": 5, "entityKind": "text", "layer": "DIMS", "position": [200, 260, 0], "height": 80, "value": "WAVE 7 SHELTER"}                          |

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
      | insert-entity      | {"index": 2, "entityKind": "circle", "layer": "0", "center": [1200, 100, 0], "radius": 30}                                                      |
      | remove-entity      | {"index": 3}                                                                                                                                     |
      | set-entity         | {"index": 5, "entityKind": "text", "layer": "DIMS", "position": [200, 260, 0], "height": 80, "value": "WAVE 7 SHELTER"}                          |
