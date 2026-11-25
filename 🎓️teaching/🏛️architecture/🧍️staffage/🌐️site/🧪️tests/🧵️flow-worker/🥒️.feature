Feature: The air flow worker is browser code
  Scenario: Worker plugins
    Given the site's build configuration
    Then the worker bundle replaces the Node cast exporter with the same browser stand-in as the page bundle
  Scenario: Worker bundle
    When the air flow worker is bundled with the worker plugins of the site
    Then the bundle is produced and reaches no Node built-in module
