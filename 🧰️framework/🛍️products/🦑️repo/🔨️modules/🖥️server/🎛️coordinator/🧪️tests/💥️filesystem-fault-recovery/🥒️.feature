@capability-repo-coordinator-filesystem-fault
@no-oracle-repo-coordinator-filesystem-faults
@comparison-ordered-json-v1
Feature: A filesystem fault at any point of an append leaves the committed log untouched
  An append is staged, published and only then swapped in, so a power loss between any two of those
  durable metadata mutations must still leave exactly the previously committed log behind and no
  recovery artifact. `shared://💥️filesystem-fault-recovery/💥️fault-plan.json` names one seed event, one second event and the
  mutation indices to fail at; both implementations expose the same `arm the fault at N` port so the
  identical plan can be replayed against each without a platform-specific fault harness.

  @id-every-injected-fault-preserves-the-committed-log
  @level-fundamental
  @mode-error
  Scenario: Failing the Nth durable mutation preserves the log and recovers on reopen
    Given the fault plan shared://💥️filesystem-fault-recovery/💥️fault-plan.json
    When the second event is appended once per failure index with the fault armed at that index
    Then every append is refused, the committed bytes are unchanged and reopening the log replays exactly the seed event
