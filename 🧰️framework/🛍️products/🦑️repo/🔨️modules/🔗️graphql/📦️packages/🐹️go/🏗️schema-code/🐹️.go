// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏗️schema_code carries the schema types of the code domain.

// #endregion 🧲️Header

package graphql

// #region 🏗️Schema Code

// #region 🏗️SchemaCommon

/** 🔢️ Declares one enum type. */
func enumType(name string, values ...[2]string) NamedType {
	members := make([]EnumValue, 0, len(values))
	for _, value := range values {
		members = append(members, EnumValue{Name: value[0], Wire: value[1]})
	}
	return &EnumType{Name: name, Values: members}
}

/** 🔢️ The enum types the repo schema declares. */
func schemaEnums() []NamedType {
	return []NamedType{
		enumType("DefinitionKind", [2]string{"IMPLEMENTATION", "implementation"}, [2]string{"INTERFACE", "interface"}, [2]string{"CONSTANT", "constant"}),
		enumType("BundleKind", [2]string{"LIBRARY", "library"}, [2]string{"SCHEMA", "schema"}, [2]string{"BINARY", "binary"}, [2]string{"UI", "ui"}, [2]string{"SITE", "site"}, [2]string{"ASSETS", "assets"}, [2]string{"REPO", "repo"}),
		enumType("FolderKind", [2]string{"ORGANIZATION", "organization"}, [2]string{"REQUIRED", "required"}),
		enumType("TicketStatus", [2]string{"OPEN", "open"}, [2]string{"CLOSED", "closed"}),
		enumType("TicketClient",
			[2]string{"COPILOT_CHAT", "copilot-chat"}, [2]string{"WINDSURF", "windsurf"}, [2]string{"WINDSURF_CHAT", "windsurf-chat"},
			[2]string{"ANTIGRAVITY", "antigravity"}, [2]string{"ANTIGRAVITY_CHAT", "antigravity-chat"}, [2]string{"CURSOR", "cursor"},
			[2]string{"CURSOR_CHAT", "cursor-chat"}, [2]string{"VSCODE", "vscode"}, [2]string{"CLAUDE_CODE", "claude-code"},
			[2]string{"CODEX", "codex"}, [2]string{"DROID", "droid"}, [2]string{"KIRO_CLI", "kiro-cli"}),
		enumType("BreachPriority", [2]string{"HIGH", "high"}, [2]string{"MEDIUM", "medium"}, [2]string{"LOW", "low"}),
		enumType("FileKind",
			[2]string{"CODE", "code"}, [2]string{"SCRIPT", "script"}, [2]string{"CONFIG", "config"}, [2]string{"LAB", "lab"},
			[2]string{"DOCS", "docs"}, [2]string{"RESOURCE", "resource"}, [2]string{"TEMPLATE", "template"}, [2]string{"LICENSE", "license"}),
	}
}

/** 🔢️ The metric objects every aggregate shares. */
func schemaMetrics() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Range", Fields: []Field{NewField("start", NN(Ty("Int"))), NewField("end", NN(Ty("Int")))}},
		&ObjectType{Name: "CountMetrics", Fields: []Field{NewField("added", NN(Ty("Int"))), NewField("updated", NN(Ty("Int"))), NewField("removed", NN(Ty("Int")))}},
		&ObjectType{Name: "PriorityCount", Fields: []Field{NewField("high", NN(Ty("Int"))), NewField("medium", NN(Ty("Int"))), NewField("low", NN(Ty("Int")))}},
		&ObjectType{Name: "AnalyzeMetrics", Fields: []Field{NewField("total", NN(Ty("Int"))), NewField("byPriority", Ty("PriorityCount")), NewField("autofixable", NN(Ty("Int")))}},
		&ObjectType{Name: "AnalyzeResult", Fields: []Field{NewField("breachs", NNList(Ty("Breach"))), NewField("metrics", NN(Ty("AnalyzeMetrics")))}},
		&ObjectType{Name: "FixResult", Fields: []Field{NewField("fixed", NN(Ty("Int"))), NewField("remaining", NN(Ty("Int"))), NewField("breachs", NNList(Ty("Breach")))}},
	}
}

// #endregion 🏗️SchemaCommon

// #region 🏗️SchemaRepo

/** 💠️ The repository root type: every aggregate reachable from one query. */
func schemaRepo() []NamedType {
	return []NamedType{&ObjectType{Name: "Repo", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("name", NN(Ty("String"))),
		NewField("path", NN(Ty("String"))),
		ResolvedField("technologies", NNList(Ty("Technology")), "technologies"),
		ResolvedField("checkpoints", NNList(Ty("Checkpoint")), "checkpoints").Arg("limit", Ty("Int")),
		ResolvedField("bundles", NNList(Ty("Bundle")), "bundles"),
		ResolvedField("folders", NNList(Ty("Folder")), "folders"),
		ResolvedField("files", NNList(Ty("File")), "files"),
		ResolvedField("sections", NNList(Ty("Section")), "sections"),
		ResolvedField("definitions", NNList(Ty("Definition")), "definitions"),
		ResolvedField("contributors", NNList(Ty("Contributor")), "contributors"),
		ResolvedField("goals", NNList(Ty("Goal")), "goals"),
		ResolvedField("tickets", NNList(Ty("Ticket")), "tickets").Arg("year", Ty("Int")).Arg("month", Ty("Int")).Arg("day", Ty("Int")).Arg("status", Ty("TicketStatus")),
		ResolvedField("policies", NNList(Ty("Policy")), "policies"),
		ResolvedField("statutes", NNList(Ty("Statute")), "statutes"),
		ResolvedField("breachs", NNList(Ty("Breach")), "breachs").Arg("scope", Ty("String")),
	}}}
}

// #endregion 🏗️SchemaRepo

// #region 🏗️SchemaTechnology

/** 📜️ The technology type and the package it publishes. */
func schemaTechnology() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Technology", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("name", NN(Ty("String"))),
			NewField("root", NN(Ty("String"))),
			NewField("kind", NN(Ty("String"))),
			NewField("bundles", NNList(Ty("Bundle"))),
			NewField("uri", NN(Ty("String"))),
		}},
		&ObjectType{Name: "Package", Fields: []Field{
			NewField("name", NN(Ty("String"))),
			NewField("version", NN(Ty("String"))),
			NewField("path", NN(Ty("String"))),
			NewField("kind", NN(Ty("String"))),
		}},
	}
}

// #endregion 🏗️SchemaTechnology

// #region 🏗️SchemaBundle

/** 🔵️ The bundle type. */
func schemaBundle() []NamedType {
	return []NamedType{&ObjectType{Name: "Bundle", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("name", NN(Ty("String"))),
		NewField("root", NN(Ty("String"))),
		NewField("sourceRoot", Ty("String")),
		NewField("projectType", Ty("String")),
		NewField("tags", NNList(Ty("String"))),
		NewField("packages", NNList(Ty("Package"))),
		NewField("kind", NN(Ty("String"))),
		NewField("uri", NN(Ty("String"))),
		ResolvedField("folders", NNList(Ty("Folder")), "empty-list"),
		ResolvedField("files", NNList(Ty("File")), "empty-list"),
		ResolvedField("breachs", NNList(Ty("Breach")), "empty-list"),
	}}}
}

// #endregion 🏗️SchemaBundle

// #region 🏗️SchemaFolder

/** 🩷️ The folder type. */
func schemaFolder() []NamedType {
	return []NamedType{&ObjectType{Name: "Folder", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("path", NN(Ty("String"))),
		NewField("uri", NN(Ty("String"))),
		NewField("name", NN(Ty("String"))),
		NewField("kind", NN(Ty("String"))),
		NewField("parent", Ty("Folder")),
		ResolvedField("children", NNList(Ty("Folder")), "folder.children"),
		ResolvedField("files", NNList(Ty("File")), "folder.files"),
		NewField("ignored", NN(Ty("Boolean"))),
		NewField("generated", NN(Ty("Boolean"))),
		NewField("bundle", Ty("Bundle")),
		ResolvedField("breachs", NNList(Ty("Breach")), "empty-list"),
	}}}
}

// #endregion 🏗️SchemaFolder

// #region 🏗️SchemaFile

/** 📄️ The file type. */
func schemaFile() []NamedType {
	return []NamedType{&ObjectType{Name: "File", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("path", NN(Ty("String"))),
		NewField("uri", NN(Ty("String"))),
		NewField("name", NN(Ty("String"))),
		NewField("extension", NN(Ty("String"))),
		NewField("folder", Ty("Folder")),
		NewField("kind", NN(Ty("String"))),
		NewField("ignored", NN(Ty("Boolean"))),
		NewField("generated", NN(Ty("Boolean"))),
		NewField("bundle", Ty("Bundle")),
		ResolvedField("sections", Lst(Ty("Section")), "file.sections"),
		ResolvedField("definitions", Lst(Ty("Definition")), "file.definitions"),
		ResolvedField("breachs", Lst(Ty("Breach")), "empty-list"),
		NewField("content", Ty("String")),
		ResolvedField("contributors", NNList(Ty("Contributor")), "empty-list"),
	}}}
}

// #endregion 🏗️SchemaFile

// #region 🏗️SchemaSection

/** 💗️ The section type and the abstract item a section contains. */
func schemaSection() []NamedType {
	return []NamedType{
		&InterfaceType{Name: "SectionItem", Fields: []Field{NewField("id", NN(Ty("ID"))), NewField("name", NN(Ty("String"))), NewField("range", Ty("Range"))}},
		&ObjectType{Name: "Section", Interfaces: []string{"SectionItem"}, Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("name", NN(Ty("String"))),
			NewField("path", NN(Ty("String"))),
			ResolvedField("file", Ty("File"), "item.file"),
			NewField("parent", Ty("Section")),
			ResolvedField("children", Lst(Ty("SectionItem")), "section.children"),
			NewField("definitions", Lst(Ty("Definition"))),
			ResolvedField("breachs", Lst(Ty("Breach")), "empty-list"),
			NewField("range", NN(Ty("Range"))),
		}},
	}
}

// #endregion 🏗️SchemaSection

// #region 🏗️SchemaDefinition

/** 💕️ The definition type. */
func schemaDefinition() []NamedType {
	return []NamedType{&ObjectType{Name: "Definition", Interfaces: []string{"SectionItem"}, Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("name", NN(Ty("String"))),
		NewField("kind", NN(Ty("DefinitionKind"))),
		ResolvedField("file", NN(Ty("File")), "item.file"),
		ResolvedField("section", Ty("Section"), "definition.section"),
		ResolvedField("breachs", NNList(Ty("Breach")), "empty-list"),
		NewField("range", NN(Ty("Range"))),
	}}}
}

// #endregion 🏗️SchemaDefinition

// #endregion 🏗️Schema Code
