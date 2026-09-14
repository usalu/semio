@capability-repo.cli.export-verb-records
@no-oracle-repo-cli-owned-export-batch
@comparison-ordered-json-v1
Feature: The `export` verb turns a repository into one deterministic event batch
  `semio export [output]` enumerates the repository snapshot — technologies, bundles, folders,
  files, sections, definitions, in that order — and appends it to an event log as one batch. Every
  entity becomes an input whose id is `<kind>:<entity id>`, the inputs are sorted by id, the sorted
  stream is hashed into the snapshot digest, and every input id is then namespaced with that
  digest. Two exports of the same repository therefore produce the same log.

  The records in local://🗄️repo-records.json are the frozen recording context 🔗️graphql owns, so
  the batch below is a function of committed data rather than of the working tree.

  The digest hashes the marshalled entity records, so it holds only while 📐️model encodes a record
  identically everywhere — in its own declaration order, which is what that owner's golden set pins.
  The last scenario states the digest and the encoded records themselves, so an encoding that drifts
  in one implementation is caught here rather than by a coordinator replaying one log into the other.

  @id-the-batch-states-its-counts
  @level-fundamental
  @mode-conformance
  Scenario: The batch carries one count per entity kind and a sha-256 digest
    Given the records local://🗄️repo-records.json
    When the host builds the export batch the verb would append
    Then the per-kind counts are the ones the batch states and the digest is a sha-256

  @id-every-input-id-is-namespaced-by-the-digest
  @level-fundamental
  @mode-conformance
  Scenario: Every input id carries the snapshot digest and its entity kind
    Given the records local://🗄️repo-records.json
    When the host builds the export batch the verb would append
    Then every input id reads `snapshot:<digest>:<kind>:<entity id>` and the ids are sorted

  @id-the-batch-is-deterministic
  @level-quick
  @mode-round-trip
  Scenario: Building the batch twice produces the same digest and the same ids
    Given the records local://🗄️repo-records.json
    When the host builds the export batch twice
    Then the two batches are equal

  @id-the-digest-and-the-encoded-records-agree
  @level-fundamental
  @mode-differential
  Scenario: The snapshot digest and every encoded record are the same everywhere
    Given the records local://🗄️repo-records.json
    When the host builds the export batch the verb would append
    Then every implementation states the same digest and the same encoded record for every input
