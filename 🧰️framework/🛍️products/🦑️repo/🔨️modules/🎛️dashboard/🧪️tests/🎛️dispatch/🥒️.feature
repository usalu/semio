Feature: Dashboard test dispatch retains the current execution context
  Scenario Outline: A selected test suite shares one current execution context
    Given the current dashboard test command and a received invocation
    When the developer selects <suite>
    Then executable selection receives that invocation's workspace and execution context
    And each selected native, owned or installed route receives the same context
    And authority checking runs before the selected suite
    And authority checking retains the installed executable reader until its process completes
    And authority checking receives the invocation and cancellation signal through the owned process runner
    And an authority process refusal releases its reader and prevents the selected suite from starting
    And every Bun test process retains the invocation and cancellation signal through the owned process runner

    Examples:
      | suite     |
      | authority |
      | execution |
      | coverage  |
      | journeys  |
      | load      |
      | smoke     |
      | quick     |
      | long      |
      | exhaustive |
      | default   |

  Scenario: Window regressions follow their current native crate owners
    Given the dashboard terminal and UI TUI crates own the window tests
    When the developer selects windows with native test arguments
    Then the terminal owner receives the dashboard docking filter
    And all current UI TUI and law owners run without the removed facade feature or namespace filter
    And every selected package name matches its authored Cargo manifest
    And all ten TUI packages use their authored framework workspace and native features
    And the actual repository test wrapper prepares and invokes each owning workspace explicitly
    And the terminal manifest has one current core dependency
    And a failed window precheck prevents native tests from starting

  Scenario: Unit selection needs no installed dashboard
    Given the current dashboard native crate owners
    When the developer selects unit with a level and native test arguments
    Then those owners receive the original invocation and native test arguments
    And no installed executable or Bun prerequisite is requested

  Scenario: Borrowed immutable observations run the current carved native owner
    Given the exact persistent package manifest, library and declared Rust law
    When the advertised source or native observation control is selected
    Then source runs only its independent oracle
    And native awaits that oracle then the exact owned persistent law with one Original
    And no unselected features or aggregate value law are requested

  Scenario: Every carved value law uses its real native test owner
    Given the current exact owner manifest, library and fully qualified law declaration
    When a dashboard value source or native control executes its actual Script class
    Then both independent TypeScript compilers agree on the selected groups
    And native preparation uses the authored six-hour fallback capped by Original
    And every native law remains awaited under that same Original
    And immutable root clone laws split across facade and companion owners
    And original retirement demand is mounted by the current companion integration owner

  Scenario: Dispatch Source laws run without native installation
    Given the declared dispatch Source target and the received Original
    When the owner selects dispatch
    Then the actual owned test port runs the dispatch law file
    And the owner awaits its completion before returning
    And no native installation or test process is selected
