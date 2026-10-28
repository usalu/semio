Feature: Required MeasureValue coordinates retain complete domain and actual candidate rules
  Scenario: The actual released domain includes every23 member without filtering controls
    Given the source-owned RigidOperation has two missing direct coordinates
    When the caller requests original type guidance
    Then both coordinates expose the complete declared MeasureValue discriminator
    And no member or numeric representation is initially selected

  Scenario Outline: Declared carrier membership never bypasses coordinate WHERE
    Given the closed MeasureValue vector <id>
    When both coordinates receive its exact caller-authored constructor
    Then structural and alias membership agrees with its domain result
    And complete readiness agrees with its actual RigidOperation WHERE result
    And source CRS references, Height and constituent order remain exact

  Scenario: Fixed ARRAY cardinality is derived from both declared bounds
    Given ComplexNumber is ARRAY1 through2 OF REAL without uniqueness
    When callers explicitly create, add and remove raw member-word drafts
    Then exactly two complete real words satisfy the array domain
    And empty, one, three and partial-member drafts stay editable and not ready
    And duplicate words and minus-zero bits are preserved without normalization

  Scenario: NUMBER representation and original ownership stay explicit
    When callers switch the selected NumericMeasure between integer and real
    Then canonical i64 text and binary64 words remain distinct complete payloads
    And incomplete UTF8 text remains exact
    And minimal central mutation and inverse restore the exact source
    And original cancellation, failure, denial and paid retirement remain authoritative
