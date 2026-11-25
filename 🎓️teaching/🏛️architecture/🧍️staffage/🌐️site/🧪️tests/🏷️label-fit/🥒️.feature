Feature: Labels fit their controls on the page of an asset
  Scenario Outline: Controls at <width> px in <language>
    Given the built site in a browser window <width> px wide that chose <language>
    When the page of the walking commuter is open with its clip held at 0.5 s
    Then no label of a button, segment, link or summary reaches outside its control
    And no segment of a choice lies over another segment or outside its choice
    And no control reaches outside the window and the page does not scroll sideways
    And on a touch screen every button, segment, summary and select is at least 44 px high
    And every text of the drawing, its caption with the held time included, lies inside the drawing

    Examples:
      | width | language |
      | 320   | English  |
      | 320   | Deutsch  |
      | 768   | English  |
      | 768   | Deutsch  |
      | 1024  | English  |
      | 1024  | Deutsch  |
      | 1440  | English  |
      | 1440  | Deutsch  |
