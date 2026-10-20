Feature: Snapshot projection owner roots
  Scenario: Every generated contract path names the declared firstparty owner
    Given an empty or field-bearing record with the shared default, crate, or explicit domain root
    When the real SnapshotProjection macro expands the record
    Then the independent Rust syntax tree has the selected root for every projection and custody path
    And no hardcoded facade root remains for an explicitly selected owner

  Scenario: Packed fields cannot enter a reference-based placement protocol
    Given a record declared packed or packed with an explicit alignment number
    When snapshot projection is derived
    Then expansion refuses before generating field references
    And naturally aligned C, transparent and aligned representations remain eligible
