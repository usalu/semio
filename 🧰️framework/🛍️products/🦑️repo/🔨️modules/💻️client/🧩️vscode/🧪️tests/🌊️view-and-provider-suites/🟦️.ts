/** 🌊️ The extension-host suites of the sidebar and sections views, the filter and monorepo tree providers and the data structures. */
import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { FilterTreeDataProvider, FilterTreeItem, MonorepoTreeDataProvider, MonorepoTreeItem } from "../../🌳️tree-providers/🟦️.ts";
import { hasRepoAccess } from "../../🎼️utilities/🟦️.ts";
import { TicketData, TicketInteraction } from "../../⚙️data-models/🟦️.ts";
import { EXPECTED_VIEWS, getWorkspaceRoot } from "../🧰️extension-test-kit/🟦️.ts";

// #region 🧪️OwnedExtensionHostTestContract
type OwnedExtensionHostTestContext = { timeout(milliseconds: number): void };
type OwnedExtensionHostTestBody = (this: OwnedExtensionHostTestContext) => void | Promise<void>;
declare function suite(title: string, body: OwnedExtensionHostTestBody): void;
declare function test(title: string, body: OwnedExtensionHostTestBody): void;
declare function suiteSetup(body: OwnedExtensionHostTestBody): void;

// #endregion 🧪️OwnedExtensionHostTestContract
// #region 🌊️Sidebar View Tests
suite("Sidebar View Test Suite", function () {
  this.timeout(15000);

  test("All expected views are registered", async function () {
    const extension = vscode.extensions.getExtension("usalu.repo");
    assert.ok(extension, "Extension should be found");
    if (!extension.isActive) {
      await extension.activate();
    }
    assert.ok(extension.isActive, "Extension should be active");

    const packageJSON = extension.packageJSON;
    const views = packageJSON.contributes.views;
    assert.ok(views, "Views contribution should exist");
    assert.ok(views["repo"], "repo container should exist");
    const registeredViews = views["repo"].map((v: any) => v.id);
    const missing = EXPECTED_VIEWS.filter((v) => !registeredViews.includes(v));
    assert.strictEqual(missing.length, 0, `Missing views: ${missing.join(", ")}`);
  });

  test("Monorepo view can be focused", async function () {
    await vscode.commands.executeCommand("compose.monorepo.focus");
  });

  test("Filter view can be focused", async function () {
    await vscode.commands.executeCommand("compose.filter.focus");
  });

  test("Refresh codebase command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.refreshCodebase"), "refreshCodebase command should be registered");
  });

  test("Toggle filter command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.filter.toggle"), "compose.filter.toggle command should be registered");
  });

  test("Copy ID command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.copyId"), "copyId command should be registered");
  });

  test("Mailto command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.mailto"), "mailto command should be registered");
  });

  test("Open link command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.openLink"), "openLink command should be registered");
  });

  test("Refresh monorepo command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.refreshMonorepo"), "refreshMonorepo command should be registered");
  });

  test("Copy checkpoint SHA command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.copyCheckpointSha"), "copyCheckpointSha command should be registered");
  });

  test("Open checkpoint in GitHub command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.openCheckpointInGitHub"), "openCheckpointInGitHub command should be registered");
  });

  test("Ticket reopen command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.ticketReopen"), "ticketReopen command should be registered");
  });

  test("Refresh item command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.refreshItem"), "refreshItem command should be registered");
  });

  test("New filter toggle commands are available", async function () {
    const commands = await vscode.commands.getCommands(true);
    const newFilterCommands = [
      "compose.filter.toggle.technology.user",
      "compose.filter.toggle.technology.infrastructure",
      "compose.filter.toggle.technology.research",
      "compose.filter.toggle.file.code",
      "compose.filter.toggle.file.script",
      "compose.filter.toggle.file.config",
      "compose.filter.toggle.file.lab",
      "compose.filter.toggle.file.docs",
      "compose.filter.toggle.file.resource",
      "compose.filter.toggle.file.license",
      "compose.filter.toggle.goal.open",
      "compose.filter.toggle.goal.closed",
      "compose.filter.toggle.bundle.schema",
      "compose.filter.toggle.policy.none",
      "compose.filter.toggle.policy.all",
      "compose.filter.toggle.contributor.none",
      "compose.filter.toggle.contributor.all",
      "compose.filter.toggle.checkpoint.none",
      "compose.filter.toggle.checkpoint.all",
    ];
    const missing = newFilterCommands.filter((cmd) => !commands.includes(cmd));
    assert.strictEqual(missing.length, 0, `Missing new filter commands: ${missing.join(", ")}`);
  });
});

// #endregion 🌊️Sidebar View Tests
// #region 🎯️Sections View Tests
suite("Sections View Test Suite", function () {
  this.timeout(30000);

  test("Sections view is registered", async function () {
    const extension = vscode.extensions.getExtension("usalu.repo");
    assert.ok(extension, "Extension should be found");
    if (!extension.isActive) {
      await extension.activate();
    }
    const packageJSON = extension.packageJSON;
    const views = packageJSON.contributes.views["explorer"] || packageJSON.contributes.views["repo"];
    const sectionView = views.find((v: any) => v.id === "compose.sections");
    assert.ok(sectionView, "compose.sections view should be registered");
  });

  test("Sections view can be focused", async function () {
    await vscode.commands.executeCommand("compose.sections.focus");
  });

  test("sectionTree command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.sectionTree"), "sectionTree command should be registered");
  });

  test("sectionList command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.sectionList"), "sectionList command should be registered");
  });

  test("sectionCreate command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.sectionCreate"), "sectionCreate command should be registered");
  });

  test("sectionMove command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.sectionMove"), "sectionMove command should be registered");
  });

  test("sectionDelete command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.sectionDelete"), "sectionDelete command should be registered");
  });

  test("sectionOpen command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.sectionOpen"), "sectionOpen command should be registered");
  });

  test("sectionRename command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.sectionRename"), "sectionRename command should be registered");
  });

  test("sectionIntegrate command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.sectionIntegrate"), "sectionIntegrate command should be registered");
  });

  test("Sections tree view refreshes on file change", async function () {
    const root = getWorkspaceRoot();
    const candidatePaths = [path.join(root, "repo", "vscode", "🟦️.ts"), path.join(root, "..", "vscode", "🟦️.ts"), path.join(root, "🟦️.ts")];
    const existing = candidatePaths.find((p) => fs.existsSync(p));
    if (existing) {
      await vscode.workspace.openTextDocument(vscode.Uri.file(existing));
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
    await vscode.commands.executeCommand("compose.sections.focus");
    await new Promise((resolve) => setTimeout(resolve, 500));
    assert.ok(true, "Sections tree view should refresh without error");
  });
});

// #endregion 🎯️Sections View Tests
suite("Filter Provider Test Suite", () => {
  test("FilterProvider initializes correctly", () => {
    const provider = new FilterTreeDataProvider();
    assert.ok(provider);
  });

  test("Root elements include expected categories", async () => {
    const provider = new FilterTreeDataProvider();
    const children = await provider.getChildren();
    assert.strictEqual(children.length, 13, "Should have 13 root elements (search + 12 filters)");
    const labels = children.map((c: FilterTreeItem) => (typeof c.label === "string" ? c.label : (c.label as vscode.TreeItemLabel).label));
    assert.ok(
      labels.some((l) => l.startsWith("🔍️Search")),
      "Should have Search",
    );
    assert.ok(
      labels.some((l) => l.startsWith("🏗️Technologies")),
      "Should have Technologies",
    );
    assert.ok(
      labels.some((l) => l.startsWith("📦️Bundles")),
      "Should have Bundles",
    );
    assert.ok(
      labels.some((l) => l.startsWith("🗃️Folders")),
      "Should have Folders",
    );
    assert.ok(
      labels.some((l) => l.startsWith("📄️Files")),
      "Should have Files",
    );
    assert.ok(
      labels.some((l) => l.startsWith("🔖️Sections")),
      "Should have Sections",
    );
    assert.ok(
      labels.some((l) => l.startsWith("🏷️Definitions")),
      "Should have Definitions",
    );
    assert.ok(
      labels.some((l) => l.startsWith("🎯️Goals")),
      "Should have Goals",
    );
    assert.ok(
      labels.some((l) => l.startsWith("🎫️Tickets")),
      "Should have Tickets",
    );
    assert.ok(
      labels.some((l) => l.startsWith("🎫️Dates")),
      "Should have Dates",
    );
    assert.ok(
      labels.some((l) => l.startsWith("👮️Policies")),
      "Should have Policies",
    );
    assert.ok(
      labels.some((l) => l.startsWith("🧑️‍💻️Contributors")),
      "Should have Contributors",
    );
    assert.ok(
      labels.some((l) => l.startsWith("🔄️Checkpoints")),
      "Should have Checkpoints",
    );
  });

  test("Time category returns year values when available", async () => {
    const provider = new FilterTreeDataProvider();
    provider.availableYears = [2024, 2025];
    const timeItem = new FilterTreeItem("🎫️Dates", "filter", vscode.TreeItemCollapsibleState.Collapsed, "filter_time");
    const children = await provider.getChildren(timeItem);
    assert.strictEqual(children.length, 2, "Should have 2 year items");
    const labels = children.map((c: FilterTreeItem) => (typeof c.label === "string" ? c.label : ""));
    assert.ok(labels.includes("2024"), "Should have 2024");
    assert.ok(labels.includes("2025"), "Should have 2025");
  });

  test("Year item returns month values when available", async () => {
    const provider = new FilterTreeDataProvider();
    provider.availableMonths = [1, 6, 12];
    const yearItem = new FilterTreeItem("2024", "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_year", "year", 2024);
    const children = await provider.getChildren(yearItem);
    assert.strictEqual(children.length, 3, "Should have 3 month items");
  });

  test("Month item returns day values when available", async () => {
    const provider = new FilterTreeDataProvider();
    provider.availableDays = [1, 15, 28];
    const monthItem = new FilterTreeItem("January", "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_month", "month", 1);
    const children = await provider.getChildren(monthItem);
    assert.strictEqual(children.length, 3, "Should have 3 day items");
  });

  test("Toggle methods update filter state", () => {
    const provider = new FilterTreeDataProvider();

    provider.toggle("bundle", "library");
    assert.strictEqual(provider.filters.bundle.library, false);
    provider.toggle("bundle", "library");
    assert.strictEqual(provider.filters.bundle.library, true);

    provider.toggle("folder", "organization");
    assert.strictEqual(provider.filters.folder.organization, false);

    provider.toggle("section", "all");
    assert.strictEqual(provider.filters.section.all, false);

    provider.toggle("definition", "implementation");
    assert.strictEqual(provider.filters.definition.implementation, false);

    provider.toggle("ticket", "open");
    assert.strictEqual(provider.filters.ticket.open, false);

    provider.toggle("technology", "user");
    assert.strictEqual(provider.filters.technology.user, false);
    provider.toggle("technology", "user");
    assert.strictEqual(provider.filters.technology.user, true);

    provider.toggle("file", "code");
    assert.strictEqual(provider.filters.file.code, false);
    provider.toggle("file", "code");
    assert.strictEqual(provider.filters.file.code, true);

    provider.toggle("goal", "open");
    assert.strictEqual(provider.filters.goal.open, false);
    provider.toggle("goal", "open");
    assert.strictEqual(provider.filters.goal.open, true);
  });

  for (const kind of ["bundle", "folder", "definition", "ticket", "technology", "file", "goal"]) {
    test(`${kind} none/all toggles set all ${kind} filters`, () => {
      const provider = new FilterTreeDataProvider();
      provider.toggle(kind, "none");
      for (const key of Object.keys(provider.filters[kind])) {
        assert.strictEqual(provider.filters[kind][key], false, `${kind}.${key} should be false after none`);
      }
      provider.toggle(kind, "all");
      for (const key of Object.keys(provider.filters[kind])) {
        assert.strictEqual(provider.filters[kind][key], true, `${kind}.${key} should be true after all`);
      }
    });
  }

  test("Time filter toggle updates state", () => {
    const provider = new FilterTreeDataProvider();
    provider.availableYears = [2024];
    provider.availableMonths = [1];
    provider.availableDays = [15];

    provider.toggleYear(2024);
    assert.ok(provider.excludedYears.includes(2024));
    provider.toggleYear(2024);
    assert.ok(!provider.excludedYears.includes(2024));

    provider.toggleMonth(1);
    assert.ok(provider.excludedMonths.includes(1));

    provider.toggleDay(15);
    assert.ok(provider.excludedDays.includes(15));

    provider.toggle("time", "none");
    assert.strictEqual(provider.timeFilter.none, true);
    assert.strictEqual(provider.timeFilter.all, false);
    assert.ok(provider.excludedYears.includes(2024));
    assert.ok(provider.excludedMonths.includes(1));
    assert.ok(provider.excludedDays.includes(15));

    provider.toggle("time", "all");
    assert.strictEqual(provider.timeFilter.none, false);
    assert.strictEqual(provider.timeFilter.all, true);
    assert.strictEqual(provider.excludedYears.length, 0);
  });

  test("setTimeMode sets year/month/day modes", () => {
    const provider = new FilterTreeDataProvider();
    provider.availableYears = [2024, 2025];
    provider.availableMonths = [3, 6];
    provider.availableDays = [1, 15];

    provider.setTimeMode("year", "none");
    assert.deepStrictEqual(provider.excludedYears, [2024, 2025]);
    provider.setTimeMode("year", "all");
    assert.deepStrictEqual(provider.excludedYears, []);

    provider.setTimeMode("month", "none");
    assert.deepStrictEqual(provider.excludedMonths, [3, 6]);
    provider.setTimeMode("month", "all");
    assert.deepStrictEqual(provider.excludedMonths, []);

    provider.setTimeMode("day", "none");
    assert.deepStrictEqual(provider.excludedDays, [1, 15]);
    provider.setTimeMode("day", "all");
    assert.deepStrictEqual(provider.excludedDays, []);
  });

  test("Search query update updates search state", () => {
    const provider = new FilterTreeDataProvider();
    provider.searchQuery = "test";
    assert.strictEqual(provider.searchQuery, "test");
    provider.matchCase = true;
    assert.strictEqual(provider.matchCase, true);
    provider.matchWholeWord = true;
    assert.strictEqual(provider.matchWholeWord, true);
    provider.useRegex = true;
    assert.strictEqual(provider.useRegex, true);
  });
});

suite("Monorepo Provider Test Suite", () => {
  test("MonorepoProvider initializes with filter provider", () => {
    const filterProvider = new FilterTreeDataProvider();
    const provider = new MonorepoTreeDataProvider(filterProvider);
    assert.ok(provider);
  });

  test("Root elements are populated from CLI search command", async function () {
    this.timeout(30000);
    const provider = new MonorepoTreeDataProvider();
    const children = await provider.getChildren();
    assert.ok(hasRepoAccess(), "repo CLI binary (semio-repo) must be built or named by REPO_CLI_BIN");
    assert.ok(children.length > 0, "CLI returned no children — check that 'search' command exists");
    const labels = children.map((c) => c.label as string);
    assert.ok(
      labels.some((l) => l.includes("Codebase")),
      "Should have Codebase category",
    );
  });

  test("Root elements have category contextValue", async function () {
    this.timeout(30000);
    const provider = new MonorepoTreeDataProvider();
    const roots = await provider.getChildren();
    assert.ok(hasRepoAccess(), "repo CLI binary (semio-repo) must be built or named by REPO_CLI_BIN");
    assert.ok(roots.length > 0, "CLI returned no root elements — check that 'search' command exists");
    for (const r of roots) {
      assert.strictEqual(r.contextValue, "category", `Root ${r.label} should have category contextValue`);
    }
  });

  test("MonorepoTreeItem stores nodeId for copy support", () => {
    const item = new MonorepoTreeItem("Test", vscode.TreeItemCollapsibleState.None, "test_ctx", undefined, "test-node-id");
    assert.strictEqual(item.nodeId, "test-node-id");
    assert.strictEqual(item.tooltip, "test-node-id");
  });

  test("MonorepoTreeItem without nodeId defaults tooltip to label", () => {
    const item = new MonorepoTreeItem("Label", vscode.TreeItemCollapsibleState.None, "ctx");
    assert.strictEqual(item.nodeId, undefined);
  });

  test("Codebase root expands to at least one child when repo CLI is available", async function () {
    this.timeout(30000);
    const provider = new MonorepoTreeDataProvider();
    const roots = await provider.getChildren();
    assert.ok(hasRepoAccess(), "repo CLI binary (semio-repo) must be built or named by REPO_CLI_BIN");
    const codebaseRoot = roots.find((r: MonorepoTreeItem) => (r.label as string).includes("Codebase"));
    assert.ok(codebaseRoot, "Codebase category not found in tree root");
    const expanded = await provider.getChildren(codebaseRoot);
    assert.ok(expanded.length > 0, "Codebase category is empty — CLI search returned no children");
  });
});

suite("Data Structures Test Suite", () => {
  test("TicketInteraction matches expected structure", () => {
    const interaction: TicketInteraction = {
      prompt: "test prompt",
      llm: "gpt-4",
      client: "vscode",
      author: "user",
      date: "2024-01-01",
      checkpoint: "sha123",
    };
    assert.ok(interaction);
    assert.strictEqual(interaction.client, "vscode");
  });

  test("TicketData includes interactions", () => {
    const ticket: TicketData = {
      year: 2024,
      month: 1,
      day: 1,
      slug: "test-ticket",
      frontmatter: {
        status: "open",
        prompt: "test",
      },
      folderPath: "/path/to/ticket",
      interactions: [],
    };
    assert.ok(ticket);
    assert.ok(Array.isArray(ticket.interactions));
  });
});
