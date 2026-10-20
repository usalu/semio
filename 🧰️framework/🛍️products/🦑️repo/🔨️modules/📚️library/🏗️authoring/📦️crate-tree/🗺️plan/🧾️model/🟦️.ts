import type { FileMeasure } from "../../📏️measure-union/🟦️.ts";
import type { CarveSlice, ModuleDecl } from "../../🧱️decl-tree/🟦️.ts";
import type { MirrorImport } from "../../📝️render/🟦️.ts";
import type { RustModuleNode, RustSourceTree } from "../../🌳️module-tree/🟦️.ts";
import type { CrateBudget } from "../💰️budget/🟦️.ts";
import type { TestTransformOptions } from "../../🧪️test-closure/🟢️shared/🟦️.ts";

/** 📐️ Measured closure of one crate in one cfg. */
export type CrateMeasure = Readonly<{ files: number; bytes: number; lines: number; embedded: number; paths: readonly string[]; sourcePaths: readonly string[]; entries: ReadonlyMap<string, FileMeasure> }>;

/** 🍰️ One planned crate: the slices it owns and everything its mirror root needs. */
export type PlannedCrate = {
  readonly id: string;
  readonly name: string;
  readonly packageDir: string;
  readonly description: string;
  readonly doc: string;
  readonly slices: readonly CarveSlice[];
  readonly attach: readonly ModuleDecl[];
  readonly dependsOn: readonly string[];
  readonly imports: readonly MirrorImport[];
  readonly itemImportLines: readonly string[];
  readonly externLines: readonly string[];
  readonly innerAttributes: readonly string[];
  readonly movedItemsText: string;
  readonly hoisted?: boolean;
  readonly lib: CrateMeasure | null;
  readonly test: CrateMeasure | null;
};

/** ✂️ A declaration in a file that moves to another crate and is replaced by a re-export. */
export type DeclCut = { readonly file: string; readonly start: number; readonly end: number; readonly text: string; readonly anchor?: { readonly eff: readonly string[]; readonly crate: string; readonly visibility: string; readonly component: boolean } };

/** 🗺️ The complete carve plan of one artifact. */
export type ArtifactCratePlan = {
  readonly repoRoot: string;
  readonly artifactDir: string;
  readonly artifactCrate: string;
  readonly packageDir: string;
  readonly rootFile: string;
  readonly tree: RustSourceTree;
  readonly crates: readonly PlannedCrate[];
  readonly wrapperNodes: ReadonlyMap<string, RustModuleNode>;
  readonly gates: ReadonlyMap<string, readonly string[]>;
  readonly rootEdits: readonly RootEdit[];
  readonly weave: readonly WovenExport[];
  readonly placement: ReadonlyMap<string, string>;
  readonly fileCuts: readonly DeclCut[];
  readonly movedUnits: readonly { readonly names: readonly string[]; readonly blocks: readonly { readonly start: number; readonly end: number }[]; readonly to: string }[];
  readonly movedAppends: readonly { readonly file: string; readonly text: string; readonly anchor: string }[];
  readonly keptModules: readonly string[];
  readonly overlay: { readonly edits: ReadonlyMap<string, string>; readonly creates: ReadonlyMap<string, string>; readonly deletes: ReadonlySet<string>; readonly notes: readonly string[] };
  readonly rootMeasure: { readonly lib: CrateMeasure; readonly test: CrateMeasure } | null;
  readonly problems: readonly string[];
  readonly blockers: readonly PlanBlocker[];
  readonly packing: PackingStats;
};

/** ✂️ An edit of the artifact root; edits that replace a carved declaration remember it so the facade can weave children into the replacement. */
export type RootEdit = { readonly start: number; readonly end: number; readonly text: string; readonly anchor?: { readonly eff: readonly string[]; readonly crate: string; readonly visibility: string; readonly component: boolean } };

/** 🧵️ A module whose mount was cut out of its parent because the parent never uses it; the facade exports it at its original path. */
export type WovenExport = { readonly eff: readonly string[]; readonly crate: string };

/** 🧱️ A set of slices the orphan rule or a dependency cycle forces into one crate although together they exceed the budget. */
export type PlanBlocker = { readonly slices: number; readonly files: number; readonly bytes: number; readonly lines: number; readonly coherence: readonly string[]; readonly cycles: readonly string[]; readonly sample: readonly string[]; readonly causes: readonly string[]; readonly edges: readonly string[]; readonly rings: readonly string[]; readonly groupReport: readonly string[] };

/** 📦️ How the packing loop decided: merges rejected by the per-file lower bound, merges that needed a full re-measure of the merged closure, and merges accepted. */
export type PackingStats = { pruned: number; measured: number; merged: number };

/** 🧮️ Options of the planner. */
export type PlanOptions = Readonly<{ target?: CrateBudget; hard?: CrateBudget; tests?: TestTransformOptions; mutationFamilies?: "off" | "on" | "auto"; inlineRootModules?: boolean; weave?: boolean; incrementalMeasure?: boolean; lift?: readonly string[] }>;

export type SliceInfo = {
  slice: CarveSlice;
  index: number;
  effs: string[];
  lib: CrateMeasure | null;
  test: CrateMeasure | null;
  wasm: CrateMeasure | null;
  libEdges: Set<string>;
  testEdges: Set<string>;
  rootRefs: Set<string>;
  testRootRefs: Set<string>;
  keptRefs: Set<string>;
  shimRefs: Set<string>;
  testShimRefs: Set<string>;
  heads: Set<string>;
  coherence: Set<string>;
  why: Map<string, string>;
  firstFile: string;
};
