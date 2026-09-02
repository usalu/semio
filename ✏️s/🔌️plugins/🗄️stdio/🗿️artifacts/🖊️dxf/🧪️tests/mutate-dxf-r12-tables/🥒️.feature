@capability-dxf-r12-tables-mutate
@oracle-dxf-crate-r12-mutate-reader
@comparison-semantic-dxf-r12-v1
@mutations-dxf-r12-tables
Feature: Apply every typed DXF R12 mutation to a real-world drawing
  See ../mutate-dxf-r12/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🏅️standards/🔖️r12/🪆️subsets/✳️header/📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                 | params                                                                                                                                          |
      | insert-layer       | {"index": 1, "name": "MARKERS", "color": 6, "linetype": "CONTINUOUS"}                                                                           |
      | remove-layer       | {"name": "DIMS"}                                                                                                                                 |
      | set-layer          | {"name": "DIMS", "color": 4, "linetype": "DASHED"}                                                                                              |
      | insert-style       | {"index": 1, "name": "LABELS", "font": "arial.ttf"}                                                                                             |
      | remove-style       | {"name": "NOTES"}                                                                                                                                |
      | set-style          | {"name": "NOTES", "font": "romans.shx"}                                                                                                         |
      | insert-linetype    | {"index": 1, "name": "CENTER", "description": "Center line"}                                                                                    |
      | remove-linetype    | {"name": "DASHED"}                                                                                                                               |
      | set-linetype       | {"name": "DASHED", "description": "Dash pattern"}                                                                                               |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://🏅️standards/🔖️r12/🪆️subsets/✳️header/📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                 | params                                                                                                                                          |
      | insert-layer       | {"index": 1, "name": "MARKERS", "color": 6, "linetype": "CONTINUOUS"}                                                                           |
      | remove-layer       | {"name": "DIMS"}                                                                                                                                 |
      | set-layer          | {"name": "DIMS", "color": 4, "linetype": "DASHED"}                                                                                              |
      | insert-style       | {"index": 1, "name": "LABELS", "font": "arial.ttf"}                                                                                             |
      | remove-style       | {"name": "NOTES"}                                                                                                                                |
      | set-style          | {"name": "NOTES", "font": "romans.shx"}                                                                                                         |
      | insert-linetype    | {"index": 1, "name": "CENTER", "description": "Center line"}                                                                                    |
      | remove-linetype    | {"name": "DASHED"}                                                                                                                               |
      | set-linetype       | {"name": "DASHED", "description": "Dash pattern"}                                                                                               |
