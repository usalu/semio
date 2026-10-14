Feature: Explicit Repository Command Execution Ownership
  Scenario: Source inspection commands retain their original source-only authority
    Given a native package with source and native command execution declarations
    When the actual repository plugin constructs the installed Nx task graph
    Then the source command keeps its literal Bun receiving command
    And it has no compiler owner wrapper or native generator prerequisite
    And the native command retains native owner issuance and all native generator prerequisites
  Scenario: Closed execution contracts refuse malformed declarations
    Given each complete execution contract example
    When the repository and an independent JSON schema validator admit the declaration
    Then both agree on every accepted and refused declaration
  Scenario: Generated native commands declare their exact compilation input scope
    Given an actual native package manifest
    When the repository generates Cargo build, check and test targets
    Then each target declares native execution explicitly
    And build and check use production dependency inputs
    And test uses the complete development dependency inputs
  Scenario: Complete inventory preparation retains original receiving controls
    Given the original receiving authority of 120000 milliseconds
    When a separately bounded preparation discovers the complete canonical manifest scope once
    Then every actual emitted command family is received unchanged
    And original source and producer fingerprints hold before and after preparation and receiving
    And each existing law retains its 5000 millisecond bound
    And cancellation, fractional remaining time and awaited progress belong to the original caller
