// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package sandbox owns the hermetic harness the Go tests of every repo module run under: a test binary binds a temporary repository it owns instead of the checkout it was started in, a guard fails the run when anything resolves or still holds that checkout, and fixture builders materialise throwaway repositories. It is a nested module of the workspace package, so no test-only code ships in the production package; each member file carries one concern.

// #endregion 🧲️Header

package sandbox
