@capability-test-host-protocol
@no-oracle-repo-test-platform
@comparison-ordered-json-v1
Feature: Every native test host implements the same protocol
  The five native hosts (Rust, TypeScript, Go, Python, .NET) each own an independently written
  implementation of the content digest, the fixture resolver and the result shape. Nothing third
  party implements this repository's host protocol, so there is no credible oracle — see the
  recorded no-oracle decision `repo-test-platform`. Confidence comes instead from pairwise
  equivalence: five implementations written from the same frozen contract must project identically.

  @id-digest-and-fixture-resolution
  @level-fundamental
  @mode-differential
  @seed-42
  Scenario: The digest, the fixture resolver and the seed agree across implementations
    Given the shared conformance vector shared://📄️protocol-vector.txt
    When the host computes the owned content digest of the vector and of the literal "semio"
    Then every implementation projects the same digests, fixture name and parsed seed

  @id-fixture-not-in-plan-is-an-error
  @level-fundamental
  @mode-error
  Scenario: Resolving an undeclared fixture is an error, never a silent default
    Given a fixture URI that the plan does not declare
    When the host asks the resolver for it
    Then the resolver reports a failure instead of returning a default path

  @id-work-directory-is-cache-local
  @level-quick
  @mode-conformance
  Scenario: The work directory a host writes into is inside the marked test cache
    Given the plan's work directory
    When the host inspects it
    Then it lies under the repository test cache and carries the ownership marker
