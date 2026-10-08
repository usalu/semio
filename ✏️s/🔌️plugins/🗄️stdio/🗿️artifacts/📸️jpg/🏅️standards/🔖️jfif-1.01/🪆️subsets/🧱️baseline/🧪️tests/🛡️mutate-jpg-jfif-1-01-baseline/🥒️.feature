@capability-jpg-jfif-1-01-baseline-native-conformance
@oracle-libjpeg-jpg-jfif-1-01-baseline-marker-cli
@comparison-ordered-json-v1
Feature: Classify native JPEG baseline observation profiles independently
  SOFn, precision, DAC, Huffman tables and component sampling belong to native IO observations.
  Both readers inspect the real photographic input. Each neutral profile changes one observation,
  and independent T.81 classification must agree with the first-party pure conformance checker.
  Authored image mutations use the document subset vocabulary.

  @id-profile
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Inspect <id> in the native observation profile
    Given the real input document shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the native observations are classified using the explicit profile
      """
      {"kind": "<id>", "code": "<code>", "params": <params>}
      """
    Then the conformance verdict contains <code> and the observation changes
    Examples:
      | id | code | params |
      | set-sof-marker | stdio.jpg.baseline.sof-marker | {"marker":194} |
      | set-sample-precision | stdio.jpg.baseline.precision | {"precision":12} |
      | set-arithmetic | stdio.jpg.baseline.arithmetic-conditioning-present | {"arithmetic":true} |
      | insert-huffman-table | stdio.jpg.baseline.huffman-table-count | {"index":4,"table":{"id":2,"class":"dc","bits":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"values":[]}} |
      | remove-huffman-table |  | {"key":{"class":"dc","id":0}} |
      | insert-frame-component |  | {"index":3,"component":{"id":4,"hSampling":1,"vSampling":1,"quantTableId":0}} |
      | remove-frame-component |  | {"id":3} |
      | set-component-sampling | stdio.jpg.baseline.component-sampling | {"id":1,"hSampling":5,"vSampling":1} |


  @id-restore-profile
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Restore <id> native observations
    Given the real input document shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the changed native observations are replaced by the independently read original
      """
      {"kind": "<id>", "code": "<code>", "params": <params>}
      """
    Then the conformance projection equals the original native observations
    Examples:
      | id | code | params |
      | set-sof-marker | stdio.jpg.baseline.sof-marker | {"marker":194} |
      | set-sample-precision | stdio.jpg.baseline.precision | {"precision":12} |
      | set-arithmetic | stdio.jpg.baseline.arithmetic-conditioning-present | {"arithmetic":true} |
      | insert-huffman-table | stdio.jpg.baseline.huffman-table-count | {"index":4,"table":{"id":2,"class":"dc","bits":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"values":[]}} |
      | remove-huffman-table |  | {"key":{"class":"dc","id":0}} |
      | insert-frame-component |  | {"index":3,"component":{"id":4,"hSampling":1,"vSampling":1,"quantTableId":0}} |
      | remove-frame-component |  | {"id":3} |
      | set-component-sampling | stdio.jpg.baseline.component-sampling | {"id":1,"hSampling":5,"vSampling":1} |

