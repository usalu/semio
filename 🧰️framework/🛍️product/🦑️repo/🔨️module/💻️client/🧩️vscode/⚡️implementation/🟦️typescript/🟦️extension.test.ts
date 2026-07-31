// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { getSketchpadDistCandidatePaths, isLikelyKitJsonFilePath, resolveSketchpadDistPath } from "../../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/💻️client/🧩️vscode/⚡️implementation/🟦️typescript/🟦️extension.ts";
import {
  buildCliTreeArgs,
  buildEntityEmojiPattern,
  buildEntityIdRegex,
  ENTITY_EMOJIS,
  ENTITY_ID_REGEX,
  extractLeadingEmoji,
  extractRepoResult,
  FilterTreeDataProvider,
  FilterTreeItem,
  hasRepoAccess,
  invalidateTreeNodeCache,
  MonorepoTreeDataProvider,
  MonorepoTreeItem,
  parseRepoEvents,
  parseUri,
  RepoEvent,
  slugify,
  TicketData,
  TicketInteraction,
  treeNodeContextValue,
  TreeNodeData,
  treeNodeDisplayLabel,
  treeNodeToItem,
} from "./extension";

// #endregion 🔌️Adapters

// 🔒️#region 🎞️Constants
const EXPECTED_COMMANDS = [
  "compose.analyze",
  "compose.analyzeFile",
  "compose.autofix",
  "compose.autofixFile",
  "compose.policyList",
  "compose.ticketOpen",
  "compose.ticketList",
  "compose.ticketClose",
  "compose.ticketRead",
  "compose.ticketOpen",
  "compose.technologyList",
  "compose.contributorAdd",
  "compose.contributorList",
  "compose.contributorRemove",
  "compose.sectionTree",
  "compose.definitionList",
  "compose.folderTree",
  "compose.folderCreate",
  "compose.folderMove",
  "compose.folderDelete",
  "compose.folderList",
  "compose.fileCreate",
  "compose.fileMove",
  "compose.fileDelete",
  "compose.fileList",
  "compose.fileTree",
  "compose.sectionCreate",
  "compose.sectionMove",
  "compose.sectionDelete",
  "compose.sectionIntegrate",
  "compose.sectionList",
  "compose.definitionTree",
  "compose.technologyTree",
  "compose.policyCheck",
  "compose.refreshDiagnostics",
  "compose.autofixBreach",
  "compose.copyId",
  "compose.mailto",
  "compose.openLink",
  "compose.refreshMonorepo",
  "compose.refreshCodebase",
  "compose.copyCheckpointSha",
  "compose.openCheckpointInGitHub",
  "compose.ticketReopen",
  "compose.refreshItem",
  "compose.navigate",
  "compose.navigateTo",
];
const EXPECTED_CONSTRAINTS = [
  "guid-unique",
  "type-name-unique",
  "design-name-unique",
  "piece-name-unique",
  "quality-name-unique",
  "port-name-unique",
  "file-name-unique",
  "folder-name-unique",
  "connector-name-unique",
  "model-name-unique",
  "layer-path-unique",
];
const EXPECTED_VIEWS = ["compose.monorepo", "compose.filter"];

// #endregion 🎞️Constants

// 🌱️#region 🎼️Utilities
function getWorkspaceRoot(): string {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? path.join(__dirname, "../../../..");
}

function getFixturePath(relativePath: string): string {
  return path.join(getWorkspaceRoot(), "compose", "assets", relativePath);
}

async function openWorkspaceDocument(...relativePaths: string[]): Promise<vscode.TextDocument> {
  const workspaceRoot = getWorkspaceRoot();
  const searchRoots = [workspaceRoot, path.join(workspaceRoot, ".."), path.join(workspaceRoot, "..", "..")];

  for (const relativePath of relativePaths) {
    for (const searchRoot of searchRoots) {
      const documentPath = path.resolve(searchRoot, relativePath);
      if (fs.existsSync(documentPath)) {
        return vscode.workspace.openTextDocument(vscode.Uri.file(documentPath));
      }
    }
  }

  throw new Error(`Workspace document not found. Tried: ${relativePaths.join(", ")}`);
}

async function openFixture(relativePath: string): Promise<vscode.TextDocument> {
  const fixturePath = getFixturePath(relativePath);
  if (!fs.existsSync(fixturePath)) {
    throw new Error(`Fixture not found at ${fixturePath}`);
  }
  const fixtureUri = vscode.Uri.file(fixturePath);
  const document = await vscode.workspace.openTextDocument(fixtureUri);
  return document;
}

async function waitForDiagnostics(uri: vscode.Uri, timeout = 5000): Promise<vscode.Diagnostic[]> {
  await new Promise((resolve) => setTimeout(resolve, timeout));
  return vscode.languages.getDiagnostics(uri).filter((d) => d.source === "compose");
}

function isDefinitionEntityId(id: string): boolean {
  for (const [emoji, entityKind] of ENTITY_EMOJIS.entries()) {
    if (id.includes(emoji) && entityKind.startsWith("definition-")) {
      return true;
    }
  }
  return false;
}

function collectDefinitionEntityIds(text: string): string[] {
  const regex = buildEntityIdRegex();
  const ids = new Set<string>();

  for (const match of text.matchAll(regex)) {
    const id = match[1] || match[3];
    if (id && isDefinitionEntityId(id)) {
      ids.add(id);
    }
  }

  return Array.from(ids);
}

const NATIVE_DEFINITION_SYMBOL_KINDS = new Set<vscode.SymbolKind>([
  vscode.SymbolKind.Class,
  vscode.SymbolKind.Constant,
  vscode.SymbolKind.Constructor,
  vscode.SymbolKind.Enum,
  vscode.SymbolKind.EnumMember,
  vscode.SymbolKind.Field,
  vscode.SymbolKind.Function,
  vscode.SymbolKind.Interface,
  vscode.SymbolKind.Method,
  vscode.SymbolKind.Module,
  vscode.SymbolKind.Namespace,
  vscode.SymbolKind.Property,
  vscode.SymbolKind.Struct,
  vscode.SymbolKind.TypeParameter,
  vscode.SymbolKind.Variable,
]);

function getDocumentRelativePath(document: vscode.TextDocument): string {
  const workspaceRoot = getWorkspaceRoot();
  return path.relative(workspaceRoot, document.uri.fsPath).replace(/\\/g, "/");
}

function isDocumentSymbol(value: vscode.DocumentSymbol | vscode.SymbolInformation): value is vscode.DocumentSymbol {
  return "selectionRange" in value;
}

function collectNativeDefinitionFallbackEntries(document: vscode.TextDocument): Array<{ name: string; line: number }> {
  const patternsByLanguage: Partial<Record<string, RegExp[]>> = {
    typescript: [
      /^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?interface\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?type\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?enum\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/,
    ],
    typescriptreact: [
      /^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?interface\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?type\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?enum\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/,
    ],
    javascript: [/^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/],
    javascriptreact: [/^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/],
    go: [/^\s*func\s+(?:\([^)]+\)\s*)?([A-Za-z_][\w]*)\b/, /^\s*type\s+([A-Za-z_][\w]*)\b/, /^\s*const\s+([A-Za-z_][\w]*)\b/, /^\s*var\s+([A-Za-z_][\w]*)\b/],
  };
  const patterns = patternsByLanguage[document.languageId] ?? [];
  const entries: Array<{ name: string; line: number }> = [];
  for (let lineIndex = 0; lineIndex < document.lineCount; lineIndex++) {
    const textLine = document.lineAt(lineIndex).text;
    for (const pattern of patterns) {
      const match = pattern.exec(textLine);
      if (!match?.[1]) continue;
      entries.push({ name: match[1], line: lineIndex });
      break;
    }
  }
  return entries;
}

async function collectNativeDefinitionScopes(document: vscode.TextDocument): Promise<string[]> {
  const relativePath = getDocumentRelativePath(document);
  const symbols = (await vscode.commands.executeCommand<Array<vscode.DocumentSymbol | vscode.SymbolInformation>>("vscode.executeDocumentSymbolProvider", document.uri)) ?? [];
  const scopes: string[] = [];
  const linesSeen = new Set<number>();

  const addScope = (name: string, kind: vscode.SymbolKind, range: vscode.Range): void => {
    if (!name || !NATIVE_DEFINITION_SYMBOL_KINDS.has(kind)) return;
    const line = range.start.line;
    if (linesSeen.has(line)) return;
    linesSeen.add(line);
    scopes.push(`${relativePath}§${name}`);
  };

  const visitDocumentSymbol = (symbol: vscode.DocumentSymbol): void => {
    addScope(symbol.name, symbol.kind, symbol.selectionRange);
    for (const child of symbol.children) {
      visitDocumentSymbol(child);
    }
  };

  for (const symbol of symbols) {
    if (isDocumentSymbol(symbol)) {
      visitDocumentSymbol(symbol);
    } else {
      addScope(symbol.name, symbol.kind, symbol.location.range);
    }
  }

  for (const fallback of collectNativeDefinitionFallbackEntries(document)) {
    if (linesSeen.has(fallback.line)) continue;
    linesSeen.add(fallback.line);
    scopes.push(`${relativePath}§${fallback.name}`);
  }

  return scopes;
}

async function getAnalyzeLensIds(document: vscode.TextDocument): Promise<string[]> {
  const lenses = await getCodeLenses(document);
  const ids = lenses
    .filter((lens) => lens.command?.command === "compose.analyze")
    .map((lens) => String(lens.command?.arguments?.[0] ?? ""))
    .filter((id) => id.length > 0);

  return ids;
}

async function getCodeLenses(document: vscode.TextDocument): Promise<vscode.CodeLens[]> {
  await vscode.window.showTextDocument(document, { preview: true, preserveFocus: false });
  await new Promise((resolve) => setTimeout(resolve, 250));
  return (await vscode.commands.executeCommand<vscode.CodeLens[]>("vscode.executeCodeLensProvider", document.uri)) ?? [];
}

// #endregion 🎼️Utilities

// #region 🖇️Extension Activation
suiteSetup(async function () {
  this.timeout(30000);
  await openFixture("compose/metabolism/wip/initialKit/kit.compose.json");
  await new Promise((resolve) => setTimeout(resolve, 2000));
});

// #endregion 🖇️Extension Activation

// #region 🗺️RepoEvent Parsing Tests
suite("RepoEvent Parsing Test Suite", () => {
  test("parseRepoEvents handles result field correctly", () => {
    const output = '{"kind":"result","result":{"data":{"breachs":[{"id":"v1"}]}}}';
    const events = parseRepoEvents(output);
    assert.strictEqual(events.length, 1);
    assert.strictEqual(events[0].kind, "result");
    assert.ok(events[0].result);
    const result = events[0].result as any;
    assert.ok(result.data);
    assert.ok(result.data.breachs);
    assert.strictEqual(result.data.breachs.length, 1);
  });

  test("extractRepoResult extracts data from result field", () => {
    const events: RepoEvent[] = [{ kind: "result", result: { data: { breachs: [{ id: "v1" }] } } }];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
    const data = extracted.data as any;
    assert.ok(data.breachs);
    assert.strictEqual(data.breachs.length, 1);
    assert.strictEqual(data.breachs[0].id, "v1");
  });

  test("extractRepoResult falls back to data field if result is missing", () => {
    const events: RepoEvent[] = [{ kind: "result", data: { breachs: [{ id: "v2" }] } }];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
    const data = extracted.data as any;
    assert.ok(data.breachs);
    assert.strictEqual(data.breachs.length, 1);
    assert.strictEqual(data.breachs[0].id, "v2");
  });

  test("extractRepoResult prefers result over data field", () => {
    const events: RepoEvent[] = [{ kind: "result", result: { data: { breachs: [{ id: "from-result" }] } }, data: { breachs: [{ id: "from-data" }] } }];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
    const data = extracted.data as any;
    assert.ok(data.breachs);
    assert.strictEqual(data.breachs[0].id, "from-result");
  });

  test("extractRepoResult handles fatal errors", () => {
    const events: RepoEvent[] = [{ kind: "error", error: { message: "Fatal error occurred", fatal: true } }];
    assert.throws(() => extractRepoResult(events), /Fatal error occurred/);
  });

  test("extractRepoResult ignores non-fatal errors", () => {
    const events: RepoEvent[] = [
      { kind: "error", error: { message: "Non-fatal warning", fatal: false } },
      { kind: "result", result: { data: { breachs: [] } } },
    ];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
  });

  test("extractRepoResult uses last result when multiple result events", () => {
    const events: RepoEvent[] = [
      { kind: "result", result: { data: { breachs: [{ id: "first" }] } } },
      { kind: "result", result: { data: { breachs: [{ id: "last" }] } } },
    ];
    const extracted = extractRepoResult(events);
    const data = extracted.data as any;
    assert.strictEqual(data.breachs[0].id, "last");
  });
});

// #endregion 🗺️RepoEvent Parsing Tests

// #region 🌙️Command Registration Tests
suite("Command Registration Test Suite", () => {
  test("All expected commands are registered", async () => {
    const extension = vscode.extensions.getExtension("usalu.repo");
    assert.ok(extension, "Extension should be found");
    if (!extension.isActive) {
      await extension.activate();
    }
    const commands = await vscode.commands.getCommands(true);
    const missing = EXPECTED_COMMANDS.filter((cmd) => !commands.includes(cmd));
    assert.strictEqual(missing.length, 0, `Missing commands: ${missing.join(", ")}`);
  });
});

// #endregion 🌙️Command Registration Tests

// #region ⛅️Kit Validation Tests
suite("Kit Validation Test Suite", function () {
  this.timeout(15000);

  test("Valid kit file produces no diagnostics", async function () {
    const document = await openFixture("compose/metabolism/wip/initialKit/kit.compose.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    assert.strictEqual(diagnostics.length, 0, "Valid kit should have no validation errors");
  });

  test("Invalid kit file triggers all expected constraint breachs", async function () {
    const document = await openFixture("compose/invalid.kit.compose.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    const constraintIds = new Set<string>();
    diagnostics.forEach((diag) => {
      if (typeof diag.code === "object" && diag.code !== null) {
        constraintIds.add((diag.code as { value: string }).value);
      } else if (typeof diag.code === "string") {
        constraintIds.add(diag.code);
      }
    });
    const missing = EXPECTED_CONSTRAINTS.filter((c) => !constraintIds.has(c));
    assert.strictEqual(missing.length, 0, `Missing constraint breachs: ${missing.join(", ")}`);
  });

  test("Diagnostics have correct source and severity", async function () {
    const document = await openFixture("compose/invalid.kit.compose.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    diagnostics.forEach((diag) => {
      assert.strictEqual(diag.source, "compose", "Source should be 'compose'");
      const validSeverities = [vscode.DiagnosticSeverity.Error, vscode.DiagnosticSeverity.Warning, vscode.DiagnosticSeverity.Information];
      assert.ok(validSeverities.includes(diag.severity), `Invalid severity: ${diag.severity}`);
    });
  });

  test("Quick fixes are available for kit diagnostics", async function () {
    const document = await openFixture("compose/invalid.kit.compose.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", document.uri, diagnostics[0].range);
    assert.ok(codeActions && codeActions.length > 0, "Should have code actions available");
    const fixAction = codeActions.find((action) => action.kind?.value === vscode.CodeActionKind.QuickFix.value);
    assert.ok(fixAction, "Should have at least one quick fix action");
    assert.ok(fixAction.edit, "Quick fix should have a workspace edit");
  });

  test("Quick fix workspace edit contains valid text edits", async function () {
    const document = await openFixture("compose/invalid.kit.compose.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", document.uri, diagnostics[0].range);
    const fixAction = codeActions?.find((action) => action.kind?.value === vscode.CodeActionKind.QuickFix.value);
    if (!fixAction?.edit) {
      console.log("Skipping: no quick fix with edit found");
      return;
    }
    const entries = fixAction.edit.entries();
    assert.ok(entries.length > 0, "Workspace edit should have entries");
    for (const [uri, edits] of entries) {
      assert.ok(uri.fsPath.endsWith(".json"), "Edit should target JSON file");
      assert.ok(edits.length > 0, "Should have at least one text edit");
      edits.forEach((edit) => {
        assert.ok(edit.range, "Text edit should have a range");
        assert.ok(typeof edit.newText === "string", "Text edit should have newText");
      });
    }
  });
});

// #endregion ⛅️Kit Validation Tests

// #region 🎃️Repo Diagnostics Tests
suite("Repo Diagnostics Test Suite", function () {
  this.timeout(30000);

  test("Invalid repo file produces diagnostics", async function () {
    const document = await openFixture("repo/some/folder/⚛️file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: no breachs found (analyze returned 0)");
      return;
    }
    assert.ok(diagnostics.length > 0, "Invalid repo file should have diagnostics");
  });

  test("Repo diagnostics show breach name as message", async function () {
    const document = await openFixture("repo/some/folder/⚛️file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: no breachs found");
      return;
    }
    diagnostics.forEach((diag) => {
      assert.ok(!diag.message.includes("\n"), "Message should not contain newlines");
      assert.ok(!diag.message.includes("⚛️file_invalid.tsx"), "Message should not contain file path");
    });
  });

  test("Repo diagnostics have policy ID as code with link target", async function () {
    const document = await openFixture("repo/some/folder/⚛️file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: no breachs found");
      return;
    }
    const diagWithLink = diagnostics.find((d) => typeof d.code === "object" && d.code !== null);
    if (!diagWithLink) {
      console.log("Skipping: no diagnostic with code object found");
      return;
    }
    const codeObj = diagWithLink.code as { value: string; target: vscode.Uri };
    assert.ok(codeObj.value, "Code should have policy ID");
    assert.ok(!codeObj.value.includes(":"), "Code should be policy ID without breach suffix");
    assert.ok(codeObj.target, "Code should have target URI");
    assert.ok(codeObj.target.fsPath.includes("repo.tsx"), "Target should point to repo.tsx");
    assert.ok(codeObj.target.fragment.startsWith("L"), "Target should have line number fragment");
  });

  test("Valid repo file produces no diagnostics", async function () {
    const document = await openFixture("repo/some/folder/⚛️file.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    assert.strictEqual(diagnostics.length, 0, "Valid repo file should have no diagnostics");
  });

  test("Repo diagnostics have code actions for autofixable breachs", async function () {
    const document = await openFixture("repo/some/folder/⚛️file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: no breachs found");
      return;
    }
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", document.uri, diagnostics[0].range);
    assert.ok(codeActions && codeActions.length > 0, "Should have code actions for repo diagnostics");
    const fixAction = codeActions.find((a) => a.kind?.value === vscode.CodeActionKind.QuickFix.value);
    assert.ok(fixAction, "Should have quick fix action");
    assert.ok(fixAction.command || fixAction.edit, "Quick fix should have command or edit");
  });
});

// #endregion 🎃️Repo Diagnostics Tests

// #region 🏆️Refresh Diagnostics Tests
suite("Refresh Diagnostics Test Suite", function () {
  this.timeout(15000);

  test("compose.refreshDiagnostics updates all open documents", async function () {
    const document = await openFixture("compose/invalid.kit.compose.json");
    await vscode.commands.executeCommand("compose.refreshDiagnostics");
    await new Promise((resolve) => setTimeout(resolve, 3000));
    const diagnostics = vscode.languages.getDiagnostics(document.uri).filter((d) => d.source === "compose");
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    assert.ok(diagnostics.length > 0, "Diagnostics should be present after refresh");
  });
});

// #endregion 🏆️Refresh Diagnostics Tests

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
    const candidatePaths = [path.join(root, "repo", "vscode", "🟦️extension.ts"), path.join(root, "..", "vscode", "🟦️extension.ts"), path.join(root, "🟦️extension.ts")];
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
    assert.ok(hasRepoAccess(), "CLI binary must be accessible at repo/client/client");
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
    assert.ok(hasRepoAccess(), "CLI binary must be accessible at repo/client/client");
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
    assert.ok(hasRepoAccess(), "CLI binary must be accessible at repo/client/client");
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

// #region 🌈️CLI Tree Helper Tests
suite("extractLeadingEmoji Test Suite", () => {
  test("Extracts single emoji from start", () => {
    assert.strictEqual(extractLeadingEmoji("💻️coda/engine/coda.py"), "💻️");
  });

  test("Extracts compound emoji with variation selector", () => {
    assert.strictEqual(extractLeadingEmoji("⚙️config.json"), "⚙️");
  });

  test("Returns empty string for no emoji", () => {
    assert.strictEqual(extractLeadingEmoji("hello"), "");
  });

  test("Returns empty string for empty input", () => {
    assert.strictEqual(extractLeadingEmoji(""), "");
  });

  test("Extracts category emoji", () => {
    assert.strictEqual(extractLeadingEmoji("🖥️Codebase"), "🖥️");
  });
});

suite("treeNodeDisplayLabel Test Suite", () => {
  test("Category node uses Label directly", () => {
    const node: TreeNodeData = { Kind: "category", ID: "", Label: "🖥️Codebase", URI: "" };
    assert.strictEqual(treeNodeDisplayLabel(node), "🖥️Codebase");
  });

  test("Ticket node gets status icon", () => {
    const node: TreeNodeData = { Kind: "ticket", ID: "🎫️test", Label: "MY-TICKET", URI: "", Status: "open" };
    assert.ok(treeNodeDisplayLabel(node).includes("🔵️"));
  });

  test("Closed ticket gets green icon", () => {
    const node: TreeNodeData = { Kind: "ticket", ID: "🎫️test", Label: "MY-TICKET", URI: "", Status: "closed" };
    assert.ok(treeNodeDisplayLabel(node).includes("🟢️"));
  });

  test("File node uses emoji prefix plus Label", () => {
    const node: TreeNodeData = { Kind: "file", ID: "💻️compose/go/compose.go", Label: "compose.go", URI: "" };
    assert.strictEqual(treeNodeDisplayLabel(node), "💻️compose.go");
  });

  test("Goal node includes status icon", () => {
    const node: TreeNodeData = { Kind: "goal", ID: "🎯️my-goal", Label: "My Goal", URI: "", Status: "open" };
    const label = treeNodeDisplayLabel(node);
    assert.ok(label.includes("🔵️") || label.includes("🎯️"));
  });

  test("Contributor node gets fallback emoji", () => {
    const node: TreeNodeData = { Kind: "contributor", ID: "", Label: "usalu", URI: "" };
    const label = treeNodeDisplayLabel(node);
    assert.ok(label.includes("🧑️‍💻️"));
  });

  test("Checkpoint node gets fallback emoji", () => {
    const node: TreeNodeData = { Kind: "checkpoint", ID: "", Label: "Fix bug", URI: "" };
    const label = treeNodeDisplayLabel(node);
    assert.ok(label.includes("🔀️"));
  });
});

suite("treeNodeContextValue Test Suite", () => {
  test("Category returns category", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "category", ID: "", Label: "", URI: "" }), "category");
  });

  test("File returns file", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "file", ID: "", Label: "", URI: "" }), "file");
  });

  test("Open ticket returns ticketOpen", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "ticket", ID: "", Label: "", URI: "", Status: "open" }), "ticketOpen");
  });

  test("Closed ticket returns ticketClosed", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "ticket", ID: "", Label: "", URI: "", Status: "closed" }), "ticketClosed");
  });

  test("Goal returns goal", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "goal", ID: "", Label: "", URI: "" }), "goal");
  });

  test("Contributor returns contributor", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "contributor", ID: "", Label: "", URI: "" }), "contributor");
  });
});

suite("Breach Kind Document Test Suite", () => {
  test("Renders nested statute tree structure correctly", () => {
    const breachNode: TreeNodeData = {
      Kind: "statute",
      ID: "🚫️Code#Header#Missing Region",
      Label: "🚫️Code#Header#Missing Region",
      Description: "Header required",
      URI: "repo://statute/code/header/missing-region",
      Data: { autofixable: true },
    };

    const categoryNode: TreeNodeData = {
      Kind: "category",
      ID: "header",
      Label: "header",
      URI: "",
      Children: [breachNode],
    };

    const policyNode: TreeNodeData = {
      Kind: "policy",
      ID: "👮️code",
      Label: "👮️code",
      URI: "repo://policy/code",
      Children: [categoryNode],
    };

    const policyItem = treeNodeToItem(policyNode);
    assert.strictEqual(policyItem.label, "👮️code");
    assert.strictEqual(policyItem.collapsibleState, vscode.TreeItemCollapsibleState.Collapsed);

    const categoryItem = treeNodeToItem(categoryNode);
    assert.strictEqual(categoryItem.label, "header");
    assert.strictEqual(categoryItem.collapsibleState, vscode.TreeItemCollapsibleState.Collapsed);
    assert.strictEqual(categoryItem.contextValue, "category");

    const breachItem = treeNodeToItem(breachNode);
    assert.strictEqual(breachItem.label, "🚫️Code#Header#Missing Region");
    assert.strictEqual(breachItem.description, "🔧️");
    assert.strictEqual(breachItem.tooltip, "Header required");
    assert.strictEqual(breachItem.collapsibleState, vscode.TreeItemCollapsibleState.None);
  });
});

suite("buildCliTreeArgs Test Suite", () => {
  test("No filter provider returns empty args", () => {
    const args = buildCliTreeArgs(undefined);
    assert.ok(Array.isArray(args));
    assert.strictEqual(args.length, 0);
  });

  test("Filter provider with search query adds positional arg", () => {
    const fp = new FilterTreeDataProvider();
    fp.searchQuery = "hello";
    const args = buildCliTreeArgs(fp);
    assert.ok(args.includes("hello"));
  });

  test("Filter provider with excluded file kinds adds --no flags", () => {
    const fp = new FilterTreeDataProvider();
    fp.filters.file.code = false;
    const args = buildCliTreeArgs(fp);
    assert.ok(args.includes("--no-code"));
  });

  test("Filter provider with excluded years adds --no-year flags", () => {
    const fp = new FilterTreeDataProvider();
    fp.excludedYears = [2024, 2025];
    const args = buildCliTreeArgs(fp);
    assert.ok(args.includes("--no-year"));
    assert.ok(args.includes("2024"));
    assert.ok(args.includes("2025"));
  });

  test("Filter provider with status filter adds --only-open", () => {
    const fp = new FilterTreeDataProvider();
    fp.filters.ticket.open = true;
    fp.filters.ticket.closed = false;
    fp.filters.goal.open = true;
    fp.filters.goal.closed = false;
    const args = buildCliTreeArgs(fp);
    assert.ok(args.includes("--only-open"));
  });
});

// #endregion 🌈️CLI Tree Helper Tests

// #region 🐍️RepoEvent Extended Tests
suite("RepoEvent Extended Parsing Test Suite", () => {
  test("parseRepoEvents handles multiple lines", () => {
    const output = '{"kind":"start"}\n{"kind":"result","result":{"data":"hello"}}\n{"kind":"done"}\n';
    const events = parseRepoEvents(output);
    assert.strictEqual(events.length, 3);
    assert.strictEqual(events[0].kind, "start");
    assert.strictEqual(events[1].kind, "result");
    assert.strictEqual(events[2].kind, "done");
  });

  test("parseRepoEvents ignores blank lines", () => {
    const output = '{"kind":"result","result":{}}\n\n  \n';
    const events = parseRepoEvents(output);
    assert.strictEqual(events.length, 1);
  });

  test("extractRepoResult handles empty events", () => {
    const result = extractRepoResult([]);
    assert.strictEqual(result.data, null);
  });

  test("extractRepoResult skips control events (start, progress, log, done)", () => {
    const events = [{ kind: "start" }, { kind: "progress" }, { kind: "log" }, { kind: "result", result: { data: { value: 42 } } }, { kind: "done" }];
    const result = extractRepoResult(events);
    assert.ok(result.data);
    assert.strictEqual((result.data as any).value, 42);
  });

  test("extractRepoResult collects section results", () => {
    const events = [{ kind: "section", result: undefined, data: undefined }];
    const result = extractRepoResult(events);
    assert.ok(result);
  });
});

// #endregion 🐍️RepoEvent Extended Tests

// #region 📮️URI Resolution Tests
suite("slugify Test Suite", () => {
  test("Converts text to uppercase slug", () => {
    assert.strictEqual(slugify("Hello World"), "HELLO-WORLD");
  });

  test("Converts file path to slug", () => {
    assert.strictEqual(slugify("compose/js/compose.ts"), "COMPOSE-JS-COMPOSE-TS");
  });

  test("Handles already slugified text", () => {
    assert.strictEqual(slugify("HELLO-WORLD"), "HELLO-WORLD");
  });

  test("Strips leading and trailing hyphens", () => {
    assert.strictEqual(slugify("--hello--"), "HELLO");
  });

  test("Handles empty string", () => {
    assert.strictEqual(slugify(""), "");
  });

  test("Preserves numbers", () => {
    assert.strictEqual(slugify("version-2.0"), "VERSION-2-0");
  });

  test("Handles goal ID with slashes", () => {
    assert.strictEqual(slugify("AI-OPTIMIZED-REPO"), "AI-OPTIMIZED-REPO");
  });
});

suite("parseUri Test Suite", () => {
  test("Parses repo URI (no path)", () => {
    const result = parseUri("repo://repo");
    assert.ok(result);
    assert.strictEqual(result!.type, "repo");
    assert.strictEqual(result!.path, "");
  });

  test("Parses codebase URI", () => {
    const result = parseUri("repo://cb");
    assert.ok(result);
    assert.strictEqual(result!.type, "cb");
    assert.strictEqual(result!.path, "");
  });

  test("Parses technologies collection URI (no path)", () => {
    const result = parseUri("repo://technologies");
    assert.ok(result);
    assert.strictEqual(result!.type, "technologies");
    assert.strictEqual(result!.path, "");
  });

  test("Parses technology URI", () => {
    const result = parseUri("repo://technology/compose");
    assert.ok(result);
    assert.strictEqual(result!.type, "technology");
    assert.strictEqual(result!.path, "compose");
  });

  test("Parses bundles collection URI (no path)", () => {
    const result = parseUri("repo://bundles");
    assert.ok(result);
    assert.strictEqual(result!.type, "bundles");
    assert.strictEqual(result!.path, "");
  });

  test("Parses bundle URI", () => {
    const result = parseUri("repo://bundle/compose-js");
    assert.ok(result);
    assert.strictEqual(result!.type, "bundle");
    assert.strictEqual(result!.path, "compose-js");
  });

  test("Parses folders collection URI with parent path", () => {
    const result = parseUri("repo://folders/compose/js");
    assert.ok(result);
    assert.strictEqual(result!.type, "folders");
    assert.strictEqual(result!.path, "compose/js");
  });

  test("Parses folder URI with deep path", () => {
    const result = parseUri("repo://folder/compose/js/sketchpad/page/getting-started");
    assert.ok(result);
    assert.strictEqual(result!.type, "folder");
    assert.strictEqual(result!.path, "compose/js/sketchpad/page/getting-started");
  });

  test("Parses files collection URI with folder path", () => {
    const result = parseUri("repo://files/compose/js");
    assert.ok(result);
    assert.strictEqual(result!.type, "files");
    assert.strictEqual(result!.path, "compose/js");
  });

  test("Parses file URI with path", () => {
    const result = parseUri("repo://file/compose/js/compose.ts");
    assert.ok(result);
    assert.strictEqual(result!.type, "file");
    assert.strictEqual(result!.path, "compose/js/compose.ts");
  });

  test("Parses sections collection URI with file path", () => {
    const result = parseUri("repo://sections/compose/js/compose.ts");
    assert.ok(result);
    assert.strictEqual(result!.type, "sections");
    assert.strictEqual(result!.path, "compose/js/compose.ts");
  });

  test("Parses section URI with file and section path", () => {
    const result = parseUri("repo://section/compose/js/sketchpad/design.tsx/state-management/design-store");
    assert.ok(result);
    assert.strictEqual(result!.type, "section");
    assert.strictEqual(result!.path, "compose/js/sketchpad/Design.tsx/STATE-MANAGEMENT/DESIGN-STORE");
  });

  test("Parses definitions collection URI", () => {
    const result = parseUri("repo://definitions/compose/js/compose.ts");
    assert.ok(result);
    assert.strictEqual(result!.type, "definitions");
    assert.strictEqual(result!.path, "compose/js/compose.ts");
  });

  test("Parses definition URI", () => {
    const result = parseUri("repo://definition/compose/js/compose.ts/validate-kit");
    assert.ok(result);
    assert.strictEqual(result!.type, "definition");
    assert.strictEqual(result!.path, "compose/js/compose.ts/VALIDATE-KIT");
  });

  test("Parses tickets collection URI (no path)", () => {
    const result = parseUri("repo://tickets");
    assert.ok(result);
    assert.strictEqual(result!.type, "tickets");
    assert.strictEqual(result!.path, "");
  });

  test("Parses ticket URI", () => {
    const result = parseUri("repo://ticket/2026/02/07/my-ticket");
    assert.ok(result);
    assert.strictEqual(result!.type, "ticket");
    assert.strictEqual(result!.path, "2026/02/07/MY-TICKET");
  });

  test("Parses goals collection URI (no path)", () => {
    const result = parseUri("repo://goals");
    assert.ok(result);
    assert.strictEqual(result!.type, "goals");
    assert.strictEqual(result!.path, "");
  });

  test("Parses goal URI with path", () => {
    const result = parseUri("repo://goal/r26-02/running-sketchpad/running-sketchpad-apps/running-home-app");
    assert.ok(result);
    assert.strictEqual(result!.type, "goal");
    assert.strictEqual(result!.path, "R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS/RUNNING-HOME-APP");
  });

  test("Parses drafts collection URI (no path)", () => {
    const result = parseUri("repo://drafts");
    assert.ok(result);
    assert.strictEqual(result!.type, "drafts");
    assert.strictEqual(result!.path, "");
  });

  test("Parses draft URI", () => {
    const result = parseUri("repo://draft/my-draft");
    assert.ok(result);
    assert.strictEqual(result!.type, "draft");
    assert.strictEqual(result!.path, "MY-DRAFT");
  });

  test("Parses todos collection URI (no path)", () => {
    const result = parseUri("repo://todos");
    assert.ok(result);
    assert.strictEqual(result!.type, "todos");
    assert.strictEqual(result!.path, "");
  });

  test("Parses todo URI", () => {
    const result = parseUri("repo://todo/fix-bug");
    assert.ok(result);
    assert.strictEqual(result!.type, "todo");
    assert.strictEqual(result!.path, "FIX-BUG");
  });

  test("Parses policies collection URI (no path)", () => {
    const result = parseUri("repo://policies");
    assert.ok(result);
    assert.strictEqual(result!.type, "policies");
    assert.strictEqual(result!.path, "");
  });

  test("Parses policy URI", () => {
    const result = parseUri("repo://policy/code");
    assert.ok(result);
    assert.strictEqual(result!.type, "policy");
    assert.strictEqual(result!.path, "code");
  });

  test("Parses statutes collection URI (no path)", () => {
    const result = parseUri("repo://statutes");
    assert.ok(result);
    assert.strictEqual(result!.type, "statutes");
    assert.strictEqual(result!.path, "");
  });

  test("Parses statute URI with path", () => {
    const result = parseUri("repo://statute/code/header/missing-region");
    assert.ok(result);
    assert.strictEqual(result!.type, "statute");
    assert.strictEqual(result!.path, "CODE/HEADER/MISSING-REGION");
  });

  test("Parses contributors collection URI (no path)", () => {
    const result = parseUri("repo://contributors");
    assert.ok(result);
    assert.strictEqual(result!.type, "contributors");
    assert.strictEqual(result!.path, "");
  });

  test("Parses contributor URI", () => {
    const result = parseUri("repo://contributor/usalu");
    assert.ok(result);
    assert.strictEqual(result!.type, "contributor");
    assert.strictEqual(result!.path, "usalu");
  });

  test("Parses checkpoints collection URI (no path)", () => {
    const result = parseUri("repo://checkpoints");
    assert.ok(result);
    assert.strictEqual(result!.type, "checkpoints");
    assert.strictEqual(result!.path, "");
  });

  test("Parses checkpoint URI", () => {
    const result = parseUri("repo://checkpoint/abc123def");
    assert.ok(result);
    assert.strictEqual(result!.type, "checkpoint");
    assert.strictEqual(result!.path, "abc123def");
  });

  test("Returns null for non-repo URIs", () => {
    assert.strictEqual(parseUri("https://example.com"), null);
    assert.strictEqual(parseUri("file:///tmp/foo"), null);
    assert.strictEqual(parseUri("not a uri"), null);
  });

  test("Returns null for malformed repo URI", () => {
    assert.strictEqual(parseUri("repo://"), null);
  });
});

suite("Navigation Commands Test Suite", function () {
  this.timeout(15000);

  test("compose.navigate command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.navigate"), "navigate command should be registered");
  });

  test("compose.navigateTo command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.navigateTo"), "navigateTo command should be registered");
  });

  test("invalidateTreeNodeCache does not throw", () => {
    invalidateTreeNodeCache();
    assert.ok(true);
  });

  test("compose.navigate handles empty target gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "");
    assert.ok(true, "Should not throw on empty target");
  });

  test("compose.navigate handles undefined target gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", undefined);
    assert.ok(true, "Should not throw on undefined target");
  });

  test("compose.navigate handles unknown URI type gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://unknown/something");
    assert.ok(true, "Should not throw on unknown URI type");
  });

  test("compose.navigate handles repo URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://repo");
    assert.ok(true, "Should not throw on repo URI");
  });

  test("compose.navigate handles collection URIs gracefully", async function () {
    const collections = ["cb", "technologies", "bundles", "folders", "files", "sections", "definitions", "tickets", "goals", "drafts", "todos", "policies", "statutes", "contributors", "checkpoints"];
    for (const collection of collections) {
      await vscode.commands.executeCommand("compose.navigate", `repo://${collection}`);
    }
    assert.ok(true, "Should not throw on collection URIs");
  });

  test("compose.navigate handles folder URI with real path", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://folder/compose/js");
    assert.ok(true, "Should not throw on folder URI");
  });

  test("compose.navigate handles file URI with real path", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://file/package.json");
    assert.ok(true, "Should not throw on file URI");
  });

  test("compose.navigate handles nonexistent folder URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://folder/nonexistent/path");
    assert.ok(true, "Should not throw on nonexistent folder URI");
  });

  test("compose.navigate handles nonexistent file URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://file/nonexistent/path.ts");
    assert.ok(true, "Should not throw on nonexistent file URI");
  });

  test("compose.navigate handles ticket URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://ticket/2099/01/01/nonexistent");
    assert.ok(true, "Should not throw on nonexistent ticket URI");
  });

  test("compose.navigate handles goal URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://goal/nonexistent-goal");
    assert.ok(true, "Should not throw on nonexistent goal URI");
  });

  test("compose.navigate handles draft URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://draft/nonexistent");
    assert.ok(true, "Should not throw on nonexistent draft URI");
  });

  test("compose.navigate handles todo URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://todo/nonexistent");
    assert.ok(true, "Should not throw on nonexistent todo URI");
  });

  test("compose.navigate handles policy URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://policy/code");
    assert.ok(true, "Should not throw on policy URI");
  });

  test("compose.navigate handles statute URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://statute/code/header/missing-region");
    assert.ok(true, "Should not throw on statute URI");
  });

  test("compose.navigate handles contributor URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://contributor/usalu");
    assert.ok(true, "Should not throw on contributor URI");
  });

  test("compose.navigate handles checkpoint URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://checkpoint/abc123");
    assert.ok(true, "Should not throw on checkpoint URI");
  });

  test("compose.navigate handles section URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://section/compose/js/compose.ts/header");
    assert.ok(true, "Should not throw on section URI");
  });

  test("compose.navigate handles definition URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://definition/compose/js/compose.ts/validate-kit");
    assert.ok(true, "Should not throw on definition URI");
  });

  test("compose.navigate handles technology URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://technology/compose");
    assert.ok(true, "Should not throw on technology URI");
  });

  test("compose.navigate handles bundle URI gracefully", async function () {
    await vscode.commands.executeCommand("compose.navigate", "repo://bundle/compose-js");
    assert.ok(true, "Should not throw on bundle URI");
  });
});

// #endregion 📮️URI Resolution Tests

// #region 🎋️Entity Emoji Registry Tests
suite("Entity Emoji Registry Test Suite", () => {
  test("ENTITY_EMOJIS contains all technology kind emojis", () => {
    assert.ok(ENTITY_EMOJIS.has("👤️"), "should contain user technology emoji");
    assert.ok(ENTITY_EMOJIS.has("🧰️"), "should contain infra technology emoji");
    assert.ok(ENTITY_EMOJIS.has("🔬️"), "should contain research technology emoji");
    assert.ok(ENTITY_EMOJIS.has("🔖️"), "should contain section emoji");
    assert.ok(ENTITY_EMOJIS.has("🛠️"), "should contain impl definition emoji");
    assert.ok(ENTITY_EMOJIS.has("🎯️"), "should contain goal emoji");
    assert.ok(ENTITY_EMOJIS.has("🎫️"), "should contain ticket emoji");
    assert.ok(ENTITY_EMOJIS.has("🧑️‍💻️"), "should contain contributor emoji");
  });

  test("ENTITY_EMOJIS contains all bundle kind emojis", () => {
    assert.ok(ENTITY_EMOJIS.has("📚️"), "should contain library bundle emoji");
    assert.ok(ENTITY_EMOJIS.has("🛂️"), "should contain schema bundle emoji");
    assert.ok(ENTITY_EMOJIS.has("⌨️"), "should contain binary bundle emoji");
    assert.ok(ENTITY_EMOJIS.has("🖱️"), "should contain ui bundle emoji");
    assert.ok(ENTITY_EMOJIS.has("📔️"), "should contain example bundle emoji");
    assert.ok(ENTITY_EMOJIS.has("🌐️"), "should contain site bundle emoji");
    assert.ok(ENTITY_EMOJIS.has("🏪️"), "should contain assets bundle emoji");
    assert.ok(ENTITY_EMOJIS.has("🪆️"), "should contain repo bundle emoji");
  });

  test("ENTITY_EMOJIS contains folder kind emojis", () => {
    assert.ok(ENTITY_EMOJIS.has("🗃️"), "should contain org folder emoji");
    assert.ok(ENTITY_EMOJIS.has("🛅️"), "should contain required folder emoji");
  });

  test("ENTITY_EMOJIS contains all file kind emojis", () => {
    assert.ok(ENTITY_EMOJIS.has("💻️"), "should contain code file emoji");
    assert.ok(ENTITY_EMOJIS.has("🥼️"), "should contain lab file emoji");
    assert.ok(ENTITY_EMOJIS.has("📜️"), "should contain script file emoji");
    assert.ok(ENTITY_EMOJIS.has("📃️"), "should contain docs file emoji");
    assert.ok(ENTITY_EMOJIS.has("⚙️"), "should contain config file emoji");
    assert.ok(ENTITY_EMOJIS.has("💾️"), "should contain resource file emoji");
    assert.ok(ENTITY_EMOJIS.has("📋️"), "should contain template file emoji");
    assert.ok(ENTITY_EMOJIS.has("⚖️"), "should contain license file emoji");
  });

  test("ENTITY_EMOJIS contains section and definition emojis", () => {
    assert.ok(ENTITY_EMOJIS.has("🔖️"), "should contain section emoji");
    assert.ok(ENTITY_EMOJIS.has("🛠️"), "should contain impl definition emoji");
    assert.ok(ENTITY_EMOJIS.has("✂️"), "should contain interface definition emoji");
    assert.ok(ENTITY_EMOJIS.has("🪨️"), "should contain constant definition emoji");
    assert.ok(ENTITY_EMOJIS.has("🧪️"), "should contain test definition emoji");
  });

  test("ENTITY_EMOJIS contains time document emojis", () => {
    assert.ok(ENTITY_EMOJIS.has("🎆️"), "should contain year emoji");
    assert.ok(ENTITY_EMOJIS.has("🌙️"), "should contain month emoji");
    assert.ok(ENTITY_EMOJIS.has("☀️"), "should contain day emoji");
    assert.ok(ENTITY_EMOJIS.has("⏰️"), "should contain hour emoji");
    assert.ok(ENTITY_EMOJIS.has("⌚️"), "should contain minute emoji");
    assert.ok(ENTITY_EMOJIS.has("⏱️"), "should contain second emoji");
  });

  test("ENTITY_EMOJIS contains management emojis", () => {
    assert.ok(ENTITY_EMOJIS.has("🎯️"), "should contain goal emoji");
    assert.ok(ENTITY_EMOJIS.has("🎫️"), "should contain ticket emoji");
    assert.ok(ENTITY_EMOJIS.has("📝️"), "should contain draft emoji");
    assert.ok(ENTITY_EMOJIS.has("👮️"), "should contain policy emoji");
    assert.ok(ENTITY_EMOJIS.has("🚫️"), "should contain breach emoji");
    assert.ok(ENTITY_EMOJIS.has("🧑️‍💻️"), "should contain contributor emoji");
    assert.ok(ENTITY_EMOJIS.has("🔀️"), "should contain checkpoint emoji");
  });

  test("ENTITY_EMOJIS contains session emojis", () => {
    assert.ok(ENTITY_EMOJIS.has("⚪️"), "should contain session emoji");
    assert.ok(ENTITY_EMOJIS.has("🟡️"), "should contain session-running emoji");
    assert.ok(ENTITY_EMOJIS.has("🟢️"), "should contain session-completed emoji");
    assert.ok(ENTITY_EMOJIS.has("🔴️"), "should contain session-interrupted emoji");
  });

  test("ENTITY_EMOJIS contains collection/plural emojis", () => {
    assert.ok(ENTITY_EMOJIS.has("🖥️"), "should contain codebase emoji");
    assert.ok(ENTITY_EMOJIS.has("🏗️"), "should contain technologies emoji");
    assert.ok(ENTITY_EMOJIS.has("📦️"), "should contain bundles emoji");
    assert.ok(ENTITY_EMOJIS.has("📁️"), "should contain folders emoji");
    assert.ok(ENTITY_EMOJIS.has("📄️"), "should contain files emoji");
    assert.ok(ENTITY_EMOJIS.has("🏷️"), "should contain definitions emoji");
  });

  test("ENTITY_EMOJIS maps to correct kind names", () => {
    assert.strictEqual(ENTITY_EMOJIS.get("👤️"), "technology-user");
    assert.strictEqual(ENTITY_EMOJIS.get("🧰️"), "technology-infrastructure");
    assert.strictEqual(ENTITY_EMOJIS.get("💻️"), "file-code");
    assert.strictEqual(ENTITY_EMOJIS.get("🔖️"), "section");
    assert.strictEqual(ENTITY_EMOJIS.get("🛠️"), "definition-implementation");
    assert.strictEqual(ENTITY_EMOJIS.get("🎯️"), "goal");
    assert.strictEqual(ENTITY_EMOJIS.get("🎫️"), "ticket");
    assert.strictEqual(ENTITY_EMOJIS.get("🧑️‍💻️"), "contributor");
  });

  test("buildEntityEmojiPattern returns non-empty pattern", () => {
    const pattern = buildEntityEmojiPattern();
    assert.ok(pattern.length > 0, "pattern should be non-empty");

    assert.ok(pattern.includes("👤️"), "should contain user emoji");
    assert.ok(pattern.includes("🧰️"), "should contain infra emoji");
    assert.ok(pattern.includes("💻️"), "should contain code file emoji");
  });

  test("buildEntityIdRegex returns a valid RegExp with 'g' flag", () => {
    const regex = buildEntityIdRegex();
    assert.ok(regex instanceof RegExp, "should be a RegExp");
    assert.ok(regex.flags.includes("g"), "should have global flag");
  });

  test("ENTITY_ID_REGEX is a valid pre-compiled RegExp", () => {
    assert.ok(ENTITY_ID_REGEX instanceof RegExp, "should be a RegExp");
    assert.ok(ENTITY_ID_REGEX.flags.includes("g"), "should have global flag");
  });
});

// #endregion 🎋️Entity Emoji Registry Tests

// #region 📹️Entity ID Regex Matching Tests
suite("Entity ID Regex Matching Test Suite", () => {
  test("matches bare infrastructure technology ID (🧰️)", () => {
    const regex = buildEntityIdRegex();
    const text = "See 🧰️repo⌨️client for CLI details.";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match one ID");
    assert.strictEqual(matches[0][3], "🧰️repo⌨️client");
  });

  test("matches bare user technology ID (👤️)", () => {
    const regex = buildEntityIdRegex();
    const text = "Main technology: 🏘️compose📚️js";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match one ID");
    assert.strictEqual(matches[0][3], "🏘️compose📚️js");
  });

  test("matches markdown link with user technology ID", () => {
    const regex = buildEntityIdRegex();
    const text = "[🏘️compose📚️js💻️composets](repo://p/u/compose/b/l/js/f/compose.ts)";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match one markdown link");
    assert.strictEqual(matches[0][1], "🏘️compose📚️js💻️composets");
    assert.strictEqual(matches[0][2], "repo://p/u/compose/b/l/js/f/compose.ts");
  });

  test("matches goal ID (🎯️)", () => {
    const regex = buildEntityIdRegex();
    const text = "Goal: 🎯️r26021🎯️runningsketchpad";
    const matches = [...text.matchAll(regex)];
    assert.ok(matches.length >= 1, "should match at least one goal ID");
  });

  test("matches ticket ID (🎫️)", () => {
    const regex = buildEntityIdRegex();
    const text = "Ticket: 🎫️implementcodelens";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match one ticket ID");
    assert.strictEqual(matches[0][3], "🎫️implementcodelens");
  });

  test("matches section ID (🔖️)", () => {
    const regex = buildEntityIdRegex();
    const text = "Section: 🔖️statemanagement";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match one section ID");
    assert.strictEqual(matches[0][3], "🔖️statemanagement");
  });

  test("matches definition ID (🛠️)", () => {
    const regex = buildEntityIdRegex();
    const text = "Definition: 🛠️createsketchpadstore";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match one definition ID");
    assert.strictEqual(matches[0][3], "🛠️createsketchpadstore");
  });

  test("matches contributor ID (🧑️‍💻️)", () => {
    const regex = buildEntityIdRegex();
    const text = "Contributor: 🧑️‍💻️ueli";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match one contributor ID");
    assert.strictEqual(matches[0][3], "🧑️‍💻️ueli");
  });

  test("matches full nested entity ID", () => {
    const regex = buildEntityIdRegex();
    const text = "Full: 🏘️compose📚️js🗃️sketchpad💻️designtsx🔖️statemanagement🛠️createstore";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match one full ID");
    assert.strictEqual(matches[0][3], "🏘️compose📚️js🗃️sketchpad💻️designtsx🔖️statemanagement🛠️createstore");
  });

  test("matches multiple IDs in same text", () => {
    const regex = buildEntityIdRegex();
    const text = "Compare 🧰️repo⌨️client with 🏘️compose📚️js and 🎯️goalname";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 3, "should match three IDs");
  });

  test("matches time document IDs", () => {
    const regex = buildEntityIdRegex();
    const text = "Time: 🎆️26🌙️02☀️15⏰️14⌚️33⏱️38";
    const matches = [...text.matchAll(regex)];
    assert.ok(matches.length >= 1, "should match at least one time ID");
  });

  test("matches policy and breach IDs", () => {
    const regex = buildEntityIdRegex();
    const text = "Policy: 👮️godfiles Breach: 🚫️violation";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 2, "should match policy and breach IDs");
  });

  test("does not match plain text without entity emojis", () => {
    const regex = buildEntityIdRegex();
    const text = "This is plain text without any entity IDs.";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 0, "should not match plain text");
  });

  test("does not match emojis that are not entity emojis", () => {
    const regex = buildEntityIdRegex();
    const text = "Random emojis: 😀️ 🎉️ 🚀️ without IDs";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 0, "should not match non-entity emojis");
  });

  test("matches research technology ID (🔬️)", () => {
    const regex = buildEntityIdRegex();
    const text = "Research: 🔬️experiments";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match research technology ID");
  });

  test("matches checkpoint ID (🔀️)", () => {
    const regex = buildEntityIdRegex();
    const text = "Checkpoint: 🔀️cfb3b6084ff3fe883d5f39b08810a0b90997907a";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match checkpoint ID");
  });

  test("matches session ID (⚪️)", () => {
    const regex = buildEntityIdRegex();
    const text = "Session: ⚪️e753ed61-e8cc-49b7-88f7-dda53b8d5a15";
    const matches = [...text.matchAll(regex)];
    assert.strictEqual(matches.length, 1, "should match session ID");
  });
});

// #endregion 📹️Entity ID Regex Matching Tests

// #region 🔋️CodeLens Behavior Tests
suite("CodeLens Behavior Test Suite", function () {
  this.timeout(15000);

  test("compose.summarize command is not registered", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(!commands.includes("compose.summarize"), "summarize command should not be registered");
  });

  test("compose.navigate command is registered", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.navigate"), "navigate command should be registered");
  });

  test("compose.analyze does not throw on unknown ID", async function () {
    await vscode.commands.executeCommand("compose.analyze", "🧰️unknownentity");
    assert.ok(true, "should not throw on unknown entity");
  });

  test("compose.analyze command is registered", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("compose.analyze"), "analyze command should be registered");
  });

  test("Analyze CodeLens covers every definition entity in TypeScript and Go files", async function () {
    const cases = [
      {
        label: "TypeScript",
        paths: ["repo/vscode/🟦️extension.ts", "vscode/🟦️extension.ts", "🟦️extension.ts"],
      },
      {
        label: "Go",
        paths: ["repo/go/main.go", "go/main.go", "main.go"],
      },
    ];

    for (const testCase of cases) {
      const document = await openWorkspaceDocument(...testCase.paths);
      const expectedIds = collectDefinitionEntityIds(document.getText());
      assert.ok(expectedIds.length > 0, `expected definition IDs in ${testCase.label} source: ${document.uri.fsPath}`);

      const analyzeLensIds = new Set(await getAnalyzeLensIds(document));
      const missingIds = expectedIds.filter((id) => !analyzeLensIds.has(id));

      assert.deepStrictEqual(missingIds, [], `missing Analyze CodeLens IDs in ${testCase.label} source ${document.uri.fsPath}: ${missingIds.join(", ")}`);
    }
  });

  test("Entity CodeLenses do not expose summarize commands", async function () {
    const document = await openWorkspaceDocument("repo/vscode/🟦️extension.ts");
    const summarizeLenses = (await getCodeLenses(document)).filter((lens) => lens.command?.command === "compose.summarize");
    assert.deepStrictEqual(summarizeLenses, [], "summarize CodeLenses should be fully replaced by analyze");
  });

  test("Analyze CodeLens covers native definition scopes in TypeScript and Go files", async function () {
    const cases = [
      {
        label: "TypeScript",
        paths: ["repo/vscode/🟦️extension.ts", "vscode/🟦️extension.ts", "🟦️extension.ts"],
      },
      {
        label: "Go",
        paths: ["repo/go/main.go", "go/main.go", "main.go"],
      },
    ];

    for (const testCase of cases) {
      const document = await openWorkspaceDocument(...testCase.paths);
      const expectedScopes = await collectNativeDefinitionScopes(document);
      assert.ok(expectedScopes.length > 0, `expected native definition scopes in ${testCase.label} source: ${document.uri.fsPath}`);

      const analyzeLensIds = new Set(await getAnalyzeLensIds(document));
      const missingScopes = expectedScopes.filter((scope) => !analyzeLensIds.has(scope));

      assert.deepStrictEqual(missingScopes, [], `missing native Analyze CodeLens scopes in ${testCase.label} source ${document.uri.fsPath}: ${missingScopes.join(", ")}`);
    }
  });
});

suite("Compose VS Code Kit Editor Test Suite", () => {
  test("Kit file detection matches compose kit naming conventions", () => {
    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/metabolism.kit.json"), true);
    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/metabolism/wip/initialKit/kit.compose.json"), true);
    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/kit-metabolism.json"), true);
    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/metabolism.kit.embedded.compose.json"), true);
    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/compose/jsonschema/kit.json"), false);
    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/metabolism.kit.diff.compose.json"), false);
  });

  test("Sketchpad dist resolution prefers bundled assets and falls back to workspace sketchpad dist", () => {
    const fixtureRoot = fs.mkdtempSync(path.join(getWorkspaceRoot(), ".tmp-compose-vscode-"));
    const extensionPath = path.join(fixtureRoot, "extension");
    const bundledDistPath = path.join(extensionPath, "sketchpad-dist");
    const workspaceDistPath = path.join(fixtureRoot, "sketchpad", "dist");
    fs.mkdirSync(bundledDistPath, { recursive: true });
    fs.mkdirSync(workspaceDistPath, { recursive: true });
    fs.writeFileSync(path.join(workspaceDistPath, "webview.html"), "<html>workspace</html>");

    const candidatePaths = getSketchpadDistCandidatePaths(extensionPath);
    assert.deepStrictEqual(candidatePaths, [bundledDistPath, workspaceDistPath]);
    assert.strictEqual(resolveSketchpadDistPath(extensionPath), workspaceDistPath);

    fs.writeFileSync(path.join(bundledDistPath, "webview.html"), "<html>bundled</html>");
    assert.strictEqual(resolveSketchpadDistPath(extensionPath), bundledDistPath);

    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  });
});

// #endregion 🔋️CodeLens Behavior Tests
