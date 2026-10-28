Feature: Declared BIM project phase names retain exact ordered records
  Scenario: Explicit empty list differs from one blank phase name
    Given an authored PhaseCount and optional nonempty IFCTEXT PhaseNames list
    When the real exporter and importer roundtrip the source
    Then empty strings, separators, duplicates, Unicode, long text and ordering are unchanged
  Scenario: Singular foreign IFC phase is one label
    Given no authored project phase-list contract
    When IfcContext.Phase is a string containing a semicolon
    Then one exact phase name is projected without delimiter decoding
  Scenario: Caller failure and malformed owned definitions cannot publish
    Given original source ownership and cooperative phase-list traversal
    When a definition is ambiguous or original progress aborts or throws
    Then no partial phase list artifact is returned
