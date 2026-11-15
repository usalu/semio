Feature: Scale and mode of the 3D model are one choice with the 2D drawing
  Scenario: Scales and their reasons
    Given an asset with a model at 1:50, 1:100 and 1:200 and none at 1:500
    Then the four preset scales are offered in order, the missing one is marked unavailable with the reason of its representation in English and German, and without such a reason with the scales that exist
  Scenario: The controls of the stage
    When the stage shows a model
    Then a group of scale buttons and a group of the seven modes sit in the toolbar, the current choice is pressed, a click on a scale or mode selects the scene of the scale and mode and reports it to the host, and an unavailable scale is announced with its reason and changes nothing
  Scenario: Keyboard
    When focus is in one of the two groups
    Then the arrow keys, Home and End move the focus over the buttons of the group (an unavailable scale stays focusable so that its reason can be read), and Enter or Space chooses
  Scenario: Announcement
    When the scale or the mode changes
    Then the viewer's polite status names the scale, the mode of the scene and the representation in the reader's language
  Scenario: One scale and one view for 2D and 3D
    Given the viewer of an asset in the 3D model viewpoint
    When a scale or mode is chosen in the stage or in the controls of the viewer
    Then both show the same choice, the same patch reaches the address, and the 2D drawing of the next viewpoint is drawn at that scale and view
  Scenario: The custom element
    When the stage of a staffage-figure element chooses a scale or a mode
    Then the attributes scale and view follow and the events staffage-scale and staffage-view fire
  Scenario: Both languages are complete
    Then every new text exists in English and German with the same placeholders
