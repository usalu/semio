@capability-repo.goals.document-codec
@oracle-ajv-goal-document
@comparison-ordered-json-v1
Feature: A stored goal document decodes and re-encodes to the same bytes
  A `🎯️goal.json` is the goal: the identifier comes from the directory the document sits in, the
  stored `parent` member is the 🎯️-tagged compose form of that directory's parent, and everything
  else is the goal's own state. Rewriting one must reproduce the file byte for byte — two space
  indentation, members in declaration order, the `omitempty` rule that decides whether a member
  appears at all, and a trailing newline — or the next commit is a diff nobody asked for. The
  fixtures are real documents copied verbatim out of `.🧬semio/🦑️repo/🎯️goals`, so the bytes were
  written by neither implementation, and `ajv` decides whether each one actually satisfies
  `🧬️schema/🔣️.json` before re-serialising it. The identifier's own parent path is projected
  beside the stored compose form, so the derivation is judged against the document rather than
  against itself.

  @id-stored-documents-round-trip
  @level-fundamental
  @mode-differential
  Scenario: Decoding a stored document and encoding it again reproduces it
    Given the real goal documents shared://🎯️goal-documents.json
    And the goals schema schema://repo.goals/GoalDocument
    When each implementation decodes every document under the identifier of its directory and encodes it again
    Then every implementation projects the same bytes and the same members for every document, and the oracle finds every document schema-valid
