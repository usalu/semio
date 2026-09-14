@capability-repo.hooks.plan-step-extraction
@oracle-hooks-plan-merge-typescript
@comparison-ordered-json-v1
Feature: A plan update event becomes a set of steps, and folding it into the recorded plan keeps its history
  VS Code sends a `todoList` whose entries carry a `title`; every other client sends `steps` whose
  entries carry a `name`; either may sit at the top level or under `tool_input`, and when the payload
  is absent the raw tool argument string carries the same JSON. Extraction turns all of that into one
  ordered list of `{name, status}`.

  Folding that list into the plan already recorded on the session is where the history lives: a step
  is ideated the first time it is seen, started the first time it reports progress, and completed only
  if it had started; a step the new plan no longer mentions is abandoned unless it already carries a
  timestamp, in which case it is kept exactly as it was. The oracle is a second, independently written
  implementation of those rules in TypeScript — the rules are this repository's own, so no third party
  can adjudicate them, but they are simple enough to state in prose and re-derive.

  @id-every-payload-shape-yields-the-same-steps
  @level-fundamental
  @mode-differential
  Scenario: Every payload shape yields the same ordered list of steps
    Given the plan vectors local://🗺️plans.json
    When the steps are extracted from every payload and tool argument string
    Then every implementation projects the same ordered steps for every payload

  @id-folding-a-plan-keeps-its-history
  @level-fundamental
  @mode-differential
  Scenario: Folding an incoming plan into a recorded one keeps every lifecycle timestamp
    Given the plan vectors local://🗺️plans.json
    When each merge vector is folded at its own second
    Then every implementation projects the same merged steps, in the same order

  @id-folding-the-same-plan-twice-changes-nothing
  @level-fundamental
  @mode-round-trip
  Scenario: Folding the same plan a second time at a later second changes no timestamp
    Given the plan vectors local://🗺️plans.json
    When each merge vector is folded and then folded again with the same incoming plan at a later second
    Then every implementation projects a second fold identical to the first
