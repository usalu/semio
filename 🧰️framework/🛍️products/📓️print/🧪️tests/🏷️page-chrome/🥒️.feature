Feature: Unified page chrome chip borders
  Running page metadata uses the same joined chip and border design as chapters.

  Scenario Outline: A single shared edge closes the metadata chips
    Given the page chrome geometry in the shared language-neutral fixture
    When the <bar> is rendered
    Then its chips are open toward the <side> shared border
    And the chip sides meet that border without a gap
    And exactly one hairline closes the shared edge

    Examples:
      | bar    | side   |
      | navbar | bottom |
      | footer | top    |

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
