package statutes

import (
	"github.com/usalu/semio/repo/workspace"
	"github.com/usalu/semio/repo/model"

)

// #region 🖊️Editor Policy

// ⚖️ SystemPolicy rejects editor-owned configuration in the active repository.
func SystemPolicy(ctx *PolicyContext) []model.Breach {
	paths := ctx.filesOverride
	if paths == nil {
		var err error
		paths, err = workspace.IdeConfigurationPaths(ctx.RootDir, ctx.Scope.FilePath)
		if err != nil {
			return []model.Breach{ctx.CreateBreach(err.Error(), model.BreachSystemIdeConfiguration, ctx.Scope.FilePath, 1, 0, "")}
		}
	}
	var breachs []model.Breach
	for _, path := range paths {
		if workspace.IsIdeConfigurationPath(path) {
			breachs = append(breachs, ctx.CreateBreach("Developer commands and configuration belong to the repository dashboard", model.BreachSystemIdeConfiguration, path, 1, 0, ""))
		}
	}
	return ctx.FilterIgnored(breachs)
}

// #endregion 🖊️Editor Policy
