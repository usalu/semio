Feature: Dashboard test dispatch retains the current execution context
  Scenario Outline: A selected test suite shares one current execution context
    Given the current dashboard test command and a received invocation
    When the developer selects <suite>
    Then executable selection receives that invocation's workspace and execution context
    And each selected native, owned or installed route receives the same context
    And authority checking runs before the selected suite

    Examples:
      | suite     |
      | authority |
      | execution |
      | coverage  |
      | journeys  |
      | load      |
      | smoke     |
      | quick     |
