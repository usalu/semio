Feature: Cantilever structural solution
  A language-agnostic specification of the cantilever case; every implementation must reproduce the anastruct oracle in 🧫️fixtures/🏗️cantilever/🔣️.json.

  Scenario: Wall reaction and bending of a tip-loaded cantilever
    Given a 2 m steel beam clamped at x = 0 with a fixed support, self-weight off
    And a 1000 N downward point load at the tip
    When the structure is solved quasi-statically
    Then the wall reaction is +1000 N vertical and +2000 N·m counter-clockwise, with no horizontal force
    And the bending moment is -1000 · (2 - x) N·m (hogging) and the shear is constant +1000 N between wall and tip
    And the tip deflection equals P·L³ / (3·E·I) downward within 0.5 % of the anastruct value
    And the residual force and moment are below 1e-9 relative
    And at least 24 diagram stations are reported and the notes state "quasi-static" in English and German
