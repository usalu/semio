Feature: Native Renderer Publication During Execution
  Scenario: Build while the previous native renderer is running
    Given the published development renderer is running and reports its original output
    When its source changes and Nx publishes the development and release outputs
    Then both newly published executables match the independent Cargo output
    And the original process keeps running until its owner releases it
