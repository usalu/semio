@capability-repo.hooks.session-logging
@oracle-hooks-session-log-typescript
@comparison-ordered-json-v1
Feature: An agent session is recorded exactly as far as the repository configuration allows
  `.🧬semio/🦑️repo/📋️config.toml` decides how much of an agent session survives. `[logging] session`
  is off by default and nothing at all is written then — not one file, not one entry. With it on,
  `detail` decides the depth: `minimal` keeps only the neutral event, `standard` adds what the hook
  answered, `full` adds the IDE's own payload verbatim. `[logging] plan` decides whether a finished
  plan update folds into the session's recorded plan. Version hooks are never recorded at all: they
  belong to the version control lifecycle, not to an agent session.

  A session that never identifies itself is still recorded — under its Kiro parent process id for the
  Kiro CLI and under the literal `unknown` otherwise — because a hook that silently dropped its record
  would be indistinguishable from one that never ran.

  The oracle is a second, independently written implementation of the config parse and the record in
  TypeScript (`hooks-session-log-typescript`). The store itself is a port, so both implementations
  record into memory and nothing here touches a disk or a clock.

  @id-the-configuration-decides-whether-anything-is-recorded
  @level-fundamental
  @mode-differential
  Scenario: Every configuration is parsed and obeyed the same way
    Given the session vectors shared://📓️session-logging/📓️sessions.json
    When each configuration document is parsed and each session replayed under it
    Then every implementation projects the same switches and the same number of recorded entries

  @id-detail-decides-how-much-of-each-entry-survives
  @level-fundamental
  @mode-differential
  Scenario: minimal keeps the event, standard adds the response, full adds the native payload
    Given the session vectors shared://📓️session-logging/📓️sessions.json
    When one refused invocation is recorded at each detail level
    Then every implementation projects the same members for every level

  @id-a-session-is-recorded-under-a-resolvable-identity
  @level-fundamental
  @mode-differential
  Scenario: A session that does not identify itself is still recorded, under a resolvable identity
    Given the session vectors shared://📓️session-logging/📓️sessions.json
    When each session is replayed with session logging on
    Then every implementation records it under the same identity and the same uri

  @id-a-version-hook-is-never-recorded
  @level-fundamental
  @mode-conformance
  Scenario: No version hook ever reaches the session log
    Given the session vectors shared://📓️session-logging/📓️sessions.json
    When the version hook session is replayed with session logging on at full detail
    Then every implementation records nothing at all

  @id-the-recorded-plan-follows-the-plan-switch
  @level-fundamental
  @mode-differential
  Scenario: The recorded plan is folded only while the plan switch is on
    Given the session vectors shared://📓️session-logging/📓️sessions.json
    When the planning session is replayed with the plan switch on and then off
    Then every implementation projects the folded plan in the first case and no plan in the second
