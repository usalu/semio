Feature: Frames share their borders
  Every bordered element of a page is a closed rectangle, and two neighbours share the edge between them.

  Scenario Outline: A frame that directly follows another shares its edge
    Given the junction <case> in the shared language-neutral fixture
    When the second frame opens while the first frame's record is pending
    Then the glue the first frame left below itself is cancelled
    And the second frame moves up by exactly one hairline
    And the shared edge is one hairline in the neutral border colour, whatever accent either frame has

  Scenario: Only vertical edges carry an accent colour
    Given a frame with an accent colour in the shared language-neutral fixture
    When its edges are drawn
    Then its horizontal edges, shared with the sections above and below, are neutral
    And its vertical edges may carry its accent colour

    Examples:
      | case               |
      | heading to heading |
      | heading to window  |
      | window to window   |
      | window to heading  |
      | list to heading    |

  Scenario: A paragraph or authored space keeps frames apart
    Given a closed frame in the shared language-neutral fixture
    When a paragraph begins or vertical space is authored
    Then the record is released and the next frame keeps its ordinary separation

  Scenario: Heading chips are in-content
    Given a heading in the shared language-neutral fixture
    When it is rendered
    Then only the top edge of its section frame is drawn across the column
    And its title chip draws its right and bottom edges and its number chip its left and bottom edges
    And no rule runs below the chip row

  Scenario: The text column is a closed frame
    Given the page geometry in the shared language-neutral fixture
    When a page with running bars is shipped
    Then the navbar rule is the top edge of the text area, overlapping it by one hairline
    And the footer rule closes the column one body padding below the text area, so prose never touches it
    And a window part that breaks at the page end is drawn across that padding onto the footer rule
    And two side borders run from the navbar rule to the footer rule
    And prose is inset from them by the window body padding while frames span the column

  Scenario: A long table that breaks across pages stays open at the break
    Given a long table in the shared language-neutral fixture
    When it breaks across pages
    Then the rule below its last row on the page is taken back, so the row runs down to the footer rule
    And the table's real last row keeps its closing rule

  Scenario: A list is split into one cell per point
    Given a list in the shared language-neutral fixture
    When it is rendered
    Then one hairline opens the list, one separates each pair of neighbouring points and one closes it

  Scenario: Neighbouring tiles of a row or column share their border
    Given a window arrangement in the shared language-neutral fixture
    When its tiles are placed
    Then consecutive tiles overlap by exactly one hairline
