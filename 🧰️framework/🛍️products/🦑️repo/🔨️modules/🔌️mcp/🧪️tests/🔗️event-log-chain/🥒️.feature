@capability-mcp-event-log-chain
@oracle-node-crypto-sha256
@comparison-ordered-json-v1
Feature: The MCP event log is a verifiable hash chain
  Every frame the server accepts or emits is appended to a `semio.mcp.event/1` JSONL log whose records
  are chained by SHA-256 over their own canonical rendering. The digest is hand-rolled inside each
  implementation, so Node's `crypto` module is the oracle that proves the hand-rolled one is right.

  @id-chain-digests-match-the-golden
  @level-fundamental
  @mode-differential
  Scenario: The chain built from the fixture inputs matches the golden digests
    Given the golden chain shared://🔗️event-chain.json
    When the implementation commits the fixture's inputs in order
    Then every implementation projects the same per-record digests and the same JSONL rendering

  @id-a-tampered-chain-is-refused
  @level-quick
  @mode-error
  Scenario: Replaying a chain whose first record was altered fails
    Given the golden chain shared://🔗️event-chain.json
    When one record's kind is altered and the log is replayed
    Then every implementation refuses the log instead of accepting a broken chain
