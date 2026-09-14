@capability-repo.todos.lifecycle
@no-oracle-repo-todos-lifecycle
@comparison-ordered-json-v1
Feature: Creating, changing, promoting and deleting a todo has one outcome
  Every effect of the todo lifecycle is a port, so the whole outcome can be observed: the files the
  tree is left holding, the tickets the opener was asked to open, the envelopes the emitter
  received and the class of every refusal. Where a todo is written is decided by what its parent
  is — a directory gets an item appended to its own `.todos.md`, a file gets a comment appended in
  that file's own comment style, and a parent that is neither is refused. Promoting a todo into a
  ticket puts the todo's description at the head of the ticket prompt and then removes the todo,
  so promoting the same todo twice cannot succeed twice. No third party has a todo aggregate to
  compare against; see `repo-todos-lifecycle`.

  @id-the-script-produces-one-history
  @level-fundamental
  @mode-differential
  Scenario: Replaying the script leaves the same tree, tickets, envelopes and refusals
    Given the lifecycle script shared://🔓️lifecycle-script.json
    When each implementation replays every step against its in-memory tree, ticket opener and emitter
    Then every implementation projects the same step outcomes, the same files, the same opened tickets and the same emitted envelopes
