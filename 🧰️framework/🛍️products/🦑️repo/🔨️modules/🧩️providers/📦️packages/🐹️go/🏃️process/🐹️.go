// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏃️process carries the process runner seam: system, recorded and transcript runners.

// #endregion 🧲️Header

package providers

import (
	json "encoding/json"
	fmt "fmt"
	exec "os/exec"
	strings "strings"
)

// #region 🏃️ProcessRunner

// 📨️ProcessRequest is one external process invocation; Argv[0] is the program.
type ProcessRequest struct {
	Argv  []string `json:"argv"`
	Cwd   string   `json:"cwd,omitempty"`
	Stdin string   `json:"stdin,omitempty"`
}

// 📬️ProcessOutcome is what a process left behind.
type ProcessOutcome struct {
	Stdout string `json:"stdout"`
	Stderr string `json:"stderr"`
	Status int    `json:"status"`
}

// 🎞️ProcessExchange is one recorded exchange of a process transcript fixture.
type ProcessExchange struct {
	Argv   []string `json:"argv"`
	Stdin  string   `json:"stdin,omitempty"`
	Stdout string   `json:"stdout"`
	Stderr string   `json:"stderr"`
	Status int      `json:"status"`
}

// 🎬️ProcessTranscript is the ordered set of exchanges a recorded runner replays.
type ProcessTranscript struct {
	Exchanges []ProcessExchange `json:"exchanges"`
}

// 🏃️ProcessRunner is the single seam through which a provider touches the operating system.
type ProcessRunner interface {
	Run(request ProcessRequest) ProcessOutcome
	Issued() [][]string
}

// 💻️SystemProcessRunner runs the real thing through os/exec.
type SystemProcessRunner struct {
	issued [][]string
}

// 🆕️NewSystemProcessRunner returns a runner with an empty log.
func NewSystemProcessRunner() *SystemProcessRunner { return &SystemProcessRunner{} }

// ▶️Run executes the request and reports what the process left behind.
func (r *SystemProcessRunner) Run(request ProcessRequest) ProcessOutcome {
	r.issued = append(r.issued, append([]string(nil), request.Argv...))
	if len(request.Argv) == 0 {
		return ProcessOutcome{Stderr: "empty argv", Status: 127}
	}
	cmd := exec.Command(request.Argv[0], request.Argv[1:]...)
	if request.Cwd != "" {
		cmd.Dir = request.Cwd
	}
	if request.Stdin != "" {
		cmd.Stdin = strings.NewReader(request.Stdin)
	}
	var stdout, stderr strings.Builder
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err := cmd.Run()
	status := 0
	if err != nil {
		var exitErr *exec.ExitError
		if ok := asExitError(err, &exitErr); ok {
			status = exitErr.ExitCode()
		} else {
			return ProcessOutcome{Stdout: stdout.String(), Stderr: err.Error(), Status: 127}
		}
	}
	return ProcessOutcome{Stdout: stdout.String(), Stderr: stderr.String(), Status: status}
}

// 📜️Issued lists every argv this runner issued, in order.
func (r *SystemProcessRunner) Issued() [][]string { return r.issued }

func asExitError(err error, target **exec.ExitError) bool {
	exitErr, ok := err.(*exec.ExitError)
	if ok {
		*target = exitErr
	}
	return ok
}

// 🎞️RecordedProcessRunner replays a recorded transcript: the first not-yet-consumed exchange whose
// argv matches wins, so a provider's optional calls may be omitted from the fixture.
type RecordedProcessRunner struct {
	transcript ProcessTranscript
	consumed   []bool
	issued     [][]string
}

// 🆕️NewRecordedProcessRunner wraps a transcript.
func NewRecordedProcessRunner(transcript ProcessTranscript) *RecordedProcessRunner {
	return &RecordedProcessRunner{transcript: transcript, consumed: make([]bool, len(transcript.Exchanges))}
}

// ▶️Run replays the matching exchange, or reports exit 127 when the transcript does not carry it.
func (r *RecordedProcessRunner) Run(request ProcessRequest) ProcessOutcome {
	r.issued = append(r.issued, append([]string(nil), request.Argv...))
	for index, exchange := range r.transcript.Exchanges {
		if r.consumed[index] || !equalArgv(exchange.Argv, request.Argv) {
			continue
		}
		r.consumed[index] = true
		return ProcessOutcome{Stdout: exchange.Stdout, Stderr: exchange.Stderr, Status: exchange.Status}
	}
	return ProcessOutcome{Stderr: fmt.Sprintf("no recorded exchange for %q", request.Argv), Status: 127}
}

// 📜️Issued lists every argv this runner issued, in order.
func (r *RecordedProcessRunner) Issued() [][]string { return r.issued }

func equalArgv(left, right []string) bool {
	if len(left) != len(right) {
		return false
	}
	for index := range left {
		if left[index] != right[index] {
			return false
		}
	}
	return true
}

// 📥️ParseProcessTranscript parses a transcript fixture into the shape a recorded runner replays.
func ParseProcessTranscript(source []byte) (ProcessTranscript, error) {
	var transcript ProcessTranscript
	if err := json.Unmarshal(source, &transcript); err != nil {
		return ProcessTranscript{}, fmt.Errorf("failed to parse process transcript: %w", err)
	}
	return transcript, nil
}

// #endregion 🏃️ProcessRunner
