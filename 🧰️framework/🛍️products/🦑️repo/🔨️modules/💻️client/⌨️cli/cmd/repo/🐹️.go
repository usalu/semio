// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

// Native executable entry point for the repository CLI package.
// #endregion 🧲️Header

package main

import (
	"fmt"
	"os"

	client "github.com/usalu/semio/repo/client"
)

func main() {
	if err := client.RunCLI(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
