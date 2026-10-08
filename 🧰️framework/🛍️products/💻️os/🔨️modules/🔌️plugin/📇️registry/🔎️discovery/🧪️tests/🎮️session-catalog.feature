Feature: Declared Runtime Session Prerequisites
  Scenario: A cold selected component has an old owner descriptor
    Given the playground catalog contains the selected source variant
    And its compiled descriptor speaks an older channel
    When Nx prepares the variant
    Then its component and descriptor emitter are built before description
    And its session carries the current compiled catalog
    And another variant cannot change that session catalog
    And descriptor emission starts no hidden compiler process
