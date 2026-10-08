Feature: TIFF Baseline Native Conformance
  Native storage observations belong to IO and never enter an authored snapshot.

  Scenario Outline: Classify each neutral observation profile
    Given the owned neutral base observations and the case patch
    When the native baseline classifier evaluates the profile
    Then the exact ordered diagnostic codes match the independent TIFF reference tables

    Examples:
      | id |
      | baseline |
      | no-raster |
      | lzw |
      | ycbcr |
      | sixteen-bit |
      | tiles |
      | no-offsets |
