@capability-tiff-6-0-baseline-native-conformance
@oracle-tiff-tiff-6-0-baseline-native-reader
@comparison-ordered-json-v1
Feature: Classify native TIFF storage observations at IO
  Compression, strip offsets and tile organization are ephemeral native observations.
  The independent tiff reader and the paid first-party native reader inspect the real scan.
  Each profile alters observations and evaluates the exact ordered Baseline diagnostic codes.
  No profile is an authored image mutation or an instruction to rewrite carrier bytes.

  @id-classify
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Classify the <id> native observation profile
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the IO conformance profile changes the native observations
      """
      {"kind":"<id>","code":"<code>","setup":<setup>,"params":<params>}
      """
    Then the exact ordered codes match <code> and the named observation moves
    Examples:
      | id | code | setup | params |
      | set-compression | stdio.tiff.baseline.unsupported-compression | {} | {"compression":5} |
      | set-photometric-interpretation | stdio.tiff.baseline.unsupported-photometric | {} | {"photometric":6} |
      | set-bits-per-sample | stdio.tiff.baseline.unsupported-bits-per-sample | {} | {"bits":[16,16,16]} |
      | insert-tile-tags | stdio.tiff.baseline.tiled-not-baseline | {} | {"tileWidth":256,"tileLength":256} |
      | remove-tile-tags |  | {"kind": "insert-tile-tags", "params": {"tileWidth": 256, "tileLength": 256}} | {} |
      | set-strip-offsets |  | {} | {"offsets":[8,65536]} |
      | remove-strip-offsets | stdio.tiff.baseline.missing-strip-offsets | {} | {} |

