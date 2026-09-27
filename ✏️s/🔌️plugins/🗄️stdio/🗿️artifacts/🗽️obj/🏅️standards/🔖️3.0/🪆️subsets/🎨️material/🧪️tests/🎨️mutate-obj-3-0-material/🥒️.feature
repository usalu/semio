@capability-obj-3-0-material-mutate
@oracle-three-obj-3-0-document-reader
@comparison-semantic-obj-document-v1
@mutations-obj-3.0-material
Feature: Apply every typed OBJ 3.0 material mutation and read the result with three's OBJLoader
  The judge is `three`'s OBJLoader, a third-party READER (`three-obj-3-0-document-reader`): the expected
  document is not computed, it is the COMMITTED `➡️after.obj` of each row's pair (`⬅️before.obj` for an
  inverse row), and the `obj-3-0-document-compare-v1` pipeline reads it and the subject's document with the
  same loader (`obj-document-import` admits both, `obj-document-compare` compares the material library,
  the per-child material names, the smoothing state and the resolved geometry). The rows' parameters are
  the committed pairs' own difference: `mtllib base.mtl → other.mtl`, `usemtl red → blue`.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed before-document
    Given the committed before-document shared://<fixture>/⬅️before.obj
    And the committed after-document shared://<fixture>/➡️after.obj
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then three's OBJLoader reads the subject's document and the committed after-document as the same OBJ document
    Examples:
      | id         | fixture        | params                                              |
      | set-mtllib | 🎨️set-mtllib  | {"mtllib":"other.mtl"}                              |
      | set-usemtl | 🖌️set-usemtl  | {"usemtl":[{"faceIndexFrom":0,"material":"blue"}]} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document shared://<fixture>/⬅️before.obj
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then three's OBJLoader reads the restored document and the committed before-document as the same OBJ document
    Examples:
      | id         | fixture        | params                                              |
      | set-mtllib | 🎨️set-mtllib  | {"mtllib":"other.mtl"}                              |
      | set-usemtl | 🖌️set-usemtl  | {"usemtl":[{"faceIndexFrom":0,"material":"blue"}]} |
