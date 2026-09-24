@capability-repo.todos.line-grammar
@oracle-ajv-todo-line-vectors
@comparison-ordered-json-v1
Feature: Rewriting the line a todo is and reading it back agrees
  Changing a todo is not a write to a record, it is a rewrite of the one line the todo is: the
  `- TODO Name: description` item of a `.todos.md`, or the `// TODO Name: description` comment of a
  source file. Both rewrites have to leave every other line of the document untouched, keep the
  indentation and the comment opener the line already had, and produce a line the grammar reads
  back as the todo that was asked for — a rewrite the parser no longer recognises has destroyed
  the todo it claimed to change. The refusals belong to the same contract: a name no line carries,
  a line number outside the document and the one based zero. `ajv` decides whether the vectors
  actually satisfy `🧬️schema/🔣️.json`, and the grammar itself is restated in the oracle with a
  JavaScript regular expression rather than either hand written scanner.

  @id-rewriting-a-line-and-reading-it-back-agrees
  @level-fundamental
  @mode-round-trip
  Scenario: Every rewrite writes a line the grammar reads back
    Given the line vectors shared://📝️line-vectors.json
    And the todos schema schema://repo.todos/Todo
    When each implementation parses both documents, applies every rewrite and every removal, and parses the result again
    Then every implementation projects the same parses, the same rewritten documents, the same refusals and the same comment opener per path
