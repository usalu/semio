import { artifactKindChoices, decodeArtifactKindChoice, decodeSurfaceAppChoice, encodeArtifactKindChoice, encodeSurfaceAppChoice } from "./🔁️host-effects-choices-and-fault-notices/🟦️.ts";
export type { WindowStackCorner, WindowLayoutWindowNode, WindowLayoutStackNode, WindowLayoutAxisNode, WindowLayout } from "@semio-tech/framework-onboarding/window-layout";
export { surfaceAppId, parseSurfaceAppId, type AppRole, type AppRef } from "./🧬️schema/🟦️.ts";

// #region 🛂️Manifest
/// <reference types="vitest/importMeta" />
/** 🛂️ `@semio-tech/framework` — AppDefinition, PluginManifest, contributions, and declarative UI contract. */
import type { IconName } from "@semio-tech/assets";
export type { IconName };
import { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "./🤖️generated/🎚️ui-axes/🟦️.ts";
export { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology };
export type { ShellLocale, ShellTerminology, LocalizedLabel };
// 🧭️ `ContextMenuItemSpec`/`Effect` are hand-written types owned by sibling modules aggregated
// alongside this one into `@semio-tech/framework` (see `🟦️.ts`) — that aggregation only helps
// EXTERNAL consumers of the package; this file's own internal references (`PluginContextMenuResponse`,
// `PluginUiRefreshResponse`) still need a real import, type-only so the cycle back through
// `🖱️ui/🎬️scene/🟦️.ts`'s own `ActionDescriptor` import from this file erases cleanly.

// #region 🧬️GeneratedMirror
/** 🧬️ Types generated from `framework/core/rs/lib.rs` via the owned schema exporter (`bun repo run @semio-tech/framework:generate`); re-exported below alongside their hand-written neighbors so this stays the one import surface. */

// #endregion 🧬️GeneratedMirror
//#region 🔌️PluginAndAppContract
//#region PluginRuntime
//#region 🔖️HostResolvedArgs
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️hostresolvedargs/🟦️.ts");
  await registerTests1(import.meta.vitest, { artifactKindChoices, decodeArtifactKindChoice, decodeSurfaceAppChoice, encodeArtifactKindChoice, encodeSurfaceAppChoice }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🔖️HostResolvedArgs

/**
 * 📡️ Host-facing shape of one loaded plugin, mirroring the 5-function `semio:framework/plugin` WIT
 * ABI exactly (`world.wit`): `manifest`/`instantiate-app`(as `createApp`)/`exchange` are the whole
 * runtime surface now — every former per-verb call (`handleAction`, `render`, `refreshUi`,
 * `contextMenu`, ...) is a binary `protocol_channel::AppCommand` sent through {@link exchange}
 * instead (see `🔖️AppChannelClient` in the os-product package, which frames these bytes). `dispose`
 * remains host-side only (never part of the WIT ABI) for worker/resource teardown.
 */
//#endregion PluginRuntime
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest
export type { AccessibilitySpec, ActionDescriptor, BuiltNode, Component, InputKind, LayoutSpec, NamedLayout, PatchRejection, StyleSpec, UiDocumentLimits, UiIntent, UiNodeRecord, UiPatch, UiSnapshot } from "./🏷️ui-contract-and-canvas-types/🟦️.ts";
export { argControl } from "./🎬️action-types-and-semantics/🟦️.ts";
export type { ActionArgDef } from "./🎬️action-types-and-semantics/🟦️.ts";
export { EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID } from "./🔘️definitions-action-ids-and-shell-brand/🟦️.ts";
export { faultNoticeText, fillFaultNotice, isFaultNoticeCode, validateFaultNotices } from "./🔁️host-effects-choices-and-fault-notices/🟦️.ts";
export type { FaultNoticeDefinition } from "./🔁️host-effects-choices-and-fault-notices/🟦️.ts";
export { inputLabelGlossary, inputNumericTransport, inputShape, mutationInputAudit, mutationInputDefs, mutationInputInstance } from "./📥️mutation-input-schema/🟦️.ts";
export type { InputSchemaAudit } from "./📥️mutation-input-schema/🟦️.ts";
export { PUBLIC_INVOCATION_BODY_BYTES, PUBLIC_INVOCATION_STRING_BYTES, hostArmedViewContext, panelViewContext, parseResolvedPluginViewState, publicInvocationStringPages, windowViewContext } from "./👁️view-state-and-public-invocation/🟦️.ts";
export type { PluginViewState, ResolvedPluginViewState } from "./👁️view-state-and-public-invocation/🟦️.ts";
export { UI_REFRESH_SECTIONS, panelTabKindId, sectionViewContext } from "./📋️app-definition-refresh-and-context-menu/🟦️.ts";
export type { PanelTabKind, PluginUiRefreshRequest, PluginUiRefreshResponse } from "./📋️app-definition-refresh-and-context-menu/🟦️.ts";
export { exampleBodyAssetPrefix, examplesForApp } from "./🗣️contributions-and-examples/🟦️.ts";
export type { PluginManifest, ProgramContributionEntry } from "./🗣️contributions-and-examples/🟦️.ts";
