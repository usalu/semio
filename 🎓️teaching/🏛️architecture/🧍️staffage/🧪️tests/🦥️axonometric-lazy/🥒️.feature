@capability-staffage-axonometric
Feature: The axonometric depiction is loaded on demand
  The volume solids, the kernel tessellation and the hidden-line removal of the axonometric viewpoint are a chunk of their own, so a program that draws the planar viewpoints only never loads them. `loadAxonometric` and the asynchronous `depictAsync` and `modelSceneAsync` load it once.

  Scenario: Planar viewpoints need nothing
    Given a posed figure and nothing loaded
    When the side elevation is drawn
    Then it is drawn without the axonometric module

  Scenario: A synchronous axonometric depiction before the load is refused by name
    Given a posed figure and nothing loaded
    When the axonometric viewpoint is drawn synchronously
    Then it throws an error that names loadAxonometric and depictAsync in English and German

  Scenario: The asynchronous depiction loads the module
    Given a posed figure
    When the axonometric viewpoint is drawn through depictAsync
    Then the picture is the one the synchronous depiction draws afterwards

  Scenario: Loading twice loads once
    Given a loaded module
    When it is loaded twice at once and a CAD scene is asked asynchronously
    Then the CAD scene equals the synchronous one
