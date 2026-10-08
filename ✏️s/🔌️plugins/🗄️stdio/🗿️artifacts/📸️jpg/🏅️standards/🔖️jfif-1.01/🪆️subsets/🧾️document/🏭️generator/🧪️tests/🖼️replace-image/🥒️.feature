Feature: Independent Whole-Image Replacement Recipe
  Scenario: Native JPEG replacement changes authored image content
    Given the neutral replace-image-applied recipe
    When image-rs generates and independently reads both JPEG files
    And Pillow independently reads their JFIF metadata
    Then the base dimensions are 32x24 and replacement dimensions are 16x8
    And the replacement has 384 decoded RGB bytes with a changed raster digest
    And replacement XMP is present and JFIF density is 300x150 dots per inch
