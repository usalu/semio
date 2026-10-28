Feature: Original source-ordered IFC relationship preparation
  Scenario: Borrowed qualified role contributes only once per source relation
    Given a private admitted source and a closed declaration-owner plan
    When one relation contains repeated references to the same target
    Then lookup retains one relation in source order with its exact original slot pointer
  Scenario: Preparation does not admit ignored payloads
    Given malformed property payloads outside selected relationship roles
    When family and reference membership indexes are prepared cooperatively
    Then payload semantics are deferred to their original domain consumer
  Scenario: Unresolved role status is never an empty match
    Given late unknown or ambiguous declared-owner constituents
    When a role lookup completes
    Then unresolved source evidence is retained in source order
