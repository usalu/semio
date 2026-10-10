Feature: Replication Representation Ownership
  Scenario: Semantic mutation contracts do not own wire representations
    Given the language-neutral representation ownership fixture
    Then text, binary, and diff codec contracts are owned by I/O
    And physical operation octets are owned by binary I/O
    And no codec contract is declared or aliased by semantic mutations
    And an independent minimatch oracle identifies the same ownership boundary

  Scenario: Canonical Mutation Representation Belongs To Text IO
    Given original semantic edit and mutation metadata owners
    When their canonical JSON fields and framed hash are projected
    Then every JSON tree projection and hash cursor belongs to text IO
    And semantic mutation owners contain no JSON tree implementation
    And the previous physical mutation seal owner is absent
    And independent minimatch accepts the same text IO ownership

  Scenario: Causal Mutation Types Do Not Own Binary Representations
    Given original causal envelopes and frontier domain types
    When wire and storage consumers encode or decode those values
    Then HLC, envelope, frontier, batch and operation payload codecs belong to binary IO
    And causal mutation semantics contain no binary codec or compatibility alias
    And the original bounded batch corpus retains its exact ceilings and malformed inputs
    And closed neutral ownership agrees with an independent Ajv schema oracle
