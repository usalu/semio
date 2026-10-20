Feature: One Native Repository Entrypoint
  Scenario: Build commands preserve their complete argument tail
    Given the shared native entrypoint vectors
    When each native implementation selects a route
    Then build commands enter the owned build script unchanged
    And only explicit mcp selects stdio protocol
    And dashboard expands to its authored native run target
    And domain commands retain their original arguments

  Scenario: OS MCP keeps protocol stdout quiet
    Given the explicit native dev mcp stdio os route
    When its existing workspace script owner serves the protocol
    Then its exact arguments and Original environment are retained
    And progress remains on stderr
    And stdout contains only the existing MCP protocol
