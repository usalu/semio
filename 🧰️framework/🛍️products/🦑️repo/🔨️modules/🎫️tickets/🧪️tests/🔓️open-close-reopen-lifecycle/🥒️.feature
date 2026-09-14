@capability-repo.tickets.lifecycle
@no-oracle-repo-ticket-lifecycle
@comparison-ordered-json-v1
Feature: A ticket opens, closes, reopens and changes without anything on disk
  The whole lifecycle runs against an in-memory store, a frozen clock, a null issue tracker and a
  recording event sink, so a case never touches `.🧬semio` and never needs `gh`. What is projected is
  the observable outcome of each step: the document that was written, the paths the store holds
  afterwards, the transaction journal, and the event kinds and payloads that were emitted. Nothing
  outside this repository runs this lifecycle, so there is no reference — see the recorded decision
  `repo-ticket-lifecycle`. The frozen request vectors are the specification these scenarios conform
  to; a second, independently written implementation joins as a differential subject once the Go twin
  of `github.com/usalu/semio/repo/tickets` exists.

  @id-an-open-materialises-the-folder-and-emits
  @level-fundamental
  @mode-conformance
  Scenario: Opening writes the document, creates the empty important document and emits one event
    Given the lifecycle vectors local://🔓️lifecycle.json
    When the open request is executed against an in-memory store
    Then every implementation projects the same document, the same store paths and the same emitted event

  @id-a-close-consumes-the-important-document
  @level-fundamental
  @mode-conformance
  Scenario: Closing removes the important document bundle and records the summary
    Given the lifecycle vectors local://🔓️lifecycle.json
    When the open request is executed and then the close request
    Then every implementation projects the closed document, the journal of the transaction and the store paths that remain

  @id-a-reopen-restores-the-important-document
  @level-fundamental
  @mode-conformance
  Scenario: Reopening recreates the empty important document and appends a reopen interaction
    Given the lifecycle vectors local://🔓️lifecycle.json
    When the open, close and reopen requests are executed in order
    Then every implementation projects the reopened document, the recreated paths and the three emitted event kinds

  @id-a-change-renames-the-folder
  @level-fundamental
  @mode-conformance
  Scenario: Changing the title moves the whole ticket folder to the new slug
    Given the lifecycle vectors local://🔓️lifecycle.json
    When the open request is executed and then the change request
    Then every implementation projects the new id, the moved store paths and the changed document

  @id-a-close-refuses-a-ticket-that-is-not-open
  @level-fundamental
  @mode-error
  Scenario: A second close, a close with no summary and a close with no files are all refused
    Given the lifecycle vectors local://🔓️lifecycle.json
    When each malformed close is executed against an already closed and an open ticket
    Then every implementation projects the same refusal class and message for each

  @id-file-inputs-are-normalised-before-they-are-recorded
  @level-fundamental
  @mode-conformance
  Scenario: Repo-relative, dotted, file-URI and excluded paths collapse to one ordered set
    Given the lifecycle vectors local://🔓️lifecycle.json
    When the close request's file identifiers are normalised
    Then every implementation projects the same de-duplicated, ordered list

  @id-an-oversized-artifact-is-purged
  @level-quick
  @mode-conformance
  Scenario: A closed ticket loses its oversized artifacts but never its document
    Given the lifecycle vectors local://🔓️lifecycle.json
    When a ticket folder holding an oversized file and a small one is purged
    Then every implementation projects the same removed paths and the same surviving paths
