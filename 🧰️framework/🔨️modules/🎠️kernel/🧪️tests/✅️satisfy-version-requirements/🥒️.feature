@capability-version-requirement-satisfaction
@oracle-semver
@comparison-ordered-json-v1
Feature: Decide whether a version satisfies an exact dependency pin
  A manifest dependency is an exact pin `=X.Y.Z` and nothing else: one tree is one catalog, and a trusted
  catalog admits only exact pins inside its closure. `=X.Y.Z` is also a published semver comparator, so
  `semver` is a genuine oracle for it: for every pin, the two must agree on every version.

  Ranges (`*`, `^`, `~`, `>=`, a bare triple) are outside the pin grammar and refused by design — the
  refusal vectors live in `🚫️reject-malformed-version-input`, not here, because `semver` would satisfy
  them and asserting that would be measuring a deliberate divergence, not a bug.

  @id-exact-pins
  @level-fundamental
  @mode-differential
  Scenario: The exact pin across every component
    Given the version and requirement pairs
      | version | requirement |
      | 1.2.3   | =1.2.3      |
      | 1.2.4   | =1.2.3      |
      | 1.2.2   | =1.2.3      |
      | 1.3.3   | =1.2.3      |
      | 2.2.3   | =1.2.3      |
      | 0.1.0   | =0.1.0      |
      | 0.1.1   | =0.1.0      |
      | 0.0.0   | =0.0.0      |
      | 10.20.30 | =10.20.30  |
    Then the reference implementation and this repository agree on every pair
