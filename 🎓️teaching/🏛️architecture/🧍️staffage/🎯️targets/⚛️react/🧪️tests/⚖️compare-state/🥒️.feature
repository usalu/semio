Feature: Comparison state is shareable and synchronized
  Scenario: Four panes survive the address
    Given a comparison of four panes at 1:50 paused at 1.25 s
    When the address is written and read again
    Then the four panes, the scale and the time are identical

  Scenario: Unknown values never break the page
    Given an address with an unknown view, an invalid asset id and six panes
    When the comparison is read
    Then unknown views fall back to the default view, invalid panes are dropped and at most four panes remain

  Scenario: One clock drives every pane
    Given two panes on one clock of 2 seconds at 30 frames per second
    When the clock seeks to 0.5 s and steps three frames
    Then both panes show frame 18
