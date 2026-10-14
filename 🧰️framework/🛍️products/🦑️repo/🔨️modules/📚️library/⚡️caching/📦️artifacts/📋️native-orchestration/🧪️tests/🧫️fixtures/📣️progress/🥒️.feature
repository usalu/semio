Feature: Native progress ownership
  Scenario: An interactive control plane owns its terminal screen
    Given its owner command declares delegated progress
    When the command remains active beyond the wrapper progress interval
    Then the wrapper emits no elapsed progress into the interactive screen
    And the child output matches an independent Node and esbuild execution

  Scenario: An ordinary build owns no interactive progress interface
    When its owner command remains active beyond the wrapper progress interval
    Then the wrapper continues emitting elapsed progress
