// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The coordinator binary: nothing but a delegation to the package entry point.

// #endregion 🧲️Header

package main

import coordinator "github.com/usalu/semio/repo/coordinator"

// ▶️main delegates to the coordinator package.
func main() {
	coordinator.Main()
}
