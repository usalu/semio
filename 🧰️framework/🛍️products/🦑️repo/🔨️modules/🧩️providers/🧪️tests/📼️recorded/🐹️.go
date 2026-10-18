package providers

import (
	"encoding/json"
	"runtime"
	"testing"
)

// #region 🧪️ProcessRunner

func TestRecordedProcessRunnerReplaysByArgvAndLogsEveryCall(t *testing.T) {
	runner := NewRecordedProcessRunner(ProcessTranscript{Exchanges: []ProcessExchange{
		{Argv: []string{"gh", "api", "user", "--jq", ".login"}, Stdout: "usalu\n"},
	}})
	outcome := runner.Run(ProcessRequest{Argv: []string{"gh", "api", "user", "--jq", ".login"}})
	if outcome.Stdout != "usalu\n" || outcome.Status != 0 {
		t.Fatalf("expected the recorded stdout, got %+v", outcome)
	}
	if again := runner.Run(ProcessRequest{Argv: []string{"gh", "api", "user", "--jq", ".login"}}); again.Status != 127 {
		t.Fatalf("expected a consumed exchange to be replayed at most once, got %+v", again)
	}
	if missing := runner.Run(ProcessRequest{Argv: []string{"gh", "nope"}}); missing.Status != 127 {
		t.Fatalf("expected exit 127 for an unrecorded argv, got %+v", missing)
	}
	if len(runner.Issued()) != 3 {
		t.Fatalf("expected every call to be logged, got %d", len(runner.Issued()))
	}
}

func TestRecordedProcessRunnerParsesTheFixtureShape(t *testing.T) {
	var transcript ProcessTranscript
	if err := json.Unmarshal([]byte(`{"exchanges":[{"argv":["git","rev-parse","HEAD"],"stdout":"abc\n","stderr":"","status":0}]}`), &transcript); err != nil {
		t.Fatalf("transcript fixture must parse: %v", err)
	}
	if got := NewRecordedProcessRunner(transcript).Run(ProcessRequest{Argv: []string{"git", "rev-parse", "HEAD"}}).Stdout; got != "abc\n" {
		t.Fatalf("expected abc, got %q", got)
	}
}

// 🐢️Named for the `quick` level: it spawns three real child processes, which the fundamental level
// deliberately does not pay for. `goLevelTestArgs` skips `Test<Level>`-prefixed tests below it.
func TestQuickSystemProcessRunnerReportsStatusAndLogsArgv(t *testing.T) {
	runner := NewSystemProcessRunner()
	program, args := "sh", []string{"-c", "exit 3"}
	if runtime.GOOS == "windows" {
		program, args = "cmd", []string{"/c", "exit 3"}
	}
	outcome := runner.Run(ProcessRequest{Argv: append([]string{program}, args...)})
	if outcome.Status != 3 {
		t.Fatalf("expected exit 3, got %+v", outcome)
	}
	if missing := runner.Run(ProcessRequest{Argv: []string{"semio-no-such-program"}}); missing.Status != 127 {
		t.Fatalf("expected exit 127 for a missing program, got %+v", missing)
	}
	if len(runner.Issued()) != 2 {
		t.Fatalf("expected two logged calls, got %d", len(runner.Issued()))
	}
	if empty := runner.Run(ProcessRequest{}); empty.Status != 127 || empty.Stderr != "empty argv" {
		t.Fatalf("expected an empty argv to be reported, got %+v", empty)
	}
}

// #endregion 🧪️ProcessRunner
