@capability-bcf-2-1-viewpoint-mutate
@oracle-jszip-bcf-2-1-mutate-reader
@comparison-semantic-bcf-jszip-v1
@mutations-bcf-2.1-viewpoint
Feature: Apply every typed BCF 2.1 viewpoint mutation to a committed review pair
  See ../../../🖊️markup/🧪️tests/🔀️mutate-bcf-2-1/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

  The judge is the third-party jszip READER: each row's expected document is the COMMITTED `➡️after.bcf` of the
  before/after pair the markup subset's generator (`../../../🖊️markup/🏭️generator/📜️script.ts`) builds directly with
  jszip + fast-xml-parser for this subset (`⬅️before.bcf` for an inverse row); the parameters state exactly that pair's
  one change, and `bcf-2-1-jszip-compare-v1` reads it and the subject's `actual-bcf` with the same reader. The
  cross-semio `zip`+`quick-xml` composition stays as the Rust supplement.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the committed review pair
    Given the real input document shared://<fixture>/⬅️before.bcf
    And the committed after-document shared://<fixture>/➡️after.bcf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the jszip reader reads the subject's review and the committed after-document as the same BCF
    Examples:
      | id                       | fixture                            | params |
      | insert-viewpoint         | 👁️insert-viewpoint-applied         | {"topicGuid": "topic-clash-01", "viewpoint": {"guid": "viewpoint-02", "camera": {"kind": "perspective", "viewPoint": {"x": -3, "y": 8, "z": 1}, "direction": {"x": 0, "y": 0, "z": -1}, "upVector": {"x": 0, "y": 1, "z": 0}, "fieldOfView": 45}}} |
      | remove-viewpoint         | 🙈️remove-viewpoint-applied         | {"topicGuid": "topic-clash-01", "guid": "viewpoint-01"} |
      | set-viewpoint-camera     | 📷️set-viewpoint-camera-applied     | {"topicGuid": "topic-clash-01", "guid": "viewpoint-01", "camera": {"kind": "perspective", "viewPoint": {"x": 20, "y": 20, "z": 20}, "direction": {"x": 0, "y": 0, "z": -1}, "upVector": {"x": 0, "y": 1, "z": 0}, "fieldOfView": 90}} |
      | set-viewpoint-components | 🧱️set-viewpoint-components-applied | {"topicGuid": "topic-clash-01", "guid": "viewpoint-01", "components": {"selection": ["ifc-beam-1"], "visibility": {"defaultVisibility": true, "exceptions": []}, "coloring": []}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed review
    Given the real input document shared://<fixture>/⬅️before.bcf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the jszip reader reads the restored review and the committed before-document as the same BCF
    Examples:
      | id                       | fixture                            | params |
      | insert-viewpoint         | 👁️insert-viewpoint-applied         | {"topicGuid": "topic-clash-01", "viewpoint": {"guid": "viewpoint-02", "camera": {"kind": "perspective", "viewPoint": {"x": -3, "y": 8, "z": 1}, "direction": {"x": 0, "y": 0, "z": -1}, "upVector": {"x": 0, "y": 1, "z": 0}, "fieldOfView": 45}}} |
      | remove-viewpoint         | 🙈️remove-viewpoint-applied         | {"topicGuid": "topic-clash-01", "guid": "viewpoint-01"} |
      | set-viewpoint-camera     | 📷️set-viewpoint-camera-applied     | {"topicGuid": "topic-clash-01", "guid": "viewpoint-01", "camera": {"kind": "perspective", "viewPoint": {"x": 20, "y": 20, "z": 20}, "direction": {"x": 0, "y": 0, "z": -1}, "upVector": {"x": 0, "y": 1, "z": 0}, "fieldOfView": 90}} |
      | set-viewpoint-components | 🧱️set-viewpoint-components-applied | {"topicGuid": "topic-clash-01", "guid": "viewpoint-01", "components": {"selection": ["ifc-beam-1"], "visibility": {"defaultVisibility": true, "exceptions": []}, "coloring": []}} |
