@capability-graphql-mutation-execution
@no-oracle-graphql-owned-recording-context
@comparison-ordered-json-v1
Feature: A mutation returns its payload, changes the records and leaves an event behind
  The write half of the schema is not a second read: `ticketOpen`, `goalClose`, `todoChange`,
  `draftCreate`, `folderMove`, `sectionDelete` and the rest each return an aggregate AND change the
  repository. Two things therefore have to be measured together, because either alone can be right
  while the other is wrong: the payload the executor projects, and the state the context is left in.

  Every implementation builds ONE context from the frozen records shared://🔣️repo-records.json and
  runs the whole script shared://✏️mutation-execution/🔣️mutations.json against it in order, so a later mutation sees what
  an earlier one wrote — `goalChange` finds the goal `goalCreate` made, `ticketClose` closes the
  ticket `ticketOpen` opened, `integrate` reads the file `extract` produced.

  @id-script-changes-records-and-emits-events
  @level-fundamental
  @mode-conformance
  Scenario: The whole write script produces the same payloads, records and events
    Given the frozen repository shared://🔣️repo-records.json
    And the write script shared://✏️mutation-execution/🔣️mutations.json
    When each implementation runs every mutation of the script in order against one context
    Then every implementation returns the same payloads and leaves the same records and events

  @id-a-mutation-that-cannot-apply-is-refused
  @level-quick
  @mode-error
  Scenario: Closing a ticket that does not exist refuses instead of inventing one
    Given the frozen repository shared://🔣️repo-records.json
    When each implementation closes a ticket whose slug no record carries
    Then every implementation refuses with the same message and writes no event
