@capability-repo.tickets.id-scheme
@no-oracle-repo-ticket-id-scheme
@comparison-ordered-json-v1
Feature: A ticket is named by a date and a slug, in two spellings of the same identity
  `26/09/06/SOME-TICKET` is what a dev types and what `repo://ticket/` carries; `🎆️26/🌙️09/☀️06/SOME-TICKET`
  is the folder it lives in, and every path the ticket domain owns hangs off that folder. The slug comes
  from the title through the repository's own camel-case-aware upper-kebab rule, and a child ticket
  carries its parent's slug as a path prefix. Nothing outside this repository names tickets this way, so
  there is no reference to compare against — see the recorded decision `repo-ticket-id-scheme`; the
  evidence is two independently written implementations answering for the same vectors, plus the
  round-trip law that reading either spelling back reproduces the identity exactly.

  @id-both-spellings-name-the-same-identity
  @level-fundamental
  @mode-round-trip
  Scenario: Parsing either spelling and re-rendering it reproduces the same identity
    Given the identity vectors local://🪪️id-vectors.json
    When every id is parsed and rendered back in both spellings
    Then every implementation projects the same logical id, folder path and URI for each vector

  @id-a-malformed-id-is-refused
  @level-fundamental
  @mode-error
  Scenario: An id that is not a date and a slug is refused rather than repaired
    Given the identity vectors local://🪪️id-vectors.json
    When every malformed id is parsed
    Then every implementation projects the same refusal class for each vector

  @id-a-title-becomes-one-slug
  @level-fundamental
  @mode-property
  Scenario: A title becomes exactly one upper-kebab slug, and the rule is idempotent
    Given the identity vectors local://🪪️id-vectors.json
    When every title is slugified and the result is slugified again
    Then every implementation projects the same slug, and the second pass changes nothing

  @id-an-emoji-and-title-pair-is-validated-together
  @level-fundamental
  @mode-error
  Scenario: A ticket needs exactly one entity emoji and a title with something in it
    Given the identity vectors local://🪪️id-vectors.json
    When every emoji and title pair is validated
    Then every implementation projects the same slug or the same refusal message for each pair

  @id-every-owned-path-hangs-off-the-folder
  @level-fundamental
  @mode-conformance
  Scenario: The layout derives every ticket-owned path from one meta directory
    Given the identity vectors local://🪪️id-vectors.json
    When the tickets directory, folder, document, important directory and important document are derived for every id
    Then every implementation projects the same five paths for each id
