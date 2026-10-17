/** 🌈️ The extension-host suites of the CLI tree helpers: leading emoji, display labels, context values, breach kind documents and the CLI tree arguments. */
import * as assert from "assert";
import * as vscode from "vscode";
import { buildCliTreeArgs, FilterTreeDataProvider, treeNodeToItem } from "../../🌳️tree-providers/🟦️.ts";
import { extractLeadingEmoji, treeNodeContextValue, TreeNodeData, treeNodeDisplayLabel } from "../../🏬️uri-resolution/🟦️.ts";

// #region 🧪️OwnedExtensionHostTestContract
type OwnedExtensionHostTestContext = { timeout(milliseconds: number): void };
type OwnedExtensionHostTestBody = (this: OwnedExtensionHostTestContext) => void | Promise<void>;
declare function suite(title: string, body: OwnedExtensionHostTestBody): void;
declare function test(title: string, body: OwnedExtensionHostTestBody): void;
declare function suiteSetup(body: OwnedExtensionHostTestBody): void;

// #endregion 🧪️OwnedExtensionHostTestContract
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
