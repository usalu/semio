@capability-version-requirement-error-law
@no-oracle-frozen-requirement-grammar
@comparison-ordered-json-v1
Feature: Reject malformed version input without raising
  Outside the frozen requirement grammar the contract is this repository's own: "unsatisfied, never a
  throw". No third-party matcher can adjudicate that — `semver` throws on some of these inputs and
  coerces others, which is a different, equally valid contract — so this is a recorded no-oracle case
  specified by the vectors below.

  @id-malformed-input-is-unsatisfied-never-a-throw
  @level-quick
  @mode-error
  Scenario: Malformed input is unsatisfied, and never throws
    Given the malformed pairs
      | version | requirement |
      | not-a-version | ^1.2.3  |
      |         | ^1.2.3        |
      | 1.2.3   | not-a-requirement |
      | 1.2.3   |               |
      | 1.2     | ^1.2.3        |
      | 1.2.3.4 | ^1.2.3        |
    Then every pair reports unsatisfied without raising
