@capability-mcp-capability-listing
@oracle-modelcontextprotocol-sdk
@comparison-ordered-json-v1
Feature: Every profile advertises exactly one surface
  The repo MCP server ships nine tools, eight resources and four prompts. The names, their order and
  their per-IDE descriptions come from one authored table, so every implementation and the reference
  SDK server built from that same table must list exactly the same surface.

  @id-generic-profile-surface
  @level-fundamental
  @mode-differential
  Scenario: The generic profile lists the whole surface in sorted order
    Given the authored surface table shared://📋️surface.json and the authored descriptions asset://🧬️schema/🔣️descriptions.json
    And a server started with no client profile
    When the client lists tools, resources and prompts
    Then every implementation projects the same names in the same order with the same descriptions

  @id-ide-profile-surface
  @level-quick
  @mode-differential
  Scenario: An IDE profile adds only its own plan or spec argument
    Given the authored surface table shared://📋️surface.json and the authored descriptions asset://🧬️schema/🔣️descriptions.json
    And a server started for each of the cursor, kiro, copilot, claude and codex profiles
    When the client lists tools
    Then only the kiro profile carries `spec_id`, only the plan profiles carry `plan_id`, and the generic profile carries neither
