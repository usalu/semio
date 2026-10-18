// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏛️schema_governance carries the schema types of the governance domain.

// #endregion 🧲️Header

package graphql

// #region 🏛️Schema Governance

// #region 🏗️SchemaStatute

/** 📜️ The statute type. */
func schemaStatute() []NamedType {
	return []NamedType{&ObjectType{Name: "Statute", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		ResolvedField("policy", NN(Ty("Policy")), "statute.policy"),
		NewField("priority", NN(Ty("BreachPriority"))),
		NewField("autofixable", NN(Ty("Boolean"))),
		NewField("reason", NN(Ty("String"))),
		NewField("solution", NN(Ty("String"))),
		NewField("breachs", NNList(Ty("Breach"))).Arg("scope", Ty("String")),
	}}}
}

// #endregion 🏗️SchemaStatute

// #region 🏗️SchemaBreach

/** 🔶️ The breach type. */
func schemaBreach() []NamedType {
	return []NamedType{&ObjectType{Name: "Breach", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("kindId", NN(Ty("ID"))),
		NewField("kind", NN(Ty("Statute"))),
		NewField("scope", NN(Ty("String"))),
		NewField("file", Ty("File")),
		NewField("folder", Ty("Folder")),
		NewField("line", Ty("Int")),
		NewField("column", Ty("Int")),
		NewField("excerpt", Ty("String")),
		NewField("summary", NN(Ty("String"))),
		NewField("priority", NN(Ty("BreachPriority"))),
		NewField("autofixable", NN(Ty("Boolean"))),
	}}}
}

// #endregion 🏗️SchemaBreach

// #region 🏗️SchemaPolicy

/** 👮️ The policy and territory types. */
func schemaPolicy() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Territory", Fields: []Field{
			NewField("name", NN(Ty("String"))),
			NewField("description", NN(Ty("String"))),
			NewField("scopes", NNList(Ty("String"))),
			NewField("groups", NNList(Ty("Territory"))),
			NewField("kinds", NNList(Ty("Statute"))),
		}},
		&ObjectType{Name: "Policy", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("name", NN(Ty("String"))),
			NewField("description", Ty("String")),
			NewField("scopes", NNList(Ty("String"))),
			NewField("groups", NNList(Ty("Territory"))),
			NewField("statutes", NNList(Ty("Statute"))),
		}},
	}
}

// #endregion 🏗️SchemaPolicy

// #region 🏗️SchemaInteraction

/** 💬️ The interaction types. */
func schemaInteraction() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Interaction", Fields: []Field{
			NewField("kind", NN(Ty("String"))),
			NewField("prompt", NN(Ty("String"))),
			NewField("checkpoint", Ty("String")),
			NewField("llm", Ty("String")),
			NewField("effort", Ty("String")),
			NewField("date", NN(Ty("String"))),
			NewField("system", NN(Ty("String"))),
			NewField("client", NN(Ty("String"))),
			NewField("author", NN(Ty("String"))),
		}},
		&ObjectType{Name: "InteractionResource", Fields: []Field{
			NewField("kind", NN(Ty("String"))),
			NewField("prompt", NN(Ty("String"))),
			NewField("checkpoint", Ty("String")),
			NewField("author", NN(Ty("String"))),
			NewField("sourceKind", NN(Ty("String"))),
			NewField("sourceId", NN(Ty("String"))),
			NewField("goalId", Ty("String")),
			NewField("ticketId", Ty("String")),
			NewField("llm", Ty("String")),
			NewField("effort", Ty("String")),
			NewField("date", NN(Ty("String"))),
			NewField("system", NN(Ty("String"))),
			NewField("client", NN(Ty("String"))),
		}},
	}
}

// #endregion 🏗️SchemaInteraction

// #region 🏗️SchemaGoal

/** 🎯️ The goal type. */
func schemaGoal() []NamedType {
	return []NamedType{&ObjectType{Name: "Goal", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("title", NN(Ty("String"))),
		NewField("description", Ty("String")),
		NewField("prompt", Ty("String")),
		NewField("dueDate", Ty("String")),
		NewField("createdAt", Ty("String")),
		NewField("client", Ty("String")),
		NewField("llm", Ty("String")),
		NewField("effort", Ty("String")),
		NewField("status", NN(Ty("String"))),
		NewField("milestone", Ty("Int")),
		NewField("issue", Ty("String")),
		NewField("parent", Ty("String")),
		NewField("interactions", NNList(Ty("Interaction"))),
	}}}
}

// #endregion 🏗️SchemaGoal

// #region 🏗️SchemaDraft

/** 📝️ The draft type. */
func schemaDraft() []NamedType {
	return []NamedType{&ObjectType{Name: "Draft", Fields: []Field{NewField("id", NN(Ty("ID")))}}}
}

// #endregion 🏗️SchemaDraft

// #region 🏗️SchemaTodo

/** ✅️ The todo type and the position it points at. */
func schemaTodo() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Location", Fields: []Field{NewField("filePath", Ty("String")), NewField("line", Ty("Int")), NewField("column", Ty("Int"))}},
		&ObjectType{Name: "Todo", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("name", NN(Ty("String"))),
			NewField("description", Ty("String")),
			NewField("parentId", NN(Ty("ID"))),
			NewField("location", Ty("Location")),
		}},
	}
}

// #endregion 🏗️SchemaTodo

// #region 🏗️SchemaTicket

/** 🎫️ The ticket type and the date pair it carries. */
func schemaTicket() []NamedType {
	return []NamedType{
		&ObjectType{Name: "TicketDate", Fields: []Field{NewField("started", NN(Ty("DateTime"))), NewField("finished", Ty("DateTime"))}},
		&ObjectType{Name: "Ticket", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("year", NN(Ty("Int"))),
			NewField("month", NN(Ty("Int"))),
			NewField("day", NN(Ty("Int"))),
			NewField("slug", NN(Ty("String"))),
			NewField("path", NN(Ty("String"))),
			NewField("llm", Ty("String")),
			NewField("effort", Ty("String")),
			NewField("client", Ty("TicketClient")),
			NewField("checkpoint", Ty("String")),
			NewField("uri", NN(Ty("String"))),
			NewField("title", NN(Ty("String"))),
			NewField("emoji", Ty("String")),
			NewField("prompt", NN(Ty("String"))),
			NewField("summary", Ty("String")),
			NewField("status", NN(Ty("TicketStatus"))),
			NewField("interactions", NNList(Ty("Interaction"))),
			ResolvedField("author", Ty("Contributor"), "ticket.author"),
			NewField("dates", NN(Ty("TicketDate"))),
			NewField("goal", Ty("String")),
			NewField("parent", Ty("String")),
			NewField("bundles", NNList(Ty("Bundle"))),
			NewField("files", NNList(Ty("File"))),
		}},
		&ObjectType{Name: "TicketDay", Fields: []Field{NewField("day", NN(Ty("Int"))), NewField("tickets", NNList(Ty("Ticket")))}},
		&ObjectType{Name: "TicketMonth", Fields: []Field{NewField("month", NN(Ty("Int"))), NewField("days", NNList(Ty("TicketDay")))}},
		&ObjectType{Name: "TicketYear", Fields: []Field{NewField("year", NN(Ty("Int"))), NewField("months", NNList(Ty("TicketMonth")))}},
	}
}

// #endregion 🏗️SchemaTicket

// #region 🏗️SchemaCheckpoint

/** ✔️ The checkpoint type. */
func schemaCheckpoint() []NamedType {
	return []NamedType{&ObjectType{Name: "Checkpoint", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("sha", NN(Ty("String"))),
		NewField("title", NN(Ty("String"))),
		NewField("date", NN(Ty("DateTime"))),
	}}}
}

// #endregion 🏗️SchemaCheckpoint

// #region 🏗️SchemaContributor

/** 🧑️‍💻️ The contributor type and its contribution tree. */
func schemaContributor() []NamedType {
	named := func(name string, child string, plural string) NamedType {
		return &ObjectType{Name: name, Fields: []Field{NewField("name", NN(Ty("String"))), NewField(plural, NNList(Ty(child)))}}
	}
	return []NamedType{
		&ObjectType{Name: "ContributorIcons", Fields: []Field{NewField("avatar", Ty("String")), NewField("avatarRound", Ty("String")), NewField("github", Ty("String"))}},
		&ObjectType{Name: "ContributorLink", Fields: []Field{NewField("name", NN(Ty("String"))), NewField("url", NN(Ty("String")))}},
		&ObjectType{Name: "ContributorDefinition", Fields: []Field{NewField("name", NN(Ty("String")))}},
		named("ContributorSection", "ContributorDefinition", "definitions"),
		named("ContributorFile", "ContributorSection", "sections"),
		named("ContributorFolder", "ContributorFile", "files"),
		named("ContributorBundle", "ContributorFolder", "folders"),
		&ObjectType{Name: "ContributorContributions", Fields: []Field{
			NewField("checkpoints", NNList(Ty("Checkpoint"))),
			NewField("tickets", NNList(Ty("TicketYear"))),
			NewField("bundles", NNList(Ty("ContributorBundle"))),
		}},
		&ObjectType{Name: "Contributor", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("github", NN(Ty("String"))),
			NewField("emoji", Ty("String")),
			NewField("name", NN(Ty("String"))),
			NewField("names", NNList(Ty("String"))),
			NewField("email", NN(Ty("String"))),
			NewField("emails", NNList(Ty("String"))),
			NewField("fingerprint", Ty("String")),
			NewField("fingerprints", NNList(Ty("String"))),
			NewField("links", NNList(Ty("ContributorLink"))),
			ResolvedField("contributions", Ty("ContributorContributions"), "contributor.contributions"),
			NewField("icons", Ty("ContributorIcons")),
			NewField("bundles", NNList(Ty("Bundle"))),
			NewField("files", NNList(Ty("File"))),
			NewField("tickets", NNList(Ty("Ticket"))),
		}},
	}
}

// #endregion 🏗️SchemaContributor

// #endregion 🏛️Schema Governance
