/** 📚️ The plugin catalog: catalog and playground targets, the host config, the activation-event prefixes and the artifact-kind activation owner.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { type PackageAssetDeclaration } from "../📦️package-assets/🟦️.ts";

// #region 🎠️Kernel
//#region 🗂️PluginCatalog
/** 🗂️ Framework-owned mirror of the OS product's generated `PluginBuildTarget` row — kept
 * shape-compatible so `🛍️products/💻️os/…/🟦️.ts` can build one straight off the generated array
 * without a mapping layer drifting out of sync. */
export type PluginCatalogTarget = {
  readonly pluginId: string;
  readonly wasmOut: string;
  readonly role: "plugin" | "extension";
  readonly capabilities?: readonly string[];
  readonly extends?: string;
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
  /** 🔗️ Direct RUNTIME plugin dependency ids — the sibling plugins whose own actor this one needs
   * loaded beside it, declared in `[package.metadata.semio].depends-on` (`extends` target first for an
   * extension) and mirrored by the builder's `.depends_on(id, VersionPin)`. A Cargo `[dependencies]`
   * link on another plugin crate is a build-time rlib link and is NOT one of these. Mirrors the
   * generated `PluginBuildTarget.dependsOn` (ticket
   * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS §W2-C report). No
   * pin travels with these (the registry's pre-build view has none to derive it from) —
   * `resolvePlaygroundBoot` maps each id to a versionless edge, which is enough for
   * {@link PluginGraph} to validate presence/cycles and compute load order. */
  readonly dependsOn?: readonly string[];
  /** 🎬️ The declared activation events in `📓️design-abi.md` §2's dash-separated string form
   * (`on-artifact-kind:<kind>`, `on-command:<id>`, `on-file-type:<ext>`, …). Read by
   * {@link artifactKindActivationOwner}: the host resolves which plugin opens an artifact kind from
   * these rows alone, with no manifest and no wasm module loaded. */
  readonly activationEvents?: readonly string[];
  /** 📦️ The package assets this plugin's descriptor declares; the registry fetches, verifies and delivers them on `instance-open`. */
  readonly assets?: readonly PackageAssetDeclaration[];
};

/** 🗂️ Framework-owned mirror of the OS product's generated `PlaygroundBuildTarget` row — only the
 * columns the kernel's playground resolvers actually read. */
export type PlaygroundCatalogTarget = {
  readonly variant: string;
  readonly pluginId: string;
  readonly app?: string;
  readonly aliases: readonly string[];
};

/**
 * 🗂️ Everything the kernel's plugin/playground resolvers need, injected by the caller instead of
 * imported from a specific product's generated build output — inverts the upward dependency a generic
 * framework module must never have on a product's build artifacts. The OS product's
 * `🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` is the one place allowed to import the generated
 * registry and build this shape; every other product wanting kernel resolvers builds its own.
 */
export interface PluginCatalog {
  readonly plugins: readonly PluginCatalogTarget[];
  readonly extensions: readonly PluginCatalogTarget[];
  readonly hosts: readonly PluginHostConfig[];
  readonly playgrounds: readonly PlaygroundCatalogTarget[];
  moduleUrl(pluginId: string): string;
  extensionModuleUrl(pluginId: string): string;
}

/** 🎬️ The `on-artifact-kind:` prefix `📓️design-abi.md` §2 gives `ActivationEvent::OnArtifactKind`. */
export const ON_ARTIFACT_KIND_ACTIVATION_PREFIX = "on-artifact-kind:";

/** 🧩️ The `on-extension-request:` prefix `📓️design-abi.md` §2 gives
 * `ActivationEvent::OnExtensionRequest` — its payload is the extension point that pulled the actor
 * up, which for the `registerCatalog` cascade is the parent plugin id. */
export const ON_EXTENSION_REQUEST_ACTIVATION_PREFIX = "on-extension-request:";

/**
 * 🎬️ The plugin an artifact kind activates, from declared activation events alone — the one
 * resolution path that works BEFORE the owner's manifest exists, which is exactly the hub case:
 * {@link AppRouter} can only route kinds whose plugin is already loaded, so a `s` session that has
 * never touched `cad` has no router entry for `s.cad.cad` and {@link resolveOpeningApp} throws
 * `surface.unknown-dialect`. Catalog rows carry both kind namespaces a descriptor declares (the
 * crate's own `ArtifactKindSpec.id` and the artifact kind its app surfaces name), each claimed by
 * exactly one owner, so this answers "install which plugin?" for either spelling. Ties — two
 * unrelated crates claiming one kind — resolve to the first by ascending `pluginId`, matching the
 * router's own deterministic ordering rather than picking arbitrarily.
 */
export function artifactKindActivationOwner(catalog: PluginCatalog, artifactKind: string): string | undefined {
  if (artifactKind === "") return undefined;
  const event = `${ON_ARTIFACT_KIND_ACTIVATION_PREFIX}${artifactKind}`;
  let owner: string | undefined;
  for (const target of [...catalog.plugins, ...catalog.extensions]) {
    if (!(target.activationEvents ?? []).includes(event)) continue;
    if (owner === undefined || target.pluginId.localeCompare(owner) < 0) owner = target.pluginId;
  }
  return owner;
}
//#endregion 🗂️PluginCatalog

// #region 🎮️PlaygroundResolution
//#region 🏠️🧳️PluginHostConfig
/** 🏠️🧳️ Declares, for a plugin whose manifest offers a host-style multi-app experience (one app is the
 * landing/default view, another hosts other apps as spawned sub-instances — e.g. the `space` plugin's home/studio
 * pair), which app ids play which role. Callers resolve controller ids and default panel tabs from
 * the *loaded manifest*'s own `controllerId`/`panelTabs` on those apps rather than hardcoding separate
 * literals — this table only ever needs to carry app-id role assignments. A pluginFilter absent here
 * simply boots through the ordinary single-app path (`resolvePlaygroundDefaultAppId`). Mirrored by
 * `PLUGIN_HOST_CONFIGS`/`resolve_plugin_host_config` in `framework/os/renderer/wgpu/rs/lib.rs`'s
 * `program_bridge` module for the WGPU renderer. */
export type PluginHostConfig = {
  readonly pluginId: string;
  readonly landingAppId: string;
  readonly hostAppId: string;
};
//#endregion 🏠️🧳️PluginHostConfig
// #endregion 🎮️PlaygroundResolution
// #endregion 🎠️Kernel
