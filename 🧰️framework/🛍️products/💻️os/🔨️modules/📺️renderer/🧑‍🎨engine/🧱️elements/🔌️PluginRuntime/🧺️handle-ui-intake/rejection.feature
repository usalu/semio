Feature: A guest UI patch the shell refuses becomes the visible error
  Scenario: The boot refresh names the refused surface, contract rule and node
    Given a guest publishes a surface patch the retained-UI contract refuses
    When the intake rejects it
    Then a console diagnostic names the surface, rule and node once
    And every later refresh that fails while that surface is unpublished fails with the rejection, not only as missing
    And the shell's error card shows the rejection as a localized notice in English and German with its code and surface
  Scenario: A surface that later installs forgets its rejection
    Given a recorded rejection for a surface
    When a later patch of that surface installs
    Then a failing refresh reports its own failure again
