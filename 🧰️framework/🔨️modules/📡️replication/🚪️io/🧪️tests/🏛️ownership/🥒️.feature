Feature: Replication Representation Ownership
  Scenario: Semantic mutation contracts do not own wire representations
    Given the language-neutral representation ownership fixture
    Then text, binary, and diff codec contracts are owned by I/O
    And physical operation octets are owned by binary I/O
    And no codec contract is declared or aliased by semantic mutations
    And an independent minimatch oracle identifies the same ownership boundary
