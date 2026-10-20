Feature: Snapshot projection owner roots
  Scenario: Every generated contract path names the declared firstparty owner
    Given an empty or field-bearing record with the shared default, crate, or explicit domain root
    When the real SnapshotProjection macro expands the record
    Then the independent Rust syntax tree has the selected root for every projection and custody path
    And no hardcoded facade root remains for an explicitly selected owner
