Feature: Level of detail
  Small scales draw fewer shapes.

  Scenario: texture at 1:500
    Given a figure with a texture shape
    When it is drawn at 1:500
    Then no texture item is present
    But at 1:200 the texture item is present

  Scenario: lod thresholds
    Given details of lod 1 and lod 3
    Then the lod 1 detail appears from 1:200 and the lod 3 detail only at 1:50
