Feature: Assets travel to the workers as they were shipped
  Scenario: A catalog asset is handed to the drawing worker
    Given a catalog asset whose geometry of every scale is still compressed
    When a figure retains the drawing worker for it
    Then the page posts the compressed payload of the asset
    And no geometry of any scale has been decoded on the page

  Scenario: The worker draws from the compressed payload
    Given the posted payload, copied as a worker transport copies it
    When the worker draws an instant of the asset
    Then the drawing equals the drawing of the authored asset as an independent decoder reads it

  Scenario: An asset that was not shipped compressed
    Given an asset built in memory
    When a figure retains the drawing worker for it
    Then the page posts the asset itself

  Scenario: A catalog asset is handed to the air flow worker
    Given a catalog asset whose geometry of every scale is still compressed
    When its air flow message is put on the wire and taken off it
    Then the wire carries the compressed payload and the worker receives the same asset
