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

  Scenario: Start an installed dashboard after source changes
    Given a dashboard installation exists
    And a compiled source changed since installation
    When the developer runs the dashboard bootstrap
    Then the bootstrap creates or receives an original process envelope
    And its rebuild receives that same policy, deadline and capabilities
    And the rebuilt executable starts only after successful installation

  Scenario: An inherited deadline remains authoritative
    Given the dashboard bootstrap received a process envelope
    When it takes the installed executable route
    Then it retains the original deadline without extending it
    And refuses conflicting explicit policy or capabilities

  Scenario: Cancel a bootstrap child
    Given the dashboard owns a rebuild or terminal subprocess
    When its original process invocation is cancelled
    Then its owned subprocess tree is stopped
    And the bootstrap reports exit code 130
    And a normal child retains its actual exit code

  Scenario: Cancel a dashboard view without stopping its workspace
    Given a dashboard view started an independently supervised workspace daemon
    When the view's original invocation is cancelled
    Then the view process ends
    And the workspace daemon and its tasks remain controllable

  Scenario: The full native suite declares its actual minimum tier
    Given the native suite includes real terminal and multi-view integration journeys
    When a developer runs the default native suite
    Then its minimum test level is quick
    And an explicitly requested higher level is retained

  Scenario: Install the current optimized release while obsolete output exists
    Given the canonical native builder publishes its release profile output
    And an obsolete unprofiled executable remains in a different directory
    When the dashboard installation owner publishes immutable executable bytes
    Then the installed bytes equal the current release bytes
    And their address equals the independent SHA-256 digest
    And the installation directory agrees with the independently compiled builder

  Scenario: Sources changed during preparation are captured at compilation
    Given the owner prepares a canonical source revision before the compiler starts
    When preparation and profile queueing finish
    Then the full source identity names the revision consumed by the compiler
    And source edits during compilation refuse installation
    And cancellation remains bound to the original compiler invocation

  Scenario: Validate current metadata without serial filesystem waits
    Given a source workspace has many real project owner directories
    When the bootstrap revalidates their current custody
    Then at least two independent observations can overlap
    And at most sixteen observations are outstanding
    And every observation is settled before publication
    And symlink replacement, peer edits and cancellation still refuse publication
