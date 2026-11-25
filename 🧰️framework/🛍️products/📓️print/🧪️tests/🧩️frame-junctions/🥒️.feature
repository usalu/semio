Feature: Frames share their borders
  Every bordered element of a page is a closed rectangle, and two neighbours share the edge between them.

  Scenario Outline: A frame that directly follows another shares its edge
    Given the junction <case> in the shared language-neutral fixture
    When the second frame opens while the first frame's record is pending
    Then the glue the first frame left below itself is cancelled
    And the second frame moves up by exactly one hairline
    And the shared edge is one hairline in the neutral border colour, whatever accent either frame has

    Examples:
      | case                      |
      | heading to heading        |
      | heading to window         |
      | window to window          |
      | window to heading         |
      | list to heading           |
      | chapter to kind window    |
      | heading to list           |
      | stretched list to heading |

  Scenario: Only vertical edges carry an accent colour
    Given a frame with an accent colour in the shared language-neutral fixture
    When its edges are drawn
    Then its horizontal edges, shared with the sections above and below, are neutral
    And its vertical edges may carry its accent colour

  Scenario: A breakable window after a heading opens without an empty paragraph
    Given a heading followed by a numbered window in the shared language-neutral fixture
    When the window is breakable and the heading forbids a break before it
    Then the window still opens on the heading's bottom edge

  Scenario: A list that directly follows a frame opens as a frame
    Given a heading followed by a list in the shared language-neutral fixture
    When the list opens
    Then its opening line is the heading's bottom edge
    And its first cell is padded below that line like every following cell

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

  Scenario Outline: A frame that ends a page closes on the footer rule
    Given the edge sample <sample> of a window, table or list of the column in the shared language-neutral fixture
    When its bottom edge is shipped within the edge reach above the bottom of the text area
    Then the next pass keeps the edge's glue but draws no hairline, so the footer rule closes the frame
    And this holds whether the page breaks directly below it, later at a heading, or by force, and whatever anchors, writes or colour changes follow it
    And when no break follows, its bottom edge is drawn exactly where it was
    And when it ends higher on a page that is ended by force, it keeps its bottom edge

    Examples:
      | sample                               |
      | edge on the text bottom              |
      | edge below the text bottom           |
      | edge just inside the reach           |
      | edge just outside the reach          |
      | edge higher on a page ended by force |

  Scenario: An arrangement as tall as the text area fills its page
    Given a window arrangement of the text height in the shared language-neutral fixture
    When it is placed
    Then it occupies exactly the text height
    And it is drawn one body padding taller, so its bottom edge and its tile dividers run onto the footer rule

  Scenario: A list is split into one cell per point
    Given a list in the shared language-neutral fixture
    When it is rendered
    Then one hairline opens the list, one separates each pair of neighbouring points and one closes it
    And a page that breaks between two points drops the hairline at the break, so the cell above closes on the footer rule

  Scenario: Neighbouring tiles of a row or column share their border
    Given a window arrangement in the shared language-neutral fixture
    When its tiles are placed
    Then consecutive tiles overlap by exactly one hairline
