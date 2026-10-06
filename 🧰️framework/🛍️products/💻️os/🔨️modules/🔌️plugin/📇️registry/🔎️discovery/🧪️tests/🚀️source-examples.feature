Feature: Authored startup choices survive compiled catalog withholding
  Scenario: The current host requires a rebuilt component
    Given a source owner declares browser ports and engine dependencies
    And its artifact source root contains demo and session examples
    And its compiled descriptor uses a withheld channel
    When the source launch catalog is generated
    Then both exact example identities remain selectable
    And the declared ports and engines are preserved
