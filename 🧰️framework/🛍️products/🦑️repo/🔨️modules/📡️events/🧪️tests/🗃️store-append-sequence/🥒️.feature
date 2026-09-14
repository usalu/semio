@capability-repo-event-store-append
@oracle-node-crypto-repo-events
@no-oracle-repo-event-store-recovery
@comparison-ordered-json-v1
Feature: The append-only event log is deterministic, recoverable and byte-identical across implementations
  The event log is the durable spine of the repo domain: an append stages its batch, writes it,
  syncs it and only then drops the stage. An interruption at any phase must leave the committed
  prefix untouched and no stage behind. The vectors come from the CLI g1 contract fixture, now
  owned here as `🧫️fixtures/🗄️store-vectors.json`. Node's crypto judges the record checksums;
  nothing third party implements the staged-append recovery contract, so its confidence comes from
  Go and Rust producing identical bytes.

  @id-append-then-replay-sequences
  @level-fundamental
  @mode-differential
  Scenario: Appending the vectors yields the frozen sequence numbers and identical bytes
    Given the store vectors shared://🗄️store-vectors.json
    When the implementation appends them to an empty log and replays it
    Then the replayed sequences equal the frozen sequences and a second append of the same inputs produces the same file bytes

  @id-duplicate-and-corrupt-are-refused
  @level-fundamental
  @mode-error
  Scenario: A repeated id is refused and a flipped bit is detected
    Given a log holding the first vector
    When the same input is appended again, and when one byte of the log is flipped
    Then the append reports a duplicate and the replay reports a corrupt log

  @id-interruption-preserves-committed-log
  @level-long
  @mode-error
  Scenario: Cancelling at any append phase leaves the committed log and no stage
    Given a log holding the first vector and the interrupt phases of the fixture
    When the second vector is appended and the caller cancels at each phase in turn
    Then the committed bytes are unchanged, no stage file remains and the replay still yields exactly the first event

  @id-record-checksum-is-sha256
  @level-fundamental
  @mode-conformance
  Scenario: A record checksum is the SHA-256 of its schema, sequence, id, kind and data
    Given the store vectors shared://🗄️store-vectors.json
    When the implementation appends them and projects each record checksum
    Then each checksum equals the SHA-256 the oracle computes over the same NUL-separated preimage
