@capability-repo-workspace-ignore-divergence
@no-oracle-repo-workspace-ignore-divergence
@comparison-ordered-json-v1
Feature: Two ignore rule shapes deliberately differ from git
  A leading slash is NOT a root anchor here: the slash is dropped and a single-segment rule is lifted
  to every depth, so `/target` also ignores `nested/target`. And a negation CAN re-include a file
  underneath an excluded directory, because precedence is purely positional — git refuses that. A
  gitignore-conformant oracle disagrees by construction, so this case has none; see the recorded
  decision `repo-workspace-ignore-divergence`.

  @id-recorded-divergences-from-git-hold
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: The two deliberate divergences behave as recorded
    Given the shared vector set shared://📡️ignore-vectors.json
    When the host asks its ignore matcher for a verdict on each recorded divergence
    Then every implementation projects the recorded owned verdict, not the git verdict
