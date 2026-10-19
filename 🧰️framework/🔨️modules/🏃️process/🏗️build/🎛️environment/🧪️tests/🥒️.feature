Feature: Dashboard-Owned Runtime Environment
  Scenario: Compiler startup uses the owner environment in every build mode
    Given the language-neutral environment cases in 🔣️.json
    When the compiler owner prepares prototype and release environments
    Then Bun and Node implicit compilation caches are disabled
    And the inherited Node compilation cache path is removed
    And unrelated environment values and the caller input remain unchanged
    And the result agrees with SQLite and both schema validators

  Scenario: Verification is discoverable without IDE configuration
    Given the process owner declares test-runtime-environment
    When the dashboard discovers the owner's targets
    Then verification resolves through the owner's script
    And no IDE executable catalog defines the runtime contract
