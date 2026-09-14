@capability-repo-event-export-content-hash
@oracle-node-crypto-repo-events
@comparison-ordered-json-v1
Feature: A repository snapshot is content-addressed by one SHA-256 over its sorted entities
  Exporting the repository writes every entity as an append-only event whose id is namespaced by
  the snapshot identity, so re-exporting an unchanged repository is refused as a duplicate rather
  than duplicating history. The identity is a SHA-256 over the id-sorted entities, each hashed as
  `id NUL kind NUL json(data)`. Node's crypto computes the same digest independently.

  @id-snapshot-identity
  @level-fundamental
  @mode-differential
  Scenario: The snapshot identity is the SHA-256 over the id-sorted entities
    Given the export vectors shared://📤️export-vectors.json
    When the implementation builds the snapshot through its ExportSource port
    Then the identity equals the frozen snapshot and the oracle's own SHA-256 of the same preimage

  @id-input-ids-are-namespaced
  @level-fundamental
  @mode-differential
  Scenario: Every input id is namespaced by the snapshot and sorted
    Given the export vectors shared://📤️export-vectors.json
    When the implementation builds the snapshot
    Then the input ids are the sorted entity ids each prefixed with the snapshot identity

  @id-unchanged-export-is-refused
  @level-quick
  @mode-error
  Scenario: Re-exporting an unchanged repository is refused instead of duplicating history
    Given a log already holding one exported snapshot
    When the same snapshot is exported again
    Then the append reports a duplicate and the log is unchanged
