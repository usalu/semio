Feature: Local-first persistence

  Scenario: Favorites survive a reload
    Given a reader stars the walking commuter and the park bench and unstars the commuter
    When the page is opened again on the same storage
    Then the park bench is the only favorite

  Scenario: Another tab's change arrives
    Given two tabs on one storage
    When one tab stars an asset and the browser announces the storage event
    Then the other tab shows the favorite

  Scenario: Foreign or damaged data is ignored
    Given a stored document of another version or with invalid members
    Then the archive starts from its initial state and keeps only valid members

  Scenario: Blocked storage is reported
    Given storage that refuses every write
    When the reader stars an asset
    Then the asset is starred for this session and the archive says nothing is kept
