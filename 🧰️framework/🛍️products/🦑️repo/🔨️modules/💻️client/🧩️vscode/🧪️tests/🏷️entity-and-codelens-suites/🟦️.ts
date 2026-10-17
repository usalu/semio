/** 🏷️ The extension-host suites of the entity emoji registry, the entity ID regex matching and the CodeLens behavior. */
import * as assert from "assert";
import * as vscode from "vscode";
import { buildEntityEmojiPattern, buildEntityIdRegex, ENTITY_EMOJIS, ENTITY_ID_REGEX } from "../../🌪️entity-emoji-registry/🟦️.ts";
import { collectDefinitionEntityIds, collectNativeDefinitionScopes, getAnalyzeLensIds, getCodeLenses, openWorkspaceDocument } from "../🧰️extension-test-kit/🟦️.ts";

// #region 🧪️OwnedExtensionHostTestContract
type OwnedExtensionHostTestContext = { timeout(milliseconds: number): void };
type OwnedExtensionHostTestBody = (this: OwnedExtensionHostTestContext) => void | Promise<void>;
declare function suite(title: string, body: OwnedExtensionHostTestBody): void;
declare function test(title: string, body: OwnedExtensionHostTestBody): void;
declare function suiteSetup(body: OwnedExtensionHostTestBody): void;

// #endregion 🧪️OwnedExtensionHostTestContract
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
        paths: ["repo/vscode/🟦️.ts", "vscode/🟦️.ts", "🟦️.ts"],
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
    const document = await openWorkspaceDocument("repo/vscode/🟦️.ts");
    const summarizeLenses = (await getCodeLenses(document)).filter((lens) => lens.command?.command === "compose.summarize");
    assert.deepStrictEqual(summarizeLenses, [], "summarize CodeLenses should be fully replaced by analyze");
  });

  test("Analyze CodeLens covers native definition scopes in TypeScript and Go files", async function () {
    const cases = [
      {
        label: "TypeScript",
        paths: ["repo/vscode/🟦️.ts", "vscode/🟦️.ts", "🟦️.ts"],
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
// #endregion 🔋️CodeLens Behavior Tests
