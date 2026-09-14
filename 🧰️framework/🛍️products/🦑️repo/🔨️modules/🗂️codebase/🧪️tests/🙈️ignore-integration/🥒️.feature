@capability-repo.codebase.ignore-integration
@no-oracle-repo-codebase-ignore-integration
@comparison-ordered-json-v1
Feature: A path is excluded from the codebase for the same reason everywhere
  Three independent rules decide whether a path reaches any codebase aggregate: the structural repo
  exclusion (the meta root, vendored trees, build output, generated C# designer files), the
  repository's own `.gitignore` compiled by the workspace matcher, and the generated-folder
  detection the tree renderer marks nodes with. They are deliberately separate — a path can be
  gitignored without being structurally excluded and the other way round — and every one of them is
  consulted on the walk's output. The combination is this repository's own policy, so the recorded
  no-oracle decision `repo-codebase-ignore-integration` carries the case: the gitignore semantics
  themselves are already judged against a real engine by the workspace module's own cases, and what
  is compared here is how the three verdicts combine.

  @id-every-vector-gets-the-same-three-verdicts
  @level-fundamental
  @mode-differential
  Scenario: Structural exclusion, gitignore and generated detection agree per path
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    And the ignore vectors shared://📡️ignore-vectors.json
    When each implementation reports the three exclusion verdicts of every vector
    Then every implementation projects the same three verdicts per vector

  @id-filtering-a-walk-drops-the-same-paths
  @level-fundamental
  @mode-differential
  Scenario: Filtering the raw walk output drops the same paths
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    When each implementation filters the raw glob output through the considered-file rules
    Then every implementation projects the same kept and dropped paths in the same order
