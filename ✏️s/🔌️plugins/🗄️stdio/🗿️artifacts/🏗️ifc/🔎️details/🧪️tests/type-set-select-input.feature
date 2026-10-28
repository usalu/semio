Feature: Caller-authored SET SELECT definitions use exact aggregate or entity carriers
 Scenario: Edit every actual property definition alternative
  Given the complete released PropertySetDefinitionSelect and original source constituents
  When the caller supplies an ordinary entity reference or a typed SET of references
  Then each canonical ID resolves uniquely to an actual property-definition subtype
  And the SET contains at least one distinct reference identity
  And central mutation and inverse preserve exact transport order and original source

 Scenario: Build an incomplete SET through explicit controls
  Given no selected member or an empty caller-selected SET draft
  When the caller adds, edits or removes a reference
  Then no scalar ID or minimum cardinality is silently filled
  And blank, partial and UTF8 draft text stays editable
  And duplicate, missing and wrong-subtype references never become ready

 Scenario: Page every member without truncation
  Given the explicit eighteen-member valid SET
  When the caller visits the original source-bound member windows
  Then all eighteen exact members remain in the command
  And changing pages edits no command data
  And whole-candidate validity includes every off-page member
  And no form content or choice cap is raised

 Scenario: Deny or cancel original retained work
  Given donated requests, partial receiver owners and the original paid source loan
  When any demanded axis is denied or original cancellation occurs
  Then demand stays pure and no denied progress is posted
  And each partial owner retires before the exact original source
