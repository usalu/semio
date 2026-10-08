Feature: Terminal Text Width Model
  The cell grid must measure and cut text the way a Unicode terminal draws it, including the emoji
  path segments of this repository such as 🧰️framework and 🖥️platform.

  Scenario: A list of strings measures the same as the third party oracles
    Given the shared width fixture of strings with their grapheme clusters and cell widths
    When the cluster model segments and measures every string
    Then the clusters and cell widths equal the fixture
    And the unicode-segmentation and unicode-width oracles produce the same clusters and cell widths

  Scenario: Every scalar agrees with the oracle
    Given every Unicode scalar value
    When the model measures it alone and in context before and after other scalars
    Then its width and its grapheme boundaries equal the oracle result

  Scenario: Emoji presentation changes the width of the base character
    Given a text-default pictograph such as the desktop computer
    When the variation selector 16 follows it
    Then the cluster is two cells wide
    And a variation selector 15 on a wide emoji makes it one cell wide

  Scenario: Wide clusters occupy a lead cell and continuation cells
    Given a cell buffer and a string with wide clusters and a ZWJ sequence
    When the string is written and a narrow glyph overwrites half of a wide cluster
    Then the other half is blanked and no orphaned continuation cell remains

  Scenario: Ellipsis keeps the discriminating end of a name
    Given a name longer than its budget
    When it is elided in the middle with a tail of six cells
    Then the result fits the budget, never splits a cluster and keeps the tail
