Feature: Original-owned caller-authored type guidance
  Scenario Outline: The actual controller owns a completed required-reference form
    Given the original complex MaterialConstituent source and canonical hash
    And a donated setSnapshotValue request for MaterialProfile
    When the original render-work controller prepares <draft>
    Then the completed body retains the exact caller reference text
    And its required reference remains editable even when ready
    And readiness is false for absent, blank, partial or overflow text
    And a different canonical revision refuses all output
    And an explicit null input array refuses all output
    And cancellation retires every accepted prefix under its original owner

