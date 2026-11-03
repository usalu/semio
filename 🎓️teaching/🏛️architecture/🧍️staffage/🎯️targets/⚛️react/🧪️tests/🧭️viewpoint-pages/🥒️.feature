Feature: Compare, playground and export carry the viewpoint, and a placement has a 3D position
  A playground placement has a position along the line (x), a sideways position (z, the asset's left is +) and a yaw. Every viewpoint projects that one position through its camera.

  Scenario: A compare pane has its own viewpoint
    Given the compare page with two panes
    When the reader changes the viewpoint of the second pane
    Then the address carries it and the pane draws it
    And one button shows the same asset in the side elevation, the front elevation and the plan on one clock
    And a wrong viewpoint in the link is reported in words

  Scenario: The playground offers the viewpoint and the position
    Given the playground page
    Then the viewpoint is one radio group of side elevation, front elevation and plan
    And a focused figure moves sideways with Up and Down and turns with Page Up and Page Down
    And the sideways position and the yaw have localized fields and travel in the address
    And a yaw that cannot be drawn yet is named in words

  Scenario: Overlapping figures are depth ordered
    Given two figures at the same sideways position and different places along the line
    When the viewpoint is the front elevation
    Then they sit at the same horizontal place and the one further along the line is painted later

  Scenario: The export panel names what it exports
    Given a thermal plan at 1:50
    Then the embed snippet carries view, viewpoint and scale and the lede names the representation and the viewpoint

  Scenario: One vocabulary
    Then the app labels of the three viewpoints are the library captions in English and German
