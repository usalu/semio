@capability-viz-showcase-families
@no-oracle-showcase-families
@comparison-viz-probe-v1
Feature: Every showcase capability kind draws geometry of its own
  Taxonomy sections 74 to 79 do not enumerate charts. They enumerate the capabilities the library
  claims to have: the fifty-four namespaces of section 76, the thirty-two encoding channels of
  section 74, the twenty-three grammar elements of section 79, the twelve figure facilities of
  section 78, and the scales, shapes, layout algorithms and transforms the same sections list. A
  catalogue entry for one of them is a promise that the library can show that capability, and the
  cheapest way to break the promise is to render every entry of a family as the same figure with a
  different caption.

  `semio-viz-showcase` owns the nine families behind those 163 catalogue kinds, and this case holds
  them to the promise. There is no third-party library that draws a taxonomy of print
  visualization capabilities, so the comparison is a conformance test against the catalogue
  itself: every kind the catalogue registers for these families must emit geometry through the
  probe, and no two kinds of one family may emit the same geometry. A kind that quietly reuses
  another kind's drawing is caught by the distinctness count and not by a reviewer paging through
  163 figures.

  Both scenarios sit at the long level: a probe document that draws a hundred figures spends more
  than the quick budget in the typesetter alone, and the namespace document loads the whole library
  on top of that, because the namespace family runs the canonical family of every namespace it
  names. They are split because the two documents load different halves of the library, not
  because one of them is cheap.

  @id-capabilities
  @level-long
  @mode-conformance
  Scenario: The 109 capability kinds emit geometry and are pairwise distinct inside their family
    Given the committed probe document local://showcase-capabilities.tex and the showcase families
      | family                | kinds |
      | encoding              | 32    |
      | grammar               | 23    |
      | figure                | 12    |
      | scale                 | 15    |
      | shape                 | 5     |
      | layout-algorithm      | 7     |
      | transform-data        | 9     |
      | transform-statistical | 6     |
    Then the compiled probe and the reference implementation agree on every value

  @id-namespaces
  @level-long
  @mode-conformance
  Scenario: The 54 namespace kinds each render their own namespace's canonical family
    Given the committed probe document local://showcase-namespaces.tex and the showcase families
      | family    | kinds |
      | namespace | 54    |
    Then the compiled probe and the reference implementation agree on every value
