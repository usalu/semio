Feature: Thermal scenario sensitivity
  The same asset under different environments, activities, exposures and clothing responds in the physically expected direction and magnitude.

  Scenario: Cold versus mild air
    Given a walking commuter standing in air of 2 degrees Celsius and in an office at 22 degrees Celsius
    When both thermal fields are solved
    Then the skin under the coat and the coat surface are colder in the cold air, by more than 1 K and 10 K

  Scenario: Rest versus walking
    Given the commuter at 1.2 met and at 3 met in the cold
    Then metabolic heat is more than 2.4 times higher, the core is warmer and the coat surface is warmer

  Scenario: Sun versus shade
    Given a hot sunny day and the commuter fully sunlit or fully shaded
    Then the sunlit coat absorbs more than 50 W/m2 more and is more than 2 K warmer

  Scenario: Engine off versus on
    Given a parked car at engine load 0 and 1
    Then the exhaust system surface is more than 80 K hotter at load 1 and stays below 600 degrees Celsius
