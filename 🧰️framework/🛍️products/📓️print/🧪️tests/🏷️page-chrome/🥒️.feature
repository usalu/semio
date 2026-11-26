Feature: Unified page chrome chip borders
  Running page metadata is one rectangular band whose chips sit inside it as glass overlays.

  Scenario Outline: A single shared rule is the band's outer edge
    Given the page chrome geometry in the shared language-neutral fixture
    When the <bar> is rendered
    Then its chips are anchored to the <side> shared rule inside the band
    And each chip draws only its inner edges and none on the shared rule
    And exactly one hairline closes the shared edge
    And the band is a closed rectangle whose opposite rule and two side edges are shared by its chips
    And no gap, overlap or welded seam joins a chip to the rule

    Examples:
      | bar    | side   |
      | navbar | bottom |
      | footer | top    |

  Scenario Outline: Page margins and interior have distinct opaque levels
    Given a <theme> sheet with a printable content frame
    When a cover or a running page is rendered
    Then its physical margin uses the base level
    And prose, headings, chrome bands and ordinary window and table contents share one opaque window-level background
    And the window-level surface ends at the frame boundary

    Examples:
      | theme |
      | light |
      | dark  |

  Scenario: Standalone articles retain their typesetting with shared page surfaces
    Given an article produced by the shared Print pipeline
    When it is rendered with its authored page style and geometry
    Then its physical margin is base and its full text area is window
    And the running text and page numbers belong to the interior

  Scenario: Chips are glass surfaces one step above the interior
    Given the chip surface in the shared language-neutral fixture
    When a chip is rendered with transparency enabled
    Then its fill is the glass of the level above its frame at the alpha derived from the styling tokens, under a veil of the foreground at the chip tint, so the content beneath stays visible
    And its title and number meet normal-text contrast against the composited interior
    When the transparency switch is off
    Then the same two layers are filled flattened onto the interior

  Scenario: Heading depth accents the inner side of each chip
    Given a heading at any depth
    When its title and number chips are rendered in light or dark print
    Then their fill is pane-level glass and their labels use the foreground
    And the heading-depth accent appears on the inner vertical edge

  Scenario: Window, table, figure and heading chips share one rectangle
    Given the frame geometry in the shared language-neutral fixture
    When a window, figure, table or heading is rendered
    Then the component is one rectangle with a continuous top rule
    And its title chip draws only its right and bottom edges inside the frame
    And its number chip draws only its left and bottom edges inside the frame
    And no raised cap, open-bottom tab or gap connector exists

  Scenario: Long running headings retain full-height borders
    Given a running heading wider than its available slot
    When the label is fitted
    Then only the label is resized
    And the chip sides still meet the shared border

  Scenario Outline: University logo uses the original report asset
    Given the original Leibniz University Hannover logo from Zwischenbericht
    When a <theme> document is rendered
    Then an explicitly configured footer contains the corresponding unchanged logo asset
    And the logo chip stays at the bottom left on both page sides
    And authors can override the footer logos explicitly

    Examples:
      | theme |
      | light |
      | dark  |

  Scenario Outline: Semio emblem belongs to the project chip
    Given the project label for <language> in the shared chip fixture
    When the running header is composed
    Then one border surrounds both the semio emblem and the label
    And no separate border or gap precedes the combined chip

    Examples:
      | language |
      | en       |
      | de       |

  Scenario: Empty project labels retain the semio emblem
    Given an empty project label in the shared chip fixture
    When the running header is composed
    Then one chip contains the semio emblem without an empty label gap

  Scenario Outline: Assignment header has three authored chips
    Given the explicit <language> header from the shared fixture
    When the assignment begins
    Then the left chip combines the semio emblem and course title
    And the middle chip names the exercise
    And the right chip names the topic
    And section changes do not replace these labels
    And the three zones stay in their authored left-to-right order

    Examples:
      | language |
      | en       |
      | de       |
