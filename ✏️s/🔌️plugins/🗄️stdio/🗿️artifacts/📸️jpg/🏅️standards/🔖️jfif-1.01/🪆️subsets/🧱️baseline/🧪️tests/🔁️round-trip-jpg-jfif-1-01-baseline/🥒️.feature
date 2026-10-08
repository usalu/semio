@capability-jpg-jfif-1-01-baseline-round-trip
@oracle-libjpeg-jpg-jfif-1-01-baseline-marker-cli
@comparison-ordered-json-v1
Feature: Reopen an owned photographic image exported as baseline JPEG
  Physical export options derive JPEG markers from owned pixels and metadata.
  An independent image encoder exports the same input; libjpeg reads its native conformance facts.
  Both results must remain baseline-conforming with the same owned geometry and JFIF metadata.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Independently encode and reopen the real photographic image
    Given the real input document shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When owned image content is exported using explicit physical JPEG options
    Then both independent native readers admit baseline conformance and the image geometry is preserved
