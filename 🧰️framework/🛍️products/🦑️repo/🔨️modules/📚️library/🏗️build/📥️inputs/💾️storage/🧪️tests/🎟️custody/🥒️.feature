Feature: Optional Graph Inventory Requires Actual Exclusive Custody

  Scenario Outline: Preserve headroom decisions while avoiding discarded inventories
    Given the current optional cache policy and <state>
    When nonwaiting exclusive maintenance is <custody>
    Then deferred maintenance performs no optional-tree inventory
    And acquired maintenance inventories exactly once inside custody
    And healthy or boundary state returns no-op
    And pressure retires only exact unchanged optional leaves
    And cancellation identity changes or unavailable required headroom refuse

    Examples:
      | state                  | custody |
      | pressure               | deferred |
      | healthy                | deferred |
      | healthy                | acquired |
      | exact low-water        | acquired |
      | low-water pressure     | acquired |
      | physical pressure      | acquired |
      | changed leaf identity  | acquired |
      | canceled before unlink | acquired |
      | no reclaimable bytes   | acquired |
