/**
 * 🌳️ The TypeScript twin of `🖱️ui/🧪️tests/🌳️document-tree-reconcile/🦀️.rs`. Both read the SAME
 * neutral fixture (`🖱️ui/🧫️fixtures/🌳️document-tree-reconcile/🔣️.json`): the Rust law drives the live
 * arena and proves a published document mounts, keeps its identities and paints; this one proves the
 * wgpu frame path is actually wired to it and that the projection table is complete and agrees with
 * React's own — the two renderers of the one semantic contract.
 *
 * Source-scanning is the honest oracle for the wiring half: the reconcile only ever runs inside a
 * `wasm32` frame worker's chrome walk, which no unit harness can construct, and the sources ARE the
 * contract (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-document-reconcile-2026-09-12.md`).
 */
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import laws from "../../🧫️fixtures/🌳️wgpu-document-reconcile/🔣️.json";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const engineRoot = resolve(suiteRoot, "../..");

type FixtureRecord = { readonly id: number; readonly key: string; readonly component: { readonly type: string; readonly role?: string }; readonly children?: readonly number[] };
const fixture = JSON.parse(readFileSync(resolve(suiteRoot, laws.fixture), "utf8")) as {
  readonly phases: readonly string[];
  readonly unitsPerStep: number;
  readonly framePhaseOrder: readonly string[];
  readonly identityRule: { readonly keyedBy: string; readonly reactTwin: string };
  readonly consumesSubtree: Record<string, unknown>;
  readonly componentProjection: Record<string, string>;
  readonly surfaceIdentity: Record<string, string | null>;
  readonly spaceTokens: Record<string, string | null>;
  readonly document: { readonly root: number; readonly nodeCount: number; readonly nodes: readonly FixtureRecord[] };
  readonly expected: {
    readonly arenaNodeCount: number;
    readonly rootKey: string;
    readonly mountedKeysInTreeOrder: readonly string[];
    readonly mountedIdsInTreeOrder: readonly number[];
    readonly skippedIds: readonly number[];
    readonly childrenOfRootInOrder: readonly number[];
    readonly surfaceNode: { readonly id: number; readonly kind: string; readonly surfaceId: string; readonly paneId: string; readonly controllerId: string };
    readonly paint: { readonly drawCallsAtLeast: number };
  };
  readonly secondGeneration: { readonly removedIds: readonly number[]; readonly expected: { readonly arenaNodeCount: number; readonly preservedIds: readonly number[]; readonly mountedIdsInTreeOrder: readonly number[] } };
};

const source = (key: "reconcileSource" | "treeSource" | "engineSource" | "paintSource" | "interpreterSource" | "shellSource" | "reactInterpreterSource") => readFileSync(resolve(engineRoot, laws[key]), "utf8");
const record = (id: number) => fixture.document.nodes.find((node) => node.id === id);

describe("wgpu retained document reconcile", () => {
  it("keeps the fixture's own document arithmetic self-consistent", () => {
    expect(fixture.document.nodes).toHaveLength(fixture.document.nodeCount);
    expect(fixture.expected.mountedIdsInTreeOrder).toHaveLength(fixture.expected.arenaNodeCount);
    expect(fixture.expected.mountedIdsInTreeOrder.length + fixture.expected.skippedIds.length).toBe(fixture.document.nodeCount);
    expect(fixture.expected.mountedKeysInTreeOrder[0]).toBe(fixture.expected.rootKey);
    expect(record(fixture.document.root)?.key).toBe(fixture.expected.rootKey);
    for (const id of fixture.expected.mountedIdsInTreeOrder) expect(record(id), `mounted id ${id} is a real record`).toBeDefined();
    for (const id of fixture.expected.skippedIds) expect(record(id), `skipped id ${id} is a real record`).toBeDefined();
  });

  it("skips exactly the subtree of the one component that owns it", () => {
    const surface = fixture.document.nodes.find((node) => node.component.type === "surface");
    expect(surface, "the fixture carries a surface record").toBeDefined();
    const descendants = new Set<number>();
    const walk = (id: number) => {
      for (const child of record(id)?.children ?? []) {
        descendants.add(child);
        walk(child);
      }
    };
    walk(surface!.id);
    expect([...descendants].sort()).toEqual([...fixture.expected.skippedIds].sort());
    expect(Object.keys(fixture.consumesSubtree)).toContain("surface");

    const reconcile = source("reconcileSource");
    const predicate = reconcile.slice(reconcile.indexOf(`fn ${laws.consumeSubtreePredicate}`));
    const body = predicate.slice(0, predicate.indexOf("\n}\n"));
    expect(body, "Surface is the one component whose subtree the projection consumes").toContain("Component::Surface(_)");
    expect(body, "a Tree's rows must mount as real arena children — this engine's interactive sync resolves them by id").not.toContain("Component::Tree(");
  });

  it("mounts a tree's section and item rows as keyed arena children, because the paint sync resolves them by id", () => {
    const paint = source("paintSource");
    for (const phase of laws.treeRowSyncPhases) expect(paint, `the interactive sync still has a ${phase} phase`).toContain(phase);
    const reconcile = source("reconcileSource");
    expect(reconcile, "section and item records project onto the keyed Stack rows the sync scans for").toContain("Component::TreeSection(_) | ui_contract::Component::TreeItem(_)");
  });

  it("covers every semantic component with exactly one retained projection", () => {
    const reconcile = source("reconcileSource");
    const projection = reconcile.slice(reconcile.indexOf(`pub fn ${laws.projection}`));
    for (const component of Object.keys(fixture.componentProjection)) {
      const variant = component.split(".")[0]!;
      const rustVariant = variant.charAt(0).toUpperCase() + variant.slice(1);
      expect(projection, `the projection handles Component::${rustVariant}`).toContain(`Component::${rustVariant}`);
    }
    // 🧩️ The contract's own enum is the authority on the SET — a variant added there without a
    // projection arm would fail to compile in Rust, and this pins the fixture to the same count.
    expect(new Set(Object.values(fixture.componentProjection)).size).toBeGreaterThan(10);
  });

  it("keys identity on the document node id, the same rule React's interpreter uses", () => {
    expect(fixture.identityRule.keyedBy).toBe("uiNodeId");
    const tree = source("treeSource");
    expect(tree, "the tree carries a document identity ledger").toContain(laws.identityLedger);
    expect(tree, "…and a lookup by document id").toContain(`fn ${laws.identityLookup}`);
    const react = source("reactInterpreterSource");
    expect(react, "the React twin the fixture names still exists").toContain(fixture.identityRule.reactTwin.split(" ")[0]!);

    const preserved = new Set(fixture.secondGeneration.expected.preservedIds);
    for (const id of fixture.secondGeneration.removedIds) expect(preserved.has(id)).toBe(false);
    expect(fixture.secondGeneration.expected.mountedIdsInTreeOrder).toHaveLength(fixture.secondGeneration.expected.arenaNodeCount);
    for (const id of fixture.secondGeneration.expected.preservedIds) expect(fixture.expected.mountedIdsInTreeOrder).toContain(id);
  });

  it("mounts a surface under React's own three identities", () => {
    expect(fixture.expected.surfaceNode.paneId).toBe(record(fixture.expected.surfaceNode.id)?.key);
    expect(fixture.surfaceIdentity.bindingId).toBeNull();
    const react = source("reactInterpreterSource");
    expect(react, "the identity rule is stated once, in React, and mirrored here").toContain(laws.reactIdentityHelper);
    const reconcile = source("reconcileSource");
    expect(reconcile, "the Rust projection names the same rule").toContain(laws.reactIdentityHelper);
  });

  it("is driven from the frame build between ingress and viewport, and is the only production arena writer", () => {
    const interpreter = source("interpreterSource");
    const order = fixture.framePhaseOrder.map((phase) => `UiDocumentFramePhase::${phase.charAt(0).toUpperCase()}${phase.slice(1)}`);
    for (const phase of order) expect(interpreter.indexOf(phase), `the document frame cursor has a ${phase} phase`).toBeGreaterThan(-1);
    expect(interpreter.indexOf("UiDocumentFramePhase::Reconcile,"), "ingress hands off to the reconcile, not straight to viewport").toBeGreaterThan(-1);
    expect(interpreter, "the reconcile is driven through the engine entry").toContain(`engine.${laws.engineEntry}(`);
    expect(interpreter.indexOf("UiDocumentFramePhase::Reconcile =>"), "and the Reconcile arm sits before Viewport").toBeLessThan(interpreter.indexOf("UiDocumentFramePhase::Viewport =>"));

    const engine = source("engineSource");
    const testkitWriter = engine.slice(0, engine.indexOf(`pub fn ${laws.testkitOnlyArenaWriter}(`));
    expect(testkitWriter.endsWith('#[cfg(any(test, feature = "testkit"))]\n    '), "apply_tree stays testkit-only — the reconcile is what production uses").toBe(true);
  });

  it("measures what the production paint entry actually painted, not the retained list it never fills", () => {
    const engine = source("engineSource");
    expect(engine, "the engine publishes a per-window paint census").toContain(laws.paintCensus);
    const interpreter = source("interpreterSource");
    expect(interpreter, "the frame-stats probe reads the census").toContain("paint_census(");
    expect(interpreter, "…and the structure probe reads the layout the paint walk consumes").toContain(`${laws.mountedLayoutAccessor}(`);
    expect(fixture.expected.paint.drawCallsAtLeast).toBeGreaterThan(0);
  });

  it("hands the reconcile the owning app's controller, since the contract moved it off the node", () => {
    const shell = source("shellSource");
    expect(shell, "the shell resolves one controller for every retained document it paints").toContain("fn document_controller_id");
    const interpreter = source("interpreterSource");
    expect(interpreter, "…and threads it into the document step").toContain("controller_id: &str");
  });
});
