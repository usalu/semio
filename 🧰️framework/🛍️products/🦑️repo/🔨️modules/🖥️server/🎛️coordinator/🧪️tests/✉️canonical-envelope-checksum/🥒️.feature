@capability-repo-coordinator-event-checksum
@oracle-node-crypto-repo-coordinator
@comparison-ordered-json-v1
Feature: One coordinator event has exactly one canonical line and one checksum
  The coordinator log is compared byte for byte across implementations, so the envelope encoding is
  not an implementation detail: the seven fields appear in one frozen order, the payload is
  re-encoded with sorted object keys, and the checksum is the SHA-256 of the five header fields
  separated by NUL followed by the canonical payload bytes. Both statements are written down once,
  as constants of the scope contract `G3EventLogContract`, and the one golden record that satisfies them is
  `shared://📜️g3-event-log.jsonl`. Node's crypto reaches OpenSSL and decides the checksum.

  @id-golden-line-is-canonical
  @level-fundamental
  @mode-conformance
  Scenario: The golden record re-encodes to exactly the committed line
    Given the golden log shared://📜️g3-event-log.jsonl and the schema schema://repo.server.coordinator/G3EventLogContract
    When the implementation decodes the record and encodes it again
    Then the encoded line equals the committed bytes and the declared field order and formula are unchanged

  @id-checksum-is-sha256-of-the-preimage
  @level-fundamental
  @mode-differential
  Scenario: The checksum is the SHA-256 of the NUL-separated preimage
    Given the golden log shared://📜️g3-event-log.jsonl
    When the implementation projects the record header, its canonical payload and its checksum
    Then the checksum equals the SHA-256 the oracle computes over the same preimage
