Feature: A long-lived reader of one component generation never blocks the publication of the next
  Scenario: A reader holds generation N while generation N+1 is published
    Given a plugin whose head names generation N and a live process holding the shared lease of N
    When generation N+1 is staged and promoted
    Then the publication completes without waiting for the reader
    And the bytes of N are untouched and N stays in the head's reclamation ledger
    And a reader that resolves the head afterwards is handed generation N+1

  Scenario: The holder leaves and the next publication reclaims what it held
    Given generation N is superseded and its holder has exited
    When generation N+2 is promoted
    Then no superseded generation remains and the ledger is empty

  Scenario: The same generation is published again while a reader holds it
    Given a live process holding the shared lease of the current generation
    When the identical bytes are staged as an immutable publication
    Then the existing directory is reused in place without waiting
    And replacing it through the exclusive path is blocked by the reader, which is the incident

  Scenario: Generation names equal an independent digest
    Given the receipt of a generation
    Then its directory name equals the SHA-256 digest computed by an independent implementation
