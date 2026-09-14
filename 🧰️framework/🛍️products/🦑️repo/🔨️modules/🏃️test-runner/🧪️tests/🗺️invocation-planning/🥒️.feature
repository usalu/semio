@capability-repo.test-runner.invocation-planning
@no-oracle-repo-test-runner-planning
@comparison-ordered-json-v1
Feature: A scope becomes an ordered list of runner invocations before anything runs
  Planning is a pure function of a filesystem snapshot and a scope: the same snapshot and the same
  scope always yield the same ordered argv list, with no process started and no clock read. That is
  what makes a plan a golden value. The narrowing order is repository → technology → bundle → file →
  section → definition, and every step of it is transcribed into shared://🗺️planning-vectors.json
  with the exact argv the Go implementation produces (recorded decision `repo-test-runner-planning`).
  A scope the planner cannot serve is refused with a message, never with a silently empty plan.

  @id-scope-plans-match-the-frozen-argv
  @level-fundamental
  @mode-conformance
  Scenario: Every scope plans exactly the argv the vector table freezes
    Given the recorded planning vectors shared://🗺️planning-vectors.json
    When the host plans every vector's scope against the vector table's snapshot
    Then each plan equals the plan the vector declares, argument for argument and in order

  @id-a-scope-with-no-runner-is-refused
  @level-fundamental
  @mode-error
  Scenario: A bundle with no manifest and a file outside every bundle are refused, not skipped
    Given the recorded planning vectors shared://🗺️planning-vectors.json
    When the host plans the vectors whose declared plan carries a problem
    Then the planner reports that problem and plans no invocation for them

  @id-narrowing-never-widens-the-plan
  @level-quick
  @mode-property
  Scenario: Each narrowing step plans no more invocations than the step above it
    Given the recorded planning vectors shared://🗺️planning-vectors.json
    When the host plans the repository, one technology, one bundle and one file of that bundle
    Then each plan has at most as many invocations as the plan above it, and planning twice agrees
