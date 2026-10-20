// #region 🧲️Header
/** 🎭️ Scene-host actions of an actor-bound (hub) document reach its browser actor (🎫️ 26/09/23 C10): every window and
 * panel an actor renders hands its component scene hosts (text editor, canvases, boards, 3D worlds) the shell's ONE input
 * funnel, whose actor branch forwards the action to the verified browser actor. A no-op handler stood there since
 * 26/09/09 and dropped every keystroke typed into a hub writer document without a trace (hub head stayed 0, measured on
 * hubs 7800 and 8021). The funnel's own routing is pinned by the call sites it must keep. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import ts from "typescript";
import panelHostingSource from "../../🧱️elements/🏛️ShellHost/📋️panel-hosting/🟦️.tsx?raw";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import documentTransferTabsSource from "../../🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🟦️.tsx?raw";

const shellActionsSource = readModuleTreeSource(join(import.meta.dirname, "..", "..", "🧱️elements", "🏛️ShellHost", "🖱️shell-actions", "🟦️.ts"));
const shellTreeSource = Object.values(import.meta.glob<string>(["../../🧱️elements/🏛️ShellHost/🟦️.tsx", "../../🧱️elements/🏛️ShellHost/**/🟦️.ts", "../../🧱️elements/🏛️ShellHost/**/🟦️.tsx", "!../../🧱️elements/🏛️ShellHost/🧪️tests/**"], { query: "?raw", import: "default", eager: true })).join("\n");
// #endregion 🔌️Adapters

//#region 🧪️Laws
describe("actor window actions", () => {
  it("hands every actor-rendered window and panel the shell's input funnel, never a handler that drops actions", () => {
    expect(shellTreeSource).toContain("onActionStable");
    expect(shellTreeSource).not.toContain("refuseBrowserActorActionDescriptor");
    const tree = ts.createSourceFile("panel-hosting.tsx", panelHostingSource, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
    const windows: ts.JsxSelfClosingElement[] = [];
    const visit = (node: ts.Node): void => {
      if (ts.isJsxSelfClosingElement(node) && node.tagName.getText(tree) === "InterpretedUiNode") windows.push(node);
      ts.forEachChild(node, visit);
    };
    visit(tree);
    expect(windows.length).toBeGreaterThan(0);
    for (const window of windows) {
      const action = window.attributes.properties.find((attribute) => ts.isJsxAttribute(attribute) && attribute.name.getText(tree) === "onAction");
      expect(action && ts.isJsxAttribute(action) && action.initializer && ts.isJsxExpression(action.initializer) && action.initializer.expression?.getText(tree)).toBe("onActionStable");
    }
    const primaryStores = windows.flatMap((window) => window.attributes.properties.flatMap((attribute) => {
      if (!ts.isJsxAttribute(attribute) || attribute.name.getText(tree) !== "store" || !attribute.initializer || !ts.isJsxExpression(attribute.initializer)) return [];
      const expression = attribute.initializer.expression;
      if (!expression || !ts.isBinaryExpression(expression) || expression.left.getText(tree) !== "browserActorStore") return [];
      return [expression];
    }));
    expect(primaryStores).toHaveLength(1);
    expect(primaryStores[0]!.operatorToken.kind).toBe(ts.SyntaxKind.QuestionQuestionToken);
    expect(ts.isCallExpression(primaryStores[0]!.right) && primaryStores[0]!.right.expression.getText(tree)).toBe("builtNodeStoreFor");
    expect(documentTransferTabsSource).toContain("{ stores: new Map(currentBrowserActorUi.panels), onIntent: browserActorPanelIntent, onAction: onActionStable }");
  });

  it("forwards a funnelled action of an actor-bound session to its verified browser actor", () => {
    const funnel = shellActionsSource.slice(shellActionsSource.indexOf("const onAction = useCallback("));
    expect(funnel.length).toBeGreaterThan(0);
    expect(funnel).toContain("directBrowserActor = directBrowserActorForSession(targetSession);");
    expect(funnel).toMatch(/if \(directBrowserActor !== null\) \{[\s\S]*?await dispatchDirectBrowserActorCommand\(\s*directBrowserActor,/u);
  });
});
//#endregion 🧪️Laws
