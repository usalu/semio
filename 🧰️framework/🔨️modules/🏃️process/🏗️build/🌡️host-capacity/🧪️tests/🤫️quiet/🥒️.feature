Feature: A Quiet Forecast Holds Until Work Really Runs

  Scenario: A Nested Invocation Runs Under The Quiet Grant It Inherits
    Given a run that holds a quiet grant beside a full machine
    When a nested invocation of the run inherits the grant
    Then its body starts at once with the jobs, cores and plan of the full request and the grant stays quiet
    And the grant grows only when an executor of the run says that work has to run

  Scenario: Work No Executor Announced Grows The Quiet Grant Without Pausing The Run
    Given a quiet grant whose process tree comes to hold more than its quiet reservation
    When the holder sees it at a heartbeat
    Then the grant asks to grow into the full request in the fair queue while the run goes on
    And it says so in English or German, and the run is learned as one that executed work
    And a run that ends while the growth still waits leaves no waiter behind

  Scenario: A Quiet Run That Stays Within Its Reservation Stays Quiet
    Given a quiet grant whose process tree never holds more than its quiet reservation
    When the run ends
    Then the grant never grew and the run is learned as quiet with the full jobs
