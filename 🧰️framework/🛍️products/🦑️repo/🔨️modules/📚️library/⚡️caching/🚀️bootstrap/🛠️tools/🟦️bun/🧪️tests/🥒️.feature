Feature: Workspace-Owned Bun Bootstrap
  Scenario: A developer uses a different installed Bun version
    Given the tooling recipe pins Bun 1.3.14
    And the installed Bun is 1.4.2
    When Nx needs to acquire its frozen toolset
    Then it acquires the pinned executable for the current platform
    And validates the release archive checksum and executable version
    And leaves the developer's installed Bun unchanged

  Scenario: Cancel acquisition
    Given a pinned executable is being acquired
    When the developer cancels
    Then acquisition stops and no partial executable is published

  Scenario: Reuse an installed toolset
    Given the immutable Nx toolset is already complete
    When a developer starts the dashboard
    Then no Bun download is required

  Scenario: Run tasks with different ephemeral environments
    Given two developer tasks select different renderers and examples
    When each task requests its graph
    Then ordinary invocations use their own environment without the shared daemon
    And source watchers retain Nx daemon support
