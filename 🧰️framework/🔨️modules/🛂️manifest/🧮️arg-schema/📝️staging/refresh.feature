Feature: A consumed staged form surface has an explicit real window context
 Scenario: Two forms preserve their own context while another window has focus
  Given two real windows with separate utilities and one expanded staged action each
  When the auxiliary form targets are admitted
  Then each consumed form has a distinct surface and its own real window context
  And the window roster and viewport identities remain unchanged
  And staged requests are separate from ordinary view metadata
 Scenario: A form cannot acquire an undeclared action or surface
  When its action, body, locale, real window, exact generation or surface is invalid
  Then the whole target batch is refused before mounting any form
  And duplicate owners and occupied surface keys are refused
