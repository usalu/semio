// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Editor surface overlays and system policy.

// #endregion 🧲️Header

package client

import (
	"github.com/usalu/semio/repo/workspace"

)

// ⚖️ SystemPolicy rejects editor-owned configuration in the active repository.
func systemPolicy(ctx *PolicyContext) []Breach {
	paths := ctx.filesOverride
	if paths == nil {
		var err error
		paths, err = workspace.IdeConfigurationPaths(ctx.RootDir, ctx.Scope.FilePath)
		if err != nil {
			return []Breach{ctx.CreateBreach(err.Error(), BreachSystemIdeConfiguration, ctx.Scope.FilePath, 1, 0, "")}
		}
	}
	var breachs []Breach
	for _, path := range paths {
		if workspace.IsIdeConfigurationPath(path) {
			breachs = append(breachs, ctx.CreateBreach("Developer commands and configuration belong to the repository dashboard", BreachSystemIdeConfiguration, path, 1, 0, ""))
		}
	}
	return ctx.FilterIgnored(breachs)
}
