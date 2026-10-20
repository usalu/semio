Feature: Native Source Observation Scope
  Scenario: Declaration and Source tasks do not prepare unrelated native work
    Given a selected Source task whose freshness files include workspace Cargo metadata
    When a dependency owner is admitted for its declaration alone
    Then its native workspace is not expanded
    And explicitly requested named and caret source inputs retain structural dependency facts
    And selected native tests retain their original dev closure and preparation targets

  Scenario Outline: Selected project assembly retains its closed owner admission
    Given a closed <selection> with selected <targets> and requested <inputs>
    When actual project nodes normalize the full authored declaration
    Then inactive declarations remain available for display
    And native owner discovery runs only for all-owner selection
    And only selected non-Source operations receive native preparation
    And only selected release operations receive release preparation
    And newly admitted helper operations and metadata remain in the graph

    Examples:
      | selection | targets       | inputs  |
      | selected  | none          | none    |
      | selected  | source        | none    |
      | selected  | none          | default |
      | selected  | test          | none    |
      | selected  | source,test   | none    |
      | selected  | release       | none    |
      | all       | every declared| none    |

  Scenario: Image observation laws demand runtime Source and preserve explicit compilation
    Given the actual native-image-observation route runs Bun tests and excludes TypeScript noEmit
    When its authored own-file input is observed
    Then the input is freshness-only and the full runtime command closure is still traced
    And an explicit compilation entry remains compilation-demanded

  Scenario: Freshness-only tasks retain exact data without native package traversal
    Given a selected Source task or inherited DATA-only named input
    When its closed observation role is freshness
    Then matching files and actual runtime imported owners remain admitted
    And package declaration tables do not introduce unrelated native source graphs
    And generated self-output refusal remains strict
    And later selected Native work upgrades and reobserves its complete native closure
