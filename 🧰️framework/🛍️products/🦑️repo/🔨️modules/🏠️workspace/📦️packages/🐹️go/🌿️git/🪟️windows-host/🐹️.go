//go:build windows

package workspace

import (
 "os/exec"
 "syscall"
)

// 🫥️hideConsoleWindow keeps every registered tool, git included, from opening a console window: CREATE_NO_WINDOW and a hidden window on top of any command line set before. See https://learn.microsoft.com/windows/win32/procthread/process-creation-flags
func hideConsoleWindow(command *exec.Cmd) {
 command.SysProcAttr = hiddenAttributes(command.SysProcAttr)
}

// 🫥️hiddenAttributes returns the attributes with the window hidden and CREATE_NO_WINDOW set.
func hiddenAttributes(attributes *syscall.SysProcAttr) *syscall.SysProcAttr {
 if attributes == nil {
  attributes = &syscall.SysProcAttr{}
 }
 attributes.HideWindow = true
 attributes.CreationFlags |= 0x08000000
 return attributes
}

// 🪟️setRegisteredBatchCommandLine bypasses CRT quoting only for the closed batch encoding.
func setRegisteredBatchCommandLine(command *exec.Cmd, fragment string) {
 command.SysProcAttr = &syscall.SysProcAttr{CmdLine: "cmd.exe /d /v:off /s /c " + fragment}
}
