Feature: One process route for every implementation
  Scenario: One process route for every implementation
    Given the shared native entrypoint vectors
    When the launcher plans each argument vector
    Then build, workspace, script, mcp and domain routes agree with the vectors
    And script options map to the flags of the selected runtime
    And planning never changes the caller's arguments
