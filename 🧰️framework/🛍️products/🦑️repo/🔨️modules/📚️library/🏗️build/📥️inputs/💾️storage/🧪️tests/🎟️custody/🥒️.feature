Feature: Optional Graph Inventory Requires Actual Exclusive Custody

  Scenario Outline: Preserve headroom decisions while avoiding discarded inventories
    Given the current optional cache policy and <state>
    When nonwaiting exclusive maintenance is <custody>
    Then deferred maintenance performs no optional-tree inventory
    And acquired maintenance inventories exactly once inside custody
    And healthy or boundary state returns no-op
    And pressure retires optional leaves through the actual custody
    And unavailable required headroom refuses

    Examples:
      | state                  | custody |
      | pressure               | deferred |
      | healthy                | deferred |
      | healthy                | acquired |
      | exact low-water        | acquired |
      | low-water pressure     | acquired |
      | physical pressure      | acquired |
      | no reclaimable bytes   | acquired |
