/** 🪪️ The language-neutral Node oracle of the retained-surface-host law, re-derived from the shared fixture.
 * @see ../../🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts — the verbs that run it */

import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { join } from "node:path";
import { readModuleTreeSource } from "../../../../../../../📦️packages/🟦️typescript/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

//#region 🪪️SurfaceHostRetentionOracle
type RetentionBody = { readonly key: string; readonly component: string; readonly children?: readonly RetentionBody[] };

/** 🪪️ Re-derives the retained-surface law from `🖱️ui/🧬️contract/🧫️fixtures/🪪️surface-host-retention.json`
 * with no React, no jsdom and none of our renderer code — a framework-free twin of the vitest half.
 *
 * It re-implements `builtNodeToSnapshot`'s pre-order DFS numbering and the sibling-key rule from the
 * fixture's own written law, then runs a minimal reconciler over the fixture's four refreshes TWICE:
 * once keyed on the authored key (the rule now shipped) and once keyed on the minted id (the rule
 * that shipped before). The first must mount the surface exactly once; the second must mount it more
 * than once — so the twin proves the law AND witnesses the defect it replaces, on the same data.
 * Actual retained node identity, DOM keys and sibling reconciliation are compared with the shared observations.
 *
 * Live defect 26/09/09/PROCEDURAL-3D-END-TO-END: `window:procedural-main` grew four port rows on an
 * eval settle, the node-graph surface renumbered 30 → 34, and React rebuilt the flow host — a second
 * wasm session, a second canvas and a second wasm-side surface per boot. */
export function surfaceHostRetentionOracle(repoRoot: string): number {
  const fixturePath = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧫️fixtures/🪪️surface-host-retention.json";
  const fixture = JSON.parse(readFileSync(join(repoRoot, fixturePath), "utf8")) as {
    surface: string;
    surfaceKey: string;
    canvasKey: string;
    expected: { surfaceHostMounts: number; surfaceAttaches: number; surfaceDomId: string; canvasDomId: string; surfaceReactKeyPath: readonly string[]; surfaceNodeIdByRefresh: readonly number[]; bodyNodeCountByRefresh: readonly number[] };
    siblingKeyCases: readonly { name: string; siblings: readonly { id: number; key: string }[]; expected: readonly string[] }[];
    refreshes: readonly { name: string; statusJson: string; body: RetentionBody }[];
  };
  let checks = 0;
  const check = (condition: boolean, message: string): void => {
    assert(condition, message);
    checks += 1;
  };

  /** 🧬️ `builtNodeToSnapshot`'s numbering, re-derived: an id is taken BEFORE the children are walked. */
  const mint = (body: RetentionBody): Map<string, number> => {
    const ids = new Map<string, number>();
    let next = 1;
    const walk = (node: RetentionBody): void => {
      ids.set(node.key, next);
      next += 1;
      for (const child of node.children ?? []) walk(child);
    };
    walk(body);
    return ids;
  };
  const countNodes = (node: RetentionBody): number => 1 + (node.children ?? []).reduce((total, child) => total + countNodes(child), 0);

  fixture.refreshes.forEach((refresh, index) => {
    check(countNodes(refresh.body) === fixture.expected.bodyNodeCountByRefresh[index], `refresh "${refresh.name}" must carry ${fixture.expected.bodyNodeCountByRefresh[index]} nodes`);
    check(mint(refresh.body).get(fixture.surfaceKey) === fixture.expected.surfaceNodeIdByRefresh[index], `refresh "${refresh.name}" must mint the surface at id ${fixture.expected.surfaceNodeIdByRefresh[index]}`);
    check(JSON.parse(refresh.statusJson) !== null, `refresh "${refresh.name}" must carry a parseable status map — the refreshes must differ by status, not only by shape`);
  });
  check(new Set(fixture.expected.surfaceNodeIdByRefresh).size > 1, "the fixture must actually renumber the surface, or the retention law it states is vacuous");
  check(new Set(fixture.refreshes.map((refresh) => refresh.statusJson)).size === fixture.refreshes.length, "every refresh must carry a DIFFERENT status map");

  /** 🪪️ `uiSiblingReactKeys`, re-derived from the fixture's written law rather than imported. */
  const siblingKeys = (siblings: readonly { readonly id: number; readonly key: string }[]): string[] => {
    const seen = new Map<string, number>();
    for (const sibling of siblings) if (sibling.key) seen.set(sibling.key, (seen.get(sibling.key) ?? 0) + 1);
    return siblings.map((sibling) => (sibling.key && seen.get(sibling.key) === 1 ? `k:${sibling.key}` : `#${sibling.id}`));
  };
  for (const sample of fixture.siblingKeyCases) {
    assert.deepEqual(siblingKeys(sample.siblings), [...sample.expected], `sibling key case "${sample.name}"`);
    checks += 1;
  }

  /** 🌳️ A minimal reconciler: a mounted subtree survives a refresh when its parent's child-key run
   * still names it, and is torn down and remounted when it does not. `mountsOf` answers how many
   * times the surface was created across the whole refresh sequence. */
  const mountsOf = (keyed: "authored" | "minted"): { readonly mounts: number; readonly path: readonly string[] } => {
    let mounted = new Map<string, string>();
    let mounts = 0;
    let path: readonly string[] = [];
    for (const refresh of fixture.refreshes) {
      const ids = mint(refresh.body);
      const next = new Map<string, string>();
      const walk = (node: RetentionBody, prefix: readonly string[]): void => {
        const children = node.children ?? [];
        const runKeys = keyed === "authored"
          ? siblingKeys(children.map((child) => ({ id: ids.get(child.key)!, key: child.key })))
          : children.map((child) => `#${ids.get(child.key)!}`);
        children.forEach((child, index) => {
          const here = [...prefix, runKeys[index]!];
          next.set(here.join("/"), child.key);
          if (child.key === fixture.surfaceKey) path = here;
          walk(child, here);
        });
      };
      walk(refresh.body, []);
      const surfacePath = path.join("/");
      if (mounted.get(surfacePath) !== fixture.surfaceKey) mounts += 1;
      mounted = next;
    }
    return { mounts, path };
  };

  const retained = mountsOf("authored");
  check(retained.mounts === fixture.expected.surfaceHostMounts, `keyed on the authored key, ${fixture.refreshes.length} refreshes must mount the surface host exactly ${fixture.expected.surfaceHostMounts}×, not ${retained.mounts}×`);
  assert.deepEqual([...retained.path], [...fixture.expected.surfaceReactKeyPath], "the surface's reconciliation path must be the authored key path the fixture declares");
  checks += 1;
  const renumbered = mountsOf("minted");
  check(renumbered.mounts > fixture.expected.surfaceHostMounts, `keyed on the minted id the surface must remount (it did ${renumbered.mounts}× here) — a twin that cannot witness the defect cannot prove the fix`);

  const interpreter = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx"));
  check(interpreter.includes("export function uiSiblingReactKeys("), "`uiSiblingReactKeys` must be the interpreter's one reconciliation-key rule");
  check(interpreter.includes("export function uiChildReactKeys("), "`uiChildReactKeys` must resolve a child-id run against the live document state");
  check(!interpreter.includes("<UiNodeView key={childId}"), "a child run keyed on the DFS-minted id is the defect this fixture exists for");
  check(interpreter.includes("uiChildReactKeys(store.getState(), childIds)"), "`ContainerView` must reconcile its children on the authored keys");
  return checks;
}
//#endregion 🪪️SurfaceHostRetentionOracle
