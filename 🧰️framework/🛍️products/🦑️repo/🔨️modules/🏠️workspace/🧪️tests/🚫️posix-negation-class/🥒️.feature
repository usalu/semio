@capability-repo-workspace-glob-divergence
@no-oracle-repo-workspace-glob-divergence
@comparison-ordered-json-v1
Feature: The POSIX character-class negation spelling is accepted, unlike in picomatch
  The owned matcher accepts `[!…]` as a negated character class, the POSIX spelling. picomatch — and
  therefore micromatch — reads that as a literal class containing an exclamation mark, so a
  micromatch-based oracle disagrees by construction and this case has none. Inside a negated class
  every member is literal: `[!a-c]` excludes `a`, `-` and `c`, not the range. See the recorded
  decision `repo-workspace-glob-divergence`.

  @id-the-posix-negation-spelling-is-accepted
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: The recorded divergences behave as recorded
    Given the shared vector set shared://📡️glob-vectors.json
    When the host asks its matcher for a verdict on each recorded divergence
    Then every implementation projects the recorded owned verdict, not the picomatch verdict
