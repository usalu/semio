/** 🕰️ The framework history body renders through the interpreter from the shell's history projection, with no host-built history tab, and the presence of peers' open history edits reads back through the ephemeral peer fields. */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { createElement, Fragment } from "react";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { PresenceBar } from "@semio-tech/ui-react";
import { encodePresenceHistoryEdit, encodePresenceInteraction, encodePresenceToolRun } from "@semio-tech/framework/replication";
import { type ActionDescriptor, type UiIntent } from "@semio-tech/framework";
import { builtNodeToSnapshot, UiDocumentStore } from "../../../../📃️UiDocumentStore/🟦️.tsx";
import { checkinSubmitMessageV1 } from "../../../🧾️transfer-checkin-tutorial/🟦️.ts";
import { presenceEphemeralPeerFieldsV1 } from "../../../🔩️plumbing/👥️presence-identity/🟦️.ts";
import { FRAMEWORK_CHECKIN_CONTROLLER_ID, syncShellLabelLocale } from "../../../🏷️labels/🟦️.tsx";
import { panelActionRoutesThroughHostV1 } from "../../../🌳️tree-windows/🟦️.tsx";
import { panelTabDefinitionToNode } from "../../../🛣️route-and-chrome/🟦️.tsx";
import { timeTravelPeerPresenceV1 } from "../../🟦️.tsx";
import { UiPresenceOverlayContext } from "../../../../🗣️Interpreter/👥️presence-overlay/🟦️.ts";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
import { LOCALES, PeersCorpusCase, PeersCorpusRow, bind, button, computeAccessibleName, controller, node, overlay, peersCorpus, section, shellHelpers, treeItem } from "../🧰️history-test-support/🟦️.tsx";

const shellHelpersSource = (directory: string): string =>
  readdirSync(directory, { withFileTypes: true })
    .filter((entry) => !["🧪️tests", "🧫️fixtures", "🧬️schema"].includes(entry.name))
    .map((entry) => (entry.isDirectory() ? shellHelpersSource(join(directory, entry.name)) : /\.tsx?$/u.test(entry.name) ? readFileSync(join(directory, entry.name), "utf8") : ""))
    .join("\n");

describe("🕰️ the framework history body renders through the interpreter", () => {
  afterEach(() => cleanup());
  const body = node("framework.history", { type: "tree", interactionDomain: null }, [
    section("framework.history.actions", "Actions", [
      treeItem("framework.history.undo", "Undo", [button("framework.history.undo.run", "Undo", bind(controller, "undo"))]),
      treeItem("framework.history.checkin", "Check In", [button("s-checkin", "Check In", bind(FRAMEWORK_CHECKIN_CONTROLLER_ID, "submit"))]),
    ]),
    section("framework.history.commands", "Commands", [
      treeItem("framework.history.entry.4", "Drag selection", [button("framework.history.entry.4.edit", "Edit", bind(controller, "historyEditBegin", { mutationId: "m-2" }))]),
    ]),
  ]);

  it("mounts the guest's body on the History leaf and dispatches its authored verbs, a row's lone button becoming the row's activation only where the row says what it does", () => {
    const store = new UiDocumentStore("panel:framework.panel.history");
    store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", body));
    const dispatched: ActionDescriptor[] = [];
    const tab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: { native: { en: "History", de: "Verlauf" }, reuse: { en: "History", de: "Verlauf" } }, group: "settings" as const, bodyKey: "framework.body.history", children: [] };
    const leaf = panelTabDefinitionToNode(tab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", { "framework.panel.history": store }, (action) => dispatched.push(action), 1, overlay);
    expect([leaf.kind, leaf.id]).toEqual(["leaf", "framework.panel.history"]);
    if (leaf.kind !== "leaf") return;
    const source = leaf.trees[0]!.tree;
    const config = "resolveTree" in source ? source.resolveTree() : source;
    const view = render(createElement(Fragment, null, config.emptyState));
    for (const name of ["Undo", "Check In", "Edit"]) (view.getByRole("button", { name }) as HTMLButtonElement).click();
    expect(dispatched.map(({ controllerId, action, args }) => ({ controllerId, action, args: args ?? null }))).toEqual([
      { controllerId: controller, action: "undo", args: null },
      { controllerId: FRAMEWORK_CHECKIN_CONTROLLER_ID, action: "submit", args: null },
      { controllerId: controller, action: "historyEditBegin", args: { mutationId: "m-2" } },
    ]);
    expect([checkinSubmitMessageV1(undefined), checkinSubmitMessageV1({ message: "  release  " }), checkinSubmitMessageV1({ message: " " })]).toEqual(["check-in", "release", "check-in"]);
  });

  it("routes an actor-bound document's Undo and check-in through the host, and every other gesture to its actor", () => {
    const store = new UiDocumentStore("panel:framework.panel.history");
    store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", body));
    const hosted: ActionDescriptor[] = [];
    const actor: string[] = [];
    const tab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: { native: { en: "History", de: "Verlauf" }, reuse: { en: "History", de: "Verlauf" } }, group: "settings" as const, bodyKey: "framework.body.history", children: [] };
    const actorPanels = { stores: new Map([["framework.panel.history", store]]), onIntent: (tabId: string, intent: UiIntent) => void actor.push(`${tabId}:${intent.action.name}`), onAction: (action: ActionDescriptor) => void hosted.push(action) };
    const leaf = panelTabDefinitionToNode(tab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", {}, () => undefined, 1, overlay, "native", "en", null, undefined, actorPanels);
    if (leaf.kind !== "leaf") throw new Error("history tab is a leaf");
    const source = leaf.trees[0]!.tree;
    const config = "resolveTree" in source ? source.resolveTree() : source;
    const view = render(createElement(Fragment, null, config.emptyState));
    for (const name of ["Undo", "Check In", "Edit"]) (view.getByRole("button", { name }) as HTMLButtonElement).click();
    expect(hosted.map(({ controllerId, action }) => `${controllerId}:${action}`)).toEqual([`${controller}:undo`, `${FRAMEWORK_CHECKIN_CONTROLLER_ID}:submit`]);
    expect(actor).toEqual(["framework.panel.history:historyEditBegin"]);
    expect([panelActionRoutesThroughHostV1({ controllerId: controller, action: "redo" }), panelActionRoutesThroughHostV1({ controllerId: controller, action: "historyEditRerun" })]).toEqual([false, false]);
  });

  it("marks the rows and chips of peers editing in time travel exactly as the shared peers corpus says, in both languages", () => {
    const localized = (text: { readonly en: string; readonly de: string }) => ({ native: text, reuse: text });
    const rows = peersCorpus.rows as readonly PeersCorpusRow[];
    const entries = rows.map((row) => ({ seq: row.seq, editId: row.editId, actionId: "apply", label: localized({ en: "Apply", de: "Anwenden" }), kind: "mutation", timestamp: "t", mutations: row.mutations.map((mutation, index) => ({ mutationId: mutation.mutationId, position: index, opIndex: index, label: localized(mutation.label) })) })) as unknown as Parameters<typeof timeTravelPeerPresenceV1>[1];
    const peerBody = node("framework.history", { type: "tree", interactionDomain: null }, [
      section("framework.history.commands", "Commands", rows.map((row) => treeItem(`framework.history.entry.${row.seq}`, row.mutations[0]!.label.en, [node(`framework.history.mutation.${row.mutations[0]!.mutationId}`, { type: "treeItem", label: row.mutations[0]!.label.en, description: null, icon: "circle", defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [{ icon: "edit", label: "Edit", verb: "historyEditBegin", placement: "row", disabled: false }], target: { scope: controller, version: 1, args: { mutationId: row.mutations[0]!.mutationId }, activation: "historyEditBegin" } })]))),
    ]);
    for (const peerCase of peersCorpus.cases as readonly PeersCorpusCase[]) {
      for (const locale of LOCALES) {
        syncShellLabelLocale(locale);
        const expected = peerCase.expect[locale];
        const presence = timeTravelPeerPresenceV1(peerCase.peers, entries, { terminology: "native", locale });
        expect(presence.peers.some((peer) => "historyEdit" in peer), `${peerCase.name} (${locale}): the wire field never reaches the roster`).toBe(false);
        expect(presence.peers.flatMap((peer) => (peer.activity === undefined ? [] : [{ actor: peer.actor, text: peer.activity.text, badge: peer.activity.badge }])), `${peerCase.name} (${locale}): chips`).toEqual(expected.chips);
        expect([...presence.overlay.byKey].map(([key, entry]) => ({ key, text: (entry.notes ?? []).join(" · ") })).sort((a, b) => (a.key < b.key ? -1 : a.key > b.key ? 1 : 0)), `${peerCase.name} (${locale}): notes`).toEqual(expected.notes);
        const store = new UiDocumentStore("panel:framework.panel.history");
        store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", peerBody));
        const tab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: localized({ en: "History", de: "Verlauf" }), group: "settings" as const, bodyKey: "framework.body.history", children: [] };
        const leaf = panelTabDefinitionToNode(tab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", { "framework.panel.history": store }, () => undefined, 1, overlay);
        if (leaf.kind !== "leaf") throw new Error("history tab is a leaf");
        const source = leaf.trees[0]!.tree;
        const config = "resolveTree" in source ? source.resolveTree() : source;
        const view = render(createElement(UiPresenceOverlayContext.Provider, { value: presence.overlay, children: createElement(Fragment, null, config.emptyState) }));
        for (const note of expected.notes.filter((entry) => entry.key.startsWith("framework.history.entry."))) expect(view.container.textContent, `${peerCase.name} (${locale}): ${note.key}`).toContain(note.text);
        const roster = render(createElement(PresenceBar, { peers: presence.peers }));
        for (const peer of peerCase.peers) {
          const chip = roster.container.querySelector<HTMLElement>(`[data-row-id="peer:${peer.actor}"]`)!;
          const activity = expected.chips.find((entry) => entry.actor === peer.actor);
          expect([chip.hasAttribute("data-presence-activity"), activity === undefined || computeAccessibleName(chip).includes(activity.text), roster.container.querySelector(`[data-row-id="peer-activity-badge:${peer.actor}"]`)?.textContent ?? null], `${peerCase.name} (${locale}): ${peer.actor}`).toEqual([activity !== undefined, true, activity?.badge ?? null]);
        }
        view.unmount();
        roster.unmount();
      }
    }
    syncShellLabelLocale("en");
  });

  it("publishes this human's own history edit, tool run and interaction in the presence heartbeat, and nothing it does not carry", () => {
    const historyEdit = { mutationId: "m-2", stage: "choosing", drafts: 2 } as const;
    const toolRun = { toolId: "fill", state: "paused", stage: 0, completed: 3 } as const;
    const interaction = { app_id: "app", domains: [] };
    expect(presenceEphemeralPeerFieldsV1({ historyEdit: encodePresenceHistoryEdit(historyEdit), toolRun: encodePresenceToolRun(toolRun), interaction: encodePresenceInteraction(interaction) })).toEqual({ historyEdit, toolRun, interaction });
    expect([presenceEphemeralPeerFieldsV1({ historyEdit: [], toolRun: [], interaction: [] }), presenceEphemeralPeerFieldsV1(undefined)]).toEqual([{}, {}]);
  });

  it("keeps no host-built history tab or frozen history labels in the shell", () => {
    const shellHost = readModuleTreeSource(join(shellHelpers, "..", "🏛️ShellHost", "🟦️.tsx"));
    const helpers = shellHelpersSource(shellHelpers);
    for (const gone of ["frameworkUtilitiesHistoryTab", "historyPanelText", "checkinDialog", "s-checkin-message"]) expect(shellHost.includes(gone), gone).toBe(false);
    for (const gone of ["HISTORY_PANEL_LABELS", "SHELL_OWNED_PANEL_TAB_IDS", "shellRendersPanelTabItself"]) expect(helpers.includes(gone), gone).toBe(false);
  });
});
