// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Native executable entry point for the repository CLI.

// #endregion 🧲️Header

package main

import (
	"fmt"
	"os"

	cli "github.com/usalu/semio/repo/cli"
)

// 🚀️main runs the repository CLI and reports a failure on standard error.
func main() {
	if err := cli.RunCLI(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
