// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📊️parsing carries the test outcome types and the parsers of every runner output format.

// #endregion 🧲️Header

package testrunner

import (
	json "encoding/json"
	strconv "strconv"
	strings "strings"
	unicode "unicode"
)

// #region 📊️Parsing

// 🚦️TestStatus is how one test ended.
type TestStatus string

// ✅️TestStatusPassed is a passing test.
const TestStatusPassed TestStatus = "passed"

// ❌️TestStatusFailed is a failing test.
const TestStatusFailed TestStatus = "failed"

// ⏭️TestStatusSkipped is a skipped, ignored or pending test.
const TestStatusSkipped TestStatus = "skipped"

// 🚦️RunStatus is how one invocation ended.
type RunStatus string

// ✅️RunStatusPassed means every test passed.
const RunStatusPassed RunStatus = "passed"

// ❌️RunStatusFailed means at least one test failed, or the runner exited non-zero.
const RunStatusFailed RunStatus = "failed"

// 🛑️RunStatusCancelled means the invocation was cancelled before it produced a verdict.
const RunStatusCancelled RunStatus = "cancelled"

// ⚪️RunStatusNotRun means the invocation never started.
const RunStatusNotRun RunStatus = "not-run"

// 🧪️TestCaseOutcome is one test, normalised across every runner dialect.
type TestCaseOutcome struct {
	// 🏷️Name is the test name as the runner reported it.
	Name string `json:"name"`
	// 📦️Suite is the package, file or suite the runner attributed it to.
	Suite string `json:"suite,omitempty"`
	// 🚦️Status is the verdict.
	Status TestStatus `json:"status"`
	// ⏱️DurationMs is the duration in milliseconds where the runner reports one.
	DurationMs *float64 `json:"duration_ms,omitempty"`
	// 💬️Message is the failure text where the runner reports one.
	Message string `json:"message,omitempty"`
}

// 🔢️TestTotals are counts, always recomputed from the cases so a report's own summary cannot drift.
type TestTotals struct {
	// ✅️Passed is the number of passing tests.
	Passed int `json:"passed"`
	// ❌️Failed is the number of failing tests.
	Failed int `json:"failed"`
	// ⏭️Skipped is the number of skipped tests.
	Skipped int `json:"skipped"`
	// 🔢️Total is the number of reported tests.
	Total int `json:"total"`
}

// 📊️TestOutcome is one invocation's result in the one model every runner folds into.
type TestOutcome struct {
	// 🏭️Runner is which runner produced it.
	Runner Runner `json:"runner"`
	// 🚦️Status is the overall verdict.
	Status RunStatus `json:"status"`
	// 🧪️Tests is every reported test, in report order.
	Tests []TestCaseOutcome `json:"tests"`
	// 🔢️Totals are the counts.
	Totals TestTotals `json:"totals"`
	// 🔢️ExitStatus is the runner exit status where one was observed.
	ExitStatus *int `json:"exit_status,omitempty"`
}

// 📊️ParseOutcome dispatches a runner's native report to its parser.
func ParseOutcome(runner Runner, output ProcessOutput) TestOutcome {
	switch runner {
	case RunnerCargo:
		return ParseCargoTest(output)
	case RunnerCargoNextest:
		return ParseCargoNextest(output)
	case RunnerDotnet:
		return ParseDotnetTest(output)
	case RunnerNpx, RunnerNpm:
		return ParseVitestJSON(output)
	case RunnerUv, RunnerPytest:
		return ParsePytest(output)
	case RunnerRspec:
		return ParseRspecJSON(output)
	}
	return ParseGoTestJSON(output)
}

// 🐹️ParseGoTestJSON reads `go test -json`: one JSON object per line, verdicts on pass, fail and skip.
func ParseGoTestJSON(output ProcessOutput) TestOutcome {
	messages := map[string]string{}
	tests := []TestCaseOutcome{}
	for _, line := range textLines(output.Stdout) {
		var event map[string]any
		if json.Unmarshal([]byte(strings.TrimSpace(line)), &event) != nil || event == nil {
			continue
		}
		action, _ := event["Action"].(string)
		pkg, _ := event["Package"].(string)
		name, ok := event["Test"].(string)
		if !ok {
			continue
		}
		key := pkg + "::" + name
		if action == "output" {
			text, _ := event["Output"].(string)
			messages[key] += text
			continue
		}
		var status TestStatus
		switch action {
		case "pass":
			status = TestStatusPassed
		case "fail":
			status = TestStatusFailed
		case "skip":
			status = TestStatusSkipped
		default:
			continue
		}
		var durationMs *float64
		if elapsed, ok := event["Elapsed"].(float64); ok {
			value := elapsed * 1000
			durationMs = &value
		}
		message := ""
		if status == TestStatusFailed {
			message = strings.TrimRightFunc(messages[key], unicode.IsSpace)
		}
		tests = append(tests, TestCaseOutcome{Name: name, Suite: pkg, Status: status, DurationMs: durationMs, Message: message})
	}
	return finishOutcome(RunnerGo, tests, output)
}

// 🟦️ParseVitestJSON reads vitest's JSON reporter, the shape jest's reporter also emits.
func ParseVitestJSON(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	var report map[string]any
	if json.Unmarshal([]byte(jsonBody(output.Stdout)), &report) == nil {
		files, _ := report["testResults"].([]any)
		for _, entry := range files {
			file, _ := entry.(map[string]any)
			suite, _ := file["name"].(string)
			assertions, _ := file["assertionResults"].([]any)
			for _, item := range assertions {
				assertion, _ := item.(map[string]any)
				name, ok := assertion["fullName"].(string)
				if !ok {
					name, _ = assertion["title"].(string)
				}
				status := TestStatusSkipped
				switch verdict, _ := assertion["status"].(string); verdict {
				case "passed":
					status = TestStatusPassed
				case "failed":
					status = TestStatusFailed
				}
				var durationMs *float64
				if duration, ok := assertion["duration"].(float64); ok {
					value := duration
					durationMs = &value
				}
				failures, _ := assertion["failureMessages"].([]any)
				parts := make([]string, 0, len(failures))
				for _, failure := range failures {
					if text, ok := failure.(string); ok {
						parts = append(parts, text)
					}
				}
				tests = append(tests, TestCaseOutcome{Name: name, Suite: suite, Status: status, DurationMs: durationMs, Message: strings.Join(parts, "\n")})
			}
		}
	}
	return finishOutcome(RunnerNpx, tests, output)
}

// 🐍️ParsePytest reads pytest's verbose terminal report: `<nodeid> <VERDICT>` lines.
func ParsePytest(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	for _, line := range textLines(output.Stdout) {
		trimmed := strings.TrimRightFunc(line, unicode.IsSpace)
		node, tail, ok := strings.Cut(trimmed, " ")
		if !ok || !strings.Contains(node, "::") {
			continue
		}
		verdict := ""
		if fields := strings.Fields(tail); len(fields) > 0 {
			verdict = fields[0]
		}
		var status TestStatus
		switch verdict {
		case "PASSED", "XPASS":
			status = TestStatusPassed
		case "FAILED", "ERROR":
			status = TestStatusFailed
		case "SKIPPED", "XFAIL":
			status = TestStatusSkipped
		default:
			continue
		}
		suite, name, _ := strings.Cut(node, "::")
		tests = append(tests, TestCaseOutcome{Name: name, Suite: suite, Status: status})
	}
	return finishOutcome(RunnerPytest, tests, output)
}

// 🦀️ParseCargoTest reads libtest's text report: `test <path> ... ok|FAILED|ignored`.
func ParseCargoTest(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	for _, line := range textLines(output.Stdout) {
		rest, ok := strings.CutPrefix(strings.TrimSpace(line), "test ")
		if !ok {
			continue
		}
		name, verdict, ok := strings.Cut(rest, " ... ")
		if !ok {
			continue
		}
		var status TestStatus
		switch trimmed := strings.TrimSpace(verdict); {
		case trimmed == "ok":
			status = TestStatusPassed
		case trimmed == "FAILED":
			status = TestStatusFailed
		case strings.HasPrefix(trimmed, "ignored"):
			status = TestStatusSkipped
		default:
			continue
		}
		message := ""
		if status == TestStatusFailed {
			message = cargoFailureBody(output.Stdout, strings.TrimSpace(name))
		}
		tests = append(tests, TestCaseOutcome{Name: strings.TrimSpace(name), Status: status, Message: message})
	}
	return finishOutcome(RunnerCargo, tests, output)
}

// 🦀️ParseCargoNextest reads cargo-nextest's report: `        PASS [   0.012s] <crate> <test>`.
func ParseCargoNextest(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	for _, line := range append(textLines(output.Stdout), textLines(output.Stderr)...) {
		verdict, rest, ok := strings.Cut(strings.TrimSpace(line), " ")
		if !ok {
			continue
		}
		var status TestStatus
		switch verdict {
		case "PASS":
			status = TestStatusPassed
		case "FAIL", "TRY":
			status = TestStatusFailed
		case "SKIP":
			status = TestStatusSkipped
		default:
			continue
		}
		rest = strings.TrimSpace(rest)
		closing := strings.Index(rest, "]")
		if closing < 0 {
			continue
		}
		var durationMs *float64
		text := strings.TrimSpace(strings.TrimRight(strings.TrimSpace(strings.TrimLeft(rest[:closing], "[")), "s"))
		if seconds, err := strconv.ParseFloat(text, 64); err == nil {
			value := seconds * 1000
			durationMs = &value
		}
		identifier := strings.TrimSpace(rest[closing+1:])
		suite, name, ok := strings.Cut(identifier, " ")
		if !ok {
			suite, name = "", identifier
		}
		tests = append(tests, TestCaseOutcome{Name: strings.TrimSpace(name), Suite: strings.TrimSpace(suite), Status: status, DurationMs: durationMs})
	}
	return finishOutcome(RunnerCargoNextest, tests, output)
}

// 🔷️ParseDotnetTest reads `dotnet test`'s console logger: `  Passed <name> [1 ms]`.
func ParseDotnetTest(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	for _, line := range textLines(output.Stdout) {
		verdict, rest, ok := strings.Cut(strings.TrimSpace(line), " ")
		if !ok {
			continue
		}
		var status TestStatus
		switch verdict {
		case "Passed":
			status = TestStatusPassed
		case "Failed":
			status = TestStatusFailed
		case "Skipped":
			status = TestStatusSkipped
		default:
			continue
		}
		rest = strings.TrimSpace(rest)
		if rest == "" || strings.HasPrefix(rest, "!") || strings.HasPrefix(rest, "-") {
			continue
		}
		name, durationMs := rest, (*float64)(nil)
		if index := strings.LastIndex(rest, " ["); index >= 0 {
			name = strings.TrimSpace(rest[:index])
			durationMs = parseDotnetDuration(strings.TrimRight(rest[index+2:], "]"))
		}
		tests = append(tests, TestCaseOutcome{Name: name, Status: status, DurationMs: durationMs})
	}
	return finishOutcome(RunnerDotnet, tests, output)
}

// 💎️ParseRspecJSON reads RSpec's `--format json` document.
func ParseRspecJSON(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	var report map[string]any
	if json.Unmarshal([]byte(jsonBody(output.Stdout)), &report) == nil {
		examples, _ := report["examples"].([]any)
		for _, entry := range examples {
			example, _ := entry.(map[string]any)
			name, ok := example["full_description"].(string)
			if !ok {
				name, _ = example["description"].(string)
			}
			status := TestStatusSkipped
			switch verdict, _ := example["status"].(string); verdict {
			case "passed":
				status = TestStatusPassed
			case "failed":
				status = TestStatusFailed
			}
			message := ""
			if exception, ok := example["exception"].(map[string]any); ok {
				message, _ = exception["message"].(string)
			}
			var durationMs *float64
			if runTime, ok := example["run_time"].(float64); ok {
				value := runTime * 1000
				durationMs = &value
			}
			suite, _ := example["file_path"].(string)
			tests = append(tests, TestCaseOutcome{Name: name, Suite: suite, Status: status, DurationMs: durationMs, Message: message})
		}
	}
	return finishOutcome(RunnerRspec, tests, output)
}

func finishOutcome(runner Runner, tests []TestCaseOutcome, output ProcessOutput) TestOutcome {
	totals := TestTotals{Total: len(tests)}
	for _, test := range tests {
		switch test.Status {
		case TestStatusPassed:
			totals.Passed++
		case TestStatusFailed:
			totals.Failed++
		case TestStatusSkipped:
			totals.Skipped++
		}
	}
	status := RunStatusPassed
	if totals.Failed > 0 || output.Status != 0 {
		status = RunStatusFailed
	}
	exitStatus := output.Status
	return TestOutcome{Runner: runner, Status: status, Tests: tests, Totals: totals, ExitStatus: &exitStatus}
}

func textLines(text string) []string {
	if text == "" {
		return nil
	}
	lines := strings.Split(text, "\n")
	if lines[len(lines)-1] == "" {
		lines = lines[:len(lines)-1]
	}
	for index, line := range lines {
		lines[index] = strings.TrimSuffix(line, "\r")
	}
	return lines
}

func jsonBody(text string) string {
	start := strings.Index(text, "{")
	if start < 0 {
		start = 0
	}
	end := len(text)
	if last := strings.LastIndex(text, "}"); last >= 0 {
		end = last + 1
	}
	if start < end {
		return text[start:end]
	}
	return text
}

func parseDotnetDuration(value string) *float64 {
	number, unit, ok := strings.Cut(strings.TrimSpace(value), " ")
	if !ok {
		return nil
	}
	amount, err := strconv.ParseFloat(number, 64)
	if err != nil {
		return nil
	}
	switch unit {
	case "ms":
		return &amount
	case "s":
		scaled := amount * 1000
		return &scaled
	case "us":
		scaled := amount / 1000
		return &scaled
	}
	return nil
}

func cargoFailureBody(stdout string, name string) string {
	header := "---- " + name + " stdout ----"
	index := strings.Index(stdout, header)
	if index < 0 {
		return ""
	}
	rest := stdout[index+len(header):]
	end := strings.Index(rest, "\n----")
	if end < 0 {
		end = len(rest)
	}
	return strings.TrimSpace(rest[:end])
}

// #endregion 📊️Parsing
