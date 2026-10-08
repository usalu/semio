Feature: All-App Discovery Proves Developer Reach
  `verify interactivity apps` discovers every plugin descriptor and proves that a developer can start
  every app context and every verification verb from the dashboard. It reads the plugin descriptors,
  the generated playground catalog, the verify router and the root project manifest.

  Scenario: Discover a plugin descriptor
    Given a schema-first plugin descriptor whose apps are labelled in English and German
    When the gate reads the descriptor
    Then it discovers every app up to the fixed capacity without a failure
    And its parse equals the TypeScript compiler's JSON projection

  Scenario: Reject a hostile descriptor
    Given descriptors that are empty, over capacity or unlabelled in German
    When the gate reads each descriptor
    Then each one is rejected with its own failure

  Scenario: Resolve an extension through its parent
    Given an extension descriptor beside the source that registers its bundle
    When the gate reads the bundle identifier and the parent activation
    Then the extension delegates its apps to the parent plugin
    But an extension with the plugin role or without a resolvable bundle is rejected

  Scenario: Join migrated actions to production commands
    Given descriptor actions with and without a migrated interactive-job disposition
    When the gate joins them to the accepted production commands
    Then an owner-local command or a shared reserved route satisfies a migrated action
    But a foreign command, a missing disposition or an action above the fixed capacity is rejected

  Scenario: Start every playground variant in each renderer
    Given a generated playground catalog whose variants declare a React and a WGPU port
    When the gate reads the catalog
    Then every variant can be started in React, WGPU Wasm and WGPU native
    And its rows equal the TypeScript compiler's evaluation of the catalog
    But a variant without two distinct valid ports is rejected with the manifest key to declare

  Scenario: Cover every app context with an owner-qualified playground variant
    Given an app descriptor and playground variants of its own and of another plugin
    When the gate matches the app context without its editor or viewer role
    Then a startable variant of the owning plugin covers it
    But a foreign, mismatched or unstartable variant leaves it uncovered with the manifest key to declare

  Scenario: Derive the verification verbs from the verify router
    Given the source of a verify router with one-word and two-word routes
    When the gate reads its route heads
    Then its routes equal the TypeScript compiler's syntax-tree projection
    But a route head in another shape, a repeated route or a missing router fails closed

  Scenario: Declare every verification verb as a dashboard argument form
    Given a root project manifest whose verify target declares one choice parameter
    When its values carry every route of the router and every required argument gate
    Then the gate accepts the declaration
    And the manifest satisfies the dashboard registry schema under Ajv
    But an undeclared, repeated, unanswered, split or malformed form is rejected with the value to declare

  Scenario: Read the workspace sources as the TypeScript compiler does
    Given the workspace's verify router and generated playground catalog
    When the gate and the TypeScript compiler project them independently
    Then both projections are equal
    And a declaration of exactly the derived routes and required argument gates is accepted
