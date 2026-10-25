//#region 🔖️PlaygroundEntry
import type { PlaygroundNativeHostV1 } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";

/** 🗂️ One `[[package.metadata.semio.assets]]` row: a dev-time asset-serving need declared by a
 * plugin crate. `app` optionally scopes the row to one playground variant of a multi-app crate (unset
 * ⇒ every variant of the crate). Mirrors the TS discriminated union emitted for consumers as
 * `AssetDeliveryDeclarationV1` (see `emitPlaygroundsTypeScript`). */
export type AssetSpecRow = {
  readonly kind: "tile-proxy" | "static-dir" | "mesh-collection";
  readonly route: string;
  readonly app?: string;
  readonly upstream?: string;
  readonly cache?: string;
  readonly userAgent?: string;
  readonly root?: string;
  readonly catalog?: string;
};


/** 🎮️ One `[[package.metadata.semio.playground]]` row scoped to its owning plugin crate. */
export type PlaygroundEntry = {
  readonly variant: string;
  readonly catalogDefault?: boolean;
  readonly pluginId: string;
  readonly cratePath: string;
  readonly app?: string;
  /** 🏷️ Shell brand id (see `framework/os/dev/brand`) this variant ships as. */
  readonly brand?: string;
  readonly devContribution?: string;
  readonly compositionConfigPath?: string;
  readonly nativeHost?: PlaygroundNativeHostV1;
  readonly mcpHost?: PlaygroundNativeHostV1;
  /** 📦️ Repo-root-relative CDN output directory for `build-<variant>-react-release` instead of framework-os-dev `dist/build-…`. */
  readonly distDir?: string;
  readonly aliases: readonly string[];
  readonly ports: { readonly react: number; readonly wgpu: number };
  /** 👥️ Extra per-user dev ports for a multi-user collaborative session (e.g. hub-backed `s`
   * studio dev servers) — one port per concurrent user, over and above the single-user `ports` row. */
  readonly userPorts?: { readonly react: readonly number[]; readonly wgpu: readonly number[] };
  /** 🔗️ The hub a collaborative session of this variant joins (`hub = "…"`); the dashboard offers it as the `hub` flag. */
  readonly hub?: string;
  /** 💾️ Repo-root-relative directory the variant keeps its documents in (`data_dir`), and per user slot (`user_data_dir`, `{N}` is the slot). */
  readonly dataDir?: string;
  readonly userDataDir?: string;
  /** 🔒️ The variant can run without any hub (`local_only = true`). */
  readonly localOnly?: boolean;
  /** 👁️ The path a browser opens for the viewer role (`viewer_path`). */
  readonly viewerPath?: string;
  readonly examples: readonly string[];
  /** 🔌️ Crate paths whose `wasm` build target must run for this playground variant. */
  readonly engines: readonly string[];
  /** 🗂️ Dev-time asset-serving needs for this variant. */
  readonly assets: readonly AssetSpecRow[];
};
