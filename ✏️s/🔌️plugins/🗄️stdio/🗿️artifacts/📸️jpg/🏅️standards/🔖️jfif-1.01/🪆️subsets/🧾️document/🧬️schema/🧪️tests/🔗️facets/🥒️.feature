Feature: JPEG schema facets declare only authored content
  Scenario: Independently parse the authored snapshot and diff declarations
    Given the neutral authored snapshot, image and sparse diff field lists
    When GraphQL and Protocol Buffers independently parse the committed facets
    Then the declared fields match the owned image model and every type resolves
