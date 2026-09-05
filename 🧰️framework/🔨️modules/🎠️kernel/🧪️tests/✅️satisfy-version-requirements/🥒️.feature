@capability-version-requirement-satisfaction
@oracle-semver
@comparison-ordered-json-v1
Feature: Decide whether a version satisfies a requirement
  `versionSatisfies` implements a deliberate SUBSET of the semver requirement grammar — `*`,
  `=X.Y.Z`, `^X.Y.Z`, `~X.Y.Z`, `>=X.Y.Z` — with the standard leading-zero caret tiers. Because that
  subset is a published contract rather than a repository invention, `semver` is a genuine oracle for
  it: for every requirement inside the subset, the two must agree on every version.

  The vectors stay inside the subset on purpose. Ranges, prerelease tags, `x` wildcards, build
  metadata and whitespace-separated comparators are outside the frozen grammar and are rejected by
  design; asserting those against `semver` would be measuring a deliberate divergence, not a bug.

  @id-exact-and-any
  @level-fundamental
  @mode-differential
  Scenario: The `*` and `=` requirements
    Given the version and requirement pairs
      | version | requirement |
      | 1.2.3   | *           |
      | 0.0.0   | *           |
      | 1.2.3   | =1.2.3      |
      | 1.2.4   | =1.2.3      |
      | 1.2.2   | =1.2.3      |
      | 2.0.0   | =1.2.3      |
    Then the reference implementation and this repository agree on every pair

  @id-caret-tiers
  @level-fundamental
  @mode-differential
  Scenario: Caret across the major, minor and patch tiers
    Given the version and requirement pairs
      | version | requirement |
      | 1.2.3   | ^1.2.3      |
      | 1.9.9   | ^1.2.3      |
      | 2.0.0   | ^1.2.3      |
      | 1.2.2   | ^1.2.3      |
      | 0.2.3   | ^0.2.3      |
      | 0.2.9   | ^0.2.3      |
      | 0.3.0   | ^0.2.3      |
      | 0.2.2   | ^0.2.3      |
      | 0.0.3   | ^0.0.3      |
      | 0.0.4   | ^0.0.3      |
      | 0.0.2   | ^0.0.3      |
      | 0.1.0   | ^0.0.3      |
    Then the reference implementation and this repository agree on every pair

  @id-tilde-and-at-least
  @level-fundamental
  @mode-differential
  Scenario: Tilde and at-least
    Given the version and requirement pairs
      | version | requirement |
      | 1.2.3   | ~1.2.3      |
      | 1.2.9   | ~1.2.3      |
      | 1.3.0   | ~1.2.3      |
      | 1.2.2   | ~1.2.3      |
      | 0.0.0   | ~0.0.0      |
      | 1.2.3   | >=1.2.3     |
      | 9.9.9   | >=1.2.3     |
      | 1.2.2   | >=1.2.3     |
      | 0.0.1   | >=0.0.0     |
    Then the reference implementation and this repository agree on every pair
