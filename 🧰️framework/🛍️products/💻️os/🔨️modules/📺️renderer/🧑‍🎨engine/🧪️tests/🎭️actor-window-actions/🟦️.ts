// #region 🧲️Header
/** @emoji 🎭️ Scene-host actions of an actor-bound (hub) document reach its browser actor (🎫️ 26/09/23 C10): every window and
 * panel an actor renders hands its component scene hosts (text editor, canvases, boards, 3D worlds) the shell's ONE input
 * funnel, whose actor branch forwards the action to the verified browser actor. A no-op handler stood there since
 * 26/09/09 and dropped every keystroke typed into a hub writer document without a trace (hub head stayed 0, measured on
 * hubs 7800 and 8021). The funnel's own routing is pinned by the call sites it must keep. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { describe, expect, it } from "vitest";
import shellSource from "../../🧱️elements/🏛️ShellHost/🟦️.tsx?raw";
// #endregion 🔌️Adapters

//#region 🧪️Laws
describe("actor window actions", () => {
  it("hands every actor-rendered window and panel the shell's input funnel, never a handler that drops actions", () => {
    expect(shellSource).not.toContain("refuseBrowserActorActionDescriptor");
    expect(shellSource).toMatch(/<InterpretedUiNode store=\{browserActorStore \?\? builtNodeStoreFor\([^)]*\)[^}]*\} onAction=\{onActionStable\}/u);
    expect(shellSource).toContain("{ stores: new Map(currentBrowserActorUi.panels), onIntent: browserActorPanelIntent, onAction: onActionStable }");
  });

  it("forwards a funnelled action of an actor-bound session to its verified browser actor", () => {
    const funnel = shellSource.slice(shellSource.indexOf("const onAction = useCallback("), shellSource.indexOf("const onActionStable = useCallback("));
    expect(funnel.length).toBeGreaterThan(0);
    expect(funnel).toContain("directBrowserActor = directBrowserActorForSession(targetSession);");
    expect(funnel).toMatch(/if \(directBrowserActor !== null\) \{[\s\S]*?await dispatchDirectBrowserActorCommand\(\s*directBrowserActor,/u);
  });
});
//#endregion 🧪️Laws
