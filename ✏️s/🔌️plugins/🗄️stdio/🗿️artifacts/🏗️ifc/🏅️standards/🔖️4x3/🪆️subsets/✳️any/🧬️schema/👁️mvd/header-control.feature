Feature: Original released-view header control scope
  Scenario: Later header planning cannot recapture mutable caller limits
    Given original depth zero and a valid nested FILE_DESCRIPTION
    When the first callback raises the caller depth limit
    Then declaration and selection operations retain the original depth refusal
  Scenario: One operation reads each caller control only once
    Given declared header read or selection under original controls
    When nested copy and admission generators are entered
    Then they share the same private operation scope through completion
