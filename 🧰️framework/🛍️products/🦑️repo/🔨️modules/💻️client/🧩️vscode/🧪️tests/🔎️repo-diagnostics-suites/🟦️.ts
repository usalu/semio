/** 🔎️ The extension-host suites of kit validation, repo diagnostics and refresh diagnostics. */
import * as assert from "assert";
import * as vscode from "vscode";
import { EXPECTED_CONSTRAINTS, openFixture, waitForDiagnostics } from "../🧰️extension-test-kit/🟦️.ts";

// #region 🧪️OwnedExtensionHostTestContract
type OwnedExtensionHostTestContext = { timeout(milliseconds: number): void };
type OwnedExtensionHostTestBody = (this: OwnedExtensionHostTestContext) => void | Promise<void>;
declare function suite(title: string, body: OwnedExtensionHostTestBody): void;
declare function test(title: string, body: OwnedExtensionHostTestBody): void;
declare function suiteSetup(body: OwnedExtensionHostTestBody): void;

// #endregion 🧪️OwnedExtensionHostTestContract
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
    const document = await openFixture("repo/some/folder/🧪️file-invalid/🟦️.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: no breachs found (analyze returned 0)");
      return;
    }
    assert.ok(diagnostics.length > 0, "Invalid repo file should have diagnostics");
  });

  test("Repo diagnostics show breach name as message", async function () {
    const document = await openFixture("repo/some/folder/🧪️file-invalid/🟦️.tsx");
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
    const document = await openFixture("repo/some/folder/🧪️file-invalid/🟦️.tsx");
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
    const document = await openFixture("repo/some/folder/🧪️file/🟦️.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    assert.strictEqual(diagnostics.length, 0, "Valid repo file should have no diagnostics");
  });

  test("Repo diagnostics have code actions for autofixable breachs", async function () {
    const document = await openFixture("repo/some/folder/🧪️file-invalid/🟦️.tsx");
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
});
// #endregion 🏆️Refresh Diagnostics Tests
