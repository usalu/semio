Feature: Transport on catalog assets
  The real assets drive the transport solver through the real evaluator.

  Scenario: Playing equals scrubbing across clip loops
    Given the walking commuter in cold air
    When the field is solved in time order and then in reverse order with a cold cache
    Then the fields are bit-identical

  Scenario: Parcels stay continuous at the loop boundary
    When fields just before and after each walk-cycle loop boundary are compared
    Then no parcel moves farther than its speed allows

  Scenario: Excavator operating states
    Given an excavator that ignites at 0.8 s, idles at level 0.2 and runs at rated level 1
    Then no exhaust exists before ignition and the rated flow is five times the idle flow

  Scenario: Vegetation
    Then the oak releases only vapour parcels from distributed anchors and has no respiration

  Scenario: Representation
    Then the field is identical whatever representation the state is evaluated with
