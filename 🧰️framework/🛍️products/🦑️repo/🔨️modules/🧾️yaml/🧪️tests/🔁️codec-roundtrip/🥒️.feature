@capability-repo-yaml-decode
@oracle-yaml-js
@comparison-ordered-json-v1
Feature: The repo YAML codec decodes the shared vector set the way YAML says, and round trips it
  The repository owns a bounded YAML decoder and a deterministic, key-sorted encoder. The decoder is
  a data decoder, not a YAML processor: it accepts the JSON superset first, then an indentation
  scanner covering block mappings, block sequences, flow sequences, quoted and plain scalars,
  comments and document markers. What each vector MEANS is decided by a complete third-party YAML
  implementation, never by this repository — and for this vector set a decode/encode/decode round
  trip must reach the same value the reference reaches, so the owned encoder is judged too. The one
  document where the owned subset deliberately loses information lives in the sibling case
  `🕳️empty-container-encoding`.

  @id-vectors-decode-to-the-same-value
  @level-fundamental
  @mode-differential
  @seed-1
  Scenario: Every vector decodes to the value a real YAML implementation decodes it to
    Given the shared vector set shared://📡️codec-vectors.json
    When the host decodes every vector and renders each result as canonical JSON
    Then every implementation projects the same canonical JSON per vector

  @id-decode-encode-decode-is-idempotent
  @level-quick
  @mode-round-trip
  @seed-1
  Scenario: Decoding, encoding and decoding again reaches the value the reference reaches
    Given the shared vector set shared://📡️codec-vectors.json
    When the host decodes every vector, re-encodes the value with its own encoder and decodes that
    Then every implementation projects the same canonical JSON per vector
