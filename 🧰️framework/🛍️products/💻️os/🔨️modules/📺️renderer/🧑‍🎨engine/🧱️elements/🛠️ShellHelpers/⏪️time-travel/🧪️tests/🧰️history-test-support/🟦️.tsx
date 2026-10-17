/** 🧰️ History test support: the corpus readers, the accessibility and ARIA oracles, the band mount helpers, the history body builders and the draft editor fixtures that the time-travel component suites share. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import { createElement, Fragment } from "react";
import { act, render } from "@semio-tech/ui-react/test";
import { composeControlKeybindings, UiKeybindingsProvider } from "@semio-tech/ui-react";
import { type ActionDescriptor, type BuiltNode, type HistoryTimeTravel } from "@semio-tech/framework";
import { builtNodeToSnapshot, UiDocumentStore } from "../../../../📃️UiDocumentStore/🟦️.tsx";
import { panelTabDefinitionToNode } from "../../../🛣️route-and-chrome/🟦️.tsx";
import { TimeTravelBand, type TimeTravelFocusTargetV1 } from "../../🟦️.tsx";

export const { computeAccessibleName, computeAccessibleDescription, getRole }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");
const { roles: ariaRoles, aria: ariaProperties } = createRequire(import.meta.url)("aria-query") as { readonly roles: ReadonlyMap<string, { readonly props: Readonly<Record<string, unknown>> }>; readonly aria: ReadonlyMap<string, unknown> };
const here = dirname(fileURLToPath(import.meta.url));
export const shellHelpers = join(here, "..", "..", "..");
export const framework = join(here, "..", "..", "..", "..", "..", "..", "..", "..", "..", "..");
export const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
export const corpus = readJson(join(shellHelpers, "🧪️tests/🧫️fixtures", "🧫️time-travel-band", "🔣️.json"));
export const rowActionCorpus = readJson(join(framework, "🔨️modules", "🖱️ui", "🧪️tests/🧫️fixtures", "♿️disabled-row-action", "🔣️.json")) as readonly { readonly id: string; readonly label: string; readonly focusable: boolean; readonly actionable: boolean }[];
type Case = { readonly name: string; readonly session: HistoryTimeTravel; readonly text: Readonly<Record<"en" | "de", Record<string, string | null>>>; readonly controls: readonly { readonly control: string; readonly controlId: string; readonly action: string; readonly disabledBy: string | null; readonly args?: Readonly<Record<string, string>> }[]; readonly indicator: Readonly<Record<"en" | "de", string>> };
/** 🏷️ The corpus's one copy of every band, indicator, peer and refusal text: label key → tier → locale, placeholders in single braces. */
type LabelRow = Readonly<Record<"normal" | "beginner", Readonly<Record<"en" | "de", string>>>>;
export const labels = corpus.labels as Readonly<Record<string, LabelRow>>;
/** 🧩️ A corpus label at the normal tier with its placeholders filled. */
export const corpusLabel = (key: string, locale: "en" | "de", values: Readonly<Record<string, string | number>> = {}): string => labels[key]!.normal[locale].replace(/\{(\w+)\}/gu, (_, name: string) => String(values[name]));
export const cases = corpus.cases as readonly Case[];
/** 👥️ The peers' history-edit corpus (`🧫️time-travel-peers`) both shells assert: the rows a replica holds, the peers on
 * its roster, and per locale every editing peer's chip and every history-body node's note. */
export const peersCorpus = readJson(join(shellHelpers, "🧪️tests/🧫️fixtures", "🧫️time-travel-peers", "🔣️.json"));
export type PeersCorpusRow = { readonly seq: number; readonly editId: string; readonly mutations: readonly { readonly mutationId: string; readonly label: { readonly en: string; readonly de: string } }[] };
export type PeersCorpusCase = {
  readonly name: string;
  readonly peers: readonly { readonly actor: string; readonly label: string; readonly historyEdit?: { readonly mutationId: string; readonly stage: "editing" | "replaying" | "reviewing" | "choosing" | "finalizing"; readonly drafts: number } }[];
  readonly expect: Readonly<Record<"en" | "de", { readonly chips: readonly { readonly actor: string; readonly text: string; readonly badge: string }[]; readonly notes: readonly { readonly key: string; readonly text: string }[] }>>;
};
export const axes = { terminology: corpus.axes.terminology as string };
export const LOCALES = ["en", "de"] as const;

export function mountBand(session: HistoryTimeTravel, locale: string, overrides: Readonly<Record<string, string>> = {}) {
  const dispatched: ActionDescriptor[] = [];
  const bindings = composeControlKeybindings(new Map(), overrides);
  const view = render(createElement(UiKeybindingsProvider, { bindings, children: createElement(TimeTravelBand, { session, terminology: axes.terminology, locale, controllerId: corpus.controllerId, onAction: (action: ActionDescriptor) => dispatched.push(action) }) }));
  return { ...view, dispatched };
}

export const band = (container: HTMLElement): HTMLElement => container.querySelector("[data-semio-time-travel]") as HTMLElement;
export const controlButton = (container: HTMLElement, control: string): HTMLButtonElement | null => container.querySelector(`[data-semio-time-travel-control="${control}"]`);
/** 💬️ What `aria-describedby` of `element` reads: the text of every element it names, in order. */
export const describedBy = (element: Element): string => (element.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ");
/** 👁️ The disabled reason a sighted person reads right now (`DisabledReasonHint` revealed), with the id it carries. */
export const revealedReason = (): { readonly id: string; readonly text: string } | null => {
  const shown = document.querySelector('[data-slot="row-action-reason"][data-revealed]');
  return shown === null ? null : { id: shown.id, text: shown.textContent ?? "" };
};

/** 🌲️ Rust-shaped history-body nodes (`🔌️plugin/🦀️.rs` `ui_history_panel`, `⏪️time-travel` band and editor sections), built
 * the way the guest's `BuiltNode` arrives: every input row holds one control, every button row one button. */
const STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };
const LEAF = { kind: "leaf", width: "hug", height: "hug" };
type NodeExtra = { readonly tone?: string; readonly label?: string; readonly disabled?: boolean };
export const node = (key: string, component: Record<string, unknown>, children: readonly BuiltNode[] = [], bindings: readonly unknown[] = [], extra: NodeExtra = {}): BuiltNode => ({ key, component, layout: LEAF, style: { ...STYLE, tone: extra.tone ?? "neutral" }, activity: "idle", disabled: extra.disabled ?? false, accessibility: { ...ACCESSIBILITY, label: extra.label ?? null }, bindings: [...bindings], menu: null, children: [...children] }) as unknown as BuiltNode;
export const bind = (scope: string, name: string, args: Record<string, unknown> | null = null, trigger = "activate") => ({ trigger, action: { scope, name, version: 1 }, args });
export const treeItem = (key: string, label: string, children: readonly BuiltNode[], row: { readonly description?: string; readonly icon?: string; readonly tone?: string; readonly open?: boolean } = {}) => node(key, { type: "treeItem", label, description: row.description ?? null, icon: row.icon ?? null, defaultOpen: row.open ?? null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null }, children, [], { tone: row.tone });
export const section = (key: string, label: string, children: readonly BuiltNode[]) => node(key, { type: "treeSection", label, defaultOpen: true, headerToolbar: null, window: null }, children);
export const button = (key: string, label: string, binding: unknown, extra: NodeExtra = {}) => node(key, { type: "button", label, icon: "" }, [], [binding], extra);
export const controller = "toy.controller";
export const overlay = { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {}, actionLabels: {}, utilityLabels: {}, exampleLabels: {}, actionArgLabels: {}, dialogLabels: {}, introductionLabels: {}, groupLabels: {} };
export const historyTab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: { native: { en: "History", de: "Verlauf" }, reuse: { en: "History", de: "Verlauf" } }, group: "settings" as const, bodyKey: "framework.body.history", children: [] };

/** 🧱️ Renders `body` as the History leaf exactly as the shell mounts it, dispatching into `onAction`. */
export function mountHistoryBody(body: BuiltNode, onAction: (action: ActionDescriptor) => void) {
  const store = new UiDocumentStore("panel:framework.panel.history");
  store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", body));
  const leaf = panelTabDefinitionToNode(historyTab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", { "framework.panel.history": store }, onAction, 1, overlay);
  if (leaf.kind !== "leaf") throw new Error("history tab is a leaf");
  const source = leaf.trees[0]!.tree;
  const config = "resolveTree" in source ? source.resolveTree() : source;
  return render(createElement(Fragment, null, config.emptyState));
}

//#region ⏪️DraftEditor
/** ♿️ The axe-level structural findings under `roots`: an unknown `aria-*` attribute, one the element's role does not support
 * (the WAI-ARIA role model of `aria-query`), a `labelledby`/`describedby` naming no element, a control without an accessible
 * name (`dom-accessibility-api`) and a repeated id. */
export function ariaFindings(roots: readonly ParentNode[]): readonly string[] {
  const findings: string[] = [];
  const ids = new Map<string, number>();
  for (const root of roots) {
    for (const element of root.querySelectorAll<HTMLElement>("*")) {
      if (element.id) ids.set(element.id, (ids.get(element.id) ?? 0) + 1);
      const role = getRole(element);
      for (const { name, value } of [...element.attributes]) {
        if (!name.startsWith("aria-")) continue;
        if (!ariaProperties.has(name)) findings.push(`${element.tagName}#${element.id}: unknown ${name}`);
        else if (role !== null && ariaRoles.get(role) !== undefined && !(name in ariaRoles.get(role)!.props) && name !== "aria-hidden") findings.push(`${role}#${element.id}: ${name} not supported`);
        if ((name === "aria-describedby" || name === "aria-labelledby") && value.split(/\s+/u).some((id) => id !== "" && document.getElementById(id) === null)) findings.push(`${role}#${element.id}: ${name} → missing ${value}`);
      }
      const interactive = element.matches('button, input, select, textarea, [role="slider"], [role="combobox"], [role="spinbutton"]') && element.closest('[aria-hidden="true"]') === null;
      if (interactive && computeAccessibleName(element).trim() === "") findings.push(`${role}#${element.id}: no accessible name`);
    }
  }
  for (const [id, count] of ids) if (count > 1) findings.push(`duplicate id ${id}`);
  return findings;
}

/** 🔢️ The session generation every draft verb of the editor body is stamped with. */
export const GENERATION = 7;
export const draftArgs = (path: string, value?: unknown) => ({ generation: GENERATION, path, ...(value === undefined ? {} : { value }) });
const container = (key: string, role: string, label: string | null, children: readonly BuiltNode[]) => node(key, { type: "container", role, label, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, children);
/** ✏️ The Rust-shaped body while a draft is open (`time_travel_band_section`, `time_travel_editor_sections`,
 * `ui_history_panel`): the band and editor sections, one row per input — a stepper, a slider with snaps, a select, a vector
 * and a reference list with "Use selection" — the alternatives, and a history row whose mutation rows carry their outcome. */
export const draftBody = (inputs: boolean): BuiltNode =>
  node("framework.history", { type: "tree", interactionDomain: null }, [
    section("framework.history.timeTravel", "History editing", [
      treeItem("framework.history.timeTravel.status", "Editing a mutation: Drag selection · Error", [], { icon: "clock", tone: "danger" }),
      treeItem("framework.history.timeTravel.exit.row", "Exit time travel", [button("framework.history.timeTravel.exit", "Exit time travel", bind(controller, "historyEditExit"))]),
    ]),
    section("framework.history.editor", "Drag selection", [
      treeItem("framework.history.editor.target", "Draft: Drag selection · Error: Target missing", [], { icon: "alert-circle", tone: "danger" }),
      ...(["Accept", "Discard", "Withdraw"] as const).map((verb) => treeItem(`framework.history.editor.${verb.toLowerCase()}.row`, verb, [button(`framework.history.editor.${verb.toLowerCase()}`, verb, bind(controller, `historyEdit${verb}`, { generation: GENERATION }))])),
    ]),
    ...(inputs
      ? [
          section("framework.history.editor.inputs", "Inputs", [
            treeItem("framework.history.editor.input.dx.row", "dx", [node("framework.history.editor.input.dx", { type: "numberStepper", value: 80, step: 1, uniform: true, min: -1000, max: 1000, precision: null }, [], [bind(controller, "historyEditInput", draftArgs("/dx"), "change")], { label: "dx" })]),
            treeItem("framework.history.editor.input.angle.row", "Angle", [node("framework.history.editor.input.angle", { type: "slider", value: 0, min: -180, max: 180, step: 1, unit: "°", snaps: [-90, 0, 90] }, [], [bind(controller, "historyEditInput", draftArgs("/angle"), "change")], { label: "Angle" })]),
            treeItem("framework.history.editor.input.mode.row", "Mode", [node("framework.history.editor.input.mode", { type: "select", value: "pivot", items: [{ value: "pivot", label: "Pivot" }, { value: "centroid", label: "Centroid" }], placeholder: null }, [], [bind(controller, "historyEditInput", draftArgs("/mode"), "change")], { label: "Mode" })]),
            treeItem("framework.history.editor.input.pivot.row", "Pivot", [
              container("framework.history.editor.input.pivot", "group", "Pivot", (["x", "y"] as const).map((axis, index) =>
                node(`framework.history.editor.input.pivot.${index}.axis`, { type: "container", role: "field", label: axis, description: "mm", required: null, error: null, defaultOpen: null, dropOverlay: null }, [
                  node(`framework.history.editor.input.pivot.${index}`, { type: "input", kind: "number", value: String(index * 10), placeholder: null, commit: "blur", min: -500, max: 500, step: 0.5, accept: null, precision: 1, snaps: [0] }, [], [bind(controller, "historyEditInput", draftArgs(`/pivot/${index}`), "commit")], { label: axis }),
                ]),
              )),
            ]),
            treeItem("framework.history.editor.input.targets.row", "Targets", [
              container("framework.history.editor.input.targets", "group", "Targets", [
                container("framework.history.editor.input.targets.chips", "toolbar", "Targets", [
                  button("framework.history.editor.input.targets.chip.0", "node-1", bind(controller, "historyEditInput", draftArgs("/targets", ["node-2"])), { label: "Remove node-1" }),
                  button("framework.history.editor.input.targets.chip.1", "node-2", bind(controller, "historyEditInput", draftArgs("/targets", ["node-1"])), { label: "Remove node-2" }),
                ]),
                container("framework.history.editor.input.targets.actions", "plain", null, [button("framework.history.editor.input.targets.useSelection", "Use selection", bind(controller, "historyEditUseSelection", draftArgs("/targets")), { label: "Use selection" })]),
              ]),
            ]),
          ]),
        ]
      : []),
    section("framework.history.alternatives", "Alternatives", [
      treeItem("framework.history.alternative.trunk", "Main line", [], { description: "Current", icon: "check" }),
      node("framework.history.alternative.alt-1", { type: "treeItem", label: "Variant", description: "Branched by Ada at 2026-10-01 09:12 UTC · Edited history", icon: "git-branch", defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [{ icon: "git-branch", label: "Switch", verb: "switchAlternative", placement: "row", disabled: false }], target: { scope: controller, version: 1, args: { alternativeId: "alt-1" }, activation: "switchAlternative" } }),
    ]),
    section("framework.history.commands", "Commands", [
      treeItem(
        "framework.history.entry.4",
        "Drag 2 items by (80, 40)",
        [
          treeItem("framework.history.mutation.m-2", "Drag selection", [], { description: "Edited", icon: "edit" }),
          treeItem("framework.history.mutation.m-3", "Drag selection", [], { description: "Warning: Partially applied", icon: "triangle-alert", tone: "warning" }),
          treeItem("framework.history.mutation.m-4", "Drag selection", [], { description: "Error: Target missing", icon: "alert-circle", tone: "danger" }),
        ],
        { open: true },
      ),
    ]),
  ]);
export const editingSession = cases.find((entry) => entry.session.stage === "editing" && entry.session.fault === undefined)!.session;
export const byIdSuffix = (root: ParentNode, key: string): HTMLElement | null => [...root.querySelectorAll<HTMLElement>("[id]")].find((element) => element.id.endsWith(`/${key}`)) ?? null;
export const nextFrames = (count: number) => act(async () => {
  for (let frame = 0; frame < count; frame += 1) await new Promise((resolve) => requestAnimationFrame(() => resolve(null)));
});
export type Transition = { readonly name: string; readonly from: HistoryTimeTravel | null; readonly to: HistoryTimeTravel | null; readonly reveal: boolean; readonly focus: TimeTravelFocusTargetV1 | null; readonly scrollTo: string | null };
/** 🗝️ The draft editor's node keys the corpus holds the Rust producer and every shell's focus resolution to. */
export type EditorKeys = { readonly panel: string; readonly accept: { readonly control: string; readonly row: string }; readonly inputs: readonly { readonly pointer: string; readonly control: string; readonly row: string }[] };
//#endregion ⏪️DraftEditor
