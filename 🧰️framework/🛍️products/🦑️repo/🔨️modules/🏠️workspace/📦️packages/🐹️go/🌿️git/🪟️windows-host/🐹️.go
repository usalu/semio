//go:build windows

package workspace

import (
 "os/exec"
 "syscall"
)

// 🪟️setRegisteredBatchCommandLine bypasses CRT quoting only for the closed batch encoding.
func setRegisteredBatchCommandLine(command *exec.Cmd, fragment string) {
 command.SysProcAttr = &syscall.SysProcAttr{CmdLine: "cmd.exe /d /v:off /s /c " + fragment}
}
