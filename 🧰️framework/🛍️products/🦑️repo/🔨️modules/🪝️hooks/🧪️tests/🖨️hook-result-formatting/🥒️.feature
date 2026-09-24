@capability-repo.hooks.result-formatting
@no-oracle-repo-hooks-result-formatting
@comparison-ordered-json-v1
Feature: A hook result reaches every client in the shape that client reads, and a refusal is unmistakable
  A hook answers with one record. Copilot Chat and VS Code read it wrapped in `hookSpecificOutput`,
  which carries `permissionDecision: deny` plus the reason for `PreToolUse` and `additionalContext`
  for everything else — so a refusal reaches them on stdout with exit code 0, inside the record.
  Every other client reads the bare record: in JSON mode it goes to stdout, and otherwise a refusal
  goes to stderr with exit code 2 while an allowed result prints only its message. Getting that wrong
  in either direction is the difference between a blocked `git stash` and a silently executed one.

  Version hooks additionally delegate `micro-commit` to the monorepo script, and they must all reach
  it through one argv — `<bun> ./📜️script.ts micro-commit <args…>` — because `post-commit`,
  `post-checkout`, `post-merge` and `post-rewrite` all call it and a second spelling would silently
  split the workflow in two.

  There is no third party that produces or consumes any of this, so the recorded decision is
  `repo-hooks-result-formatting`. What stands in for an oracle is the fixture: every invocation
  carries the pinned verdict and exit code it must produce, and the wrapped/plain distinction is
  asserted for all eight clients at once, so an implementation must agree with the specification
  rather than with itself.

  @id-every-client-receives-the-shape-it-reads
  @level-fundamental
  @mode-conformance
  Scenario: Copilot Chat receives a wrapped record and the other seven receive the bare record
    Given the hook invocation vectors shared://🖨️hook-result-formatting/🖨️invocations.json
    When each invocation is dispatched and rendered for all eight clients in plain mode
    Then every implementation wraps exactly the Copilot Chat record and leaves the other seven bare

  @id-a-refusal-is-unmistakable-in-both-shapes
  @level-fundamental
  @mode-conformance
  Scenario: A refusal denies inside the wrapped record and exits 2 outside it
    Given the hook invocation vectors shared://🖨️hook-result-formatting/🖨️invocations.json
    When each invocation is rendered for a wrapping client and for a bare client
    Then every implementation reports the pinned verdict, the pinned exit code and a deny decision wherever the record carries one

  @id-json-mode-always-prints-the-bare-record-to-stdout
  @level-fundamental
  @mode-conformance
  Scenario: JSON mode prints the record to stdout and never exits 2
    Given the hook invocation vectors shared://🖨️hook-result-formatting/🖨️invocations.json
    When each invocation is rendered in JSON mode for a bare client
    Then every implementation prints a parseable record to stdout with exit code 0 and an empty stderr

  @id-the-record-carries-the-event-specific-members
  @level-fundamental
  @mode-conformance
  Scenario: Each event contributes its own members to the record and no others
    Given the hook invocation vectors shared://🖨️hook-result-formatting/🖨️invocations.json
    When each invocation is dispatched
    Then every implementation projects the same member names for every invocation

  @id-micro-commit-is-delegated-through-one-argv
  @level-fundamental
  @mode-conformance
  Scenario: Every micro-commit subcommand is delegated through the same argv and reaches the runner
    Given the hook invocation vectors shared://🖨️hook-result-formatting/🖨️invocations.json
    When each micro-commit subcommand is delegated through a recorded process runner
    Then every implementation issues `<bun> ./📜️script.ts micro-commit <args…>` in the repository root and reads back what the runner answered
