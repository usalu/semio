Feature: IFC 4x3 edits publish reversible sparse diffs
  Scenario: A named attribute changes without replacing the document
    Given the IFC4X3_ADD2 graph in the editing fixture
    When entity 1 argument 0 is set to the string "Edited"
    Then the mutation yields one modified argument for entity 1
    And central diff application agrees with the independent JSON Patch result
    And the inverse diff restores the exact input graph

  Scenario: Positional edits retain their original targets
    Given an argument list [10,20,30,40]
    When arguments 1 and 3 are removed and argument 2 becomes 35
    And arguments 5 and 15 are inserted at final positions 0 and 2
    Then the argument list is [5,10,15,35]
    And the inverse diff restores [10,20,30,40]

  Scenario: Complex and nested typed values are editable
    Given a typed aggregate and a complex constituent in the editing fixture
    When a nested value and the complex constituent arguments are changed
    Then both changes pass through the central diff applier
    And their inverse mutations restore the exact graph

  Scenario: Invalid edits have no effect
    Given the IFC4X3_ADD2 graph in the editing fixture
    When a diff duplicates an entity identity or targets a missing argument
    Then central application refuses the diff
    And the input graph remains unchanged

  Scenario: Binary attributes preserve exact bits
    Given a one-bit IFC binary value with canonical STEP content "31"
    When the value is assigned through a mutation
    Then the binary value and its inverse retain every bit
    And invalid unused-bit padding is refused atomically

  Scenario: History is revision checked and local first
    Given two edits to the same local graph
    When undo and redo are performed through central diff application
    Then the graph and revision match the published events
    And a stale revision is refused without changing history


  Scenario: A tiny value budget refuses a wide aggregate before visiting every child
    Given an aggregate with 100000 children and a maximum of 3 values
    When a diff or mutation is admitted
    Then at most 3 child values have been read
    And the original snapshot and all history state are unchanged
  Scenario: Public mutable results own every returned graph value
    Given an IFC graph and an unchanged diff
    When the returned graph is mutated
    Then the original graph is unchanged
  Scenario: Concurrent asynchronous commands publish one revision atomically
    Given two commands targeting the same original revision
    When both commands finish cooperative preparation
    Then exactly one event is published
    And the other command refuses its stale revision
  Scenario: Cancellation during undo redo or snapshot reading preserves history
    Given an event history with undo and redo entries
    When a cooperative operation is cancelled
    Then snapshot revision events undo and redo entries are unchanged


  Scenario: Caller mutation during capture cannot publish an invalid owned graph
    Given a valid snapshot and a mutable proposed integer value
    When a progress callback replaces that integer with a string during owned capture
    Then the operation either refuses or returns a structurally valid owned graph
    And history never publishes an invalid snapshot or consumes undo state
  Scenario: A recorded command is the captured command that produced its diff
    Given a command whose caller-owned value changes during cooperative capture
    When a valid event is published
    Then its recorded command and replay diff reproduce its owned snapshot
  Scenario: Base-free absorption admits native coordinate and identity domains
    Given a diff with NaN, negative or duplicate coordinates or overlapping targets
    When sparse diffs are absorbed
    Then admission rejects the malformed diff before algebra coalesces its targets


  Scenario: Omitted complex constituents mean the canonical empty collection
    Given an IFC entity whose optional complex field is omitted
    When the entity is captured edited and restored by its inverse
    Then its complex collection is empty
    And its transport validates against the native snapshot schema
    And independent JSON Patch agrees with the canonical graph


  Scenario Outline: Native shape admission agrees with the declared schema
    Given each of the eleven mutation branches and every header entity and list diff lane
    When <variation> changes each required or optional field
    Then synchronous and asynchronous admission agree with the native schema oracle
    And refused commands publish no event and preserve their original sources
    Examples:
      | variation |
      | missing |
      | undefined |
      | null |
      | wrong type |
      | extra property |
      | array hole |
      | undefined array item |
  Scenario: Every published recorded command satisfies its serialized transport schema
    Given a structurally admitted command
    When history publishes its event
    Then its recorded native and decimal JSON command shapes are valid

  Scenario: Shared graph and artifact document envelope have distinct identities
    Given a native IFC4x3 snapshot
    Then its graph identity is stdio.ifc
    And its host document envelope identity is s.stdio.ifc@4x3/*

