Feature: Native IFC4x3 source composition
  Source STEP files and the native stdio.ifc graph pack have distinct representations.

  Scenario Outline: Preserve the complete official informative example
    Given the official <fixture> IFC4X3_ADD2 STEP input
    When the native IFC4x3 reader imports it
    And the resulting graph is exported and imported again
    Then every header, identity, entity, typed value and complex constituent is preserved
    And its entity count is <count>
    Examples:
      | fixture      | count |
      | triangulated | 35    |
      | alignment    | 3161  |
      | structural   | 188   |

  Scenario: Native packed graph preserves its IFC4x3 declaration
    Given an IFC4X3_ADD2 snapshot
    When the shared stdio.ifc native graph pack encodes and decodes it
    Then FILE_SCHEMA remains IFC4X3_ADD2
    And the host envelope remains s.stdio.ifc@4x3/*

  Scenario: Reject an invalid source before publication
    Given a source with a foreign schema, duplicate ids, zero ids or unresolved nested references
    When native IFC4x3 source import runs
    Then it returns an error without publishing a snapshot

  Scenario: Cancellation reaches interior source work
    Given a caller-owned cancellation callback
    When it refuses an interior record checkpoint during native import or export
    Then no completed output is published
    And the source graph remains unchanged

  Scenario: Register the complete natural text route
    Given the IFC artifact declaration
    Then a composer reads UTF-8 text and writes s.stdio.ifc@4x3/*
    And an exporter reads s.stdio.ifc@4x3/* and writes UTF-8 text

  Scenario: Export respects every physical record and complete framing
    Given a model with no entities and a large escaped FILE_NAME value
    When a header record exceeds the record admission bound
    Then export refuses before returning source
    And the total source bound includes all section markers and the final terminator
    And source exactly at the complete bound round-trips

  Scenario Outline: Locate an invalid token in original source bytes
    Given an earlier record containing Unicode and comments
    And a later record containing <defect>
    When native source import fails
    Then the error offset points at <token> in the original UTF-8 bytes
    Examples:
      | defect                   | token  |
      | duplicate instance id    | #1     |
      | nested missing reference | #999   |
      | nested overflowing real  | 1.E999 |
