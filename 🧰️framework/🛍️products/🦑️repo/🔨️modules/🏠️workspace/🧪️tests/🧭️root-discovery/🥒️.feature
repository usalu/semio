@capability-repo-workspace-root-discovery
@capability-repo-workspace-layout
@capability-repo-workspace-config
@no-oracle-repo-workspace-root-discovery
@comparison-ordered-json-v1
Feature: A monorepo root is discovered, its layout is named, and its settings are read
  Discovery walks upwards three times: first for the legacy `repo/cli/main.go` entry point, then for
  a `.git` checkout, then for a `go.mod`; the first hit wins, and the starting directory itself is
  the fallback. The layout under `.🧬semio/🦑️repo` is a fixed vocabulary — tickets, goals, devs, the
  file index, the settings file — and `📋️config.toml` contributes only its `[logging]` section, with
  every recognised affirmative spelling. Nothing outside this repository knows any of these
  conventions, so there is no oracle; see the recorded decision `repo-workspace-root-discovery`.

  @id-the-first-marker-upwards-wins
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: Each workspace shape resolves to the recorded root
    Given the shared tree descriptions shared://📡️root-discovery-trees.json
    When the host materialises every tree inside its work directory and runs discovery from the recorded start
    Then every implementation projects the same root per tree

  @id-the-layout-vocabulary-is-fixed
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: The layout paths compose from the root in one fixed way
    Given the shared tree descriptions shared://📡️root-discovery-trees.json
    When the host asks for every layout path of a known root
    Then every implementation projects the same path per layout key

  @id-settings-fall-back-to-the-defaults
  @level-quick
  @mode-conformance
  @seed-1
  Scenario: A missing, empty or unrelated settings document leaves the defaults in place
    Given the shared tree descriptions shared://📡️root-discovery-trees.json
    When the host writes each settings document under a materialised root and loads it
    Then every implementation projects the same settings per document
