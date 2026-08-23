@capability-gif-raster
@oracle-gif
@comparison-semantic-raster-v1
Feature: Create and round-trip a GIF image
  GIF is palette-based, so a colour quantizer's exact output is a writer choice and the projection
  deliberately reports frame geometry and opaque-sample counts rather than pretending exact RGBA
  survives quantization. What IS normative is that this repository's decode and re-encode preserve
  the logical screen, the frame layout and the frame count.

  @id-single-frame-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A single-frame image survives decode and re-encode
    Given the image
    """
    { "width": 4, "height": 4 }
    """
    When the image is written and read back
    Then the frame geometry and frame count are unchanged

  @id-non-square-frame-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A non-square frame survives decode and re-encode
    Given the image
    """
    { "width": 6, "height": 2 }
    """
    When the image is written and read back
    Then the frame geometry and frame count are unchanged
