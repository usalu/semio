Feature: Node tool entrypoints from package metadata
  Scenario: A Bun-installed scoped tool has no Unix launcher file
    Given a package declares its tool entrypoint in its bin map
    And the installation uses platform launcher shims
    When the repository resolves the tool for Node
    Then the declared JavaScript entrypoint is returned
    And Node executes that entrypoint successfully
