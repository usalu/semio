@capability-repo.hooks.tool-blocking-policy
@oracle-hooks-blocking-policy-typescript
@comparison-ordered-json-v1
Feature: A tool invocation that would rewrite shared repository state is refused before it runs
  Many agents and many developers work the same checkout at once, so a `git commit`, `git stash` or
  `git checkout` from inside an agent session destroys work that is not its own. The policy therefore
  refuses those verbs — and refuses them wherever they hide: behind `&&`, behind a pipe, inside
  `bash -c`, inside `python3 -c`, in list form inside inline code, behind `xargs`, behind an
  environment assignment or `sudo`, spelled with an absolute path or in capitals. Read-only git
  (`status`, `log`, `diff`) stays allowed, because refusing it would make the repository unreadable.
  `git clean` is refused only with `-fd`/`-df`, and `kill $(lsof -t -i:PORT)` is refused outright
  because in a container the matched pid can be 1.

  The oracle is a second, independently written implementation of the same written rules in
  TypeScript (`hooks-blocking-policy-typescript`) — the rules are this repository's own, so nothing
  third party can adjudicate them, but a reader who has never seen the Rust source can implement the
  prose and must arrive at the same verdict and the same refusal text.

  @id-every-invocation-is-judged-the-same-way
  @level-fundamental
  @mode-differential
  Scenario: Every fixture invocation gets the same verdict and the same refusal text
    Given the invocation vectors shared://🛡️tool-blocking-policy/🛡️invocations.json
    When each invocation is judged by the blocking policy
    Then every implementation projects the same verdict and the same reason for every invocation

  @id-the-verdict-matches-the-pinned-specification
  @level-fundamental
  @mode-conformance
  Scenario: Every verdict matches the one the fixture pins beside the invocation
    Given the invocation vectors shared://🛡️tool-blocking-policy/🛡️invocations.json
    When each invocation is judged and compared against its pinned verdict
    Then every implementation agrees with the specification for every invocation

  @id-a-command-splits-into-the-same-segments
  @level-fundamental
  @mode-differential
  Scenario: A composite command splits into the same segments, and quotes never split it
    Given the invocation vectors shared://🛡️tool-blocking-policy/🛡️invocations.json
    When each composite command is split into segments
    Then every implementation projects the same segments

  @id-inline-code-is-scanned-the-same-way
  @level-fundamental
  @mode-differential
  Scenario: Inline code is scanned for both the shell form and the list form of a blocked git call
    Given the invocation vectors shared://🛡️tool-blocking-policy/🛡️invocations.json
    When each inline code sample is scanned
    Then every implementation projects the same finding and the same reason

  @id-refusing-is-idempotent-and-order-free
  @level-quick
  @mode-property
  Scenario: Appending an allowed segment to a refused command never makes it allowed
    Given the invocation vectors shared://🛡️tool-blocking-policy/🛡️invocations.json
    When each refused invocation is extended with an allowed segment on either side
    Then every implementation still refuses it, with the reason of whichever refused segment comes first
