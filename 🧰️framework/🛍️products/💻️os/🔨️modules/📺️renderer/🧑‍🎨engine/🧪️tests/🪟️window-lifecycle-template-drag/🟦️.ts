/** 🪟️ Language-neutral oracle for window-template drops and World3d retirement. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { partitionRefreshWindowInstancesV1 } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";

const engineRoot = join(import.meta.dir, "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🪟️window-lifecycle-template-drag", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🪟️window-lifecycle-template-drag", "🔣️.json"), "utf8"));

type Node = { kind: "stack"; windows: string[] } | { kind: "row" | "column"; children: Node[] };
type Zone = { kind: "tab"; path: number[]; index: number } | { kind: "split"; path: number[]; side: "left" | "right" | "top" | "bottom" } | { kind: "rootSplit"; side: "left" | "right" | "top" | "bottom" };
type PublicationAttempt = { render: "refuse" | "document"; journal: "held" | "dispatch-on-next-settle"; topologyOwed: boolean; surfaceFault: boolean };
type Rect = { x: number; y: number; w: number; h: number };
type DisplayBranchPublication = {
  flow: "up";
  acceptedGeneration: number;
  staleGeneration: number;
  viewport: Rect;
  section: { id: string; defaultOpen: false; rows: Array<{ id: string; branch: boolean; defaultOpen?: false; rect: Rect }>; headerRect: Rect };
  closedPublishedIds: string[];
  openedPublishedIds: string[];
};

const intersects = (left: Rect, right: Rect) => left.x < right.x + right.w && left.x + left.w > right.x && left.y < right.y + right.h && left.y + left.h > right.y;

const displayBranchIds = (law: DisplayBranchPublication, open: boolean, generation: number): string[] => {
  if (generation !== law.acceptedGeneration) return [];
  if (!open) return intersects(law.section.headerRect, law.viewport) ? [`section.chevron.${law.section.id}`] : [];
  const ids: string[] = [];
  for (const row of law.section.rows) {
    if (!intersects(row.rect, law.viewport)) continue;
    ids.push(`tree.label.${row.id}`);
    if (row.branch) ids.push(`tree.chevron.${row.id}`);
  }
  if (intersects(law.section.headerRect, law.viewport)) ids.push(`section.chevron.${law.section.id}`);
  return ids;
};

const reducePublication = (attempts: readonly ("refuse" | "document")[], retryCeiling: number) => {
  const accepted = attempts.slice(0, retryCeiling);
  const published = accepted.indexOf("document");
  if (published >= 0) return { attempts: published + 1, journal: "dispatch-on-next-settle", topologyOwed: false, surfaceFault: false, dispatches: 1 };
  const terminal = accepted.length === retryCeiling;
  return { attempts: accepted.length, journal: terminal ? "terminal-refusal" : "held", topologyOwed: !terminal, surfaceFault: accepted.length > 0, dispatches: 0 };
};

const paths = (node: Node, path: number[] = [], out: Record<string, number[]> = {}): Record<string, number[]> => {
  if (node.kind === "stack") node.windows.forEach((id) => (out[id] = path));
  else node.children.forEach((child, index) => paths(child, [...path, index], out));
  return out;
};

const insert = (root: Node, id: string, zone: Zone): Node => {
  if (zone.kind === "tab") return { kind: "stack", windows: [...(root.kind === "stack" ? root.windows : []), id] };
  if (root.kind === "stack" && root.windows.length === 0) return { kind: "stack", windows: [id] };
  const side = zone.side;
  const axis = side === "left" || side === "right" ? "row" : "column";
  const incoming: Node = { kind: "stack", windows: [id] };
  return { kind: axis, children: side === "left" || side === "top" ? [incoming, root] : [root, incoming] };
};

describe("🪟️ window lifecycle and template drag contract", () => {
  test("the neutral vectors satisfy the Ajv schema oracle", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("the neutral drop vectors match the independent layout oracle", () => {
    for (const authored of fixture.dropCases as Array<{ initialWindows?: string[]; zone: Zone; expectedRootKind: Node["kind"]; expectedPaths: Record<string, number[]> }>) {
      const root: Node = { kind: "stack", windows: authored.initialWindows ?? ["main"] };
      const actual = insert(root, "world-2", authored.zone);
      expect(actual.kind).toBe(authored.expectedRootKind);
      expect(paths(actual)).toEqual(authored.expectedPaths);
    }
  });

  test("React fetches the newly mounted instance and retires the closed instance from the live request", () => {
    const publication = fixture.publication as {
      windowKindId: string;
      bodyKey: string;
      closedInstances: string[];
      openedInstance: string;
      mountedAfter: string[];
      fetchedAfter: string[];
    };
    const declared = [
      ...publication.closedInstances.map((id) => ({ id, windowKindId: publication.windowKindId, bodyKey: publication.bodyKey })),
      { id: publication.openedInstance, windowKindId: publication.windowKindId, bodyKey: publication.bodyKey },
    ];
    const { fetched, skipped } = partitionRefreshWindowInstancesV1(declared, new Set(publication.mountedAfter));
    expect(fetched.map((instance) => instance.id)).toEqual(publication.fetchedAfter);
    expect(skipped.map((instance) => instance.id)).toEqual(publication.closedInstances);
  });

  test("React exposes the same MIME, cancellation and targeted drop oracle", () => {
    const canvas = readFileSync(join(engineRoot, "..", "..", "..", "..", "..", "🔨️modules", "🖱️ui", "🧱️elements", "🎨️Canvas", "🟦️.tsx"), "utf8");
    const shell = readFileSync(join(engineRoot, "🧱️elements", "🏛️ShellHost", "🟦️.tsx"), "utf8");
    const tests = readFileSync(join(engineRoot, "..", "..", "..", "..", "..", "🔨️modules", "🖱️ui", "🧪️tests", "🧪️owned-locale-detector-retirement", "🟦️.tsx"), "utf8");
    expect(canvas).toContain(`COMPOSE_WINDOW_TEMPLATE_MIME = "${fixture.mime}"`);
    expect(canvas).toContain("insertWindowAtDropZone");
    expect(canvas).toContain("cancelWindowTemplatePointerDrag");
    expect(shell).toContain("handleTemplateDrop");
    expect(tests).toContain("insertWindowAtDropZone adds a window on root-split");
    expect(tests).toContain("cancelWindowTemplatePointerDrag clears an active template drag session");
  });

  test("publication refusal holds the transfer journal until recovery or one terminal refusal", () => {
    const outcomes = fixture.publicationOutcomes as {
      retryCeiling: number;
      recovery: PublicationAttempt[];
      permanentFault: { renderAttempts: "refuse"[]; journal: string; topologyOwed: boolean; surfaceFault: boolean };
    };
    const recovered = outcomes.recovery.map((_, index) => reducePublication(outcomes.recovery.slice(0, index + 1).map((attempt) => attempt.render), outcomes.retryCeiling));
    expect(recovered.map(({ journal, topologyOwed, surfaceFault }) => ({ journal, topologyOwed, surfaceFault }))).toEqual(
      outcomes.recovery.map(({ journal, topologyOwed, surfaceFault }) => ({ journal, topologyOwed, surfaceFault })),
    );
    expect(recovered.map((outcome) => outcome.dispatches)).toEqual([0, 1]);
    const refused = reducePublication(outcomes.permanentFault.renderAttempts, outcomes.retryCeiling);
    expect(refused).toEqual({
      attempts: outcomes.retryCeiling,
      journal: outcomes.permanentFault.journal,
      topologyOwed: outcomes.permanentFault.topologyOwed,
      surfaceFault: outcomes.permanentFault.surfaceFault,
      dispatches: 0,
    });
  });

  test("one refusing publication cannot head-of-line block a ready peer", () => {
    const cohort = fixture.publicationOutcomes.cohort as {
      refused: string;
      ready: string;
      firstRefreshReleased: string[];
      afterRetryDispatches: Record<string, number>;
    };
    const publications = [
      { id: cohort.refused, rendered: false, attempts: 0 },
      { id: cohort.ready, rendered: true, attempts: 0 },
    ];
    const released = publications.filter(({ rendered }) => rendered).map(({ id }) => id);
    const retained = publications.filter(({ rendered }) => !rendered).map(({ id, attempts }) => ({ id, attempts: attempts + 1 }));
    expect(released).toEqual(cohort.firstRefreshReleased);
    expect(retained).toEqual([{ id: cohort.refused, attempts: 1 }]);
    expect(Object.values(cohort.afterRetryDispatches)).toEqual([1, 1]);
  });

  test("item and byte credit refusals precede every dock mutation", () => {
    const refusals = fixture.publicationOutcomes.atomicCreditRefusals as Array<{ credit: string; dockMutation: string; journal: string }>;
    expect(refusals.map(({ credit }) => credit).sort()).toEqual(["byte", "item"]);
    for (const refusal of refusals) {
      expect(refusal.dockMutation).toBe("none");
      expect(refusal.journal).toBe("refused-before-mutation");
    }
  });

  test("React journals template transfers while its Display palette remains drag-only", () => {
    const shell = readFileSync(join(engineRoot, "🧱️elements", "🏛️ShellHost", "🟦️.tsx"), "utf8");
    const panels = readFileSync(join(engineRoot, "🧱️elements", "📌️ChromePanels", "🟦️.tsx"), "utf8");
    const transfer = shell.slice(shell.indexOf("const handleTemplateDrop"), shell.indexOf("const displayHostRef"));
    const display = panels.slice(panels.indexOf("function buildDisplayWindowsTree"), panels.indexOf("function buildDisplayLayoutTree"));
    expect(transfer).toContain(`noteShellCommand("${fixture.publicationOutcomes.journalPolicy.transferCommand}"`);
    expect(display).toContain("dragData:");
    expect(display).not.toContain("onClick:");
    expect(fixture.publicationOutcomes.journalPolicy.directOpen).toBe("none");
  });

  test("renderer asset completion is guarded by the checked-out surface generation", () => {
    const renderer = readFileSync(join(engineRoot, "🎯️targets", "🧊️wgpu", "🧊️renderer", "🦀️.rs"), "utf8");
    expect(fixture.retirement.lateCompletion).toContain("old surface generation token");
    expect(renderer).toContain("surface_token: AdmittedSurfaceToken");
    expect(renderer).toContain("world3d_states.get_token_mut(surface_token)");
    expect(renderer).toContain("world3d_states.get_token(surface_token).is_some()");
  });

  test("Display publishes only painted rows from one accepted generation and gives visible branches a real gutter toggle", () => {
    const law = fixture.displayBranchPublication as DisplayBranchPublication;
    expect(law.flow).toBe("up");
    expect(displayBranchIds(law, false, law.acceptedGeneration)).toEqual(law.closedPublishedIds);
    expect(displayBranchIds(law, true, law.acceptedGeneration)).toEqual(law.openedPublishedIds);
    expect(displayBranchIds(law, true, law.staleGeneration)).toEqual([]);

    const panels = readFileSync(join(engineRoot, "🧱️elements", "📌️ChromePanels", "🟦️.tsx"), "utf8");
    const tree = readFileSync(join(engineRoot, "..", "..", "..", "..", "..", "🔨️modules", "🖱️ui", "🧱️elements", "🌳️Tree", "🟦️.tsx"), "utf8");
    const projectionBuilder = panels.slice(panels.indexOf("function worldProjectionTemplatesToTreeItems"), panels.indexOf("function displayWindowKindIcon"));
    expect(projectionBuilder).toContain("defaultOpen: false");
    expect(projectionBuilder).toContain("template.children?.length ? { items:");
    const sortableItem = tree.slice(tree.indexOf("const SortableTreeItem:"), tree.indexOf("export const SortableTreeItems:"));
    expect(sortableItem).toContain("if (hasChildren && displayLabel)");
    expect(sortableItem).toContain("<button");
    expect(sortableItem).toContain("<GroupFoldChevron");
  });
});
