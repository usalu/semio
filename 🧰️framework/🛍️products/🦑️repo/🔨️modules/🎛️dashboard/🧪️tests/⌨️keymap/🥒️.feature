Feature: The keymap is data
  The prefix key and every binding are data with action ids, in three scopes: the prefix scope after
  the prefix key, the window scope and the view scope. Customizations are preference events that fold
  over the shipped defaults binding by binding. The native keymap and independent readers resolve the
  same shared vectors to the same actions.

  Scenario: Key spellings fold into one canonical key
    Given the shared spellings of keys and the texts that name no key
    When the native parser and an independent spelling reader fold them
    Then both name every key by the same canonical spelling and label
    And both refuse the same texts

  Scenario: The keymap is data with action ids
    Given the shipped keymap with a prefix key and bindings in the prefix, window and view scopes
    When Ajv validates it against the keymap schema
    Then every binding names a scope, an action id and at least one key
    And no scope binds a key twice and no direct scope binds a key that only types

  Scenario: Customizations fold over the defaults without losing a key
    Given the shared keymap vectors with swapped keys, collisions, unknown bindings and a shadowing prefix
    When the native keymap and an independent builder fold the same customizations
    Then both resolve every probed key to the same action
    And a customization that collides or cannot be bound is ignored and reported, never dropped silently

  Scenario: Every action stays reachable by keyboard under any customization
    Given a keymap whose prefix and several bindings were customized
    When the keys of every action are listed
    Then every action of every scope still has at least one key
    And the label of an action names the prefix key first when the action is in the prefix scope

  Scenario: Terminal bytes become the canonical key spellings of the keymap
    Given the shared terminal byte vectors for arrows, Shift Tab, Alt letters, Ctrl letters, Page keys and function keys
    When the native parser and Node readline decode them independently
    Then both name every key by the same canonical spelling such as ctrl+w, backtab, alt+b or pageup
