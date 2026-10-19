//go:build exhaustive

// ⚖️ Exhaustive tests of the cli domain: MCP ticket close, list parity and mermaid commands.

package cli

import (
	bytes "bytes"
	fmt "fmt"
	os "os"
	strings "strings"
	testing "testing"
	unicode "unicode"

	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
)

func TestExhaustiveMcpTicketCloseAutoResolve(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mcp ticket close auto-resolve test in short mode")
	}
	withExhaustiveRepo(t)

	result := ToolTicketOpen("🎫️", "Test Auto Resolve Close", "Test prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*model.Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}
	defer func() {
		ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "cleanup", []string{"repo/client/package.json"}, "", true)
		os.RemoveAll(ticketspkg.GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug))
	}()

	year, month, day, slug, err := resolveTicketForClose("")
	if err != nil {
		t.Fatalf("resolveTicketForClose('') error: %v", err)
	}
	resolved, err := ticketspkg.ReadTicket(year, month, day, slug)
	if err != nil {
		t.Fatalf("resolved ticket not readable: %v", err)
	}
	if resolved.Status != model.TicketStatusOpen {
		t.Errorf("resolveTicketForClose('') resolved to non-open ticket (status=%s)", resolved.Status)
	}
}

func TestExhaustiveMcpTicketCloseWithFullYearPath(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mcp ticket close full-year-path test in short mode")
	}
	withExhaustiveRepo(t)

	result := ToolTicketOpen("🎫️", "Test Full Year Path", "Test prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*model.Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}
	defer func() {
		os.RemoveAll(ticketspkg.GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug))
	}()

	fullYearPath := fmt.Sprintf("%d/%02d/%02d/%s", 2000+ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	year, month, day, slug, err := parseTicketPath(fullYearPath)
	if err != nil {
		t.Fatalf("parseTicketPath(%q) error: %v", fullYearPath, err)
	}

	closeResult := ToolTicketClose(year, month, day, slug, "Test summary", []string{"repo/client/package.json"}, "", true)
	if closeResult.Error != "" {
		t.Fatalf("ToolTicketClose with full year path returned error: %s", closeResult.Error)
	}
}

// ⛳️#region 🕹️Output Parity
func TestExhaustiveParityGoalList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity goal list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-goal")
		if err != nil {
			t.Fatalf("CLI goal list failed: %v", err)
		}
		toolResult := ToolGoalList()
		if toolResult.Error != "" {
			t.Fatalf("ToolGoalList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of goals", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-goal")
		toolResult := ToolGoalList()
		mcpOut := toolOutputText(toolResult)
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})

	t.Run("empty output when no goals match filter", func(t *testing.T) {

		cliOut, _, _ := executeCommandMd("list", "--only-goal")
		mcpOut := toolOutputText(ToolGoalList())
		if len(cliOut) == 0 && len(mcpOut) != 0 {
			t.Error("CLI produced empty output but MCP did not")
		}
		if len(cliOut) != 0 && len(mcpOut) == 0 {
			t.Error("MCP produced empty output but CLI did not")
		}
	})
}

func TestExhaustiveParityContributorList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity contributor list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-contributor")
		if err != nil {
			t.Fatalf("CLI contributor list failed: %v", err)
		}
		toolResult := ToolContributorList()
		if toolResult.Error != "" {
			t.Fatalf("ToolContributorList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of contributors", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-contributor")
		mcpOut := toolOutputText(ToolContributorList())
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestExhaustiveParityTicketList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity ticket list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-ticket")
		if err != nil {
			t.Fatalf("CLI ticket list failed: %v", err)
		}
		toolResult := ToolTicketList(nil, nil, nil)
		if toolResult.Error != "" {
			t.Fatalf("ToolTicketList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of tickets", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-ticket")
		mcpOut := toolOutputText(ToolTicketList(nil, nil, nil))
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestExhaustiveParityTechnologyList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity technology list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-technology")
		if err != nil {
			t.Fatalf("CLI technology list failed: %v", err)
		}
		toolResult := ToolTechnologyList()
		if toolResult.Error != "" {
			t.Fatalf("ToolTechnologyList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return non-empty output", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-technology")
		mcpOut := toolOutputText(ToolTechnologyList())
		if len(cliOut) == 0 {
			t.Error("CLI technology list returned empty output")
		}
		if len(mcpOut) == 0 {
			t.Error("MCP technology list returned empty output")
		}
	})
}

func TestExhaustiveParityTechnologyTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity technology tree test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("search", "--only-technology")
		if err != nil {
			t.Fatalf("CLI technology tree failed: %v", err)
		}
		toolResult := ToolTechnologyTree()
		if toolResult.Error != "" {
			t.Fatalf("ToolTechnologyTree returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("technologies are sorted alphabetically", func(t *testing.T) {
		mcpOut := toolOutputText(ToolTechnologyTree())
		lines := strings.Split(strings.TrimSpace(mcpOut), "\n")
		var technologyNames []string
		for _, line := range lines {
			trimmed := strings.TrimSpace(line)
			if idx := strings.Index(trimmed, "repo://technology/"); idx >= 0 {
				rest := trimmed[idx+len("repo://technology/"):]
				endParen := strings.Index(rest, ")")
				if endParen >= 0 {
					technologyNames = append(technologyNames, strings.TrimLeftFunc(rest[:endParen], func(r rune) bool { return !unicode.IsLetter(r) && !unicode.IsDigit(r) }))
				}
			}
		}
		for i := 1; i < len(technologyNames); i++ {
			if technologyNames[i] < technologyNames[i-1] {
				t.Errorf("technologies not sorted: %q comes after %q", technologyNames[i], technologyNames[i-1])
			}
		}
	})
}

func TestExhaustiveParityPolicyList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity policy list test in short mode")
	}
	withExhaustiveRepo(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-policy")
		if err != nil {
			t.Fatalf("CLI policy list failed: %v", err)
		}
		toolResult := ToolPolicyList()
		if toolResult.Error != "" {
			t.Fatalf("ToolPolicyList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of policies", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-policy")
		mcpOut := toolOutputText(ToolPolicyList())
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestExhaustiveMermaidCommandLocByTechnologiesBundlesFoldersFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mermaid loc command test in short mode")
	}
	withExhaustiveRepo(t)
	cmd := NewRoot(testEngineFactory)
	buf := new(bytes.Buffer)
	cmd.SetOut(buf)
	cmd.SetArgs([]string{"mermaid", "loc-by-technologies-bundles-folders-files"})
	if err := cmd.Execute(); err != nil {
		t.Fatalf("command failed: %v", err)
	}
	output := buf.String()
	if !strings.HasPrefix(output, "treemap-beta\n") {
		t.Errorf("expected treemap-beta output, got: %s", output[:min(len(output), 100)])
	}
}

func TestExhaustiveMermaidCommandLocByLanguage(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mermaid loc-by-language command test in short mode")
	}
	withExhaustiveRepo(t)
	cmd := NewRoot(testEngineFactory)
	buf := new(bytes.Buffer)
	cmd.SetOut(buf)
	cmd.SetArgs([]string{"mermaid", "loc-by-language"})
	if err := cmd.Execute(); err != nil {
		t.Fatalf("command failed: %v", err)
	}
	output := buf.String()
	if !strings.HasPrefix(output, "treemap-beta\n") {
		t.Errorf("expected treemap-beta output, got: %s", output[:min(len(output), 100)])
	}
}
