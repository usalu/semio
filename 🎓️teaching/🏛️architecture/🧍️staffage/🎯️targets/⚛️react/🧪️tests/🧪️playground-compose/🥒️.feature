Feature: The playground composes figures at true paper offsets
  Scenario Outline: A placement lands at margin plus its metres times millimetres per metre minus its origin
    Given a composition 10 m long drawn at 1:<scale> with a 20 mm wide part whose ground point is 10 mm from its left edge
    When the part is placed <metres> m from the left end of the ground
    Then the exported SVG puts the part's left edge at <left> mm from the left of the paper
    And an independent rasteriser at one pixel per millimetre shows the drawn pixels starting at column <left>

    Examples:
      | scale | metres | left |
      | 100   | 2      | 18   |
      | 50    | 3      | 58   |

  Scenario: A mirrored placement flips around its own ground point
    Given a part drawn in its left five millimetres placed 2 m along at 1:100
    When it is mirrored
    Then its drawn pixels start at column 33 instead of 18

  Scenario: The file names what it contains
    Given a composition of two assets at 1:100 in the light line view
    Then its title contains "1:100" and its metadata lists both asset ids, the scale and the view

  Scenario: Placement events keep figures on the ground and survive a reload
    Given an empty playground
    When a figure is added, moved beyond the ground's end, moved below its start, duplicated and one copy removed
    Then positions stay between 0 and the ground length
    And a reloaded store holds the same composition, and the address round trip keeps scale, view, pick, offset and mirroring
