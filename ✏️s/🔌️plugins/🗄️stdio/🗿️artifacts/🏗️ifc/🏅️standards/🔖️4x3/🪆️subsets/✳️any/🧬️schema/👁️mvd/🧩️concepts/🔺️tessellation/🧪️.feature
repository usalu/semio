Feature: Explicit released Body Tessellation predicates are distinct from full-view conformance
 Scenario: Applicable representations obey the three authenticated concept clauses
  Given an explicit applicability decision for each exact representation ID
  When the released identifier, representation type and item type predicates are checked
  Then Body and Tessellation with triangulated or polygonal face sets pass this concept
  And a known counterexample fails this concept with its actual attribute pointer
  And schema and full-view conformance remain not evaluated
 Scenario: Unknown source facts never become a pass
  When applicability, a target, a constituent attribute or an item reference is unresolved
  Then the concept result is incomplete
  And a caller-excluded unrelated body remains not applicable
 Scenario: Capture and semantic work preserve original controls
  When the caller replaces controls or mutates source after private capture
  Then only the captured graph and original cancellation are used
  And cancellation at interior or completion work cannot publish a partial report
