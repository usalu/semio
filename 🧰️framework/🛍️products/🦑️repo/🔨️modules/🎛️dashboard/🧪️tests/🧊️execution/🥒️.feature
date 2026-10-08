Feature: Native Dashboard Installation and Launch
  Scenario: Ordinary launch of an installed dashboard
    Given an installation record references an immutable native executable
    When a developer starts the dashboard or its daemon controls
    Then no project graph, dependency provisioning, build or executable hash runs
    And the digest of the sources it was built from is recomputed without reading any file
    And the native arguments match the shared launch vectors
  Scenario: Publishing a new executable while a view is alive
    Given the first installed executable is running
    When a new build publishes and installs another executable
    Then the running executable retains its original bytes
    And new launches select the new installation record
    And Nx remains responsible for build and installation tasks
  Scenario: Starting after a source of the dashboard changed
    Given an installation record names the digest of the dashboard sources and of the sources of its path dependencies
    When a file below one of those source trees is edited, added or only touched
    Then the next start reports why the installation is stale and rebuilds and reinstalls with visible progress
    And a file in tests, fixtures, build output or the dev dependencies of the crate does not make it stale
    And a failed or cancelled rebuild stops the start instead of running the outdated executable
