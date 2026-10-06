Feature: Renderer consumers use completed producers from one selected session
  Scenario Outline: Browser startup owns its shared Flow runtime
    Given the draw playground and the <profile> profile
    When Nx schedules preparation for <renderer>
    Then the authored Flow browser producer precedes the renderer
    Examples:
      | profile | renderer |
      | dev | react |
      | release | react |
      | dev | wgpu |
      | release | wgpu |
  Scenario: Native publication and launch consume the selected session catalog
    Given an empty global catalog and a completed draw session catalog
    When native publication and launch execute
    Then both select draw from the session projection
    And the example and smoke arguments reach the native binary
  Scenario: Native foreground process failures remain visible
    Given a native run or smoke target
    When its child exits with code 23
    Then Nx reports a failed foreground task
    And the dashboard retains that failure outcome
