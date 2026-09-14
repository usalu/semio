@capability-repo-yaml-round-trip
@no-oracle-repo-yaml-owned-encoder
@comparison-ordered-json-v1
Feature: The owned encoder renders an empty container as an empty block, and loses its kind
  The encoder is deliberately not a general YAML emitter: it produces one deterministic, key-sorted
  subset chosen for diffability. An empty sequence and an empty mapping therefore both emit as a key
  with an empty block, so re-decoding turns `list: []` into an empty mapping rather than an empty
  sequence. That is a recorded property of the subset, not a bug to hide: every implementation must
  lose exactly the same information. No third-party emitter reproduces this subset, so the case has
  no oracle — see `repo-yaml-owned-encoder`.

  @id-the-owned-encoder-loses-an-empty-container
  @level-fundamental
  @mode-round-trip
  @seed-1
  Scenario: Decoding, encoding and decoding again reaches the same fixed point everywhere
    Given the shared vector set shared://📡️codec-vectors.json
    When the host decodes every vector and edge vector, re-encodes it and decodes the result
    Then every implementation projects the same canonical JSON per vector
