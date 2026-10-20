Feature: Read-Only Native Image Ancestry
  Scenario: Admit only the same ordered ancestor inode chain
    Given a current closed native image publication
    When its boot or parent mount device namespace changes
    Then its unchanged ancestry binds without rewriting the publication
    And a missing, reordered, malformed or replaced ancestor refuses
    And live image, receipt and kernel mapping checks remain required
