Feature: Centre of pressure of a footprint contact
  A language-agnostic specification; the anastruct oracle lives in 🧫️fixtures/🦶️centre-of-pressure/🔣️.json.

  Scenario: Centre of mass within half the footprint
    Given a single foot with a 0.25 m footprint carrying a body whose centre of mass is 0.035 m beside the anchor
    When the structure is solved quasi-statically
    Then the contact transmits a moment N·e, the centre of pressure lies under the centre of mass and the residual is zero

  Scenario: Centre of mass beyond half the footprint
    Given the centre of mass 0.177 m beside the anchor
    When the structure is solved
    Then the centre of pressure is clamped at the toe (0.125 m), the moment residual is non-zero and the notes say the foot tips
