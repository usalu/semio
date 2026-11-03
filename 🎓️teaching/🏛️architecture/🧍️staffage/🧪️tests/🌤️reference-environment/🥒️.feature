Feature: Reference environment of an asset
  An asset may declare the ambient case its standalone thermal previews use. The export command line takes it by default and an explicit preset overrides it. Without either, the asset's own thermal environment applies.

  Scenario: the collector is sunny by declaration
    Given the solar thermal collector declares the preset "summer-heatwave-calm"
    When it is exported in the thermal view without --environment
    Then the drawing equals the library drawing under 900 W/m2 and the absorber is warmer than under its sunless default

  Scenario: an explicit preset overrides the declaration
    Given the same export with --environment "office-neutral"
    Then the drawing equals the library drawing under "office-neutral"

  Scenario: no declaration, no flag
    Given the tealight declares no reference environment
    When it is exported in the thermal view
    Then the drawing equals the library drawing under its own thermal environment

  Scenario: refusals
    Given --environment with another view, or an unknown preset
    Then the command refuses and names the offending flag or the known presets

  Scenario: validation
    Given a reference environment that names no preset
    Then the validator reports an unresolved reference at "thermal.referenceEnvironment"
