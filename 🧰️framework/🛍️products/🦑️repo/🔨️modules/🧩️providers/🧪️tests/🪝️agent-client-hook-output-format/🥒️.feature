@capability-repo.providers.agent-client-hook-output
@no-oracle-repo-agent-client-hook-output
@comparison-ordered-json-v1
Feature: All eight agent-client providers agree on how a hook result reaches the agent client
  Seven of the eight agent clients receive the marshalled hook result verbatim; VS Code and Copilot Chat
  receive it wrapped in a hookSpecificOutput member that carries a permission decision for PreToolUse
  and additional context for everything else. Nothing outside this repository produces that record,
  so there is no reference to compare against — see the recorded no-oracle decision
  `repo-agent-client-hook-output`. What stands in for it is two independently written implementations, Rust
  and Go, formatting the same frozen fixture events for the same eight agent clients. The formatted string
  is parsed before it is projected, so only the record's content is compared and never the incidental
  key order of one language's JSON writer.

  @id-every-agent-client-formats-the-same-result
  @level-fundamental
  @mode-differential
  Scenario: Every agent client formats every fixture result into the same record
    Given the hook output vectors shared://🪝️agent-client-hook-output-format/🪝️hook-outputs.json
    When each of the eight agent-client providers formats every fixture result
    Then every implementation projects the same parsed record for every agent client and every result

  @id-native-event-names-are-derived-the-same-way
  @level-fundamental
  @mode-differential
  Scenario: Every agent client derives the same native event name for every neutral hook event
    Given the hook output vectors shared://🪝️agent-client-hook-output-format/🪝️hook-outputs.json
    When each agent client is asked for the native event name of every hook event, with and without a subagent parent
    Then every implementation projects the same name, including the empty name for an event the agent client does not surface

  @id-an-unknown-native-event-is-rejected
  @level-fundamental
  @mode-error
  Scenario: A native event an agent client does not know is an error, never a silently defaulted hook event
    Given the hook output vectors shared://🪝️agent-client-hook-output-format/🪝️hook-outputs.json
    When each agent client resolves every fixture native event against its declared tool kind
    Then every implementation projects the same resolved hook event, or the same refusal
