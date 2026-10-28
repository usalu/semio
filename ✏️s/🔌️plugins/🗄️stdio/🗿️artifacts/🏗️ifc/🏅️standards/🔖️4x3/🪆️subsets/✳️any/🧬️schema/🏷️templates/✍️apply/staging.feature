Feature: Released template staged authoring
 Scenario: Every declaration produces its own closed entry controls
  Given any of the 760 pinned released property or quantity templates
  When the source-owned staging producer resolves the selected declaration
  Then only that declaration's entry names and kind discriminants are offered
  And the selected entry's exact declared value domains remain distinct
 Scenario: Actual referenced enumeration choices preserve complete typed values
  Given an IfcPropertyEnumeratedValue referencing an IfcPropertyEnumeration
  When contextual choices are queried at the complete list item path
  Then all actual enumeration values are returned in source order with exact tags and scalar words
  And missing references produce an explicit unavailable diagnostic
 Scenario: Identity requirements come from the existing planner
  Given a structurally prepared selected application and the original source revision
  Then identity fields use the planner's exact keys and explicit authored ids and GlobalIds
  And no field event grants semantic publication
