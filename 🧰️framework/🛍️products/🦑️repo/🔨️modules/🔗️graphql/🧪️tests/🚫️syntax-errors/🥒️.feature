@capability-graphql-syntax-errors
@oracle-graphql-js
@comparison-diagnostic-v1
Feature: A malformed request is rejected, never silently half-parsed
  Every input in this corpus is malformed for the GraphQL grammar AND for the reference
  implementation, so agreement is a real claim rather than a restatement of our own subset. What is
  compared is the rejection itself: the `diagnostic-v1` profile drops `offset` and `detail`, because
  the reference implementation runs a different lexer and reports its own positions. The exact
  message and byte offset of each implementation is pinned instead by that implementation's own unit
  tests, which is where a Go/Rust divergence would surface.

  @id-malformed-inputs-are-rejected
  @level-fundamental
  @mode-error
  Scenario: Every malformed input is rejected by every implementation
    Given the malformed corpus shared://🚫️syntax-errors/🔣️malformed.json
    When each implementation parses every input
    Then every input is reported as rejected rather than parsed into a partial document
