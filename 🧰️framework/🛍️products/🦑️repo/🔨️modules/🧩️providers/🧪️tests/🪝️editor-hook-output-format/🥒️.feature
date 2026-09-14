@capability-repo.providers.editor-hook-output
@no-oracle-repo-editor-hook-output
@comparison-ordered-json-v1
Feature: All eight editor providers agree on how a hook result reaches the editor
  Seven of the eight editors receive the marshalled hook result verbatim; VS Code and Copilot Chat
  receive it wrapped in a hookSpecificOutput member that carries a permission decision for PreToolUse
  and additional context for everything else. Nothing outside this repository produces that record,
  so there is no reference to compare against — see the recorded no-oracle decision
  `repo-editor-hook-output`. What stands in for it is two independently written implementations, Rust
  and Go, formatting the same frozen fixture events for the same eight editors. The formatted string
  is parsed before it is projected, so only the record's content is compared and never the incidental
  key order of one language's JSON writer.

  @id-every-editor-formats-the-same-result
  @level-fundamental
  @mode-differential
  Scenario: Every editor formats every fixture result into the same record
    Given the hook output vectors local://🪝️hook-outputs.json
    When each of the eight editor providers formats every fixture result
    Then every implementation projects the same parsed record for every editor and every result

  @id-native-event-names-are-derived-the-same-way
  @level-fundamental
  @mode-differential
  Scenario: Every editor derives the same native event name for every neutral hook event
    Given the hook output vectors local://🪝️hook-outputs.json
    When each editor is asked for the native event name of every hook event, with and without a subagent parent
    Then every implementation projects the same name, including the empty name for an event the editor does not surface

  @id-an-unknown-native-event-is-rejected
  @level-fundamental
  @mode-error
  Scenario: A native event an editor does not know is an error, never a silently defaulted hook event
    Given the hook output vectors local://🪝️hook-outputs.json
    When each editor resolves every fixture native event against its declared tool kind
    Then every implementation projects the same resolved hook event, or the same refusal
