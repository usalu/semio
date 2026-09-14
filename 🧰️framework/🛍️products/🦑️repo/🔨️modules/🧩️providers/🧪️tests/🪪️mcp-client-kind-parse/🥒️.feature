@capability-repo.providers.mcp-client-identity
@no-oracle-repo-mcp-client-identity
@comparison-ordered-json-v1
Feature: Which IDE is talking is one closed vocabulary with one spelling rule
  A repo MCP entry point, a hook client slug and an editor provider all name the same eight IDEs, and
  the mapping between those three spellings is this repository's own. Nothing outside it implements
  the mapping, so there is no reference to compare against — see the recorded no-oracle decision
  `repo-mcp-client-identity`. Two independently written implementations, Rust and Go, answer for
  every accepted spelling, for the whitespace and case folding the parser promises, and for the
  inputs that must be refused rather than folded into the generic surface.

  @id-every-accepted-spelling-parses-to-one-kind
  @level-fundamental
  @mode-differential
  Scenario: Case and surrounding whitespace never change which kind an input names
    Given the client identity vectors local://🪪️client-kinds.json
    When every input is parsed
    Then every implementation projects the same kind, or the same refusal

  @id-each-kind-names-one-server-and-one-hook-client
  @level-fundamental
  @mode-differential
  Scenario: Each kind names exactly one MCP server and one hook client slug
    Given the client identity vectors local://🪪️client-kinds.json
    When every kind is asked for its server name and its hook client slug
    Then every implementation projects the same pair for every kind

  @id-a-resolved-client-maps-back-to-a-kind
  @level-fundamental
  @mode-differential
  Scenario: A validated ticket client slug maps back onto the surface that serves it
    Given the client identity vectors local://🪪️client-kinds.json
    When every resolved client slug is mapped back to a kind
    Then every implementation projects the same kind, with the generic surface for the slugs that have no MCP entry point

  @id-an-unknown-spelling-is-refused
  @level-fundamental
  @mode-error
  Scenario: An unknown spelling is refused instead of silently becoming the generic surface
    Given the client identity vectors local://🪪️client-kinds.json
    When an input outside the vocabulary is parsed
    Then every implementation projects a refusal rather than a kind
