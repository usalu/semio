@capability-repo-workspace-glob-match
@oracle-micromatch
@comparison-ordered-json-v1
Feature: The owned glob matcher agrees with a real glob engine
  Every path decision in the repository — ignore rules, codebase traversal, statute scopes — runs
  through one owned matcher, so its meaning is load bearing. `**` spans separators, `*` and `?` stop
  at one, a character class may carry ranges, and a negated class is a literal set. What each pattern
  MEANS is decided by micromatch, never by this repository.

  @id-vectors-match-the-same-way
  @level-fundamental
  @mode-differential
  @seed-1
  Scenario: Every pattern and path pair gets the same verdict everywhere
    Given the shared vector set shared://📡️glob-vectors.json
    When the host asks its matcher for a verdict on each pair
    Then every implementation projects the same verdict per vector

  @id-brace-alternation-expands-the-same-way
  @level-fundamental
  @mode-differential
  @seed-1
  Scenario: A brace group stands for the patterns it expands to
    Given the brace vector set shared://📡️glob-vectors.json
    When the host asks its matcher for a verdict on each brace pair
    Then every implementation projects the same verdict per brace vector
