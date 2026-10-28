Feature: A Waiting Run Says What It Stands Behind

  Scenario: The Door Names The Runs That Hold The Memory
    Given two runs of other agents that hold most of the memory of a declared 16 GB machine
    When a third agent's run has to wait at the door for memory
    Then its progress line names both runs by the target they build, whether they run, their jobs and their megabytes, largest first
    And the line is in the agent's language, English or German

  Scenario: A Growing Run Names What It Waits For
    Given a run whose quiet reservation has to grow while another run holds the memory
    When it waits to grow
    Then its progress line names that run and does not count its own quiet grant as running

  Scenario: The Host Itself Holds It Back
    Given a request that waits for raised memory pressure, swap or foreign memory with nothing of ours in the way
    When it prints its progress line
    Then the line says that the host itself holds it back, not a run of ours
