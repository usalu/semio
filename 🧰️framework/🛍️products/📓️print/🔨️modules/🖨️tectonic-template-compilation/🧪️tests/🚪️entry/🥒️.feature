Feature: Compilation Entry Workspace Authority
  Scenario: Route Authored Commands in an Isolated Caller Workspace
    Given a genuine native invocation for a private repository
    When the actual compilation router is created
    Then its workspace equals the native caller workspace
    And its runtime bundle remains in the source checkout
    And malformed commands fail before compiler or store effects
    And copied invocation data cannot create a router
