@capability-repo.test-runner.scope-identifiers
@no-oracle-repo-test-runner-scope-identifiers
@comparison-ordered-json-v1
Feature: Two implementations agree on how a scope selector is normalised
  Before a selector can be resolved into a scope, it is normalised: flattening keeps alphanumerics
  and every non-ASCII rune and lowercases the rest, and URI path decoding restores the spaces that
  path-to-URI encoding replaced with %20. Both rules are semio's own, so there is no external
  reference — but both are reachable today from two independently written implementations, the
  committed Go client and the new Rust crate, which is what this case compares. It is deliberately
  the only part of the test-runner domain the Go adapter can exercise before the `go-split` wave
  moves the domain's unexported functions into `github.com/usalu/semio/repo/testrunner`.

  @id-flattening-agrees-across-implementations
  @level-fundamental
  @mode-differential
  Scenario: Flattening every recorded selector answers the same in Go and in Rust
    Given the recorded scope selectors shared://🧬️scope-selectors.json
    When each implementation flattens every selector with its own implementation of the rule
    Then both project the same flattened list, emoji and all

  @id-uri-path-decoding-agrees-across-implementations
  @level-fundamental
  @mode-differential
  Scenario: Decoding every recorded selector as a URI path answers the same in Go and in Rust
    Given the recorded scope selectors shared://🧬️scope-selectors.json
    When each implementation decodes every selector segment by segment
    Then both project the same decoded list, with %20 restored to a space in every segment
