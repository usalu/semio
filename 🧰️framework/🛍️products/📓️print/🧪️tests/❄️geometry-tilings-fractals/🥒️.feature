@capability-viz-scientific-geometry
@no-oracle-l-system
@comparison-viz-probe-v1
Feature: Substitution tilings and L-systems grow at the rate their own rules dictate
  A Penrose tiling and a Koch curve are not decoration: each is the fixed number of parts its
  substitution rule produces after n steps, and if the rule is applied wrongly the count is wrong long
  before the picture looks wrong. `sci-tiling` and `sci-fractal` therefore report their part counts and
  their lattice positions through the probe, and this case pins them.

  There is no third-party library in the print oracle registry that rewrites L-systems or substitutes
  Penrose rhombs, and the growth rates are exactly determined by the rules themselves, so the reference
  is the rule arithmetic: the P3 substitution used here replaces every rhomb with two, and the Koch
  production replaces every F with four F symbols while leaving the turn symbols in place. This is a
  `@no-oracle-l-system` conformance case with those closed forms as its specification vectors.

  @id-penrose-substitution
  @level-quick
  @mode-conformance
  Scenario: Each substitution step doubles the tile count of the ten-rhomb wheel
    Given the committed probe document local://geometry-tilings-fractals.tex and the substitution depths
      | depth | tiles |
      | 0     | 10    |
      | 1     | 20    |
      | 2     | 40    |
      | 3     | 80    |
    Then the compiled probe and the reference implementation agree on every value

  @id-l-system-words
  @level-quick
  @mode-conformance
  Scenario: Rewriting the axiom grows the word exactly as the productions say
    Given the committed probe document local://geometry-tilings-fractals.tex and the L-systems
      | system     | axiom   | production                    | depth | symbols |
      | koch       | F--F--F | F to F+F--F+F                 | 1     | 28      |
      | koch       | F--F--F | F to F+F--F+F                 | 2     | 112     |
      | koch       | F--F--F | F to F+F--F+F                 | 3     | 448     |
      | sierpinski | F-G-G   | F to F-G+F+G-F and G to GG    | 3     | 135     |
      | dragon     | F       | F to F+G and G to F-G         | 6     | 127     |
    Then the compiled probe and the reference implementation agree on every value

  @id-regular-lattice
  @level-quick
  @mode-conformance
  Scenario: A square tiling puts its cells on the lattice its own packing defines
    Given the committed probe document local://geometry-tilings-fractals.tex and the lattice
      | kind   | columns | rows | size | spacing     |
      | square | 3       | 3    | 6    | 6 times 1.732 |
    Then the compiled probe and the reference implementation agree on every value
