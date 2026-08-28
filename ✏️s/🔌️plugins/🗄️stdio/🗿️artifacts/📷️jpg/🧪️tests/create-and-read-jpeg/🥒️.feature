@capability-jpg-raster
@oracle-image-jpeg
@comparison-semantic-raster-v1
Feature: Create and read a JPEG image
  The reference implementation writes the image; this repository decodes that artifact and re-encodes
  it. Both byte streams are read back by the INDEPENDENT decoder before the profile compares them.
  Encoder-specific choices — row order, compression scheme, ancillary tags — are canonicalized away.

  JPEG is LOSSY. Asserting byte-equal samples through it would be a test that can only pass by
  accident, so the projection reports geometry and a coarse luma histogram — what the codec actually
  preserves — and the round trip is judged on that.

  @id-gradient-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A small gradient survives decode and re-encode
    Given the image
    """
    { "width": 4, "height": 4 }
    """
    When the image is written and read back
    Then the recovered image matches the reference

  @id-non-square-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A non-square image survives decode and re-encode
    Given the image
    """
    { "width": 8, "height": 4 }
    """
    When the image is written and read back
    Then the recovered image matches the reference
