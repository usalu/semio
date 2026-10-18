// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗣️mcp_description carries the MCP tool and prompt descriptions of the cli.

// #endregion 🧲️Header

package cli

import (
	providers "github.com/usalu/semio/repo/providers"
)

// #region 🔌️Mcp Surface

// #region 🗣️McpDescriptions

func mcpDesc(kind providers.McpClientKind, key string) string {
	if kind == "" {
		kind = providers.McpClientGeneric
	}
	if m, ok := mcpDescriptionTable[key]; ok {
		if s, ok2 := m[kind]; ok2 && s != "" {
			return s
		}
		if s, ok3 := m[providers.McpClientGeneric]; ok3 {
			return s
		}
	}
	return ""
}

// mcpDescriptionTable: keys = stable ids; inner map must include generic + each IDE kind used.
var mcpDescriptionTable = map[string]map[providers.McpClientKind]string{
	"prompt_enhance": {
		providers.McpClientGeneric: "Use when you are about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientCursor:  "Use when the Cursor agent is about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientKiro:    "Use when the Kiro agent is about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientCopilot: "Use when the Copilot agent is about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientClaude:  "Use when Claude Code is about to add behavior and must align the change with repository rules before editing.",
		providers.McpClientCodex:   "Use when Codex is about to add behavior and must align the change with repository rules before editing.",
	},
	"prompt_refactor": {
		providers.McpClientGeneric: "Use when a refactor is required and you must not regress behavior or tests.",
		providers.McpClientCursor:  "Use when the Cursor agent must refactor without regressing behavior or tests.",
		providers.McpClientKiro:    "Use when the Kiro agent must refactor without regressing behavior or tests.",
		providers.McpClientCopilot: "Use when the Copilot agent must refactor without regressing behavior or tests.",
		providers.McpClientClaude:  "Use when Claude Code must refactor without regressing behavior or tests.",
		providers.McpClientCodex:   "Use when Codex must refactor without regressing behavior or tests.",
	},
	"prompt_test": {
		providers.McpClientGeneric: "Use when tests must be extended before you claim coverage for new paths.",
		providers.McpClientCursor:  "Use when the Cursor agent must extend tests before claiming coverage for new paths.",
		providers.McpClientKiro:    "Use when the Kiro agent must extend tests before claiming coverage for new paths.",
		providers.McpClientCopilot: "Use when the Copilot agent must extend tests before claiming coverage for new paths.",
		providers.McpClientClaude:  "Use when Claude Code must extend tests before claiming coverage for new paths.",
		providers.McpClientCodex:   "Use when Codex must extend tests before claiming coverage for new paths.",
	},
	"prompt_comply": {
		providers.McpClientGeneric: "Use when the tree is red and you must converge implementation to passing tests without deleting assertions.",
		providers.McpClientCursor:  "Use when the Cursor agent must converge a red workspace to passing tests without deleting assertions.",
		providers.McpClientKiro:    "Use when the Kiro agent must converge a red workspace to passing tests without deleting assertions.",
		providers.McpClientCopilot: "Use when the Copilot agent must converge a red workspace to passing tests without deleting assertions.",
		providers.McpClientClaude:  "Use when Claude Code must converge a red workspace to passing tests without deleting assertions.",
		providers.McpClientCodex:   "Use when Codex must converge a red workspace to passing tests without deleting assertions.",
	},
	"res_root": {
		providers.McpClientGeneric: "Fetch when the session anchor for the repository root is required before any scoped read.",
		providers.McpClientCursor:  "Fetch when the Cursor session anchor for the repository root is required before any scoped read.",
		providers.McpClientKiro:    "Fetch when the Kiro session anchor for the repository root is required before any scoped read.",
		providers.McpClientCopilot: "Fetch when the Copilot session anchor for the repository root is required before any scoped read.",
		providers.McpClientClaude:  "Fetch when Claude Code needs the repository root anchor before any scoped read.",
		providers.McpClientCodex:   "Fetch when Codex needs the repository root anchor before any scoped read.",
	},
	"res_bundles": {
		providers.McpClientGeneric: "Fetch when bundle inventory is required before choosing a technology subtree.",
		providers.McpClientCursor:  "Fetch when bundle inventory is required before the Cursor agent chooses a technology subtree.",
		providers.McpClientKiro:    "Fetch when bundle inventory is required before the Kiro agent chooses a technology subtree.",
		providers.McpClientCopilot: "Fetch when bundle inventory is required before the Copilot agent chooses a technology subtree.",
		providers.McpClientClaude:  "Fetch when bundle inventory is required before Claude Code chooses a technology subtree.",
		providers.McpClientCodex:   "Fetch when bundle inventory is required before Codex chooses a technology subtree.",
	},
	"res_bundle_one": {
		providers.McpClientGeneric: "Fetch when a single bundle context gate is required before editing that bundle.",
		providers.McpClientCursor:  "Fetch when the Cursor agent needs one bundle context gate before editing that bundle.",
		providers.McpClientKiro:    "Fetch when the Kiro agent needs one bundle context gate before editing that bundle.",
		providers.McpClientCopilot: "Fetch when the Copilot agent needs one bundle context gate before editing that bundle.",
		providers.McpClientClaude:  "Fetch when Claude Code needs one bundle context gate before editing that bundle.",
		providers.McpClientCodex:   "Fetch when Codex needs one bundle context gate before editing that bundle.",
	},
	"res_folders": {
		providers.McpClientGeneric: "Fetch when folder enumeration is required before path-sensitive edits.",
		providers.McpClientCursor:  "Fetch when folder enumeration is required before the Cursor agent performs path-sensitive edits.",
		providers.McpClientKiro:    "Fetch when folder enumeration is required before the Kiro agent performs path-sensitive edits.",
		providers.McpClientCopilot: "Fetch when folder enumeration is required before the Copilot agent performs path-sensitive edits.",
		providers.McpClientClaude:  "Fetch when folder enumeration is required before Claude Code performs path-sensitive edits.",
		providers.McpClientCodex:   "Fetch when folder enumeration is required before Codex performs path-sensitive edits.",
	},
	"res_folder_one": {
		providers.McpClientGeneric: "Fetch when one folder subtree must be confirmed before moves or renames under it.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must confirm one folder subtree before moves or renames under it.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must confirm one folder subtree before moves or renames under it.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must confirm one folder subtree before moves or renames under it.",
		providers.McpClientClaude:  "Fetch when Claude Code must confirm one folder subtree before moves or renames under it.",
		providers.McpClientCodex:   "Fetch when Codex must confirm one folder subtree before moves or renames under it.",
	},
	"res_files": {
		providers.McpClientGeneric: "Fetch when file listing is required before batch reads across a scope.",
		providers.McpClientCursor:  "Fetch when file listing is required before the Cursor agent batch-reads a scope.",
		providers.McpClientKiro:    "Fetch when file listing is required before the Kiro agent batch-reads a scope.",
		providers.McpClientCopilot: "Fetch when file listing is required before the Copilot agent batch-reads a scope.",
		providers.McpClientClaude:  "Fetch when file listing is required before Claude Code batch-reads a scope.",
		providers.McpClientCodex:   "Fetch when file listing is required before Codex batch-reads a scope.",
	},
	"res_file_one": {
		providers.McpClientGeneric: "Fetch when a single file identity must be verified before a targeted edit.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must verify a single file identity before a targeted edit.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must verify a single file identity before a targeted edit.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must verify a single file identity before a targeted edit.",
		providers.McpClientClaude:  "Fetch when Claude Code must verify a single file identity before a targeted edit.",
		providers.McpClientCodex:   "Fetch when Codex must verify a single file identity before a targeted edit.",
	},
	"res_sections": {
		providers.McpClientGeneric: "Fetch when section discovery is required before region-scoped refactors.",
		providers.McpClientCursor:  "Fetch when section discovery is required before the Cursor agent runs region-scoped refactors.",
		providers.McpClientKiro:    "Fetch when section discovery is required before the Kiro agent runs region-scoped refactors.",
		providers.McpClientCopilot: "Fetch when section discovery is required before the Copilot agent runs region-scoped refactors.",
		providers.McpClientClaude:  "Fetch when section discovery is required before Claude Code runs region-scoped refactors.",
		providers.McpClientCodex:   "Fetch when section discovery is required before Codex runs region-scoped refactors.",
	},
	"res_section_one": {
		providers.McpClientGeneric: "Fetch when one section boundary must be loaded before a surgical edit inside that section.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must load one section boundary before a surgical edit inside that section.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must load one section boundary before a surgical edit inside that section.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must load one section boundary before a surgical edit inside that section.",
		providers.McpClientClaude:  "Fetch when Claude Code must load one section boundary before a surgical edit inside that section.",
		providers.McpClientCodex:   "Fetch when Codex must load one section boundary before a surgical edit inside that section.",
	},
	"res_definitions": {
		providers.McpClientGeneric: "Fetch when definition discovery is required before symbol-level work.",
		providers.McpClientCursor:  "Fetch when definition discovery is required before the Cursor agent performs symbol-level work.",
		providers.McpClientKiro:    "Fetch when definition discovery is required before the Kiro agent performs symbol-level work.",
		providers.McpClientCopilot: "Fetch when definition discovery is required before the Copilot agent performs symbol-level work.",
		providers.McpClientClaude:  "Fetch when definition discovery is required before Claude Code performs symbol-level work.",
		providers.McpClientCodex:   "Fetch when definition discovery is required before Codex performs symbol-level work.",
	},
	"res_definition_one": {
		providers.McpClientGeneric: "Fetch when one definition record must be confirmed before renaming or extracting it.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must confirm one definition record before renaming or extracting it.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must confirm one definition record before renaming or extracting it.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must confirm one definition record before renaming or extracting it.",
		providers.McpClientClaude:  "Fetch when Claude Code must confirm one definition record before renaming or extracting it.",
		providers.McpClientCodex:   "Fetch when Codex must confirm one definition record before renaming or extracting it.",
	},
	"res_tickets": {
		providers.McpClientGeneric: "Fetch when ticket inventory is required before choosing where to attach work.",
		providers.McpClientCursor:  "Fetch when ticket inventory is required before the Cursor agent chooses where to attach work.",
		providers.McpClientKiro:    "Fetch when ticket inventory is required before the Kiro agent chooses where to attach work.",
		providers.McpClientCopilot: "Fetch when ticket inventory is required before the Copilot agent chooses where to attach work.",
		providers.McpClientClaude:  "Fetch when ticket inventory is required before Claude Code chooses where to attach work.",
		providers.McpClientCodex:   "Fetch when ticket inventory is required before Codex chooses where to attach work.",
	},
	"res_ticket_one": {
		providers.McpClientGeneric: "Fetch when one ticket record must be read before closing or reopening that ticket.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must read one ticket record before closing or reopening that ticket.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must read one ticket record before closing or reopening that ticket.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must read one ticket record before closing or reopening that ticket.",
		providers.McpClientClaude:  "Fetch when Claude Code must read one ticket record before closing or reopening that ticket.",
		providers.McpClientCodex:   "Fetch when Codex must read one ticket record before closing or reopening that ticket.",
	},
	"res_goals": {
		providers.McpClientGeneric: "Fetch when goal document is required before opening or linking a ticket.",
		providers.McpClientCursor:  "Fetch when goal document is required before the Cursor agent opens or links a ticket.",
		providers.McpClientKiro:    "Fetch when goal document is required before the Kiro agent opens or links a ticket.",
		providers.McpClientCopilot: "Fetch when goal document is required before the Copilot agent opens or links a ticket.",
		providers.McpClientClaude:  "Fetch when goal document is required before Claude Code opens or links a ticket.",
		providers.McpClientCodex:   "Fetch when goal document is required before Codex opens or links a ticket.",
	},
	"res_goal_one": {
		providers.McpClientGeneric: "Fetch when one goal record must be confirmed before milestone or ticket association.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must confirm one goal record before milestone or ticket association.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must confirm one goal record before milestone or ticket association.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must confirm one goal record before milestone or ticket association.",
		providers.McpClientClaude:  "Fetch when Claude Code must confirm one goal record before milestone or ticket association.",
		providers.McpClientCodex:   "Fetch when Codex must confirm one goal record before milestone or ticket association.",
	},
	"res_policies": {
		providers.McpClientGeneric: "Fetch when policy inventory is required before an audit or autofix pass.",
		providers.McpClientCursor:  "Fetch when policy inventory is required before the Cursor agent starts an audit or autofix pass.",
		providers.McpClientKiro:    "Fetch when policy inventory is required before the Kiro agent starts an audit or autofix pass.",
		providers.McpClientCopilot: "Fetch when policy inventory is required before the Copilot agent starts an audit or autofix pass.",
		providers.McpClientClaude:  "Fetch when policy inventory is required before Claude Code starts an audit or autofix pass.",
		providers.McpClientCodex:   "Fetch when policy inventory is required before Codex starts an audit or autofix pass.",
	},
	"res_policy_one": {
		providers.McpClientGeneric: "Fetch when one policy scope must be verified before interpreting breaches.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must verify one policy scope before interpreting breaches.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must verify one policy scope before interpreting breaches.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must verify one policy scope before interpreting breaches.",
		providers.McpClientClaude:  "Fetch when Claude Code must verify one policy scope before interpreting breaches.",
		providers.McpClientCodex:   "Fetch when Codex must verify one policy scope before interpreting breaches.",
	},
	"res_statutes": {
		providers.McpClientGeneric: "Fetch when breach-kind listing is required before triage ordering.",
		providers.McpClientCursor:  "Fetch when breach-kind listing is required before the Cursor agent orders triage.",
		providers.McpClientKiro:    "Fetch when breach-kind listing is required before the Kiro agent orders triage.",
		providers.McpClientCopilot: "Fetch when breach-kind listing is required before the Copilot agent orders triage.",
		providers.McpClientClaude:  "Fetch when breach-kind listing is required before Claude Code orders triage.",
		providers.McpClientCodex:   "Fetch when breach-kind listing is required before Codex orders triage.",
	},
	"res_statute_one": {
		providers.McpClientGeneric: "Fetch when one breach-kind record must be read before applying a fix strategy.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must read one breach-kind record before applying a fix strategy.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must read one breach-kind record before applying a fix strategy.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must read one breach-kind record before applying a fix strategy.",
		providers.McpClientClaude:  "Fetch when Claude Code must read one breach-kind record before applying a fix strategy.",
		providers.McpClientCodex:   "Fetch when Codex must read one breach-kind record before applying a fix strategy.",
	},
	"res_contributors": {
		providers.McpClientGeneric: "Fetch when contributor listing is required before attributing sessions or checkpoints.",
		providers.McpClientCursor:  "Fetch when contributor listing is required before the Cursor agent attributes sessions or checkpoints.",
		providers.McpClientKiro:    "Fetch when contributor listing is required before the Kiro agent attributes sessions or checkpoints.",
		providers.McpClientCopilot: "Fetch when contributor listing is required before the Copilot agent attributes sessions or checkpoints.",
		providers.McpClientClaude:  "Fetch when contributor listing is required before Claude Code attributes sessions or checkpoints.",
		providers.McpClientCodex:   "Fetch when contributor listing is required before Codex attributes sessions or checkpoints.",
	},
	"res_contributor_one": {
		providers.McpClientGeneric: "Fetch when one contributor record must be read before ownership-sensitive edits.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must read one contributor record before ownership-sensitive edits.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must read one contributor record before ownership-sensitive edits.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must read one contributor record before ownership-sensitive edits.",
		providers.McpClientClaude:  "Fetch when Claude Code must read one contributor record before ownership-sensitive edits.",
		providers.McpClientCodex:   "Fetch when Codex must read one contributor record before ownership-sensitive edits.",
	},
	"res_checkpoints": {
		providers.McpClientGeneric: "Fetch when checkpoint inventory is required before comparing versions.",
		providers.McpClientCursor:  "Fetch when checkpoint inventory is required before the Cursor agent compares versions.",
		providers.McpClientKiro:    "Fetch when checkpoint inventory is required before the Kiro agent compares versions.",
		providers.McpClientCopilot: "Fetch when checkpoint inventory is required before the Copilot agent compares versions.",
		providers.McpClientClaude:  "Fetch when checkpoint inventory is required before Claude Code compares versions.",
		providers.McpClientCodex:   "Fetch when checkpoint inventory is required before Codex compares versions.",
	},
	"res_checkpoint_one": {
		providers.McpClientGeneric: "Fetch when one checkpoint record must be read before tying hooks or tickets to history.",
		providers.McpClientCursor:  "Fetch when the Cursor agent must read one checkpoint record before tying hooks or tickets to history.",
		providers.McpClientKiro:    "Fetch when the Kiro agent must read one checkpoint record before tying hooks or tickets to history.",
		providers.McpClientCopilot: "Fetch when the Copilot agent must read one checkpoint record before tying hooks or tickets to history.",
		providers.McpClientClaude:  "Fetch when Claude Code must read one checkpoint record before tying hooks or tickets to history.",
		providers.McpClientCodex:   "Fetch when Codex must read one checkpoint record before tying hooks or tickets to history.",
	},
	"tool_ticket_open": {
		providers.McpClientGeneric: "Use at the start of any tracked task that will touch the repository and needs a durable workspace folder.",
		providers.McpClientCursor:  "Use at the start of a Cursor agent task that will touch the repository and needs a durable workspace folder; use when a plan id should be bound for later archival on close.",
		providers.McpClientKiro:    "Use at the start of a Kiro agent task that will touch the repository and needs a durable workspace folder; use when a spec id should be bound for later archival on close.",
		providers.McpClientCopilot: "Use at the start of a Copilot agent task that will touch the repository and needs a durable workspace folder; use when a memory plan id should be bound for later archival on close.",
		providers.McpClientClaude:  "Use at the start of a Claude Code task that will touch the repository and needs a durable workspace folder; use when a plan id should be bound for later archival on close.",
		providers.McpClientCodex:   "Use at the start of a Codex task that will touch the repository and needs a durable workspace folder; use when a memory id should be bound for later archival on close.",
	},
	"tool_ticket_close": {
		providers.McpClientGeneric: "Use only when the tracked task is finished, the summary is final, and the touched paths list is complete. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientCursor:  "Use only when the Cursor agent task is finished, the summary is final, the touched paths list is complete, and any bound plan should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientKiro:    "Use only when the Kiro agent task is finished, the summary is final, the touched paths list is complete, and any bound spec should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientCopilot: "Use only when the Copilot agent task is finished, the summary is final, the touched paths list is complete, and any bound memory plan should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientClaude:  "Use only when the Claude Code task is finished, the summary is final, the touched paths list is complete, and any bound plan should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		providers.McpClientCodex:   "Use only when the Codex task is finished, the summary is final, the touched paths list is complete, and any bound memory should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
	},
	"tool_ticket_reopen": {
		providers.McpClientGeneric: "Use when work must continue on a closed ticket before any new edits land.",
		providers.McpClientCursor:  "Use when the Cursor agent must continue a closed ticket before new edits; use when rebinding a plan id for archival on the next close.",
		providers.McpClientKiro:    "Use when the Kiro agent must continue a closed ticket before new edits; use when rebinding a spec id for archival on the next close.",
		providers.McpClientCopilot: "Use when the Copilot agent must continue a closed ticket before new edits; use when rebinding a memory plan id for archival on the next close.",
		providers.McpClientClaude:  "Use when Claude Code must continue a closed ticket before new edits; use when rebinding a plan id for archival on the next close.",
		providers.McpClientCodex:   "Use when Codex must continue a closed ticket before new edits; use when rebinding a memory id for archival on the next close.",
	},
	"tool_goal_open": {
		providers.McpClientGeneric: "Call when a new goal must be recorded before any ticket can reference it.",
		providers.McpClientCursor:  "Call when the Cursor agent must record a new goal before any ticket references it.",
		providers.McpClientKiro:    "Call when the Kiro agent must record a new goal before any ticket references it.",
		providers.McpClientCopilot: "Call when the Copilot agent must record a new goal before any ticket references it.",
		providers.McpClientClaude:  "Call when Claude Code must record a new goal before any ticket references it.",
		providers.McpClientCodex:   "Call when Codex must record a new goal before any ticket references it.",
	},
	"tool_goal_close": {
		providers.McpClientGeneric: "Call when a goal is fulfilled and must be closed with an evidence summary.",
		providers.McpClientCursor:  "Call when the Cursor agent has fulfilled a goal and must close it with an evidence summary.",
		providers.McpClientKiro:    "Call when the Kiro agent has fulfilled a goal and must close it with an evidence summary.",
		providers.McpClientCopilot: "Call when the Copilot agent has fulfilled a goal and must close it with an evidence summary.",
		providers.McpClientClaude:  "Call when Claude Code has fulfilled a goal and must close it with an evidence summary.",
		providers.McpClientCodex:   "Call when Codex has fulfilled a goal and must close it with an evidence summary.",
	},
	"tool_goal_reopen": {
		providers.McpClientGeneric: "Call when a closed goal must be reopened because its outcome did not hold.",
		providers.McpClientCursor:  "Call when the Cursor agent must reopen a closed goal because its outcome did not hold.",
		providers.McpClientKiro:    "Call when the Kiro agent must reopen a closed goal because its outcome did not hold.",
		providers.McpClientCopilot: "Call when the Copilot agent must reopen a closed goal because its outcome did not hold.",
		providers.McpClientClaude:  "Call when Claude Code must reopen a closed goal because its outcome did not hold.",
		providers.McpClientCodex:   "Call when Codex must reopen a closed goal because its outcome did not hold.",
	},
	"arg_plan_id": {
		providers.McpClientCursor:  "Set when a `.cursor/plans/*_<id>.plan.md` file exists and must be archived into the ticket on close.",
		providers.McpClientCopilot: "Set when a Copilot project memory file for this id exists and must be archived into the ticket on close.",
		providers.McpClientClaude:  "Set when a `.claude/plan/<id>.md` file exists and must be archived into the ticket on close.",
		providers.McpClientCodex:   "Set when a `.codex/memory/<repo>/<id>.md` file exists and must be archived into the ticket on close.",
	},
	"arg_spec_id": {
		providers.McpClientKiro: "Set when a `.kiro/specs/<id>/` directory exists and must be archived into the ticket on close.",
	},
}

// #endregion 🗣️McpDescriptions

// #endregion 🔌️Mcp Surface
