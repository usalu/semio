@capability-repo.goals.lifecycle
@no-oracle-repo-goals-lifecycle
@comparison-ordered-json-v1
Feature: Opening, changing, closing, reopening and deleting a goal has one outcome
  Every effect of the goal lifecycle is a port, so the whole outcome can be observed: the documents
  left in the store, the ordered calls the management provider was asked to make, the envelopes the
  emitter received and the class of every refusal. The script replays one ordered history against
  an in-memory store, an in-memory provider handing out deterministic milestone and issue numbers
  and an in-memory emitter — including the refusals, because a blank member, an unknown model, a
  duplicate identifier, a slug used as a title, closing a closed goal and reopening an open one are
  as much part of the contract as the successes. Nothing third party has a goal aggregate to
  compare against; see `repo-goals-lifecycle`.

  @id-the-script-produces-one-history
  @level-fundamental
  @mode-differential
  Scenario: Replaying the script leaves the same store, calls, envelopes and refusals
    Given the lifecycle script shared://🔓️lifecycle-script.json
    When each implementation replays every step against its in-memory store, provider and emitter
    Then every implementation projects the same step outcomes, the same stored documents, the same provider calls and the same emitted envelopes

  @id-management-can-be-switched-off
  @level-quick
  @mode-conformance
  Scenario: A history run with management switched off asks the provider for nothing
    Given the lifecycle script shared://🔓️lifecycle-script.json
    When each implementation replays every step with the no-management flag set on every input
    Then every implementation projects the same store and an empty provider call log
