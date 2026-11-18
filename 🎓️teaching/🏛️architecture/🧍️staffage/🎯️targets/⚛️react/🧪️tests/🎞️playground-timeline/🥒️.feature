Feature: The playgrounds share one scene and one timeline
  A placement has one position (x ahead, z to the asset's left), a turn, a pose or clip and a window on one timeline. The section, the floor plan and the 3D playground read that one state, so an instant is the same instant in all three.

  Scenario Outline: Where a placement is in its clip
    Given a clip of <clip> seconds on a timeline of <timeline> seconds
    And a placement with start <start>, length <length>, speed <speed>, offset <offset>, loop <loop> and hold <hold>
    When the timeline is at <time> seconds
    Then the clip time drawn is <drawn> and the placement is <phase>

    Examples:
      | clip | timeline | start | length | speed | offset | loop | hold | time | drawn | phase    |
      | 1    | 10       | 0     | end    | 1     | 0      | yes  | none | 0.25 | 0.25  | running  |
      | 1    | 10       | 2     | end    | 1     | 0      | yes  | none | 1    | 0     | waiting  |
      | 1    | 10       | 1     | 2      | 1     | 0      | yes  | none | 5    | 2     | finished |
      | 1    | 10       | 0     | end    | 2     | 0      | yes  | none | 0.75 | 1.5   | running  |
      | 1    | 10       | 0     | end    | 1     | 0      | no   | none | 3    | 1     | running  |
      | 1    | 10       | 0     | end    | 1     | 0      | yes  | 0.4  | 7    | 0.4   | held     |

  Scenario: The timeline is as long as the longest window unless the reader sets it
    Given three placements with different starts, lengths and speeds
    Then the timeline is the longest window, and a length the reader sets replaces it

  Scenario: The clip is laid on the 3D animation exactly as the 2D reading says
    Given a looping, a held-at-the-end and a delayed window
    When three interpolates the original clip at the time the rules read and the retimed channel at the timeline instant
    Then both give the same value at every instant

  Scenario: The 3D scene shows the figure the evaluator evaluates
    Given a walking figure at x 2 m and z 1 m and a standing figure turned by 90 degrees
    Then the composed document is valid, with one scene per mode and one master animation
    And the walker stands on its ground point at (z, 0, x) of the neutral frame at every instant, at the height the evaluator gives
    And a positive yaw turns the forward direction to the asset's left

  Scenario: One position in three playgrounds
    Given a placement at x 3 m, z 1.5 m
    Then the side elevation puts it at 3 m, the front elevation at the depth's middle plus 1.5 m, the plan at 3 m and 1.5 m up the page, and the 3D scene at (1.5, 0, 3)
    And a drag moves it along x in the side elevation, along z in the front elevation and in both in the plan

  Scenario: The scene and the playground are in the address
    Given a scene with a window, a speed, a hold and a camera
    Then #/playground, #/playground/plan and #/playground/3d are the section, the floor plan and the 3D playground
    And the scene survives the address round trip
