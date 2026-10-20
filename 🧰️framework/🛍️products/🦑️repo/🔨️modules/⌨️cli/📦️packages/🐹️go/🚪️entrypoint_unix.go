//go:build !windows

package cli

import ("os"; "os/exec"; "syscall")

// 🔄️ Retains the native PID, inherited streams and Original process custody.
func runNativeScriptProcess(arguments []string) error {
 binary, err := exec.LookPath("bun"); if err != nil { return err }
 return syscall.Exec(binary, append([]string{binary}, arguments...), os.Environ())
}
