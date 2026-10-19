// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌐️schema_roots carries the input types and the query and mutation roots of the schema.

// #endregion 🧲️Header

package graphql

// #region 🌐️Schema Roots

// #region 🏗️SchemaInputs

/** 📥️ Declares one input object type. */
func inputType(name string, fields ...InputField) NamedType {
	return &InputObjectType{Name: name, Fields: fields}
}

/** 📥️ Declares one input field. */
func inputField(name string, kind TypeRef) InputField { return InputField{Name: name, Type: kind} }

/** 📥️ Every input object the query and mutation roots accept. */
func schemaInputs() []NamedType {
	return []NamedType{
		inputType("FilterInput",
			inputField("filter", Ty("String")), inputField("regex", Ty("Boolean")), inputField("matchCase", Ty("Boolean")),
			inputField("matchWholeWord", Ty("Boolean")), inputField("showIgnored", Ty("Boolean")), inputField("showGenerated", Ty("Boolean")),
			inputField("excludeKinds", Lst(Ty("FileKind"))), inputField("includeKinds", Lst(Ty("FileKind")))),
		inputType("DraftCreateInput", inputField("title", NN(Ty("String"))), inputField("files", Lst(NN(Ty("String"))))),
		inputType("TicketOpenInput",
			inputField("emoji", NN(Ty("String"))), inputField("title", NN(Ty("String"))), inputField("prompt", NN(Ty("String"))),
			inputField("llm", Ty("String")), inputField("effort", Ty("String")), inputField("client", NN(Ty("TicketClient"))),
			inputField("noIssue", Ty("Boolean")), inputField("draft", Ty("String")), inputField("goal", NN(Ty("String"))),
			inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean")), inputField("issue", Ty("String")),
			inputField("planId", Ty("String")), inputField("specId", Ty("String"))),
		inputType("TicketCloseInput",
			inputField("year", Ty("Int")), inputField("month", Ty("Int")), inputField("day", Ty("Int")), inputField("slug", Ty("String")),
			inputField("summary", Ty("String")), inputField("files", Lst(NN(Ty("String")))), inputField("title", Ty("String")),
			inputField("noManagement", Ty("Boolean")), inputField("all", Ty("Boolean"))),
		inputType("TicketReopenInput",
			inputField("year", NN(Ty("Int"))), inputField("month", NN(Ty("Int"))), inputField("day", NN(Ty("Int"))), inputField("slug", NN(Ty("String"))),
			inputField("prompt", NN(Ty("String"))), inputField("client", NN(Ty("TicketClient"))), inputField("llm", Ty("String")),
			inputField("effort", Ty("String")), inputField("title", Ty("String")), inputField("draft", Ty("String")),
			inputField("goal", Ty("String")), inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean")),
			inputField("planId", Ty("String")), inputField("specId", Ty("String"))),
		inputType("TicketChangeInput",
			inputField("year", NN(Ty("Int"))), inputField("month", NN(Ty("Int"))), inputField("day", NN(Ty("Int"))), inputField("slug", NN(Ty("String"))),
			inputField("title", Ty("String")), inputField("prompt", Ty("String")), inputField("llm", Ty("String")), inputField("effort", Ty("String")),
			inputField("client", Ty("TicketClient")), inputField("goal", Ty("String")), inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean"))),
		inputType("TodoCreateInput", inputField("name", NN(Ty("String"))), inputField("description", NN(Ty("String"))), inputField("parentId", NN(Ty("String")))),
		inputType("TodoChangeInput", inputField("id", NN(Ty("String"))), inputField("name", Ty("String")), inputField("description", Ty("String"))),
		inputType("GoalCreateInput",
			inputField("title", NN(Ty("String"))), inputField("description", NN(Ty("String"))), inputField("prompt", NN(Ty("String"))),
			inputField("dueDate", NN(Ty("String"))), inputField("llm", NN(Ty("String"))), inputField("effort", Ty("String")),
			inputField("client", NN(Ty("String"))), inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean")), inputField("milestone", Ty("String"))),
		inputType("GoalChangeInput",
			inputField("id", NN(Ty("String"))), inputField("title", Ty("String")), inputField("description", Ty("String")),
			inputField("dueDate", Ty("String")), inputField("llm", Ty("String")), inputField("effort", Ty("String")),
			inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean"))),
		inputType("GoalCloseInput", inputField("id", NN(Ty("String"))), inputField("summary", NN(Ty("String"))), inputField("noManagement", Ty("Boolean"))),
		inputType("GoalReopenInput",
			inputField("id", NN(Ty("String"))), inputField("prompt", NN(Ty("String"))), inputField("client", NN(Ty("String"))),
			inputField("llm", NN(Ty("String"))), inputField("effort", Ty("String")), inputField("title", Ty("String")),
			inputField("description", Ty("String")), inputField("dueDate", Ty("String")), inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean"))),
		inputType("ContributorAddInput",
			inputField("github", NN(Ty("String"))), inputField("name", Ty("String")), inputField("names", Lst(NN(Ty("String")))),
			inputField("email", Ty("String")), inputField("emails", Lst(NN(Ty("String")))), inputField("fingerprint", Ty("String")),
			inputField("fingerprints", Lst(NN(Ty("String"))))),
	}
}

// #endregion 🏗️SchemaInputs

// #region 🏗️SchemaRoots

/** 🔍️ The query root and the node union it discriminates. */
func schemaQuery() []NamedType {
	return []NamedType{
		&UnionType{Name: "Node", Types: []string{"Repo", "Bundle", "Folder", "File", "Section", "Definition", "Contributor", "Ticket", "Policy", "Statute", "Breach", "Draft"}},
		&ObjectType{Name: "Query", Fields: []Field{
			ResolvedField("node", NN(Ty("Node")), "query.node").Arg("id", NN(Ty("ID"))),
			ResolvedField("repo", NN(Ty("Repo")), "query.repo"),
			ResolvedField("technologies", NNList(Ty("Technology")), "technologies").Arg("filter", Ty("FilterInput")),
			ResolvedField("bundles", NNList(Ty("Bundle")), "bundles").Arg("filter", Ty("FilterInput")),
			ResolvedField("folders", NNList(Ty("Folder")), "folders"),
			ResolvedField("files", NNList(Ty("File")), "files"),
			ResolvedField("sections", NNList(Ty("Section")), "sections"),
			ResolvedField("definitions", NNList(Ty("Definition")), "definitions"),
			ResolvedField("contributors", NNList(Ty("Contributor")), "contributors").Arg("filter", Ty("FilterInput")),
			ResolvedField("todos", NNList(Ty("Todo")), "query.todos").Arg("filter", Ty("FilterInput")),
			ResolvedField("tickets", NNList(Ty("Ticket")), "tickets").Arg("year", Ty("Int")).Arg("month", Ty("Int")).Arg("day", Ty("Int")).Arg("status", Ty("TicketStatus")).Arg("filter", Ty("FilterInput")),
			ResolvedField("interactions", NNList(Ty("InteractionResource")), "query.interactions"),
			ResolvedField("drafts", NNList(Ty("Draft")), "query.drafts"),
			ResolvedField("policies", NNList(Ty("Policy")), "policies").Arg("filter", Ty("FilterInput")),
			ResolvedField("statutes", NNList(Ty("Statute")), "statutes"),
			ResolvedField("breachs", NNList(Ty("Breach")), "breachs").Arg("scope", Ty("String")),
			ResolvedField("bundle", Ty("Bundle"), "query.bundle").Arg("name", NN(Ty("String"))),
			ResolvedField("folder", Ty("Folder"), "query.folder").Arg("path", NN(Ty("String"))),
			ResolvedField("file", Ty("File"), "query.file").Arg("path", NN(Ty("String"))),
			ResolvedField("section", Ty("Section"), "query.section").Arg("path", NN(Ty("String"))).Arg("sectionPath", NNList(Ty("String"))),
			ResolvedField("definition", Ty("Definition"), "query.definition").Arg("path", NN(Ty("String"))).Arg("name", NN(Ty("String"))),
			ResolvedField("contributor", Ty("Contributor"), "query.contributor").Arg("id", NN(Ty("String"))),
			ResolvedField("ticket", Ty("Ticket"), "query.ticket").Arg("year", NN(Ty("Int"))).Arg("month", NN(Ty("Int"))).Arg("day", NN(Ty("Int"))).Arg("slug", NN(Ty("String"))),
			ResolvedField("policy", Ty("Policy"), "query.policy").Arg("id", NN(Ty("String"))),
			ResolvedField("statute", Ty("Statute"), "query.statute").Arg("id", NN(Ty("String"))),
			ResolvedField("analyze", NN(Ty("AnalyzeResult")), "query.analyze").Arg("scope", Ty("String")),
		}},
	}
}

/** ✍️ The mutation root. */
func schemaMutation() []NamedType {
	return []NamedType{&ObjectType{Name: "Mutation", Fields: []Field{
		ResolvedField("syncManagement", NN(Ty("Boolean")), "mutation.syncManagement"),
		ResolvedField("goalCreate", Ty("Goal"), "mutation.goalCreate").Arg("input", NN(Ty("GoalCreateInput"))),
		ResolvedField("goalChange", Ty("Goal"), "mutation.goalChange").Arg("id", NN(Ty("ID"))).Arg("input", NN(Ty("GoalChangeInput"))),
		ResolvedField("goalClose", Ty("Goal"), "mutation.goalClose").Arg("input", NN(Ty("GoalCloseInput"))),
		ResolvedField("goalReopen", Ty("Goal"), "mutation.goalReopen").Arg("input", NN(Ty("GoalReopenInput"))),
		ResolvedField("draftCreate", Ty("Draft"), "mutation.draftCreate").Arg("input", NN(Ty("DraftCreateInput"))),
		ResolvedField("draftDelete", NN(Ty("Boolean")), "mutation.draftDelete").Arg("id", NN(Ty("String"))),
		ResolvedField("todoCreate", Ty("Todo"), "mutation.todoCreate").Arg("input", NN(Ty("TodoCreateInput"))),
		ResolvedField("todoChange", Ty("Todo"), "mutation.todoChange").Arg("input", NN(Ty("TodoChangeInput"))),
		ResolvedField("todoDelete", Ty("Boolean"), "mutation.todoDelete").Arg("id", NN(Ty("ID"))),
		ResolvedField("ticketOpen", Ty("Ticket"), "mutation.ticketOpen").Arg("input", NN(Ty("TicketOpenInput"))),
		ResolvedField("ticketClose", Ty("Ticket"), "mutation.ticketClose").Arg("input", NN(Ty("TicketCloseInput"))),
		ResolvedField("ticketReopen", Ty("Ticket"), "mutation.ticketReopen").Arg("input", NN(Ty("TicketReopenInput"))),
		ResolvedField("ticketChange", Ty("Ticket"), "mutation.ticketChange").Arg("input", NN(Ty("TicketChangeInput"))),
		ResolvedField("contributorAdd", Ty("Contributor"), "mutation.contributorAdd").Arg("input", NN(Ty("ContributorAddInput"))),
		ResolvedField("contributorRemove", NN(Ty("Boolean")), "mutation.contributorRemove").Arg("github", NN(Ty("String"))),
		ResolvedField("folderCreate", Ty("Folder"), "mutation.folderCreate").Arg("path", NN(Ty("String"))),
		ResolvedField("folderMove", Ty("Folder"), "mutation.folderMove").Arg("src", NN(Ty("String"))).Arg("dst", NN(Ty("String"))),
		ResolvedField("folderDelete", NN(Ty("Boolean")), "mutation.folderDelete").Arg("path", NN(Ty("String"))),
		ResolvedField("fileCreate", Ty("File"), "mutation.fileCreate").Arg("path", NN(Ty("String"))),
		ResolvedField("fileMove", Ty("File"), "mutation.fileMove").Arg("src", NN(Ty("String"))).Arg("dst", NN(Ty("String"))),
		ResolvedField("fileDelete", NN(Ty("Boolean")), "mutation.fileDelete").Arg("path", NN(Ty("String"))),
		ResolvedField("sectionCreate", Ty("Section"), "mutation.sectionCreate").Arg("file", NN(Ty("String"))).Arg("name", NN(Ty("String"))).Arg("parent", Ty("String")),
		ResolvedField("sectionMove", Ty("Section"), "mutation.sectionMove").Arg("file", NN(Ty("String"))).Arg("oldName", NN(Ty("String"))).Arg("newName", NN(Ty("String"))),
		ResolvedField("sectionDelete", NN(Ty("Boolean")), "mutation.sectionDelete").Arg("file", NN(Ty("String"))).Arg("name", NN(Ty("String"))),
		ResolvedField("integrate", Ty("File"), "mutation.integrate").Arg("source", NN(Ty("String"))).Arg("targetSection", NN(Ty("String"))).Arg("targetFile", NN(Ty("String"))).Arg("targetParent", Ty("String")),
		ResolvedField("extract", Ty("File"), "mutation.extract").Arg("sourceFile", NN(Ty("String"))).Arg("sourceSection", NN(Ty("String"))).Arg("targetFile", NN(Ty("String"))),
		ResolvedField("fix", NN(Ty("FixResult")), "mutation.fix").Arg("scope", Ty("String")),
	}}}
}

/** 🗺️ Assembles the executable schema from every per-aggregate fragment. */
func BuildSchema() Schema {
	types := map[string]NamedType{}
	for _, scalar := range Scalars {
		types[scalar] = &ScalarType{Name: scalar}
	}
	fragments := [][]NamedType{
		schemaEnums(), schemaMetrics(), schemaRepo(), schemaTechnology(), schemaBundle(), schemaFolder(), schemaFile(),
		schemaSection(), schemaDefinition(), schemaStatute(), schemaBreach(), schemaPolicy(), schemaInteraction(),
		schemaGoal(), schemaDraft(), schemaTodo(), schemaTicket(), schemaCheckpoint(), schemaContributor(),
		schemaInputs(), schemaQuery(), schemaMutation(),
	}
	for _, fragment := range fragments {
		for _, declared := range fragment {
			types[declared.TypeName()] = declared
		}
	}
	return Schema{Types: types, Query: "Query", Mutation: "Mutation"}
}

/** 🗺️ Builds the schema one resolver serves, registering it as the process-wide instance. */
func buildSchema(resolver *Resolver) (Schema, error) {
	RepoResolverInstance = resolver
	return BuildSchema(), nil
}

// #endregion 🏗️SchemaRoots

// #endregion 🌐️Schema Roots
