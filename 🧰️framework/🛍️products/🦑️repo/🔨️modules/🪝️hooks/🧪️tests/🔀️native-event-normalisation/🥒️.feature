@capability-repo.hooks.native-event-normalisation
@no-oracle-repo-hooks-native-normalisation
@comparison-ordered-json-v1
Feature: A native IDE hook payload becomes one neutral hook event and one set of neutral facts
  Eight IDEs send eight different shapes down the same pipe. Copilot Chat sends `PreToolUse` with a
  flat `tool_input`; Cursor sends `beforeSubmitPrompt` with a `conversation_id`; Windsurf sends
  `pre_run_command` with a `trajectory_id`; Kiro sends `agentSpawn` with an `agent_id`; Claude Code,
  Droid, Codex and Antigravity share one table but disagree on capitalisation. Normalisation turns
  all of it into one neutral event slug plus one set of facts — session, transcript, tool, command,
  working directory, parent, model, effort — and refuses anything it does not recognise.

  Nothing outside this repository produces or consumes that mapping, so there is no reference to
  compare against; the recorded decision is `repo-hooks-native-normalisation`. What stands in for an
  oracle is the fixture itself: `shared://🔀️native-event-normalisation/🔀️native-events.json` pins the expected neutral event
  beside every native one, written from the hook configurations this repository actually commits
  (`.claude/settings.json`, `.cursor/hooks.json`, `.windsurf/hooks.json`, `.kiro/agents/repo.json`),
  so an implementation cannot agree with itself — it has to agree with the specification.

  @id-every-native-event-resolves-to-its-pinned-neutral-event
  @level-fundamental
  @mode-conformance
  Scenario: Every native event resolves to the neutral event the specification pins beside it
    Given the native event vectors shared://🔀️native-event-normalisation/🔀️native-events.json
    When each vector is normalised against its own client, tool name and payload
    Then every implementation resolves the pinned event and the pinned parent hint for every vector

  @id-the-command-outranks-the-tool-name-when-it-is-more-specific
  @level-fundamental
  @mode-conformance
  Scenario: A shell command decides the tool kind when the tool name is generic or terminal
    Given the native event vectors shared://🔀️native-event-normalisation/🔀️native-events.json
    When each vector's tool name and command are classified separately and then together
    Then every implementation reports the same tool kind for the name, for the command and for the pair

  @id-an-unrecognised-native-event-is-refused
  @level-fundamental
  @mode-error
  Scenario: A native event a client does not know is refused, never defaulted
    Given the native event vectors shared://🔀️native-event-normalisation/🔀️native-events.json
    When each refusal vector is normalised against the client that does not know it
    Then every implementation refuses it and names the client in the refusal

  @id-neutral-facts-are-read-out-of-every-payload-shape
  @level-fundamental
  @mode-conformance
  Scenario: The same fact is found whether it sits flat, under event, or under native.event
    Given the native event vectors shared://🔀️native-event-normalisation/🔀️native-events.json
    When the neutral facts are read out of every payload shape
    Then every implementation reports the same session, transcript, tool, command, working directory, parent, model and effort
