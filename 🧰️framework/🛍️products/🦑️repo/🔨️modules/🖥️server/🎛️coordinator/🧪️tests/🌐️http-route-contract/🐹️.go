// 🐹️ Go side of the HTTP route case. Starts the coordinator on an ephemeral port, replays the
// committed request fixture and projects the status and normalised body of every answer.
package adapter

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"time"

	coordinator "github.com/usalu/semio/repo/coordinator"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type routeRequest struct {
	Name   string            `json:"name"`
	Method string            `json:"method"`
	Path   string            `json:"path"`
	Header map[string]string `json:"header"`
	Body   json.RawMessage   `json:"body"`
}

type routeFixture struct {
	Requests     []routeRequest `json:"requests"`
	VolatileKeys []string       `json:"volatileKeys"`
}

func loadRouteFixture(ctx *host.Context) (routeFixture, error) {
	var fixture routeFixture
	data, err := ctx.FixtureBytes("local://🌐️requests.json")
	if err != nil {
		return fixture, err
	}
	return fixture, json.Unmarshal(data, &fixture)
}

// startCoordinator brings up one coordinator with an empty log in a directory of this scenario's own.
func startCoordinator(ctx *host.Context, name string) (*coordinator.Service, error) {
	dir := filepath.Join(ctx.WorkDir, name)
	if err := os.RemoveAll(dir); err != nil {
		return nil, err
	}
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return nil, err
	}
	return coordinator.Start(coordinator.Config{
		Address:          "127.0.0.1:0",
		DatabasePath:     filepath.Join(dir, "coordinator.events"),
		RepoRoot:         dir,
		RequestBodyLimit: 10 * 1024 * 1024,
	})
}

func normalizeBody(raw []byte, volatile []string) any {
	trimmed := strings.TrimSpace(string(raw))
	if trimmed == "" {
		return ""
	}
	var value any
	if json.Unmarshal([]byte(trimmed), &value) != nil {
		return trimmed
	}
	return scrub(value, volatile)
}

func scrub(value any, volatile []string) any {
	switch typed := value.(type) {
	case map[string]any:
		out := map[string]any{}
		for key, item := range typed {
			if item != nil && contains(volatile, key) {
				out[key] = "<volatile>"
				continue
			}
			out[key] = scrub(item, volatile)
		}
		return out
	case []any:
		out := make([]any, len(typed))
		for index, item := range typed {
			out[index] = scrub(item, volatile)
		}
		return out
	default:
		return value
	}
}

func contains(values []string, expected string) bool {
	for _, value := range values {
		if value == expected {
			return true
		}
	}
	return false
}

func replay(ctx *host.Context, name string, keep func(routeRequest, int) bool) ([]map[string]any, error) {
	fixture, err := loadRouteFixture(ctx)
	if err != nil {
		return nil, err
	}
	service, err := startCoordinator(ctx, name)
	if err != nil {
		return nil, err
	}
	defer service.Stop()
	answers := make([]map[string]any, 0, len(fixture.Requests))
	for _, request := range fixture.Requests {
		status, body, err := issueWithVolatile(service.Address, request, fixture.VolatileKeys)
		if err != nil {
			return nil, err
		}
		if !keep(request, status) {
			continue
		}
		answers = append(answers, map[string]any{"name": request.Name, "status": status, "body": body})
	}
	return answers, nil
}

func issueWithVolatile(address string, request routeRequest, volatile []string) (int, any, error) {
	var body io.Reader
	if len(request.Body) > 0 {
		body = bytes.NewReader(request.Body)
	}
	built, err := http.NewRequest(request.Method, "http://"+address+request.Path, body)
	if err != nil {
		return 0, nil, err
	}
	if len(request.Body) > 0 {
		built.Header.Set("Content-Type", "application/json")
	}
	for name, value := range request.Header {
		built.Header.Set(name, value)
	}
	client := &http.Client{Timeout: 10 * time.Second}
	response, err := client.Do(built)
	if err != nil {
		return 0, nil, err
	}
	defer response.Body.Close()
	raw, err := io.ReadAll(response.Body)
	if err != nil {
		return 0, nil, err
	}
	return response.StatusCode, normalizeBody(raw, volatile), nil
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyRouteAnswersTheSameStatusAndBody(ctx *host.Context) (host.Outcome, error) {
	answers, err := replay(ctx, "route-contract", func(routeRequest, int) bool { return true })
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"answers": answers}}, nil
}

func aMethodMismatchAndAMissingFieldAreRefused(ctx *host.Context) (host.Outcome, error) {
	answers, err := replay(ctx, "route-refusals", func(_ routeRequest, status int) bool { return status >= 400 })
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"refusals": answers}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-route-answers-the-same-status-and-body", everyRouteAnswersTheSameStatusAndBody).
		Subject("a-method-mismatch-and-a-missing-field-are-refused", aMethodMismatchAndAMissingFieldAreRefused)
}

// endregion 🔖️Registration
