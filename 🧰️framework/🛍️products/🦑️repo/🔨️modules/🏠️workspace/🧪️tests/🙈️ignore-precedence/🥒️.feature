@capability-repo-workspace-ignore-match
@oracle-gitignore-js
@comparison-ordered-json-v1
Feature: Ignore rules are ordered, and the last matching one decides
  An ignore file is an ordered rule list: a blank line and a `#` comment are skipped, a trailing `/`
  makes a directory rule, a leading `!` negates, and a single-segment rule is lifted to every depth.
  For the vector set this repository claims git compatibility on, a gitignore-conformant matcher
  decides the verdict, never this repository. The two rule shapes that are deliberately not git
  compatible live in the sibling case `🐙️gitignore-divergence`.

  @id-vectors-are-ignored-the-same-way
  @level-fundamental
  @mode-differential
  @seed-1
  Scenario: Every rule set and path pair gets the same verdict everywhere
    Given the shared vector set shared://📡️ignore-vectors.json
    When the host asks its ignore matcher for a verdict on each pair
    Then every implementation projects the same verdict per vector
