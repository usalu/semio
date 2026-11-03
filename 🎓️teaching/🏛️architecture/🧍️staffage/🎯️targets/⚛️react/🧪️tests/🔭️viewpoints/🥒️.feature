Feature: Side elevation, front elevation and plan are three views of one figure
  The viewpoint selector offers the side elevation, the front elevation, the plan (top view) and the three views together. They project one evaluated instant at one scale in one representation, so switching never changes pose, time, playback, seed, environment or scale.

  Scenario: The selector is an accessible radio group in two languages
    Given the language is English or German
    Then the group is "Viewpoint" or "Blickrichtung"
    And the choices are "Side elevation", "Front elevation", "Plan (top view)" and "Three views", or "Seitenansicht", "Vorderansicht", "Draufsicht" and "Drei Ansichten"
    And the arrow keys move and select, with one tab stop and one polite live region

  Scenario: Switching writes only the camera
    Given a viewer at 1:50 in the thermal representation on a clip at 0.5 s
    When the reader picks the plan, the three views and the front elevation
    Then only the viewpoint and the layout change and the time stays 0.5 s

  Scenario: The three views are synchronized
    Given the three-view sheet on one clock
    When the clock is scrubbed
    Then all three figures show the same instant and the zoom does not change
    And the plan is above the side elevation and the front elevation is to its right, with the tab order plan, side, front

  Scenario: Nothing is substituted
    Given an asset without a plan, or a scale the asset does not draw
    Then the cell names the viewpoint, gives the reason from the resolver and says no other view is shown
    And the other cells keep their place

  Scenario: The address carries the viewpoint
    Given a viewer, a compare pane, a playground and a link with an unknown viewpoint
    Then vp and vl round-trip, each compare pane carries its viewpoint, and the unknown value is reported in words

  Scenario: Mirroring is the side elevation only
    Given a mirrored placement
    Then it is mirrored in the side elevation and drawn unmirrored in front elevation and plan

  Scenario: The element and the files name the viewpoint
    Given the custom element and the download names
    Then viewpoint is required, an unknown value shows an alert, and every file name contains the viewpoint
