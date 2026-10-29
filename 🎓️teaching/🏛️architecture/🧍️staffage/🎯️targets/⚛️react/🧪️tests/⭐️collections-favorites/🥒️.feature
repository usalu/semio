Feature: Favorites and collections stay on this device
  Scenario: Favorites survive a reload
    Given a reader who starred the cyclist and the park bench and unstarred the bench
    When the archive is opened again on the same storage
    Then only the cyclist is a favorite

  Scenario: Saved collections are events
    Given a saved collection "Street" with assets a and b
    When asset c is added, asset a is removed and later the collection is deleted
    Then the stored collections reflect each step and end empty

  Scenario: A curated collection is placed side by side
    Given a curated collection of a 1 m wide and a 2 m wide asset and an empty playground
    When all are placed
    Then their centres are 0.6 m plus half of each width apart and the ground grows when they do not fit
