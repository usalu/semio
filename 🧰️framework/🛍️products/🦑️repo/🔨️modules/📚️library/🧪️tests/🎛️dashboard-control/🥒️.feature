Feature: Developer commands belong to the dashboard registry
  Scenario: IDE generation cannot return through command routing
    Given neutral build targets and dashboard owner declarations
    Then editor launch generation targets and public APIs are absent
    And authoring persists build owner declarations without editor output
    And native dashboard run, build and daemon targets remain available

  Scenario: Source regressions remain independent of editor configuration
    Given the current test owner and typed tool declarations
    Then source and registration tests read neutral metadata
    And synthetic parser paths and hostile input rejection vectors remain unchanged

  Scenario: Setup and root infrastructure retain only current headless controls
    Given canonical AGENTS instructions and current neutral build owners
    When setup instruction aliases and root asset contracts are inspected
    Then Claude is the only generated instruction alias
    And root tests do not require removed Cursor or VS Code MCP files
    And Rust boundary tasks depend on neutral owner declarations
