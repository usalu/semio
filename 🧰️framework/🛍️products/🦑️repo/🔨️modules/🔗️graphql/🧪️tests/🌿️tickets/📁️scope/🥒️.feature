Feature: Repository ticket requests retain their selected root
  Scenario: Interleaved repositories keep independent lifecycle documents and draft attachments
    Given two explicit private repositories with distinct draft contents
    When both open the same title and only the first changes closes reopens and deletes its ticket
    Then the first follows the portable interaction and draft vectors
    And the second document and ambient repository remain unchanged
  Scenario: Invalid interaction metadata does not rename or persist a ticket
    Given an open ticket in the first repository
    When its title and an invalid client are submitted together
    Then no document or folder is changed
