//go:build windows

package cli

import ("errors"; "os"; "os/exec"; "os/signal"; "github.com/usalu/semio/repo/workspace")

// 🪟️ Shares the console and waits for the owned tool's terminal result.
func runNativeScriptProcess(arguments []string) error {
 command := exec.Command("bun", arguments...)
 command.Stdin, command.Stdout, command.Stderr = os.Stdin, os.Stdout, os.Stderr
 command.Env = os.Environ()
 interrupts := make(chan os.Signal, 2); signal.Notify(interrupts, os.Interrupt); defer signal.Stop(interrupts)
 if err := command.Start(); err != nil { return err }
 done := make(chan struct{}); defer close(done)
 go func() { for { select { case received := <-interrupts: _ = command.Process.Signal(received); case <-done: return } } }()
 err := command.Wait(); var exited *exec.ExitError
 if errors.As(err, &exited) { code := exited.ExitCode(); if code < 0 { code = 130 }; return workspace.ExitError{Code: code} }
 return err
}
