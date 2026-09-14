@capability-repo.todos.drafts
@no-oracle-repo-todos-drafts
@comparison-ordered-json-v1
Feature: A draft is the slug of its title and the files copied under it
  A draft is a directory of copied files addressed by the slug of the title it was created from,
  which makes three refusals part of the contract rather than accidents: a title carrying no slug
  has no identifier to take, a title whose slug is already taken would silently replace someone
  else's draft, and a file is copied under its base name so a deep source path collapses to one
  name. Removing a draft that was never there is not a refusal — it is the state the caller asked
  for. Nothing outside this repository has that notion of a draft; see `repo-todos-drafts`.

  @id-the-draft-script-produces-one-history
  @level-fundamental
  @mode-differential
  Scenario: Replaying the script leaves the same drafts, files, identifiers and refusals
    Given the draft script shared://✏️draft-script.json
    When each implementation replays every step against its in-memory draft store
    Then every implementation projects the same step outcomes, the same stored files and the same artifact identifier and URI per draft
