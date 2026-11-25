Feature: World inspection uses one legend and attribution rail
  Scenario: Terrain and a valid scalar field share the lower corner
    Given a world with terrain attribution and a scalar field that matches its mesh
    Then the legend and the attribution are cells in one pane-level glass rail
    And the source remains a keyboard-accessible link

  Scenario: A field mismatch uses the same space as the legend
    Given a world with terrain attribution and a scalar field that does not match its mesh
    Then the rail announces the mismatch with the source link present
    And the legend is absent
