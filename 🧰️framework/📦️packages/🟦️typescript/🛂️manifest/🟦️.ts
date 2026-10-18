/** 🛂️ SDK entry `@semio-tech/framework/manifest`: explicit public surface of the framework module that plugins build on. */
export { SHELL_LOCALES, SHELL_TERMINOLOGIES, isShellLocale, isShellTerminology } from "../../../🔨️modules/🛂️manifest/🟦️.ts";
export { argControl } from "../../../🔨️modules/🛂️manifest/🎬️action-types-and-semantics/🟦️.ts";
export { inputLabelGlossary, inputNumericTransport, inputShape, mutationInputAudit, mutationInputDefs, mutationInputInstance } from "../../../🔨️modules/🛂️manifest/📥️mutation-input-schema/🟦️.ts";
export { EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID } from "../../../🔨️modules/🛂️manifest/🔘️definitions-action-ids-and-shell-brand/🟦️.ts";
export { PUBLIC_INVOCATION_BODY_BYTES, PUBLIC_INVOCATION_STRING_BYTES, hostArmedViewContext, panelViewContext, parseResolvedPluginViewState, publicInvocationStringPages, windowViewContext } from "../../../🔨️modules/🛂️manifest/👁️view-state-and-public-invocation/🟦️.ts";
export { dialectFromLegacyExampleAppId, exampleBodyAssetPrefix, examplesForApp, normalizeManifestExampleRow, normalizeManifestExamples } from "../../../🔨️modules/🛂️manifest/🗣️contributions-and-examples/🟦️.ts";
export { faultNoticeText, fillFaultNotice, isFaultNoticeCode, validateFaultNotices } from "../../../🔨️modules/🛂️manifest/🔁️host-effects-choices-and-fault-notices/🟦️.ts";
export { UI_REFRESH_SECTIONS, panelTabKindId, sectionViewContext } from "../../../🔨️modules/🛂️manifest/📋️app-definition-refresh-and-context-menu/🟦️.ts";
export type { LocalizedLabel, ShellLocale, ShellTerminology, WindowLayout } from "../../../🔨️modules/🛂️manifest/🟦️.ts";
export type { AccessibilitySpec, ActionDescriptor, BuiltNode, Component, InputKind, LayoutSpec, NamedLayout, PatchRejection, StyleSpec, UiDocumentLimits, UiIntent, UiNodeRecord, UiPatch, UiSnapshot } from "../../../🔨️modules/🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts";
export type { ActionArgDef } from "../../../🔨️modules/🛂️manifest/🎬️action-types-and-semantics/🟦️.ts";
export type { InputSchemaAudit } from "../../../🔨️modules/🛂️manifest/📥️mutation-input-schema/🟦️.ts";
export type { PluginViewState, ResolvedPluginViewState } from "../../../🔨️modules/🛂️manifest/👁️view-state-and-public-invocation/🟦️.ts";
export type { PluginManifest, ProgramContributionEntry } from "../../../🔨️modules/🛂️manifest/🗣️contributions-and-examples/🟦️.ts";
export type { FaultNoticeDefinition } from "../../../🔨️modules/🛂️manifest/🔁️host-effects-choices-and-fault-notices/🟦️.ts";
export type { PanelTabKind, PluginUiRefreshRequest, PluginUiRefreshResponse } from "../../../🔨️modules/🛂️manifest/📋️app-definition-refresh-and-context-menu/🟦️.ts";
