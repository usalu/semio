@capability-repo.cli.hook-verb-dispatch
@no-oracle-repo-cli-owned-hook-verb-wiring
@comparison-ordered-json-v1
Feature: The `hook` verb resolves one native invocation, dispatches it and writes the client's bytes
  `semio hook <event> <client>` takes a neutral repo event or a native IDE event, the tool name and
  arguments the IDE reported and the payload it piped, resolves them to one neutral event, asks the
  hook domain for a result and writes it the way that client reads it. Copilot Chat gets a wrapped
  record on standard output whatever the decision is; `--json` prints the plain record for every
  other client; otherwise a refusal goes to standard error with exit code 2 and an allowed result
  prints only its message.

  The vectors in shared://🪝️hook-verb-dispatch/🪝️hook-invocations.json state one invocation each. The environment and
  the test-file resolver are the inert ones, so a vector is a pure function of its request and the
  case never reads a checkpoint, a message file or a codebase.

  @id-every-invocation-writes-its-bytes
  @level-fundamental
  @mode-conformance
  Scenario: Every invocation writes the bytes and exit code its client reads
    Given the hook invocations shared://🪝️hook-verb-dispatch/🪝️hook-invocations.json
    When the host dispatches every accepted invocation
    Then each writes the same standard output, standard error and exit code

  @id-a-refusal-never-reaches-the-domain
  @level-fundamental
  @mode-error
  Scenario: An event slug no client owns is refused before the hook domain sees it
    Given the hook invocations shared://🪝️hook-verb-dispatch/🪝️hook-invocations.json
    When the host dispatches every refusing invocation
    Then each is refused and nothing is written

  @id-dispatch-is-deterministic
  @level-quick
  @mode-round-trip
  Scenario: Dispatching the same invocation twice writes the same bytes
    Given the hook invocations shared://🪝️hook-verb-dispatch/🪝️hook-invocations.json
    When the host dispatches every accepted invocation twice
    Then the two answers are equal for every invocation
