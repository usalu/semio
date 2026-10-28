Feature: Complete retained IFC4X3_ADD2 EXPRESS evaluation
  Scenario: Compile every retained declaration
    Given the exact official IFC4X3_ADD2 catalog
    When semantic compilation finishes
    Then all 777 WHERE expressions and 60 derived expressions have canonical ASTs
    And all 48 functions and two global rules have executable statement ASTs
    And inverse, UNIQUE and contextual aggregate bounds are indexed

  Scenario Outline: Preserve EXPRESS scalar and aggregate semantics
    Given an expression in cases.json
    When semantic evaluation finishes
    Then its exact tagged result matches the language-neutral expected value

  Scenario: Independently validate informative models and authored violations
    Given the official triangulated, alignment and structural fixtures
    When own semantic evaluation and IfcOpenShell express_rules=True run
    Then named rule failures agree
    And every unsupported construct or exhausted budget yields incomplete status

  Scenario: Cancellation prevents a false completed result
    Given a caller cancellation signal or an exhausted compile/evaluation budget
    When a bounded work checkpoint runs
    Then evaluation refuses before publishing a conformance result

 Scenario: Qualified schema enumeration literals enforce USERDEFINED companion attributes
  Given each concrete IFC entity with a USERDEFINED PredefinedType member
  When PredefinedType is USERDEFINED and its companion label is indeterminate
  Then each applicable CorrectPredefinedType rule is FALSE
  And the independent IfcOpenShell validator identifies the same 291 failures

 Scenario: Every retained clause and function has an executable indeterminate path
  Given all 876 entity declaration environments and all named type domains
  When each of 777 WHERE clauses and 48 functions receives indeterminate inputs
  Then execution emits no unsupported-construct or unbound-identifier failure
  And indeterminate path coverage does not imply every branch has been proved

 Scenario: Rational spline arrays preserve dynamic bounds in both dimensions
  Given curve and surface control points and weights with zero-based ARRAY domains
  When all six retained contextual bound expressions execute
  Then their lower and upper indices match their declared bounds

 Scenario: BAG subset consumes matching occurrences
  Given BAG [1,1] and BAG [1,2]
  When subset or its reversed superset is evaluated
  Then the result is FALSE
  And an independent multiset library computes the same result

 Scenario: Constructor TYPEOF has the same ancestry as snapshot values
  Given an IfcCartesianPoint constructor and an IfcPositiveLengthMeasure constructor
  When TYPEOF is evaluated
  Then all entity supertypes and defined type aliases are included
  And IfcOpenShell independently returns those IFC type names

 Scenario: Partial aggregate membership remains explicit
  Given a partial aggregate with indeterminate member comparisons
  When a subset relation cannot be proved or refuted
  Then the LOGICAL result is UNKNOWN
  When union difference or intersection cannot determine its output membership
  Then evaluation refuses with aggregate-indeterminate rather than asserting absence

 Scenario: Controlled structural admission can be cancelled during its real work
  Given a large typed aggregate awaiting structural domain validation
  When cancellation arrives during type traversal
  Then validation stops before completing that traversal
  And no synchronous catalog validation pass occurs behind the progress callback

 Scenario: Structural instance uniqueness includes implicit SET domains
  Given repeated references to one entity in a SET
  Then structural admission reports aggregate-unique
  Given distinct entities with equal values in a SET
  Then structural instance uniqueness is satisfied

 Scenario: Complex derived redeclarations retain their input markers
  Given a complex IFC unit with Dimensions effectively redeclared as derived
  When the base constituent supplies a literal instead of a derived marker
  Then semantic normalization refuses that input before erasing the literal
