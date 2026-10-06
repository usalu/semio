Feature: Print Chart Representation Ownership
  Scenario: Chart mutation and snapshot schemas own only semantic declarations
    Given the authored print chart ownership fixture
    Then text and binary codecs are owned by their I/O representation
    And relational SQLite codecs are owned by SQLite I/O
    And snapshot and mutation schemas contain no codec mounts or exports
    And an independent minimatch oracle agrees with the ownership boundary
