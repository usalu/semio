@capability-repo.tickets.important-document-transaction
@no-oracle-repo-ticket-important-document
@comparison-ordered-json-v1
Feature: The important document is a transaction, not a file that happens to be deleted
  An open ticket owns exactly one empty regular file at `📌️important/📝️.md`, and it is the ticket's
  proof of life: a close must find that exact bundle — a regular file, empty, and the only child of
  its directory — before it removes it, and a reopen must recreate it and remove ONLY what it created
  if the save then fails. Four ways the bundle can be wrong are refused rather than repaired, and the
  journal records every step in the order it was attempted, so a partial transaction is visible rather
  than inferred. Nothing outside this repository has this rule, so there is no reference — see the
  recorded decision `repo-ticket-important-document`.

  @id-a-wrong-bundle-is-refused-before-anything-is-removed
  @level-fundamental
  @mode-error
  Scenario: A missing, non-empty, non-file or accompanied document stops the close
    Given the transaction vectors shared://💾️important-document-transaction/💾️cases.json
    When a close is attempted against each of the four wrong bundles
    Then every implementation projects the same refusal message and leaves the store untouched

  @id-a-failed-save-rolls-the-creation-back
  @level-fundamental
  @mode-error
  Scenario: A reopen whose save fails removes exactly the document and directory it created
    Given the transaction vectors shared://💾️important-document-transaction/💾️cases.json
    When the document write is made to fail and a reopen is attempted
    Then every implementation projects the injected failure, the rollback journal and a store with neither the document nor its directory

  @id-a-preserved-document-is-not-rolled-back
  @level-fundamental
  @mode-conformance
  Scenario: A reopen that found an existing document leaves it alone when the save fails
    Given the transaction vectors shared://💾️important-document-transaction/💾️cases.json
    When an important document already exists, the save is made to fail and a reopen is attempted
    Then every implementation projects a preserved journal step and a store that still holds the document

  @id-the-journal-records-every-step-in-order
  @level-fundamental
  @mode-conformance
  Scenario: A successful close and reopen record their steps in the order they were attempted
    Given the transaction vectors shared://💾️important-document-transaction/💾️cases.json
    When a ticket is opened, closed and reopened
    Then every implementation projects the same ordered journal for each step
