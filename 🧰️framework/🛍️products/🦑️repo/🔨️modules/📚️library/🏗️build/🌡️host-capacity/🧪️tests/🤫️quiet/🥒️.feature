Feature: The Door Keeps A Quiet Forecast Until Work Really Runs

  Scenario: The Door Re-Enters Under The Quiet Grant It Holds
    Given a run of a class whose earlier runs executed nothing, beside a full machine
    When the door admits it with the small quiet grant and re-enters under that grant to start the tool
    Then the re-entered door starts at once, tells the tool all its jobs and never grows the grant
    And the run is learned as quiet

  Scenario: Work No Executor Announced Is Learned As Work
    Given a quiet run whose process tree comes to hold more than its quiet reservation
    When no executor asked for capacity
    Then the reservation grows anyway, in English or German, and the run is learned as one that executed work

  Scenario: The Learned Outcomes Equal An Independent Model
    Given sequences of runs that asked for work or not and peaked at different sizes, from an empty store, random stores and the live store of 14:50 in which every run had been learned busy
    When each run is learned by the real door
    Then whether it executed work and the forecast before every run equal the Python model
