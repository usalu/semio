@capability-repo-move-rename-casings
@no-oracle-repo-move-token-casings
@comparison-ordered-json-v1
Feature: A token is renamed in every casing it is spelled in, contents and names alike
  The repository-wide rename rewrites three foldings of a token and no others: the whole token
  upper-cased, the whole token lower-cased, and the lower-cased token with its first character
  upper-cased. A kebab, snake or screaming spelling therefore renames cleanly, while a camel or
  pascal spelling only renames through its upper and lower foldings — a deliberate limit, recorded
  here so it stays a decision. Over a workspace the same rewrite runs over every content first, then
  over every file and folder name, deepest path first so a parent rename never invalidates a child.
  The `.git` and `node_modules` directories are never entered. A scope narrows where occurrences are
  rewritten; it does not exempt the scope itself, so the scope root is renamed exactly when its own
  name carries the token — which is why the round trip renames back under the rewritten scope. See
  the recorded decision `repo-move-token-casings`.

  @id-every-spelling-folds-the-same-way
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: Each token vector rewrites to the recorded text
    Given the shared vector set shared://🔤️rename-vectors.json
    When the host applies the casing rewrite to each vector content
    Then every implementation projects the recorded text per vector

  @id-a-workspace-renames-deepest-first
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: Each workspace vector reaches the recorded tree, counters and output line
    Given the shared vector set shared://🔤️rename-vectors.json
    When the host plans the rename over each before-tree and applies the plan
    Then every implementation projects the recorded tree, counters and output line per vector

  @id-renaming-back-restores-the-workspace
  @level-fundamental
  @mode-round-trip
  @seed-1
  Scenario: A rename is undone by renaming the new token back to the old one
    Given the shared vector set shared://🔤️rename-vectors.json
    When the host renames a workspace and then renames the new token back to the old one
    Then every implementation projects the original tree again

  @id-a-refused-rename-names-its-reason
  @level-fundamental
  @mode-error
  @seed-1
  Scenario: An empty, an identical or an unknown-scope rename is refused with the recorded message
    Given the shared vector set shared://🔤️rename-vectors.json
    When the host plans each refused rename
    Then every implementation projects the recorded refusal message per vector
