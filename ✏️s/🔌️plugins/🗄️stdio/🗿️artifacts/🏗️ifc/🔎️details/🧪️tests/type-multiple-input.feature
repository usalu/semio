Feature: Source-bound progressive entry of every required type reference
  Scenario: A two-reference target can be completed in either field order
    Given an original complete complex source and explicit target declaration
    Then every unmatched required entity reference has an editable source-bound field
    When the caller edits a later required reference before an earlier one
    Then the next original staged carrier preserves it at its exact sorted target index
    And already supplied fields remain editable with their original exact text
    And readiness stays false until the full target tuple passes strict admission
    And the same central batch and progressive inverse agree with independent JSON Patch
  Scenario: All released direct required entity-reference controls fit the actual body caps
    Given the released IFC4X3_ADD2 declarations
    Then the maximum is four on IfcSurfaceStyleLighting
    And all four fields fit the actual sixteen-content eighteen-control body
    And no required input is omitted or replaced by a generated model identifier
  Scenario: Original custody covers partial request copies and field bindings
    Then denied original axes preserve receivers, cursor scaffolds and prepared fields
    And every metadata, row-copy, field, domain and output prefix retires after cancellation
    And a substituted source, revision, profile or body cannot reuse prepared row indices
