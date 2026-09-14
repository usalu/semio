@capability-repo.contributors.session-kind
@no-oracle-repo-contributors-sessions
@comparison-ordered-json-v1
Feature: A session directory says for itself whether it ran, finished or was interrupted
  Nothing records the fate of a session; it is read back out of the directory the session wrote.
  A recorded agent-ended event completes it, a write inside the last thirty minutes leaves it
  running, and anything else interrupted it — and when the `session.json` cannot be read at all the
  file names and their age have to answer the same question. The client and the earliest second
  follow the same three-step fallback, and the artifact identifier a session takes depends on
  whether it recorded a checkpoint. Nothing third party knows any of that; see
  `repo-contributors-sessions`.

  @id-every-branch-of-the-derivation-agrees
  @level-fundamental
  @mode-differential
  Scenario: Reading each session directory yields one kind, client, second and identifier
    Given the session directories shared://📡️session-vectors.json
    When each implementation derives the kind, client, earliest second and checkpoint of every session
    Then every implementation projects the same session records, artifact identifiers and URIs

  @id-the-kind-emoji-comes-from-the-shared-vocabulary
  @level-quick
  @mode-conformance
  Scenario: Every kind renders as the entity emoji the identity table declares
    Given the session directories shared://📡️session-vectors.json
    When each implementation renders the emoji of every derived kind and of the absent kind
    Then every implementation projects the same emoji per kind
