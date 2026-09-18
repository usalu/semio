// 🐹️ Go subject for the advertised capability surface. The Go implementation is a `package main`
// process, so the adapter drives the real `semio-repo-mcp` binary once per profile and lists what a
// client would see.
package adapter

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"

	host "semio.tech/repo/test"
)

// region 🔖️Support

// 🗃️binaryPath returns the built MCP binary, building it into the marked cache when it is absent.
func binaryPath(repoRoot string) (string, error) {
	name := "semio-repo-mcp"
	if runtime.GOOS == "windows" {
		name += ".exe"
	}
	binary := filepath.Join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "🗃️bin", name)
	if _, err := os.Stat(binary); err == nil {
		return binary, nil
	}
	if err := os.MkdirAll(filepath.Dir(binary), 0o755); err != nil {
		return "", err
	}
	build := exec.Command("bun", "./📜️script.ts", "build")
	build.Dir = filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "💻️client", "🔌️mcp")
	build.Env = append(os.Environ(), "GOWORK="+filepath.Join(repoRoot, "go.work"), "GOFLAGS=")
	if output, err := build.CombinedOutput(); err != nil {
		return "", fmt.Errorf("build semio-repo-mcp: %v: %s", err, output)
	}
	return binary, nil
}

type listing struct {
	Tools []struct {
		Name        string `json:"name"`
		Description string `json:"description"`
		InputSchema struct {
			Properties map[string]json.RawMessage `json:"properties"`
		} `json:"inputSchema"`
	} `json:"tools"`
	Resources []struct {
		URI         string `json:"uri"`
		Description string `json:"description"`
	} `json:"resources"`
	Prompts []struct {
		Name        string `json:"name"`
		Description string `json:"description"`
	} `json:"prompts"`
}

// 📋️listSurface runs one binary for one profile and returns its three listings.
func listSurface(repoRoot string, profile string) (listing, error) {
	binary, err := binaryPath(repoRoot)
	if err != nil {
		return listing{}, err
	}
	command := exec.Command(binary)
	command.Dir = repoRoot
	command.Env = append(os.Environ(), "SEMIO_REPO_MCP_CLIENT="+profile)
	stdin, err := command.StdinPipe()
	if err != nil {
		return listing{}, err
	}
	stdout, err := command.StdoutPipe()
	if err != nil {
		return listing{}, err
	}
	if err := command.Start(); err != nil {
		return listing{}, err
	}
	reader := bufio.NewReader(stdout)
	read := func() (json.RawMessage, error) {
		line, err := reader.ReadBytes('\n')
		if err != nil {
			return nil, err
		}
		var envelope struct {
			Result json.RawMessage `json:"result"`
			Error  json.RawMessage `json:"error"`
		}
		if err := json.Unmarshal(line, &envelope); err != nil {
			return nil, err
		}
		if envelope.Error != nil {
			return nil, fmt.Errorf("listing failed: %s", envelope.Error)
		}
		return envelope.Result, nil
	}
	if _, err := fmt.Fprintln(stdin, `{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"listing","version":"1"}}}`); err != nil {
		return listing{}, err
	}
	if _, err := read(); err != nil {
		return listing{}, err
	}
	if _, err := fmt.Fprintln(stdin, `{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}`); err != nil {
		return listing{}, err
	}
	var surface listing
	for _, method := range []string{"tools/list", "resources/list", "prompts/list"} {
		if _, err := fmt.Fprintf(stdin, "{\"jsonrpc\":\"2.0\",\"id\":%q,\"method\":%q,\"params\":{}}\n", method, method); err != nil {
			return listing{}, err
		}
		result, err := read()
		if err != nil {
			return listing{}, err
		}
		if err := json.Unmarshal(result, &surface); err != nil {
			return listing{}, err
		}
	}
	_ = stdin.Close()
	_ = command.Wait()
	return surface, nil
}

type entry struct {
	ID          string `json:"id"`
	Description string `json:"description"`
}

// endregion 🔖️Support

// region 🔖️Scenarios

func genericProfileSurface(ctx *host.Context) (host.Outcome, error) {
	surface, err := listSurface(ctx.RepoRoot, "client")
	if err != nil {
		return host.Outcome{}, err
	}
	tools := make([]entry, 0, len(surface.Tools))
	for _, tool := range surface.Tools {
		tools = append(tools, entry{ID: tool.Name, Description: tool.Description})
	}
	resources := make([]entry, 0, len(surface.Resources))
	for _, resource := range surface.Resources {
		resources = append(resources, entry{ID: resource.URI, Description: resource.Description})
	}
	prompts := make([]entry, 0, len(surface.Prompts))
	for _, prompt := range surface.Prompts {
		prompts = append(prompts, entry{ID: prompt.Name, Description: prompt.Description})
	}
	return host.Outcome{Projection: map[string]any{"tools": tools, "resources": resources, "prompts": prompts}}, nil
}

func ideProfileSurface(ctx *host.Context) (host.Outcome, error) {
	raw, err := ctx.FixtureBytes("shared://📋️surface.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var fixture struct {
		Profiles []struct {
			Slug string `json:"slug"`
		} `json:"profiles"`
		TicketTools []string `json:"ticketTools"`
	}
	if err := json.Unmarshal(raw, &fixture); err != nil {
		return host.Outcome{}, err
	}
	ticket := map[string]bool{}
	for _, name := range fixture.TicketTools {
		ticket[name] = true
	}
	rows := make([]map[string]any, 0, len(fixture.Profiles))
	for _, profile := range fixture.Profiles {
		surface, err := listSurface(ctx.RepoRoot, profile.Slug)
		if err != nil {
			return host.Outcome{}, err
		}
		arguments := make([]map[string]any, 0, len(ticket))
		for _, tool := range surface.Tools {
			if !ticket[tool.Name] {
				continue
			}
			extra := ""
			if _, ok := tool.InputSchema.Properties["plan_id"]; ok {
				extra = "plan_id"
			} else if _, ok := tool.InputSchema.Properties["spec_id"]; ok {
				extra = "spec_id"
			}
			arguments = append(arguments, map[string]any{"tool": tool.Name, "extraArgument": extra})
		}
		rows = append(rows, map[string]any{"slug": profile.Slug, "ticketArguments": arguments})
	}
	return host.Outcome{Projection: map[string]any{"profiles": rows}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("generic-profile-surface", genericProfileSurface).
		Subject("ide-profile-surface", ideProfileSurface)
}

// endregion 🔖️Registration
