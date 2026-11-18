Feature: Three playgrounds over one scene, each placed figure and the whole timeline playable
  The section, the floor plan and the 3D playground are tabs with addresses of their own over one shared scene; a figure placed in one is in the others at the same position.

  Scenario: The tabs and their addresses
    Given the playground page
    Then it offers Section, Floor plan and 3D at "#/playground", "#/playground/plan" and "#/playground/3d" in the reader's language
    And a figure placed in the section is at the same place in the floor plan and still there after going back

  Scenario: The section chooses between the side and the front elevation
    Given the section playground
    Then the viewpoint is one radio group of side elevation and front elevation, and the floor plan and the 3D playground offer none

  Scenario: A figure is dragged in both directions in the floor plan
    Given a placed figure in the floor plan and one metre on screen is some pixels
    When the pointer moves one metre to the right and half a metre up
    Then the figure is one metre further ahead and half a metre to its left, and a polite status says so

  Scenario: Yaw and mirroring are drawn in the floor plan
    Given a placed figure in the floor plan turned by 90 degrees and mirrored
    Then the plan turns it about its ground point, a positive yaw turning the forward direction up the page, to the left, and flips it across its forward axis
    And the page does not say the mirror is ignored

  Scenario: Every figure plays on its own
    Given two placed figures
    When the first is paused, scrubbed, played again, set not to loop, sped up and given another clip
    Then only the first changed

  Scenario: The window of a figure is adjusted on the timeline by keyboard
    Given a placed figure with its bar on the timeline
    When the reader presses Right, Shift+Right, Home on the start handle and Shift+Left, Alt+Right, End on the length handle
    Then the start moves by 0.1 s, 1 s and to 0, the length by 1 s less, 0.01 s more and to the end of the timeline, and each change is announced

  Scenario: One transport for the whole timeline
    Given a placed figure
    Then the timeline has play, step, scrub, rate and loop, and its length follows the longest window until the reader sets it

  Scenario: The page follows the catalog
    Given a figure with a clip the catalog's next revision of its asset no longer has
    When the category is loaded again
    Then the clips offered are those of the new revision and the missing pick is named while its default plays

  Scenario: The 3D playground shows every placed figure in one scene with progress and cancel
    Given two placed figures in the 3D playground
    Then it prepares one scene with a progress bar and a cancel button, shows it, offers the five camera presets and keeps the chosen one
    And cancelling stops the preparation and "Try again" prepares it again
    And an asset without a model at the chosen scale is named and no other scale stands in

  Scenario: Both languages
    Given the German reader
    Then the tabs, the timeline and the controls are named in German
