@capability-dxf-r12-tables-mutate
@oracle-dxf-crate-r12-mutate-reader
@comparison-semantic-dxf-r12-v1
@mutations-dxf-r12-tables
Feature: Apply every typed DXF R12 mutation to a real-world drawing
  See ../📰️mutate-dxf-r12/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🚏️bus-shelter/🖊️.dxf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                 | params                                                                                                                                          |
      | insert-layer       | {"index": 1, "layer": {"name": "MARKERS", "color": 6, "linetype": "CONTINUOUS", "flags": 0}} |
      | remove-layer       | {"name": "DIMS"}                                                                                                                                 |
      | set-layer          | {"name": "DIMS", "layer": {"name": "DIMS", "color": 4, "linetype": "DASHED", "flags": 0}} |
      | insert-style       | {"index": 1, "style": {"name": "LABELS", "flags": 0, "fontName": "arial.ttf"}} |
      | remove-style       | {"name": "NOTES"}                                                                                                                                |
      | set-style          | {"name": "NOTES", "style": {"name": "NOTES", "flags": 0, "fontName": "romans.shx"}} |
      | insert-linetype    | {"index": 1, "linetype": {"name": "CENTER", "flags": 0, "description": "Center line"}} |
      | remove-linetype    | {"name": "DASHED"}                                                                                                                               |
      | set-linetype       | {"name": "DASHED", "linetype": {"name": "DASHED", "flags": 0, "description": "Dash pattern"}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://🚏️bus-shelter/🖊️.dxf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                 | params                                                                                                                                          |
      | insert-layer       | {"index": 1, "layer": {"name": "MARKERS", "color": 6, "linetype": "CONTINUOUS", "flags": 0}} |
      | remove-layer       | {"name": "DIMS"}                                                                                                                                 |
      | set-layer          | {"name": "DIMS", "layer": {"name": "DIMS", "color": 4, "linetype": "DASHED", "flags": 0}} |
      | insert-style       | {"index": 1, "style": {"name": "LABELS", "flags": 0, "fontName": "arial.ttf"}} |
      | remove-style       | {"name": "NOTES"}                                                                                                                                |
      | set-style          | {"name": "NOTES", "style": {"name": "NOTES", "flags": 0, "fontName": "romans.shx"}} |
      | insert-linetype    | {"index": 1, "linetype": {"name": "CENTER", "flags": 0, "description": "Center line"}} |
      | remove-linetype    | {"name": "DASHED"}                                                                                                                               |
      | set-linetype       | {"name": "DASHED", "linetype": {"name": "DASHED", "flags": 0, "description": "Dash pattern"}} |
