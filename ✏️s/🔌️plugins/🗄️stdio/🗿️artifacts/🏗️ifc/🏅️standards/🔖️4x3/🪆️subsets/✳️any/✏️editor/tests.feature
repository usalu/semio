Feature: IFC declaration controls remain reachable through domain mutations
  Scenario: Type changes preserve an entity throughout the retained transaction
    Given a wall referenced by another entity with an exact identity above 2^53
    When the Door type variant is selected from details
    Then only sparse name and attribute mutations are emitted
    And every intermediate diff retains the entity and incoming reference
    And inherited identity, name and tag values are preserved
    And values in different declared domains are initialized from the target declaration
    And inverse diffs restore the complete original snapshot

  Scenario: Whole entity controls retain a changed binary64 sign bit
    Given a Cartesian point coordinate with positive zero
    When its whole entity value is changed to negative zero
    Then one sparse argument mutation carries the changed sign bit
    And the inverse restores the original positive zero bit pattern

  Scenario: The first complex constituent is created from an ordinary entity
    Given an entity with an empty omitted complex list
    Then its details object exposes an empty complex array
    When a constituent is inserted through the complex row
    Then the shared optional-field resolver yields SetEntityComplex
    And central diff application creates the constituent atomically

  Scenario: Every complex constituent uses its partial attribute layout
    Given IFCWALL is the first component of a complex record
    Then its argument zero is presented as PredefinedType
    And the enum input offers STANDARD

  Scenario: Complex type controls create the complete direct-attribute layout
    Given an empty geometric representation constituent
    When its type is selected or its IFC name is changed
    Then the direct declared attributes are created as adjustable controls
    And one central complex mutation preserves the entity identity and exact inverse
    And abstract partial types remain selectable for every complex constituent

  Scenario: Aggregate optionality belongs to the declared slot
    Given an optional TagList containing a required string element
    Then the TagList offers an unset state
    And the string element does not offer an unset state

  Scenario: Unique reference collections reach more than one target
    Given a unit assignment containing the first of two admitted unit entities
    When a new unit reference is inserted from details
    Then the insertion template selects the second unused target

  Scenario: An exact input appends references beyond exhausted suggestions
    Given a SET containing all references in the bounded suggestion window
    And an admitted unused entity outside that window
    When its exact positive decimal ID is submitted to Add Reference
    Then the retained SetValue action produces one SetEntityArg mutation
    And central application appends the reference and its inverse restores the graph
    And duplicate, missing, wrong-type and out-of-range IDs are refused

  Scenario: Binary values use the same central publication authority
    Given an empty IfcPropertySingleValue nominal value
    Then its SELECT control offers IFCBINARY
    When nonzero unused padding is submitted
    Then central diff application refuses the value
    And no document change is published

  Scenario: IFC4x3 model geometry and declaration controls share one editor
    Given the official buildingSMART ADD2 triangulated model
    When it is opened in the IFC4x3 editor
    Then the mesh window projects its eight vertices and twelve triangles
    And the details window edits that same artifact graph
    When geometry projection is cancelled
    Then its published scene contains a cancellation status and no mesh
