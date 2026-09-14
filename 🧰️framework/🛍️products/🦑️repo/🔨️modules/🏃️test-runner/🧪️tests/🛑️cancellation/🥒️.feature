@capability-repo.test-runner.cancellation
@no-oracle-repo-test-runner-planning
@comparison-ordered-json-v1
Feature: A cancelled run keeps what it already finished
  Cancellation is cooperative and is observed between invocations, never inside one: a plan that is
  cancelled after its first runner has answered reports that one outcome, says it was cancelled, and
  says how far it got. It never discards the work it already has, and it never reports the invocations
  it skipped as passing. The same is true of a runner that could not be started at all: that is a
  problem on the report, not an empty green result (recorded decision `repo-test-runner-planning`).

  @id-cancelling-mid-plan-keeps-completed-outcomes
  @level-fundamental
  @mode-conformance
  Scenario: Cancelling after the first invocation reports one outcome and stops
    Given the recorded cancellation vectors shared://🛑️cancellation-vectors.json
    When the host runs the plan against the recorded transcripts and cancels after the declared invocation
    Then the report carries the outcomes already produced, is marked cancelled, and names how many of the plan ran

  @id-progress-is-reported-before-and-after-every-invocation
  @level-fundamental
  @mode-conformance
  Scenario: Every invocation is announced before it starts and again when it ends
    Given the recorded cancellation vectors shared://🛑️cancellation-vectors.json
    When the host runs the whole plan without cancelling and records the progress stream
    Then the stream opens with the plan size, brackets each invocation, and closes with the completed count

  @id-a-missing-transcript-is-a-problem-not-a-pass
  @level-fundamental
  @mode-error
  Scenario: A runner that cannot be started is a problem, never a silent pass
    Given the recorded cancellation vectors shared://🛑️cancellation-vectors.json
    When the host runs the plan against a runner that has no transcripts at all
    Then every invocation reports not-run with a problem, and no outcome claims to have passed
