Feature: Independent Authored TIFF Mutation Recipes
  Scenario: Every current authored mutation has a genuine TIFF recipe
    Given the six neutral owned mutation recipe identities
    When tiff generates and independently decodes each native before and after image
    Then InsertIfd, RemoveIfd, ReplaceTag, RemoveTag, PaintRegion and ReplaceSamples have observable authored differences
    And PaintRegion changes only pixel 1,1 to RGB 9,8,7
    And ReplaceSamples changes only the first three owned words to 24,96,192
