Feature: A Run Holds Its Execution Memory Only While It Executes

  Scenario: A Planned Run Waits Small
    Given a run whose plan is made by the host service
    When it waits for planning and graph derivation
    Then it holds a small quiet grant and never the memory it executes with

  Scenario: A Planned Run Executes Full
    Given a planned run whose plan arrived and whom the scheduler lets start
    When it admits its execution
    Then its grant grows into the full request and is no longer quiet

  Scenario: The Graph Worker Is Not Starved By The Runs That Wait For It
    Given a planned run waiting for the graph and a worker that asks for memory beside it
    When the host has no room for the full grant of the run beside the worker
    Then the worker is admitted at once
