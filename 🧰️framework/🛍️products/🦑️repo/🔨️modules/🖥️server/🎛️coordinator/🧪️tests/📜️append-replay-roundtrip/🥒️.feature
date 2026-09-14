@capability-repo-coordinator-store-append
@no-oracle-repo-coordinator-store-recovery
@comparison-ordered-json-v1
Feature: An append and a replay of the coordinator log agree, byte for byte, across implementations
  The coordinator persists its whole state as one append-only JSONL log, so an append that is not
  reproducible is a state divergence, not a formatting difference. Appending the frozen vectors of
  `🧫️fixtures/📜️append-vectors.json` into an empty log must produce the same sequence numbers and
  the same bytes in every implementation, replaying must return exactly what was written, a repeated
  id must be idempotent rather than duplicated, and a flipped bit must be refused.

  @id-append-then-replay-yields-frozen-sequences
  @level-fundamental
  @mode-differential
  Scenario: Appending the vectors yields the frozen sequences and identical bytes
    Given the append vectors shared://📜️append-vectors.json
    When the implementation appends them to an empty log and replays it
    Then the replayed sequences equal the frozen sequences and a second append of the same inputs produces the same file bytes

  @id-duplicate-is-idempotent-and-corruption-is-refused
  @level-fundamental
  @mode-error
  Scenario: A repeated id is idempotent and a flipped bit is detected
    Given a log holding the first vector
    When the same input is appended again, when a stale expected sequence is offered, and when one byte of the log is flipped
    Then the repeated append is reported as a duplicate that changed nothing, the stale sequence is refused, and the replay reports a corrupt log
