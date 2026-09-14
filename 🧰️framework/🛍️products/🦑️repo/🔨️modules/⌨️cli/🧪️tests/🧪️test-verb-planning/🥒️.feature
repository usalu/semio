@capability-repo.cli.test-verb-planning
@no-oracle-repo-cli-owned-test-verb-planning
@comparison-ordered-json-v1
Feature: The `test` verb turns operands into an ordered list of runner invocations
  `semio test [testable-id-or-uri]...` names zero or more testable entities. No operand means
  every bundle whose language is detectable; a technology means its bundles; a bundle means one
  runner; a file, a section or a definition narrows further. The verb announces each invocation
  with `Running: <argv> (in <cwd>)` before it starts, and reports a scope it could not plan as a
  refusal instead of running anything.

  The vectors in local://🧪️test-verb-vectors.json freeze one filesystem snapshot — the bundle
  table plus the manifests planning probes — and state, per operand list, the announcement lines in
  order and the refusals. Nothing is executed here: what the case holds is the verb's decision, not
  the runners' behaviour.

  @id-operands-plan-their-stated-invocations
  @level-fundamental
  @mode-conformance
  Scenario: Every operand list plans the invocations it states, in order
    Given the vectors local://🧪️test-verb-vectors.json
    When the host plans every vector against the frozen snapshot
    Then each plan announces the stated lines in the stated order

  @id-unplannable-scopes-refuse-instead-of-running
  @level-fundamental
  @mode-error
  Scenario: A scope with no detectable runner refuses and announces nothing
    Given the vectors local://🧪️test-verb-vectors.json
    When the host plans every vector against the frozen snapshot
    Then each refusing vector carries its stated problem and announces no line for it

  @id-planning-is-deterministic
  @level-quick
  @mode-round-trip
  Scenario: Planning the same operands twice announces the same lines
    Given the vectors local://🧪️test-verb-vectors.json
    When the host plans every vector twice
    Then the two announcements are equal for every vector
