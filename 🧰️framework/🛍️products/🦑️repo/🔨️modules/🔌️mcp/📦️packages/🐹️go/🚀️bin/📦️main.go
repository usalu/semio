// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Native executable entry point for the repository Model Context Protocol server.

// #endregion 🧲️Header

package main

import (
	"context"
	"fmt"
	"os"

	mcp "github.com/usalu/semio/repo/mcp"
)

// 🚀️main serves the repo MCP over standard input and output and reports a failure on standard error.
func main() {
	if err := mcp.RunStdio(context.Background(), os.Args[1:], os.LookupEnv, os.Stdin, os.Stdout); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
