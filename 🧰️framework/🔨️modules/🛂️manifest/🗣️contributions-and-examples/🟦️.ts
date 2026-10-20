/** 🗣️ The app labels overlay, the program and topic contributions, the space index, the manifest examples with their asset names and the plugin manifest type. */
import { type ArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
import { dialectCoordinate } from "../../🚪️io/🧬️schema/📦️packages/🟦️typescript/🟦️.ts";
import type { CommandDefinition } from "../🔘️definitions-action-ids-and-shell-brand/🟦️.ts";

// #region 🛂️Manifest
//#region 🔌️PluginAndAppContract
//#region PluginRuntime
/** 🗣️ Locale/terminology-aware label patch for an app's window-kind/panel-tab/mode labels, resolved fresh per {@link PluginViewState} — merge over the static {@link PluginManifest} app labels by id. */
export type PluginAppLabelsOverlay = {
  readonly windowKindLabels: Readonly<Record<string, string>>;
  readonly panelTabLabels: Readonly<Record<string, string>>;
  readonly modeLabels: Readonly<Record<string, string>>;
  readonly actionLabels: Readonly<Record<string, string>>;
  readonly utilityLabels: Readonly<Record<string, string>>;
  readonly exampleLabels: Readonly<Record<string, string>>;
  readonly actionArgLabels: Readonly<Record<string, string>>;
  readonly dialogLabels: Readonly<Record<string, string>>;
  readonly introductionLabels: Readonly<Record<string, string>>;
  readonly groupLabels: Readonly<Record<string, string>>;
};

export const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
  groupLabels: {},
};

/** 🗣️ Rust's `skip_serializing_if` omits empty maps entirely, so a parsed overlay may be missing keys — fill them back in. */
export function normalizeAppLabelsOverlay(raw: Partial<PluginAppLabelsOverlay> | null | undefined): PluginAppLabelsOverlay {
  return {
    windowKindLabels: raw?.windowKindLabels ?? {},
    panelTabLabels: raw?.panelTabLabels ?? {},
    modeLabels: raw?.modeLabels ?? {},
    actionLabels: raw?.actionLabels ?? {},
    utilityLabels: raw?.utilityLabels ?? {},
    exampleLabels: raw?.exampleLabels ?? {},
    actionArgLabels: raw?.actionArgLabels ?? {},
    dialogLabels: raw?.dialogLabels ?? {},
    introductionLabels: raw?.introductionLabels ?? {},
    groupLabels: raw?.groupLabels ?? {},
  };
}

export type ProgramContributionEntry = {
  readonly pluginId: string;
  readonly topicContribution?: TopicContribution;
};

/** 🗂️ Open plugin contribution shape — see Rust `TopicContribution` (`🦀️.rs`) for the full
 * rationale. `topic` reuses the same dot-namespaced vocabulary as a crate's existing
 * `contributes`/`consumes` metadata (e.g. `"flow.extension"`, `"playbook.blockKind"`,
 * `"cad.computer"`); this type does not enumerate topics, each producer/consumer picks its own. */
export type TopicContribution = {
  readonly topic: string;
  readonly payload: unknown;
};

/** 📇️ Topic under which a plugin names the artifact-index document the os shell opens for a hub space; mirrors Rust `SPACE_INDEX_TOPIC`. The framework reads the contribution and never names the plugin that supplies it. */
export const SPACE_INDEX_TOPIC = "os.space-index";

/** 📇️ Payload of {@link SPACE_INDEX_TOPIC}: the dialect of the app that edits the space index and the document schema the shell opens it with — mirrors Rust `SpaceIndexContribution`. */
export type SpaceIndexContribution = {
  readonly dialect: ArtifactDialect;
  readonly documentSchema: string;
};

/** 📥️ Reads the first well-formed {@link SPACE_INDEX_TOPIC} row among a manifest's topic contributions. */
export function spaceIndexOf(topicContributions: readonly TopicContribution[] | undefined): SpaceIndexContribution | undefined {
  for (const row of topicContributions ?? []) {
    if (row.topic !== SPACE_INDEX_TOPIC) continue;
    const payload = row.payload as { readonly dialect?: Partial<ArtifactDialect>; readonly documentSchema?: unknown } | null;
    const dialect = payload?.dialect;
    if (typeof dialect?.artifactKind === "string" && typeof dialect.standard === "string" && typeof dialect.subset === "string" && typeof payload?.documentSchema === "string") {
      return { dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset }, documentSchema: payload.documentSchema };
    }
  }
  return undefined;
}

/** 📚️ One authored example document on the manifest — TS twin of Rust `ExampleDefinition`. It carries
 * the DIALECT it was authored for, never one app id: every surface bound to that dialect (the editor
 * and the viewer alike) opens the same fixtures (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export type ManifestExample = {
  readonly id: string;
  readonly label: string;
  readonly documentJson?: string;
  readonly artifactJson?: string;
  readonly dialect: ArtifactDialect;
};

/** 📚️ THE example-picker predicate — every example authored for `dialect`, in manifest order,
 * deduplicated by id. Twinned by Rust `manifest::examples_for_dialect`, both pinned against
 * `🧫️fixtures/📚️example-picker.json` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function examplesForDialect<E extends { readonly id: string; readonly dialect?: ArtifactDialect }>(examples: readonly E[], dialect: ArtifactDialect): E[] {
  const wanted = dialectCoordinate(dialect);
  const seen = new Set<string>();
  const resolved: E[] = [];
  for (const example of examples) {
    if (!example.dialect || dialectCoordinate(example.dialect) !== wanted || seen.has(example.id)) continue;
    seen.add(example.id);
    resolved.push(example);
  }
  return resolved;
}

/** 🏷️ One asset-name component — everything outside `[0-9A-Za-z._-]` folds to `-`. TS twin of Rust
 * `describe::asset_name_segment`. */
function assetNameSegment(raw: string): string {
  return raw.replace(/[^0-9A-Za-z._-]/g, "-");
}

/** 📦️ The directory-and-stem prefix an externalized example body travels under — TS twin of Rust
 * `manifest::example_body_asset_prefix`. The suffix after it names the BYTES declared: `.json` for a
 * body `describe::externalize_oversized_example_bodies` moved out of the manifest, and the authored
 * fixture's own extension for a body the plugin deferred and never materialised
 * (`ExampleSource::deferred`). Match the prefix, never one fixed extension. */
export function exampleBodyAssetPrefix(example: { readonly id: string; readonly dialect: ArtifactDialect }): string {
  return `📚️examples/${assetNameSegment(example.dialect.artifactKind)}.${assetNameSegment(example.dialect.standard)}.${assetNameSegment(example.dialect.subset)}/${assetNameSegment(example.id)}`;
}

/** 📦️ The `AssetDeclaration.name` an externalized INLINE example body travels under — TS twin of
 * Rust `describe::externalize_oversized_example_bodies`. A descriptor never inlines a body over
 * `DESCRIPTOR_INLINE_EXAMPLE_MAX_BYTES` (256 KiB); it keeps the row and declares the body as this
 * asset instead, so the 4 MiB descriptor bound cannot be breached by an authored document. */
export function exampleBodyAssetName(example: { readonly id: string; readonly dialect: ArtifactDialect }): string {
  return `${exampleBodyAssetPrefix(example)}.json`;
}

/** 📚️ The examples one surface may offer — {@link examplesForDialect} against that surface's own
 * dialect. Role plays no part: an editor and its viewer are two surfaces of ONE dialect and offer
 * exactly the same picker. */
export function examplesForApp<E extends { readonly id: string; readonly dialect?: ArtifactDialect }>(examples: readonly E[], app: { readonly dialect?: ArtifactDialect }): E[] {
  return app.dialect ? examplesForDialect(examples, app.dialect) : [];
}

export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly Record<string, unknown>[];
  readonly workflows: readonly {
    readonly workflowStepId: string;
    readonly appId: string;
    readonly label: string;
    readonly breadcrumb?: readonly string[];
    readonly yields: string;
  }[];
  readonly examples: readonly ManifestExample[];
  /** 🗂️ Open plugin contributions — see `TopicContribution`. */
  readonly topicContributions?: readonly TopicContribution[];
  /** 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused. */
  readonly commands?: readonly CommandDefinition[];
};
//#endregion PluginRuntime
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest
