@capability-obj-3-0-material-mutate
@oracle-three-obj-3-0-document-reader
@comparison-semantic-obj-document-v1
@mutations-obj-3.0-material
Feature: Apply every typed OBJ 3.0 mutation to a real-world mesh
  See ../🦁️mutate-obj-3-0/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real mesh
    Given the real input mesh shared://🧪️pattern-sphere/🧊️.obj
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                                                       |
      | set-mtllib            | {"mtllib":"pattern-sphere.mtl"}                                                              |
      | set-usemtl            | {"usemtl":[{"faceIndexFrom":0,"material":"clay"}]}                                           |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real mesh
    Given the real input mesh shared://🧪️pattern-sphere/🧊️.obj
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the mesh matches its pre-mutation semantic projection
    Examples:
      | id                    | params                                                                                       |
      | set-mtllib            | {"mtllib":"pattern-sphere.mtl"}                                                              |
      | set-usemtl            | {"usemtl":[{"faceIndexFrom":0,"material":"clay"}]}                                           |
