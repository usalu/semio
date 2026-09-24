@capability-repo.tickets.document-codec
@oracle-ajv-ticket-schema
@comparison-ordered-json-v1
Feature: The 🎫️ticket.json document reads and writes exactly the way it is committed
  The vectors are REAL documents copied byte for byte out of `.🧬semio/🦑️repo/🎫️tickets`, so the case
  cannot pass against a shape only this repository's own implementations believe in. `ajv` — a real
  draft 2020-12 validator — decides whether each one satisfies `🧬️schema/🔣️.json`, so a drifted schema
  fails here instead of agreeing with itself, and its `JSON.parse` is a second, independently written
  reader of the same bytes. Three behaviours are pinned that a naive port would get wrong: the status
  member is mandatory and closed, the encoder HTML-escapes `<`, `>` and `&` because Go's does, and
  every member the shape does not declare is read and then dropped.

  @id-real-documents-decode-to-the-same-ticket
  @level-fundamental
  @mode-differential
  Scenario: Every committed document decodes to the same ticket
    Given the committed documents shared://📄️ticket-document-codec/📄️documents.json
    And the ticket schema schema://repo.tickets/TicketDocument
    When each document is decoded
    Then every implementation projects the same title, emoji, status, description, summary, issue, goal, plan and sessions

  @id-encoding-is-go-marshal-indent
  @level-fundamental
  @mode-differential
  Scenario: Re-encoding a decoded document reproduces Go's two-space indented, HTML-escaped bytes
    Given the committed documents shared://📄️ticket-document-codec/📄️documents.json
    When each decoded document is encoded again
    Then every implementation projects the same bytes, with the three HTML characters written as their six-character escapes

  @id-unknown-members-are-dropped
  @level-fundamental
  @mode-round-trip
  Scenario: Encoding is idempotent, and the members the shape does not declare are gone
    Given the committed documents shared://📄️ticket-document-codec/📄️documents.json
    When each document is decoded, encoded, decoded and encoded again
    Then every implementation projects identical bytes for the two encodings and names the members that were lost

  @id-a-document-without-a-status-is-refused
  @level-fundamental
  @mode-error
  Scenario: A document with no explicit status, or a status outside the vocabulary, is refused
    Given the committed documents shared://📄️ticket-document-codec/📄️documents.json
    When each refused document is decoded
    Then every implementation projects a refusal rather than a defaulted ticket
