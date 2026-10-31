//go:build !windows

package workspace

import "os/exec"

// 🧭️setRegisteredBatchCommandLine is unreachable from ordinary non-Windows routing.
func setRegisteredBatchCommandLine(command *exec.Cmd, fragment string) {}
