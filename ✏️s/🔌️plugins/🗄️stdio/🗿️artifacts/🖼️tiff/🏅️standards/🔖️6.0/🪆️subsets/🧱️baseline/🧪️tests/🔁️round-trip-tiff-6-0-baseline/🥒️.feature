@capability-tiff-6-0-baseline-round-trip
@oracle-tiff-tiff-6-0-baseline-native-reader
@comparison-ordered-json-v1
Feature: Preserve canonical TIFF image identities through native export
  Both roles decode the scan and independently re-encode its interpretation.
  The independent TIFF reader must see the same metadata and exact raster samples.
  First-party storage choices are ephemeral IO policy.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Export exact owned samples and reopen them independently
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the scan is decoded to canonical entries and sample-word blocks and exported
    Then the owned image and independent semantic projection are preserved and native Baseline conformance is clean
