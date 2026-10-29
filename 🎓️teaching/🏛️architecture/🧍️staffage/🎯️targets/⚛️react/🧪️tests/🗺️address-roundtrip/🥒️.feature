Feature: Shareable addresses
  Every meaningful state of the archive is in its address, so a link reproduces it.

  Scenario: A route survives a round trip
    Given the address "#/humans/walking-commuter"
    When it is parsed and formatted again
    Then the address is unchanged

  Scenario: A viewer state survives a round trip
    Given a viewer showing the thermal view at 1:50 on the clip "walk" at 1.25 s, zoomed 2.5 times and panned
    When its parameters are written and read again
    Then every setting, including the environment overrides, is equal

  Scenario: Defaults write nothing
    Given the default viewer
    Then its address query is empty

  Scenario: Unreadable values fall back
    Given the query "view=bogus&scale=-3&zoom=nope"
    Then the viewer uses the default view, scale 100 and fitted zoom

  Scenario: A comparison keeps four panes
    Given four panes with different assets, views and poses or clips
    When written and read again
    Then the four panes are equal

  Scenario: A playground layout keeps positions on the ground
    Given a layout whose bench lies beyond the ground length
    Then it is clamped to the end of the ground
