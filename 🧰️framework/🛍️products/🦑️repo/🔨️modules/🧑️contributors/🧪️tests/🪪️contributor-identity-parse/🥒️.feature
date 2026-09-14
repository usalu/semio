@capability-repo.contributors.identity
@oracle-ajv-contributor-document
@comparison-ordered-json-v1
Feature: An author line resolves to the contributor who wrote it
  Git names an author as `Name <email>` on a line that a log or a shortlog prefixes with a number.
  A contributor claims any number of names, emails and handles in their `🧑️‍💻️contributor.json`, and
  resolving one to the other is what turns a commit into a person: an email match wins over a name
  match, both are case-insensitive across the primary value and every alternative, and an author
  nobody claims falls back to the line itself rather than to a wrong alias. The fixtures are the
  committed documents of every dev in this repository and real `git log` and `git shortlog` output,
  so neither side of the match was written by an implementation, and `ajv` decides whether each
  document actually satisfies `🧬️schema/🔣️.json`.

  @id-author-lines-resolve-to-aliases
  @level-fundamental
  @mode-differential
  Scenario: Splitting and resolving every identity line agrees everywhere
    Given the contributor documents shared://🧑️‍💻️contributor-documents.json
    And the real git identity lines shared://🏁️checkpoint-log.json
    And the contributors schema asset://🧬️schema/🔣️.json
    When each implementation splits every identity line and resolves the name and email it carries against the documents
    Then every implementation projects the same split, the same alias per line and the same contributor list, and the oracle finds every document schema-valid
