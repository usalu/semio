/** 📮️ The extension-host suites of extended RepoEvent parsing, slugify, URI parsing and the navigation commands. */
import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { invalidateTreeNodeCache, parseUri, slugify } from "../../🏬️uri-resolution/🟦️.ts";
import { extractRepoResult, parseRepoEvents, resolveRepoBinaryPath } from "../../🎼️utilities/🟦️.ts";

// #region 🧪️OwnedExtensionHostTestContract
type OwnedExtensionHostTestContext = { timeout(milliseconds: number): void };
type OwnedExtensionHostTestBody = (this: OwnedExtensionHostTestContext) => void | Promise<void>;
declare function suite(title: string, body: OwnedExtensionHostTestBody): void;
declare function test(title: string, body: OwnedExtensionHostTestBody): void;
declare function suiteSetup(body: OwnedExtensionHostTestBody): void;

// #endregion 🧪️OwnedExtensionHostTestContract
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

  test("resolveRepoBinaryPath names the semio-repo deliverable of the shared Cargo target-dir, or REPO_CLI_BIN", () => {
    const root = fs.mkdtempSync(path.join(require("os").tmpdir(), "repo-binary-"));
    try {
      const ext = process.platform === "win32" ? ".exe" : "";
      const built = path.join(root, ".🧬semio", "🦑️repo", "⚡️cache", "cargo", "target", "debug", `semio-repo${ext}`);
      assert.strictEqual(resolveRepoBinaryPath(root, {}), undefined);
      fs.mkdirSync(path.dirname(built), { recursive: true });
      fs.writeFileSync(built, "");
      assert.strictEqual(resolveRepoBinaryPath(root, {}), built);
      const own = path.join(root, "own-cli");
      assert.strictEqual(resolveRepoBinaryPath(root, { REPO_CLI_BIN: own }), undefined);
      fs.writeFileSync(own, "");
      assert.strictEqual(resolveRepoBinaryPath(root, { REPO_CLI_BIN: own }), own);
    } finally {
      fs.rmSync(root, { recursive: true, force: true });
    }
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
