Feature: Ordinary Control Source Authority
  Scenario Outline: Dispatch Only The Current Enabled Record And Its Unique Binding
    Given an ordinary control rendered from one document source
    When its source changes according to a language neutral case
    And the user triggers the rendered record
    Then only the current enabled record may dispatch
    And its trigger must have exactly one resident binding
    And the source mints the intent revision and sequence
    And the control retains neither a copied document nor a copied argument graph
