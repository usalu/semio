@capability-repo-statutes-breach-cache
@oracle-node-zlib-crypto
@oracle-input-subject-raw
@comparison-ordered-json-v1
Feature: A compressed breach cache member is gzip and its digest is SHA-256
  The breach cache stores an envelope of an entity id, the script that produced it and its breaches,
  compressed as a gzip member and keyed by the SHA-256 digest of its canonical JSON encoding. Both
  are standards, so both are judged by the runtime's own zlib and OpenSSL bindings rather than by
  this repository: a member written here must inflate there, a member written there must inflate
  here, and the digest must be the published one for the published test vectors.

  @id-the-digest-is-sha-256
  @level-fundamental
  @mode-differential
  @seed-1
  Scenario: Every digest vector hashes to the published value
    Given the envelope vectors local://🔣️vectors.json
    When the host digests every vector and every envelope encoding
    Then every implementation and the reference agree on every digest

  @id-a-member-inflates-anywhere
  @level-fundamental
  @mode-round-trip
  @seed-1
  Scenario: A member this implementation writes inflates under the reference
    Given the envelope vectors local://🔣️vectors.json
    When the host compresses every payload and inflates the result again
    Then every implementation and the reference recover the original payload byte for byte
