@capability-repo.test-runner.runner-detection
@no-oracle-repo-test-runner-planning
@comparison-ordered-json-v1
Feature: A bundle's manifests decide which test runner reaches it
  Detection reads nothing but the manifest files a bundle root carries, in a fixed order:
  go.mod, then Cargo.toml, then *.csproj, then *.sln, then package.json, then pyproject.toml or
  requirements.txt. A JavaScript bundle is narrowed a second time by the text of its package.json:
  vitest wins over jest, jest wins over a bare "test" script, and a bundle with no readable manifest
  still falls back to vitest. Nothing outside this repository defines that order, so the reference is
  the vector table in shared://🧭️detection-vectors.json, transcribed from the Go implementation this
  port reproduces (recorded decision `repo-test-runner-planning`).

  @id-manifest-selects-the-language
  @level-fundamental
  @mode-conformance
  Scenario: Every manifest shape resolves to the language the vector table declares
    Given the recorded detection vectors shared://🧭️detection-vectors.json
    When the host detects the bundle language of every vector's bundle root
    Then each answer equals the language the vector declares, and a bundle with no manifest answers nothing

  @id-javascript-manifest-selects-the-runner
  @level-fundamental
  @mode-conformance
  Scenario: A JavaScript bundle's manifest text selects the runner and its arguments
    Given the recorded detection vectors shared://🧭️detection-vectors.json
    When the host detects the JavaScript runner for every vector that declares an expected argv
    Then each answer equals the argv the vector declares, including where a filter adds -t

  @id-detection-is-idempotent-and-filter-only-appends
  @level-quick
  @mode-property
  Scenario: Detecting twice answers the same, and a filter only ever appends arguments
    Given the recorded detection vectors shared://🧭️detection-vectors.json
    When the host detects every bundle twice, once without a filter and once with one
    Then both passes agree, and the filtered argv keeps the unfiltered argv as its prefix
