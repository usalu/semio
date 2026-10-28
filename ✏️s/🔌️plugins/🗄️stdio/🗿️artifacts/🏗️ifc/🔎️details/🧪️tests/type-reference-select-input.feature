Feature: Source-owned entity membership supplies required SELECT references
  Scenario: Complete entity-only SELECT domain
    Given the original snapshot and complete released SELECT members
    Then the existing exact Reference input remains unselected and editable
    When the caller supplies an existing member or subtype ID
    Then the actual source determines its SELECT membership
    And the exact Reference reaches central mutation and inverse
  Scenario: Required reference refusal
    Then known nonmembers and missing targets cannot become ready
    And incomplete decimal drafts remain editable
    And entity typedValue wrappers cannot replace exact References
    And source, request and original controls retain their authority
