@capability-png-raster
@oracle-png
@comparison-semantic-raster-v1
Feature: Create and round-trip a PNG image
  The reference implementation writes an RGBA image; this repository decodes that artifact and
  re-encodes it. Both results are read back by the INDEPENDENT decoder before the
  `semantic-raster-v1` profile compares them.

  Filter selection, interlacing, chunk order, compression level, gamma and every ancillary chunk are
  encoder choices and are canonicalized away. Dimensions, colour model, bit depth and the decoded
  samples are normative — a round trip that loses one pixel is a failure.

  @id-rgba-gradient-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A small RGBA gradient survives decode and re-encode
    Given the image
    """
    { "width": 4, "height": 4 }
    """
    When the image is written and read back
    Then every decoded sample is unchanged

  @id-non-square-image-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A non-square image survives decode and re-encode
    Given the image
    """
    { "width": 7, "height": 3 }
    """
    When the image is written and read back
    Then every decoded sample is unchanged

  @id-single-pixel-round-trips
  @level-fundamental
  @mode-round-trip
  Scenario: A one-pixel image survives decode and re-encode
    Given the image
    """
    { "width": 1, "height": 1 }
    """
    When the image is written and read back
    Then every decoded sample is unchanged
