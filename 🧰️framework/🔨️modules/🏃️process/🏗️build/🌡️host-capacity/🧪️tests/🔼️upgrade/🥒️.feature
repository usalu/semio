Feature: A Run That Executes Nothing Answers Before It Takes A Capacity Grant

  Scenario: A Forecast Quiet Run Does Not Queue Behind Full Builds
    Given a machine whose capacity is fully reserved by other runs and a queue of full requests
    And the earlier runs of this class mostly executed nothing and the newest one executed nothing
    When the run starts
    Then it is admitted at once with a small quiet reservation, passing the queue barrier within the declared number of quiet grants
    And the build tool is told the full jobs it may use after an upgrade
    And a run that executes nothing is learned as quiet and never teaches the class of real builds

  Scenario: Work That Has To Run Grows The Reservation In The Fair Queue
    Given a run that holds a quiet reservation
    When its planner, a graph rebuild or its first build action has to run
    Then it asks to grow into its full request and waits in the machine-wide queue with progress lines in English or German
    And it keeps the queue position it arrived with, so it is never worse off than asking for the full grant at the door and never better
    And only the difference to the held reservation has to fit, the held reservation never counts twice and never drops to nothing
    And no action of the build starts before the full reservation is held

  Scenario: Cancelling The Wait Keeps The Small Grant
    Given a run that waits to grow its reservation
    When the wait is cancelled
    Then the request leaves the queue, the small grant stays and the run says so in English or German

  Scenario: The Run After An Edit Asks For The Full Grant At The Door
    Given a run that executed work
    When the next run of the same class starts
    Then it is not forecast quiet and queues like any other full request
