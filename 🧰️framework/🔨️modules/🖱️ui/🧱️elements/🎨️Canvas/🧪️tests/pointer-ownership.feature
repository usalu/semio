Feature: Controls retain pointer ownership beside shared window edges
  Scenario Outline: Semantic editable controls beside an invisible resize region
    Given a control with <attribute> equal to <value> beside a shared window edge
    When the pointer presses the control inside the expanded resize region
    Then the control receives its pointer event without starting a resize
    Examples:
      | attribute       | value          |
      | role            | textbox        |
      | role            | combobox       |
      | contenteditable |                |
      | contenteditable | plaintext-only |
