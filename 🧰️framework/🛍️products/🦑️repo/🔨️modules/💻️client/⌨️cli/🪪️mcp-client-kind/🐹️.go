// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// MCP client kind selection.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"strings"
)

type McpClientKind string

const (
	McpClientGeneric McpClientKind = "generic"
	McpClientCursor  McpClientKind = "cursor"
	McpClientKiro    McpClientKind = "kiro"
	McpClientCopilot McpClientKind = "copilot"
	McpClientClaude  McpClientKind = "claude"
	McpClientCodex   McpClientKind = "codex"
)

// ParseMcpClientKind maps a CLI/MCP profile slug to an MCP server kind.
func ParseMcpClientKind(raw string) (McpClientKind, error) {
	switch strings.ToLower(strings.TrimSpace(raw)) {
	case "", "generic", "client":
		return McpClientGeneric, nil
	case "cursor":
		return McpClientCursor, nil
	case "kiro":
		return McpClientKiro, nil
	case "copilot":
		return McpClientCopilot, nil
	case "claude":
		return McpClientClaude, nil
	case "codex":
		return McpClientCodex, nil
	default:
		return "", fmt.Errorf("unknown mcp kind %q (expected client, cursor, copilot, claude, codex, or kiro)", raw)
	}
}

// HookClientForMcpKind maps an MCP entry binary to the hook client id used by ResolveHookEvent.
func HookClientForMcpKind(kind McpClientKind) string {
	switch kind {
	case McpClientCursor:
		return "cursor-chat"
	case McpClientKiro:
		return "kiro-cli"
	case McpClientCopilot:
		return "copilot-chat"
	case McpClientClaude:
		return "claude-code"
	case McpClientCodex:
		return "codex"
	default:
		return ""
	}
}

// 🧾️resolveMcpTicketClient derives native ticket metadata from the MCP entrypoint.
func resolveMcpTicketClient(kind McpClientKind, client string) string {
	if strings.TrimSpace(client) != "" {
		return client
	}
	return HookClientForMcpKind(kind)
}

// McpKindFromResolvedClient maps a validated ticket client slug to an MCP surface kind for plan/spec attachment.
func McpKindFromResolvedClient(client string) McpClientKind {
	switch strings.TrimSpace(client) {
	case "cursor-chat", "cursor":
		return McpClientCursor
	case "kiro-cli":
		return McpClientKiro
	case "copilot-chat":
		return McpClientCopilot
	case "claude-code":
		return McpClientClaude
	case "codex":
		return McpClientCodex
	default:
		return McpClientGeneric
	}
}

// McpServerName returns the MCP server identifier string for the given kind.
func McpServerName(kind McpClientKind) string {
	switch kind {
	case McpClientCursor, McpClientKiro, McpClientCopilot, McpClientClaude, McpClientCodex, McpClientGeneric, "":
		return "repo"
	default:
		return "repo"
	}
}
