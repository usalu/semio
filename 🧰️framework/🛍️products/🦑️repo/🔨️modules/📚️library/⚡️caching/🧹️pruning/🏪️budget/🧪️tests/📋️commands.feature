Feature: Guarded Current Compiler Store Maintenance Controls
  Scenario Outline: A declared control enters the existing current-domain owner
    Given a genuine Original with a selected compiler mode and ticket artifacts
    When <target> dispatches <operation>
    Then the exact original control and allocation reach guarded maintenance
    And compiler writer custody precedes waiting for inactive-store maintenance
    And no caller path, force flag or capacity override is accepted
    And the summary identifies the actual compiler root and retained plan directory

    Examples:
      | target                        | operation |
      | inactive-preview               | preview   |
      | inactive                       | retire    |
