Feature: Discover current authored source through editable parameter suggestions
  Registered project owners and current physical source supply suggestions without another executable catalog.

  Scenario: Search current owners and source files
    Given the shared owner tree and authored source fixture
    Then the deepest registered project owns each suggested value
    And unknown owners offer no source values and report a problem
    And unselected owners can search across current projects
    And compiler configurations and canonical permanent scripts are discoverable
    And only direct runtime Bun test imports identify suggested tests
    And comments, string examples, regular expressions and type-only imports do not identify tests
    And an independent TypeScript parser and filesystem glob oracle agree

  Scenario: Preserve physical source boundaries and bounded discovery
    Given dependency, fixture, generated cache and temporary roots
    Then they supply no authored suggestions
    And nested authored build, dist and cache directories remain discoverable
    And Windows reparse points, junctions and symbolic links supply no outside source
    And entry, result, byte and metadata-poll limits remain explicit
    And partial reads and incomplete results are visibly reported

  Scenario: Share an index without sharing cancellation
    Given two forms subscribe to the same owner index
    When one form cancels or closes
    Then the other form continues discovering current source
    And a subsequent form can subscribe without inheriting cancellation
    And physical source changes invalidate the bounded cache

  Scenario: Choose a source and retain explicit working-directory control
    Given a source suggestion with its current project and registered owner directory
    When a developer accepts it
    Then the related project and derived working directory are filled
    And selecting another source updates a derived working directory
    And an explicitly edited working directory is preserved
    And changing the project clears dependent derived source and directory values
    And an explicitly edited source remains under developer control
    And pointer and keyboard acceptance use the same transition
    And customized suggestion bindings appear in localized help
    And ordinary text cursor keys retain their editing behavior

  Scenario: Adopt a current registry while a form is open
    Given an open form with derived source values and explicit developer edits
    When command discovery replaces the registry
    Then the command and fields are rebound by their stable identities
    And current parameter declarations and mutation confirmation apply
    And derived source values and the replaced subscription are released
    And compatible manual edits and extra argument slots are retained
    And removed choice values are cleared
    And a removed command returns to browsing
    And changing locale with the same registry retains the form and subscription
