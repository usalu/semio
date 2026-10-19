/// <reference types="vite/client" />
// #region 🧲️Header
// 💻️ ✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🟦️.ts
// Specs: The `🏗️fem` scope's story coordination: the story-local reader of the two fem DSL dialects and the scene projections, bilingual labels and command emulators built on it — everything the `🎭️*` stories need that is not itself a story.
// Specs: Story-local reader for the two `🏗️fem` DSL dialects (`fem.fem2d.dsl`, `fem.fem3d.dsl` — the
// `🗣️.dsl.semio` example assets under `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/**/📚️examples/🎬️demo/🖼️assets/`).
// Summary: Shared by every `stories/fem/**` story file so the fixture data in the stories is the REAL
// shipped example document, not hand-authored story data — the same discipline `../block/dsl.ts` follows.
// fem's `📦️packages/🦀️rust` crate is a WASM *component*, not a `wasm-bindgen` module, so there is no
// browser-callable `parse_dsl` to reuse; this is a small TypeScript reader of the same text grammar and
// deliberately a READER only — it never re-emits DSL, so it can never drift into a second authority.
// Grammar (as emitted by `🚪️io/📝️text/📸️snapshot/🦀️.rs` via the `dsl::DslArtifact` derive): a
// `semio <dialect> v<n>` banner, top-level `key=value` lines, `name { … }` statement blocks whose lines are
// each `<keyword> key=value …` records (`elements`) or a bare assignment run (`analysis`), and
// `name [col:TYPE …] { rows }` tables. fem rows go beyond block's flat rows in three ways this reader
// handles: quantity-suffixed scalars (`-4m`, `210000000000Pa`, `7850kg/m3`, `0.001m2`), a `BLOCK` column
// holding a nested multi-line `{ … }` statement run (`load-cases.loads`) and a `MAP` column holding a
// nested `{ k=v … }` run (fem3d `combinations.terms`) — so rows are read from a brace-aware TOKEN stream
// chunked by column count, not line by line.
//
// Specs: The `🏗️fem` scope's shared story fixtures, scene-node projections, window-render summaries,
// bilingual (en/de) labels and story-local command emulators — everything the
// `stories/fem/**/*.stories.tsx` files need that is NOT itself a story.
// Summary: Lives beside the story files rather than inside them because Storybook's CSF indexer treats
// EVERY named export of a `*.stories.*` module as a story, so a shared helper exported from a story file
// would be indexed as a broken story (same split `../block/scene.ts` uses). The real shipped example
// documents (`📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`, one per artifact) are `?raw`-imported here and
// parsed by `./dsl.ts`; the scene projections are line-for-line ports of the windows' own Rust builders —
// `fem2d_structure_layers`/`screen_2d`/`vector_layer`
// (`🗿️artifacts/◻️2d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs`) and
// `fem3d_structural_instances`/`quat_z_to`/`quat_roll_z`/`quat_mul`/`mesh_box`
// (`🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` + `🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`).
// Two halves of those windows are deliberately NOT reproduced because they are solver/mesher output the
// browser cannot compute without fem's plugin wasm: fem2d's `mesh-edge-*` overlay
// (`fem2d_region_triangles` → `fem2d_mesh_preview`) and fem3d's `solid-*` boundary meshes
// (`fem3d_solid_mesh_entries` → `fem3d_mesh_preview`), plus every results-window layer derived from
// `fem2d_solve_all`/`fem3d_solve_all`. Each is reported as a counted omission in the story's debug
// readout rather than faked — see `femStoryOmissions`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { fem2dDemoDsl } from "@semio-tech/fem-2d/standards/1/subsets/any/assets/demo";
import { fem3dDemoDsl } from "@semio-tech/fem-3d/standards/1/subsets/any/assets/demo";
import { type Fem2dSnapshot, type Fem3dSnapshot, parseFem2dDsl, parseFem3dDsl } from "./📸️snapshot-projection/🟦️.ts";

//#region 🔖️Locale
/** 🗣️ Explicit OS locale supplied to the story host; artifact commands do not mutate it. */
export type FemStoryLocale = "en-US" | "de-DE";

export const FEM_STORY_LOCALES: readonly FemStoryLocale[] = ["en-US", "de-DE"];

export type FemStoryTextKey =
  | "model"
  | "results"
  | "viewer"
  | "language"
  | "example"
  | "nodes"
  | "members"
  | "supports"
  | "regions"
  | "solids"
  | "materials"
  | "sections"
  | "loadCases"
  | "combinations"
  | "deformationScale"
  | "activeCase"
  | "noLoadCase"
  | "displayMode"
  | "addNode"
  | "loadExample"
  | "clearExample"
  | "omitted";

const FEM_STORY_TEXT: Readonly<Record<FemStoryTextKey, Readonly<Record<FemStoryLocale, string>>>> = {
  model: { "en-US": "Model", "de-DE": "Modell" },
  results: { "en-US": "Results", "de-DE": "Ergebnisse" },
  viewer: { "en-US": "Viewer", "de-DE": "Betrachter" },
  language: { "en-US": "Language", "de-DE": "Sprache" },
  example: { "en-US": "Example", "de-DE": "Beispiel" },
  nodes: { "en-US": "Nodes", "de-DE": "Knoten" },
  members: { "en-US": "Members", "de-DE": "Stäbe" },
  supports: { "en-US": "Supports", "de-DE": "Auflager" },
  regions: { "en-US": "Regions", "de-DE": "Flächen" },
  solids: { "en-US": "Solids", "de-DE": "Volumen" },
  materials: { "en-US": "Materials", "de-DE": "Materialien" },
  sections: { "en-US": "Sections", "de-DE": "Querschnitte" },
  loadCases: { "en-US": "Load cases", "de-DE": "Lastfälle" },
  combinations: { "en-US": "Combinations", "de-DE": "Kombinationen" },
  deformationScale: { "en-US": "Deformation scale", "de-DE": "Verformungsmaßstab" },
  activeCase: { "en-US": "Active case", "de-DE": "Aktiver Lastfall" },
  noLoadCase: { "en-US": "No load case defined", "de-DE": "Kein Lastfall definiert" },
  displayMode: { "en-US": "Display mode", "de-DE": "Anzeigemodus" },
  addNode: { "en-US": "Add node", "de-DE": "Knoten hinzufügen" },
  loadExample: { "en-US": "Load example", "de-DE": "Beispiel laden" },
  clearExample: { "en-US": "Clear document", "de-DE": "Dokument leeren" },
  omitted: { "en-US": "Not rendered here", "de-DE": "Hier nicht gezeichnet" },
};

/** 🗣️ One label in the requested locale. Throws on an unknown key so a typo surfaces at render time instead of printing `undefined` into the panel. */
export function femStoryLabel(key: FemStoryTextKey, locale: FemStoryLocale): string {
  const entry = FEM_STORY_TEXT[key];
  if (!entry) throw new Error(`[fem-story] unknown label key ${JSON.stringify(key)}`);
  return entry[locale];
}

//#endregion 🔖️Locale

//#region 🔖️Fixtures
/** 🎬️ The single `📚️examples/*` unit each fem subset registers (`🌐️any/🦀️.rs`'s `examples()`), keyed by the id its `ExampleSource` carries. */
export const FEM2D_STORY_EXAMPLE_ID = "demo";

/** 🎬️ The registered example and an explicit empty-document selection. */
export const FEM3D_STORY_EXAMPLE_ID = "demo";

export const FEM3D_CLEARED_EXAMPLE_ID = "cleared";

export const FEM2D_DEMO_SNAPSHOT: Fem2dSnapshot = parseFem2dDsl(fem2dDemoDsl);

export const FEM3D_DEMO_SNAPSHOT: Fem3dSnapshot = parseFem3dDsl(fem3dDemoDsl);

/** 📨️ `ActionDescriptor.args` is declared `unknown`, so every story reducer narrows it here once instead of casting at each read. A non-object payload becomes an empty bag, exactly as a Rust handler sees no named arguments. */
export function femStoryActionArgs(args: unknown): Record<string, unknown> {
  return typeof args === "object" && args !== null && !Array.isArray(args) ? (args as Record<string, unknown>) : {};
}

/** 🪪️ Port of `crate::app_surface::next_id` (`⚙️engine/🖥️app-surface/🦀️.rs`): the smallest `"{prefix}{n}"` not already taken, starting at the current count. */
export function femNextId(existing: Iterable<string>, prefix: string): string {
  const ids = new Set(existing);
  let index = ids.size;
  while (ids.has(`${prefix}${index}`)) index += 1;
  return `${prefix}${index}`;
}

//#endregion 🔖️Fixtures

//#region 🔖️Omissions
/** 🕳️ One half of a window's Rust render this story cannot reproduce in the browser, with the reason — surfaced in every debug readout so a blank region is never mistaken for a bug. */
export type FemStoryOmission = { readonly id: string; readonly reason: string };

export const FEM2D_MESH_PREVIEW_OMISSION: FemStoryOmission = {
  id: "mesh-edge-*",
  reason: "fem2d_region_triangles → fem2d_mesh_preview is Rust-only (plugin wasm); the region mesh overlay is omitted rather than approximated",
};

export const FEM2D_SOLVER_OMISSION: FemStoryOmission = {
  id: "deformed-* / reaction-* / moment-* / contour-*",
  reason: "fem2d_solve_all is Rust-only (plugin wasm); every displacement/reaction/moment/von-Mises layer is omitted",
};

export const FEM3D_SOLID_MESH_OMISSION: FemStoryOmission = {
  id: "solid-*",
  reason: "fem3d_solid_mesh_entries → fem3d_mesh_preview is Rust-only (plugin wasm); solid boundary meshes are omitted",
};

export const FEM3D_SOLVER_OMISSION: FemStoryOmission = {
  id: "displacement offsets / von-Mises vertex colors",
  reason: "fem3d_solve_all is Rust-only (plugin wasm); the results scene renders the undeformed, uncolored structure",
};

/** 🕳️ The omission list as flat JSON for a `data-testid`'d debug panel. */
export function femStoryOmissions(omissions: readonly FemStoryOmission[]): readonly FemStoryOmission[] {
  return omissions;
}

//#endregion 🔖️Omissions
