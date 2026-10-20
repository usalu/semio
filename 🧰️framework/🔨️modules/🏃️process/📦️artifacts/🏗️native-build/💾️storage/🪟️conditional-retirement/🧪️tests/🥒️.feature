Feature: Windows conditional immutable compiler artifact retirement
  Scenario: Kernel acceptance is distinct from actual namespace and physical reclamation
    Given an authenticated finite compiler writer with exact single-link no-reparse candidate identities
    When DELETE and FORCE_IMAGE_SECTION_CHECK are accepted without POSIX or readonly overrides
    Then immutable readers and their mappings preserve their original bytes
    And completed, pending, refused and partial effects remain distinct
    And physical availability is measured after every owned handle has settled
  Scenario: Refusal never changes attributes or recursively removes a new child
    Given a sharing conflict, writable mapping, executable image, hard link, reparse point or inserted child
    Then the exact kernel operation refuses with accepted effects retained in its receipt
    And cancellation stops before the next irreversible operation and closes every owned handle

  Scenario: Partial effects remain distinct from physical recovery
    Given kernel-conditional receipts for two exact compiler units
    When a unit, recording, handle settlement or fresh availability read refuses
    Then every accepted effect remains in the retained result
    And physical recovery is unknown until all handles settle and a fresh headroom read succeeds

  Scenario: Windows kernel boundaries preserve unrelated readers and fresh children
    Given native NTFS reader, data mapping, executable image and permission fixtures
    When the same issued Original requests conditional retirement
    Then a foreign owned process reader, writable view and executable image refuse
    And a surviving readonly view keeps its original bytes
    And a newly inserted child survives the exact directory disposition refusal

  Scenario: Windows planning measures the whole selected physical unit
    Given an incremental unit with newest sessions, stale sessions and sibling lockfiles
    When Windows observes the unit before selection and again before custody
    Then every physical regular file contributes to its bounded plan extent
    And exact deletion extent validation remains unchanged
