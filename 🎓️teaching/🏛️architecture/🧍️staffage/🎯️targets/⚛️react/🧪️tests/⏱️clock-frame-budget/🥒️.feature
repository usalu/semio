Feature: a running clock costs no React render per frame
  Scenario: placed figures share one clock subscription, step only while visible and never re-render
    Given 40 placed figures on one clock
    When the clock runs for ten frames
    Then the clock has one subscriber and no figure is re-rendered
    And a figure that is not visible keeps its last frame until it is visible again

  Scenario: displayed frames repeat with the clip
    Given a one-second loop driven at twelve frames per second
    When the first fractional frame returns fifty cycles later
    Then the figure reuses the drawing of that fractional frame

  Scenario: Decorative archive previews update only their artwork
    Given a live row preview in an inert card illustration
    When the ambient clock steps to another frame
    Then the SVG depicts the new frame without a React commit
    And the illustration stays hidden from assistive technology

  Scenario: Ambient drawing has a bounded slice of each display frame
    Given three animated figures with drawing work above one frame budget
    When the clock advances before all three are painted
    Then the clock returns without drawing
    And each display frame paints the next figure at the latest clock time
    And removing the last figure cancels the remaining work

  Scenario: A drawing projects the views it uses
    Given a moving asset with a side elevation and a front view
    When the preview draws only its side elevation
    Then the side elevation is identical to the drawing made by full evaluation
    And the front view is left unevaluated

  Scenario: Expensive preview frames are produced by a worker
    Given two figures share one worker with the first figure drawing
    When the second figure requests a frame and the first figure advances twice
    Then the first in-progress frame is shown and the second figure gets its turn
    And the worker computes only the latest pending frame of the first figure
    And releasing either figure prevents its later result from painting
