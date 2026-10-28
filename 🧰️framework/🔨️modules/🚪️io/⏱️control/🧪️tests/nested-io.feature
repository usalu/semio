Feature: Original foreign IO phase selection
  Scenario: An empty borrowed pending slot permits a completed native phase to change direction
    Given one original decoder, encoder and physical wallet in a nested receiving frame
    And its pending refusal owner slot is empty
    When decoding finishes and encoding begins
    Then the same original encoder is selected
    And the original wallet and decoder identity remain unchanged

  Scenario: A retained refusal owner prevents a direction change
    Given a nested receiving frame whose original pending slot owns a refusal frontier
    When the opposite native phase is requested
    Then the request refuses without replacing or dropping that owner
