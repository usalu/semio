Feature: Native Dashboard Installation and Launch
  Scenario: Ordinary launch of an installed dashboard
    Given an installation record references an immutable native executable
    When a developer starts the dashboard or its daemon controls
    Then no project graph, dependency provisioning, build or executable hash runs
    And the native arguments match the shared launch vectors
  Scenario: Publishing a new executable while a view is alive
    Given the first installed executable is running
    When a new build publishes and installs another executable
    Then the running executable retains its original bytes
    And new launches select the new installation record
    And Nx remains responsible for build and installation tasks
