Feature: Repo commands are actions of the repo domain
  Statutes, goals, trees and tickets are answered in process by the Rust domain crates, or by the Go
  binary of the repo CLI; the registry offers each action as a command with a key of its own.

  Scenario: Repo actions run in process against the domain crates
    Given an empty workspace
    When the statute catalog, the trees, the goal list and an absent ticket are asked for
    Then the Rust domain crates answer in process

  Scenario: A repo action carries the argument list of the Go implementation
    Given repo actions
    When their Go argument list is asked for
    Then each is the argument list the Go repo CLI performs

  Scenario: Every action has a distinct key the registry grammar accepts
    Given all repo actions
    When their keys are derived
    Then all keys differ
    And each one is a valid repo command identity

  Scenario: The Go binary lives in the marked cache
    Given a workspace
    When the path of the Go repo binary is asked for
    Then it is below the marked cache folder of the workspace
