import utilityActionPolicy from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧰️utility-action-policy/🔣️.json";
import utilityAssignmentCases from "../../🧱️elements/🐚️Shell/🧰️utility-assignment/🧫️fixtures/🔣️.json";
import initialWindowUtilityCases from "../../../../../../../🔨️modules/🛂️manifest/🪛️utilities/🌅️initial/🧫️fixtures/🔣️.json";

import drawingActionCases from "../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🎬️actions/🧫️fixtures/🔣️.json";
import drawingActionSchema from "../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🎬️actions/🧬️schema/🔣️.json";
import drawingInterruptionCases from "../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🎬️actions/🧫️fixtures/🛑️interruption/🔣️.json";
import drawingInterruptionSchema from "../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📍️point/🧬️schema/🔣️.json";
import { resolveUtilityActivationV1 as resolveAssignmentPress } from "../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import chromePanelSafeAreaFixture from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🛟️chrome-panel-safe-area/🔣️.json";
import { shouldStartIntroduction } from "../../../../../../../🔨️modules/🖱️ui/🎓️introduction/🟦️.ts";
import { ChromeAwareWindowScrollSurface } from "@semio-tech/ui-react";
import canvasClearance from "../../🧱️elements/🗣️Interpreter/🧫️fixtures/🪟️canvas-clearance/🔣️.json";
import { boardTestSession } from "../../🧱️elements/🪪️WasmSessionLoader/🔮️oracles/🪪️session-double/🟦️.ts";
import vfsDescriptorFixture from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🧫️fixtures/🧾️descriptors/🔣️.json";
import gumballTargetsFixture from "../../🧱️elements/🌐️World3dHost/🧫️fixtures/🧭️gesture-targets.json";
import gumballTargetsSchema from "../../🧱️elements/🌐️World3dHost/🧬️schema/🧭️gesture-targets/🔣️.json";
import gumballLiveProtocolFixture from "../../🧱️elements/🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json";
import { worldGumballStep, WORLD_GUMBALL_IDLE, type WorldGumballEvent, type WorldGumballTargets } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";
import { renderVirtualFileSystemDescriptorCell, type DescriptorKind, type FileNodeDescriptorValue } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🟦️.tsx";
import gizmoTipBoundsFixture from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️gizmo-tip-bounds/🔣️.json";
import textInputFixture from "../../../../../../../🔨️modules/✍️editor/🧫️fixtures/⌨️text-input/🔣️.json";
import editorDeliveryFixture from "../../🧱️elements/✏️TextEditor/🧫️fixtures/📮️delivery/🔣️.json";
import { act as reactAct, createElement, useLayoutEffect, useState, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { flushSync } from "react-dom";
import type { BackboneWorkerResponse } from "@semio-tech/framework-os";
import { applyPatch } from "fast-json-patch";
import {
  COMPACT_UI_DRIVER,
  DEFAULT_UI_DRIVER,
  Layout,
  buildVirtualFileSystemSceneRows,
  TreeContext,
  TreeItem,
  UIDialog,
  UiDriverProvider,
  chromePanelSafeArea,
  childElementId,
  closestCenter,
  createTutorialClock,
  deriveTreeDragRoles,
  isElementId,
  singleTreeLeaf,
  treeDataActivation,
  detectShellLocale,
  uiI18n,
  type Anchor,
  type SafeAreaYield,
} from "@semio-tech/ui-react";
import { createWorldProjectionTemplates, worldCameraReportTargetV1, worldProjectionSwitchTreeItems } from "@semio-tech/infinite-world-r3f";
import { createLocalInteractionStoreV1, resolvePluginCanvasStatus, type PluginSupervisorState } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import bootCanvasFixture from "../../🧱️elements/🐚️Shell/🧫️fixtures/🔣️.json";
import windowIconOverrideFixture from "../../🧱️elements/🐚️Shell/🧫️fixtures/🪟️window-icon-overrides/🔣️.json" with { type: "json" };

import {
  dispatchInvokeExtensionEffect,
  RUNTIME_DIAGNOSTICS_KEY,
  runtimeDiagnosticsEnabled,
  setRuntimeDiagnostics,
  artifactKindChoiceDraftRetirementsV1,
  captureSpaceArtifactCreationCatalogAuthorityV1,
  runInvokeExtensionEffect,
  spaceArtifactCreationCatalogRefreshRequestV1,
  spaceArtifactCreationOwnerAcceptsStatus,
  spaceArtifactCreationReadyOpening,
  spaceArtifactCreationRequestFromAction,
  selectedSpaceArtifactCreationCatalogV1,
  tutorialInteractionSelectionActions,
  createBuiltNodeStoreCacheV1,
  publishBuiltNodesV1,
  publishPanelBodiesV1,
  type SpaceArtifactCreationCatalogAuthorityV1,
  type SpaceArtifactCreationOwnerV1,
} from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import {
  DOWNLOAD_MEDIA_EXPORT_REVOKE_MS,
  EMPTY_APP_LABELS_OVERLAY,
  SET_ACTIVE_EXAMPLE_ACTION_ID,
  appSwitchesExamples,
  appOffersRegisteredExamples,
  frameworkOwnsExampleSwitch,
  buildActiveExampleAction,
  navbarExampleIdFromHistoryUpserts,
  rememberedExampleIdFromDispatchV1,
  interactionViewFromLeftoverOutput,
  leftoverInteractionStateV1,
  leftoverWorldGumballPoseV1,
  historyPatchShouldApplyV1,
  historyRefreshNeededV1,
  undeclaredActionDiagnostic,
  downloadMediaExport,
  naturalFileFormatV1,
  naturalFileNameV1,
  naturalMediaDescriptorMatchesV1,
  openNaturalFileOwnerV1,
  readBlobBytesBoundedV1,
  requestFileSelectionV1,
  mediaExportEncodingText,
  makeEffectDispatchOne,
  renderStagedArgControl,
  resolveDialogDefinition,
  world3dMarqueeOverlayShape,
  windowMeasuresChrome,
  windowMeasureDomId,
  qualifyWindowMeasureIds,
  automaticCheckinWaitsV1,
} from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import { FRAMEWORK_HISTORY_BODY_KEY, OPEN_ARTIFACT_FILE_ACTION_ID, SAVE_ARTIFACT_FILE_ACTION_ID, resolveUiDirtyScope, type UiDirtyScope } from "@semio-tech/framework";
import { hostArmedViewContext, panelViewContext, parseResolvedPluginViewState, windowViewContext } from "../../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import { world3dComputeStatusV1 } from "../../../../../../../🔨️modules/🖱️ui/🎬️scene/🟦️.ts";
import surfaceControlsFixture from "../../🧱️elements/🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json";
import bootExampleFixture from "../../🧱️elements/🐚️Shell/🧫️fixtures/📚️boot-example/🔣️.json";
import exampleOfferFixture from "../../🧱️elements/🐚️Shell/🧫️fixtures/📚️example-offer/🔣️.json";
import exampleOfferSchema from "../../🧱️elements/🐚️Shell/📚️example-offer/🔣️.json";
import { BOOT_QUERY_CAPACITY, BOOT_QUERY_EXAMPLE_PARAM, resolveBootQueryExampleId } from "../../../../🧑‍💻dev/🔗️boot-query/🟦️.ts";
import { createContinuationScheduler } from "../../../../../../../🔨️modules/⏳️async/🪃️continuation/🟦️.ts";
import { createVirtualContinuationHost } from "../../../../../../../🔨️modules/⏳️async/🪃️continuation/🧪️tests/⚖️oracle/🟦️.ts";
import { SHARD_RUNTIME_DIAGNOSTICS_KEY, SHARD_WORKER_DIAGNOSTICS_PARAM, SHARD_WORKER_URL, shardWorkerUrl } from "../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts";
/** 🗣️ The resolver every `makeEffectDispatchOne` owner passes — the shell's `resolvedTargetViewState`, reduced to what a test session needs: both host preferences always stamped. */
const resolvedViewStateFixture = (session: { readonly viewState?: unknown }) => parseResolvedPluginViewState({ ...(session.viewState as Record<string, unknown> | undefined), locale: "en", terminology: "native" }) as never;
import { openSurfaceContextMenu, uiNodeDomId, VirtualFileSystemHost, virtualFileSystemNavigation } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";
import { contextMenuItemClassName } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx";
import type { LoadedProgramState } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import extensionInvocationFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json";
import extensionInvocationWireFixture from "../../../../🌊️flow/🧩️extensions/🕸️wasm/🧫️fixtures/🔁️extension-invocation-wire/🔣️.json";
import rendererSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import directorySchema from "../../../../📇️directory/🧬️schema/🔣️.json" with { type: "json" };
import { type ValidateFunction } from "ajv";
import Ajv2020 from "ajv/dist/2020";
import automaticCheckinCorpus from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧫️automatic-checkin/🔣️.json";
import deepEqual from "fast-deep-equal";
import viewport2dSchema from "../../../../../../../🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🔣️.json";
import viewportPoseFixture from "../../../../../../../🔨️modules/🖱️ui/🪟️viewport/🧫️fixtures/🪟️poses/🔣️.json";
import treeDragHandleFixture from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-drag-handles/🔣️.json";
import sceneListTransferFixture from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🔀️scene-list-transfer/🔣️.json" with { type: "json" };
import tableStepperKeyboardFixture from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/⌨️table-stepper-keyboard/🔣️.json" with { type: "json" };
import sliderPresentationFixture from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🎚️slider-presentation/🔣️.json" with { type: "json" };
import graphTimelineAuthorsFixture from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/👥️graph-timeline-authors/🔣️.json" with { type: "json" };
import virtualFileSystemInteractionFixture from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/📁️virtual-file-system-interaction/🔣️.json" with { type: "json" };
import { BlockListHost } from "../../🧱️elements/🧩️BlockListHost/🟦️.tsx";
import dialogOriginFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🔣️.json";
import { createAdmittedShellInstanceV1, shellDialogOriginIsCurrentV1, shellDialogOriginV1, shellEffectOwnerIsCurrentV1, shellEffectSourceIsCurrentV1, type ShellDialogOriginV1 } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🟦️.ts";
import admittedInstanceFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🛂️admission/🔣️.json";
import artifactCreationProgressFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🔣️.json";
import replayRefusalVocabulary from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🔣️.json";
import { REPLAY_REFUSAL_LABELS_V1, replayRefusalCodeV1, replayRefusalNoticeTextV1, type ReplayRefusalReasonV1 } from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts";
import artifactCreationCatalogAuthorityFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🪪️catalog-authority/🔣️.json";
import artifactCreationReadyOpeningFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🚪️ready-opening/🔣️.json";
import { runArtifactCreationReadyOpeningV1 } from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import { runDocumentOpeningAttemptV1 } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import {
  ARTIFACT_CREATION_PROGRESS_CAPACITY,
  ARTIFACT_CREATION_PROGRESS_TEXT_V1,
  ArtifactCreationCatalogNotice,
  ArtifactCreationProgressNotice,
  artifactCreationProgressLocaleV1,
  artifactCreationProgressRoleV1,
  artifactCreationProgressTerminalV1,
  reduceArtifactCreationProgressUiV1,
  type ArtifactCreationProgressOwnerV1,
} from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🟦️.tsx";
import { OwnedShellDialog, type OwnedShellDialogProps } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🌐️browser/🟦️.tsx";
import tutorialRunFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/🔣️.json";
import { OwnedTutorialRunV1, TutorialDriveV1, runPausedTutorialSeekV1 } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🎥️tutorial/🟦️.ts";
import tutorialSeekFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/⏩️seek/🔣️.json";
import tutorialSerialFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/🧵️serial/🔣️.json";
import descriptorLoadFixture from "../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/📇️descriptor-load/🔣️.json";

import { createInstance as createTranslationOracle } from "i18next";
import chordKeyTokensFixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/⌨️chord-key-tokens.json";
import labelResolutionFixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🔣️label-resolution.json";
import tutorialInteractionFixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🎥️tutorial-interaction/🔣️.json";
import pluginAvailabilityRouteFixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🔁️plugin-availability-route/🔣️.json";
import naturalFileLifecycleFixture from "../../../../🔌️plugin/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json" with { type: "json" };
import interactionSchema from "../../../../../../../🔨️modules/🕹️interaction/🧬️schema/🔣️.json";
import type { InteractionState } from "../../../../../../../🔨️modules/🕹️interaction/🟦️.ts";
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";

import actionSemanticsFixture from "../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/⚖️action-semantics.json";
import examplePickerFixture from "../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/📚️example-picker.json";
import tutorialDocumentFixture from "../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🎞️tutorial-document-track.json";
import { tutorialSlice, validateTutorial } from "@semio-tech/ui-react";
import type { DialogDefinition, TutorialDefinition, TutorialUiChange, TutorialUiSnapshot } from "@semio-tech/framework";
import presenceOverlayFixture from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/👥️presence-overlay.json";

import { createRequire } from "node:module";
import * as THREE from "three";

import world3dLightingFixture from "../../../../♾️infinite/🌍️world/🧫️fixtures/🌞️scene-lighting/🔣️.json" with { type: "json" };
import world3dShadowFixture from "../../../../♾️infinite/🌍️world/🧫️fixtures/🌑️scene-shadows/🔣️.json" with { type: "json" };
import world3dShadowParityFixture from "../../../../♾️infinite/🌍️world/🧫️fixtures/🌑️scene-shadow-parity/🔣️.json" with { type: "json" };
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import { decodeLocalInteractionCaptureJson, LOCAL_INTERACTION_CAPTURE_MAX_BYTES } from "@semio-tech/framework-replication";
import { unresolvedActionArgs } from "@semio-tech/framework";
import { examplesForApp, examplesForDialect, surfaceAppId, type AppRole } from "@semio-tech/framework";
import choiceFixture from "../../../../../../../🔨️modules/🧩️action-argument-resolution/🧫️fixtures/🔽️choices/🔣️.json";

import { semioSchemaAjvV1 } from "../../../../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

const ownedExports = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(rendererSchema).addSchema(directorySchema);
/** 🧬️ Compiles one named export of the `os.renderer` schema module. */
/** 🧬️ Compiles one named `$defs` export of a peer scope's `🧬️schema/` module. */
const peerExport = (module: { $id: string }, exportId: string): ValidateFunction => semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(module).getSchema(`${module.$id}#/$defs/${exportId}`) as ValidateFunction;
const rendererExport = (exportId: string): ValidateFunction => ownedExports.getSchema(`${rendererSchema.$id}#/$defs/${exportId}`) as ValidateFunction;
/** 🧬️ Compiles one named export of the `os.directory` schema module. */
const directoryExport = (exportId: string): ValidateFunction => ownedExports.getSchema(`${directorySchema.$id}#/$defs/${exportId}`) as ValidateFunction;

const { computeAccessibleName }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");

describe("catalog-resolved artifact creation kinds", () => {
  it("matches the independent enum validator for unresolved required and host-resolved choices", () => {
    const ajv = semioSchemaAjvV1({ strict: true });
    
    for (const row of choiceFixture.cases) {
      const def: ActionArgDef = {
        id: "kindChoice",
        label: "Kind",
        required: row.required,
        schema: {
          kind: "string",
          options: row.options.map((value) => ({ value, label: value })),
          ...(row.format === "text" ? {} : { format: row.format === "artifactKind" ? { kind: "artifactKind", roles: ["editor"] } : { kind: "surfaceApp", roles: ["editor"], dialectArg: "dialect" } }),
        },
      };
      const data = row.value === null || row.value === "" ? {} : { kindChoice: row.value };
      const property = row.options.length > 0 ? { enum: row.options } : row.format === "text" ? {} : false;
      const oracle = ajv.compile({ type: "object", required: row.required ? ["kindChoice"] : [], properties: { kindChoice: property } });
      expect(!oracle(data), row.id).toBe(row.unresolved);
      expect(unresolvedActionArgs([def], data), row.id).toEqual(row.unresolved ? ["kindChoice"] : []);
    }
  });
  const localized = (en: string, de: string) => ({ native: { en, de }, reuse: { en, de } });
  const dialog: DialogDefinition = {
    id: "createArtifact",
    title: localized("Create Artifact", "Artefakt erstellen"),
    args: [{ id: "kindChoice", label: localized("Kind", "Art"), required: true, schema: { kind: "string", options: [], format: { kind: "artifactKind", roles: ["editor"] } } }],
    submitAction: "createArtifact",
    submitLabel: localized("Create", "Erstellen"),
  };
  const manifests = [
    {
      pluginId: "neutral-host-fixture",
      label: "Fixture",
      version: "1",
      apps: [
        { id: "fixture-editor", role: "editor", dialect: { artifactKind: "fixture.neutral-host-fixture.counter", standard: "1", subset: "*" }, label: localized("Editor", "Editor"), io: { artifactSchema: "fixture.counter" } },
        { id: "fixture-viewer", role: "viewer", dialect: { artifactKind: "fixture.other-owner.counter", standard: "1", subset: "*" }, label: localized("Viewer", "Betrachter"), io: { artifactSchema: "fixture.counter" } },
      ],
      artifactKinds: [{ schema: "fixture.counter", label: localized("Counter", "Zähler") }],
      workflows: [],
      examples: [],
    },
  ];

  it("keeps unavailable catalog feedback inside the dialog and requires a fresh selection after catalog withdrawal", async () => {
    for (const locale of ["en", "de"] as const) {
      await uiI18n.changeLanguage(locale);
      const ready = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", locale, manifests);
      const unavailable = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", locale, manifests, []);
      const submit = vi.fn();
      const kindLabel = locale === "en" ? "Kind" : "Art";
      const kindName = locale === "en" ? "Counter" : "Zähler";
      const submitLabel = locale === "en" ? "Create" : "Erstellen";
      const props = (definition: typeof ready, phase: "ready" | "loading" | "unavailable"): OwnedShellDialogProps<ResolvedActionArgDef> => ({
        owner: { openingId: 1, dialogId: dialog.id, origin: dialogOriginFixture.owner, seedArgs: { kindChoice: "forged" } },
        dialog: definition,
        isCurrent: () => true,
        close: () => true,
        dispatch: submit,
        notice: createElement(ArtifactCreationCatalogNotice, { status: { phase }, locale }),
        renderField: (def, value, change, field) => renderStagedArgControl(def, value, change, false, field),
      });
      const view = render(createElement(OwnedShellDialog<ResolvedActionArgDef>, props(ready, "ready")));
      try {
        expect(view.getByRole("button", { name: submitLabel }).hasAttribute("disabled")).toBe(true);
        for (const phase of choiceFixture.catalogTransitions as ("loading" | "unavailable")[]) {
          fireEvent.click(view.getByRole("combobox", { name: kindLabel }));
          fireEvent.click(view.getByRole("option", { name: kindName }));
          expect(view.getByRole("button", { name: submitLabel }).hasAttribute("disabled")).toBe(false);
          view.rerender(createElement(OwnedShellDialog<ResolvedActionArgDef>, props(unavailable, phase)));
          expect(view.getByRole("dialog").querySelector("input#kindChoice")).toBeNull();
          expect(view.getByRole("combobox", { name: kindLabel }).hasAttribute("disabled")).toBe(true);
          const notice = view.getByRole(phase === "unavailable" ? "alert" : "status");
          expect(view.getByRole("dialog").contains(notice)).toBe(true);
          expect(computeAccessibleName(notice)).toBe(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].catalog[phase]);
          expect(notice.closest('[inert],[aria-hidden="true"]')).toBeNull();
          const button = view.getByRole("button", { name: submitLabel });
          expect(button.hasAttribute("disabled")).toBe(true);
          fireEvent.click(button);
          fireEvent.keyDown(view.getByRole("dialog"), { key: "Enter", ctrlKey: true });
          expect(submit).not.toHaveBeenCalled();
          view.rerender(createElement(OwnedShellDialog<ResolvedActionArgDef>, props(ready, "ready")));
          expect(view.getByRole("button", { name: submitLabel }).hasAttribute("disabled")).toBe(true);
        }
        fireEvent.click(view.getByRole("combobox", { name: kindLabel }));
        fireEvent.click(view.getByRole("option", { name: kindName }));
        fireEvent.click(view.getByRole("button", { name: submitLabel }));
        expect(submit).toHaveBeenCalledTimes(1);
        expect(JSON.parse(submit.mock.calls[0]![2].kindChoice).kindId).toBe("fixture.neutral-host-fixture.counter");
      } finally {
        view.unmount();
      }
    }
  });

  it("retires only choices when their catalog generation changes, even if the same tuple returns", async () => {
    await uiI18n.changeLanguage("en");
    const resolved = resolveDialogDefinition(
      {
        ...dialog,
        args: [
          ...dialog.args,
          { id: "name", label: localized("Name", "Name"), required: true, schema: { kind: "string", options: [] } },
          { id: "color", label: localized("Color", "Farbe"), required: true, schema: { kind: "string", options: [{ value: "blue", label: localized("Blue", "Blau") }] } },
        ],
      },
      EMPTY_APP_LABELS_OVERLAY,
      "native",
      "en",
      manifests,
    );
    const props = {
      dialog: resolved,
      onSubmit: vi.fn(),
      onChoose: vi.fn(),
      onCancel: vi.fn(),
      renderField: (def: ResolvedActionArgDef, value: unknown, change: (value: unknown) => void, field: Parameters<OwnedShellDialogProps<ResolvedActionArgDef>["renderField"]>[3]) => renderStagedArgControl(def, value, change, false, field),
    };
    const view = render(createElement(UIDialog<ResolvedActionArgDef>, { ...props, choiceRevisions: { kindChoice: choiceFixture.generation.before } }));
    try {
      fireEvent.change(view.getByRole("textbox", { name: "Name" }), { target: { value: choiceFixture.generation.text } });
      fireEvent.click(view.getByRole("combobox", { name: "Color" }));
      fireEvent.click(view.getByRole("option", { name: "Blue" }));
      fireEvent.click(view.getByRole("combobox", { name: "Kind" }));
      fireEvent.click(view.getByRole("option", { name: "Counter" }));
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(false);
      view.rerender(createElement(UIDialog<ResolvedActionArgDef>, { ...props, choiceRevisions: { kindChoice: choiceFixture.generation.after } }));
      expect((view.getByRole("textbox", { name: "Name" }) as HTMLInputElement).value).toBe(choiceFixture.generation.text);
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(true);
      fireEvent.click(view.getByRole("combobox", { name: "Kind" }));
      fireEvent.click(view.getByRole("option", { name: "Counter" }));
      fireEvent.click(view.getByRole("button", { name: "Create" }));
      expect(props.onSubmit).toHaveBeenCalledTimes(1);
      expect(props.onSubmit.mock.calls[0]![0].name).toBe(choiceFixture.generation.text);
      expect(props.onSubmit.mock.calls[0]![0].color).toBe(choiceFixture.generation.staticChoice);
    } finally {
      view.unmount();
    }
  });

  it("rejects an already queued submit handler when capture retires its catalog generation", async () => {
    await uiI18n.changeLanguage("en");
    const resolved = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "en", manifests);
    const submit = vi.fn();
    function Harness(): ReactElement {
      const [revision, setRevision] = useState(choiceFixture.generation.before);
      return createElement(
        "div",
        { onKeyDownCapture: () => flushSync(() => setRevision(choiceFixture.generation.after)) },
        createElement(UIDialog<ResolvedActionArgDef>, {
          dialog: resolved,
          choiceRevisions: { kindChoice: revision },
          onSubmit: submit,
          onChoose: () => {},
          onCancel: () => {},
          renderField: (def, value, change, field) => renderStagedArgControl(def, value, change, false, field),
        }),
      );
    }
    const view = render(createElement(Harness));
    try {
      fireEvent.click(view.getByRole("combobox", { name: "Kind" }));
      fireEvent.click(view.getByRole("option", { name: "Counter" }));
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(false);
      fireEvent.keyDown(view.getByRole("dialog"), { key: "Enter", ctrlKey: true });
      expect(submit).toHaveBeenCalledTimes(choiceFixture.generation.staleSubmits);
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(true);
    } finally {
      view.unmount();
    }
  });

  it("names the actual staged kind picker and retains the chosen catalog tuple inside its modal", async () => {
    await uiI18n.changeLanguage("en");
    const resolved = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "en", manifests);
    const submit = vi.fn();
    const cancel = vi.fn();
    const view = render(createElement(UIDialog<ResolvedActionArgDef>, { dialog: resolved, onSubmit: submit, onChoose: vi.fn(), onCancel: cancel, renderField: (def, value, change, field) => renderStagedArgControl(def, value, change, false, field) }));
    try {
      const picker = view.getByRole("combobox", { name: "Kind" });
      expect(computeAccessibleName(picker)).toBe("Kind");
      expect(picker.getAttribute("aria-required")).toBe("true");
      fireEvent.click(picker);
      expect(document.activeElement).toBe(view.getByRole("listbox"));
      fireEvent.click(view.getByRole("option", { name: "Counter" }));
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(false);
      fireEvent.click(view.getByRole("button", { name: "Create" }));
      expect(cancel).not.toHaveBeenCalled();
      expect(submit).toHaveBeenCalledTimes(1);
      expect(JSON.parse(submit.mock.calls[0]![0].kindChoice).kindId).toBe("fixture.neutral-host-fixture.counter");
    } finally {
      view.unmount();
    }
  });

  it("projects only live catalog editor kinds into the ordinary dialog in the requested locale", () => {
    const english = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "en", manifests);
    const german = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "de", manifests);
    expect(english.args[0]?.schema).toMatchObject({ kind: "string", options: [{ label: "Counter" }] });
    expect(german.args[0]?.schema).toMatchObject({ kind: "string", options: [{ label: "Zähler" }] });
    const option = english.args[0]?.schema.kind === "string" ? english.args[0].schema.options[0] : undefined;
    expect(option?.value).toBe('{"kindId":"fixture.neutral-host-fixture.counter","schema":"fixture.counter","dialect":{"artifactKind":"fixture.neutral-host-fixture.counter","standard":"1","subset":"*"},"label":{"en":"Counter","de":"Zähler"}}');
    expect(JSON.parse(option?.value ?? "null").kindId).toBe("fixture.neutral-host-fixture.counter");
    expect(JSON.stringify(english)).not.toContain("fixture.other-owner.counter");
  });

  it("uses only the selected trusted catalog projection and fails closed while it is unavailable", () => {
    const selected = [{ kindId: "fixture.neutral-host-fixture.counter", schema: "fixture.neutral-host-fixture.counter", dialect: { artifactKind: "fixture.neutral-host-fixture.counter", standard: "1", subset: "any" }, label: { en: "Shared Counter", de: "Gemeinsamer Zähler" } }];
    const exact = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "de", manifests, selected);
    const unavailable = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "en", manifests, []);
    expect(exact.args[0]?.schema).toMatchObject({ kind: "string", options: [{ label: "Gemeinsamer Zähler" }] });
    expect(JSON.stringify(exact)).not.toContain("fixture.other-owner.counter");
    expect(unavailable.args[0]?.schema).toMatchObject({ kind: "string", options: [] });
  });
});

describe("Shell dialog origin", () => {
  it("serializes a seek behind the admitted director write and drains before retirement completes", async () => {

    for (const row of tutorialSerialFixture.cases) {
      const drive = new TutorialDriveV1();
      let owner = true;
      let cursor = 0;
      const writes: string[] = [];
      let resolve!: () => void;
      const gate = new Promise<void>((done) => {
        resolve = done;
      });
      const director = drive.enqueue(
        () => owner,
        async (token) => {
          writes.push("M");
          await gate;
          if (owner && drive.accepts(token)) cursor = 100;
        },
      );
      await Promise.resolve();
      expect(writes).toEqual(["M"]);
      const reconcile = async (token: number) => {
        expect(cursor).toBe(100);
        if (owner && drive.accepts(token)) {
          writes.push("inverse-M");
          cursor = 0;
        }
      };
      const stale = vi.fn(async () => {});
      const seek = drive.enqueue(() => owner, row.replace ? stale : reconcile);
      const latest = row.replace ? drive.enqueue(() => owner, reconcile) : Promise.resolve();
      if (row.close) {
        owner = false;
        drive.retire();
      }
      let drained = false;
      const drain = drive.drain().then(() => {
        drained = true;
      });
      await Promise.resolve();
      expect(drained).toBe(false);
      expect(writes).toEqual(["M"]);
      expect(cursor).toBe(0);
      resolve();
      await drain;
      expect(deepEqual({ writes, cursor }, { writes: row.writes, cursor: row.cursor })).toBe(true);
      expect(drive.busy).toBe(false);
      await Promise.all([director, seek, latest]);
      expect(stale).not.toHaveBeenCalled();
      expect(deepEqual({ writes, cursor }, { writes: row.writes, cursor: row.cursor })).toBe(true);
      expect(drive.busy).toBe(false);
    }
  });

  it("restores the original tutorial snapshot only after the admitted write drains", async () => {
    const drive = new TutorialDriveV1();
    let documentValue = 0;
    let resolve!: () => void;
    const gate = new Promise<void>((done) => {
      resolve = done;
    });
    const restore = vi.fn(async () => {
      documentValue = 0;
    });
    const run = new OwnedTutorialRunV1("tour", dialogOriginFixture.owner, () => true, {
      read: async () => ({ pack: new Uint8Array([0]), spr: new Uint8Array([0]) }),
      drain: () => drive.drain(),
      restore,
    });
    await run.start();
    const write = drive.enqueue(
      () => run.ready,
      async () => {
        await gate;
        documentValue = 1;
      },
    );
    await Promise.resolve();
    drive.retire();
    const stopping = run.stop();
    expect(restore).not.toHaveBeenCalled();
    resolve();
    await Promise.all([write, stopping]);
    expect(restore).toHaveBeenCalledTimes(1);
    expect(documentValue).toBe(0);
    expect(run.ready).toBe(false);
  });

  it("drains a promoted delayed successor and rejects direct token replacement of admitted writes", async () => {
    const drive = new TutorialDriveV1();
    let finishFirst!: () => void;
    let finishSecond!: () => void;
    let startSecond!: () => void;
    const firstGate = new Promise<void>((resolve) => {
      finishFirst = resolve;
    });
    const secondGate = new Promise<void>((resolve) => {
      finishSecond = resolve;
    });
    const secondStarted = new Promise<void>((resolve) => {
      startSecond = resolve;
    });
    const first = drive.enqueue(
      () => true,
      () => firstGate,
    );
    await Promise.resolve();
    expect(() => drive.claim()).toThrow("Cannot replace an admitted tutorial document drive");
    const second = drive.enqueue(
      () => true,
      async () => {
        startSecond();
        await secondGate;
      },
    );
    let drained = false;
    const drain = drive.drain().then(() => {
      drained = true;
    });
    finishFirst();
    await secondStarted;
    expect(drained).toBe(false);
    finishSecond();
    await drain;
    expect(drive.busy).toBe(false);
    await Promise.all([first, second]);
  });

  it("quarantines a failed physical drive until explicit retirement and never runs queued reconciliation", async () => {
    const drive = new TutorialDriveV1();
    let reject!: (error: Error) => void;
    const gate = new Promise<void>((_resolve, fail) => {
      reject = fail;
    });
    const first = drive
      .enqueue(
        () => true,
        () => gate,
      )
      .catch((error) => error);
    await Promise.resolve();
    const reconcile = vi.fn(async () => {});
    const second = drive.enqueue(() => true, reconcile).catch((error) => error);
    const failure = new Error("uncertain physical write");
    reject(failure);
    expect(await first).toBe(failure);
    expect(await second).toBe(failure);
    await expect(drive.enqueue(() => true, reconcile)).rejects.toBe(failure);
    expect(reconcile).not.toHaveBeenCalled();
    drive.retire();
    await drive.enqueue(() => true, reconcile);
    expect(reconcile).toHaveBeenCalledTimes(1);
  });

  it("pauses the actual playback clock across a delayed owned seek without replaying its mutation", async () => {

    for (const row of tutorialSeekFixture.cases) {
      let frame: FrameRequestCallback | undefined;
      vi.stubGlobal("requestAnimationFrame", (next: FrameRequestCallback) => {
        frame = next;
        return 1;
      });
      vi.stubGlobal("cancelAnimationFrame", () => {});
      const clock = createTutorialClock(1000);
      try {
        if (row.playing) clock.play();
        const drive = new TutorialDriveV1();
        let owner = true;
        let wantsPlaying = row.requested;
        let mutations = 0;
        let playhead = 0;
        let resolve!: () => void;
        const delayed = new Promise<void>((done) => {
          resolve = done;
        });
        clock.subscribe(() => {
          if (clock.isPlaying() && playhead !== clock.getTimeMs()) mutations++;
        });
        const seek = runPausedTutorialSeekV1(
          clock,
          drive,
          () => owner,
          () => wantsPlaying,
          async (token) => {
            mutations++;
            await delayed;
            if (!owner || !drive.accepts(token)) return;
            playhead = 200;
            clock.seek(playhead);
          },
        );
        expect(clock.isPlaying()).toBe(false);
        await Promise.resolve();
        frame?.(100);
        frame?.(200);
        expect(mutations).toBe(1);
        expect(playhead).toBe(0);
        if (row.interrupt === "pause") wantsPlaying = false;
        if (row.interrupt === "play") wantsPlaying = true;
        if (row.interrupt === "close") {
          owner = false;
          drive.retire();
        }
        resolve();
        await seek;
        const observed = { mutations, playhead, resumed: clock.isPlaying() };
        expect(deepEqual(observed, { mutations: row.mutations, playhead: row.playhead, resumed: row.resumed })).toBe(true);
        expect(drive.active).toBe(false);
      } finally {
        clock.dispose();
        vi.unstubAllGlobals();
      }
    }
  });

  it("carries current Play intent through coalesced seeks without executing their superseded target", async () => {
    vi.stubGlobal("requestAnimationFrame", () => 1);
    vi.stubGlobal("cancelAnimationFrame", () => {});
    const clock = createTutorialClock(1000);
    const drive = new TutorialDriveV1();
    const writes: number[] = [];
    let resolve!: () => void;
    const gate = new Promise<void>((done) => {
      resolve = done;
    });
    try {
      clock.play();
      const first = runPausedTutorialSeekV1(
        clock,
        drive,
        () => true,
        () => true,
        async () => {
          writes.push(200);
          await gate;
          clock.seek(200);
        },
      );
      await Promise.resolve();
      const obsolete = runPausedTutorialSeekV1(
        clock,
        drive,
        () => true,
        () => true,
        async () => {
          writes.push(300);
          clock.seek(300);
        },
      );
      const latest = runPausedTutorialSeekV1(
        clock,
        drive,
        () => true,
        () => true,
        async () => {
          writes.push(500);
          clock.seek(500);
        },
      );
      expect(clock.isPlaying()).toBe(false);
      resolve();
      await Promise.all([first, obsolete, latest]);
      expect(writes).toEqual([200, 500]);
      expect(clock.getTimeMs()).toBe(500);
      expect(clock.isPlaying()).toBe(true);
      expect(drive.busy).toBe(false);
    } finally {
      clock.dispose();
      vi.unstubAllGlobals();
    }
  });

  it("releases cancelled tutorial drive ownership without clearing a newer seek or tween", () => {

    for (const row of tutorialRunFixture.drives) {
      const drive = new TutorialDriveV1();
      const tokens = new Map<string, number>();
      let oracle: string | null = null;
      const states: boolean[] = [];
      for (const event of row.events) {
        if (event === "retire") {
          drive.retire();
          oracle = null;
        } else if (event.startsWith("claim")) {
          const key = event.slice(-1);
          tokens.set(key, drive.claim());
          oracle = key;
        } else {
          const key = event.slice(-1);
          drive.release(tokens.get(key)!);
          if (oracle === key) oracle = null;
        }
        expect(drive.active, row.id).toBe(!deepEqual(oracle, null));
        states.push(drive.active);
      }
      expect(states, row.id).toEqual(row.active);
      expect(drive.active, row.id).toBe(false);
    }
  });

  it("admits an explicit target handoff once and retires a target created after its source expired", async () => {

    for (const row of admittedInstanceFixture.cases) {
      let admitted = row.before;
      const target = { instanceId: 9 };
      const create = vi.fn(async () => {
        admitted = row.after;
        return target;
      });
      const retire = vi.fn(async () => {});
      const result = await createAdmittedShellInstanceV1(() => admitted, create, retire);
      expect(deepEqual(result, target), row.id).toBe(row.before && row.after);
      expect(result !== null, row.id).toBe(row.accepted);
      expect(create, row.id).toHaveBeenCalledTimes(row.creates);
      expect(retire, row.id).toHaveBeenCalledTimes(row.retires);
      if (row.retires === 1) expect(retire).toHaveBeenCalledWith(target);
    }
  });

  it("admits a spawned program's own progress and completion passes only when the primary session presents them", () => {
    const program = { pluginId: dialogOriginFixture.owner.pluginId, instanceId: dialogOriginFixture.owner.sessionInstanceId, app: { id: dialogOriginFixture.owner.appId, controllerId: dialogOriginFixture.owner.controllerId } };
    const primary = { pluginId: "host", instanceId: 1, app: { id: "host", controllerId: "host" } };
    const spawned = [{ pluginId: program.pluginId, appId: program.app.id, instanceId: program.instanceId }];
    const current = { presentation: shellDialogOriginV1(primary, []), mounted: shellDialogOriginV1(program, []), primary, spawned };
    expect(shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(primary, []), source: shellDialogOriginV1(program, []) }, current), "presented by the primary session, sourced by the live program").toBe(true);
    expect(shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(program, []), source: shellDialogOriginV1(program, []) }, current), "the defect: a program presenting its own pass is never current").toBe(false);
    expect(shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(primary, []), source: shellDialogOriginV1(program, []) }, { ...current, spawned: [] }), "a retired program's pass is dropped").toBe(false);
    expect(shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(primary, []), source: shellDialogOriginV1(primary, []) }, { ...current, mounted: shellDialogOriginV1(primary, []) }), "the primary session's own lane").toBe(true);
    expect(
      shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(primary, []), source: shellDialogOriginV1(program, []) }, { ...current, presentation: shellDialogOriginV1({ ...primary, instanceId: 2 }, []) }),
      "a replaced primary session drops every pass it presented",
    ).toBe(false);
  });

  it("retires a spawned source independently of its still-visible primary presentation owner", async () => {
    const source = dialogOriginFixture.owner;
    const primary = { pluginId: "host", instanceId: 1, app: { id: "host", controllerId: "host" } };
    const spawned = [{ pluginId: source.pluginId, appId: source.appId, instanceId: source.sessionInstanceId }];
    for (const row of dialogOriginFixture.cases) {
      const mounted = applyPatch(structuredClone(source), row.patch as Parameters<typeof applyPatch>[1]).newDocument;
      expect(shellEffectSourceIsCurrentV1(source, mounted, primary, spawned), row.id).toBe(row.accepted);
    }
    expect(shellEffectSourceIsCurrentV1(source, source, primary, [])).toBe(false);
    expect(shellEffectSourceIsCurrentV1(source, source, primary, [{ ...spawned[0]!, appId: "replacement" }])).toBe(false);
    expect(shellEffectSourceIsCurrentV1(source, source, primary, [{ ...spawned[0]!, pluginId: "replacement" }])).toBe(false);
    const applyEffects = vi.fn(async () => {});
    let members = spawned;
    let complete!: (response: { requestedEffects: readonly unknown[] }) => void;
    const plugin = {
      handle: {
        handleAction: () =>
          new Promise((done) => {
            complete = done;
          }),
      },
    } as unknown as LoadedProgramState;
    const session = { pluginId: source.pluginId, instanceId: source.sessionInstanceId, app: { id: source.appId, controllerId: source.controllerId, modes: [], windowKinds: [], commands: [] }, viewState: {} } as unknown as Parameters<
      typeof makeEffectDispatchOne
    >[1];
    const pending = makeEffectDispatchOne(plugin, session, applyEffects, () => shellEffectSourceIsCurrentV1(source, source, primary, members), resolvedViewStateFixture)("lateAction");
    members = [];
    complete({ requestedEffects: [{ navigate: { uri: "/wrong" } }, { setPanel: { panelJson: "{}" } }] });
    await pending;
    expect(applyEffects).not.toHaveBeenCalled();
  });

  it("keeps tutorial snapshots on their exact run and never restores into a replacement session", async () => {

    for (const row of tutorialRunFixture.cases) {
      let current = structuredClone(dialogOriginFixture.owner);
      let epoch = 1;
      const restore = vi.fn(async () => {});
      let resolve!: (snapshot: { pack: Uint8Array; spr: Uint8Array }) => void;
      const run = new OwnedTutorialRunV1("tour", dialogOriginFixture.owner, () => epoch === 1 && shellDialogOriginIsCurrentV1(dialogOriginFixture.owner, current), {
        read: () =>
          new Promise<{ pack: Uint8Array; spr: Uint8Array }>((done) => {
            resolve = done;
          }),
        drain: async () => {},
        restore,
      });
      const started = run.start();
      let oracleCurrent = true;
      let oracleClosed = false;
      let oracleSnapshot = false;
      let oracleStarted = false;
      let oracleRestores = 0;
      for (const event of row.events) {
        if (event === "switch") {
          current = applyPatch(current, [{ op: "replace", path: "/document/clientInstanceId", value: "client-b" }]).newDocument;
          oracleCurrent = deepEqual(dialogOriginFixture.owner, current);
        } else if (event === "replace") {
          epoch = 2;
          oracleCurrent = false;
        } else if (event === "resolve") {
          resolve({ pack: Uint8Array.of(1, 2), spr: Uint8Array.of(3) });
          oracleStarted = oracleCurrent && !oracleClosed;
          oracleSnapshot = oracleStarted;
          expect(await started, row.id).toBe(oracleStarted);
        } else {
          if (!oracleClosed && oracleCurrent && oracleSnapshot) oracleRestores += 1;
          oracleClosed = true;
          await run.stop();
        }
      }
      expect(await started, row.id).toBe(row.started);
      expect(oracleRestores, row.id).toBe(row.restores);
      expect(restore, row.id).toHaveBeenCalledTimes(row.restores);
      await run.stop();
      expect(restore, row.id).toHaveBeenCalledTimes(row.restores);
      expect(run.isCurrent()).toBe(false);
    }
  });

  it("remounts staged fields and rejects old callbacks before they can dispatch into or close a replacement", () => {
    const a = { openingId: 1, dialogId: "createArtifact", origin: dialogOriginFixture.owner, seedArgs: { name: "A seed" } };
    const b = { openingId: 2, dialogId: "createArtifact", origin: { ...dialogOriginFixture.owner, document: { ...dialogOriginFixture.owner.document, clientInstanceId: "client-b" } }, seedArgs: { name: "B seed" } };
    let current: ShellDialogOriginV1 | null = a.origin;
    let live: number | null = a.openingId;
    const dispatch = vi.fn();
    const props = (owner: typeof a): OwnedShellDialogProps => ({
      owner,
      dialog: {
        id: "createArtifact",
        title: { native: { en: "Create Artifact", de: "Artefakt erstellen" } },
        args: [{ id: "name", label: { native: { en: "Name", de: "Name" } }, required: true, schema: { kind: "string", options: [] } }],
        submitAction: "createArtifact",
        submitLabel: { native: { en: "Create", de: "Erstellen" } },
        cancelAction: "cancelArtifact",
      },
      renderField: (_def, value, onChange, field) =>
        createElement("input", { id: field.id, "aria-labelledby": field.labelledBy, required: field.required, value: String(value ?? ""), onChange: (event: { target: { value: string } }) => onChange(event.target.value) }),
      isCurrent: (origin) => shellDialogOriginIsCurrentV1(origin, current),
      close: (openingId) => {
        if (live !== openingId) return false;
        live = null;
        return true;
      },
      dispatch,
    });
    const firstCallbacks = OwnedShellDialog(props(a))!.props as { onSubmit: (args: Record<string, unknown>) => void; onCancel: () => void };
    const view = render(createElement(OwnedShellDialog, props(a)));
    fireEvent.change(view.getByRole("textbox", { name: "Name" }), { target: { value: "A staged" } });
    expect((view.getByRole("textbox", { name: "Name" }) as HTMLInputElement).value).toBe("A staged");
    current = b.origin;
    live = b.openingId;
    view.rerender(createElement(OwnedShellDialog, props(b)));
    expect((view.getByRole("textbox", { name: "Name" }) as HTMLInputElement).value).toBe("B seed");
    firstCallbacks.onSubmit({ name: "A staged" });
    firstCallbacks.onCancel();
    expect(dispatch).not.toHaveBeenCalled();
    expect(live).toBe(b.openingId);
    const secondCallbacks = OwnedShellDialog(props(b))!.props as { onSubmit: (args: Record<string, unknown>) => void };
    current = null;
    secondCallbacks.onSubmit({ name: "B stale" });
    expect(dispatch).not.toHaveBeenCalled();
    expect(live).toBeNull();
    current = b.origin;
    live = b.openingId;
    secondCallbacks.onSubmit({ name: "B accepted" });
    secondCallbacks.onSubmit({ name: "B duplicate" });
    expect(dispatch).toHaveBeenCalledExactlyOnceWith("createArtifact", b.origin, { name: "B accepted" });
    view.unmount();
  });

  it("drops delayed effect replies and scheduled invocations after the owning mount retires", async () => {
    let current = true;
    let resolve!: (value: { requestedEffects: readonly unknown[] }) => void;
    const handleAction = vi.fn(
      () =>
        new Promise<{ requestedEffects: readonly unknown[] }>((done) => {
          resolve = done;
        }),
    );
    const plugin = { handle: { handleAction } } as unknown as LoadedProgramState;
    const session = { pluginId: "space", instanceId: 7, app: { id: "space.editor", controllerId: "space.editor", windowKinds: [], commands: [], modes: [] }, viewState: {} } as unknown as Parameters<typeof makeEffectDispatchOne>[1];
    const applyEffects = vi.fn(async () => {});
    const invoke = makeEffectDispatchOne(plugin, session, applyEffects, () => current, resolvedViewStateFixture);
    const pending = invoke("openCreateArtifact", {});
    expect(handleAction).toHaveBeenCalledTimes(1);
    current = false;
    resolve({ requestedEffects: [{ openDialog: { dialogId: "createArtifact" } }] });
    await pending;
    await invoke("openCreateArtifact", {});
    expect(applyEffects).not.toHaveBeenCalled();
    expect(handleAction).toHaveBeenCalledTimes(1);
  });

  it("rejects every changed origin with the same result as the independent JSON Patch and equality oracle", () => {
    expect(rendererExport("ShellDialogOriginV1")(dialogOriginFixture.owner)).toBe(true);
    for (const row of dialogOriginFixture.cases) {
      const current = applyPatch(structuredClone(dialogOriginFixture.owner), row.patch as Parameters<typeof applyPatch>[1]).newDocument;
      expect(deepEqual(dialogOriginFixture.owner, current), row.id).toBe(row.accepted);
      expect(shellDialogOriginIsCurrentV1(dialogOriginFixture.owner, current), row.id).toBe(row.accepted);
    }
    expect(shellDialogOriginIsCurrentV1(dialogOriginFixture.owner, null)).toBe(false);
    expect(shellDialogOriginIsCurrentV1(null, dialogOriginFixture.owner)).toBe(false);
  });

  it("requires a unique exact mounted session and supports documentless dialogs without lending document authority", () => {
    const source = dialogOriginFixture.owner;
    const session = { pluginId: source.pluginId, instanceId: source.sessionInstanceId, app: { id: source.appId, controllerId: source.controllerId } };
    const mount = { session, ...source.document };
    expect(shellDialogOriginV1(session, [mount])).toEqual(source);
    expect(shellDialogOriginV1(session, [mount, mount])).toBeNull();
    expect(shellDialogOriginV1(null, [mount])).toBeNull();
    const unmounted = shellDialogOriginV1(session, []);
    expect(unmounted).toEqual({ ...source, document: null });
    expect(shellDialogOriginIsCurrentV1(unmounted, structuredClone(unmounted))).toBe(true);
    expect(shellDialogOriginIsCurrentV1(source, unmounted)).toBe(false);
  });
});

describe("Space artifact creation host owner", () => {
  const requestId = "1".repeat(32);
  const choice = '{"kindId":"fixture.neutral-host-fixture.counter","schema":"fixture.neutral-host-fixture.counter","dialect":{"artifactKind":"fixture.neutral-host-fixture.counter","standard":"1","subset":"*"},"label":{"en":"Counter","de":"Zähler"}}';
  const catalogOrigin: ShellDialogOriginV1 = {
    pluginId: "space",
    appId: "space-editor",
    controllerId: "space",
    sessionInstanceId: 7,
    document: {
      runtimeKey: artifactCreationCatalogAuthorityFixture.catalog.runtimeKey,
      clientInstanceId: artifactCreationCatalogAuthorityFixture.catalog.clientInstanceId,
      scope: { spaceId: artifactCreationCatalogAuthorityFixture.catalog.spaceId, documentId: "index" },
    },
  };
  const catalog = {
    kind: "space-artifact-creation-catalog" as const,
    clientInstanceId: artifactCreationCatalogAuthorityFixture.catalog.clientInstanceId,
    spaceId: artifactCreationCatalogAuthorityFixture.catalog.spaceId,
    catalogGenerationId: artifactCreationCatalogAuthorityFixture.catalog.catalogGenerationId,
    kinds: [JSON.parse(choice)],
  };
  const catalogStatus = { kind: "space-artifact-creation-catalog-status" as const, clientInstanceId: catalog.clientInstanceId, spaceId: catalog.spaceId, phase: "ready" as const };
  const catalogAuthority = captureSpaceArtifactCreationCatalogAuthorityV1(catalog, catalogStatus, catalogOrigin)!;
  const owner: SpaceArtifactCreationOwnerV1 = {
    requestId,
    name: "Shared Map",
    spaceId: "space-a",
    expectedCatalogGenerationId: catalogAuthority.catalogGenerationId,
    kindId: "fixture.neutral-host-fixture.counter",
    runtimeKey: artifactCreationCatalogAuthorityFixture.catalog.runtimeKey,
    clientInstanceId: artifactCreationCatalogAuthorityFixture.catalog.clientInstanceId,
    sessionInstanceId: 7,
    opening: false,
    cancelRequested: false,
    ready: null,
  };

  it("forwards only the user intent and rejects presentation overposts or mismatched kinds", () => {
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice, name: "Shared Map" }, "space-a", requestId, catalogAuthority, catalogAuthority)).toEqual({
      kind: "space-artifact-create",
      requestId,
      spaceId: "space-a",
      expectedCatalogGenerationId: catalogAuthority.catalogGenerationId,
      kindId: "fixture.neutral-host-fixture.counter",
      name: "Shared Map",
    });
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice, name: "Shared Map", documentId: "forged" }, "space-a", requestId, catalogAuthority, catalogAuthority)).toBeNull();
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice.replaceAll("fixture.neutral-host-fixture.counter", "fixture.other-owner.counter"), name: "Shared Map" }, "space-a", requestId, catalogAuthority, catalogAuthority)).toBeNull();
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice.replace('"kindId":"fixture.neutral-host-fixture.counter"', '"kindId":"forged"'), name: "Shared Map" }, "space-a", requestId, catalogAuthority, catalogAuthority)).toBeNull();
  });

  it("creates and opens a catalog member whose creation kind differs from the dialect that opens it", () => {
    const drawing = '{"kindId":"2d.drawing","schema":"drawing.document","dialect":{"artifactKind":"s.draw.drawing","standard":"1","subset":"*"},"label":{"en":"Editor","de":"Editor"}}';
    const drawingCatalog = { ...catalog, kinds: [JSON.parse(drawing)] };
    const drawingAuthority = captureSpaceArtifactCreationCatalogAuthorityV1(drawingCatalog, catalogStatus, catalogOrigin)!;
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: drawing, name: "Plan" }, "space-a", requestId, drawingAuthority, drawingAuthority)).toEqual({
      kind: "space-artifact-create",
      requestId,
      spaceId: "space-a",
      expectedCatalogGenerationId: drawingAuthority.catalogGenerationId,
      kindId: "2d.drawing",
      name: "Plan",
    });
    expect(
      spaceArtifactCreationReadyOpening({
        kind: "space-artifact-creation-status",
        requestId,
        spaceId: "space-a",
        catalogGenerationId: drawingAuthority.catalogGenerationId,
        phase: "ready",
        ready: { artifactId: `artifact-${"6".repeat(32)}`, kindId: "2d.drawing", artifactSchema: "drawing.document", parentDialect: { artifactKind: "s.draw.drawing", standard: "1", subset: "*" } },
      }),
    ).toEqual({ artifactRef: "s.draw.drawing@1/*", artifactId: `artifact-${"6".repeat(32)}`, spaceId: "space-a", schema: "drawing.document" });
  });

  it("admits only an exact captured and live catalog generation member", () => {
    const withoutMember = { ...catalogAuthority, kindChoices: [] };
    const variants: Readonly<Record<string, SpaceArtifactCreationCatalogAuthorityV1 | null>> = {
      current: catalogAuthority,
      "without-member": withoutMember,
      unavailable: null,
      "rotated-generation": { ...catalogAuthority, catalogGenerationId: "4".repeat(64) },
      "replaced-client": { ...catalogAuthority, clientInstanceId: "22222222-2222-4222-8222-222222222222" },
      "other-space": { ...catalogAuthority, spaceId: "space-b" },
    };
    for (const row of artifactCreationCatalogAuthorityFixture.cases) {
      const selected = row.choice === "member" ? choice : choice.replaceAll("fixture.neutral-host-fixture.counter", "fixture.other-owner.counter");
      const request = spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: selected, name: "Shared Map" }, "space-a", requestId, variants[row.captured], variants[row.live]);
      expect(request !== null, row.id).toBe(row.admitted);
    }
    const ready = selectedSpaceArtifactCreationCatalogV1(catalog, catalogStatus, catalogOrigin)!;
    const withdrawn = selectedSpaceArtifactCreationCatalogV1(null, { ...catalogStatus, phase: "unavailable" }, catalogOrigin)!;
    expect(ready).toMatchObject({ phase: "ready", kinds: catalog.kinds, authority: catalogAuthority });
    expect(withdrawn).toMatchObject({ phase: "unavailable", kinds: [], authority: null });
    expect(ready.choiceRevision).not.toBe(withdrawn.choiceRevision);
  });

  it("binds creation acceptance and ready status to the selected catalog generation", () => {
    const request = spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice, name: "Shared Map" }, "space-a", requestId, catalogAuthority, catalogAuthority);
    const captured = { ...owner, expectedCatalogGenerationId: catalogAuthority.catalogGenerationId };
    const statuses = artifactCreationCatalogAuthorityFixture.statusCases.map((row) => {
      const message = {
        kind: "space-artifact-creation-status" as const,
        requestId,
        spaceId: owner.spaceId,
        ...(row.generation === "missing" ? {} : { catalogGenerationId: row.generation === "current" ? catalogAuthority.catalogGenerationId : "4".repeat(64) }),
        phase: "ready" as const,
        ready: { artifactId: `artifact-${"2".repeat(32)}`, kindId: owner.kindId, artifactSchema: "fixture.neutral-host-fixture.counter", parentDialect: { artifactKind: owner.kindId, standard: "1", subset: "*" } },
      };
      return spaceArtifactCreationOwnerAcceptsStatus(captured, message as Extract<BackboneWorkerResponse, { kind: "space-artifact-creation-status" }>);
    });
    expect({ requestGeneration: (request as unknown as Record<string, unknown>)?.expectedCatalogGenerationId, statuses }).toEqual({
      requestGeneration: catalogAuthority.catalogGenerationId,
      statuses: artifactCreationCatalogAuthorityFixture.statusCases.map((row) => row.admitted),
    });
  });

  it("refreshes only the exact current selected catalog after an owned initial conflict", () => {
    const refresh = { kind: "space-artifact-creation-catalog-refresh-required" as const, requestId, spaceId: owner.spaceId, catalogGenerationId: owner.expectedCatalogGenerationId };
    const ready = {
      kind: "space-artifact-creation-status" as const,
      requestId,
      spaceId: owner.spaceId,
      catalogGenerationId: owner.expectedCatalogGenerationId,
      phase: "ready" as const,
      ready: { artifactId: `artifact-${"2".repeat(32)}`, kindId: owner.kindId, artifactSchema: "fixture.neutral-host-fixture.counter", parentDialect: { artifactKind: owner.kindId, standard: "1", subset: "*" } },
    };
    const owners: Readonly<Record<string, SpaceArtifactCreationOwnerV1 | null>> = { current: owner, absent: null, ready: { ...owner, ready }, opening: { ...owner, opening: true } };
    const authorities: Readonly<Record<string, SpaceArtifactCreationCatalogAuthorityV1 | null>> = {
      current: catalogAuthority,
      unavailable: null,
      "rotated-generation": { ...catalogAuthority, catalogGenerationId: "4".repeat(64) },
      "replaced-runtime": { ...catalogAuthority, runtimeKey: "hub:space-a:replacement" },
    };
    const origins: Readonly<Record<string, ShellDialogOriginV1 | null>> = {
      current: catalogOrigin,
      "replaced-session": { ...catalogOrigin, sessionInstanceId: catalogOrigin.sessionInstanceId + 1 },
      "replaced-client": { ...catalogOrigin, document: { ...catalogOrigin.document!, clientInstanceId: "22222222-2222-4222-8222-222222222222" } },
      "other-document": { ...catalogOrigin, document: { ...catalogOrigin.document!, scope: { spaceId: owner.spaceId, documentId: "other" } } },
    };
    const messages: Readonly<Record<string, Extract<BackboneWorkerResponse, { kind: "space-artifact-creation-catalog-refresh-required" }>>> = {
      current: refresh,
      "replaced-request": { ...refresh, requestId: "2".repeat(32) },
      "rotated-generation": { ...refresh, catalogGenerationId: "4".repeat(64) },
    };
    for (const row of artifactCreationCatalogAuthorityFixture.refreshCases) {
      const request = spaceArtifactCreationCatalogRefreshRequestV1(owners[row.owner], authorities[row.authority], origins[row.origin], messages[row.message]);
      expect(request, row.id).toEqual(row.admitted ? { kind: "space-artifact-creation-catalog-open", clientInstanceId: owner.clientInstanceId, spaceId: owner.spaceId } : null);
    }
  });

  it("retires only staged artifact-kind fields on catalog revision", () => {
    const definitions = artifactCreationCatalogAuthorityFixture.draftCases.map((row) => ({ ownerId: row.id, args: [{ id: "value", artifactKind: row.format === "artifactKind" }] }));
    const staged = Object.fromEntries(artifactCreationCatalogAuthorityFixture.draftCases.map((row) => [row.id, { value: row.format }]));
    const retired = new Set(artifactKindChoiceDraftRetirementsV1(staged, definitions).map((row) => row.ownerId));
    for (const row of artifactCreationCatalogAuthorityFixture.draftCases) expect(retired.has(row.id), row.id).toBe(row.clear);
  });

  it("opens only an exact Ready tuple owned by the originating Space mount", () => {
    const ready = {
      kind: "space-artifact-creation-status" as const,
      requestId,
      spaceId: "space-a",
      catalogGenerationId: owner.expectedCatalogGenerationId,
      phase: "ready" as const,
      ready: {
        artifactId: `artifact-${"2".repeat(32)}`,
        kindId: "fixture.neutral-host-fixture.counter",
        artifactSchema: "fixture.neutral-host-fixture.counter",
        parentDialect: { artifactKind: "fixture.neutral-host-fixture.counter", standard: "1", subset: "*" },
      },
    };
    expect(spaceArtifactCreationOwnerAcceptsStatus(owner, ready)).toBe(true);
    expect(spaceArtifactCreationOwnerAcceptsStatus(owner, { ...ready, requestId: "3".repeat(32) })).toBe(false);
    expect(spaceArtifactCreationOwnerAcceptsStatus(owner, { ...ready, ready: { ...ready.ready, kindId: "s.draw.draw" } })).toBe(false);
    expect(spaceArtifactCreationOwnerAcceptsStatus({ ...owner, ready }, ready)).toBe(true);
    expect(spaceArtifactCreationOwnerAcceptsStatus({ ...owner, ready }, { ...ready, ready: { ...ready.ready, artifactId: `artifact-${"5".repeat(32)}` } })).toBe(false);
    expect(spaceArtifactCreationReadyOpening(ready)).toEqual({
      artifactRef: "fixture.neutral-host-fixture.counter@1/*",
      artifactId: `artifact-${"2".repeat(32)}`,
      spaceId: "space-a",
      schema: "fixture.neutral-host-fixture.counter",
    });
    expect(spaceArtifactCreationReadyOpening({ kind: "space-artifact-creation-status", requestId, spaceId: "space-a", catalogGenerationId: owner.expectedCatalogGenerationId, phase: "preparing" })).toBeNull();
  });

  it("publishes only a committed current target and releases every private rejected target once", async () => {
    const validate = directoryExport("ArtifactCreationReadyOpeningV1");
    expect(validate(artifactCreationReadyOpeningFixture), JSON.stringify(validate.errors)).toBe(true);
    for (const row of artifactCreationReadyOpeningFixture.cases) {
      let currentIndex = 0;
      let releases = 0;
      let publishes = 0;
      let failures = 0;
      const outcome = await runArtifactCreationReadyOpeningV1({
        current: () => row.current[Math.min(currentIndex++, row.current.length - 1)]!,
        prepare: async () => {
          if (row.prepare === "unused") throw new Error("unexpected prepare");
          return row.prepare === "missing" ? null : { id: row.id };
        },
        open: async () => {
          if (row.open === "unused") throw new Error("unexpected open");
          if (row.open === "failed") throw new Error("attach failed");
          return row.open === "committed" ? { committed: true as const, runtimeKey: row.id, clientInstanceId: "client" } : null;
        },
        publish: () => {
          publishes += 1;
        },
        release: async () => {
          releases += 1;
          if (row.release === "failed") throw new Error("release failed");
        },
        failed: () => {
          failures += 1;
        },
      });
      expect(outcome, row.id).toBe(row.outcome);
      expect(releases, row.id).toBe(row.release === "unused" ? 0 : 1);
      expect(publishes, row.id).toBe(row.publish ? 1 : 0);
      expect(failures, row.id).toBe(row.failure ? 1 : 0);
    }
  });

  it("requires an explicit same-generation mount after the document port becomes ready", async () => {
    expect(directoryExport("ArtifactCreationReadyOpeningV1")(artifactCreationReadyOpeningFixture)).toBe(true);
    const { createArtifactCreationCatalogMountV1 } = await import("../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🟦️.ts");
    for (const row of artifactCreationReadyOpeningFixture.mountCases) {
      const gate = createArtifactCreationCatalogMountV1(catalogAuthority.catalogGenerationId);
      let outcome = "pending";
      const settled = gate.ready.then(
        () => {
          outcome = "ready";
        },
        () => {
          outcome = "failed";
        },
      );
      await Promise.resolve();
      expect(outcome, row.id).toBe("pending");
      expect(gate.current(), row.id).toBe(true);
      const accepted: boolean[] = [];
      for (const action of row.actions) {
        if (action === "close") gate.close(new Error("document closed"));
        else accepted.push(gate.accept(action === "current" ? catalogAuthority.catalogGenerationId : action === "rotated" ? "4".repeat(64) : ""));
      }
      await settled;
      const expected = { accepted: row.accepted, outcome: row.outcome };
      expect({ accepted, outcome }, row.id).toEqual(expected);
      expect(deepEqual({ accepted, outcome }, expected), row.id).toBe(true);
      expect(gate.current(), row.id).toBe(row.committed);
    }
    for (const generation of ["", "0".repeat(64), "A".repeat(64), "1".repeat(63), "1".repeat(65), null, ["1".repeat(64)]]) expect(() => createArtifactCreationCatalogMountV1(generation as string)).toThrow();
  });

  it("commits private document opening only after a still-current creation mount", async () => {
    expect(directoryExport("ArtifactCreationReadyOpeningV1")(artifactCreationReadyOpeningFixture)).toBe(true);
    const { createArtifactCreationCatalogMountV1 } = await import("../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🟦️.ts");
    for (const row of artifactCreationReadyOpeningFixture.mountCases) {
      const gate = createArtifactCreationCatalogMountV1(catalogAuthority.catalogGenerationId);
      let commits = 0,
        closes = 0,
        detaches = 0,
        retires = 0;
      let failure: string | null = null;
      let attached!: () => void;
      const attachment = new Promise<void>((resolve) => {
        attached = resolve;
      });
      const documentReady = row.id === "rebootstrap-before-port-ready" ? new Promise<void>(() => {}) : Promise.resolve();
      const opening = runDocumentOpeningAttemptV1({
        deadlineMs: 1_000,
        current: gate.current,
        socket: async () => {},
        attach: async () => {
          attached();
          await gate.attach(documentReady);
        },
        commit: () => {
          commits += 1;
        },
        close: () => {
          closes += 1;
          gate.close(new Error("opening closed"));
        },
        detach: async () => {
          detaches += 1;
        },
        retire: () => {
          retires += 1;
        },
      }).catch((error) => {
        failure = error instanceof Error ? error.message : String(error);
        return false;
      });
      await attachment;
      expect({ commits, closes, detaches, retires }, row.id).toEqual({ commits: 0, closes: 0, detaches: 0, retires: 0 });
      for (const action of row.actions) {
        if (action === "close") gate.close(new Error("document closed"));
        else gate.accept(action === "current" ? catalogAuthority.catalogGenerationId : action === "rotated" ? "4".repeat(64) : "");
      }
      const result = { committed: await opening, commits, closes, detaches, retires };
      const expected = { committed: row.committed, commits: Number(row.committed), closes: Number(!row.committed), detaches: Number(!row.committed), retires: 1 };
      expect(result, row.id).toEqual(expected);
      expect(deepEqual(result, expected), row.id).toBe(true);
      if (row.outcome === "failed") expect(failure, row.id).toBe(row.actions.includes("close") ? "document closed" : "artifact-creation.catalog-generation-mismatch");
    }
  });

  it("retains exact sibling owners through one-shot cancellation and accepts a racing Ready", () => {
    const progressOwner: ArtifactCreationProgressOwnerV1 = owner;
    const sibling: ArtifactCreationProgressOwnerV1 = { ...progressOwner, requestId: "2".repeat(32), name: "Second Map" };
    let state = reduceArtifactCreationProgressUiV1({}, { kind: "issued", owner: progressOwner, atMs: 0 });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: sibling, atMs: 0 });
    const unchanged = state;
    expect(
      reduceArtifactCreationProgressUiV1(state, { kind: "status", message: { kind: "space-artifact-creation-status", requestId: "3".repeat(32), spaceId: "space-a", catalogGenerationId: owner.expectedCatalogGenerationId, phase: "failed" } }),
    ).toBe(unchanged);
    expect(reduceArtifactCreationProgressUiV1(state, { kind: "status", message: { kind: "space-artifact-creation-status", requestId, spaceId: "foreign-space", catalogGenerationId: owner.expectedCatalogGenerationId, phase: "failed" } })).toBe(
      unchanged,
    );
    state = reduceArtifactCreationProgressUiV1(state, { kind: "cancel-requested", requestId, spaceId: "space-a" });
    expect(state[requestId]).toMatchObject({ phase: "accepted", cancelRequested: true });
    expect(state[sibling.requestId]).toMatchObject({ phase: "accepted", cancelRequested: false });
    expect(reduceArtifactCreationProgressUiV1(state, { kind: "cancel-requested", requestId, spaceId: "space-a" })).toBe(state);
    const wrongKind = {
      kind: "space-artifact-creation-status" as const,
      requestId,
      spaceId: "space-a",
      catalogGenerationId: owner.expectedCatalogGenerationId,
      phase: "ready" as const,
      ready: { artifactId: `artifact-${"4".repeat(32)}`, kindId: "s.draw.draw", artifactSchema: "s.draw.draw", parentDialect: { artifactKind: "s.draw.draw", standard: "1", subset: "*" } },
    };
    expect(reduceArtifactCreationProgressUiV1(state, { kind: "status", message: wrongKind })).toBe(state);
    const ready = { ...wrongKind, ready: { ...wrongKind.ready, kindId: owner.kindId, artifactSchema: owner.kindId, parentDialect: { artifactKind: owner.kindId, standard: "1", subset: "*" } } };
    state = reduceArtifactCreationProgressUiV1(state, { kind: "status", message: ready });
    expect(state[requestId]).toMatchObject({ phase: "ready", cancelRequested: true, openingDisposition: "idle" });
    expect(state[sibling.requestId]).toMatchObject({ phase: "accepted", cancelRequested: false });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "open-failed", requestId, spaceId: "space-a" });
    expect(state[requestId]).toMatchObject({ phase: "ready", openingDisposition: "failed" });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "opening", requestId, spaceId: "space-a" });
    expect(state[requestId]).toMatchObject({ phase: "ready", openingDisposition: "opening" });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "cleared", requestId });
    expect(state[requestId]).toBeUndefined();
    expect(state[sibling.requestId]).toBeDefined();
  });

  it("bounds presentation state without evicting a live retained owner", () => {
    let state: ReturnType<typeof reduceArtifactCreationProgressUiV1> = {};
    const owners = Array.from({ length: ARTIFACT_CREATION_PROGRESS_CAPACITY }, (_, index) => ({
      ...owner,
      requestId: index.toString(16).padStart(32, "0"),
      name: `Map ${index}`,
    }));
    for (const candidate of owners) state = reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: candidate, atMs: 0 });
    const full = state;
    const replacement = { ...owner, requestId: "f".repeat(32), name: "Replacement" };
    expect(reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: replacement, atMs: 0 })).toBe(full);
    state = reduceArtifactCreationProgressUiV1(state, {
      kind: "status",
      message: { kind: "space-artifact-creation-status", requestId: owners[0]!.requestId, spaceId: owner.spaceId, catalogGenerationId: owner.expectedCatalogGenerationId, phase: "failed" },
    });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: replacement, atMs: 0 });
    expect(Object.keys(state)).toHaveLength(ARTIFACT_CREATION_PROGRESS_CAPACITY);
    expect(state[owners[0]!.requestId]).toBeUndefined();
    expect(state[replacement.requestId]).toMatchObject({ phase: "accepted", cancelRequested: false });
  });

  it("renders the schema-owned English and German lifecycle without inventing numeric progress", async () => {
    const validate = directoryExport("ArtifactCreationProgressUiV1");
    expect(validate(artifactCreationProgressFixture), JSON.stringify(validate.errors)).toBe(true);
    expect(deepEqual(artifactCreationProgressFixture.locales, ARTIFACT_CREATION_PROGRESS_TEXT_V1)).toBe(true);
    const oracle = createTranslationOracle();
    await oracle.init({
      fallbackLng: false,
      resources: {
        en: { translation: artifactCreationProgressFixture.locales.en },
        de: { translation: artifactCreationProgressFixture.locales.de },
      },
    });
    for (const row of artifactCreationProgressFixture.cases) {
      const phase = row.phase as keyof typeof ARTIFACT_CREATION_PROGRESS_TEXT_V1.en.phases;
      const locale = row.locale as "en" | "de";
      expect(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].phases[phase], row.id).toBe(oracle.t(`phases.${phase}`, { lng: locale }));
      expect(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].opening.failed, row.id).toBe(oracle.t("opening.failed", { lng: locale }));
      expect(artifactCreationProgressRoleV1(phase, row.openingDisposition as "idle" | "opening" | "failed"), row.id).toBe(row.role);
      expect(artifactCreationProgressTerminalV1(phase), row.id).toBe(["ready", "indeterminate", "failed", "cancelled"].includes(phase));
      const view = render(
        createElement(ArtifactCreationProgressNotice, {
          state: { ...owner, issuedAtMs: Date.now(), phase, cancelRequested: row.cancelRequested, openingDisposition: row.openingDisposition as "idle" | "opening" | "failed" },
          locale,
          onCancel: () => {},
          onOpen: () => {},
        }),
      );
      const region = view.getByRole(row.role, { name: `${artifactCreationProgressFixture.locales[locale].heading}: Shared Map` });
      expect(computeAccessibleName(region), row.id).toBe(`${artifactCreationProgressFixture.locales[locale].heading}: Shared Map`);
      expect(region.getAttribute("aria-live"), row.id).toBe(row.live);
      expect(region.querySelector("progress"), row.id).toBeNull();
      const button = region.querySelector("button");
      if (row.retryable) expect(button?.textContent, row.id).toBe(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].opening.retry);
      else if (artifactCreationProgressTerminalV1(phase)) expect(button, row.id).toBeNull();
      else expect(button?.hasAttribute("disabled"), row.id).toBe(row.cancelRequested);
      view.unmount();
    }
    for (const row of artifactCreationProgressFixture.catalogCases) {
      const locale = row.locale as "en" | "de";
      const phase = row.phase as "loading" | "ready" | "unavailable";
      const textKey = row.effectivePhase as "loading" | "ready" | "unavailable";
      expect(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].catalog[textKey], row.id).toBe(oracle.t(`catalog.${textKey}`, { lng: locale }));
      const view = render(
        createElement(ArtifactCreationCatalogNotice, {
          status: { phase },
          hasChoices: row.hasChoices,
          locale,
        }),
      );
      const region = view.getByRole(row.role, { name: artifactCreationProgressFixture.locales[locale].catalog[textKey] });
      expect(region.getAttribute("aria-live"), row.id).toBe(row.live);
      expect(region.getAttribute("aria-busy"), row.id).toBe(textKey === "loading" ? "true" : null);
      expect(region.getAttribute("data-semio-artifact-creation-catalog"), row.id).toBe(textKey);
      view.unmount();
    }
    for (const locale of artifactCreationProgressFixture.unsupportedLocales) {
      expect(artifactCreationProgressLocaleV1(locale), locale).toBeNull();
      expect(
        renderToStaticMarkup(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now(), phase: "accepted", cancelRequested: false, openingDisposition: "idle" }, locale, onCancel: () => {}, onOpen: () => {} })),
        locale,
      ).toBe("");
    }
  });

  // 🐢️ ticket 26/09/23 S15: a creation the hub keeps working on says how long, in the person's language, and keeps its Cancel.
  it("tells how long the hub has been working on a running creation and keeps the Cancel, en and de", () => {
    for (const [locale, elapsed] of [
      ["en", "2 min 5 s"],
      ["de", "2 Min. 5 s"],
    ] as const) {
      const view = render(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now() - 125_400, phase: "accepted", cancelRequested: false, openingDisposition: "idle" }, locale, onCancel: () => {}, onOpen: () => {} }));
      const region = view.getByRole("status");
      expect(region.querySelector("[data-semio-artifact-creation-elapsed]")?.textContent).toBe(artifactCreationProgressFixture.locales[locale].waiting.replace("{elapsed}", elapsed));
      expect(view.getByRole("button", { name: `${artifactCreationProgressFixture.locales[locale].cancel}: Shared Map` }).hasAttribute("disabled")).toBe(false);
      view.unmount();
      const fresh = render(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now(), phase: "accepted", cancelRequested: false, openingDisposition: "idle" }, locale, onCancel: () => {}, onOpen: () => {} }));
      expect(fresh.container.querySelector("[data-semio-artifact-creation-elapsed]"), "no waiting line before the threshold").toBeNull();
      fresh.unmount();
      const done = render(
        createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now() - 125_400, phase: "indeterminate", cancelRequested: false, openingDisposition: "idle" }, locale, onCancel: () => {}, onOpen: () => {} }),
      );
      expect(done.container.querySelector("[data-semio-artifact-creation-elapsed]"), "a concluded creation shows no waiting line").toBeNull();
      done.unmount();
    }
  });

  it("dispatches cancel once with the exact retained owner", () => {
    const onCancel = vi.fn();
    const view = render(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now(), phase: "preparing", cancelRequested: false, openingDisposition: "idle" }, locale: "de", onCancel, onOpen: () => {} }));
    fireEvent.click(view.getByRole("button", { name: "Erstellung abbrechen: Shared Map" }));
    expect(onCancel).toHaveBeenCalledExactlyOnceWith(requestId, "space-a");
    view.rerender(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now(), phase: "preparing", cancelRequested: true, openingDisposition: "idle" }, locale: "de", onCancel, onOpen: () => {} }));
    fireEvent.click(view.getByRole("button", { name: "Erstellung abbrechen: Shared Map" }));
    expect(onCancel).toHaveBeenCalledTimes(1);
    view.unmount();
  });

  it("offers a bilingual open retry without issuing creation or cancellation again", () => {
    for (const locale of ["en", "de"] as const) {
      const onCancel = vi.fn();
      const onOpen = vi.fn();
      const view = render(
        createElement(ArtifactCreationProgressNotice, {
          state: { ...owner, issuedAtMs: Date.now(), phase: "ready", cancelRequested: false, openingDisposition: "failed" },
          locale,
          onCancel,
          onOpen,
        }),
      );
      fireEvent.click(view.getByRole("button", { name: `${ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].opening.retry}: Shared Map` }));
      expect(onOpen).toHaveBeenCalledExactlyOnceWith(requestId);
      expect(onCancel).not.toHaveBeenCalled();
      view.unmount();
    }
  });
});

//#region 🩺️RuntimeDiagnostics
/** 🩺️ The module that GENERATES the shard worker, read as text. `import.meta.url` is not a file URL
 * under this suite's transform and the module itself cannot be imported here (it reaches the repo
 * library's `bun:sqlite` lease store, which a jsdom bundle refuses), so it is located by walking up
 * from the runner's cwd — the same shape the world3d-host law above uses. */
function readGeneratedShardWorkerOwnerSource(): string {
  const relative = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
  let root = process.cwd();
  for (let hop = 0; hop < 12 && !existsSync(`${root}/${relative}`); hop += 1) root = `${root}/..`;
  expect(existsSync(`${root}/${relative}`)).toBe(true);
  return readFileSync(`${root}/${relative}`, "utf8");
}

describe("shell runtime diagnostics switch", () => {
  afterEach(() => {
    setRuntimeDiagnostics(undefined);
  });

  it("is off by default and names the same key the guest switch does", () => {
    // 🪞️ `RUNTIME_DIAGNOSTICS_ENV` in `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` — one name, two languages.
    expect(RUNTIME_DIAGNOSTICS_KEY).toBe("SEMIO_RUNTIME_DIAGNOSTICS");
    expect(runtimeDiagnosticsEnabled()).toBe(false);
  });

  // ⚖️ LAW: the key is spelled once per realm that has to resolve it — the shell, the shard-worker
  // bootstrap, and the generated worker/host-shim source — and all of them must agree, because the
  // guest's own `RUNTIME_DIAGNOSTICS_ENV` is the only reader and it matches on the exact string
  // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-guest-tick-cost-2026-09-12.md` §4 rank 1).
  it("spells the same diagnostics key in every realm that resolves it", () => {
    expect(SHARD_RUNTIME_DIAGNOSTICS_KEY).toBe(RUNTIME_DIAGNOSTICS_KEY);
    // 🔌️ The generated shard worker's own copy lives in `🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`,
    // which this jsdom suite cannot import (it reaches the repo library's `bun:sqlite` lease store),
    // so it is read as text — the same shape `⏱️wgpu-ui-turn-budget` uses for `ShellHost`.
    const pluginPackage = readGeneratedShardWorkerOwnerSource();
    expect(pluginPackage).toContain(`export const SHARD_RUNTIME_DIAGNOSTICS_KEY = "${RUNTIME_DIAGNOSTICS_KEY}"`);
    expect(pluginPackage).toContain(`export const SHARD_WORKER_DIAGNOSTICS_PARAM = "${SHARD_WORKER_DIAGNOSTICS_PARAM}"`);
  });

  // ⚖️ LAW: an unarmed page boots the shard worker at the bare url, and an armed one stamps the
  // parameter the worker reads back — the ONLY channel the switch has into a Worker realm, which
  // owns no `localStorage`. The generated worker must read that same parameter and seed
  // `wasi:cli/environment` from it, which is what makes the guest's `[TRACE]` sites reachable.
  it("carries the armed switch into the worker realm on the worker url", () => {
    const stored = new Map<string, string>();
    const original = globalThis.localStorage;
    Object.defineProperty(globalThis, "localStorage", {
      configurable: true,
      value: { getItem: (key: string) => stored.get(key) ?? null, setItem: (key: string, value: string) => void stored.set(key, value), removeItem: (key: string) => void stored.delete(key) },
    });
    try {
      expect(shardWorkerUrl()).toBe(SHARD_WORKER_URL);
      stored.set(SHARD_RUNTIME_DIAGNOSTICS_KEY, "1");
      expect(shardWorkerUrl()).toBe(`${SHARD_WORKER_URL}?${SHARD_WORKER_DIAGNOSTICS_PARAM}=1`);
    } finally {
      if (original === undefined) Reflect.deleteProperty(globalThis, "localStorage");
      else Object.defineProperty(globalThis, "localStorage", { configurable: true, value: original });
    }
    const pluginPackage = readGeneratedShardWorkerOwnerSource();
    expect(pluginPackage).toContain('searchParams.get("${SHARD_WORKER_DIAGNOSTICS_PARAM}")');
    expect(pluginPackage).toContain('_setEnv({ "${SHARD_RUNTIME_DIAGNOSTICS_KEY}": "1" })');
    expect(pluginPackage.indexOf("await armGuestRuntimeDiagnostics()")).toBeLessThan(pluginPackage.indexOf("const bridge = await import("));
  });

  it("an explicit host override outranks every resolved source, in both directions", () => {
    setRuntimeDiagnostics(true);
    expect(runtimeDiagnosticsEnabled()).toBe(true);
    setRuntimeDiagnostics(false);
    expect(runtimeDiagnosticsEnabled()).toBe(false);
    setRuntimeDiagnostics(undefined);
    expect(runtimeDiagnosticsEnabled()).toBe(false);
  });
});
//#endregion 🩺️RuntimeDiagnostics

//#region 🔁️ExtensionInvocation
/** 📦️ What a capability actually answers across the door: BYTES in the capability's own encoding
 * (`evaluate_invoke_json` emits JSON text), never a JS string — `PluginWasmHandle.invoke` returns the
 * guest's `respond` OK pack verbatim and the shell re-encodes it once, on the completion. */
const extensionAnswerBytes = (answer: unknown): Uint8Array => new TextEncoder().encode(JSON.stringify(answer));

describe("extension invocation completion ownership", () => {
  const fixture = extensionInvocationFixture;
  const entry = (handle: Record<string, unknown>) => ({ handle, manifest: {} }) as unknown as LoadedProgramState;
  const completionHandle = (complete: (instanceId: number, req: bigint, outcome: unknown) => Promise<unknown>) => ({
    captureExtensionCompletion: (instanceId: number, req: bigint) => ({ instanceId, req, assertActive: () => {}, complete: (outcome: unknown) => complete(instanceId, req, outcome) }),
  });
  const call = (requester: LoadedProgramState, extension?: LoadedProgramState) => runInvokeExtensionEffect(requester, extension, fixture.instanceId, fixture.extensionId, fixture.capability, JSON.stringify(fixture.request), BigInt(fixture.requestId));

  it("validates the language-neutral request and fault vectors with the schema oracle", () => {
    expect(rendererExport("ExtensionInvocationCompletionV1")(fixture.completion)).toBe(true);
    expect(fixture.completionFailures.every((row) => rendererExport("ExtensionCompletionRefusalV1")(row))).toBe(true);
    expect(fixture.missing.every((row) => rendererExport("ExtensionInvocationMissV1")(row))).toBe(true);
    expect(rendererExport("ExtensionInvocationFaultV1")(fixture.fault)).toBe(true);
    expect(rendererExport("ExtensionInvocationCompletionV1")({ ...fixture.completion, revision: 0 })).toBe(false);
  });

  it.each(fixture.missing)("faults the exact requester when $phase is unavailable", async ({ phase, code }) => {
    const { decodePackValue, decodeFaultFromWire } = await import("@semio-tech/framework-os");
    const complete = vi.fn(async () => {});
    await call(entry(completionHandle(complete)), phase === "extension" ? undefined : entry({}));
    expect(complete).toHaveBeenCalledOnce();
    const [instance, req, outcome] = complete.mock.calls[0] as unknown as [number, bigint, { fault: Uint8Array }];
    expect([instance, req]).toEqual([fixture.instanceId, BigInt(fixture.requestId)]);
    expect(outcome).toHaveProperty("fault");
    const fault = decodeFaultFromWire(Array.from(outcome.fault), decodePackValue);
    expect(fault?.code).toBe(code);
    expect(fault?.origin).toBe("os");
  });

  it("refuses a missing completion capability before executing the extension", async () => {
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    await expect(call(entry({}), entry({ invoke }))).rejects.toThrow("extension.completion-unavailable");
    expect(invoke).not.toHaveBeenCalled();
  });

  it("preserves a typed extension fault and sends exactly one completion", async () => {
    const { SemioFaultError } = await import("@semio-tech/framework");
    const { decodePackValue } = await import("@semio-tech/framework-os");
    const complete = vi.fn(async () => {});
    const invoke = vi.fn(async () => {
      throw new SemioFaultError(fixture.fault as ConstructorParameters<typeof SemioFaultError>[0]);
    });
    await call(entry(completionHandle(complete)), entry({ invoke }));
    expect(complete).toHaveBeenCalledOnce();
    const [instance, req, outcome] = complete.mock.calls[0] as unknown as [number, bigint, { fault: Uint8Array }];
    expect([instance, req]).toEqual([fixture.instanceId, BigInt(fixture.requestId)]);
    expect(decodePackValue(outcome.fault)).toEqual(fixture.fault);
  });

  it("propagates a failed completion without retrying it as a second fault completion", async () => {
    const failure = new Error("completion transport refused");
    const complete = vi.fn(async () => {
      throw failure;
    });
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    await expect(call(entry(completionHandle(complete)), entry({ invoke }))).rejects.toBe(failure);
    expect(invoke).toHaveBeenCalledOnce();
    expect(complete).toHaveBeenCalledOnce();
  });

  it("returns the requesting actor's completion publication to its host owner", async () => {
    const response = {
      output: fixture.response,
      mutations: [],
      inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] },
      requestedEffects: [{ notify: { message: fixture.completion.notification } }],
      uiScope: fixture.completion.uiScope,
      historyPatch: fixture.completion.historyPatch,
    };
    const complete = vi.fn(async () => response);
    expect(await call(entry(completionHandle(complete)), entry({ invoke: async () => extensionAnswerBytes(fixture.response) }))).toBe(response);
    expect(complete).toHaveBeenCalledOnce();
  });


  // 🔁️ The Rust law over the SAME fixture is `evaluate_invoke_json`'s
  // `the_evaluate_wire_answers_every_fixture_row` (`🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🦀️.rs`):
  // it produces the answer bytes, this twin proves the shell classifies and packs exactly those.
  it.each(extensionInvocationWireFixture.rows)("classifies the $name evaluate answer the way the extension SDK produced it", async (row) => {
    const { decodePackValue } = await import("@semio-tech/framework-os");
    const request = JSON.parse(row.requestJson) as Record<string, unknown>;
    const present = extensionInvocationWireFixture.requestFields.filter((field) => request[field] !== undefined);
    expect(present.length === extensionInvocationWireFixture.requestFields.length).toBe(row.outcome === "ok");
    const answer = row.outcome === "ok" ? Object.fromEntries((row.outputKeys ?? []).map((key) => [key, row.outputEmpty ? {} : { value: 1 }])) : undefined;
    const complete = vi.fn(async () => {});
    const invoke = vi.fn(async () => {
      if (answer === undefined) throw new Error(row.fault);
      return extensionAnswerBytes(answer);
    });
    await runInvokeExtensionEffect(entry(completionHandle(complete)), entry({ invoke }), fixture.instanceId, fixture.extensionId, extensionInvocationWireFixture.capability, row.requestJson, BigInt(fixture.requestId));
    // 🛑️ The shell hands every extension request an `AbortSignal` so a surface's declared cancel
    // can retire it from outside the per-actor queue (`📓️preview-eval-cancellation-2026-09-12.md`).
    expect(invoke).toHaveBeenCalledExactlyOnceWith(extensionInvocationWireFixture.capability, row.requestJson, { originInstanceId: fixture.instanceId, signal: expect.any(AbortSignal) });
    const [, , outcome] = complete.mock.calls[0] as unknown as [number, bigint, { ok?: Uint8Array; fault?: Uint8Array }];
    expect("ok" in outcome ? "ok" : "fault").toBe(row.outcome);
    if (row.outcome === "ok") expect(decodePackValue(outcome.ok!)).toEqual(answer);
    else expect(decodePackValue(outcome.fault!)).toMatchObject({ code: "extension.invoke-failed", message: row.fault });
  });

  it.each(fixture.genericRequests)("dispatches $capability without requiring unrelated domain fields", async ({ capability, request }) => {
    const complete = vi.fn(async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }));
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const requester = entry({ pluginId: "requester", ...completionHandle(complete) });
    const extension = entry({ pluginId: fixture.extensionId, invoke });
    const requestJson = JSON.stringify(request);
    const publish = vi.fn(async () => {});
    await dispatchInvokeExtensionEffect([requester, extension], { pluginId: "requester", instanceId: fixture.instanceId }, { req: BigInt(fixture.requestId), extensionId: fixture.extensionId, capability, requestJson }, publish);
    // 🛑️ The shell hands every extension request an `AbortSignal` so a surface's declared cancel
    // can retire it from outside the per-actor queue (`📓️preview-eval-cancellation-2026-09-12.md`).
    expect(invoke).toHaveBeenCalledExactlyOnceWith(capability, requestJson, { originInstanceId: fixture.instanceId, signal: expect.any(AbortSignal) });
    expect(complete).toHaveBeenCalledOnce();
    expect(publish).toHaveBeenCalledExactlyOnceWith(requester, await complete.mock.results[0]!.value);
  });

  // 🪪️ LAW: an invocation addresses an extension by the CONTRIBUTING PLUGIN's id. A producer whose
  // own domain names the extension differently (flow calls the text kernel `text`, its plugin is
  // `flow-extension-text`) must translate before it emits — this shell knows no topic vocabulary and
  // resolves nothing but `handle.pluginId`. Regression guard: the removed fallback scanned
  // `manifest.contributions` for an `extensionId` field no manifest carries, so the miss was silent
  // and every flow evaluation stalled (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  it("resolves the extension actor by plugin id and faults a flow-domain address", async () => {
    const { decodePackValue, decodeFaultFromWire } = await import("@semio-tech/framework-os");
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const address = async (extensionId: string) => {
      const complete = vi.fn(async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }));
      const requester = entry({ pluginId: "requester", ...completionHandle(complete) });
      const extension = { handle: { pluginId: fixture.extensionId, invoke }, manifest: { topicContributions: [{ topic: "flow.extension", payload: { extensionId: fixture.foreignAddress } }] } } as unknown as LoadedProgramState;
      await dispatchInvokeExtensionEffect(
        [requester, extension],
        { pluginId: "requester", instanceId: fixture.instanceId },
        { req: BigInt(fixture.requestId), extensionId, capability: fixture.capability, requestJson: JSON.stringify(fixture.request) },
        async () => {},
      );
      return complete.mock.calls[0] as unknown as [number, bigint, { ok?: Uint8Array; fault?: Uint8Array }];
    };
    const [, , resolved] = await address(fixture.extensionId);
    expect(invoke).toHaveBeenCalledOnce();
    expect(decodePackValue(resolved.ok!)).toEqual(fixture.response);
    const [, , missed] = await address(fixture.foreignAddress);
    expect(invoke).toHaveBeenCalledOnce();
    expect(decodeFaultFromWire(Array.from(missed.fault!), decodePackValue)?.code).toBe("extension.missing");
  });

  it("rejects a missing originating plugin rather than silently discarding its request", async () => {
    await expect(
      dispatchInvokeExtensionEffect(
        [],
        { pluginId: "requester", instanceId: fixture.instanceId },
        { req: BigInt(fixture.requestId), extensionId: fixture.extensionId, capability: fixture.capability, requestJson: JSON.stringify(fixture.request) },
        async () => {},
      ),
    ).rejects.toThrow("extension.requester-unavailable");
  });

  it("propagates publication refusal after delivering only one completion", async () => {
    const refusal = new Error("extension.requester-retired");
    const complete = vi.fn(async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }));
    const requester = entry({ pluginId: "requester", ...completionHandle(complete) });
    const extension = entry({ pluginId: fixture.extensionId, invoke: async () => extensionAnswerBytes(fixture.response) });
    await expect(
      dispatchInvokeExtensionEffect(
        [requester, extension],
        { pluginId: "requester", instanceId: fixture.instanceId },
        { req: BigInt(fixture.requestId), extensionId: fixture.extensionId, capability: fixture.capability, requestJson: JSON.stringify(fixture.request) },
        async () => {
          throw refusal;
        },
      ),
    ).rejects.toBe(refusal);
    expect(complete).toHaveBeenCalledOnce();
  });

  it("decodes a real response without losing the owning method receiver", async () => {
    const { decodePackValue } = await import("@semio-tech/framework-os");
    const complete = vi.fn(async () => {});
    const handle = {
      response: fixture.response,
      invoke: async function () {
        return extensionAnswerBytes(this.response);
      },
    };
    await call(entry(completionHandle(complete)), entry(handle));
    const [instance, req, outcome] = complete.mock.calls[0] as unknown as [number, bigint, { ok: Uint8Array }];
    expect([instance, req]).toEqual([fixture.instanceId, BigInt(fixture.requestId)]);
    expect(decodePackValue(outcome.ok)).toEqual(fixture.response);
  });

  it("captures the exact completion authority before executing extension evaluation", async () => {
    const events: string[] = [];
    const complete = vi.fn(async () => {
      events.push("complete");
      return { output: null };
    });
    const capture = vi.fn((instanceId: number, req: bigint) => {
      events.push("capture");
      return { instanceId, req, assertActive: () => {}, complete };
    });
    const invoke = vi.fn(async () => {
      events.push("invoke");
      return extensionAnswerBytes(fixture.response);
    });
    await call(entry({ captureExtensionCompletion: capture }), entry({ invoke }));
    expect(events).toEqual(["capture", "invoke", "complete"]);
    expect(capture).toHaveBeenCalledExactlyOnceWith(fixture.instanceId, BigInt(fixture.requestId));
    expect(complete).toHaveBeenCalledOnce();
  });

  it("does not evaluate when initial activation capture is refused", async () => {
    const failure = new Error("actor-activation.revoked");
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const complete = vi.fn(async () => ({ output: null }));
    await expect(
      call(
        entry({
          captureExtensionCompletion: () => {
            throw failure;
          },
        }),
        entry({ invoke }),
      ),
    ).rejects.toBe(failure);
    expect(invoke).not.toHaveBeenCalled();
    expect(complete).not.toHaveBeenCalled();
  });

  it.each(fixture.completionFailures)("refuses $kind before extension execution", async ({ kind, code }) => {
    const complete = vi.fn(async () => ({ output: null }));
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const capture = (instanceId: number, req: bigint) =>
      kind === "missing-lease"
        ? null
        : {
            instanceId: kind === "foreign-instance" ? instanceId + 1 : instanceId,
            req: kind === "foreign-request" ? req + 1n : req,
            assertActive: kind === "missing-guard" ? undefined : () => {},
            complete: kind === "missing-complete" ? undefined : complete,
          };
    await expect(call(entry({ captureExtensionCompletion: capture }), entry({ invoke }))).rejects.toThrow(code);
    expect(invoke).not.toHaveBeenCalled();
    expect(complete).not.toHaveBeenCalled();
  });

  it("does not deliver a result to an activation replaced during extension evaluation", async () => {
    const entered = Promise.withResolvers<void>();
    const release = Promise.withResolvers<Uint8Array>();
    let generation = 1;
    const complete = vi.fn(async () => ({ output: null }));
    const capture = (instanceId: number, req: bigint) => {
      const captured = generation;
      return {
        instanceId,
        req,
        assertActive: () => {
          if (captured !== generation) throw new Error("actor-activation.revoked");
        },
        complete,
      };
    };
    const pending = call(
      entry({ captureExtensionCompletion: capture }),
      entry({
        invoke: async () => {
          entered.resolve();
          return release.promise;
        },
      }),
    );
    const observed = expect(pending).rejects.toThrow("actor-activation.revoked");
    await entered.promise;
    generation += 1;
    release.resolve(extensionAnswerBytes(fixture.response));
    await observed;
    expect(complete).not.toHaveBeenCalled();
  });

  it("captures before the host queue and cancels queued evaluation after replacement", async () => {
    const { serializePerActor } = await import("../../🧱️elements/🔌️PluginRuntime/🟦️.tsx");
    const entered = Promise.withResolvers<void>();
    const release = Promise.withResolvers<void>();
    let generation = 1;
    const complete = vi.fn(async () => ({ output: null }));
    const capture = vi.fn((instanceId: number, req: bigint) => {
      const captured = generation;
      return {
        instanceId,
        req,
        assertActive: () => {
          if (captured !== generation) throw new Error("actor-activation.revoked");
        },
        complete,
      };
    });
    const requester = entry({ pluginId: "requester", captureExtensionCompletion: capture });
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const held = serializePerActor(`requester:${fixture.instanceId}`, async () => {
      entered.resolve();
      await release.promise;
    });
    await entered.promise;
    const publish = vi.fn(async () => {});
    const pending = dispatchInvokeExtensionEffect(
      [requester, entry({ pluginId: fixture.extensionId, invoke })],
      { pluginId: "requester", instanceId: fixture.instanceId },
      { req: BigInt(fixture.requestId), extensionId: fixture.extensionId, capability: fixture.capability, requestJson: JSON.stringify(fixture.request) },
      publish,
    );
    const observed = expect(pending).rejects.toThrow("actor-activation.revoked");
    const capturedBeforeRelease = capture.mock.calls.length;
    generation += 1;
    release.resolve();
    await held;
    await observed;
    expect(capturedBeforeRelease).toBe(1);
    expect(capture).toHaveBeenCalledOnce();
    expect(invoke).not.toHaveBeenCalled();
    expect(complete).not.toHaveBeenCalled();
    expect(publish).not.toHaveBeenCalled();
  });
});
//#endregion 🔁️ExtensionInvocation

//#region 📇️DescriptorAdmission
describe("descriptor load admission", () => {
  

  it.each(descriptorLoadFixture.rejected)("refuses $name before starting the actor runtime", async (vector) => {
    const originalFetch = globalThis.fetch;
    const initialize = vi.fn();
    const fetch = vi.fn(async () => new Response(vector.body, { status: vector.status, headers: { "content-type": vector.contentType } }));
    globalThis.fetch = stubFetch(fetch);
    try {
      await expect(resolveDescriptorBeforeRuntime(() => fetchDescriptorManifest(descriptorLoadFixture.pluginId, descriptorLoadFixture.moduleUrl), initialize)).rejects.toMatchObject({
        fault: { code: vector.fault, scope: { pluginId: descriptorLoadFixture.pluginId } },
      });
      expect(fetch).toHaveBeenCalledExactlyOnceWith(descriptorLoadFixture.descriptorUrl, undefined);
      expect(initialize).not.toHaveBeenCalled();
    } finally {
      globalThis.fetch = originalFetch;
    }
  });

  it("admits an actual descriptor including an extension's empty app roster", async () => {
    const originalFetch = globalThis.fetch;
    try {
      for (const apps of [descriptorLoadFixture.descriptor.manifest.apps, []]) {
        const descriptor = { manifest: { ...descriptorLoadFixture.descriptor.manifest, apps } };
        globalThis.fetch = stubFetch(async () => new Response(JSON.stringify(descriptor), { headers: { "content-type": "application/json" } }));
        expect(await fetchDescriptorManifest(descriptorLoadFixture.pluginId, descriptorLoadFixture.moduleUrl)).toEqual(descriptor.manifest);
      }
    } finally {
      globalThis.fetch = originalFetch;
    }
  });
});
//#endregion 📇️DescriptorAdmission

//#region 🎞️TutorialDocumentTrack
describe("tutorial document wire contract", () => {
  it("keeps native document-track names and bidirectional event order", () => {
    
    
    
    const document = tutorialDocumentFixture.document.map(({ at, kind }) => ({ at, kind: { ...kind, kind: "load" as const } }));
    const definition: TutorialDefinition = {
      id: "document-wire",
      title: "Document Wire",
      durationMs: 250,
      chapters: [],
      base: { ui: { activeUtilityByWindowId: {}, activePanelTabByGroup: {}, interactionSelection: {}, expandedTreeIds: [], commandPanelOpen: false }, cameras: [] },
      tracks: { narration: [], video: [], events: [], ui: [], document, camera: [], gestures: [] },
    };
    expect(validateTutorial(definition)).toBeNull();
    for (const vector of tutorialDocumentFixture.cases) {
      const slice = tutorialSlice(definition, vector.from, vector.to);
      expect(slice.forward).toBe(vector.forward);
      expect(slice.document.map(({ at }) => at)).toEqual(vector.expectedAt);
      expect(slice.document).toEqual(vector.expectedAt.map((at) => document.find((event) => event.at === at)));
    }
  });
});
//#endregion 🎞️TutorialDocumentTrack


import graphSliderFixture from "../../../../♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧫️fixtures/🎚️slider-overlay.json";
import dagVcsSchema from "../../../../♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🔣️.json" with { type: "json" };
import graphParameterFixture from "../../../../🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/🧫️fixtures/🔣️.json";
import graphPickFixture from "../../🧱️elements/🕸️NodeGraph/🧫️fixtures/🔣️pick-target.json";
import flowParameterSchema from "../../../../🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/🧬️schema/🔣️.json" with { type: "json" };
import * as flowSessionLoader from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import * as infiniteCanvasRenderer from "@semio-tech/canvas-react-renderer";
import { createFlowBrowserRuntime } from "@semio-tech/flow-core/🌐️flow-browser.js";
import { MockFlowBridge } from "../../../../🌊️flow/🕸️wasm/🧪️tests/🎭️mock-flow-bridge/🟦️.ts";
import flowAbi from "../../../../🌊️flow/🕸️wasm/🧬️schema/📡️abi/🔣️.json" with { type: "json" };
import flowBrowserRuntimeFixture from "../../../../🌊️flow/🕸️wasm/🧫️fixtures/🧑‍🤝‍🧑️browser-runtime/🔣️.json";
import flowWasmSchema from "../../../../🌊️flow/🕸️wasm/🧬️schema/🔣️.json" with { type: "json" };
import { cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, test, vi } from "vitest";
import { deriveUtilityNodes, actionSemanticsForKind, resolveWindowActions, resolveModeTools, partitionWindowMeasures, type ActionArgDef, type ActionDefinition, type AppDefinition, type AppModeDefinition, type AppWindowKindDefinition, type CommandDefinition, type UtilityDefinition, type WindowMeasure, type UtilityNode, type BuiltNode, type Component, type NodeGraphScene, type UiNodeRecord, type UiComponentSceneNode, type UiSnapshot, type ActionBinding, type UiIntent, type LayoutSpec, createMemoryStoragePort, createTurnOutcomeBroadcast, pendingPanelUiNode, type TurnOutcome, type ActionDescriptor } from "@semio-tech/framework";
import { progressPanelTabSelection, resolvePanelBranchBodyLeaf, resolveTranslationLabel, SelectionMarquee, uiDataLabel, formatKeybindingShortcut, buildKeysByActionId, type PanelTabNode, type TreeDataSection } from "@semio-tech/ui-react";
import { renderUiControl } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";
import { worldHoverPaintIdV1 } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";
import { worldInstanceHighlighted } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";
import type { GumballPose } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🟦️.tsx";
import {
  WorldOrbitProjectionSwitchPane,
  world3dProjectionPaneElementId,
  resolveClickInstanceIdFromProjected,
  world3dInstancePickUsesInteractionDomain,
  world3dMarqueePointerCaptureArmed,
  world3dProjectedAabbContainsClick,
  world3dFrameCameraFromBounds,
  world3dFrameDistanceForRadius,
  world3dBoundsRadius,
  world3dAutoFitOwed,
  world3dAutoFitKey,
  world3dFrameCameraFromInstances,
  world3dSuggestionsGestureArmed,
  world3dRetainLocalVortexHover,
  leftoverHoveredVortexFullIdV1,
  leftoverOverlayCarryingSelectionV1,
  leftoverSelectIdsMustNameHoverPickV1,
  leftoverOverlayCarryingUtilityV1,
  leftoverOverlayArmedBrushUtilityV1,
  leftoverTreeItemSelectedV1,
  leftoverWorldOverlayAppliesV1,
  mergeWorldInteractionWithLeftoverV1,
  mergeWorldSelectionWithLeftoverV1,
  gumballPreviewOwnedWorldPoint,
  gumballPreviewOwnedWorldDirection,
  worldGumballOwnerPoses,
  worldVorticesWithGumballPreview,
  worldAttractionsWithGumballPreview,
  world3dSuggestionsGestureConsumesContextMenu,
  world3dSuggestionsRightDownRoutesOnWindowCapture,
  worldVortexHitProxy,
  worldInstanceMeshRaycast,
  applyWorldInstanceMeshRaycast,
} from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";
import { leftoverInspectionPanelHash, leftoverInspectionRefreshScope, uiRefreshSectionUnchanged } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";

import { Canvas2dHost, canvasLayerDisplayLabel, worldToScreenLogical, readCanvas2dSurfaceColors, Board2dHost, applyBoard2dHighlightedIds, board2dCameraActionArgs, beginPuzzle2dPeerGesture, collectPuzzle2dLiveMirrorMutations, board2dGranularityById, board2dHoverActionArgs, latestBoard2dHoverId, parseBoard2dSuggestionMenu, board2dSuggestionMenuOwnsWindow, board2dSuggestionMenuItems, coalesceBoard2dEvents, parseBoard2dTransformFlags, board2dStatusJson, endPuzzle2dPeerGesture, mapContextMenuSpecs, surfaceContextMenuTitleKey, suggestionMenuItems, notifyPuzzle2dPeersGestureEnded, parsePuzzle2dCatalogueDragPayload, board2dPeers, puzzle2dDropPreviewJson, puzzle2dPeerOwnsGesture, puzzle2dScreenToWorld, puzzle2dWorldToScreen, pushPuzzle2dLiveMirrorMutations, registerBoard2dPeer, unregisterBoard2dPeer, NodeGraphHost, FlowGraphCanvasHost, InterpretedUiNode, builtNodeToSnapshot, resizeCanvasBackingStore, catalogueGhostDescriptorJson, computeDagMarqueeOverlay, flowCatalogueItemDescriptor, flowSurfaceRenderAllowed, flowRankCatalogueSuggestions, flowSpotlightSuggestionListScrollClass, nodeGraphHoverActionArgs, nodeGraphSelectionActionArgs, world3dHoverActionArgs, world3dSelectionActionArgs, interactionTargetsForInstances, WORLD3D_DEFAULT_INTERACTION_GRANULARITY, WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dMarkerInteractionTarget, world3dInstanceInteractionTarget, world3dInstanceInteractionTargets, world3dSelectionTargetsActionArgs, nodeGraphViewportActionArgs, parseNodeGraphSessionViewport, nodeGraphPickChannel, nodeGraphConnectionIsValid, parseDagWireTypeRefusalJson, portValueTypes, portValueTypesCompatible, wireRefusalLabelOptions, dagContentBounds, dagContentCoverage, dagEllipsizeByMeasure, dagFitCamera, dagStartupCamera, nodeGraphContentSignature, paintDagLabelOverlays, DAG_CONTENT_FIT_PADDING_PX, DAG_CONTENT_FRAMED_MIN_COVERAGE, DAG_CONTENT_REFIT_MAX_COVERAGE, DAG_LABEL_ELLIPSIS, parseCatalogueAppDragPayload, parseDagSliderOverlays, dagSliderValueText, GraphSliderOverlays, resolveHostSnapshotWidgetInstanceId, Paint2dHost, TableHost, tableStepperClampedDelta, tableStepperKeyDelta, resolveMapInteractionSync, GraphTimelineHost, TextEditorHost, lineRangeAt, multiSpanReplace, World3dHost, worldGhostMeshUrl, parsePuzzle3dCatalogueDragPayload, mergeWorldViewportCamera, raycastGroundPoint, resolveMeshStyle, resolveMeshSelectionPreviewStyle, semanticColorsFromPalette, celebrateWorldInstances, isWorldInstanceCelebrating, isCurveOnlyWorldMesh, meshBoundsCorners, resolveVortexPointerDownIntent, worldMeshMaterialRevision, worldVortexMaterialRevision, resolveWorldMergeMode, resolveWorldContextMenuTarget, world3dContextMenuSurfaceV1, shouldReattachWorldViewportCamera, worldCameraPoseApproxEqual, buildWorldCameraDispatchArgs, worldCameraSetCameraDispatchArgs, snapWorldPointToGrid, world3dViewportCameraSeedKey, world3dFitProjectionContent, world3dFramingInstances, world3dFrameVisibleOverlayOffered, world3dProjectionContentFrameMounted, world3dCameraDomJson, worldInstancePickBlocked, parseWorldTerrainStyle, clearWorldCatalogueDropPreview, getWorldCatalogueDropPreview, clearWorldSelectionPreview, getWorldSelectionPreview, clearWorldGumballTransformPreview, getWorldGumballTransformPreview, setWorldGumballTransformPreview, subscribeWorldGumballTransformPreview, pushPuzzle2dDropPreview, registerWorldCatalogueDropHost, setWorldCatalogueDropPreview, subscribeWorldCatalogueDropPreview, setWorldSelectionPreview, subscribeWorldSelectionPreview, worldCatalogueDropHostContainsPoint, InkCanvasHost, inkItemBounds, eraseInkStrokePointsInItem, inkParagraphsToHtml, inkResizeBounds, inkScaleItemWithinGroup, inkClipboardPayload, inkItemsFromClipboardPayload, screenToWorld, worldToScreen, type InkDocument, type InkStrokeItem, appBreadcrumb, appWindowLabel, adaptPluginHandle, fetchDescriptorManifest, resolveDescriptorBeforeRuntime, applyUiPatchToRetained, decodeWirePatchOps, UiDocumentStore, type UiInterpreterContext, UiPresenceOverlayContext, type UiPresenceOverlayEntry, serializeCommandIngressForActor, serializePerActor, applyUiRefreshResponseToCache, resolveAppBreadcrumb, buildUtilityRibbonSegments, buildActiveUtilityByWindowId, buildUiRefreshRequest, dedupeUtilityNodesById, flattenPanelTabLeaves, groupUtilityNodesByCategory, initialShellState, selectOpenConflicts, selectQuarantinedConflicts, isFlowGraphScene, mergeRecordPreservingIdentity, parseShellRoute, pluginAvailabilityRouteV1, pluginShouldReceiveContributions, pluginShouldEstablishSession, shellActorId, canonicalSurfaceId, reloadRetainsActiveApp, directoryCommandFromAction, mintDirectoryCommandRequestId, retainDirectoryCommandResult, DIRECTORY_COMMAND_RESULT_SLOTS, type DirectoryCommandResultSlotV1, type DirectoryCommandReceiptV1, type ShellAction, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD, AutoCheckinScheduler, canCheckIn, computeSyncPillState, syncPillText, ShellFaultBoundary, preserveJsonIdentity, reconcileUtilityPath, studioPanelFocusingSpawned, viewStateWithSpacePanel, findPressedUtilityLeafId, resolveUtilityNodes, resolveUtilities, panelTabDefinitionToNode, panelAnchorForGroup, integrateAppSettingsPanelTabsIntoFrameworkBranch, partitionFrameworkHistoryPanelTab, shellLabel, shellTabIcon, syncShellLabelLocale, uiIntentToActionDescriptor, actionStageKey, actionRequiresStagedForm, resolveKeybindingIntent, resolveUtilityActivation, isWorldTransformGumballMode, worldGumballConfigForProjection, gumballTransformDeltaBetweenPoses, gumballIdentityDelta, worldPaintStep, WORLD_PAINT_IDLE, world3dGumballSelectionArgsV1, world3dRelocateDragTargetV1, world3dRelocateDispatchArgsV1, world3dVolumeBrushOriginV1, world3dVolumeBrushCommits, gumballLivePreviewDeltaBetweenPoses, applyGumballLivePreviewDeltaToPose, WindowActionPane, resolveCommands, commandAddressKey, commandCategories, buildCommandCategoryTree, buildCommandCategoryTabs, buildOsCommands, createLatestAsyncDispatcher, createDirectionalAsyncDispatcher, createInFlightSkippingInterval, createCoalescingActionDispatcher, dispatchOsCommand, classifyWindowLayoutChange, buildNoteShellCommandAction, isShellOwnedCommandId, encodeEffectActionInvocation, encodeEffectCommandInvocation, TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS, mergeShellLockSources, resolveBootExampleId, resolveShellDefaults, resolveShellLocks, shouldPersistIntroductionSeen, shouldReplayIntroductionOnLoad, isEphemeralShellBrand, clearDurableShellStorage, type ResolvedCommand, type ResolvedActionArgDef, type ResolvedActionDefinition, type ResolvedToolDefinition, shellReducer, shellStateUnchanged, sortUtilityNodes, spawnedWindowChromeForKind, UtilityTree, type UiRefreshCache, UIFind, UIFindProvider, uiNodeToTreePanelConfig, UISearch, type UISearchItem, useUIFind, interpretUiNode, dagOverlayLabelFill, dagOverlayLabelFillHex, dispatchOpenedFiles, IMPORT_CHUNK_BYTES, importPayloadChunks, scheduleDispatchAction, sampleMediaFrameTimestampsMs, runTier2VideoFrames, requestMediaFramesSourceV1, runMediaFramesV1, createFrameworkDisplayPanelTabs, type DisplayHostApi, createFrameworkSettingsPanelTab, createFrameworkMarketplacePanelTab, type MarketplaceExtensionEntry, type MarketplaceHostApi, type MarketplacePluginEntry, type PluginPanelStatus, type PluginManifest, type PluginWasmHandle, resolveFrameworkLayoutSeed, retitleWindowLayoutNode, introductionTargetsWindow, windowMeasureTreeContainsId, renderWindowMeasuresTree, buildToolTabs, toolCategoryOpenPath, toolLeafInactiveRepress, toolIdFromPanelTabId, reconcileToolTabSelection, toolPanelTreeContentRevision, type ToolTabSelection, sceneToSyncPack, FrameworkOsShell, TutorialRecorder, synthesizeLocalizedLabel, resolveManifestLabel, type ShellPresencePeer, derivePeerInteractionByDomain, peerIdsSelecting, peerIdsHovering, SyncAttachCard } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { suggestionMenuOwnsWindow } from "../../🧱️elements/🎣️suggestion-submenu/🟦️.ts";
import {
  windowActionPaneNode,
  applyTutorialUiChangeToShell,
  applyTutorialUiSnapshotToShell,
  browserActorDispatchUiScopeV1,
  browserActorWindowConfigDispatchUiScopeV1,
  captureTutorialUiSnapshot,
  chordUsesCanonicalKeyTokens,
  clipboardWriteFragmentFromEffect,
  createUiRefreshCoalescerV1,
  hostEffectRefreshScopeV1,
  keyboardEventMatchesChord,
  mergeUiDirtyScopeV1,
  pasteActionWithRetainedFragment,
  pasteArgsFragment,
  programArmedToolRevealV1,
  typedOperationCompletionRefreshV1,
} from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import { decodeWorldProjectionTemplateId, encodeWorldProjectionTemplateId, worldSceneContentBounds, worldSceneContentBoundsKey } from "@semio-tech/infinite-world-r3f";

//#region 🔌️jsdom polyfills
// Renderer hosts measure through ResizeObserver; jsdom does not implement it.
class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}
if (!globalThis.ResizeObserver) globalThis.ResizeObserver = ResizeObserverMock as unknown as typeof ResizeObserver;
// Renderer navigation calls scrollIntoView; jsdom does not implement it.
if (!Element.prototype.scrollIntoView) Element.prototype.scrollIntoView = () => {};
//#endregion 🔌️jsdom polyfills

const noopAction = () => {};

/** 🌳️ Renders a panel body the way the dock does — through the tree's `emptyState`, full-width, never a property-layout control wrapper (ticket 26/09/02/PUZZLE-3D-END-TO-END, the inspection panel "shows nothing" defect). */
const panelTreePanelHost = (config: ReturnType<typeof uiNodeToTreePanelConfig>): ReactElement => config.emptyState as ReactElement;

//#region 🔌️PluginSessionOwnership
describe("plugin session ownership", () => {
  it("lets only the configured primary plugin establish a missing session", () => {
    expect(pluginShouldEstablishSession("demonstrator", "demonstrator", false)).toBe(true);
    expect(pluginShouldEstablishSession("cad", "demonstrator", false)).toBe(false);
    expect(pluginShouldEstablishSession("demonstrator", "demonstrator", true)).toBe(false);
  });

  it("configures only the active aggregate outside studio mode", () => {
    expect(pluginShouldReceiveContributions("demonstrator", "demonstrator", false)).toBe(true);
    expect(pluginShouldReceiveContributions("cad", "demonstrator", false)).toBe(false);
    expect(pluginShouldReceiveContributions("cad", "demonstrator", true)).toBe(true);
  });

  it("never hot-swaps a live plugin for a replayed availability snapshot", () => {
    for (const row of pluginAvailabilityRouteFixture.rows) {
      expect(pluginAvailabilityRouteV1(row.alreadyLoaded, row.loadedRebuiltAt ?? undefined, row.eventRebuiltAt ?? undefined), row.id).toBe(row.route);
    }
    const loaded = new Map<string, number | undefined>();
    const tally = { install: 0, "hot-swap": 0, drop: 0 };
    for (let connect = 0; connect < pluginAvailabilityRouteFixture.replay.connects; connect += 1) {
      for (const plugin of pluginAvailabilityRouteFixture.replay.snapshot) {
        const route = pluginAvailabilityRouteV1(loaded.has(plugin.pluginId), loaded.get(plugin.pluginId), plugin.rebuiltAt);
        tally[route] += 1;
        if (route !== "drop") loaded.set(plugin.pluginId, plugin.rebuiltAt);
      }
    }
    expect(tally.install).toBe(pluginAvailabilityRouteFixture.replay.expected.installs);
    expect(tally["hot-swap"]).toBe(pluginAvailabilityRouteFixture.replay.expected.hotSwaps);
    expect(tally.drop).toBe(pluginAvailabilityRouteFixture.replay.expected.drops);
  });
});
//#endregion 🔌️PluginSessionOwnership

//#region 🧪️Contract test fixtures
// 🧬️ MIGRATION (react-tests packet, ticket 26/08/20): helpers for building `UiSnapshot` fixtures
// directly against the semantic contract, mirroring `📃️UiDocumentStore`'s/`Interpreter`'s own inline
// test `leaf`/`snapshot` helpers so fixture shape stays one convention across the package, not a
// second drifting copy.
type ContractNodeSpec = {
  readonly key: string;
  readonly component: Component;
  readonly layout?: LayoutSpec;
  readonly disabled?: boolean;
  readonly bindings?: readonly ActionBinding[];
  readonly children?: readonly ContractNodeSpec[];
};

const CONTRACT_LEAF_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const CONTRACT_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" } as const;
const CONTRACT_ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false } as const;

function buildContractSnapshot(root: ContractNodeSpec): UiSnapshot {
  const nodes: UiNodeRecord[] = [];
  let nextId = 0;
  const walk = (spec: ContractNodeSpec): number => {
    const id = nextId;
    nextId += 1;
    const children = (spec.children ?? []).map(walk);
    nodes.push({
      id,
      key: spec.key,
      component: spec.component,
      layout: spec.layout ?? CONTRACT_LEAF_LAYOUT,
      style: CONTRACT_STYLE,
      activity: "idle",
      disabled: spec.disabled ?? false,
      transition: null,
      accessibility: CONTRACT_ACCESSIBILITY,
      bindings: [...(spec.bindings ?? [])],
      menu: null,
      children,
    });
    return id;
  };
  const rootId = walk(root);
  return { surface: "test", revision: 0, root: rootId, nodes, layoutEpoch: 0n } as UiSnapshot;
}

function buildContractNode(spec: ContractNodeSpec): BuiltNode {
  return {
    key: spec.key,
    component: spec.component,
    layout: spec.layout ?? CONTRACT_LEAF_LAYOUT,
    style: CONTRACT_STYLE,
    activity: "idle",
    disabled: spec.disabled ?? false,
    accessibility: CONTRACT_ACCESSIBILITY,
    bindings: [...(spec.bindings ?? [])],
    menu: null,
    children: (spec.children ?? []).map(buildContractNode),
  } as BuiltNode;
}

/** 🌳️ Renders `root` (and its nested `children`) through the real `📃️UiDocumentStore`/`interpretUiNode`
 * production path — never a hand-rolled shadow renderer — optionally under a `UiPresenceOverlayContext`
 * so hover/selection-driven markup (never a document field, per the contract) can be exercised too.
 *
 * 🪲️ PRODUCTION BUG (reported, not fixed — forbidden file): `📃️UiDocumentStore`'s `useUiNode` calls
 * `useSyncExternalStore(subscribe, getSnapshot)` with only two arguments — no `getServerSnapshot` —
 * which React's SSR path (`renderToStaticMarkup`/`renderToString`) throws on ("Missing
 * getServerSnapshot, which is required for server-rendered content"). Every pre-migration test in
 * this describe block used `renderToStaticMarkup` (the old Interpreter had no store/hook to trip
 * this on); this helper uses client-side `render()` instead, which does not hit the SSR path — a
 * test-only workaround, not a fix for the underlying gap. See `📃️UiDocumentStore/🟦️.tsx`'s
 * `useUiNode`/`useUiDocumentRoot`/`useUiDocumentRevision`.
 */
function renderContractTree(root: ContractNodeSpec, presenceByKey?: Readonly<Record<string, UiPresenceOverlayEntry>>): string {
  const store = new UiDocumentStore("test");
  store.loadSnapshot(buildContractSnapshot(root));
  const context: UiInterpreterContext = { store, onAction: noopAction, onIntent: () => {} };
  const tree = interpretUiNode(store, context);
  const element = presenceByKey ? createElement(UiPresenceOverlayContext.Provider, { value: { byKey: new Map(Object.entries(presenceByKey)) } }, tree) : (tree as ReactElement);
  const { container } = render(element);
  const markup = container.innerHTML;
  cleanup();
  return markup;
}
//#endregion 🧪️Contract test fixtures

it("keeps the interpreted canvas visible beneath expanded Actions chrome", () => {
  for (const row of canvasClearance.cases) {
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      const bottom = this.getAttribute("data-slot") === "window-engagement-overlay" ? row.chromeHeight : 720;
      return { x: 0, y: 0, top: 0, bottom, left: 0, right: 1280, width: 1280, height: bottom, toJSON: () => ({}) };
    });
    const store = new UiDocumentStore("canvas-clearance");
    store.loadSnapshot(buildContractSnapshot({ key: "canvas", component: { type: "surface", kind: row.kind, docSchema: `${row.kind}@1`, doc: { bytes: [] }, bindings: [] } }));
    const view = render(createElement("div", { "data-slot": "window-body" },
      createElement("div", { "data-slot": "window-engagement-overlay" }, "Actions"),
      createElement(ChromeAwareWindowScrollSurface, { ref: (el: HTMLDivElement | null) => { if (el) el.scrollTop = row.scrollTop; } },
        createElement(InterpretedUiNode, { store, onAction: noopAction, onIntent: noopAction, requestContextMenu: undefined }))));
    try {
      const surface = view.container.querySelector<HTMLElement>('[data-slot="window-dead-line-scroll"]')!;
      expect(Number.parseFloat(surface.style.paddingBlockStart) || 0, row.id).toBe(row.inset);
      expect(surface.scrollTop, row.id).toBe(row.scrollTop);
      expect(surface.querySelector('[role="application"]'), row.id).not.toBeNull();
    } finally { view.unmount(); bounds.mockRestore(); }
  }
  console.info("[DEBUG] Canvas clearance: four mounted production-interpreter fixture cases");
});

describe("framework sync utilities", () => {
  it("renders the real SyncAttachCard popover as a dismissible nonmodal dialog", () => {
    const close = vi.fn();
    const { getByRole } = render(
      createElement(SyncAttachCard, {
        activeUri: null,
        cardKind: "file",
        draftPath: "/tmp/document.json",
        syncUtilities: [],
        status: null,
        quarantinedConflicts: [],
        onAction: vi.fn(),
        onDraftPathChange: vi.fn(),
        onClose: close,
        onAttach: vi.fn(),
        onDetach: vi.fn(),
        onBrowsePath: vi.fn(),
      }),
    );
    const dialog = getByRole("dialog");
    expect(dialog.getAttribute("aria-modal")).toBeNull();
    expect(document.activeElement).toBe(getByRole("textbox"));
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    cleanup();
    expect(close).toHaveBeenCalledTimes(1);
  });

  it("offers localized file and folder picker affordances through the owned browse control", async () => {
    const browseFile = vi.fn();
    const browseFolder = vi.fn();
    await uiI18n.changeLanguage("en");
    const file = render(
      createElement(SyncAttachCard, {
        activeUri: null,
        cardKind: "file",
        draftPath: "",
        syncUtilities: [],
        status: null,
        quarantinedConflicts: [],
        onAction: vi.fn(),
        onDraftPathChange: vi.fn(),
        onClose: vi.fn(),
        onAttach: vi.fn(),
        onDetach: vi.fn(),
        onBrowsePath: browseFile,
      }),
    );
    const fileBrowse = document.querySelector<HTMLElement>("[data-semio-sync-browse]");
    expect(computeAccessibleName(fileBrowse!)).toBe("Browse");
    fileBrowse?.click();
    expect(browseFile).toHaveBeenCalledTimes(1);
    file.unmount();
    await uiI18n.changeLanguage("de");
    const folder = render(
      createElement(SyncAttachCard, {
        activeUri: null,
        cardKind: "folder",
        draftPath: "",
        syncUtilities: [],
        status: null,
        quarantinedConflicts: [],
        onAction: vi.fn(),
        onDraftPathChange: vi.fn(),
        onClose: vi.fn(),
        onAttach: vi.fn(),
        onDetach: vi.fn(),
        onBrowsePath: browseFolder,
      }),
    );
    const folderBrowse = document.querySelector<HTMLElement>("[data-semio-sync-browse]");
    expect(computeAccessibleName(folderBrowse!)).toBe("Durchsuchen");
    folderBrowse?.click();
    expect(browseFolder).toHaveBeenCalledTimes(1);
    folder.unmount();
    const remote = render(
      createElement(SyncAttachCard, {
        activeUri: null,
        cardKind: "remote",
        draftPath: "",
        syncUtilities: [],
        status: null,
        quarantinedConflicts: [],
        onAction: vi.fn(),
        onDraftPathChange: vi.fn(),
        onClose: vi.fn(),
        onAttach: vi.fn(),
        onDetach: vi.fn(),
        onBrowsePath: vi.fn(),
      }),
    );
    expect(remote.container.querySelector("[data-semio-sync-browse]")).toBeNull();
    remote.unmount();
    const { spacesUiLabel } = await import("../../🎯️targets/⚛️react/🟦️.tsx");
    expect(spacesUiLabel("ui.sync.browse", "en")).toBe("Browse…");
    expect(spacesUiLabel("ui.sync.browse", "de")).toBe("Durchsuchen…");
    await uiI18n.changeLanguage("en");
  });

  it("builds three sync backbone toggles", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os");
    const utilities = buildFrameworkSyncUtilities("file:///demo");
    expect(utilities).toHaveLength(3);
    expect(utilities.map((utility) => utility.id)).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
    expect(utilities[0]?.pressed).toBe(true);
  });

  it("has no active toggle when detached", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os");
    const utilities = buildFrameworkSyncUtilities(null);
    expect(utilities.every((utility) => !utility.pressed)).toBe(true);
  });

  it("groups File, Folder, and Remote under a single Sync category collection", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os");
    const utilities = buildFrameworkSyncUtilities("file:///demo");
    const grouped = groupUtilityNodesByCategory(utilities as unknown as UtilityNode[], ["sync"]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0]).toMatchObject({ id: "sync", kind: "collection" });
    expect(grouped[0].kind === "collection" ? grouped[0].children.map((child) => child.id) : []).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
  });

  it("matches the sync backbone path picker fixture", async () => {
    const fixture = (await import("../../../../../🧫️fixtures/🔗️sync-backbone-path-picker/🔣️.json")).default as {
      react: { helpers: string[]; hostIoOps?: string[] };
      wgpu: { hostIoOps: string[] };
    };
    const helpers = await import("../../🧱️elements/🛠️ShellHelpers/🟦️.tsx");
    for (const name of fixture.react.helpers) expect(typeof (helpers as Record<string, unknown>)[name]).toBe("function");
    const { default: hostIo } = await import("../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts?raw");
    for (const op of fixture.wgpu.hostIoOps) expect(hostIo).toContain(op);
  });
});

describe("live measure dispatch", () => {
  it("serializes document updates and skips stale slider values", async () => {
    const values: number[] = [];
    let finishFirst = () => {};
    const dispatch = createLatestAsyncDispatcher((value: number) => {
      values.push(value);
      if (value === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
    });

    dispatch(1);
    dispatch(2);
    dispatch(19);
    dispatch(24);
    expect(values).toEqual([1]);

    finishFirst();
    await vi.waitFor(() => expect(values).toEqual([1, 24]));
  });

  it("coalesces straight slider movement but preserves a down-up reversal", async () => {
    const values: number[] = [];
    const finishes: Array<() => void> = [];
    const dispatch = createDirectionalAsyncDispatcher(
      (value) =>
        new Promise<void>((resolve) => {
          values.push(value);
          finishes.push(resolve);
        }),
    );

    dispatch(20);
    dispatch(18);
    dispatch(12);
    dispatch(8);
    dispatch(14);
    dispatch(20);
    expect(values).toEqual([20]);

    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toEqual([20, 8]));
    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toEqual([20, 8, 20]));
    finishes.shift()?.();
  });

  it("caps queued direction reversals so a jittery drag cannot grow the queue unbounded", async () => {
    const values: number[] = [];
    const finishes: Array<() => void> = [];
    const dispatch = createDirectionalAsyncDispatcher(
      (value) =>
        new Promise<void>((resolve) => {
          values.push(value);
          finishes.push(resolve);
        }),
    );

    dispatch(0); // starts immediately, in flight
    // Many alternating up/down values while the first dispatch is still in flight — each reversal used
    // to push a new entry onto `queued` with no bound.
    for (let i = 1; i <= 40; i += 1) {
      dispatch(i % 2 === 0 ? 100 + i : -100 - i);
    }
    expect(values).toEqual([0]);

    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toHaveLength(2));
    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toHaveLength(3));
    finishes.shift()?.();
    // Never more than 3 total dispatches (the in-flight one plus at most the capped 2 queued) despite
    // 40 requested reversals.
    expect(values).toHaveLength(3);
  });
});

describe("in-flight skipping interval", () => {
  it("drops overlapping ticks instead of queueing them behind a slow run", async () => {
    const runs: number[] = [];
    let finishFirst = () => {};
    const timers: Array<() => void> = [];
    const stop = createInFlightSkippingInterval(
      () => {
        runs.push(runs.length + 1);
        if (runs.length === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
      },
      10,
      (fn) => {
        timers.push(fn);
        return 1;
      },
      () => {},
    );

    expect(timers).toHaveLength(1);
    timers[0]!();
    timers[0]!();
    timers[0]!();
    expect(runs).toEqual([1]);

    finishFirst();
    await Promise.resolve();
    timers[0]!();
    expect(runs).toEqual([1, 2]);
    stop();
  });

  it("gates on exactly what run returns — a discarded dispatch promise gates nothing", async () => {
    // 🏁️ The measured 2026-09-09 20:55 defect: `World3dHost`'s background tick body was
    // `() => { if (busy) return; dispatch(tick); }`, whose block form DISCARDS the dispatch
    // promise, so the in-flight flag cleared on the same microtask and 120 ms ticks queued into the
    // serialized guest until the per-actor turn queue overflowed. This pins the difference so the shape
    // cannot silently regress: same slow work, only the return value differs.
    const started: number[] = [];
    const timers: Array<() => void> = [];
    const never = new Promise<void>(() => {});
    // 🕰️ Ticks 120 ms apart are many microtask turns apart, so each `Promise.resolve(undefined)` the
    // swallowing form produces has long since cleared the flag by the next tick — the awaits below are
    // what make this test see the real interval, not one synchronous burst.
    const drive = async (run: () => unknown) => {
      timers.length = 0;
      const stop = createInFlightSkippingInterval(
        run,
        10,
        (fn) => (timers.push(fn), 1),
        () => {},
      );
      for (let tick = 0; tick < 3; tick += 1) {
        timers[0]!();
        await Promise.resolve();
        await Promise.resolve();
      }
      stop();
    };

    await drive(() => {
      started.push(started.length + 1);
      void never;
    });
    expect(started, "a run that swallows its dispatch cannot be gated — every tick fires").toHaveLength(3);

    started.length = 0;
    await drive(() => {
      started.push(started.length + 1);
      return never;
    });
    expect(started, "returning the dispatch is what keeps exactly one tick outstanding").toHaveLength(1);
  });
});

describe("coalescing action dispatcher", () => {
  it("dedupes unchanged values and keeps at most one in-flight dispatch", async () => {
    const calls: string[] = [];
    let finishFirst = () => {};
    const send = createCoalescingActionDispatcher<string | null>((value) => {
      calls.push(value ?? "null");
      if (calls.length === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
    });
    send("a");
    send("a");
    send("b");
    expect(calls).toEqual(["a"]);
    finishFirst();
    await Promise.resolve();
    expect(calls).toEqual(["a", "b"]);
    send("b");
    expect(calls).toEqual(["a", "b"]);
  });

  // 🏁️ The hover twin of the fill tick's "a discarded dispatch promise gates nothing" law. `World3dHost`'s
  // `dispatch` returns void — it drops the `onAction` promise that settles on the guest's
  // `OperationCompleted` frame — so a hover dispatcher wired to it coalesced NOTHING: wave B33 measured one
  // 70-move brush hover storm enqueuing 72 `interactionHover` + 85 `suggestionsTick` guest turns with 11/10
  // of them settled, after which the next user command (`addTargetVolume`) waited behind the whole backlog
  // and lost its 30 s budget (`📓️2026-09-12-wave-B33-full-run-vs-fresh-lane.md` §3). Same storm, same slow
  // guest, only the return value differs — so the shape cannot silently regress.
  it("gates on exactly what the dispatch returns — a swallowed round trip coalesces nothing", async () => {
    const storm = ["a", "b", "c", "d", "e"] as const;
    const drive = async (shape: (value: string, sent: readonly string[]) => unknown) => {
      const sent: string[] = [];
      const send = createCoalescingActionDispatcher<string>((value) => shape(value, (sent.push(value), sent)));
      for (const value of storm) {
        send(value);
        await Promise.resolve();
        await Promise.resolve();
      }
      return sent;
    };

    const pending = new Promise<void>(() => {});
    const swallowed = await drive(() => {
      void pending;
    });
    expect(swallowed, "a hover dispatcher whose callback drops the round trip sends one turn per pointer move").toEqual([...storm]);

    const awaited = await drive(() => pending);
    expect(awaited, "returning the round trip is what keeps exactly ONE hover outstanding while the guest is busy").toEqual(["a"]);
  });

  // 🩹️ A refused round trip must free the gate rather than wedge the lane closed — and must not surface as an
  // unhandled rejection now that the caller hands over a real awaitable (wave B33 §4).
  it("keeps flushing after a rejected round trip", async () => {
    const calls: string[] = [];
    const send = createCoalescingActionDispatcher<string>((value) => {
      calls.push(value);
      return calls.length === 1 ? Promise.reject(new Error("refused hover")) : Promise.resolve();
    });
    send("a");
    expect(calls).toEqual(["a"]);
    await Promise.resolve();
    await Promise.resolve();
    send("b");
    await Promise.resolve();
    expect(calls, "a refusal clears the in-flight flag, so the next hover still reaches the guest").toEqual(["a", "b"]);
  });
});

describe("ui refresh coalescing lane", () => {
  type LaneRequest = { readonly scope: UiDirtyScope; readonly utilities: Readonly<Record<string, string>>; readonly replaceBodies: boolean };
  const mergeLane = (owed: LaneRequest, next: LaneRequest): LaneRequest => ({ scope: mergeUiDirtyScopeV1(owed.scope, next.scope), utilities: next.utilities, replaceBodies: owed.replaceBodies || next.replaceBodies });
  /** 🧪️ One lane plus a hand-held pass: `settle()` finishes the pass the lane is currently running, so a
   * law can hold a pass open for exactly as long as it needs and read what the FOLLOW-UP carried. */
  const lane = () => {
    const taken: LaneRequest[] = [];
    let finish: { resolve: () => void; reject: (error: unknown) => void } | null = null;
    const coalescer = createUiRefreshCoalescerV1<LaneRequest>((request) => {
      taken.push(request);
      return new Promise<void>((resolve, reject) => {
        finish = { resolve, reject };
      });
    }, mergeLane);
    const settle = async (error?: unknown) => {
      const pass = finish;
      finish = null;
      if (error === undefined) pass?.resolve();
      else pass?.reject(error);
      await Promise.resolve();
      await Promise.resolve();
      await Promise.resolve();
    };
    return { coalescer, taken, settle };
  };
  const partial = (windowBodies: readonly string[], flags: Partial<UiDirtyScope & { readonly utilities: boolean }> = {}): UiDirtyScope => ({ kind: "partial", windowBodies: [...windowBodies], ...flags }) as UiDirtyScope;

  it("turns every refresh asked for during an in-flight pass into exactly ONE follow-up carrying their union", async () => {
    // 🤝️ Wave B37. The predecessor read its owed slot AFTER `await pass` with no `catch` between, and
    // joiners awaited the RUNNING pass rather than the one that covers them — so "my scope was re-taken"
    // was never true for a joiner, and one throw dropped the follow-up entirely.
    const { coalescer, taken, settle } = lane();
    const first = coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
    expect(
      taken.map((request) => request.scope),
      "the starter's own request is what the first pass runs",
    ).toEqual([partial(["world"])]);
    let firstSettled = false;
    void first.then(() => {
      firstSettled = true;
    });

    const second = coalescer.request({ scope: partial(["panel"], { utilities: true }), utilities: { "world-1": "brush" }, replaceBodies: true });
    let secondSettled = false;
    void second.then(() => {
      secondSettled = true;
    });
    void coalescer.request({ scope: partial(["measures-only"], { measures: true }), utilities: { "world-1": "brush" }, replaceBodies: false });
    expect(taken, "neither request may start a second pass while one is crossing into the guest").toHaveLength(1);

    await settle();
    expect(firstSettled, "the starter's promise settles on the pass that covered it, not on the whole drain").toBe(true);
    expect(secondSettled, "a joiner is only covered by the FOLLOW-UP, so it may not settle on the pass that was already running").toBe(false);
    expect(taken, "two requests during one pass owe exactly one follow-up").toHaveLength(2);
    expect(taken[1]!.scope, "the follow-up carries the union of everything asked for while the pass ran — a dropped field is a section that never repaints").toEqual({
      kind: "partial",
      windowBodies: ["panel", "measures-only"],
      panelBodies: [],
      utilities: true,
      tools: false,
      engagements: false,
      measures: true,
      labels: false,
    });
    expect(taken[1]!.replaceBodies, "a replace-bodies request may not be downgraded by merging").toBe(true);

    await settle();
    expect(secondSettled).toBe(true);
    expect(coalescer.passes(), "three requests, two passes — never one per request").toBe(2);
    expect(coalescer.busy()).toBe(false);
  });

  it("carries a utility armed by a host effect while a pass was in flight into the very next pass", async () => {
    // 🧰️ The measured defect: `engagementSubmit`'s typed `brush` verb emits `Effect::SetActiveUtility`,
    // `applyHostEffects` arms the host's utility map and then asks for the refresh that publishes it. With
    // a pass already crossing into the guest, that request is a JOIN — and the follow-up it is owed is the
    // only thing that can ever push the armed map. `runUiRefreshPass` re-derives
    // `activeUtilityByWindowId` from the live ref, so the union pass carries `brush` as long as it runs.
    const { coalescer, taken, settle } = lane();
    const armed: Record<string, string> = {};
    void coalescer.request({ scope: { kind: "full" }, utilities: { ...armed }, replaceBodies: false });
    expect(taken[0]!.utilities, "the pass that was already running predates the arm").toEqual({});

    armed["puzzle3d-main-perspective"] = "brush";
    void coalescer.request({ scope: { kind: "full" }, utilities: { ...armed }, replaceBodies: false });
    await settle();
    expect(taken, "the effect's own refresh is owed a pass — losing it leaves the armed pane publishing `select`").toHaveLength(2);
    expect(taken[1]!.utilities).toEqual({ "puzzle3d-main-perspective": "brush" });
    await settle();
    expect(coalescer.passes()).toBe(2);
  });

  it("answers a 50-request hover storm during one in-flight pass with exactly one follow-up", async () => {
    const { coalescer, taken, settle } = lane();
    void coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
    for (let index = 0; index < 50; index += 1) void coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
    expect(taken, "a storm behind a running pass costs nothing while it runs").toHaveLength(1);
    await settle();
    expect(taken, "50 requests during one pass owe ONE follow-up, never 50").toHaveLength(2);
    await settle();
    expect(coalescer.passes()).toBe(2);
    expect(coalescer.owedScope()).toBeNull();
  });

  it("runs the owed follow-up after a pass that REJECTED, and rejects only the requests that pass covered", async () => {
    const { coalescer, taken, settle } = lane();
    const starter = coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
    const starterOutcome = starter.then(() => "resolved" as const).catch((error: unknown) => `rejected:${String(error)}`);
    const joiner = coalescer.request({ scope: partial(["panel"]), utilities: {}, replaceBodies: false });
    const joinerOutcome = joiner.then(() => "resolved" as const).catch((error: unknown) => `rejected:${String(error)}`);

    await settle(new Error("guest refused the refresh"));
    // 🧯️ Asserted BEFORE awaiting either outcome: a lane that drops its owed work on a throw leaves both
    // promises pending forever, and this is what makes that a failed assertion instead of a hung suite.
    expect(taken, "a failed pass may not take the lane down with it").toHaveLength(2);
    expect(taken[1]!.scope).toEqual(partial(["panel"]));
    expect(await starterOutcome, "the boot refresh turns its own rejection into the session fault card — it must still arrive").toBe("rejected:Error: guest refused the refresh");
    await settle();
    expect(await joinerOutcome).toBe("resolved");
  });

  it("serves a request made from INSIDE a pass with the next pass instead of wedging the lane", async () => {
    // 🔁️ `runUiRefreshPass`'s own `requestedEffects` (a `flowEvalTick`, a `SetActiveUtility`) re-enter
    // this lane from inside the pass. A re-entrant caller that joined the RUNNING pass was waiting on the
    // promise its own pass resolves: the in-flight marker stayed pinned and nothing repainted again.
    const taken: UiDirtyScope[] = [];
    const finishers: Array<() => void> = [];
    let reentrant: Promise<void> | null = null;
    const coalescer = createUiRefreshCoalescerV1<LaneRequest>((request) => {
      taken.push(request.scope);
      return new Promise<void>((resolve) => {
        finishers.push(resolve);
        if (taken.length === 1) reentrant = coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
      });
    }, mergeLane);
    void coalescer.request({ scope: { kind: "full" }, utilities: {}, replaceBodies: false });
    expect(taken).toHaveLength(1);
    let reentrantSettled = false;
    void reentrant!.then(() => {
      reentrantSettled = true;
    });

    finishers[0]!();
    await Promise.resolve();
    await Promise.resolve();
    await Promise.resolve();
    expect(taken, "the pass's own request is owed the next pass").toEqual([{ kind: "full" }, partial(["world"])]);
    expect(reentrantSettled, "a re-entrant request is never settled by the pass that made it").toBe(false);
    finishers[1]!();
    await Promise.resolve();
    await Promise.resolve();
    expect(reentrantSettled).toBe(true);
    expect(coalescer.busy()).toBe(false);
  });

  it("asks for nothing at all on a none scope", async () => {
    const { coalescer, taken } = lane();
    await coalescer.request({ scope: { kind: "none" }, utilities: {}, replaceBodies: false });
    expect(taken).toHaveLength(0);
    expect(coalescer.passes()).toBe(0);
    expect(coalescer.busy()).toBe(false);
  });
});

describe("shell store reducer", () => {
  const baseState = () => initialShellState({ plugins: [], storage: createMemoryStoragePort() });
  const fixtureInteractionState = (value: {
    readonly selection: Readonly<Record<string, { readonly granularity: string; readonly ids: readonly string[]; readonly anchorId?: string }>>;
    readonly hover: Readonly<Record<string, { readonly channel: string; readonly ids: readonly string[] }>>;
    readonly activeMode: Readonly<Record<string, string>>;
    readonly activeGranularity: Readonly<Record<string, string>>;
  }): InteractionState => {
    const activeMode: Record<string, "single" | "multiple"> = {};
    for (const [domainId, mode] of Object.entries(value.activeMode)) {
      if (mode !== "single" && mode !== "multiple") throw new Error(`invalid fixture selection mode: ${mode}`);
      Object.defineProperty(activeMode, domainId, { value: mode, enumerable: true, writable: true, configurable: true });
    }
    return { selection: value.selection, hover: value.hover, activeMode, activeGranularity: value.activeGranularity };
  };
  const fixtureSelectionChange = (value: { readonly domainId: string; readonly granularity: string; readonly ids: readonly string[] }): TutorialUiChange => ({
    kind: "selection",
    domainId: value.domainId,
    granularity: value.granularity,
    ids: [...value.ids],
  });
  const fixtureTutorialSelection = (value: InteractionState["selection"]): TutorialUiSnapshot["interactionSelection"] => {
    const selection: TutorialUiSnapshot["interactionSelection"] = {};
    for (const [domainId, current] of Object.entries(value)) {
      Object.defineProperty(selection, domainId, { value: { ...current, ids: [...current.ids] }, enumerable: true, writable: true, configurable: true });
    }
    return selection;
  };
  const tutorialBridgeContext = (
    interactionSelection: () => InteractionState["selection"] = () => ({}),
    publishInteractionSelection: (selection: InteractionState["selection"]) => void = () => {},
  ): import("../../🧱️elements/🛠️ShellHelpers/🟦️.tsx").TutorialUiBridgeContext => ({
    session: null,
    restoreDialog: () => null,
    appLabelsOverlay: {
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
    },
    terminology: "native",
    locale: "en",
    interactionSelection,
    publishInteractionSelection,
  });

  it("shows terminal boot content instead of an infinite loading canvas", () => {
    for (const row of bootCanvasFixture) {
      const status = resolvePluginCanvasStatus(row.session, row.error, row.plugin as PluginPanelStatus, row.supervisor as PluginSupervisorState | undefined);
      expect(status === "loading", row.name).toBe(row.loading);
      const html = renderToStaticMarkup(
        createElement(Layout, {
          canvasStatus: status,
          canvasSkeleton: createElement("p", { role: "status" }, "Loading"),
          canvas: createElement("p", { role: "alert" }, row.error ?? "Ready"),
        }),
      );
      expect(html.includes('role="status"'), row.name).toBe(row.loading);
      expect(html.includes('role="alert"'), row.name).toBe(!row.loading);
    }
  });

  it("starts every panel anchor at the same 300px width", () => {
    const widths = Object.values(baseState().layout.panels).map((panel) => panel.size);
    expect(new Set(widths)).toEqual(new Set([300]));
  });

  it("toggles the overlays slice via a direct value without touching unrelated slices", () => {
    const state = baseState();
    const next = shellReducer(state, { type: "SET_SEARCH_OPEN", value: true });
    expect(next.overlays.searchOpen).toBe(true);
    expect(next.overlays.findOpen).toBe(false);
    expect(next.pluginRuntime).toBe(state.pluginRuntime);
    expect(next.uiPrefs).toBe(state.uiPrefs);
  });

  it("keeps the shell state's identity when a dispatch changes nothing, so a same-value refresh fan-out never re-renders the shell", () => {
    // 🪞️ One guest refresh fans out eight dispatches whose values are identity-preserved upstream
    // (`mergeRecordPreservingIdentity`); the reducer used to spread a fresh state anyway, and every
    // hover echo reconciled the whole shell tree.
    const state = baseState();
    expect(shellReducer(state, { type: "SET_SEARCH_OPEN", value: state.overlays.searchOpen })).toBe(state);
    expect(shellReducer(state, { type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID", value: (current) => current })).toBe(state);
    expect(shellReducer(state, { type: "SET_PANEL_BODY_STORE_BY_KEY", value: (current) => current })).toBe(state);
    expect(shellReducer(state, { type: "SET_APP_LABELS_OVERLAY", value: (current) => current })).toBe(state);
    // ✏️ A real change still produces a new state and a new slice — and only that slice.
    const opened = shellReducer(state, { type: "SET_SEARCH_OPEN", value: !state.overlays.searchOpen });
    expect(opened).not.toBe(state);
    expect(opened.overlays).not.toBe(state.overlays);
    expect(opened.windowUi).toBe(state.windowUi);
    expect(shellStateUnchanged(state, opened)).toBe(false);
    expect(shellStateUnchanged(state, state)).toBe(true);
  });

  it("keeps local interaction out of the shell state: a re-observation notifies nobody, a hover keeps every other field, an unchanged spawned-window projection is the same state", () => {
    const state = baseState();
    expect(Object.hasOwn(state, "interaction")).toBe(false);
    const read = (): InteractionState => ({ selection: { vortex: { granularity: "object", ids: ["seed-left-001"] } }, hover: {}, activeMode: { vortex: "multiple" }, activeGranularity: { vortex: "object" } });
    const interaction = createLocalInteractionStoreV1();
    let notifications = 0;
    const unsubscribe = interaction.subscribe(() => {
      notifications += 1;
    });
    interaction.observe(read());
    const observed = interaction.get();
    expect(observed).toEqual(read());
    expect(notifications).toBe(1);
    interaction.observe(read());
    expect(interaction.get()).toBe(observed);
    expect(notifications).toBe(1);
    interaction.observe({ ...read(), hover: { vortex: { channel: "pointer", ids: ["seed-left-001"] } } });
    const hovered = interaction.get();
    expect(notifications).toBe(2);
    expect(hovered.hover).toEqual({ vortex: { channel: "pointer", ids: ["seed-left-001"] } });
    expect(hovered.selection).toBe(observed.selection);
    expect(hovered.activeMode).toBe(observed.activeMode);
    expect(hovered.activeGranularity).toBe(observed.activeGranularity);
    unsubscribe();
    interaction.observe(read());
    expect(notifications).toBe(2);
    expect(shellReducer(state, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => current })).toBe(state);
    expect(shellReducer(state, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => current, fault: null })).toBe(state);
    const opened = shellReducer(state, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: { "puzzle-2::puzzle3d-main": "idle" } });
    expect(opened.spawnedWindow.spawnedWindowActivityByWindowId).toEqual({ "puzzle-2::puzzle3d-main": "idle" });
    expect(shellReducer(opened, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => mergeRecordPreservingIdentity(current, [["puzzle-2::puzzle3d-main", "idle"]]) })).toBe(opened);
  });

  it("starts, advances, and dismisses an introduction via SET_INTRODUCTION_STEP without touching unrelated slices", () => {
    const state = baseState();
    expect(state.overlays.introductionStepIndex).toBeNull();
    const started = shellReducer(state, { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(started.overlays.introductionStepIndex).toBe(0);
    expect(started.layout).toBe(state.layout);
    const advanced = shellReducer(started, { type: "SET_INTRODUCTION_STEP", value: (prev) => (prev ?? 0) + 1 });
    expect(advanced.overlays.introductionStepIndex).toBe(1);
    const dismissed = shellReducer(advanced, { type: "SET_INTRODUCTION_STEP", value: null });
    expect(dismissed.overlays.introductionStepIndex).toBeNull();
  });

  it("auto-starts each introduction launch once and keeps a skipped replay-on-load introduction dismissed", () => {
    const state = baseState();
    const started = shellReducer(state, { type: "AUTO_START_INTRODUCTION", key: "demonstrator:app" });
    expect(started.overlays.introductionStepIndex).toBe(0);
    expect(started.overlays.introductionAutoStartedKeys).toEqual(["demonstrator:app"]);

    const skipped = shellReducer(started, { type: "SET_INTRODUCTION_STEP", value: null });
    const repeatedAutoStart = shellReducer(skipped, { type: "AUTO_START_INTRODUCTION", key: "demonstrator:app" });
    expect(repeatedAutoStart.overlays.introductionStepIndex).toBeNull();
    expect(repeatedAutoStart.overlays).toBe(skipped.overlays);

    const manuallyAdvanced = shellReducer(repeatedAutoStart, { type: "SET_INTRODUCTION_STEP", value: 2 });
    const nextApp = shellReducer(manuallyAdvanced, { type: "AUTO_START_INTRODUCTION", key: "demonstrator:other-app" });
    expect(nextApp.overlays.introductionStepIndex).toBe(0);
    expect(nextApp.overlays.introductionAutoStartedKeys).toEqual(["demonstrator:app", "demonstrator:other-app"]);
  });

  it("COMPLETE_INTRODUCTION_INTERACTION appends and dedupes indices; SET_INTRODUCTION_STEP resets them", () => {
    const state = baseState();
    expect(state.overlays.introductionCompletedInteractions).toEqual([]);
    const started = shellReducer(state, { type: "SET_INTRODUCTION_STEP", value: 0 });
    const first = shellReducer(started, { type: "COMPLETE_INTRODUCTION_INTERACTION", index: 1 });
    expect(first.overlays.introductionCompletedInteractions).toEqual([1]);
    const second = shellReducer(first, { type: "COMPLETE_INTRODUCTION_INTERACTION", index: 0 });
    expect(second.overlays.introductionCompletedInteractions).toEqual([1, 0]);
    const deduped = shellReducer(second, { type: "COMPLETE_INTRODUCTION_INTERACTION", index: 1 });
    expect(deduped.overlays.introductionCompletedInteractions).toEqual([1, 0]);
    expect(deduped.layout).toBe(state.layout);
    const nextStep = shellReducer(deduped, { type: "SET_INTRODUCTION_STEP", value: 1 });
    expect(nextStep.overlays.introductionCompletedInteractions).toEqual([]);
  });

  it("opens, replaces, and closes a dialog via SET_DIALOG without touching unrelated slices", () => {
    const state = baseState();
    expect(state.overlays.dialog).toBeNull();
    const first = { openingId: 1, dialogId: "addObject", origin: dialogOriginFixture.owner, seedArgs: { objectKind: "Object" } };
    const opened = shellReducer(state, { type: "SET_DIALOG", value: first });
    expect(opened.overlays.dialog).toEqual(first);
    expect(opened.layout).toBe(state.layout);
    expect(opened.pluginRuntime).toBe(state.pluginRuntime);
    const second = { openingId: 2, dialogId: "confirmDelete", origin: dialogOriginFixture.owner };
    const replaced = shellReducer(opened, { type: "SET_DIALOG", value: second });
    expect(replaced.overlays.dialog).toEqual(second);
    const staleClose = shellReducer(replaced, { type: "CLOSE_DIALOG", openingId: first.openingId });
    expect(staleClose.overlays).toBe(replaced.overlays);
    const closed = shellReducer(staleClose, { type: "CLOSE_DIALOG", openingId: second.openingId });
    expect(closed.overlays.dialog).toBeNull();
  });

  it("toggles the layout slice via an updater function", () => {
    const state = baseState();
    const opened = shellReducer(state, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
    const toggled = shellReducer(opened, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: (prev) => !prev });
    expect(opened.layout.panels["top-left"].visible).toBe(true);
    expect(toggled.layout.panels["top-left"].visible).toBe(false);
    expect(toggled.overlays).toBe(opened.overlays);
  });

  it("toggles a middle anchor via SET_PANEL_VISIBLE the same way as a corner", () => {
    const state = baseState();
    const opened = shellReducer(state, { type: "SET_PANEL_VISIBLE", anchor: "top-middle", value: true });
    expect(opened.layout.panels["top-middle"].visible).toBe(true);
    expect(opened.layout.panels["top-left"].visible).toBe(state.layout.panels["top-left"].visible);
  });

  it("applies the neutral per-instance icon sequence through Reacts SET_WINDOW_ICON reducer", () => {
    
    
    
    let state = shellReducer(baseState(), {
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: windowIconOverrideFixture.windowIds.slice(1).map((id) => ({ id, windowKindId: windowIconOverrideFixture.windowKindId, title: id })),
    });
    const resolvedIcon = (windowId: string, projectionIconId?: string) => state.layout.windowIconsById[windowId] ?? projectionIconId ?? windowIconOverrideFixture.kindIconId;
    for (const [index, command] of windowIconOverrideFixture.commands.entries()) {
      if (index === windowIconOverrideFixture.commands.length - 1) {
        expect(resolvedIcon(windowIconOverrideFixture.projectionFallback.windowId, windowIconOverrideFixture.projectionFallback.expectedIconId)).toBe(windowIconOverrideFixture.projectionFallback.expectedIconId);
      }
      state = shellReducer(state, { type: "SET_WINDOW_ICON", windowId: command.windowId, iconId: command.iconId as never });
      expect(resolvedIcon(command.windowId, command.windowId === windowIconOverrideFixture.projectionFallback.windowId ? windowIconOverrideFixture.projectionFallback.expectedIconId : undefined)).toBe(command.expectedIconId);
    }
    expect(state.layout.windowIconsById.main).toBe("diamond");
    expect(state.layout.windowIconsById["main-2"]).toBe("circle");
    expect(resolvedIcon(windowIconOverrideFixture.projectionFallback.windowId, windowIconOverrideFixture.projectionFallback.expectedIconId)).toBe("star");
    expect(resolvedIcon(windowIconOverrideFixture.kindFallback.windowId)).toBe(windowIconOverrideFixture.kindFallback.expectedIconId);
  });

  it("rewrites window titles via SET_WINDOW_TITLE for extras and base kinds", () => {
    const state = shellReducer(baseState(), {
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: [{ id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "Top" }],
    });
    const renamedExtra = shellReducer(state, { type: "SET_WINDOW_TITLE", windowId: "puzzle3d-main-top", title: "Front" });
    expect(renamedExtra.layout.windowTitlesById["puzzle3d-main-top"]).toBe("Front");
    expect(renamedExtra.layout.extraWindowInstances[0]?.title).toBe("Front");
    expect(renamedExtra.overlays).toBe(state.overlays);
    const renamedBase = shellReducer(renamedExtra, { type: "SET_WINDOW_TITLE", windowId: "puzzle3d-main", title: "Isometric" });
    expect(renamedBase.layout.windowTitlesById["puzzle3d-main"]).toBe("Isometric");
    expect(renamedBase.layout.extraWindowInstances[0]?.title).toBe("Front");
  });

  it("resets the dock override, every anchor's active path/visible/size, drill-down memory, and tree expansion via RESET_DOCK", () => {
    const state = baseState();
    const rearranged = shellReducer(state, {
      type: "SET_DOCK_OVERRIDE",
      value: {
        version: 3,
        anchors: { "top-left": [{ id: "moved" }], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      },
    });
    const withPath = shellReducer(rearranged, { type: "SET_PANEL_PATH", anchor: "top-left", value: ["moved"] });
    const withVisible = shellReducer(withPath, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
    const withSize = shellReducer(withVisible, { type: "SET_PANEL_SIZE", anchor: "top-left", value: 999 });
    const withMemory = shellReducer(withSize, { type: "SET_PANEL_PATH_MEMORY", value: { moved: "child" } });
    const withTreeOpen = shellReducer(withMemory, { type: "SET_TREE_OPEN_STATE", id: "unit:section", open: true });
    const reset = shellReducer(withTreeOpen, { type: "RESET_DOCK" });
    expect(reset.layout.dockOverride).toBeNull();
    expect(reset.layout.panels["top-left"].path).toEqual([]);
    expect(reset.layout.panels["top-left"].visible).toBe(false);
    expect(reset.layout.panels["top-left"].size).toBe(state.layout.panels["top-left"].size);
    expect(reset.layout.panelPathMemory).toEqual({});
    expect(reset.layout.treeOpenStates).toEqual({});
  });

  it("HYDRATE_DOCK_UI restores a persisted size for the top-middle anchor", () => {
    const state = baseState();
    const hydrated = shellReducer(state, { type: "HYDRATE_DOCK_UI", value: { version: 3, anchors: { "top-middle": { visible: true, size: 420 } } } });
    expect(hydrated.layout.panels["top-middle"].visible).toBe(true);
    expect(hydrated.layout.panels["top-middle"].size).toBe(420);
    expect(hydrated.layout.panels["top-left"]).toEqual(state.layout.panels["top-left"]);
  });

  it("updates the uiPrefs slice and leaves the sync slice referentially unchanged", () => {
    const state = baseState();
    const next = shellReducer(state, { type: "SET_UI_DRIVER_ID", value: "compact" });
    expect(next.uiPrefs.uiDriverId).toBe("compact");
    expect(next.sync).toBe(state.sync);
  });

  it("action-panel slice: fold/expand/stage/reset/active-utility update only their own keys and preserve identity on no-operations", () => {
    const state = baseState();

    const folded = shellReducer(state, { type: "SET_ACTION_PANE_FOLDED", windowId: "w1", value: false });
    expect(folded.actionPane.foldedByWindowId).toEqual({ w1: false });
    expect(folded.layout).toBe(state.layout);
    // no-operation fold keeps the whole slice referentially stable
    expect(shellReducer(folded, { type: "SET_ACTION_PANE_FOLDED", windowId: "w1", value: false }).actionPane).toBe(folded.actionPane);

    const expanded = shellReducer(folded, { type: "SET_ACTION_PANE_EXPANDED", windowId: "w1", value: "extrude" });
    expect(expanded.actionPane.expandedByWindowId).toEqual({ w1: "extrude" });
    expect(shellReducer(expanded, { type: "SET_ACTION_PANE_EXPANDED", windowId: "w1", value: "extrude" }).actionPane).toBe(expanded.actionPane);

    const staged = shellReducer(expanded, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 });
    expect(staged.actionPane.stagedArgsByKey).toEqual({ "w1:extrude": { depth: 3 } });
    const stagedMore = shellReducer(staged, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "segments", value: 2 });
    expect(stagedMore.actionPane.stagedArgsByKey["w1:extrude"]).toEqual({ depth: 3, segments: 2 });
    expect(shellReducer(stagedMore, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 }).actionPane).toBe(stagedMore.actionPane);

    const reset = shellReducer(stagedMore, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" });
    expect(reset.actionPane.stagedArgsByKey["w1:extrude"]).toBeUndefined();
    // reset keeps the panel expanded
    expect(reset.actionPane.expandedByWindowId["w1"]).toBe("extrude");
    expect(shellReducer(reset, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" }).actionPane).toBe(reset.actionPane);

    const activated = shellReducer(reset, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: "pen" });
    expect(activated.actionPane.activeUtilityByWindowId).toEqual({ w1: "pen" });
    expect(shellReducer(activated, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: "pen" }).actionPane).toBe(activated.actionPane);
    const deactivated = shellReducer(activated, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: null });
    expect(deactivated.actionPane.activeUtilityByWindowId["w1"]).toBeNull();
  });

  it("actionPane slice: SET_ACTIVE_TOOL updates only activeToolId and preserves identity on no-operations (mode-scoped, not per-window)", () => {
    const state = baseState();
    expect(state.actionPane.activeToolId).toBeNull();

    const activated = shellReducer(state, { type: "SET_ACTIVE_TOOL", toolId: "fill" });
    expect(activated.actionPane.activeToolId).toBe("fill");
    expect(activated.actionPane.activeUtilityByWindowId).toBe(state.actionPane.activeUtilityByWindowId);
    expect(shellReducer(activated, { type: "SET_ACTIVE_TOOL", toolId: "fill" }).actionPane).toBe(activated.actionPane);

    const deactivated = shellReducer(activated, { type: "SET_ACTIVE_TOOL", toolId: null });
    expect(deactivated.actionPane.activeToolId).toBeNull();
  });

  it("commandPanel slice: expand/collapse and stage/reset update only their own keys and preserve identity on no-operations (category active/fold state now lives in layout.panels['bottom-middle'], not this slice)", () => {
    const state = baseState();

    const expanded = shellReducer(state, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" });
    expect(expanded.commandPanel.expandedCommandId).toBe("os.setThemeId");
    expect(expanded.layout).toBe(state.layout);
    expect(shellReducer(expanded, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" }).commandPanel).toBe(expanded.commandPanel);

    const staged = shellReducer(expanded, { type: "STAGE_COMMAND_ARG", commandId: "os.setThemeId", argId: "themeId", value: "semio" });
    expect(staged.commandPanel.stagedArgsByCommandId).toEqual({ "os.setThemeId": { themeId: "semio" } });
    expect(shellReducer(staged, { type: "STAGE_COMMAND_ARG", commandId: "os.setThemeId", argId: "themeId", value: "semio" }).commandPanel).toBe(staged.commandPanel);

    const reset = shellReducer(staged, { type: "RESET_COMMAND_ARGS", commandId: "os.setThemeId" });
    expect(reset.commandPanel.stagedArgsByCommandId["os.setThemeId"]).toBeUndefined();
    expect(shellReducer(reset, { type: "RESET_COMMAND_ARGS", commandId: "os.setThemeId" }).commandPanel).toBe(reset.commandPanel);
  });

  it("command palette category is the bottom-middle anchor's own SET_PANEL_PATH; the UI's category-switch handler additionally dispatches SET_COMMAND_EXPANDED:null (reproducing the old single-action collapse-on-switch behavior across two actions)", () => {
    const state = baseState();
    const onCategory = shellReducer(state, { type: "SET_PANEL_PATH", anchor: "bottom-middle", value: ["framework.category.command", "command.category.appearance"] });
    expect(onCategory.layout.panels["bottom-middle"].path).toEqual(["framework.category.command", "command.category.appearance"]);

    const expanded = shellReducer(onCategory, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" });
    expect(expanded.commandPanel.expandedCommandId).toBe("os.setThemeId");

    const switchedPath = shellReducer(expanded, { type: "SET_PANEL_PATH", anchor: "bottom-middle", value: ["framework.category.command", "command.category.layout"] });
    const switched = shellReducer(switchedPath, { type: "SET_COMMAND_EXPANDED", value: null });
    expect(switched.layout.panels["bottom-middle"].path).toEqual(["framework.category.command", "command.category.layout"]);
    expect(switched.commandPanel.expandedCommandId).toBeNull();
  });

  it("tutorial slice: SET_TUTORIAL starts a tutorial, resets rate/deviated, and clears an active introduction (mutual exclusivity)", () => {
    const introducing = shellReducer(baseState(), { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(introducing.overlays.introductionStepIndex).toBe(0);
    const started = shellReducer(introducing, { type: "SET_TUTORIAL", value: "welcome-tour" });
    expect(started.tutorial).toEqual({ activeTutorialId: "welcome-tour", playing: false, rate: 1, muted: false, captionsOn: true, recording: false, deviated: false });
    expect(started.overlays.introductionStepIndex).toBeNull();
  });

  it("tutorial slice: SET_INTRODUCTION_STEP (non-null) clears an active tutorial (mutual exclusivity, reverse direction)", () => {
    const started = shellReducer(baseState(), { type: "SET_TUTORIAL", value: "welcome-tour" });
    const playing = shellReducer(started, { type: "SET_TUTORIAL_PLAYING", value: true });
    expect(playing.tutorial.playing).toBe(true);
    const introduced = shellReducer(playing, { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(introduced.tutorial.activeTutorialId).toBeNull();
    expect(introduced.tutorial.playing).toBe(false);
    expect(introduced.overlays.introductionStepIndex).toBe(0);
  });

  it("tutorial slice: play/pause resets deviated only when transitioning to playing; rate/muted/captions/recording/deviated update independently", () => {
    const started = shellReducer(baseState(), { type: "SET_TUTORIAL", value: "welcome-tour" });
    const deviated = shellReducer(started, { type: "SET_TUTORIAL_DEVIATED", value: true });
    expect(deviated.tutorial.deviated).toBe(true);
    const stillPaused = shellReducer(deviated, { type: "SET_TUTORIAL_PLAYING", value: false });
    expect(stillPaused.tutorial.deviated).toBe(true);
    const resumed = shellReducer(deviated, { type: "SET_TUTORIAL_PLAYING", value: true });
    expect(resumed.tutorial.deviated).toBe(false);
    const rated = shellReducer(resumed, { type: "SET_TUTORIAL_RATE", value: 2 });
    expect(rated.tutorial.rate).toBe(2);
    const muted = shellReducer(rated, { type: "SET_TUTORIAL_MUTED", value: true });
    expect(muted.tutorial.muted).toBe(true);
    const captionsOff = shellReducer(muted, { type: "SET_TUTORIAL_CAPTIONS", value: false });
    expect(captionsOff.tutorial.captionsOn).toBe(false);
    const recording = shellReducer(captionsOff, { type: "SET_TUTORIAL_RECORDING", value: true });
    expect(recording.tutorial.recording).toBe(true);
  });

  it("APPLY_TUTORIAL_UI_SNAPSHOT restores shell-owned fields without forging actor-owned interaction state", () => {
    const state = baseState();
    const snapshot = shellReducer(state, {
      type: "APPLY_TUTORIAL_UI_SNAPSHOT",
      snapshot: {
        activeWindowId: "puzzle3d-main",
        shellLayout: { kind: "window", id: "puzzle3d-main" },
        extraWindowInstances: [],
        panelPatches: { "top-left": { visible: true, path: ["catalogue"] } },
        treeOpenStates: { "catalogue.section": true },
        activeUtilityByWindowId: { "puzzle3d-main": "transform" },
        activeToolId: "fill",
        dialog: { openingId: 3, dialogId: "addObject", origin: dialogOriginFixture.owner },
        commandPanelOpen: true,
      },
    });
    expect(snapshot.layout.activeWindowId).toBe("puzzle3d-main");
    expect(snapshot.layout.panels["top-left"]).toMatchObject({ visible: true, path: ["catalogue"] });
    expect(snapshot.layout.treeOpenStates).toEqual({ "catalogue.section": true });
    expect(snapshot.actionPane.activeUtilityByWindowId).toEqual({ "puzzle3d-main": "transform" });
    expect(snapshot.actionPane.activeToolId).toBe("fill");
    expect(snapshot.overlays.dialog).toEqual({ openingId: 3, dialogId: "addObject", origin: dialogOriginFixture.owner });
    expect(snapshot.overlays.searchOpen).toBe(true);
    expect(Object.hasOwn(snapshot, "interaction")).toBe(false);
    expect(snapshot.pluginRuntime).toBe(state.pluginRuntime);
  });

  it("validates the language-neutral interaction recording vectors against the canonical schema", () => {
    const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
    ajv;
    ajv.addSchema(interactionSchema).addSchema(rendererSchema);
    const validateCapture = ajv.getSchema(`${rendererSchema.$id}#/$defs/TutorialInteractionCaptureCapture`)!;
    expect(validateCapture(tutorialInteractionFixture.capture), JSON.stringify(validateCapture.errors)).toBe(true);
    expect(decodeLocalInteractionCaptureJson(new TextEncoder().encode(JSON.stringify(tutorialInteractionFixture.capture)))).toEqual(tutorialInteractionFixture.capture);
    expect(validateCapture({ ...tutorialInteractionFixture.capture, extra: true })).toBe(false);
  });

  it("decodes the bounded actor interaction capture and rejects noncanonical authority or state", () => {
    const bytes = new TextEncoder().encode(JSON.stringify(tutorialInteractionFixture.capture));
    expect(decodeLocalInteractionCaptureJson(bytes)).toEqual(tutorialInteractionFixture.capture);
    expect(() => decodeLocalInteractionCaptureJson(new TextEncoder().encode(JSON.stringify({ ...tutorialInteractionFixture.capture, extra: true })))).toThrow("local-interaction.capture");
    expect(() => decodeLocalInteractionCaptureJson(new TextEncoder().encode(JSON.stringify({ ...tutorialInteractionFixture.capture, identity: { ...tutorialInteractionFixture.capture.identity, generation: "01" } })))).toThrow(
      "local-interaction.identity.generation",
    );
    expect(() =>
      decodeLocalInteractionCaptureJson(new TextEncoder().encode(JSON.stringify({ ...tutorialInteractionFixture.capture, state: { ...tutorialInteractionFixture.capture.state, selection: { mesh: { granularity: "face", ids: ["same", "same"] } } } }))),
    ).toThrow("local-interaction.state.selection.mesh.ids");
    expect(() => decodeLocalInteractionCaptureJson(new Uint8Array(LOCAL_INTERACTION_CAPTURE_MAX_BYTES + 1))).toThrow("local-interaction.capture-length");
  });

  it("captures the observed typed interaction selection without aliasing ids or special domain keys", () => {
    const observed = fixtureInteractionState(tutorialInteractionFixture.observed);
    const selection = { ...observed.selection };
    Object.defineProperty(selection, "__proto__", { value: { granularity: "node", ids: ["prototype-safe"] }, enumerable: true, writable: true, configurable: true });
    const interaction = createLocalInteractionStoreV1();
    interaction.observe({ ...observed, selection });
    const snapshot = captureTutorialUiSnapshot(baseState(), interaction.get(), null);
    expect(snapshot.interactionSelection.mesh).toEqual(tutorialInteractionFixture.snapshotSelection.mesh);
    expect(snapshot.interactionSelection["special.domain"]).toEqual(tutorialInteractionFixture.snapshotSelection["special.domain"]);
    expect(snapshot.interactionSelection).not.toBe(interaction.get().selection);
    expect(snapshot.interactionSelection.mesh?.ids).not.toBe(interaction.get().selection.mesh?.ids);
    expect(Object.hasOwn(snapshot.interactionSelection, "__proto__")).toBe(true);
    expect(snapshot.interactionSelection["__proto__"]).toEqual({ granularity: "node", ids: ["prototype-safe"] });
  });

  it("plays full and sparse typed selections through the Shell reducer and the local interaction store with JSON Patch parity", () => {
    let state = baseState();
    const interaction = createLocalInteractionStoreV1(fixtureInteractionState(tutorialInteractionFixture.playbackBefore));
    const dispatch = (action: ShellAction) => {
      state = shellReducer(state, action);
    };
    const bridge = tutorialBridgeContext(
      () => interaction.get().selection,
      (selection) => interaction.observe({ ...interaction.get(), selection }),
    );
    applyTutorialUiSnapshotToShell(
      dispatch,
      {
        activeUtilityByWindowId: {},
        activePanelTabByGroup: {},
        interactionSelection: fixtureTutorialSelection(fixtureInteractionState(tutorialInteractionFixture.observed).selection),
        expandedTreeIds: [],
        commandPanelOpen: false,
      },
      bridge,
    );
    expect(interaction.get()).toEqual(applyPatch(structuredClone(tutorialInteractionFixture.playbackBefore), [{ op: "replace", path: "/selection", value: structuredClone(tutorialInteractionFixture.snapshotSelection) }], true, false).newDocument);

    state = baseState();
    interaction.observe(fixtureInteractionState(tutorialInteractionFixture.playbackBefore));
    applyTutorialUiChangeToShell(dispatch, fixtureSelectionChange({ ...tutorialInteractionFixture.delta, domainId: "mesh" }), bridge);
    const oracleAfterDelta = applyPatch(
      structuredClone(tutorialInteractionFixture.playbackBefore),
      [{ op: "replace", path: "/selection/mesh", value: { granularity: tutorialInteractionFixture.delta.granularity, ids: [...tutorialInteractionFixture.delta.ids] } }],
      true,
      false,
    ).newDocument;
    expect(interaction.get()).toEqual(oracleAfterDelta);
    expect(interaction.get()).toEqual(tutorialInteractionFixture.afterDelta);
    applyTutorialUiChangeToShell(dispatch, fixtureSelectionChange({ ...tutorialInteractionFixture.clearDelta, domainId: "mesh" }), bridge);
    expect(interaction.get()).toEqual(tutorialInteractionFixture.afterClear);
  });

  it("projects tutorial selection playback through ordinary interaction CQRS actions", () => {
    expect(tutorialInteractionSelectionActions("controller", fixtureInteractionState(tutorialInteractionFixture.observed).selection)).toEqual([
      { controllerId: "controller", action: "clearSelection" },
      { controllerId: "controller", action: "setSelectionMode", args: { domainId: "mesh", mode: "multiple" } },
      {
        controllerId: "controller",
        action: "interactionSelect",
        args: {
          domainId: "mesh",
          merge: "replace",
          method: "pick",
          targets: JSON.stringify([
            { granularity: "face", id: "face,west" },
            { granularity: "face", id: "face-east" },
          ]),
        },
      },
      { controllerId: "controller", action: "interactionSelect", args: { domainId: "mesh", merge: "additive", method: "pick", targets: JSON.stringify([{ granularity: "face", id: "face,west" }]) } },
      { controllerId: "controller", action: "interactionSelect", args: { domainId: "special.domain", merge: "replace", method: "pick", targets: JSON.stringify([{ granularity: "node", id: "owned" }]) } },
    ]);
  });

  it("records comma-bearing selection changes and explicit domain clearing as typed deltas", () => {
    const base: TutorialUiSnapshot = {
      activeUtilityByWindowId: {},
      activePanelTabByGroup: {},
      interactionSelection: fixtureTutorialSelection(fixtureInteractionState(tutorialInteractionFixture.playbackBefore).selection),
      expandedTreeIds: [],
      commandPanelOpen: false,
    };
    const recorder = new TutorialRecorder(base, null);
    recorder.recordUiDiff({ ...base, interactionSelection: fixtureTutorialSelection(fixtureInteractionState(tutorialInteractionFixture.afterDelta).selection) });
    recorder.recordUiDiff({ ...base, interactionSelection: {} });
    expect(recorder.build("interaction", "Interaction").tracks.ui.map((keyframe) => keyframe.sample)).toEqual([
      { kind: "delta", changes: [tutorialInteractionFixture.delta] },
      { kind: "delta", changes: [{ kind: "selection", domainId: "mesh", granularity: "face", ids: [] }] },
    ]);
  });

  //#region 🔌️PluginRuntime hot-swap actions
  function fakeLoadedPlugin(pluginId: string, version = "0"): { readonly handle: PluginWasmHandle; readonly manifest: PluginManifest } {
    const manifest: PluginManifest = { pluginId, label: pluginId, version, apps: [], examples: [], commands: [], artifactKinds: [], dependencies: [], contributions: [] };
    const handle = {
      pluginId,
      manifest,
      createApp: async () => 0,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] }, diagnostics: [], requestedEffects: [], events: [] }),
      refreshUi: async () => ({}),
      contextMenu: async () => [],
      dispose: () => {},
    } as unknown as PluginWasmHandle;
    return { handle, manifest };
  }

  it("UPSERT_LOADED_PLUGIN inserts a new pluginId and replaces an existing one in place (order preserved)", () => {
    const state = baseState();
    const note1 = fakeLoadedPlugin("note", "1");
    const withNote = shellReducer(state, { type: "UPSERT_LOADED_PLUGIN", value: note1 });
    expect(withNote.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["note"]);
    const withS = shellReducer(withNote, { type: "UPSERT_LOADED_PLUGIN", value: fakeLoadedPlugin("s") });
    expect(withS.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["note", "s"]);
    const note2 = fakeLoadedPlugin("note", "2");
    const reloaded = shellReducer(withS, { type: "UPSERT_LOADED_PLUGIN", value: note2 });
    expect(reloaded.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["note", "s"]);
    expect(reloaded.pluginRuntime.loadedPlugins[0]!.manifest.version).toBe("2");
    expect(reloaded.layout).toBe(withS.layout);
  });

  it("REMOVE_LOADED_PLUGIN drops only the matching pluginId", () => {
    const withBoth = [fakeLoadedPlugin("note"), fakeLoadedPlugin("s")].reduce((state, entry) => shellReducer(state, { type: "UPSERT_LOADED_PLUGIN", value: entry }), baseState());
    const removed = shellReducer(withBoth, { type: "REMOVE_LOADED_PLUGIN", pluginId: "note" });
    expect(removed.pluginRuntime.loadedPlugins.map((entry) => entry.handle.pluginId)).toEqual(["s"]);
  });

  it("SET_PLUGIN_STATUS tracks per-pluginId status independent of loadedPlugins membership", () => {
    const state = baseState();
    const installing = shellReducer(state, { type: "SET_PLUGIN_STATUS", pluginId: "note", value: "installing" });
    expect(installing.pluginRuntime.pluginStatusById).toEqual({ note: "installing" });
    const loaded = shellReducer(installing, { type: "SET_PLUGIN_STATUS", pluginId: "note", value: "loaded" });
    const failed = shellReducer(loaded, { type: "SET_PLUGIN_STATUS", pluginId: "s", value: "failed" });
    expect(failed.pluginRuntime.pluginStatusById).toEqual({ note: "loaded", s: "failed" });
  });
  //#endregion 🔌️PluginRuntime hot-swap actions
});

// 🕹️wave-2b: `derivePeerInteractionByDomain`/`peerIdsSelecting`/`peerIdsHovering` regroup the typed,
// wave-0 `PresenceInteraction` roster field into an app-agnostic per-domain shape — the replacement for
// today's per-app `presencePeersJson` decoding (see "s workflow flow routing"'s "renders presence peers
// from the scene payload" above, one of the only apps that renders peer selection at all today).
describe("Shell peer interaction (generic, app-agnostic)", () => {
  const peer = (clientId: string, name: string, interaction?: ShellPresencePeer["interaction"]): ShellPresencePeer => ({ clientId, name, interaction });

  it("regroups per-peer PresenceInteraction domains into a per-domain roster, keyed by clientId", () => {
    const roster = derivePeerInteractionByDomain([
      peer("client-a", "Ada", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n1", "n2"], hovered: [] }] }),
      peer("client-b", "Bo", { appId: "lowpoly", domains: [{ domain: "mesh", granularity: "face", selected: [], hovered: ["f7"] }] }),
    ]);
    expect(roster["graph"]).toEqual({ selectedByPeer: { "client-a": ["n1", "n2"] }, hoveredByPeer: {} });
    expect(roster["mesh"]).toEqual({ selectedByPeer: {}, hoveredByPeer: { "client-b": ["f7"] } });
  });

  it("is app-agnostic: two different apps sharing one domain id merge into the same entry", () => {
    const roster = derivePeerInteractionByDomain([
      peer("client-a", "Ada", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n1"], hovered: [] }] }),
      peer("client-b", "Bo", { appId: "dag", domains: [{ domain: "graph", granularity: "node", selected: ["n2"], hovered: [] }] }),
    ]);
    expect(roster["graph"]?.selectedByPeer).toEqual({ "client-a": ["n1"], "client-b": ["n2"] });
  });

  it("peerIdsSelecting/peerIdsHovering find which peers have a given target id", () => {
    const roster = derivePeerInteractionByDomain([
      peer("client-a", "Ada", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n1", "n2"], hovered: ["n3"] }] }),
      peer("client-b", "Bo", { appId: "flow", domains: [{ domain: "graph", granularity: "node", selected: ["n2"], hovered: [] }] }),
    ]);
    expect(peerIdsSelecting(roster, "graph", "n1")).toEqual(["client-a"]);
    expect([...peerIdsSelecting(roster, "graph", "n2")].sort()).toEqual(["client-a", "client-b"]);
    expect(peerIdsSelecting(roster, "graph", "n99")).toEqual([]);
    expect(peerIdsHovering(roster, "graph", "n3")).toEqual(["client-a"]);
  });

  it("is defensive about an absent interaction field (older heartbeat, or wave 2a's wire field not yet landed) and an unknown domain", () => {
    const roster = derivePeerInteractionByDomain([peer("client-a", "Ada", undefined), peer("client-b", "Bo")]);
    expect(roster).toEqual({});
    expect(peerIdsSelecting(roster, "graph", "n1")).toEqual([]);
    expect(peerIdsHovering({}, "unknown-domain", "x")).toEqual([]);
  });
});

// 🐢️ Puzzle 2D performance round 2: the per-interaction full-shell refresh cascade was dominated by
// React reconciling freshly-parsed-but-structurally-identical UiNode/engagement/measure trees on every
// action (select/camera/nodeMove). These helpers let unchanged bodies keep their object identity across
// a `refreshUi` so `InterpretedUiNode`'s `React.memo` (ui-interpreter.tsx) and `modeWindows`'s
// `useMemo` (os-shell.tsx) can bail instead of reconciling the whole shell every time.
describe("ui identity preservation (puzzle 2d perf)", () => {
  it("preserveJsonIdentity reuses the previous reference for structurally-equal values", () => {
    const previous = { type: "text", value: "hello" };
    const next = { type: "text", value: "hello" };
    expect(preserveJsonIdentity(previous, next)).toBe(previous);
  });

  it("preserveJsonIdentity returns the new reference when content actually differs", () => {
    const previous = { type: "text", value: "hello" };
    const next = { type: "text", value: "goodbye" };
    expect(preserveJsonIdentity(previous, next)).toBe(next);
  });

  it("preserveJsonIdentity treats nested arrays/objects structurally, not just top-level fields", () => {
    const previous = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 2 },
      ],
    };
    const next = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 2 },
      ],
    };
    expect(preserveJsonIdentity(previous, next)).toBe(previous);
    const moved = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 3 },
      ],
    };
    expect(preserveJsonIdentity(previous, moved)).toBe(moved);
  });

  it("preserveJsonIdentity treats undefined previous as always-changed", () => {
    const next = { type: "text", value: "hello" };
    expect(preserveJsonIdentity(undefined, next)).toBe(next);
  });

  it("mergeRecordPreservingIdentity reuses the whole previous record when every key is unchanged", () => {
    const prev = { overview: { type: "text", value: "a" }, detail: { type: "text", value: "b" } };
    const merged = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "b" }],
    ]);
    expect(merged).toBe(prev);
  });

  it("mergeRecordPreservingIdentity reuses per-key references, replacing only the changed key", () => {
    const prev = { overview: { type: "text", value: "a" }, detail: { type: "text", value: "b" } };
    const merged = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "changed" }],
    ]);
    expect(merged).not.toBe(prev);
    expect(merged.overview).toBe(prev.overview);
    expect(merged.detail).not.toBe(prev.detail);
  });

  it("mergeRecordPreservingIdentity treats a key being added or removed as a change", () => {
    const prev = { overview: { type: "text", value: "a" } };
    const withNewKey = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "b" }],
    ]);
    expect(withNewKey).not.toBe(prev);
    expect(withNewKey.overview).toBe(prev.overview);
  });
});

// 🐢️ Puzzle 2D performance round 3: the batched, hash-conditional `refresh-ui` protocol that replaces
// ~12 sequential per-section WASM calls with one round trip. `buildUiRefreshRequest` restricts what's
// asked for by scope and attaches known hashes; `applyUiRefreshResponseToCache` writes back only the
// sections the plugin actually says changed.
describe("batched ui refresh request/response (puzzle 2d perf round 3)", () => {
  const windowKinds = [
    { id: "overview", bodyKey: "puzzle2d.play.overview" },
    { id: "detail", bodyKey: "puzzle2d.play.detail" },
  ];
  const panelTabLeaves = [{ kind: { kind: "app" as const, id: "framework.panel.artifact" }, bodyKey: "puzzle2d.play.layers" }];

  it("buildActiveUtilityByWindowId preserves explicit clears for batched refresh", () => {
    expect(buildActiveUtilityByWindowId({ top: "transform", perspective: null, brush: "brush" })).toEqual({ top: "transform", perspective: null, brush: "brush" });
  });

  
  it("buildActiveUtilityByWindowId omits null utilities for batched refresh", () => {
    expect(buildActiveUtilityByWindowId({ top: "transform", perspective: null, brush: "brush" })).toEqual({ top: "transform", brush: "brush" });
  });
it("buildUiRefreshRequest forwards per-window utility map on viewState without a focused-window singular leak", () => {
    const viewState = { activeUtilityByWindowId: { top: "transform", perspective: "brush" }, activeUtilityId: undefined };
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, viewState, new Map());
    expect(request?.viewState.activeUtilityByWindowId).toEqual({ top: "transform", perspective: "brush" });
    expect(request?.viewState.activeUtilityId).toBeUndefined();
  });

  it("buildActiveUtilityByWindowId makes a just-activated transform visible to refresh before the next React render", () => {
    // Regression: setActiveUtility must sync activeUtilityByWindowIdRef before refreshUi; otherwise the
    // program never stamps transform and the gumball stays hidden.
    const map: Record<string, string | null> = { "puzzle3d-main-top": null };
    map["puzzle3d-main-top"] = "transform";
    const activeUtilityByWindowId = buildActiveUtilityByWindowId(map);
    expect(activeUtilityByWindowId).toEqual({ "puzzle3d-main-top": "transform" });
    const request = buildUiRefreshRequest(
      { kind: "full" },
      [
        { id: "puzzle3d-main-top", bodyKey: "puzzle3d.play.composite" },
        { id: "puzzle3d-main-perspective", bodyKey: "puzzle3d.play.composite" },
      ],
      [],
      { activeUtilityByWindowId, activeUtilityId: undefined },
      new Map(),
    );
    expect(request?.viewState.activeUtilityByWindowId).toEqual({ "puzzle3d-main-top": "transform" });
    expect(request?.viewState.activeUtilityId).toBeUndefined();
  });

  it("buildUiRefreshRequest for a full scope requests every window/panel/engagements/measures/labels section (utility bars are now registry-derived, not a plugin section)", () => {
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.windows?.map((w) => w.key)).toEqual(["overview", "detail"]);
    expect(request?.panels?.map((p) => p.key)).toEqual(["framework.panel.artifact"]);
    expect(request?.engagements).toBeDefined();
    expect(request?.measures).toBeDefined();
    expect(request?.labels).toBeDefined();
  });

  // 📌️ The panel half of the same protocol: `buildUiRefreshRequest` names the panel bodies, and
  // `PluginRuntime` mounts each of them under `panelViewContext(request.viewState)`. A panel is not
  // rendered FOR a window, so that projection strips `windowId`/`activeWindowKindId`/`activeUtilityId`
  // and keeps `focusedWindowId` — but only while the roster still carries that pane. A shell publishes
  // the new mode's roster before it refocuses, and the guest's window-config capture faults on a
  // focused pane the roster does not list BEFORE it ever matches the panel's body key: on the wgpu
  // shell, booting straight into generate mode with the edit-mode `procedural-main` still focused left
  // Artifact/Catalogue/Inspection publishing nothing at all
  // (`wgpu-ui.surface-not-published:framework.panel.*`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  // Twin of `ViewModel::for_panel`'s own Rust law.
  const generateRoster = [
    { id: "generation3d-generations", windowKindId: "generation3d-generations" },
    { id: "generation3d-generate-form", windowKindId: "generation3d-generate-form" },
    { id: "generation3d-generate-preview", windowKindId: "generation3d-generate-preview" },
  ];

  it("panelViewContext unbinds a panel from every window-scoped field", () => {
    const projected = panelViewContext({
      activeModeId: "generate",
      windowId: "generation3d-generations",
      activeWindowKindId: "generation3d-generations",
      activeUtilityId: "move",
      focusedWindowId: "generation3d-generations",
      windowInstances: generateRoster,
    });
    expect(projected.windowId).toBeUndefined();
    expect(projected.activeWindowKindId).toBeUndefined();
    expect(projected.activeUtilityId).toBeUndefined();
    expect(projected.activeModeId).toBe("generate");
  });

  it("panelViewContext keeps a focused pane the roster carries, so per-window panel controls still address it", () => {
    const projected = panelViewContext({ activeModeId: "generate", focusedWindowId: "generation3d-generate-form", windowInstances: generateRoster });
    expect(projected.focusedWindowId).toBe("generation3d-generate-form");
  });

  it("panelViewContext drops a focused pane the active mode's roster does not carry, so a mode-switch race cannot unpublish every app panel", () => {
    const projected = panelViewContext({ activeModeId: "generate", focusedWindowId: "procedural-main", windowInstances: generateRoster });
    expect(projected.focusedWindowId).toBeUndefined();
    expect(projected.windowInstances).toEqual(generateRoster);
  });

  it("panelViewContext drops a focused pane when the host carries no roster at all", () => {
    expect(panelViewContext({ focusedWindowId: "procedural-main" }).focusedWindowId).toBeUndefined();
  });

  it("buildUiRefreshRequest for none returns null", () => {
    expect(buildUiRefreshRequest({ kind: "none" }, windowKinds, panelTabLeaves, {}, new Map())).toBeNull();
  });

  it("buildUiRefreshRequest for a partial scope requests only the listed window/panel bodies and flags", () => {
    const scope = { kind: "partial" as const, windowBodies: ["puzzle2d.play.overview"], panelBodies: [], engagements: true };
    const request = buildUiRefreshRequest(scope, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.windows?.map((w) => w.key)).toEqual(["overview"]);
    expect(request?.panels).toEqual([]);
    expect(request?.engagements).toBeDefined();
    expect(request?.measures).toBeUndefined();
    expect(request?.labels).toBeUndefined();
  });

  it("buildUiRefreshRequest returns null for a partial scope that matches nothing in this app", () => {
    const scope = { kind: "partial" as const, windowBodies: ["some-other-app.body"] };
    expect(buildUiRefreshRequest(scope, windowKinds, panelTabLeaves, {}, new Map())).toBeNull();
  });

  it("buildUiRefreshRequest attaches the cached hash for a section that was already fetched once", () => {
    const cache: UiRefreshCache = new Map([["window:overview", { hash: "abc123", value: { type: "text", value: "x" } }]]);
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, cache);
    expect(request?.windows?.find((w) => w.key === "overview")?.hash).toBe("abc123");
    expect(request?.windows?.find((w) => w.key === "detail")?.hash).toBeUndefined();
  });

  // 🪟️ Two window INSTANCES of the same kind (e.g. a split top/perspective pane pair both rendering
  // `puzzle3d.play.main`) must get distinct request entries and distinct cache keys — never collapse
  // onto one shared entry, which is exactly the bug this ticket fixes.
  it("buildUiRefreshRequest gives two instances of the same window kind distinct keys and independent cached hashes", () => {
    const splitInstances = [
      { id: "puzzle3d-main", bodyKey: "puzzle3d.play.main" },
      { id: "puzzle3d-main-2", bodyKey: "puzzle3d.play.main" },
    ];
    const cache: UiRefreshCache = new Map([["window:puzzle3d-main", { hash: "base-hash", value: { type: "text", value: "base" } }]]);
    const request = buildUiRefreshRequest({ kind: "full" }, splitInstances, [], {}, cache);
    expect(request?.windows?.map((w) => w.key)).toEqual(["puzzle3d-main", "puzzle3d-main-2"]);
    expect(request?.windows?.find((w) => w.key === "puzzle3d-main")?.hash).toBe("base-hash");
    expect(request?.windows?.find((w) => w.key === "puzzle3d-main-2")?.hash).toBeUndefined();
  });

  it("applyUiRefreshResponseToCache writes changed sections and ignores hash-only (unchanged) ones", () => {
    const cache: UiRefreshCache = new Map([["window:detail", { hash: "old-hash", value: { type: "text", value: "stale-should-not-be-touched" } }]]);
    applyUiRefreshResponseToCache(cache, {
      windows: [
        { key: "overview", hash: "new-hash", value: { type: "text", value: "fresh" } },
        { key: "detail", hash: "old-hash" }, // unchanged: no `value` in the response
      ],
      engagements: { key: "engagements", hash: "eng-hash", value: { overview: {} } },
    });
    expect(cache.get("window:overview")).toEqual({ hash: "new-hash", value: { type: "text", value: "fresh" } });
    // Unchanged section: cache entry is untouched (still the old hash/value, not overwritten with nothing).
    expect(cache.get("window:detail")).toEqual({ hash: "old-hash", value: { type: "text", value: "stale-should-not-be-touched" } });
    expect(cache.get("engagements")).toEqual({ hash: "eng-hash", value: { overview: {} } });
  });

  it("buildUiRefreshRequest for a full scope also requests the mode-level tools section (keyed by tool id, not a window)", () => {
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.tools).toBeDefined();
  });

  it("buildUiRefreshRequest for a partial scope requests tools only when the scope's `tools` flag is set", () => {
    const withTools = buildUiRefreshRequest({ kind: "partial" as const, tools: true }, windowKinds, panelTabLeaves, {}, new Map());
    expect(withTools?.tools).toBeDefined();
    const withoutTools = buildUiRefreshRequest({ kind: "partial" as const, engagements: true }, windowKinds, panelTabLeaves, {}, new Map());
    expect(withoutTools?.tools).toBeUndefined();
  });

  it("applyUiRefreshResponseToCache caches the tools section same as measures/engagements/labels", () => {
    const cache: UiRefreshCache = new Map();
    applyUiRefreshResponseToCache(cache, { tools: { key: "tools", hash: "tools-hash", value: { fill: [] } } });
    expect(cache.get("tools")).toEqual({ hash: "tools-hash", value: { fill: [] } });
  });

  // 🛍️ The app-static operator/palette catalogue rides its OWN reserved section, never a node-graph
  // scene payload — with the real `brep`/`math` operator sets installed it is ~100 KB against the fixed
  // 32 KiB per-surface admission (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). It is app-static, so it
  // has no `UiDirtyScope` flag: only a full scope asks for it, and the cached hash makes the repeat free.
  it("buildUiRefreshRequest asks for the app catalogue on a full scope only, carrying its cached hash", () => {
    const full = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, new Map());
    expect(full?.catalogue).toBeDefined();
    expect(full?.catalogue?.hash).toBeUndefined();
    const cache: UiRefreshCache = new Map([["catalogue", { hash: "cat-hash", value: { operators: [] } }]]);
    expect(buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, cache)?.catalogue?.hash).toBe("cat-hash");
    const partial = buildUiRefreshRequest({ kind: "partial" as const, engagements: true }, windowKinds, panelTabLeaves, {}, new Map());
    expect(partial?.catalogue).toBeUndefined();
  });

  it("applyUiRefreshResponseToCache caches the app catalogue and leaves it untouched on an unchanged hash", () => {
    const cache: UiRefreshCache = new Map();
    const value = { operators: [{ id: "math.add", extension: "math", name: "Add", abbreviation: "Add", icon: "emoji:+", summary: "Adds", inputs: [], outputs: [] }], sections: [{ id: "math", title: "Math", items: [] }] };
    applyUiRefreshResponseToCache(cache, { catalogue: { key: "catalogue", hash: "cat-hash", value } });
    expect(cache.get("catalogue")).toEqual({ hash: "cat-hash", value });
    applyUiRefreshResponseToCache(cache, { catalogue: { key: "catalogue", hash: "cat-hash" } });
    expect(cache.get("catalogue")).toEqual({ hash: "cat-hash", value });
  });
});

describe("framework plugin runtime", () => {
  // 🔌️ HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS: the WIT ABI flipped from 14
  // per-verb `{json: string}` calls to `manifest`/`createApp`/`destroyApp` plus `protocol_channel::
  // AppCommand`/`AppFrame` bytes carried over the turn ABI, instead of the old flat `semio_plugin_*`
  // wasm-bindgen JSON exports, which no longer exist anywhere in the ABI
  // (`loadPluginModuleUncached`'s doc comment).
  // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H1-react, absorbing A4-channel's lease against this
  // file): channel v12 retired `AppCommand::RefreshUi`/`SectionProbe` and `AppFrame::UiSection` —
  // window-body refresh is no longer a request/response round trip at all, it is a
  // `Event::SurfaceVisible` submission read back through `TurnResult.uiPatches` via the
  // `ActivationRegistry`/`ShardClient` pair `loadPluginModule` owns (`🧱️elements/PluginRuntime/🟦️.tsx`'s
  // `🔖️ActorAdapter` region). `adaptPluginHandle` alone (what this test constructs, via a bare
  // fake with no actor and no ShardClient) genuinely has no wire path left to ask for a
  // section body over, so its own `refreshUi` is an honest empty result — asserted here rather than
  // deleted, since "no wire path here anymore" is itself real, worth-pinning behavior.
  //
  // 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (`exchange-removal`): the raw
  // `KernelPluginWasmHandle`'s old synchronous `exchange(instanceId, frames) -> Promise<frames>`
  // per-call RPC is gone (`📌️important.md`'s "Replace, never wrap" list) — split into fire-and-forget
  // `enqueue(instanceId, events): void` plus the handle-wide `outcomes: AsyncIterable<TurnOutcome>`
  // broadcast `AppChannelClient` correlates FIFO against (`🎠️kernel/🟦️.ts`'s
  // `PluginWasmHandle` header doc). This helper re-creates `exchange`'s old request/reply shape on
  // top of the two new primitives, purely for these fakes' own convenience — production code never
  // has a synchronous responder like this to call.
  function exchangeStyleChannel(respond: (instanceId: number, frames: Uint8Array[]) => Uint8Array[] | Promise<Uint8Array[]>): {
    readonly enqueue: (instanceId: number, events: readonly Uint8Array[]) => void;
    readonly outcomes: AsyncIterable<TurnOutcome>;
  } {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    return {
      enqueue: (instanceId, events) => {
        void (async () => {
          try {
            const frames = await respond(instanceId, [...events]);
            broadcast.push({ instanceId, frames });
          } catch (error) {
            broadcast.push({ instanceId, error });
          }
        })();
      },
      outcomes: broadcast.stream,
    };
  }

  it("adaptPluginHandle's own refreshUi is an honest empty result — window-body refresh now lives in loadPluginModule's ActivationRegistry/ShardClient turn loop, which a bare no-command handle has no access to", async () => {
    const { encodePackValue } = await import("@semio-tech/framework-os");
    const fakeHandle = {
      manifest: { pluginId: "mock-refresh", label: "Mock Refresh", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 7,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel(() => {
        throw new Error("adaptPluginHandle.refreshUi must not call enqueue() — there is no AppCommand for it anymore");
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-refresh", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");
    await expect(handle.refreshUi(instanceId, { viewState: {}, windows: [{ key: "overview", bodyKey: "overview" }] })).resolves.toEqual({});
  });

  it("fetchDescriptorManifest refuses a missing descriptor and surfaces a published one", async () => {
    const originalFetch = globalThis.fetch;
    try {
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 404 }));
      await expect(fetchDescriptorManifest("mock", "/🔌️plugin-modules/mock/index.js")).rejects.toThrow("plugin.descriptor-unavailable");

      globalThis.fetch = stubFetch(async () => new Response(JSON.stringify({ manifest: { pluginId: "mock", label: "Mock", version: "1.0.0", apps: [{ id: "main" }] } }), { headers: { "content-type": "application/json" } }));
      const real = await fetchDescriptorManifest("mock", "/🔌️plugin-modules/mock/index.js");
      expect(real).toEqual({ pluginId: "mock", label: "Mock", version: "1.0.0", apps: [{ id: "main" }] });
    } finally {
      globalThis.fetch = originalFetch;
    }
  });

  it("resolves the descriptor before initializing the shard runtime", async () => {
    const order: string[] = [];
    const resolved = await resolveDescriptorBeforeRuntime(
      async () => {
        order.push("descriptor-start");
        await Promise.resolve();
        order.push("descriptor-ready");
        return "manifest";
      },
      () => {
        order.push("runtime");
        return "shards";
      },
    );
    expect(order).toEqual(["descriptor-start", "descriptor-ready", "runtime"]);
    expect(resolved).toEqual({ manifest: "manifest", runtime: "shards" });
  });

  // 🧬️ H1-react — design-runtime.md §1 `SceneStore` / packet brief item 2: the retained-tree
  // reconciliation `PluginRuntime`'s `loadPluginModule` applies every `TurnResult.uiPatches` entry
  // through. Exercised directly (not through a fake wasm turn) since it is a pure function — real
  // coverage of "apply a `UiPatch` to a retained tree, honour `baseRevision`" that does not depend on
  // the unverified jco wasm boundary this file's `🔖️ActorAdapter` doc flags.
  describe("applyUiPatchToRetained", () => {
    const leaf = (id: number, value: string): UiNodeRecord => ({
      id,
      key: `leaf-${id}`,
      component: { type: "text", value, emphasize: null, dataAttributes: null },
      layout: { kind: "leaf", width: "hug", height: "hug" },
      style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
      activity: "idle",
      disabled: false,
      transition: null,
      accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
      bindings: [],
      menu: null,
      children: [],
    });

    it("applies the semantic upsert and set-root operations emitted by the actor WIT boundary", async () => {
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const ops = decodeWirePatchOps([
        {
          tag: "upsert",
          val: {
            node: Array.from(encodePackValue({ id: 0, key: "leaf-0", component: leaf(0, "a").component, layout: leaf(0, "a").layout, style: {}, activity: "idle", accessibility: {} })),
          },
        },
        { tag: "set-root", val: 0n },
      ]);
      const result = applyUiPatchToRetained(null, { surface: "s", revision: 1n, baseRevision: 0n, ops });
      expect(result.desynced).toBe(false);
      expect(result.surface?.revision).toBe(1);
      expect(result.surface?.nodes.get(0)?.component).toEqual({ type: "text", value: "a", emphasize: null, dataAttributes: null });
      expect(result.surface?.nodes.get(0)?.children).toEqual([]);
    });

    it("projects lossless pack integer carriers onto exact node ids, children and revisions", async () => {
      const { encodePackValue, packUInt } = await import("@semio-tech/framework-os");
      const ops = decodeWirePatchOps([
        {
          tag: "upsert",
          val: {
            node: Array.from(encodePackValue({ id: packUInt(1n), key: "leaf-1", component: leaf(1, "a").component, layout: leaf(1, "a").layout, style: {}, activity: "idle", accessibility: {}, children: [packUInt(3n)] })),
          },
        },
        { tag: "upsert", val: { node: Array.from(encodePackValue({ id: packUInt(3n), key: "leaf-3", component: leaf(3, "b").component, layout: leaf(3, "b").layout, style: {}, activity: "idle", accessibility: {} })) } },
        { tag: "set-children", val: { node: 1n, children: [3n] } },
        { tag: "set-root", val: 1n },
      ]);
      expect(ops.map((op) => op.type)).toEqual(["upsert", "upsert", "setChildren", "setRoot"]);
      expect(ops[0]).toMatchObject({ type: "upsert", id: 1, children: [3] });
      expect(ops[1]).toMatchObject({ type: "upsert", id: 3 });
      expect(ops[3]).toEqual({ type: "setRoot", id: 1 });
      const result = applyUiPatchToRetained(null, { surface: "s", revision: 1n, baseRevision: 0n, ops });
      expect(result.desynced).toBe(false);
      expect(result.surface?.nodes.get(1)?.children).toEqual([3]);
      expect(result.surface?.nodes.get(3)?.component).toEqual({ type: "text", value: "b", emphasize: null, dataAttributes: null });
      expect(() => decodeWirePatchOps([{ tag: "upsert", val: { node: Array.from(encodePackValue({ id: packUInt(2n ** 60n), key: "huge" })) } }])).toThrow(/upsert\.node\.id/u);
    });

    it("applies incremental semantic field updates with a matching base revision", () => {
      const first = applyUiPatchToRetained(null, {
        surface: "s",
        revision: 1,
        baseRevision: 0,
        ops: [
          { type: "upsert", ...leaf(0, "a") },
          { type: "setRoot", id: 0 },
        ],
      });
      const result = applyUiPatchToRetained(first.surface, { surface: "s", revision: 2, baseRevision: 1, ops: [{ type: "setComponent", id: 0, component: { type: "text", value: "b", emphasize: null, dataAttributes: null } }] });
      expect(result.desynced).toBe(false);
      expect(result.surface?.revision).toBe(2);
      expect(result.surface?.nodes.get(0)?.component).toEqual({ type: "text", value: "b", emphasize: null, dataAttributes: null });
    });

    it("keeps the previous body when a semantic patch has a stale base revision", () => {
      const { surface: previous } = applyUiPatchToRetained(null, {
        surface: "s",
        revision: 1,
        baseRevision: 0,
        ops: [
          { type: "upsert", ...leaf(0, "a") },
          { type: "setRoot", id: 0 },
        ],
      });
      const result = applyUiPatchToRetained(previous, { surface: "s", revision: 2, baseRevision: 0, ops: [{ type: "setComponent", id: 0, component: { type: "text", value: "b", emphasize: null, dataAttributes: null } }] });
      expect(result.desynced).toBe(true);
      expect(result.surface).toBe(previous);
    });

    it("keeps the previous body when a patch violates the retained document graph", () => {
      const { surface: previous } = applyUiPatchToRetained(null, {
        surface: "s",
        revision: 1,
        baseRevision: 0,
        ops: [
          { type: "upsert", ...leaf(0, "a") },
          { type: "setRoot", id: 0 },
        ],
      });
      const result = applyUiPatchToRetained(previous, { surface: "s", revision: 2, baseRevision: 1, ops: [{ type: "setChildren", id: 0, children: [99] }] });
      expect(result.desynced).toBe(true);
      expect(result.surface).toBe(previous);
    });
  });

  it("parses a typed InvocationResponse, including requestedEffects, from a plugin handle-action response", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework");
    const response = parseInvocationResponse(
      JSON.stringify({
        output: null,
        mutations: [{ diff: { payload: { schemaId: "draw.operation", document: { id: "forest" } } } }],
        inverseGroup: { invocationId: "setActiveExample:1:0", mutations: [], inverseMutations: [] },
        requestedEffects: [{ navigate: { uri: "/spaces/forest" } }],
      }),
    );
    expect(response.mutations).toHaveLength(1);
    expect(response.requestedEffects).toEqual([{ navigate: { uri: "/spaces/forest" } }]);
  });

  it("falls back to an empty InvocationResponse for malformed handle-action JSON", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework");
    expect(parseInvocationResponse("not json")).toEqual({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } });
    expect(parseInvocationResponse(JSON.stringify({ output: null }))).toEqual({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } });
  });

  // 🧬️ H1-react — `withSerializedPluginWasmHandle` (which queued concurrent per-call requests
  // transparently against the old synchronous wasm handle) is deleted alongside `PluginWorkerClient`
  // (`🎠️kernel/🟦️.ts`'s own doc comment names it). The reason it existed still applies:
  // `🟨️shard-worker.js` REJECTS (does not queue) a second in-flight `turn` for the same actor
  // (`inFlightTurnActors` guard, `🟦️.ts`). `serializePerActor`
  // (`PluginRuntime/🟦️.tsx`'s `🔖️ActorAdapter` region) is `loadPluginModule`'s real
  // replacement — every `submitTurn` call for one actor funnels through it — exercised directly here
  // since it is a plain, generic per-key promise queue.
  it("serializePerActor queues concurrent turns for the same actor one at a time, never overlapping", async () => {
    let inFlight = 0;
    let maxInFlight = 0;
    const runOne = () =>
      serializePerActor("actor-1", async () => {
        inFlight += 1;
        maxInFlight = Math.max(maxInFlight, inFlight);
        await new Promise((resolve) => setTimeout(resolve, 5));
        inFlight -= 1;
        return "done";
      });
    const results = await Promise.all([runOne(), runOne(), runOne()]);
    expect(maxInFlight).toBe(1);
    expect(results).toEqual(["done", "done", "done"]);
  });

  it("serializePerActor keys independently per actor — different actors run concurrently, not queued behind each other", async () => {
    let concurrentAcrossActors = 0;
    let maxConcurrentAcrossActors = 0;
    const runOn = (actorId: string) =>
      serializePerActor(actorId, async () => {
        concurrentAcrossActors += 1;
        maxConcurrentAcrossActors = Math.max(maxConcurrentAcrossActors, concurrentAcrossActors);
        await new Promise((resolve) => setTimeout(resolve, 5));
        concurrentAcrossActors -= 1;
      });
    await Promise.all([runOn("actor-a"), runOn("actor-b")]);
    expect(maxConcurrentAcrossActors).toBe(2);
  });

  it("serializePerActor keeps queuing subsequent turns after an earlier one rejects", async () => {
    const order: string[] = [];
    const failing = serializePerActor("actor-2", async () => {
      order.push("first");
      throw new Error("turn faulted");
    });
    const succeeding = serializePerActor("actor-2", async () => {
      order.push("second");
      return "ok";
    });
    await expect(failing).rejects.toThrow("turn faulted");
    await expect(succeeding).resolves.toBe("ok");
    expect(order).toEqual(["first", "second"]);
  });

  it("serializes complete multi-turn command ingress sequences for one actor", async () => {
    const order: string[] = [];
    let releaseFirst!: () => void;
    let markFirstStarted!: () => void;
    const firstStarted = new Promise<void>((resolve) => {
      markFirstStarted = resolve;
    });
    const firstGate = new Promise<void>((resolve) => {
      releaseFirst = resolve;
    });
    const first = serializeCommandIngressForActor("actor-ingress", async () => {
      order.push("first-page");
      markFirstStarted();
      await firstGate;
      order.push("first-terminal");
    });
    await firstStarted;
    const second = serializeCommandIngressForActor("actor-ingress", async () => {
      order.push("second-page");
      await Promise.resolve();
      order.push("second-terminal");
    });
    await Promise.resolve();
    expect(order).toEqual(["first-page"]);
    releaseFirst();
    await Promise.all([first, second]);
    expect(order).toEqual(["first-page", "first-terminal", "second-page", "second-terminal"]);
  });

  // 🔗️ INPUT-CAUSALITY-LEDGER §2 B / §6 phase 4, law L2: `serializeCommandIngressForActor`'s `order`
  // is the ledger's causal key (`causedBy ?? inputSeq`); the mailbox's `## causal order` rule inserts
  // an ordered call before the earliest queued call with a strictly larger `order`, so a follow-up of
  // input N (order N) overtakes the already-queued input N+1 while the in-flight call is never touched.
  it("serializeCommandIngressForActor dequeues queued calls by ascending order — a later call with a smaller order overtakes a larger one behind the held call", async () => {
    const ran: string[] = [];
    let releaseHeld!: () => void;
    let markHeldStarted!: () => void;
    const heldStarted = new Promise<void>((resolve) => {
      markHeldStarted = resolve;
    });
    const heldGate = new Promise<void>((resolve) => {
      releaseHeld = resolve;
    });
    const held = serializeCommandIngressForActor("actor-causal-order", async () => {
      ran.push("held");
      markHeldStarted();
      await heldGate;
    });
    await heldStarted;
    const callA = serializeCommandIngressForActor(
      "actor-causal-order",
      async () => {
        ran.push("A");
      },
      "Interactive",
      5,
    );
    const callB = serializeCommandIngressForActor(
      "actor-causal-order",
      async () => {
        ran.push("B");
      },
      "Interactive",
      7,
    );
    const callC = serializeCommandIngressForActor(
      "actor-causal-order",
      async () => {
        ran.push("C");
      },
      "Interactive",
      6,
    );
    await Promise.resolve();
    expect(ran).toEqual(["held"]);
    releaseHeld();
    await Promise.all([held, callA, callB, callC]);
    expect(ran).toEqual(["held", "A", "C", "B"]);
  });

  it("serializeCommandIngressForActor keeps plain FIFO for calls without order", async () => {
    const ran: string[] = [];
    let releaseHeld!: () => void;
    let markHeldStarted!: () => void;
    const heldStarted = new Promise<void>((resolve) => {
      markHeldStarted = resolve;
    });
    const heldGate = new Promise<void>((resolve) => {
      releaseHeld = resolve;
    });
    const held = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("held");
      markHeldStarted();
      await heldGate;
    });
    await heldStarted;
    const first = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("first");
    });
    const second = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("second");
    });
    const third = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("third");
    });
    await Promise.resolve();
    expect(ran).toEqual(["held"]);
    releaseHeld();
    await Promise.all([held, first, second, third]);
    expect(ran).toEqual(["held", "first", "second", "third"]);
  });

  // 🧬️ H1-react — `AppFrame::Effects`/`Events` no longer exist (channel v12, A4-channel). Effects
  // now travel as real `kernel::Effect` values directly on `TurnResult.effects`
  // (`⚛️reactor/🦀️.rs`'s `poll`), demuxed by `loadPluginModule`'s turn loop into
  // `pendingTurnEffects`/drained by `performInvocation` — a mechanism a bare command-only fake (no
  // ShardClient turn ever runs) has nothing to populate, so `requestedEffects` is honestly `[]` here.
  // `output`/`uiScope`/`historyPatch` still arrive on the SAME `AppFrame::Invocation` frame, unchanged
  // by the flip — real wire coverage, kept.
  it("adaptPluginHandle.handleAction round-trips an action's output/uiScope/historyPatch from AppFrame::Invocation; requestedEffects is honestly empty for a bare command-only handle", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue, decodePackValue } = await import("@semio-tech/framework-os");
    const fakeHandle = {
      manifest: { pluginId: "mock-action", label: "Mock Action", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 3,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        if (!command || typeof command !== "object" || !("Command" in command)) throw new Error("expected a Command");
        const invocation = decodePackValue(new Uint8Array(command.Command.command));
        return [
          encodeAppFrame({
            Invocation: {
              in_reply_to: command.Command.seq,
              output: Array.from(encodePackValue({ echo: invocation })),
              diagnostics: Array.from(encodePackValue([])),
              ui_scope: Array.from(encodePackValue({ kind: "partial", windowBodies: ["graph"], utilities: false })),
              history_patch: Array.from(encodePackValue({ cursor: 1, upserts: [] })),
              messages: [],
              mutations: Array.from(encodePackValue([])),
              inverse_group: Array.from(encodePackValue({ invocationId: "", mutations: [], inverseMutations: [] })),
            },
          }),
        ];
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-action", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");
    const invocation = { address: { pluginId: "mock-action", appId: "main", modeId: "edit", windowKindId: "main", windowInstanceId: "main", actionId: "addShot" }, arguments: { format: "png" } };
    const response = await handle.handleAction(instanceId, JSON.stringify(invocation), { locale: "en", terminology: "native" });
    expect(response.output).toEqual({ echo: invocation });
    expect(response.requestedEffects).toEqual([]);
    expect(response.uiScope).toEqual({ kind: "partial", windowBodies: ["graph"], utilities: false });
    expect(response.historyPatch).toEqual({ cursor: 1, upserts: [] });
  });

  it("adaptPluginHandle exposes setMergePolicy/resolveConflict/readConflicts and sends the real AppCommand wire frames — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane K2: the merge-policy Settings control and Conflicts panel Accept/Discard used to call `plugin.handle.setMergePolicy?.(…)` against a handle that never had the method, so the optional call silently no-opped — this asserts the method genuinely exists AND that calling it round-trips through the real `AppCommand`/`AppFrame` codecs, not an internal spy", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue } = await import("@semio-tech/framework-os");
    const sentCommands: unknown[] = [];
    const fakeHandle = {
      manifest: { pluginId: "mock-merge", label: "Mock Merge", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 9,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        sentCommands.push(command);
        if (command && typeof command === "object" && "setMergePolicy" in command) return [encodeAppFrame({ Done: { in_reply_to: command.setMergePolicy.seq } })];
        if (command && typeof command === "object" && "resolveConflict" in command) {
          const seq = command.resolveConflict.seq;
          return [
            encodeAppFrame({ MergeReport: { in_reply_to: seq, report: Array.from(encodePackValue({ policy: "Normal", accepted: true, insertionIndex: 0, replayed: [], worst: null, conflict: null })) } }),
            encodeAppFrame({ Conflicts: { in_reply_to: seq, conflicts: Array.from(encodePackValue([])) } }),
          ];
        }
        if (command && typeof command === "object" && "readConflicts" in command) {
          return [encodeAppFrame({ Conflicts: { in_reply_to: command.readConflicts.seq, conflicts: Array.from(encodePackValue([])) } })];
        }
        throw new Error(`unexpected command ${JSON.stringify(command)}`);
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-merge", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    expect(typeof handle.setMergePolicy).toBe("function");
    expect(typeof handle.resolveConflict).toBe("function");
    expect(typeof handle.readConflicts).toBe("function");
    const instanceId = await handle.createApp("main");

    // ⚖️ Same call the Settings merge-policy `Select`'s `dispatchSetMergePolicy` makes (`ShellHost/🟦️.tsx`).
    await handle.setMergePolicy(instanceId, "Vigilant");
    expect(sentCommands[0]).toEqual({ setMergePolicy: { seq: 1, policy: 2 } });

    // ⚔️ Same call the Conflicts panel's Accept button makes (`ChromePanels`'s `onResolve` → `dispatchResolveConflict`).
    const resolved = await handle.resolveConflict(instanceId, "conflict-abc", "accept");
    expect(sentCommands[1]).toEqual({ resolveConflict: { seq: 2, conflict_id: "conflict-abc", resolution: 0 } });
    expect(resolved.mergeReport?.accepted).toBe(true);
    expect(resolved.conflicts).toEqual([]);

    await handle.readConflicts(instanceId);
    expect(sentCommands[2]).toEqual({ readConflicts: { seq: 3 } });
  });

  it("adaptPluginHandle.applyMutations decodes an unsolicited MergeReport/Conflicts reply and it reaches ShellState — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane L1 gap 1: a peer's ApplyEnvelopes ingest batches MergeReport/Conflicts frames alongside it (contract freeze §C6/§C9 'pushed unsolicited after every ingest'), but `applyMutations` used to only look for an Error frame and silently drop everything else — this asserts the guest's real roster survives the decode AND that dispatching it through `shellReducer`'s SET_CONFLICTS (what ShellHost's `applyRemoteMerge` does) lands a remote-origin quarantined conflict in both `selectOpenConflicts` and `selectQuarantinedConflicts`, exactly the panel/badge lane K2 wired", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue, encodeMutationEnvelopesPack } = await import("@semio-tech/framework-os");
    const remoteConflict = {
      id: "conflict-remote-1",
      kind: { kind: "quarantined", envelopes: [] },
      status: "open",
      messages: [{ level: "error", code: "mutation.target-missing", message: "peer deleted the renamed node" }],
      actors: ["peer-actor"],
      timestamp: { actor: 7, physical_ms: 1000, logical: 1 },
    };
    const fakeHandle = {
      manifest: { pluginId: "mock-remote-merge", label: "Mock Remote Merge", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 11,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        if (!command || typeof command !== "object" || !("ApplyEnvelopes" in command)) throw new Error(`unexpected command ${JSON.stringify(command)}`);
        const seq = command.ApplyEnvelopes.seq;
        return [
          encodeAppFrame({ Done: { in_reply_to: seq } }),
          // ⚖️ Unsolicited: the command was ApplyEnvelopes, not ReadConflicts/ResolveConflict — this
          // is exactly the "pushed unsolicited after every ingest" reply shape contract freeze §C8
          // describes, alongside whatever `DocumentChanged`/effect frames a real ingest would also carry.
          encodeAppFrame({ MergeReport: { in_reply_to: null, report: Array.from(encodePackValue({ policy: "Normal", accepted: false, insertionIndex: 0, replayed: [], worst: "error", conflict: remoteConflict.id })) } }),
          encodeAppFrame({ Conflicts: { in_reply_to: null, conflicts: Array.from(encodePackValue([remoteConflict])) } }),
        ];
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-remote-merge", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");

    // 🐛 Pre-fix, `applyMutations` returned `Promise<void>` and this whole roster was thrown away.
    if (!handle.applyMutations) throw new Error("Mutation fixture requires its owned mutation handler");
    const result = await handle.applyMutations(instanceId, encodeMutationEnvelopesPack([]));
    expect(result.mergeReport?.accepted).toBe(false);
    expect(result.mergeReport?.worst).toBe("error");
    expect(result.conflicts).toEqual([remoteConflict]);

    // ⚖️ The other half of the gap: ShellHost's `applyRemoteMerge` (fed by `applyRemoteMergeRef` from
    // the `remoteMutations` worker-event branch) just dispatches `SET_CONFLICTS` with this roster —
    // reproduced here at the reducer level so the assertion doesn't need a mounted `ShellHost`.
    const state = shellReducer(initialShellState({ plugins: [], storage: createMemoryStoragePort() }), { type: "SET_CONFLICTS", value: result.conflicts ?? [] });
    expect(selectOpenConflicts(state)).toEqual([remoteConflict]);
    expect(selectQuarantinedConflicts(state)).toEqual([remoteConflict]);
  });

  // 🪦️ H1-react — `isPluginInstanceBusyError`/`pluginErrorText` (detecting a jco "plugin instance
  // busy" error after a concurrent call raced past `withSerializedPluginWasmHandle`) are deleted
  // alongside it and `INSTANCE_GUARD`/`clear-instance-guard` (packet H2's/`📌️important.md`'s "must
  // not exist" list). Not a dropped-coverage gap: `serializePerActor` (tested above) makes a "busy"
  // race structurally impossible at this layer — every turn for one actor is queued, never
  // concurrent — so there is no busy-error shape left to detect. Prevention replaced detection.
});

describe("framework renderer types", () => {
  it("matches native action-semantics defaults without claiming migrated interactivity", () => {
    
    
    const actual = (["mutation", "view", "interaction", "history", "clipboard", "shell"] as const).map((kind) => ({ kind, semantics: actionSemanticsForKind(kind) }));
    expect(actual).toEqual(actionSemanticsFixture);
    
    
  });

  it("keeps window tabs concise while retaining the app fallback", () => {
    const app = {
      id: "puzzle3d-play",
      label: "Puzzle 3D",
      breadcrumb: ["semio", "puzzle", "3d"],
      terminologyBreadcrumbs: { reuse: ["Entwerfen mit Bestand", "Aggregator"] },
      controllerId: "puzzle3d-play",
      modes: [],
      windowKinds: [],
      panelTabs: [],
      keybindings: [],
    };
    expect(appBreadcrumb(app.breadcrumb)).toBe("semio · puzzle · 3d");
    expect(appWindowLabel(app, "native", "Flow")).toBe("Flow");
    expect(appWindowLabel(app, "native", "Preview")).toBe("Preview");
    expect(appWindowLabel(app, "native", "")).toBe("Puzzle 3D");
    expect(appWindowLabel(app, "reuse", "")).toBe("Aggregator");
    expect(resolveAppBreadcrumb(app, "native")).toEqual(["semio", "puzzle", "3d"]);
    expect(resolveAppBreadcrumb(app, "reuse")).toEqual(["Entwerfen mit Bestand", "Aggregator"]);
    expect(appBreadcrumb(resolveAppBreadcrumb(app, "reuse"))).toBe("Entwerfen mit Bestand · Aggregator");
  });

  it("survives an app whose manifest declares no breadcrumb at all", () => {
    // 🛡️ `AppDefinition.breadcrumb` is OPTIONAL, so an app may legitimately ship without one — and
    // `appBreadcrumb` runs inside `FrameworkOsShellInner`'s RENDER. Before this guard, one such app
    // threw `Cannot read properties of undefined (reading 'join')` and took the whole shell down
    // with it; in a multi-pane host (the demonstrator) that killed all six panes at once and left
    // the page blank. A nameless title is the correct degradation, never a dead host.
    const app: Parameters<typeof resolveAppBreadcrumb>[0] = { breadcrumb: [], terminologyBreadcrumbs: {} };
    expect(resolveAppBreadcrumb(app, "native")).toEqual([]);
    expect(appBreadcrumb(resolveAppBreadcrumb(app, "native"))).toBe("");
    expect(appBreadcrumb(undefined)).toBe("");
  });

  it("flattens a recursive panelTabs tree to its leaves, depth-first", () => {
    const tabs = [
      { id: "framework.panel.artifact", label: "Artifact", group: "workbench", bodyKey: "doc" },
      {
        id: "framework.panel.catalogue",
        label: "Catalogue",
        group: "workbench",
        children: [
          { id: "framework.panel.catalogue.words", label: "Words", group: "workbench", bodyKey: "words" },
          { id: "framework.panel.catalogue.headings", label: "Headings", group: "workbench", bodyKey: "headings" },
        ],
      },
    ];
    const leaves = flattenPanelTabLeaves(tabs);
    expect(leaves.map((tab) => tab.id)).toEqual(["framework.panel.artifact", "framework.panel.catalogue.words", "framework.panel.catalogue.headings"]);
    expect(leaves.every((tab) => Boolean(tab.bodyKey))).toBe(true);
  });

  it("accepts component scene nodes", () => {
    const node = {
      type: "componentScene",
      surfaceId: "draw.play.composite",
      controllerId: "draw-play",
      componentKind: "canvas-2d",
      canvas2d: {
        cameraX: 0,
        cameraY: 0,
        zoom: 1,
        layersJson: "[]",
      },
    };
    expect(node.componentKind).toBe("canvas-2d");
  });

  it("accepts graph-timeline component scene nodes", () => {
    const node = {
      type: "componentScene",
      surfaceId: "vcs.play.history",
      controllerId: "vcs-play",
      componentKind: "graph-timeline",
      graphTimeline: {
        columnsJson: "[]",
      },
    };
    expect(node.componentKind).toBe("graph-timeline");
  });
});

describe("owned declarative controls", () => {
  it("renders and dispatches a panel input through the Interpreter export", () => {
    const onAction = vi.fn();
    const view = render(renderUiControl({ type: "input", id: "name", inputKind: "text", value: "before", onChange: { controllerId: "test", action: "rename", args: { retained: true } } }, onAction, "panel.name"));
    const input = view.container.querySelector<HTMLInputElement>('input[data-ui-path="panel.name"]')!;
    expect(input.getAttribute("data-ui-path")).toBe("panel.name");
    fireEvent.change(input, { target: { value: "after" } });
    expect(onAction).toHaveBeenCalledWith({ controllerId: "test", action: "rename", args: { retained: true, value: "after" } });
    view.unmount();
  });

  it("dispatches a declarative select through the owned listbox", () => {
    const onAction = vi.fn();
    const { getByRole } = render(
      renderUiControl(
        {
          type: "select",
          id: "mode",
          value: "alpha",
          items: [
            { value: "alpha", label: "Alpha" },
            { value: "beta", label: "Beta" },
          ],
          onChange: { controllerId: "test", action: "mode", args: { retained: true } },
        },
        onAction,
        "panel.mode",
      ),
    );
    const trigger = getByRole("combobox");
    expect(trigger.textContent).toContain("Alpha");
    fireEvent.click(trigger);
    const beta = document.querySelector<HTMLElement>('[role="option"][data-value="beta"]')!;
    expect(beta.textContent).toContain("Beta");
    fireEvent.click(beta);
    expect(onAction).toHaveBeenCalledWith({ controllerId: "test", action: "mode", args: { retained: true, value: "beta" } });
  });

  it("keeps a declarative slider's numeric readout, external unit sibling, and spoken value distinct", () => {
    const law = sliderPresentationFixture.unit;
    for (const placement of law.placements) {
      const control = renderUiControl(
        {
          type: "slider",
          id: `distance.${placement.id}`,
          value: law.value,
          min: 0,
          max: 10,
          step: 0.1,
          unit: law.unit,
          onChange: { controllerId: "test", action: "distance" },
        },
        () => {},
        `panel.${placement.id}`,
      );
      const view = render(createElement("div", placement.kind === "treeControl" ? { role: "treeitem", dir: placement.inline } : { dir: placement.inline }, control));
      const thumb = view.getByRole("slider");
      const slider = view.container.querySelector<HTMLElement>('[data-slot="slider"]')!;
      const wrapper = slider.closest<HTMLElement>(".gap-single")!;
      const external = wrapper.lastElementChild as HTMLElement;
      expect(wrapper.className).toContain("gap-single");
      expect(wrapper.children).toHaveLength(2);
      expect(wrapper.firstElementChild?.contains(slider)).toBe(true);
      expect(thumb.getAttribute("aria-valuetext")).toBe(law.accessibleValueText);
      expect(view.container.querySelector('[data-slot="slider-value"]')?.textContent).toBe(law.internalReadout);
      expect(external.textContent).toBe(law.externalReadout);
      expect(external.className).toContain("text-muted-foreground");
      expect(external.className).toContain("shrink-0");
      view.unmount();
    }
  });
});

describe("framework external slots", () => {
  it("preserves explicitly owned host extensions before contributor resolution at any depth", async () => {
    const { resolveExternalSlots } = await import("@semio-tech/framework");
    const { default: fixture } = await import("../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧩️host-slots/🔣️.json");
    
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv();
    
    const ownedIds = new Set(fixture.hostExtensionIds);
    for (const sample of fixture.cases) {
      expect(ownedIds.has(sample.id)).toBe(sample.retained);
      const node: any = {
        key: sample.id, component: { type: "extension", extension: sample.id, props: sample.props }, layout: CONTRACT_LEAF_LAYOUT,
        style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
        accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [],
      };
      const errors: string[] = [];
      const context = { plugins: new Map(), contributorInstances: new Map(), contributorPasses: new Map(), viewState: { locale: "de", terminology: "native" } as any, ownerId: "app:document:window", hostExtensions: new Set(fixture.hostExtensionIds), onError: (id: string) => errors.push(id) };
      const direct = await resolveExternalSlots(node, context);
      const nested = await resolveExternalSlots({ ...node, key: "outer", component: { type: "text", value: "parent", emphasize: null, dataAttributes: null }, children: [node] }, context);
      for (const result of [direct, nested.children[0]]) {
        if (sample.retained) expect(result).toBe(node);
        else expect(result.component.type).toBe("text");
      }
      expect(errors).toEqual(sample.retained ? [] : [sample.id, sample.id]);
      expect(context.contributorInstances.size).toBe(0);
    }
  });

  it("admits isolated extension render inputs against the neutral schema", async () => {
    const { default: fixture } = await import("../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🧩️extension-input/🔣️.json");
    const { default: schema } = await import("../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).compile(schema);
    for (const row of fixture.cases) {
      expect(validate(row.context), JSON.stringify(validate.errors)).toBe(true);
      expect(parseResolvedPluginViewState(row.context)).toEqual(row.context);
    }
    for (const length of [fixture.capacityChars, fixture.capacityChars + 1]) {
      const context = { locale: "en", terminology: "native", extensionInputJson: "x".repeat(length) };
      expect(validate(context)).toBe(length === fixture.capacityChars);
      if (length === fixture.capacityChars) expect(parseResolvedPluginViewState(context)).toEqual(context);
      else expect(() => parseResolvedPluginViewState(context)).toThrow();
    }
  });

  it("renders canonical contributor bodies with isolated instances and current inputs", async () => {
    const { resolveExternalSlots } = await import("@semio-tech/framework");
    const { default: law } = await import("../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧩️external-slots/🔣️.json");
    const created: string[] = [];
    const destroyed: number[] = [];
    const requests: { instanceId: number; request: any }[] = [];
    const node = (key: string, component: any): any => ({
      key, component, layout: CONTRACT_LEAF_LAYOUT,
      style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
      activity: "idle", disabled: false,
      accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
      bindings: [], menu: null, children: [],
    });
    const handle = {
      manifest: { apps: [{ id: law.appId, controllerId: law.controllerId, defaultModeId: law.modeId, modes: [{ id: law.modeId }], windowKinds: [{ id: law.windowKindId, bodyKey: law.bodyKey }] }] },
      createApp: async (appId: string) => { created.push(appId); await Promise.resolve(); return created.length; },
      destroyApp: async (id: number) => { destroyed.push(id); },
      refreshUi: async (instanceId: number, request: any) => {
        requests.push({ instanceId, request });
        const props = JSON.parse(request.viewState.extensionInputJson);
        return { windows: [{ key: law.windowId, hash: "rendered", value: node("parameters", { type: "text", value: props.paramsJson, emphasize: null, dataAttributes: null }) }] };
      },
    };
    const context = { plugins: new Map([[law.pluginId, handle]]), contributorInstances: new Map(), contributorPasses: new Map(), viewState: law.viewState, ownerId: law.ownerId };
    const slots = law.slots.map((slot) => node(slot.key, { type: "extension", extension: `${law.pluginId}/${law.appId}`, props: slot.props }));
    const first = await resolveExternalSlots(slots[0], { ...context, ownerId: `${law.ownerId}:a` });
    const second = await resolveExternalSlots(slots[1], { ...context, ownerId: `${law.ownerId}:b` });
    expect(created).toEqual([law.appId, law.appId]);
    expect(first.children[0].component).toMatchObject({ type: "text", value: law.slots[0].props.paramsJson });
    expect(second.children[0].component).toMatchObject({ type: "text", value: law.slots[1].props.paramsJson });
    expect(first.key).toBe(slots[0].key);
    expect(requests.map(({ instanceId }) => instanceId)).toEqual([1, 2]);
    for (const { request } of requests) {
      expect(request.windows).toEqual([{ key: law.windowId, bodyKey: law.bodyKey }]);
      expect(request.viewState.windowInstances).toEqual([{ id: law.windowId, windowKindId: law.windowKindId }]);
      expect(request.viewState.locale).toBe(law.viewState.locale);
      expect(request.viewState.activeModeId).toBe(law.modeId);
      expect(parseResolvedPluginViewState(request.viewState)).toEqual(request.viewState);
    }
    const changed = { ...slots[0], component: { ...slots[0].component, props: law.slots[1].props } };
    const current = await resolveExternalSlots(changed, { ...context, ownerId: `${law.ownerId}:a` });
    expect(created).toHaveLength(2);
    expect(requests.at(-1)?.instanceId).toBe(1);
    expect(current.children[0].component).toMatchObject({ type: "text", value: law.slots[1].props.paramsJson });
    await resolveExternalSlots(node("empty", { type: "text", value: "Empty", emphasize: null, dataAttributes: null }), { ...context, ownerId: `${law.ownerId}:a` });
    expect(destroyed).toEqual([1]);
    expect(context.contributorInstances.size).toBe(1);
  });

  it("shares pending creations and retires the exact detached contributor", async () => {
    const { ensureContributorInstance, retireContributorInstances } = await import("@semio-tech/framework");
    const { default: law } = await import("../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧩️external-slots/🔣️.json");
    let release!: (id: number) => void;
    const destroyed: number[] = [];
    let created = 0;
    const handle = {
      manifest: { apps: [] },
      createApp: async () => { created++; return new Promise<number>((resolve) => { release = resolve; }); },
      destroyApp: async (id: number) => { destroyed.push(id); },
      refreshUi: async () => ({}),
    };
    const context = { plugins: new Map([[law.pluginId, handle]]), contributorInstances: new Map(), contributorPasses: new Map(), viewState: law.viewState, ownerId: law.ownerId };
    const first = ensureContributorInstance(law.pluginId, law.appId, [law.slots[0].key], context)!;
    const second = ensureContributorInstance(law.pluginId, law.appId, [law.slots[0].key], context)!;
    expect(first).toBe(second);
    await Promise.resolve();
    expect(created).toBe(1);
    const retirement = retireContributorInstances(context.contributorInstances);
    expect(context.contributorInstances.size).toBe(0);
    release(17);
    await retirement;
    expect(destroyed).toEqual([17]);
  });

  it("does not render or recreate an extension after its parent retires during creation", async () => {
    const { resolveExternalSlots, retireContributorInstances } = await import("@semio-tech/framework");
    const { default: law } = await import("../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧩️external-slots/🔣️.json");
    let release!: (id: number) => void;
    let refreshes = 0;
    const destroyed: number[] = [];
    const handle = {
      manifest: { apps: [{ id: law.appId, controllerId: law.controllerId, defaultModeId: law.modeId, modes: [{ id: law.modeId }], windowKinds: [{ id: law.windowKindId, bodyKey: law.bodyKey }] }] },
      createApp: async () => new Promise<number>((resolve) => { release = resolve; }),
      destroyApp: async (id: number) => { destroyed.push(id); },
      refreshUi: async () => { refreshes++; return {}; },
    };
    const context = { plugins: new Map([[law.pluginId, handle]]), contributorInstances: new Map(), contributorPasses: new Map(), viewState: law.viewState, ownerId: law.ownerId };
    const slot: any = { key: law.slots[0].key, component: { type: "extension", extension: `${law.pluginId}/${law.appId}`, props: law.slots[0].props }, children: [] };
    const rendering = resolveExternalSlots(slot, context);
    await Promise.resolve();
    const retiring = retireContributorInstances(context.contributorInstances);
    release(23);
    await Promise.all([rendering, retiring]);
    expect(refreshes).toBe(0);
    expect(destroyed).toEqual([23]);
    expect(context.contributorInstances.size).toBe(0);
  });

  it("discards effects from a refresh that finishes after retirement", async () => {
    const { resolveExternalSlots, retireContributorInstances } = await import("@semio-tech/framework");
    const { default: law } = await import("../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧩️external-slots/🔣️.json");
    let release!: (response: any) => void;
    let started!: () => void;
    const refreshing = new Promise<void>((resolve) => { started = resolve; });
    const effects: unknown[] = [];
    const destroyed: number[] = [];
    const handle = {
      manifest: { apps: [{ id: law.appId, controllerId: law.controllerId, defaultModeId: law.modeId, modes: [{ id: law.modeId }], windowKinds: [{ id: law.windowKindId, bodyKey: law.bodyKey }] }] },
      createApp: async () => 24,
      destroyApp: async (id: number) => { destroyed.push(id); },
      refreshUi: async () => { started(); return new Promise<any>((resolve) => { release = resolve; }); },
    };
    const context = { plugins: new Map([[law.pluginId, handle]]), contributorInstances: new Map(), contributorPasses: new Map(), viewState: law.viewState, ownerId: law.ownerId, onEffects: (...args: unknown[]) => { effects.push(args); } };
    const slot: any = { key: law.slots[0].key, component: { type: "extension", extension: `${law.pluginId}/${law.appId}`, props: law.slots[0].props }, children: [] };
    const rendering = resolveExternalSlots(slot, context);
    await refreshing;
    const retiring = retireContributorInstances(context.contributorInstances);
    release({ requestedEffects: [{ type: "diagnostic", message: "late extension work" }], windows: [] });
    await Promise.all([rendering, retiring]);
    expect(effects).toEqual([]);
    expect(destroyed).toEqual([24]);
  });

  it("renders external slot fallback text when unresolved", () => {
    // 🧬️ MIGRATION: `Component::Extension` (the old `ExternalSlot`) collapses `pluginId`/`appId`/
    // `bodyKey` into one opaque `extension` address string — see `ExtensionProps`'s own doc.
    const markup = renderContractTree({ key: "missing-module", component: { type: "extension", extension: "missing-module", props: {} } });
    expect(markup).toContain("Extension unavailable: missing-module");
  });
});

describe("declarative forms parity", () => {
  it("renders declarative text with appearance-aware foreground", () => {
    const markup = renderContractTree({ key: "text", component: { type: "text", value: "Hello flow", emphasize: null, dataAttributes: null } });
    expect(markup).toContain("text-foreground");
    expect(markup).toContain("Hello flow");
    const emphasized = renderContractTree({ key: "text", component: { type: "text", value: "Emphasized", emphasize: true, dataAttributes: null } });
    expect(emphasized).toContain("text-foreground");
    expect(emphasized).toContain("font-semibold");
  });

  it("dag overlay label fills resolve to Canvas2D-safe hex for appearance", () => {
    const chrome = { selectedIds: new Set<string>(["sel"]), highlightedIds: new Set<string>(["hi"]) };
    expect(dagOverlayLabelFill("plain", false, null, chrome)).toBe("var(--color-muted-foreground)");
    expect(dagOverlayLabelFill("sel", false, null, chrome)).toBe("var(--color-foreground)");
    const muted = dagOverlayLabelFillHex("plain", false, null, chrome);
    const selected = dagOverlayLabelFillHex("sel", false, null, chrome);
    const highlighted = dagOverlayLabelFillHex("hi", false, null, chrome);
    const hovered = dagOverlayLabelFillHex("plain", false, "plain", chrome);
    const ghost = dagOverlayLabelFillHex("ghost", true, null, chrome);
    const dimmed = dagOverlayLabelFillHex("plain", false, null, chrome, ["plain"]);
    for (const hex of [muted, selected, highlighted, hovered, ghost, dimmed]) {
      expect(hex).toMatch(/^#[0-9a-f]{6}$/iu);
      expect(hex).not.toBe("#000000");
    }
    expect(selected).toBe(hovered);
    expect(highlighted).toBe(ghost);
  });

  it("renders field description, required marker and inline error", () => {
    // 🧬️ MIGRATION: the old `field`/`input` `UiNode` pair collapses into one `Component::Container`
    // (`role: "field"`) whose single child IS the input — `ContainerProps`'s own doc.
    const markup = renderContractTree({
      key: "forms-try.name",
      component: { type: "container", role: "field", label: "Name", description: "Your full name", required: true, error: "Name is required", defaultOpen: null, dropOverlay: null },
      children: [{ key: "forms-try.name.input", component: { type: "input", kind: "text", value: "", placeholder: null, commit: null, min: null, max: null, step: null, accept: null, precision: null, snaps: [] } }],
    });
    expect(markup).toContain("Your full name");
    expect(markup).toContain("Name is required");
    expect(markup).toContain("*");
    expect(markup).toContain('data-slot="field-error"');
  });

  it("renders slider unit readout", () => {
    const markup = renderContractTree({ key: "forms-try.volume.slider", component: { type: "slider", value: 60, min: 0, max: 100, step: 5, unit: "%", snaps: [] } });
    expect(markup).toContain("60 %");
  });

  it("renders numberStepper as a single-border Stepper control, not hand-rolled double-bordered buttons", () => {
    const markup = renderContractTree({ key: "forms-try.height.stepper", component: { type: "numberStepper", value: 3, step: 1, uniform: true, min: null, max: null, precision: null } });
    expect(markup).toContain('data-slot="stepper-group"');
    expect(markup).toContain('data-slot="stepper-minus"');
    expect(markup).toContain('data-slot="stepper-plus"');
    expect(markup).not.toContain("border-border");
  });

  it("shows the mixed-values placeholder on a non-uniform numberStepper", () => {
    const markup = renderContractTree({ key: "forms-try.height.stepper", component: { type: "numberStepper", value: 0, step: 1, uniform: false, min: null, max: null, precision: null } });
    expect(markup).toContain('data-mixed="true"');
  });

  it("renders a group node as a labeled section nesting its child controls (Origin > X/Y/Z steppers)", () => {
    // 🧬️ MIGRATION: `group`/`field` both collapse into `Component::Container` (`role: "group"` /
    // `role: "field"`) — the old `child: Box<UiNode>` singular is simply `children[0]` on the record.
    const markup = renderContractTree({
      key: "puzzle3d-play-inspector.object.origin",
      component: { type: "container", role: "group", label: "Origin", description: null, required: null, error: null, defaultOpen: true, dropOverlay: null },
      children: [
        {
          key: "puzzle3d-play-inspector.object.origin.x",
          component: { type: "container", role: "field", label: "X", description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
          children: [{ key: "puzzle3d-play-inspector.object.origin.x.stepper", component: { type: "numberStepper", value: 1, step: 0.1, uniform: true, min: null, max: null, precision: null } }],
        },
        {
          key: "puzzle3d-play-inspector.object.origin.y",
          component: { type: "container", role: "field", label: "Y", description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
          children: [{ key: "puzzle3d-play-inspector.object.origin.y.stepper", component: { type: "numberStepper", value: 2, step: 0.1, uniform: true, min: null, max: null, precision: null } }],
        },
      ],
    });
    expect(markup).toContain(">Origin</h2>");
    expect(markup).toContain(">X</label>");
    expect(markup).toContain(">Y</label>");
    expect(markup).toContain('data-slot="stepper-group"');
  });

  // 🧬️ MIGRATION: `LayoutSpec`'s stack `gap`/`padding` are now a closed `SpaceToken` enum resolved to
  // inline CSS via `spaceTokenRem` (Interpreter's own `layoutSpecStyle`/`LayoutAndStyle` region) —
  // the new architecture's real replacement for "never a hardcoded raw rem" is a renderer-neutral
  // token resolved to CSS at read time, not the pre-migration Tailwind-gap-class scheme this test
  // used to assert (`not.toContain("style=")` is no longer true BY DESIGN, not a regression — see
  // `react-renderer` packet's own decisions doc). Rewritten to assert the token resolves through that
  // closed scale (never an arbitrary raw number), while separators still avoid the raw
  // `border-border` utility class.
  it("resolves stack gap/padding through the closed SpaceToken scale as inline CSS, and keeps separators off raw border-border", () => {
    const markup = renderContractTree({
      key: "forms-blueprint.section.q1",
      component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
      layout: { kind: "stack", axis: "vertical", gap: "xs", padding: { all: "none" }, align: "start", justify: "start", grow: false, wrap: false },
      children: [
        { key: "text", component: { type: "text", value: "text · q1", emphasize: null, dataAttributes: null } },
        { key: "sep", component: { type: "separator" } },
      ],
    });
    expect(markup).toMatch(/gap:\s*calc\(1 \* var\(--ui-spacing\)\)/);
    expect(markup).not.toContain("border-border");
  });

  it("passes number bounds and file accept to inputs", () => {
    const numberMarkup = renderContractTree({ key: "forms-try.age.input", component: { type: "input", kind: "number", value: "28", placeholder: null, commit: null, min: 13, max: 120, step: 1, accept: null, precision: null, snaps: [] } });
    expect(numberMarkup).toContain('min="13"');
    expect(numberMarkup).toContain('max="120"');
    const fileMarkup = renderContractTree({ key: "forms-try.resume.input", component: { type: "input", kind: "file", value: "", placeholder: null, commit: null, min: null, max: null, step: null, accept: ".pdf,.doc", precision: null, snaps: [] } });
    expect(fileMarkup).toContain('accept=".pdf,.doc"');
  });

  it("disables gated wizard buttons", () => {
    // 🧬️ MIGRATION: `disabled` moved off the component (`ButtonProps` no longer carries it) onto the
    // record itself (`record.disabled` — `ButtonView`'s own `disabled={record.disabled}`); `action`
    // moved to the record's `bindings`, keyed by `Trigger::Activate`.
    const markup = renderContractTree({
      key: "forms-try.next",
      component: { type: "button", icon: "chevron-right", label: "Next" },
      disabled: true,
      bindings: [{ trigger: "activate", action: { scope: "forms-play", name: "nextStep", version: 1 }, args: null, capability: null }],
    });
    expect(markup).toContain("disabled");
  });

  it("renders selectable builder cards with selection ring", () => {
    // 🧬️ MIGRATION: `selected` is no longer a document field — presence (hover/selection) is a
    // separate `UiPresenceOverlayContext` channel keyed by `UiNodeRecord.key`, fed from
    // `PresenceUpdate` wire messages, never part of the retained document (`PresenceOverlay`
    // region's own doc: "presence changes at input frequency and must not touch a document
    // revision"). `data-ui-path` (a tree-position string) is gone too — the record's own stable
    // `data-ui-node-id` is the only per-node DOM handle now.
    const markup = renderContractTree(
      {
        key: "forms-blueprint.card.q1",
        component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
        bindings: [{ trigger: "activate", action: { scope: "forms-play", name: "setSelection", version: 1 }, args: null, capability: null }],
        children: [{ key: "text", component: { type: "text", value: "text · q1", emphasize: null, dataAttributes: null } }],
      },
      { "forms-blueprint.card.q1": { selected: true } },
    );
    expect(markup).toContain('data-ui-node-id="0"');
    expect(markup).toContain('role="button"');
    expect(markup).toContain("ring-primary");
  });

  it("applies separate presence fixtures without replacing the retained document", () => {
    // 🧬️ One owner for this fixture: `framework.ui.contract`'s `ContractFixture` export (ticket
    // 26/09/08 `📋️cross-partition-requests.md` row 145). `framework.ui` no longer restates it as
    // `PresenceOverlayFixture`; this consumer compiles the owning scope's export by its `$id`.
    
    
    
    
    const store = new UiDocumentStore("document");
    store.loadSnapshot(buildContractSnapshot({ key: "item:根,1", component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null } }));
    const before = store.getState();
    const context: UiInterpreterContext = { store, onAction: noopAction, onIntent: () => {} };
    const mounted = render(createElement(UiPresenceOverlayContext.Provider, { value: { byKey: new Map() } }, interpretUiNode(store, context)));
    for (const row of presenceOverlayFixture.cases) {
      const own: UiPresenceOverlayEntry = JSON.parse(JSON.stringify(row.update.own));
      mounted.rerender(createElement(UiPresenceOverlayContext.Provider, { value: { byKey: new Map([[row.update.nodeKey, own]]) } }, interpretUiNode(store, context)));
      expect(mounted.container.innerHTML.includes("ring-primary")).toBe(row.expected.selected);
      expect(mounted.container.innerHTML.includes("outline-primary/50")).toBe(row.expected.hovered);
      expect(own.previewed ?? false).toBe(row.expected.previewed);
      expect(store.getState()).toBe(before);
      expect(store.getRevisionSnapshot()).toBe(before.revision);
    }
  });

  it("renders image nodes from url sources", () => {
    const markup = renderContractTree({ key: "forms-try.avatar.image", component: { type: "image", src: "https://example.com/avatar.png", alt: "Avatar" } });
    expect(markup).toContain('src="https://example.com/avatar.png"');
    expect(markup).toContain('alt="Avatar"');
  });

  // 🪦️ MIGRATION, deleted (not rewritten): `declarativeTreeDragController` — a standalone pure
  // function taking a whole tree `UiNode` + a dispatch callback and returning a
  // `TreeDragAndDropController` — was deliberately not ported forward (react-renderer packet's own
  // decisions doc; the barrel's in-file migration comment says so too). Drag/drop for a `tree`
  // component is now wired INSIDE `🟦️Interpreter`'s own `TreeView` (built from the record's own `drop`
  // `ActionBinding`, dispatched through `dispatchTrigger`/`emitIntent`), not a separately-importable
  // factory this file's OWNS can call in isolation — there is no equivalent unit boundary left.
  // See this packet's report for a production-bug flag this deletion surfaced: `TreeView`'s current
  // `handleDrop` (`🟦️Interpreter/🟦️.tsx`) calls `dispatchTrigger(context, record, "drop")`
  // with NO input payload at all, discarding the drop event's target/payload/position entirely —
  // this test's old assertion (`args: { kind, targetId, dropPosition }`) has no successor to assert
  // against today.
});

describe("framework renderer hosts", () => {
  afterEach(() => cleanup());
  it("renders node graph host from workflow scene json", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                width: 160,
                height: 80,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });

  it("retires a mounted graph gesture without synthesizing an up or graph commit", async () => {
    const attachCanvas = vi.fn(async () => {});
    const pointerDownScreen = vi.fn();
    const pointerUpScreen = vi.fn();
    const pointerCancelScreen = vi.fn();
    const onAction = vi.fn();
    const session = {
      attachCanvas,
      setCaretVisible: vi.fn(),
      setSize: () => {},
      renderFrame: vi.fn(),

      synchronizeScene: () => {},
      setCanvasThemeJson: () => {},
      pointerDownScreen,
      pointerMoveScreen: () => {},
      pointerUpScreen,
      pointerCancelScreen,
      wheelScreen: () => {},
      labelOverlayPaintStateJson: () => '{"labels":[]}',
      sliderOverlayStateJson: () => "{}",
      selectionUnionBoundsScreenJson: () => "{}",
      selectionPreviewPointsJson: () => "[]",
      selectionPreviewCrossing: () => false,
      selectionPreviewMethod: () => "rectangle",
      selectedNodeIdsJson: () => "[]",
      hoveredNodeId: () => null,
      hoveredChannelJson: () => "{}",
      viewport: () => ({ x: 0, y: 0, zoom: 1 }),
      pickTargetsAtScreenJson: () => "[]",
      setHover: () => {},
      setHoverChannel: () => {},
      alignSelection: () => {},
      hostSnapshotJson: () => "{}",
      takePendingOpenInstanceId: () => null,
      free: vi.fn(),
    } as unknown as Awaited<ReturnType<typeof flowSessionLoader.createGraphSession>>;
    const factory = vi.spyOn(flowSessionLoader, "createGraphSession").mockResolvedValue(session);
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const view = render(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "graph.cancel",
          controllerId: "graph",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [{ id: "node-a", instanceId: "app-a", label: "Draw", x: 10, y: 20, width: 160, height: 80, inputs: [], outputs: [] }],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
            editable: true,
            // 🎯️ Hover and selection are published to an interaction DOMAIN: `nodeGraphHoverActionArgs`
            // (`🧱️elements/🕸️NodeGraph/🟦️.ts?:252`) returns `undefined` without one, so a scene that
            // declares none can never publish the retired hover this law measures.
            interactionDomain: { id: "graph.cancel", nodeTargetPrefix: "node:", edgeTargetPrefix: "edge:", handleTargetPrefix: "handle:" },
          },
        },
        onAction,
      }),
    );
    try {
      await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
      const surface = view.container.querySelector('.semio-node-graph-host [class*="z-30"]') as HTMLElement;
      await waitFor(() => {
        fireEvent.pointerDown(surface, { pointerId: 11, button: 0, clientX: 20, clientY: 20 });
        expect(pointerDownScreen).toHaveBeenCalled();
      });
      const downsBeforeCancel = pointerDownScreen.mock.calls.length;
      const actionsBeforeCancel = onAction.mock.calls.length;
      fireEvent.pointerCancel(surface, { pointerId: 11, clientX: 30, clientY: 20 });
      expect(pointerCancelScreen).toHaveBeenCalledOnce();
      expect(pointerUpScreen).not.toHaveBeenCalled();
      expect(onAction.mock.calls.slice(actionsBeforeCancel).map(([action]) => action.action)).toEqual(["interactionHover"]);
      fireEvent.pointerDown(surface, { pointerId: 12, button: 0, clientX: 40, clientY: 20 });
      expect(pointerDownScreen).toHaveBeenCalledTimes(downsBeforeCancel + 1);
    } finally {
      view.unmount();
      factory.mockRestore();
      bounds.mockRestore();
      canvasContext.mockRestore();
    }
  });

  it("renders editable node graph host with find items", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                width: 160,
                height: 80,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
            editable: true,
            findItems: [{ id: "app-a", label: "Draw", category: "Workflow" }],
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });

  it("uses the live session viewport for node graph wheel actions", () => {
    expect(parseNodeGraphSessionViewport({ x: 12, y: 24, zoom: 1.75 })).toEqual({ x: 12, y: 24, zoom: 1.75 });
    expect(nodeGraphViewportActionArgs({ x: 12, y: 24, zoom: 1.75 })).toEqual({
      viewport: { x: 12, y: 24, zoom: 1.75 },
    });
    expect(() => parseNodeGraphSessionViewport('{"x":12,"y":24,"zoom":1.75}')).toThrow();
    expect(() => nodeGraphViewportActionArgs({ x: 0, y: 0, zoom: 0 })).toThrow();
    expect(() => nodeGraphViewportActionArgs({ x: 0, y: 0, zoom: Number.NaN })).toThrow();
    expect(() => nodeGraphViewportActionArgs({ x: 0, y: 0, zoom: 1, extra: true } as never)).toThrow();
  });

  it("matches the shared neutral viewport schema at the node graph action boundary", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(viewport2dSchema);
    const cases = viewportPoseFixture.cases.filter((row) => row.dimension === "2d");
    for (const row of cases) {
      expect(validate(row.value), row.name).toBe(row.valid);
      if (row.valid) {
        expect(nodeGraphViewportActionArgs(parseNodeGraphSessionViewport(row.value)), row.name).toEqual({ viewport: row.value });
      } else {
        expect(() => nodeGraphViewportActionArgs(parseNodeGraphSessionViewport(row.value)), row.name).toThrow();
      }
    }
    expect(cases).toHaveLength(8);
  });

  it("encodes node graph selection and hover with its scene-owned framework interaction address", () => {
    const domain = { id: "graph", nodeTargetPrefix: "document.node.", edgeTargetPrefix: "document.edge.", handleTargetPrefix: "document.handle." };
    expect(nodeGraphSelectionActionArgs(domain, { nodeIds: ["node-a"], edgeIds: ["edge-a"], handleIds: ["handle-a"] })).toEqual({
      domainId: "graph",
      targets: JSON.stringify([
        { granularity: "node", id: "document.node.node-a" },
        { granularity: "edge", id: "document.edge.edge-a" },
        { granularity: "handle", id: "document.handle.handle-a" },
      ]),
      merge: "replace",
      method: "pick",
    });
    expect(nodeGraphHoverActionArgs(domain, "node-a")).toEqual({
      domainId: "graph",
      channel: "pointer",
      targets: JSON.stringify([{ granularity: "node", id: "document.node.node-a" }]),
    });
    expect(nodeGraphSelectionActionArgs(undefined, { nodeIds: ["node-a"] })).toBeUndefined();
    expect(nodeGraphHoverActionArgs(undefined, "node-a")).toBeUndefined();
  });

  // 🎯️ A world-3d window bound to an interaction domain must speak the SAME framework verbs the node
  // graph does, so hovering geometry in the 3D view and hovering its node in the graph land on one
  // shared hover state — the two halves of generation3d's bidirectional hover.
  it("encodes world3d interaction dispatch args the same way the node graph does", () => {
    expect(WORLD3D_DEFAULT_INTERACTION_GRANULARITY).toBe("handle");
    expect(world3dHoverActionArgs("graph", "handle", "extrude@solid")).toEqual({
      domainId: "graph",
      channel: "pointer",
      targets: JSON.stringify([{ granularity: "handle", id: "extrude@solid" }]),
    });
    expect(world3dHoverActionArgs("graph", "handle", null)).toEqual({
      domainId: "graph",
      channel: "pointer",
      targets: JSON.stringify([]),
    });
    expect(world3dSelectionActionArgs("graph", "handle", ["extrude@solid"], "replace")).toEqual({
      domainId: "graph",
      targets: JSON.stringify([{ granularity: "handle", id: "extrude@solid" }]),
      merge: "replace",
      method: "pick",
    });
  });

  // 🧿️ A marker hit (vortex / target volume / reference) must reach the SAME generic
  // `interactionSelect`/`interactionHover` verbs an instance hit does, at its own granularity — the
  // host never learns an app's granularity names: the layer name is the default and the scene record
  // overrides it.
  it("resolves world3d marker interaction targets from the scene record, not from host knowledge", () => {
    expect(WORLD3D_DEFAULT_MARKER_GRANULARITY).toEqual({ vortex: "vortex", attraction: "attraction", targetVolume: "targetVolume", reference: "reference" });
    expect(world3dMarkerInteractionTarget("vortex", "seed-left-001:v0")).toEqual({ granularity: "vortex", id: "seed-left-001:v0" });
    expect(world3dMarkerInteractionTarget("vortex", "seed-left-001:v0", {})).toEqual({ granularity: "vortex", id: "seed-left-001:v0" });
    expect(world3dMarkerInteractionTarget("targetVolume", "volume-1")).toEqual({ granularity: "targetVolume", id: "volume-1" });
    expect(world3dMarkerInteractionTarget("reference", "ref-1")).toEqual({ granularity: "reference", id: "ref-1" });
    expect(world3dMarkerInteractionTarget("vortex", "seed-left-001:v0", { interactionGranularityId: "pin", interactionId: "pin-7" })).toEqual({ granularity: "pin", id: "pin-7" });
  });

  // 🎯️ An instance pick, hover and marquee resolve each rendered row onto ITS OWN granularity — a fem3d
  // scene draws nodes, members, solids, supports and load glyphs as one instance lane under one domain,
  // and a member must select an `element`, never the scene default `node`. Rows without a declared
  // granularity keep the scene's default; the load glyph redirects onto its load through both fields.
  it("resolves world3d instance interaction targets per record, falling back to the scene granularity", () => {
    const instances = [{ id: "n1" }, { id: "e1", interactionGranularityId: "element" }, { id: "l2:head", interactionId: "l2", interactionGranularityId: "load" }, { id: "l2:shaft", interactionId: "l2", interactionGranularityId: "load" }];
    expect(world3dInstanceInteractionTarget(instances, "n1", "node")).toEqual({ granularity: "node", id: "n1" });
    expect(world3dInstanceInteractionTarget(instances, "e1", "node")).toEqual({ granularity: "element", id: "e1" });
    expect(world3dInstanceInteractionTarget(instances, "l2:shaft", "node")).toEqual({ granularity: "load", id: "l2" });
    expect(world3dInstanceInteractionTarget(instances, "ghost", "node")).toEqual({ granularity: "node", id: "ghost" });
    expect(world3dInstanceInteractionTargets(instances, ["l2:head", "e1", "l2:shaft", "n1"], "node")).toEqual([
      { granularity: "load", id: "l2" },
      { granularity: "element", id: "e1" },
      { granularity: "node", id: "n1" },
    ]);
    expect(world3dSelectionTargetsActionArgs("fem3d", world3dInstanceInteractionTargets(instances, ["e1", "n1"], "node"), "replace", "rectangle")).toEqual({
      domainId: "fem3d",
      targets: JSON.stringify([
        { granularity: "element", id: "e1" },
        { granularity: "node", id: "n1" },
      ]),
      merge: "replace",
      method: "rectangle",
    });
    expect(world3dSelectionTargetsActionArgs("fem3d", [{ granularity: "node", id: "n1" }], "additive")).toEqual({
      domainId: "fem3d",
      targets: JSON.stringify([{ granularity: "node", id: "n1" }]),
      merge: "additive",
      method: "pick",
    });
  });

  // 🧿️ The exact wire shapes a vortex-marker click and hover now put on the domain path — the same
  // encoder the instance path uses, so one selection authority sees both.
  it("encodes a vortex marker pick and hover as generic domain interaction args", () => {
    const target = world3dMarkerInteractionTarget("vortex", "seed-left-001:v0");
    expect(world3dSelectionActionArgs("vortex", target.granularity, [target.id], "invertive")).toEqual({
      domainId: "vortex",
      targets: JSON.stringify([{ granularity: "vortex", id: "seed-left-001:v0" }]),
      merge: "invertive",
      method: "pick",
    });
    expect(world3dHoverActionArgs("vortex", target.granularity, target.id)).toEqual({
      domainId: "vortex",
      channel: "pointer",
      targets: JSON.stringify([{ granularity: "vortex", id: "seed-left-001:v0" }]),
    });
    expect(world3dHoverActionArgs("vortex", WORLD3D_DEFAULT_MARKER_GRANULARITY.vortex, undefined)).toEqual({
      domainId: "vortex",
      channel: "pointer",
      targets: JSON.stringify([]),
    });
  });

  // 🎯️ Several rendered instances can stand for ONE interaction target — generation3d renders one
  // instance per geometry item of a channel and every one of them resolves to that channel's port,
  // which is the id its interaction topology actually declares.
  it("collapses world instance ids onto the interaction targets they stand for", () => {
    const instances = [{ id: "profile@wire#0", interactionId: "profile@wire" }, { id: "profile@wire#1", interactionId: "profile@wire" }, { id: "extrude@solid#0", interactionId: "extrude@solid" }, { id: "plain-instance" }];
    expect(interactionTargetsForInstances(instances, ["profile@wire#0", "profile@wire#1", "extrude@solid#0"])).toEqual(["profile@wire", "extrude@solid"]);
    expect(interactionTargetsForInstances(instances, ["plain-instance"])).toEqual(["plain-instance"]);
    expect(interactionTargetsForInstances(instances, ["unknown-id"])).toEqual(["unknown-id"]);
  });

  it("encodes node graph scenes as pack bytes for wasm sync", async () => {
    const scene = {
      nodes: [],
      edges: [],
      viewport: { x: 0, y: 0, zoom: 1 },
    };
    const bytes = sceneToSyncPack(scene);
    expect(bytes.length).toBeGreaterThan(8);
  });

  //#region 🎚️GraphSliderAccessibility
  it("decodes graph pick channels from the native handle grammar with strict schema parity", () => {
    const validateTarget = rendererExport("NodeGraphPickTargetV1");
    const validateChannel = rendererExport("NodeGraphInteractionChannelV1");
    for (const row of graphPickFixture) {
      const target = row.target;
      expect(validateTarget(target), JSON.stringify(validateTarget.errors)).toBe(true);
      expect(validateChannel(row.channel), JSON.stringify(validateChannel.errors)).toBe(true);
      const match = target?.domain === "handle" ? /^([^@]+)@(.+)$/u.exec(target.id) : null;
      const oracle = match ? { nodeId: match[1], portId: match[2] } : null;
      expect(oracle).toEqual(row.channel);
      expect(nodeGraphPickChannel(target)).toEqual(row.channel);
    }
    const hostile = structuredClone(graphPickFixture);
    Object.assign(hostile[1]!.target!, { portId: "invented" });
    expect(validateTarget(hostile[1]!.target)).toBe(false);
  });
  it("validates strict language-neutral graph slider labels and rejects unnamed rows", () => {
    const validate = peerExport(dagVcsSchema, "SliderOverlay");
    expect(validate(graphSliderFixture), JSON.stringify(validate.errors)).toBe(true);
    for (const label of ["", "   ", null, 42]) {
      const malformed = structuredClone(graphSliderFixture);
      (malformed.cases[0]!.row as Record<string, unknown>).label = label;
      expect(validate(malformed)).toBe(false);
      expect(parseDagSliderOverlays(JSON.stringify({ sliders: [malformed.cases[0]!.row] }))).toEqual([]);
    }
    expect(parseDagSliderOverlays('{"sliders":{}}')).toEqual([]);
  });

  it("names graph slider overlays from exact localized captions and keeps scoped ids stable", () => {
    const first = graphSliderFixture.cases[0]!;
    const second = graphSliderFixture.cases[1]!;
    const props = (item: typeof first) => ({
      scopeId: item.scopeId,
      stateJson: JSON.stringify({ camera: { x: 0, y: 0, zoom: 1 }, sliders: [item.row] }),
      logicalW: 800,
      logicalH: 600,
      editable: true,
      onSliderChange: () => {},
    });
    const view = render(createElement("div", {}, createElement(GraphSliderOverlays, props(first)), createElement(GraphSliderOverlays, props(second))));
    const english = view.getByRole("slider", { name: first.row.label });
    const german = view.getByRole("slider", { name: second.row.label });
    expect(computeAccessibleName(english)).toBe(first.row.label);
    expect(computeAccessibleName(german)).toBe(second.row.label);
    const controlId = english.closest('[data-slot="slider"]')!.id;
    expect(controlId).toBe(`graph-slider-${encodeURIComponent(JSON.stringify([first.scopeId, first.row.widgetId]))}`);
    expect(german.closest('[data-slot="slider"]')!.id).not.toBe(controlId);
    view.rerender(createElement("div", {}, createElement(GraphSliderOverlays, { ...props(first), stateJson: JSON.stringify({ sliders: [{ ...first.row, value: 3 }] }) }), createElement(GraphSliderOverlays, props(second))));
    expect(view.getByRole("slider", { name: first.row.label }).closest('[data-slot="slider"]')!.id).toBe(controlId);
  });

  it("keeps graph slider keyboard changes exact and disabled controls inert", () => {
    const item = graphSliderFixture.cases[1]!;
    const changes = vi.fn();
    const gesture = vi.fn();
    const props = { scopeId: item.scopeId, stateJson: JSON.stringify({ sliders: [item.row] }), logicalW: 800, logicalH: 600, editable: true, onSliderChange: changes, onSliderPointerDown: gesture };
    const view = render(createElement(GraphSliderOverlays, props));
    const slider = view.getByRole("slider", { name: item.row.label });
    slider.focus();
    expect(document.activeElement).toBe(slider);
    for (const action of graphSliderFixture.keyboard) {
      fireEvent.keyDown(slider, { key: action.key });
      fireEvent.keyUp(slider, { key: action.key });
      expect(changes).toHaveBeenLastCalledWith(item.row.widgetId, action.expected);
    }
    expect(gesture).not.toHaveBeenCalled();
    changes.mockClear();
    view.rerender(createElement(GraphSliderOverlays, { ...props, editable: false }));
    const disabled = view.getByRole("slider", { name: item.row.label });
    expect(disabled.getAttribute("aria-disabled")).toBe("true");
    expect(disabled.tabIndex).toBe(-1);
    fireEvent.keyDown(disabled, { key: "ArrowRight" });
    fireEvent.keyUp(disabled, { key: "ArrowRight" });
    expect(changes).not.toHaveBeenCalled();
  });

  /** ⚖️ LAW (scrub protocol, `📓️api-scrub-machine.md`): a graph slider cancels its open press like every continuous
   * control — a lost pointer capture as `captureLost`, a blur as `blur` — so the guest drops the press with zero trace. */
  it("cancels a graph slider press on lost capture and on blur", () => {
    const item = graphSliderFixture.cases[1]!;
    const aborts = vi.fn();
    const view = render(createElement(GraphSliderOverlays, { scopeId: item.scopeId, stateJson: JSON.stringify({ sliders: [item.row] }), logicalW: 800, logicalH: 600, editable: true, onSliderChange: vi.fn(), onSliderAbort: aborts }));
    const slider = view.getByRole("slider", { name: item.row.label });
    const root = slider.closest('[data-slot="slider"]')!;
    fireEvent.pointerDown(root, { pointerId: 1, clientX: 0 });
    fireEvent.pointerCancel(root, { pointerId: 1 });
    expect(aborts).toHaveBeenLastCalledWith(item.row.widgetId, "captureLost");
    slider.focus();
    fireEvent.blur(slider);
    expect(aborts).toHaveBeenLastCalledWith(item.row.widgetId, "blur");
  });

  /** ⚖️ LAW: a knob has ONE value. The readout painted beside the track and the `aria-valuenow` a
   * screen reader announces are two renderings of the same published number, so they can never
   * disagree — the readout used to be painted on the GPU, where it moved only when a frame was
   * drawn, and a released knob read `10` to a screen reader while the canvas said `0.0`
   * (`📓️slider-reevaluation-correctness-2026-09-15.md`). */
  it("renders a graph slider's painted readout and its aria value from one published number", () => {
    const item = graphSliderFixture.cases[0]!;
    const rowAt = (value: number) => ({ ...item.row, value, fontScreenPx: 9, gapScreenPx: 4 });
    const propsAt = (value: number) => ({
      scopeId: item.scopeId,
      stateJson: JSON.stringify({ camera: { x: 0, y: 0, zoom: 1 }, sliders: [rowAt(value)] }),
      logicalW: 800,
      logicalH: 600,
      editable: true,
      onSliderChange: () => {},
    });
    const view = render(createElement(GraphSliderOverlays, propsAt(item.row.min)));
    for (const value of [item.row.min, (item.row.min + item.row.max) / 2, item.row.max]) {
      view.rerender(createElement(GraphSliderOverlays, propsAt(value)));
      const knob = view.getByRole("slider", { name: item.row.label });
      const readout = view.container.querySelector(`[data-graph-slider-value="${item.row.widgetId}"]`);
      expect(readout, "the overlay paints the value beside the track it belongs to").not.toBeNull();
      expect(readout!.textContent).toBe(dagSliderValueText(value));
      expect(readout!.textContent).toBe(dagSliderValueText(Number(knob.getAttribute("aria-valuenow"))));
      expect(readout!.getAttribute("aria-hidden")).toBe("true");
    }
    const parsed = parseDagSliderOverlays(JSON.stringify({ sliders: [rowAt(item.row.max)] }))[0]!;
    expect(parsed.value).toBe(item.row.max);
    expect(dagSliderValueText(parsed.value)).toBe(view.container.querySelector(`[data-graph-slider-value="${item.row.widgetId}"]`)!.textContent);
  });
  //#endregion 🎚️GraphSliderAccessibility

  //#region 🎚️GraphParameterDispatch
  it("validates the strict language-neutral graph parameter contract for all three consumers", () => {
    
    const command = peerExport(flowParameterSchema, "GraphParameterCommand");
    
    for (const extra of ["snapshotJson", "hostSnapshotJson", "operations"]) {
      const malformed = structuredClone(graphParameterFixture);
      (malformed.cases[0] as Record<string, unknown>)[extra] = "{}";
      
      expect(command({ widgetId: "radius", value: 3, [extra]: "{}" })).toBe(false);
    }
    expect(new Set(graphParameterFixture.cases.map((value) => value.app))).toEqual(new Set(["flow", "generation2d", "generation3d"]));
    for (const value of graphParameterFixture.cases) expect(command({ widgetId: value.widgetId, value: value.request, surfaceId: value.surfaceId })).toBe(true);
  });

  it("dispatches graph parameter keyboard and drag events as bounded nodeGraphEdit operations with explicit commits", async () => {
    const task = <T>(value: T) => ({ result: Promise.resolve(value), subscribe: () => () => {}, cancel: vi.fn() });
    const scheduler = { invalidate: vi.fn(), paintNow: vi.fn(), beginContinuous: vi.fn(), endContinuous: vi.fn(), dispose: vi.fn() };
    const schedulerSpy = vi.spyOn(infiniteCanvasRenderer, "createDemandFrameScheduler").mockReturnValue(scheduler);
    const contextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const timerSpy = vi.spyOn(globalThis, "setTimeout");
    const createSpy = vi.spyOn(flowSessionLoader, "createFlowSession");
    const originalPointer = globalThis.PointerEvent;
    class TestPointerEvent extends MouseEvent {
      readonly pointerId: number;
      readonly pointerType: string;
      constructor(type: string, init: PointerEventInit = {}) {
        super(type, init);
        this.pointerId = init.pointerId ?? 1;
        this.pointerType = init.pointerType ?? "mouse";
      }
    }
    globalThis.PointerEvent = TestPointerEvent as unknown as typeof PointerEvent;
    try {
      for (const item of graphParameterFixture.cases) {
        const row = { widgetId: item.widgetId, label: item.label, ...item.before, x: 0, y: 0, w: 100, h: 16 };
        const methods: Record<string, ReturnType<typeof vi.fn>> = {
          documentJson: vi.fn(() => {
            throw new Error("slider must not serialize the fixture");
          }),
          sliderOverlayStateJson: vi.fn(() => task(JSON.stringify({ camera: { x: 0, y: 0, zoom: 1 }, sliders: [row] }))),
          labelOverlayPaintStateJson: vi.fn(() => task("{}")),
          setSliderValue: vi.fn((_id: string, value: number) => {
            row.value = value;
            return task(undefined);
          }),
          selectedWidgetIds: vi.fn(() => task("[]")),
          previewOffWidgetIds: vi.fn(() => task("[]")),
          selectionPreviewPointsJson: vi.fn(() => task("[]")),
          selectionPreviewCrossing: vi.fn(() => task(false)),
        };
        const session = new Proxy(methods, {
          get(target, key: string) {
            if (key === "then") return undefined;
            return (target[key] ??= vi.fn(() => task(undefined)));
          },
        }) as unknown as flowSessionLoader.FlowWasmSession;
        createSpy.mockResolvedValueOnce(session);
        const onAction = vi.fn();
        const view = render(
          createElement(FlowGraphCanvasHost, {
            scene: { nodes: [], edges: [], viewport: { x: 0, y: 0, zoom: 1 }, hostSnapshotJson: '{"schema":"flow.host_snapshot","widgets":[]}' },
            controllerId: item.controllerId,
            surfaceId: item.surfaceId,
            editable: true,
            keyboardPort: { current: null },
            onAction,
          }),
        );
        await waitFor(() => expect(view.getByRole("slider", { name: item.label })).toBeTruthy());
        const slider = view.getByRole("slider", { name: item.label });
        const root = slider.closest('[data-slot="slider"]') as HTMLElement;
        const stableId = root.id;
        for (const element of [slider, root]) {
          Object.defineProperties(element, { setPointerCapture: { value: () => {} }, releasePointerCapture: { value: () => {} }, hasPointerCapture: { value: () => true } });
        }
        root.getBoundingClientRect = () => ({ x: 0, y: 0, width: 100, height: 16, left: 0, right: 100, top: 0, bottom: 16, toJSON: () => ({}) });
        (root.querySelector('[data-slot="slider-track"]') as HTMLElement).getBoundingClientRect = root.getBoundingClientRect;
        const gesturePrefix = `${item.surfaceId}:${item.widgetId}`.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
        const gesturePattern = new RegExp(`^${gesturePrefix}:\\d+:\\d+$`);
        slider.focus();
        fireEvent.keyDown(slider, { key: "ArrowRight" });
        fireEvent.keyUp(slider, { key: "ArrowRight" });
        await waitFor(() =>
          expect(onAction).toHaveBeenLastCalledWith({
            controllerId: item.controllerId,
            action: graphParameterFixture.action,
            args: { surfaceId: item.surfaceId, operations: [{ operation: "setSlider", widgetId: item.widgetId, value: 3 }], gesture: expect.stringMatching(gesturePattern), commit: true },
          }),
        );
        expect(document.activeElement).toBe(slider);
        expect(computeAccessibleName(slider)).toBe(item.label);
        expect(root.id).toBe(stableId);
        let staleOverlay: ((value: string) => void) | undefined;
        methods.sliderOverlayStateJson!.mockImplementationOnce(() => ({
          ...task(""),
          result: new Promise<string>((resolve) => {
            staleOverlay = resolve;
          }),
        }));
        fireEvent.pointerDown(root, { pointerId: 1, pointerType: "mouse", button: 0, buttons: 1, clientX: 40, clientY: 8 });
        fireEvent.pointerMove(root, { pointerId: 1, pointerType: "mouse", buttons: 1, clientX: 80, clientY: 8 });
        await waitFor(() => {
          const event = onAction.mock.calls.at(-1)?.[0];
          expect(event).toMatchObject({ controllerId: item.controllerId, action: graphParameterFixture.action, args: { surfaceId: item.surfaceId, operations: [{ operation: "setSlider", widgetId: item.widgetId, value: 8 }], commit: false } });
          expect(event.args.gesture).toMatch(gesturePattern);
        });
        await waitFor(() => expect(slider.getAttribute("aria-valuenow")).toBe("8"));
        await reactAct(async () => {
          staleOverlay?.(JSON.stringify({ sliders: [{ ...row, value: 4 }] }));
        });
        expect(slider.getAttribute("aria-valuenow")).toBe("8");
        const count = onAction.mock.calls.length;
        const gesture = onAction.mock.calls.at(-1)?.[0].args.gesture;
        fireEvent.pointerUp(root, { pointerId: 1, button: 0, clientX: 80, clientY: 8 });
        await waitFor(() => expect(onAction).toHaveBeenCalledTimes(count + 1));
        expect(onAction).toHaveBeenLastCalledWith({
          controllerId: item.controllerId,
          action: graphParameterFixture.action,
          args: { surfaceId: item.surfaceId, operations: [{ operation: "setSlider", widgetId: item.widgetId, value: 8 }], gesture, commit: true },
        });
        fireEvent.pointerCancel(root, { pointerId: 1 });
        expect(onAction).toHaveBeenCalledTimes(count + 1);
        expect(methods.documentJson).not.toHaveBeenCalled();
        expect(
          onAction.mock.calls.every(
            ([event]) =>
              event.action === graphParameterFixture.action &&
              Object.keys(event.args).sort().join() === "commit,gesture,operations,surfaceId" &&
              event.args.operations.every((operation: Record<string, unknown>) => Object.keys(operation).sort().join() === "operation,value,widgetId"),
          ),
        ).toBe(true);
        view.unmount();
      }
      expect(timerSpy.mock.calls.some(([, delay]) => delay === 80)).toBe(false);
      expect(scheduler.beginContinuous).toHaveBeenCalledWith("gesture");
      expect(scheduler.endContinuous).toHaveBeenCalledWith("gesture");
    } finally {
      cleanup();
      createSpy.mockRestore();
      schedulerSpy.mockRestore();
      contextSpy.mockRestore();
      timerSpy.mockRestore();
      globalThis.PointerEvent = originalPointer;
    }
  });

  it("retains one shared Flow browser runtime across two mounted graph hosts and retires only each unmounted session", async () => {
    
    
    const expected = flowBrowserRuntimeFixture.mountedHosts;
    const bridge = new MockFlowBridge(new WebAssembly.Memory({ initial: 400 }));
    const runtime = await createFlowBrowserRuntime({ source: bridge.exports });
    const createSpy = vi.spyOn(flowSessionLoader, "createFlowSession").mockImplementation(async () => runtime.openSession());
    const contextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const scene = (revision: number) => ({
      nodes: [],
      edges: [],
      viewport: { x: 0, y: 0, zoom: 1 },
      hostSnapshotJson: JSON.stringify({ schema: "flow.host_snapshot", revision, widgets: [] }),
    });
    const host = (id: "A" | "B", revision: number) =>
      createElement(FlowGraphCanvasHost, {
        key: id,
        scene: scene(revision),
        controllerId: `flow.${id}`,
        surfaceId: `flow.${id}`,
        editable: true,
        keyboardPort: { current: null },
        onAction: vi.fn(),
      });
    const pair = (revision: number) => createElement("div", {}, host("A", revision), host("B", revision));
    const single = (revision: number) => createElement("div", {}, host("B", revision));
    const view = render(pair(0));
    try {
      expect(expected.sharedRuntime).toBe(true);
      await waitFor(() => expect(bridge.openRequestIds).toEqual(flowBrowserRuntimeFixture.openRequestIds));
      await waitFor(() => expect(bridge.operationSessions.some((row) => row.operation === expected.afterUnmountA.command && row.slot === expected.afterUnmountA.liveSlot)).toBe(true));
      view.rerender(single(1));
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual(expected.afterUnmountA.closedSlots));
      expect(bridge.globalCloseCalls).toBe(expected.afterUnmountA.globalCloseCalls);
      const beforeSiblingCommand = bridge.operationSessions.filter((row) => row.operation === expected.afterUnmountA.command && row.slot === expected.afterUnmountA.liveSlot).length;
      view.rerender(single(2));
      await waitFor(() => expect(bridge.operationSessions.filter((row) => row.operation === expected.afterUnmountA.command && row.slot === expected.afterUnmountA.liveSlot)).toHaveLength(beforeSiblingCommand + 1));
      view.unmount();
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual(expected.afterUnmountB.closedSlots));
      expect(bridge.globalCloseCalls).toBe(expected.afterUnmountB.globalCloseCalls);
      await runtime.close();
      expect(bridge.globalCloseCalls).toBe(flowBrowserRuntimeFixture.runtimeClose.globalCloseCalls);
      expect(runtime.terminalIsEmpty()).toBe(flowBrowserRuntimeFixture.runtimeClose.terminal);
    } finally {
      view.unmount();
      await runtime.close();
      createSpy.mockRestore();
      contextSpy.mockRestore();
      cleanup();
    }
  });

  it("retires a graph host unmounted before its open reply while its shared-runtime sibling remains live", async () => {
    const expected = flowBrowserRuntimeFixture.mountedHosts.lateUnmount;
    const bridge = new MockFlowBridge(new WebAssembly.Memory({ initial: 400 }), { heldOpenReplies: 1 });
    const runtime = await createFlowBrowserRuntime({ source: bridge.exports });
    const createSpy = vi.spyOn(flowSessionLoader, "createFlowSession").mockImplementation(async () => runtime.openSession());
    const contextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const scene = (revision: number) => ({
      nodes: [],
      edges: [],
      viewport: { x: 0, y: 0, zoom: 1 },
      hostSnapshotJson: JSON.stringify({ schema: "flow.host_snapshot", revision, widgets: [] }),
    });
    const host = (id: "A" | "B", revision: number) =>
      createElement(FlowGraphCanvasHost, {
        scene: scene(revision),
        controllerId: `flow.late.${id}`,
        surfaceId: `flow.late.${id}`,
        editable: true,
        keyboardPort: { current: null },
        onAction: vi.fn(),
      });
    const first = render(host("A", 0));
    let sibling: ReturnType<typeof render> | undefined;
    try {
      await waitFor(() => expect(bridge.openRequestIds).toEqual(["1"]));
      first.unmount();
      sibling = render(host("B", 0));
      await waitFor(() => expect(bridge.openRequestIds).toEqual(flowBrowserRuntimeFixture.openRequestIds));
      await waitFor(() => expect(bridge.operationSessions.some((row) => row.operation === expected.command && row.slot === expected.liveSlot)).toBe(true));
      bridge.releaseOpenReplies();
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual(expected.closedSlots));
      expect(bridge.globalCloseCalls).toBe(0);
      const beforeSiblingCommand = bridge.operationSessions.filter((row) => row.operation === expected.command && row.slot === expected.liveSlot).length;
      sibling.rerender(host("B", 1));
      await waitFor(() => expect(bridge.operationSessions.filter((row) => row.operation === expected.command && row.slot === expected.liveSlot)).toHaveLength(beforeSiblingCommand + 1));
      sibling.unmount();
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual([expected.heldOpenSlot, expected.liveSlot]));
      await runtime.close();
      expect(bridge.globalCloseCalls).toBe(flowBrowserRuntimeFixture.runtimeClose.globalCloseCalls);
      expect(runtime.terminalIsEmpty()).toBe(flowBrowserRuntimeFixture.runtimeClose.terminal);
    } finally {
      bridge.releaseOpenReplies();
      first.unmount();
      sibling?.unmount();
      await runtime.close();
      createSpy.mockRestore();
      contextSpy.mockRestore();
      cleanup();
    }
  });
  //#endregion 🎚️GraphParameterDispatch

  it("parses slider overlay state json for flow graph hosts", () => {
    const sliders = parseDagSliderOverlays(
      JSON.stringify({
        camera: { x: 0, y: 0, zoom: 1 },
        sliders: [
          {
            widgetId: "slider_2",
            label: "Radius",
            value: 2.2,
            min: 0,
            max: 10,
            step: 0.1,
            x: 100,
            y: 50,
            w: 120,
            h: 8,
          },
        ],
      }),
    );
    expect(sliders).toHaveLength(1);
    expect(sliders[0]?.widgetId).toBe("slider_2");
    expect(sliders[0]?.value).toBe(2.2);
  });

  it("renders graph slider overlays as track-only controls without a nested value readout", () => {
    const markup = renderToStaticMarkup(
      createElement(GraphSliderOverlays, {
        scopeId: "track-test",
        stateJson: JSON.stringify({
          camera: { x: 0, y: 0, zoom: 1 },
          sliders: [
            {
              widgetId: "slider_2",
              label: "Radius",
              value: 2.2,
              min: 0,
              max: 10,
              step: 0.1,
              x: 100,
              y: 50,
              w: 120,
              h: 8,
            },
          ],
        }),
        logicalW: 800,
        logicalH: 600,
        editable: true,
        onSliderChange: () => {},
      }),
    );
    expect(markup).toContain('data-slot="slider"');
    expect(markup).toContain('data-slot="slider-thumb"');
    expect(markup).not.toContain('data-slot="slider-value"');
    expect(markup).not.toContain('data-slot="slider-row"');
  });

  it("scales graph slider overlay chrome with canvas zoom so the knob matches other elements", () => {
    const markup = renderToStaticMarkup(
      createElement(GraphSliderOverlays, {
        scopeId: "zoom-test",
        stateJson: JSON.stringify({
          camera: { x: 0, y: 0, zoom: 2 },
          sliders: [
            {
              widgetId: "slider_2",
              label: "Radius",
              value: 2.2,
              min: 0,
              max: 10,
              step: 0.1,
              x: 100,
              y: 50,
              w: 120,
              h: 8,
            },
          ],
        }),
        logicalW: 800,
        logicalH: 600,
        editable: true,
        onSliderChange: () => {},
      }),
    );
    expect(markup).toContain('data-graph-slider-zoom="2"');
    expect(markup).toContain("translate(-50%, -50%) scale(2)");
    expect(markup).toContain("width:120px");
    expect(markup).toContain('data-slot="slider-thumb"');
    expect(markup).toContain("size-tiny");
    expect(markup).not.toMatch(/data-slot="slider-thumb"[^>]*size-small/);
  });

  it("renders canvas 2d host with infinite canvas session", () => {
    const markup = renderToStaticMarkup(
      createElement(Canvas2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "draw.play.canvas",
          controllerId: "draw-play",
          componentKind: "canvas-2d",
          canvas2d: {
            cameraX: 0,
            cameraY: 0,
            zoom: 1,
            layersJson: JSON.stringify([{ id: "layer-1", name: "Layer 1", x: 0, y: 0, width: 120, height: 80 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-canvas-2d-host");
  });

  it("renders canvas 2d host with draw gradient/blend/overlay/meta scene records", () => {
    const markup = renderToStaticMarkup(
      createElement(Canvas2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "draw.play.canvas",
          controllerId: "draw-play",
          componentKind: "canvas-2d",
          canvas2d: {
            cameraX: 0,
            cameraY: 0,
            zoom: 1,
            layersJson: JSON.stringify([
              { id: "meta:utility", role: "meta", utility: "selectDirect" },
              {
                id: "shape-1",
                transform: [1, 0, 0, 1, 0, 0],
                segments: [{ kind: "move", to: [0, 0] }, { kind: "line", to: [10, 0] }, { kind: "line", to: [10, 10] }, { kind: "close" }],
                fill: {
                  kind: "linearGradient",
                  x1: 0,
                  y1: 0,
                  x2: 10,
                  y2: 10,
                  stops: [
                    { offset: 0, color: [1, 0, 0, 1] },
                    { offset: 1, color: [0, 0, 1, 1] },
                  ],
                },
                stroke: { color: [0, 0, 0, 1], width: 1, cap: "round", join: "round" },
                opacity: 1,
                blendMode: "multiply",
                visible: true,
                fillRule: "evenodd",
              },
              {
                id: "overlay:sel:shape-1",
                role: "overlay",
                transform: [1, 0, 0, 1, 0, 0],
                segments: [{ kind: "move", to: [0, 0] }, { kind: "line", to: [10, 0] }, { kind: "close" }],
                fill: { kind: "solid", color: [0.98, 0.75, 0.14, 0.16] },
                stroke: { color: [0.98, 0.75, 0.14, 0.95], width: 2 },
                opacity: 1,
                blendMode: "normal",
                visible: true,
                fillRule: "evenodd",
              },
            ]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-canvas-2d-host");
  });

  it("renders puzzle 2d board host shell", () => {
    const markup = renderToStaticMarkup(
      createElement(Board2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle2d.play.composite.2d-overview",
          controllerId: "puzzle2d-play",
          componentKind: "board-2d",
          board2d: {
            snapshotJson: JSON.stringify({ nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } }),
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            glyphCatalogsJson: "{}",
            selectionJson: "[]",
            interactive: true,
            selectionMethod: "rectangle",
            gridSnapEnabled: false,
            gridFactor: 1,
            suggestionOffset: 0,
            brushWeightsJson: "{}",
            placementCompatibilityJson: "[]",
            lodMode: "automatic",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-board-2d-host");
  });

  it("projects the ids a time-travel draft references onto the puzzle 2d board host", () => {
    const board2d = {
      snapshotJson: JSON.stringify({ nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } }),
      cameraJson: '{"x":0,"y":0,"zoom":1}',
      glyphCatalogsJson: "{}",
      selectionJson: "[]",
      interactive: true,
      selectionMethod: "rectangle",
      gridSnapEnabled: false,
      gridFactor: 1,
      suggestionOffset: 0,
      brushWeightsJson: "{}",
      placementCompatibilityJson: "[]",
      lodMode: "automatic",
    };
    const mount = (highlightedIdsJson?: string) =>
      renderToStaticMarkup(createElement(Board2dHost, { node: { type: "componentScene", surfaceId: "puzzle2d.play.composite.2d-overview", controllerId: "puzzle2d-play", componentKind: "board-2d", board2d: { ...board2d, highlightedIdsJson } }, onAction: noopAction }));
    expect(mount('["mid","left"]')).toContain('data-board-highlighted-ids-json="[&quot;mid&quot;,&quot;left&quot;]"');
    expect(mount(undefined)).toContain('data-board-highlighted-ids-json="[]"');
  });

  it("forwards the ids a time-travel draft references to the puzzle 2d board session as the wgpu board sync does", () => {
    type Session = Parameters<typeof applyBoard2dHighlightedIds>[0];
    const forwarded: string[] = [];
    const session = { setHighlightedIdsJson: (json: string) => void forwarded.push(json) } as unknown as Session;
    applyBoard2dHighlightedIds(session, { highlightedIdsJson: '["mid","left"]' });
    applyBoard2dHighlightedIds(session, {});
    applyBoard2dHighlightedIds(null, { highlightedIdsJson: '["ghost"]' });
    applyBoard2dHighlightedIds({} as unknown as Session, { highlightedIdsJson: '["ghost"]' });
    applyBoard2dHighlightedIds({ setHighlightedIdsJson: () => { throw new Error("session not ready"); } } as unknown as Session, { highlightedIdsJson: '["ghost"]' });
    expect(forwarded).toEqual(['["mid","left"]', "[]"]);
  });

  it("uses the live puzzle 2d board camera for wheel persistence actions", () => {
    expect(board2dCameraActionArgs('{"x":345,"y":-123,"zoom":4.25}')).toEqual({
      camera: { x: 345, y: -123, zoom: 4.25 },
    });
  });

  it("coalesces puzzle 2d board events: drops transients and live nodeMove frames, takes the latest camera out for the view lane", () => {
    const rows = [
      { name: "preselect", payload: { ids: ["a"] } },
      { name: "camera", payload: { x: 1, y: 1, zoom: 1 } },
      { name: "nodeMove", payload: { id: "alpha", x: 10, y: 10 } },
      { name: "camera", payload: { x: 2, y: 2, zoom: 1.5 } },
      { name: "nodeMove", payload: { id: "beta", x: 5, y: 5 } },
    ];
    const { flushNow, eventsJson, camera } = coalesceBoard2dEvents(rows);
    expect(flushNow).toBe(false);
    expect(JSON.parse(eventsJson)).toEqual([]);
    expect(camera).toEqual({ x: 2, y: 2, zoom: 1.5 });
  });

  
  it("coalesces puzzle 2d board events: drops transients and live nodeMove frames, keeps the latest camera", () => {
    const rows = [
      { name: "preselect", payload: { ids: ["a"] } },
      { name: "camera", payload: { x: 1, y: 1, zoom: 1 } },
      { name: "nodeMove", payload: { id: "alpha", x: 10, y: 10 } },
      { name: "camera", payload: { x: 2, y: 2, zoom: 1.5 } },
      { name: "nodeMove", payload: { id: "beta", x: 5, y: 5 } },
    ];
    const { flushNow, eventsJson } = coalesceBoard2dEvents(rows);
    expect(flushNow).toBe(false);
    expect(JSON.parse(eventsJson)).toEqual([{ name: "camera", payload: { x: 2, y: 2, zoom: 1.5 } }]);
  });
it("keeps hover out of the board-events batch — it travels on the framework interactionHover lane instead", () => {
    const { eventsJson, flushNow } = coalesceBoard2dEvents([
      { name: "hover", payload: { id: "alpha", kind: null } },
      { name: "camera", payload: { x: 0, y: 0, zoom: 1 } },
    ]);
    expect(JSON.parse(eventsJson) as { name: string }[]).not.toContainEqual(expect.objectContaining({ name: "hover" }));
    expect(flushNow).toBe(false);
  });

  it("reads the LAST hover row of a batch — a string id hovers, an empty/absent id clears, no row leaves it alone", () => {
    expect(
      latestBoard2dHoverId([
        { name: "hover", payload: { id: "alpha" } },
        { name: "hover", payload: { id: "beta" } },
      ]),
    ).toBe("beta");
    expect(
      latestBoard2dHoverId([
        { name: "hover", payload: { id: "alpha" } },
        { name: "hover", payload: { id: null } },
      ]),
    ).toBeNull();
    expect(latestBoard2dHoverId([{ name: "camera", payload: {} }])).toBeUndefined();
  });

  it("classifies every board id into its vortex-domain granularity and publishes it on the interactionHover wire", () => {
    const fixture = JSON.stringify({ nodes: [{ id: "n1", handles: [{ id: "n1:h0" }] }], edges: [{ id: "e1" }] });
    const byId = board2dGranularityById(fixture);
    expect([byId.get("n1"), byId.get("n1:h0"), byId.get("e1"), byId.get("ghost")]).toEqual(["node", "handle", "edge", undefined]);
    expect(board2dHoverActionArgs("vortex", "handle", "n1:h0")).toEqual({ domainId: "vortex", channel: "pointer", targets: JSON.stringify([{ granularity: "handle", id: "n1:h0" }]) });
    expect(JSON.parse(board2dHoverActionArgs("vortex", "node", null).targets)).toEqual([]);
  });

  it("parses the handle-suggestions popup, scopes it to its own window, and renders hover-preview rows distinct from the commit", () => {
    const encoded = JSON.stringify({
      open: true,
      x: 12,
      y: 34,
      windowId: "w1",
      handleId: "n1:h0",
      hoveredIndex: 1,
      pending: false,
      candidates: [
        { index: 0, nodeLabel: "beam", handleLabel: "handle 0" },
        { index: 1, nodeLabel: "slab", handleLabel: "handle 1", icon: "square" },
      ],
    });
    const menu = parseBoard2dSuggestionMenu(encoded)!;
    expect(menu).toMatchObject({ open: true, x: 12, y: 34, windowId: "w1", handleId: "n1:h0", hoveredIndex: 1 });
    expect(board2dSuggestionMenuOwnsWindow(menu, "w1")).toBe(true);
    expect(board2dSuggestionMenuOwnsWindow(menu, "w2")).toBe(false);
    expect(parseBoard2dSuggestionMenu(undefined)).toBeNull();
    expect(parseBoard2dSuggestionMenu(JSON.stringify({ open: false }))).toBeNull();
    const rows = board2dSuggestionMenuItems(menu, { checkingPlacement: "checking", noPlacement: "none" });
    expect(rows.map((row) => row.action)).toEqual(["acceptSuggestion", "acceptSuggestion"]);
    expect(rows.map((row) => row.hoverAction)).toEqual(["hoverSuggestion", "hoverSuggestion"]);
    expect(rows.map((row) => row.checked)).toEqual([false, true]);
    expect(rows[1]?.args).toEqual({ index: 1, handleId: "n1:h0" });
    expect(rows[1]?.hoverArgs).toEqual({ index: 1, handleId: "n1:h0" });
  });

  it("refuses politely: a pending slot shows one disabled checking row, a resolved empty slot one disabled no-placement row", () => {
    const labels = { checkingPlacement: "checking", noPlacement: "none" };
    const pending = board2dSuggestionMenuItems(parseBoard2dSuggestionMenu(JSON.stringify({ open: true, x: 0, y: 0, hoveredIndex: 0, pending: true, candidates: [] }))!, labels);
    expect(pending).toEqual([{ id: "pending", label: "checking", disabled: true }]);
    const empty = board2dSuggestionMenuItems(parseBoard2dSuggestionMenu(JSON.stringify({ open: true, x: 0, y: 0, hoveredIndex: 0, pending: false, candidates: [] }))!, labels);
    expect(empty).toEqual([{ id: "empty", label: "none", disabled: true }]);
  });

  it("coalesces puzzle 2d board events: one drag is its tagged select plus its gesture record, frames dropped", () => {
    const select = { name: "select", payload: { ids: ["alpha"], exitHighlightIds: [], gestureId: "gesture-1" } };
    const record = { name: "gesture", payload: { gestureId: "gesture-1", kind: "drag", targets: ["alpha"], dx: 10, dy: 0, proximity: [] } };
    const { eventsJson, flushNow } = coalesceBoard2dEvents([select, { name: "nodeMove", payload: { id: "alpha", x: 10, y: 10 } }, record]);
    expect(flushNow).toBe(true);
    expect(JSON.parse(eventsJson)).toEqual([select, record]);
    expect(coalesceBoard2dEvents([select]).flushNow).toBe(false);
  });

  it("flushes puzzle 2d board events immediately for gesture/select/brushPlace/edge/delete/region rows, not for camera/nodeMove alone", () => {
    expect(coalesceBoard2dEvents([{ name: "camera", payload: { x: 0, y: 0, zoom: 1 } }]).flushNow).toBe(false);
    expect(coalesceBoard2dEvents([{ name: "nodeMove", payload: { id: "alpha", x: 0, y: 0 } }]).flushNow).toBe(false);
    for (const name of ["gesture", "select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete", "regionCreate", "regionResize"]) {
      expect(coalesceBoard2dEvents([{ name, payload: {} }]).flushNow).toBe(true);
    }
  });

  it("collects live mirror mutations: coalesces nodeMove to the latest per id, ignores unrelated rows", () => {
    const mutations = collectPuzzle2dLiveMirrorMutations([
      { name: "camera", payload: { x: 1, y: 1, zoom: 1 } },
      { name: "nodeMove", payload: { id: "alpha", x: 1, y: 1 } },
      { name: "brushPreview", payload: {} },
      { name: "nodeMove", payload: { id: "alpha", x: 9, y: 9 } },
      { name: "nodeMove", payload: { id: "beta", x: 2, y: 2 } },
    ]);
    expect(mutations.positions).toEqual([
      { id: "alpha", x: 9, y: 9 },
      { id: "beta", x: 2, y: 2 },
    ]);
    expect(mutations.selectionIds).toBeNull();
    expect(mutations.preselect).toBeNull();
    expect(mutations.clearPreselect).toBe(false);
  });

  it("board 2d gumball: a rotate record flushes at once and its live preview frames never reach the guest", () => {
    // 🔄️ `transformPreview` is the peer-pane mirror's food ONLY — forwarding it would spend one of the
    // store's 64 applied edits per drag frame, when a whole rotate gesture must be a single edit.
    const { flushNow, eventsJson } = coalesceBoard2dEvents([
      { name: "transformPreview", payload: { moves: [{ id: "alpha", x: 1, y: 1 }] } },
      { name: "gesture", payload: { gestureId: "gesture-2", kind: "rotate", targets: ["alpha"], pivotX: 0, pivotY: 0, angle: 0.5, proximity: [] } },
    ]);
    expect(flushNow).toBe(true);
    const events = JSON.parse(eventsJson) as { name: string }[];
    expect(events.map((event) => event.name)).toEqual(["gesture"]);
    expect(coalesceBoard2dEvents([{ name: "transformPreview", payload: { moves: [] } }]).flushNow).toBe(false);
  });

  it("board 2d gumball: transformPreview frames mirror into sibling panes exactly like a drag's final moves", () => {
    const mutations = collectPuzzle2dLiveMirrorMutations([
      { name: "transformPreview", payload: { moves: [{ id: "alpha", x: 3, y: 4 }] } },
      {
        name: "transformPreview",
        payload: {
          moves: [
            { id: "alpha", x: 0, y: 5 },
            { id: "beta", x: -5, y: 0 },
          ],
        },
      },
    ]);
    expect(mutations.positions).toEqual([
      { id: "alpha", x: 0, y: 5 },
      { id: "beta", x: -5, y: 0 },
    ]);
    expect(mutations.selectionIds).toBeNull();
  });

  it("board 2d gumball: transformFlags default to move+rotate and never disarm on malformed input", () => {
    expect(parseBoard2dTransformFlags(JSON.stringify({ move: false, rotate: true }))).toEqual({ move: false, rotate: true });
    expect(parseBoard2dTransformFlags(JSON.stringify({ rotate: false }))).toEqual({ move: true, rotate: false });
    for (const encoded of [undefined, null, "", "not json", "{}", JSON.stringify({ move: "yes" })]) {
      expect(parseBoard2dTransformFlags(encoded)).toEqual({ move: true, rotate: true });
    }
  });

  it("board 2d vitals: the status row names the fixture verdict, its size, the refusal and the pending flush", () => {
    expect(JSON.parse(board2dStatusJson({ snapshotParsed: true, snapshotChars: 128, refusalReason: "", pendingEvents: 0, guestRevision: 3 }))).toEqual({
      snapshotParsed: true,
      snapshotChars: 128,
      refusalReason: "",
      pendingEvents: 0,
      guestRevision: 3,
    });
    const refused = JSON.parse(board2dStatusJson({ snapshotParsed: false, snapshotChars: 9, refusalReason: "engine refused the fixture", pendingEvents: 2, guestRevision: 1 })) as { snapshotParsed: boolean; refusalReason: string };
    expect(refused.snapshotParsed).toBe(false);
    expect(refused.refusalReason).toBe("engine refused the fixture");
  });

  it("collects live mirror mutations: preselect sets the live highlight, select/preselectCancel commit selection and clear it", () => {
    expect(collectPuzzle2dLiveMirrorMutations([{ name: "preselect", payload: { ids: ["a", "b"], removedIds: ["c"] } }])).toMatchObject({
      preselect: { ids: ["a", "b"], removedIds: ["c"] },
      clearPreselect: false,
      selectionIds: null,
    });
    expect(collectPuzzle2dLiveMirrorMutations([{ name: "select", payload: { ids: ["a"] } }])).toMatchObject({
      selectionIds: ["a"],
      preselect: null,
      clearPreselect: true,
    });
    expect(collectPuzzle2dLiveMirrorMutations([{ name: "preselectCancel", payload: { ids: ["a", "b"] } }])).toMatchObject({
      selectionIds: ["a", "b"],
      preselect: null,
      clearPreselect: true,
    });
  });

  it("peer registry: registers/unregisters, excludes own surfaceId and other controllerIds", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    const peerA = { session: {} as never, onPeerGestureEnded: () => {} };
    const peerB = { session: {} as never, onPeerGestureEnded: () => {} };
    const peerOther = { session: {} as never, onPeerGestureEnded: () => {} };
    registerBoard2dPeer(scope, "puzzle2d-play", "pane.a", peerA);
    registerBoard2dPeer(scope, "puzzle2d-play", "pane.b", peerB);
    registerBoard2dPeer(scope, "other-controller", "pane.a", peerOther);

    expect(board2dPeers(scope, "puzzle2d-play", "pane.a")).toEqual([peerB]);
    expect(board2dPeers(scope, "puzzle2d-play", "pane.b")).toEqual([peerA]);
    expect(board2dPeers(scope, "other-controller", "pane.z")).toEqual([peerOther]);

    unregisterBoard2dPeer(scope, "puzzle2d-play", "pane.b", scope.peers.get("puzzle2d-play")?.get("pane.b") ?? null);
    expect(board2dPeers(scope, "puzzle2d-play", "pane.a")).toEqual([]);

    unregisterBoard2dPeer(scope, "puzzle2d-play", "pane.a", scope.peers.get("puzzle2d-play")?.get("pane.a") ?? null);
    unregisterBoard2dPeer(scope, "other-controller", "pane.a", scope.peers.get("other-controller")?.get("pane.a") ?? null);
  });

  it("peer gesture ownership: begin/end tracks the owning surfaceId; a pane never defers against its own gesture", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    registerBoard2dPeer(scope, "puzzle2d-play", "pane.a", { session: boardTestSession(), onPeerGestureEnded: () => {} });
    expect(puzzle2dPeerOwnsGesture(scope, "puzzle2d-play", "pane.a")).toBe(false);
    beginPuzzle2dPeerGesture(scope, "puzzle2d-play", "pane.a", scope.peers.get("puzzle2d-play")?.get("pane.a") ?? null);
    expect(puzzle2dPeerOwnsGesture(scope, "puzzle2d-play", "pane.a")).toBe(false);
    expect(puzzle2dPeerOwnsGesture(scope, "puzzle2d-play", "pane.b")).toBe(true);
    endPuzzle2dPeerGesture(scope, "puzzle2d-play", "pane.a", scope.peers.get("puzzle2d-play")?.get("pane.a") ?? null);
    expect(puzzle2dPeerOwnsGesture(scope, "puzzle2d-play", "pane.b")).toBe(false);
  });

  it("pushes live mirror mutations into peer sessions, skipping the source pane", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    const calls: { pane: string; method: string; arg: string }[] = [];
    const makePeer = (pane: string) => ({
      session: {
        setNodePositionsJson: (json: string) => calls.push({ pane, method: "setNodePositionsJson", arg: json }),
        setSelectionIdsJsonSilent: (json: string) => calls.push({ pane, method: "setSelectionIdsJsonSilent", arg: json }),
        setPreselectStateJsonSilent: (json: string) => calls.push({ pane, method: "setPreselectStateJsonSilent", arg: json }),
      } as never,
      onPeerGestureEnded: () => {},
    });
    registerBoard2dPeer(scope, "mirror-test", "pane.source", makePeer("pane.source"));
    registerBoard2dPeer(scope, "mirror-test", "pane.sibling", makePeer("pane.sibling"));

    pushPuzzle2dLiveMirrorMutations(scope, "mirror-test", "pane.source", {
      positions: [{ id: "alpha", x: 1, y: 2 }],
      selectionIds: ["alpha"],
      preselect: null,
      clearPreselect: false,
    });

    expect(calls).toEqual([
      { pane: "pane.sibling", method: "setNodePositionsJson", arg: JSON.stringify([{ id: "alpha", x: 1, y: 2 }]) },
      { pane: "pane.sibling", method: "setSelectionIdsJsonSilent", arg: JSON.stringify(["alpha"]) },
    ]);

    unregisterBoard2dPeer(scope, "mirror-test", "pane.source", scope.peers.get("mirror-test")?.get("pane.source") ?? null);
    unregisterBoard2dPeer(scope, "mirror-test", "pane.sibling", scope.peers.get("mirror-test")?.get("pane.sibling") ?? null);
  });

  it("notifies peers when a gesture ends, skipping the source pane, passing whether it flushed", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    const ended: { pane: string; flushed: boolean }[] = [];
    registerBoard2dPeer(scope, "notify-test", "pane.source", { session: {} as never, onPeerGestureEnded: (flushed) => ended.push({ pane: "pane.source", flushed }) });
    registerBoard2dPeer(scope, "notify-test", "pane.sibling", { session: {} as never, onPeerGestureEnded: (flushed) => ended.push({ pane: "pane.sibling", flushed }) });

    notifyPuzzle2dPeersGestureEnded(scope, "notify-test", "pane.source", true);
    expect(ended).toEqual([{ pane: "pane.sibling", flushed: true }]);

    unregisterBoard2dPeer(scope, "notify-test", "pane.source", scope.peers.get("notify-test")?.get("pane.source") ?? null);
    unregisterBoard2dPeer(scope, "notify-test", "pane.sibling", scope.peers.get("notify-test")?.get("pane.sibling") ?? null);
  });

  it("maps context menu specs onto UI items with icons, colors, hover, and select handlers", () => {
    const dispatch = vi.fn();
    const items = mapContextMenuSpecs(
      [
        { id: "hide", label: "Hide", icon: "eye-off", color: "#ff0000", action: "setFlag", args: { flag: "hidden" }, hoverAction: "hoverFlag", hoverArgs: { index: 1 } },
        { id: "sep", separator: true },
        { id: "delete", label: "Delete", icon: "trash", destructive: true, action: "deleteSelection" },
      ],
      dispatch,
    );
    expect(items[0]).toMatchObject({ id: "hide", label: "Hide", icon: "eye-off", color: "#ff0000" });
    items[0]?.onSelect?.(new Event("select"));
    items[0]?.onHover?.();
    expect(dispatch).toHaveBeenCalledWith("setFlag", { flag: "hidden" });
    expect(dispatch).toHaveBeenCalledWith("hoverFlag", { index: 1 });
    expect(items[1]).toMatchObject({ id: "sep", separator: true });
    expect(items[2]).toMatchObject({ id: "delete", destructive: true, icon: "trash" });
  });

  it("keeps exportSnapshot and importSnapshot menu rows on the same leaf onSelect bind", () => {
    const dispatch = vi.fn();
    const items = mapContextMenuSpecs(
      [
        { id: "shell-menu.action.exportSnapshot", label: "Export", action: "exportSnapshot" },
        { id: "shell-menu.action.importSnapshot", label: "Import", action: "importSnapshot" },
        { id: "shell-menu.action.openImportSnapshot", label: "Import…", action: "openImportSnapshot" },
      ],
      dispatch,
    );
    expect(items.map((item) => ({ id: item.id, action: item.action, hasSelect: typeof item.onSelect === "function", children: item.children?.length ?? 0 }))).toEqual([
      { id: "shell-menu.action.exportSnapshot", action: "exportSnapshot", hasSelect: true, children: 0 },
      { id: "shell-menu.action.importSnapshot", action: "importSnapshot", hasSelect: true, children: 0 },
      { id: "shell-menu.action.openImportSnapshot", action: "openImportSnapshot", hasSelect: true, children: 0 },
    ]);
    items[0]?.onSelect?.(new Event("select"));
    items[1]?.onSelect?.(new Event("select"));
    expect(dispatch).toHaveBeenCalledWith("exportSnapshot", {});
    expect(dispatch).toHaveBeenCalledWith("importSnapshot", {});
  });

  it("keeps context-menu leaf rows pointer-hittable under dimmed window chrome", () => {
    expect(contextMenuItemClassName({})).toContain("pointer-events-auto");
  });

  it("maps suggestion-style specs without a color swatch field", () => {
    const dispatch = vi.fn();
    const items = mapContextMenuSpecs(
      [
        {
          id: "suggestion-0",
          label: "Capsule · vortex 0",
          icon: "box",
          checked: true,
          action: "acceptSuggestion",
          args: { index: 0, fullId: "obj:v0" },
          hoverAction: "hoverSuggestion",
          hoverArgs: { index: 0 },
        },
      ],
      dispatch,
    );
    expect(items[0]).toMatchObject({ id: "suggestion-0", icon: "box", checked: true });
    expect(items[0]).not.toHaveProperty("color", expect.anything());
    expect(items[0]?.color).toBeUndefined();
  });

  it("enriches context menu shortcuts from app keybindings", () => {
    const keysByActionId = buildKeysByActionId([
      { action: { action: "deleteSelection" }, keys: "delete,backspace" },
      { action: { action: "duplicateSelection" }, keys: "mod+d" },
    ]);
    const items = mapContextMenuSpecs(
      [
        { id: "duplicate", label: "Duplicate", action: "duplicateSelection" },
        { id: "delete", label: "Delete", action: "deleteSelection" },
        { id: "custom", label: "Custom", action: "customAction", shortcut: "F2" },
      ],
      vi.fn(),
      keysByActionId,
    );
    expect(items[0]?.shortcut).toMatch(/D$/);
    expect(items[1]?.shortcut).toBe("⌦️");
    expect(items[2]?.shortcut).toBe("F2");
  });

  it("formats keybinding chords for menu shortcut labels", () => {
    expect(formatKeybindingShortcut("backspace")).toBe("⌫️");
    expect(formatKeybindingShortcut("delete,backspace")).toBe("⌦️");
    expect(formatKeybindingShortcut("mod+d")).toMatch(/D$/);
  });

  it("numbers suggestion menu rows with digit shortcuts for the first nine candidates", () => {
    const items = suggestionMenuItems(
      {
        pending: false,
        candidates: [
          { index: 2, objectLabel: "Capsule", vortexLabel: "port-a", icon: "box" },
          { index: 5, objectLabel: "Box", vortexLabel: "port-b", icon: "box" },
        ],
        vortexFullId: "obj:v0",
      },
      2,
      { checkingPlacement: uiDataLabel("Checking placement…"), noPlacement: uiDataLabel("No placement") },
    );
    expect(items[0]).toMatchObject({ checked: true });
    expect(items[1]).toMatchObject({ checked: false });
    expect(items[0]?.shortcut).toBeUndefined();
  });

  it("resolves taxonomy group rows from the chrome ribbon-parent bundle in both locales", async () => {
    // 🗂️ The guest emits `menu.group.<category>` with `label: undefined` by contract (the host owns the
    // chrome taxonomy vocabulary), and the React shell used to render the raw id — `menu.group.history`,
    // `menu.group.selection`, … — where the wgpu target resolved a real label through
    // `ribbon_parent_label`. The label must come from the SAME `ui.ribbon.parent.*` bundle the ribbon
    // reads, so EN and DE both follow the active locale with no second string table.
    const specs = [
      { id: "menu.group.history", children: [{ id: "undo", label: "Undo", action: "undo" }] },
      { id: "menu.group.selection", children: [{ id: "duplicate", label: "Duplicate", action: "duplicateSelection" }] },
      { id: "menu.group.more", children: [{ id: "fit", label: "Fit", action: "fitWorld" }] },
    ];
    await uiI18n.changeLanguage("en");
    const english = mapContextMenuSpecs(specs, () => {});
    expect(english.map((item) => item.label)).toEqual(["History", "Selection", "More"]);
    expect(english.map((item) => item.icon)).toEqual(["folder", "folder", "folder"]);
    expect(english[0]?.children?.[0]).toMatchObject({ id: "undo", label: "Undo" });

    await uiI18n.changeLanguage("de");
    expect(mapContextMenuSpecs(specs, () => {}).map((item) => item.label)).toEqual(["Verlauf", "Auswahl", "Mehr"]);
    await uiI18n.changeLanguage("en");

    // 🗂️ Only group rows are resolved: an ordinary label-less row (a bare separator) keeps none, and an
    // unknown category resolves to nothing rather than inventing chrome vocabulary.
    const untouched = mapContextMenuSpecs(
      [
        { id: "sep", separator: true },
        { id: "menu.group.not-a-category", children: [{ id: "x", label: "X" }] },
      ],
      () => {},
    );
    expect(untouched[0]?.label).toBeUndefined();
    expect(untouched[1]?.label).toBeUndefined();
  });

  it("enriches context menu shortcuts from app keybindings via mapContextMenuSpecs", () => {
    const keys = new Map([["deleteSelection", "delete,backspace"]]);
    const items = mapContextMenuSpecs([{ id: "delete-selection", label: "Delete Selection (8 nodes and 13 edges)", action: "deleteSelection", destructive: true }], () => {}, keys);
    expect(items[0]?.shortcut).toBeTruthy();
    expect(items[0]?.label).toContain("8 nodes and 13 edges");
  });

  it("maps tiled-map interaction snapshots onto the current wasm sync ABI", () => {
    expect(resolveMapInteractionSync('{"positions":["position-1"],"routes":[]}', '{"kind":"position","id":"position-2"}')).toEqual({
      granularity: "position",
      selectedIdsJson: '["position-1"]',
      hoveredId: "position-2",
    });
    expect(resolveMapInteractionSync('{"positions":[],"routes":["route-1"]}', '{"kind":"position","id":"position-2"}')).toEqual({
      granularity: "route",
      selectedIdsJson: '["route-1"]',
    });
  });

  it("parses a catalogue drag payload and builds a drop-preview JSON", () => {
    const encoded = JSON.stringify({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24 });
    const payload = parsePuzzle2dCatalogueDragPayload(encoded);
    expect(payload).toEqual({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24, width: undefined, height: undefined, iconKind: undefined });
    expect(payload).not.toBeNull();
    expect(JSON.parse(puzzle2dDropPreviewJson(payload!, 100, 200))).toMatchObject({ nodeKind: "seed", x: 100, y: 200, shape: "circle", radius: 24 });
  });

  it("rejects a catalogue drag payload without a kindId", () => {
    expect(parsePuzzle2dCatalogueDragPayload(JSON.stringify({ catalogSlice: "nodes" }))).toBeNull();
    expect(parsePuzzle2dCatalogueDragPayload(null)).toBeNull();
  });

  it("parses a puzzle 3d catalogue drag payload and snaps drop origins to the grid", () => {
    const encoded = JSON.stringify({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule" });
    expect(parsePuzzle3dCatalogueDragPayload(encoded)).toEqual({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule" });
    expect(snapWorldPointToGrid([1.2, 2.7, 0.0], true, 1)).toEqual([1, 3, 0]);
    expect(snapWorldPointToGrid([1.2, 2.7, 0.0], false, 1)).toEqual([1.2, 2.7, 0]);
    expect(parsePuzzle3dCatalogueDragPayload(JSON.stringify({ meshUrl: "puzzle3d://capsule" }))).toBeNull();
  });

  it("raycasts the Z=0 ground under orthographic top and perspective cameras", async () => {
    const { sceneHostPort } = await import("@semio-tech/ui-react");
    const { OrthographicCamera, PerspectiveCamera } = sceneHostPort.three;
    const rect = { left: 0, top: 0, width: 200, height: 100, right: 200, bottom: 100 } as DOMRect;

    const ortho = new OrthographicCamera(-100, 100, 50, -50, 0.1, 1000);
    ortho.position.set(0, 0, 100);
    ortho.up.set(0, 1, 0);
    ortho.lookAt(0, 0, 0);
    ortho.updateMatrixWorld(true);
    ortho.updateProjectionMatrix();

    const orthoCenter = raycastGroundPoint(100, 50, rect, ortho);
    expect(orthoCenter).not.toBeNull();
    expect(orthoCenter![0]).toBeCloseTo(0, 5);
    expect(orthoCenter![1]).toBeCloseTo(0, 5);
    expect(orthoCenter![2]).toBeCloseTo(0, 5);

    const orthoRight = raycastGroundPoint(150, 50, rect, ortho);
    expect(orthoRight).not.toBeNull();
    expect(orthoRight![0]).toBeCloseTo(50, 5);
    expect(orthoRight![1]).toBeCloseTo(0, 5);
    expect(orthoRight![2]).toBeCloseTo(0, 5);

    const orthoUp = raycastGroundPoint(100, 25, rect, ortho);
    expect(orthoUp).not.toBeNull();
    expect(orthoUp![0]).toBeCloseTo(0, 5);
    expect(orthoUp![1]).toBeCloseTo(25, 5);
    expect(orthoUp![2]).toBeCloseTo(0, 5);

    const perspective = new PerspectiveCamera(50, 2, 0.1, 1000);
    perspective.position.set(0, 0, 10);
    perspective.up.set(0, 1, 0);
    perspective.lookAt(0, 0, 0);
    perspective.updateMatrixWorld(true);
    perspective.updateProjectionMatrix();

    const perspectiveCenter = raycastGroundPoint(100, 50, rect, perspective);
    expect(perspectiveCenter).not.toBeNull();
    expect(perspectiveCenter![0]).toBeCloseTo(0, 4);
    expect(perspectiveCenter![1]).toBeCloseTo(0, 4);
    expect(perspectiveCenter![2]).toBeCloseTo(0, 4);
  });

  it("a host click pick hits the nearest projected instance AABB and misses empty space", () => {
    const table = {
      id: "table",
      corners: [
        [400, 300],
        [600, 300],
        [400, 380],
        [600, 380],
      ] as const,
      depth: 10,
    };
    const far = {
      id: "far",
      corners: [
        [400, 300],
        [600, 300],
        [400, 380],
        [600, 380],
      ] as const,
      depth: 40,
    };
    expect(world3dProjectedAabbContainsClick({ x: 500, y: 340 }, table.corners)).toBe(true);
    expect(world3dProjectedAabbContainsClick({ x: 10, y: 10 }, table.corners)).toBe(false);
    expect(resolveClickInstanceIdFromProjected({ x: 500, y: 340 }, [table, far])).toBe("table");
    expect(resolveClickInstanceIdFromProjected({ x: 10, y: 10 }, [table, far])).toBeNull();
    expect(world3dMarqueePointerCaptureArmed(0)).toBe(false);
    expect(world3dMarqueePointerCaptureArmed(4)).toBe(false);
    expect(world3dMarqueePointerCaptureArmed(5)).toBe(true);
    expect(world3dInstancePickUsesInteractionDomain(undefined)).toBe(false);
    expect(world3dInstancePickUsesInteractionDomain({})).toBe(false);
    expect(world3dInstancePickUsesInteractionDomain({ interactionId: "port@0" })).toBe(true);
  });

  it("paints the pane's own raycast hover before the guest echoes it, and the guest's when the pointer is elsewhere", () => {
    // 🎯️ Inside the pane the raycast leads: over an object it paints at once, over nothing it clears at
    // once — both before the `interactionHover` round trip settles and regardless of what the guest's
    // lane still says.
    expect(worldHoverPaintIdV1("bim-1", null)).toBe("bim-1");
    expect(worldHoverPaintIdV1("bim-1", "bim-1")).toBe("bim-1");
    expect(worldHoverPaintIdV1("bim-2", "bim-1")).toBe("bim-2");
    expect(worldHoverPaintIdV1(null, "bim-1")).toBeNull();
    // 🧭️ With no local claim (the pointer left the pane) the guest's hover — an outliner row, a remote
    // presence — is what the pane paints.
    expect(worldHoverPaintIdV1(undefined, "bim-1")).toBe("bim-1");
    expect(worldHoverPaintIdV1(undefined, null)).toBeNull();
  });

  it("arms the suggestions gesture from a host vortex hover without a guest InteractionView", () => {
    expect(world3dSuggestionsGestureArmed(true, "table@in")).toBe(true);
    expect(world3dSuggestionsGestureArmed(true, null)).toBe(false);
    expect(world3dSuggestionsGestureArmed(false, "table@in")).toBe(false);
    expect(world3dRetainLocalVortexHover("seed-left-001:v0", null)).toBe("seed-left-001:v0");
    expect(world3dRetainLocalVortexHover("seed-left-001:v0", undefined)).toBe("seed-left-001:v0");
    expect(world3dRetainLocalVortexHover("seed-left-001:v0", "seed-left-001:v1")).toBe("seed-left-001:v1");
    expect(world3dRetainLocalVortexHover(null, null)).toBeNull();
    expect(world3dSuggestionsGestureConsumesContextMenu(true)).toBe(true);
    expect(world3dSuggestionsGestureConsumesContextMenu(false)).toBe(false);
    expect(world3dSuggestionsGestureArmed(true, world3dRetainLocalVortexHover("seed-left-001:v3", null))).toBe(true);
    expect(world3dSuggestionsRightDownRoutesOnWindowCapture(2, true)).toBe(true);
    expect(world3dSuggestionsRightDownRoutesOnWindowCapture(2, false)).toBe(false);
    expect(world3dSuggestionsRightDownRoutesOnWindowCapture(0, true)).toBe(false);
  });

  it("frames world instances onto the table centroid without a guest selection", () => {
    const camera = { position: [40, -40, 30] as [number, number, number], target: [0, 0, 0] as [number, number, number], zoom: 1, projection: "perspective" as const, fov: 45, explicitProjection: false, projectionFrame: "content" as const };
    const framed = world3dFrameCameraFromInstances([{ id: "seed-left-001", x: 2, y: -1, z: 0.4 }], camera);
    expect(framed.target[0]).toBeCloseTo(2);
    expect(framed.target[1]).toBeCloseTo(-1);
    expect(framed.target[2]).toBeCloseTo(0.4);
    const far = Math.hypot(camera.position[0] - 2, camera.position[1] + 1, camera.position[2] - 0.4);
    const near = Math.hypot(framed.position[0] - 2, framed.position[1] + 1, framed.position[2] - 0.4);
    expect(near).toBeLessThan(far * 0.2);
    expect(near).toBeGreaterThan(1);
  });

  it("frames a table-sized world AABB without leaving the current look direction", () => {
    const camera = { position: [40, -40, 30] as [number, number, number], target: [0, 0, 0] as [number, number, number], zoom: 1, projection: "perspective" as const, fov: 45, explicitProjection: false, projectionFrame: "content" as const };
    const framed = world3dFrameCameraFromBounds([0, 0, 0.4], 1.2, camera);
    expect(framed.target[0]).toBeCloseTo(0);
    expect(framed.target[1]).toBeCloseTo(0);
    expect(framed.target[2]).toBeCloseTo(0.4);
    const distance = Math.hypot(framed.position[0], framed.position[1], framed.position[2] - 0.4);
    expect(distance).toBeGreaterThan(2);
    expect(distance).toBeLessThan(8);
    const farDir = [40, -40, 30];
    const nearDir = [framed.position[0], framed.position[1], framed.position[2] - 0.4];
    const farLen = Math.hypot(...farDir);
    const nearLen = Math.hypot(...nearDir);
    expect(nearDir[0] / nearLen).toBeCloseTo(farDir[0] / farLen, 1);
    expect(nearDir[1] / nearLen).toBeCloseTo(farDir[1] / farLen, 1);
  });

  it("treats a non-string media-export encoding as a one-shot download, not a segmented marker", () => {
    expect(mediaExportEncodingText("utf-8")).toBe("utf-8");
    expect(mediaExportEncodingText({ some: "utf-8" })).toBeUndefined();
    expect(mediaExportEncodingText(undefined)).toBeUndefined();
  });

  it("appends a download anchor and keeps the object URL alive past the click turn", () => {
    const revoked: string[] = [];
    const create = URL.createObjectURL;
    const revoke = URL.revokeObjectURL;
    URL.createObjectURL = () => "blob:export-snapshot";
    URL.revokeObjectURL = (url) => {
      revoked.push(String(url));
    };
    try {
      downloadMediaExport("puzzle-3d.json", "application/json", "{}", "utf-8");
      expect(DOWNLOAD_MEDIA_EXPORT_REVOKE_MS).toBeGreaterThanOrEqual(1000);
      expect(document.querySelector("a[download='puzzle-3d.json']")).not.toBeNull();
      expect(revoked).toEqual([]);
    } finally {
      URL.createObjectURL = create;
      URL.revokeObjectURL = revoke;
    }
  });

  it("mounts exact natural codecs and opens bytes into an isolated owner", async () => {
    const definition = (row: (typeof naturalFileLifecycleFixture.formats)[number]): AppDefinition => {
      const args = [
        { id: "formatKind", default: row.formatKind },
        { id: "extension", default: row.extension },
        { id: "mediaType", default: row.mediaType },
        { id: "binary", default: row.binary },
      ];
      return {
        id: row.appId,
        actions: [],
        io: { exportFormats: [row.formatKind], importFormats: [row.formatKind] },
        windowKinds: [{ actions: [{ id: SAVE_ARTIFACT_FILE_ACTION_ID, args }, { id: OPEN_ARTIFACT_FILE_ACTION_ID, args }] }],
      } as unknown as AppDefinition;
    };
    for (const row of naturalFileLifecycleFixture.formats) {
      const format = { formatKind: row.formatKind, extension: row.extension, mediaType: row.mediaType, binary: row.binary };
      expect(naturalFileFormatV1(definition(row))).toEqual(format);
      expect(naturalFileNameV1(row.appId, row.extension, new Date("2026-10-03T12:34:56.000Z"))).toMatch(new RegExp(`20261003T123456Z\\${row.extension}$`, "u"));
      expect(naturalMediaDescriptorMatchesV1({ portId: "artifact:native", kindId: row.formatKind, wire: { kind: "binary", format_kind: row.formatKind } }, format)).toBe(true);
      expect(naturalMediaDescriptorMatchesV1({ portId: "artifact:native", kindId: row.formatKind, wire: { Binary: { format_kind: row.formatKind } } }, format)).toBe(false);
    }
    const unsupported = { id: naturalFileLifecycleFixture.unsupported[0]!.appId, actions: [], io: { exportFormats: [], importFormats: [] }, windowKinds: [{ actions: [] }] } as unknown as AppDefinition;
    expect(naturalFileFormatV1(unsupported)).toBeNull();
    const matched = definition(naturalFileLifecycleFixture.formats[0]!);
    const mismatched = { ...matched, io: { ...matched.io, importFormats: [] } } as AppDefinition;
    expect(naturalFileFormatV1(mismatched)).toBeNull();

    const currentHistory = ["edit"];
    const openedHistory: string[] = [];
    const retired: number[] = [];
    const signal = new AbortController();
    const opened = await openNaturalFileOwnerV1({
      signal: signal.signal,
      create: async () => naturalFileLifecycleFixture.lifecycle.openedInstanceId,
      importBytes: async (instanceId) => {
        expect(instanceId).toBe(naturalFileLifecycleFixture.lifecycle.openedInstanceId);
        openedHistory.push("set-snapshot");
      },
      retire: async (instanceId) => {
        retired.push(instanceId);
      },
    });
    expect({
      saveInstanceId: naturalFileLifecycleFixture.lifecycle.currentInstanceId,
      preservedInstanceId: naturalFileLifecycleFixture.lifecycle.currentInstanceId,
      openedInstanceId: opened,
      openedHistoryEntries: openedHistory.length,
    }).toEqual(naturalFileLifecycleFixture.lifecycle.expected);
    expect(currentHistory).toEqual(["edit"]);
    expect(retired).toEqual([]);

    const cancelled = new AbortController();
    const cancelledRetirements: number[] = [];
    await expect(openNaturalFileOwnerV1({
      signal: cancelled.signal,
      create: async () => naturalFileLifecycleFixture.lifecycle.openedInstanceId,
      importBytes: async () => cancelled.abort(),
      retire: async (instanceId) => {
        cancelledRetirements.push(instanceId);
      },
    })).rejects.toBeDefined();
    expect(cancelledRetirements).toEqual([naturalFileLifecycleFixture.lifecycle.openedInstanceId]);

    const read = naturalFileLifecycleFixture.browserRead;
    const source = new Blob([Uint8Array.from(read.octets)]);
    const progress: number[] = [];
    const loaded = await readBlobBytesBoundedV1(source, {
      signal: new AbortController().signal,
      maximumBytes: read.maximumBytes,
      chunkBytes: read.chunkBytes,
      progress: (completed) => progress.push(completed),
    });
    expect([...loaded]).toEqual(read.octets);
    expect(progress).toEqual(read.expectedProgress);
    await expect(readBlobBytesBoundedV1(source, {
      signal: new AbortController().signal,
      maximumBytes: read.octets.length - 1,
      chunkBytes: read.chunkBytes,
      progress: () => {},
    })).rejects.toThrow("natural-file.read-limit");

    const interrupted = new AbortController();
    await expect(readBlobBytesBoundedV1(source, {
      signal: interrupted.signal,
      maximumBytes: read.maximumBytes,
      chunkBytes: read.chunkBytes,
      progress: (completed) => {
        if (completed === read.cancelAfterBytes) interrupted.abort("fixture cancellation");
      },
    })).rejects.toBe("fixture cancellation");
    const retried = await readBlobBytesBoundedV1(source, {
      signal: new AbortController().signal,
      maximumBytes: read.maximumBytes,
      chunkBytes: read.chunkBytes,
      progress: () => {},
    });
    expect([...retried]).toEqual(read.octets);

    const failedBlob = { size: 1, slice: () => ({ arrayBuffer: async () => { throw new Error("fixture read fault"); } }) } as unknown as Blob;
    await expect(readBlobBytesBoundedV1(failedBlob, {
      signal: new AbortController().signal,
      maximumBytes: read.maximumBytes,
      chunkBytes: read.chunkBytes,
      progress: () => {},
    })).rejects.toThrow("natural-file.read-failed");

    const picker = document.createElement("input");
    const createElement = vi.spyOn(document, "createElement").mockReturnValueOnce(picker);
    const click = vi.spyOn(picker, "click").mockImplementation(() => {});
    const remove = vi.spyOn(picker, "remove");
    const selection = requestFileSelectionV1(".csv,text/csv");
    picker.dispatchEvent(new Event("cancel"));
    expect(await selection).toEqual([]);
    expect(picker.onchange).toBeNull();
    expect(picker.oncancel).toBeNull();
    expect(click).toHaveBeenCalledOnce();
    expect(remove).toHaveBeenCalledOnce();
    createElement.mockRestore();
    click.mockRestore();
    remove.mockRestore();

    const retryPicker = document.createElement("input");
    const retryFile = new File([Uint8Array.from(read.octets)], "retry.csv", { type: "text/csv" });
    Object.defineProperty(retryPicker, "files", { configurable: true, value: [retryFile] });
    const retryCreateElement = vi.spyOn(document, "createElement").mockReturnValueOnce(retryPicker);
    const retryClick = vi.spyOn(retryPicker, "click").mockImplementation(() => {});
    const retrySelection = requestFileSelectionV1(".csv,text/csv");
    retryPicker.dispatchEvent(new Event("change"));
    expect(await retrySelection).toEqual([retryFile]);
    retryCreateElement.mockRestore();
    retryClick.mockRestore();
  });

  it("shares the world catalogue drop preview across all registered hosts", () => {
    clearWorldCatalogueDropPreview("puzzle3d-play");
    const notifications: Array<ReturnType<typeof getWorldCatalogueDropPreview>> = [];
    const unsub = subscribeWorldCatalogueDropPreview(() => {
      notifications.push(getWorldCatalogueDropPreview("puzzle3d-play"));
    });
    const unregisterA = registerWorldCatalogueDropHost("puzzle3d-play", "pane.a", (x, y) => x >= 0 && x < 100 && y >= 0 && y < 100);
    const unregisterB = registerWorldCatalogueDropHost("puzzle3d-play", "pane.b", (x, y) => x >= 100 && x < 200 && y >= 0 && y < 100);

    expect(worldCatalogueDropHostContainsPoint("puzzle3d-play", 50, 50)).toBe(true);
    expect(worldCatalogueDropHostContainsPoint("puzzle3d-play", 150, 50)).toBe(true);
    expect(worldCatalogueDropHostContainsPoint("puzzle3d-play", 250, 50)).toBe(false);
    expect(worldCatalogueDropHostContainsPoint("other-controller", 50, 50)).toBe(false);

    setWorldCatalogueDropPreview("puzzle3d-play", { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [1, 2, 0] });
    expect(getWorldCatalogueDropPreview("puzzle3d-play")).toEqual({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [1, 2, 0] });
    expect(getWorldCatalogueDropPreview("other-controller")).toBeNull();
    setWorldCatalogueDropPreview("puzzle3d-play", { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [3, 4, 0] });
    expect(getWorldCatalogueDropPreview("puzzle3d-play")?.origin).toEqual([3, 4, 0]);
    clearWorldCatalogueDropPreview("puzzle3d-play");
    expect(getWorldCatalogueDropPreview("puzzle3d-play")).toBeNull();
    expect(notifications).toEqual([{ objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [1, 2, 0] }, { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [3, 4, 0] }, null]);

    unsub();
    unregisterA();
    unregisterB();
  });

  it("shares live world selection previews across sibling panes without allowing an idle pane to clear the active gesture", () => {
    clearWorldSelectionPreview("puzzle3d-play");
    const notifications: Array<ReturnType<typeof getWorldSelectionPreview>> = [];
    const unsubscribe = subscribeWorldSelectionPreview(() => notifications.push(getWorldSelectionPreview("puzzle3d-play")));
    const preview = { sourceId: "pane.top", mergedComponentIds: null, mergedInstanceIds: ["object-a", "object-b"] } as const;

    setWorldSelectionPreview("puzzle3d-play", preview);
    clearWorldSelectionPreview("puzzle3d-play", "pane.perspective");
    expect(getWorldSelectionPreview("puzzle3d-play")).toEqual(preview);
    expect(getWorldSelectionPreview("other-controller")).toBeNull();

    setWorldSelectionPreview("puzzle3d-play", { ...preview, mergedInstanceIds: ["object-a", "object-b"] });
    expect(notifications).toEqual([preview]);

    clearWorldSelectionPreview("puzzle3d-play", "pane.top");
    expect(getWorldSelectionPreview("puzzle3d-play")).toBeNull();
    expect(notifications).toEqual([preview, null]);
    unsubscribe();
  });

  it("shares live gumball transform previews across sibling panes without allowing an idle pane to clear the active gesture", () => {
    clearWorldGumballTransformPreview("puzzle3d-play");
    const notifications: Array<ReturnType<typeof getWorldGumballTransformPreview>> = [];
    const unsubscribe = subscribeWorldGumballTransformPreview(() => notifications.push(getWorldGumballTransformPreview("puzzle3d-play")));
    const preview = {
      sourceId: "pane.top",
      transformMode: "transform",
      handleKind: "moveX" as const,
      before: { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] } satisfies GumballPose,
      after: { position: [2, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] } satisfies GumballPose,
      instanceIds: ["object-a"],
      pivot: [0, 0, 0] as const,
    };

    setWorldGumballTransformPreview("puzzle3d-play", preview);
    clearWorldGumballTransformPreview("puzzle3d-play", "pane.perspective");
    expect(getWorldGumballTransformPreview("puzzle3d-play")).toEqual(preview);

    clearWorldGumballTransformPreview("puzzle3d-play", "pane.top");
    expect(getWorldGumballTransformPreview("puzzle3d-play")).toBeNull();
    expect(notifications).toEqual([preview, null]);
    unsubscribe();
  });

  it("mergeWorldSelectionWithLeftoverV1 keeps guest transformMode when leftover still stamps move-era gumball pose", () => {
    const merged = mergeWorldSelectionWithLeftoverV1({ ids: ["object-a"], transformMode: "transform", gumballActive: true, gumballTarget: [1, 2, 3] }, { ids: ["object-a"], hoveredId: null, gumballActive: true, gumballAnchorId: "object-a" }, [
      { id: "object-a", position: [1, 2, 3] },
    ]);
    expect(merged.transformMode).toBe("transform");
  });

  it("gumball rotate handles commit rotateSelection under transform mode even when transformMode was move", () => {
    const base = { mode: "object", ids: ["obj-1"] };
    const before: GumballPose = { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] };
    const after: GumballPose = { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] };
    expect(gumballTransformDeltaBetweenPoses("move", before, after, base, "rotateY")?.action).toBe("rotateSelection");
    const translateAfter: GumballPose = { position: [2, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] };
    expect(gumballPreviewOwnedWorldPoint({ position: [0, 0, 0], rotation: [0, 0, 0, 1] }, "transform", before, translateAfter, "moveX", [1, 0, 0])).toEqual([3, 0, 0]);
  });

  it("previews a multi-object turn and scale of markers about each owner's origin, where the selection leaves land them", () => {
    const close = (actual: readonly number[], expected: readonly number[]) => actual.forEach((value, index) => expect(value).toBeCloseTo(expected[index]!, 9));
    const owners = worldGumballOwnerPoses([
      { id: "a", position: [0, 0, 0], rotation: [0, 0, 0, 1] },
      { id: "b", position: [10, 0, 0], rotation: [0, 0, Math.SQRT1_2, Math.SQRT1_2] },
    ]);
    const quarter: GumballPose = { position: [5, 0, 0], quaternion: [0, 0, Math.SQRT1_2, Math.SQRT1_2], scale: [1, 1, 1] };
    const start: GumballPose = { position: [5, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] };
    const preview = { sourceId: "pane", transformMode: "rotate", handleKind: "rotateZ" as const, before: start, after: quarter, instanceIds: ["a", "b"], pivot: [5, 0, 0] as const };
    const vortices = [
      { fullId: "a:v0", objectId: "a", position: [1, 0, 0] as const, direction: [1, 0, 0] as const },
      { fullId: "b:v0", objectId: "b", position: [11, 0, 0] as const, direction: [1, 0, 0] as const },
      { fullId: "c:v0", objectId: "c", position: [20, 0, 0] as const },
    ];
    const turned = worldVorticesWithGumballPreview(vortices, preview, owners);
    close(turned[0]!.position, [0, 1, 0]);
    close(turned[1]!.position, [10, 1, 0]);
    close(turned[1]!.direction!, [0, 1, 0]);
    expect(turned[2]).toBe(vortices[2]);
    const [attraction] = worldAttractionsWithGumballPreview([{ id: "ab", from: [1, 0, 0], to: [11, 0, 0] }], vortices, preview, owners);
    close(attraction!.from, [0, 1, 0]);
    close(attraction!.to, [10, 1, 0]);
    const stretched: GumballPose = { position: [5, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 1, 1] };
    close(gumballPreviewOwnedWorldPoint(owners.get("b")!, "scale", start, stretched, "scaleX", [10, 1, 0]), [10, 2, 0]);
    close(gumballPreviewOwnedWorldDirection(owners.get("b")!, "scale", start, stretched, "scaleX", [1, 1, 0]), [1 / Math.sqrt(5), 2 / Math.sqrt(5), 0]);
  });

  it("pushes fixture-drop previews to every board2d peer on the same controller", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    const calls: { pane: string; method: string; arg: string }[] = [];
    const makePeer = (pane: string) => ({
      session: {
        setDropPreviewJson: (json: string) => calls.push({ pane, method: "setDropPreviewJson", arg: json }),
        clearDropPreview: () => calls.push({ pane, method: "clearDropPreview", arg: "" }),
        renderFrame: () => calls.push({ pane, method: "renderFrame", arg: "" }),
      } as never,
      onPeerGestureEnded: () => {},
    });
    registerBoard2dPeer(scope, "fixture-preview", "pane.source", makePeer("pane.source"));
    registerBoard2dPeer(scope, "fixture-preview", "pane.sibling", makePeer("pane.sibling"));

    const preview = puzzle2dDropPreviewJson({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24 }, 10, 20);
    pushPuzzle2dDropPreview(scope, "fixture-preview", preview);
    pushPuzzle2dDropPreview(scope, "fixture-preview", null);

    expect(calls).toEqual([
      { pane: "pane.source", method: "setDropPreviewJson", arg: preview },
      { pane: "pane.source", method: "renderFrame", arg: "" },
      { pane: "pane.sibling", method: "setDropPreviewJson", arg: preview },
      { pane: "pane.sibling", method: "renderFrame", arg: "" },
      { pane: "pane.source", method: "clearDropPreview", arg: "" },
      { pane: "pane.source", method: "renderFrame", arg: "" },
      { pane: "pane.sibling", method: "clearDropPreview", arg: "" },
      { pane: "pane.sibling", method: "renderFrame", arg: "" },
    ]);

    unregisterBoard2dPeer(scope, "fixture-preview", "pane.source", scope.peers.get("fixture-preview")?.get("pane.source") ?? null);
    unregisterBoard2dPeer(scope, "fixture-preview", "pane.sibling", scope.peers.get("fixture-preview")?.get("pane.sibling") ?? null);
  });

  it("inverts the canonical screen-to-world transform for a fixture drop", () => {
    const cameraJson = JSON.stringify({ x: 120, y: 80, zoom: 2 });
    const world = puzzle2dScreenToWorld(cameraJson, { w: 800, h: 600 }, { x: 400, y: 300 });
    expect(world).toEqual({ x: 120, y: 80 });
  });

  it("puzzle2dWorldToScreen is the exact inverse of puzzle2dScreenToWorld", () => {
    const cameraJson = JSON.stringify({ x: 120, y: 80, zoom: 2 });
    const containerSize = { w: 800, h: 600 };
    // 🎯️ The camera's own world position always maps to the viewport center.
    expect(puzzle2dWorldToScreen(cameraJson, containerSize, { x: 120, y: 80 })).toEqual({ x: 400, y: 300 });
    for (const screen of [
      { x: 0, y: 0 },
      { x: 400, y: 300 },
      { x: 733, y: 12 },
    ]) {
      const world = puzzle2dScreenToWorld(cameraJson, containerSize, screen);
      expect(world).not.toBeNull();
      const roundTrip = puzzle2dWorldToScreen(cameraJson, containerSize, world!);
      expect(roundTrip!.x).toBeCloseTo(screen.x, 5);
      expect(roundTrip!.y).toBeCloseTo(screen.y, 5);
    }
    expect(puzzle2dWorldToScreen("not json", containerSize, { x: 0, y: 0 })).toBeNull();
  });

  it("does not use geometry kind as a canvas layer overlay label", () => {
    expect(canvasLayerDisplayLabel({ kind: "circle", id: "node-n1" })).toBe("");
    expect(canvasLayerDisplayLabel({ kind: "circle", id: "node-n1", name: "N1" })).toBe("N1");
    expect(canvasLayerDisplayLabel({ kind: "circle", id: "node-n1", base: { name: "Base" } })).toBe("Base");
  });

  it("maps a world-centered node inside the viewport with canonical camera math", () => {
    const camera = { x: 120, y: 80, zoom: 2 };
    const viewportWidth = 800;
    const viewportHeight = 600;
    const screen = worldToScreenLogical(120, 80, camera, viewportWidth, viewportHeight);
    expect(screen.x).toBeCloseTo(viewportWidth * 0.5, 5);
    expect(screen.y).toBeCloseTo(viewportHeight * 0.5, 5);
    const layersJson = JSON.stringify([
      {
        id: "node-a",
        kind: "circle",
        role: "node",
        color: "#336699",
        selected: true,
        x: 110,
        y: 70,
        width: 20,
        height: 20,
      },
    ]);
    expect(layersJson).toContain('"role":"node"');
    expect(layersJson).toContain('"selected":true');
  });

  it("canvas-2d surface colors follow the light theme canvas token instead of a hardcoded dark fill", () => {
    document.documentElement.classList.remove("dark");
    const colors = readCanvas2dSurfaceColors();
    expect(colors.clear).toBe("rgba(240, 236, 221, 1)");
    expect(colors.clear.toLowerCase()).not.toContain("17, 19, 24");
    expect(colors.grid).toMatch(/^rgba\(/);
  });

  it("renders world 3d empty state without mounting r3f canvas", () => {
    const markup = renderToStaticMarkup(
      createElement(World3dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle.play.world",
          controllerId: "puzzle-play",
          componentKind: "world-3d",
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-world-3d-empty");
    expect(markup).not.toContain("data-orbit-view-gizmo");
  });

  it("marks every non-empty world-3d host with the bottom-right orbit view gizmo", () => {
    const markup = renderToStaticMarkup(
      createElement(World3dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle.3d.play.viewport",
          controllerId: "puzzle3d-play",
          componentKind: "world-3d",
          world3d: {
            cameraJson: '{"position":[4,4,4],"target":[0,0,0],"zoom":1}',
            meshesJson: "[]",
            instancesJson: "[]",
            selectionJson: "{}",
            vorticesJson: "[]",
            attractionsJson: "[]",
            interactionJson: '{"activeUtility":"select"}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-world-3d-host");
    expect(markup).toContain('data-orbit-view-gizmo=""');
    expect(markup).toContain('data-surface-id="puzzle.3d.play.viewport"');
    expect(markup).toContain("data-world-projection-kind-switch");
  });

  it("mirrors the world-3d camera pose into a stable rounded dom json attribute", () => {
    const pose = { position: [1.000004, 2, 3], target: [0, 0, 0], zoom: 1, up: [0, 0, 1], projection: "perspective", fov: 50 } as const;
    const first = world3dCameraDomJson(pose);
    expect(first).toBe(world3dCameraDomJson({ ...pose, position: [1.000001, 2, 3] }));
    expect(JSON.parse(first)).toEqual({ position: [1, 2, 3], target: [0, 0, 0], up: [0, 0, 1], zoom: 1, fov: 50, projection: "perspective" });
    expect(world3dCameraDomJson({ ...pose, position: [9, 2, 3] })).not.toBe(first);
    expect(world3dCameraDomJson({ ...pose, target: [0, 0, 4] })).not.toBe(first);
    expect(world3dCameraDomJson({ ...pose, zoom: 2 })).not.toBe(first);
    expect(JSON.parse(world3dCameraDomJson({ position: [0, 0, 0], target: [0, 0, 0], zoom: 1 })).up).toBe(null);
  });

  it("keeps world-3d orbit camera seed local per viewport once detached", () => {
    const sceneCamera = '{"position":[1,2,3],"target":[0,0,0],"zoom":1}';
    expect(world3dViewportCameraSeedKey(sceneCamera, 0)).toBe(sceneCamera);
    expect(world3dViewportCameraSeedKey(sceneCamera, 1)).toBe("viewport:1");
    expect(world3dViewportCameraSeedKey(sceneCamera, 2)).toBe("viewport:2");
    expect(world3dViewportCameraSeedKey(sceneCamera, 0)).not.toBe(world3dViewportCameraSeedKey(sceneCamera, 1));
    expect(shouldReattachWorldViewportCamera(sceneCamera, sceneCamera)).toBe(false);
    expect(shouldReattachWorldViewportCamera(sceneCamera, '{"position":[9,9,9],"target":[0,0,0],"zoom":1}')).toBe(true);
    const merged = mergeWorldViewportCamera(
      { position: [1, 2, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: true, projectionFrame: "preserveCamera", up: [0, 0, 1] },
      { position: [4, 5, 6], target: [1, 1, 1], zoom: 2, projection: "orthographic", up: [0, 1, 0] },
    );
    expect(merged.projectionFrame).toBe("preserveCamera");
    expect(merged.position).toEqual([4, 5, 6]);
    expect(merged.target).toEqual([1, 1, 1]);
    expect(merged.zoom).toBe(2);
    expect(merged.projection).toBe("orthographic");
    expect(merged.fov).toBe(45);
    expect(merged.explicitProjection).toBe(true);
    expect(merged.up).toEqual([0, 1, 0]);
  });

  it("world3dFitProjectionContent pauses fill-driven projection reframes while the user is navigating or fill is armed", () => {
    expect(world3dFitProjectionContent(false, false, true)).toBe(true);
    expect(world3dFitProjectionContent(true, false, true)).toBe(false);
    expect(world3dFitProjectionContent(false, true, true)).toBe(false);
    expect(world3dFitProjectionContent(false, false, true, true)).toBe(false);
    expect(world3dFitProjectionContent(false, false, false)).toBe(false);
    expect(world3dFitProjectionContent(false, false, true, false, "preserveCamera")).toBe(false);
    expect(world3dProjectionContentFrameMounted(true, false, false)).toBe(true);
    expect(world3dProjectionContentFrameMounted(true, false, true)).toBe(false);
    expect(world3dProjectionContentFrameMounted(false, true, true)).toBe(false);
    expect(world3dProjectionContentFrameMounted(false, true, false)).toBe(true);
  });

  it("world3dFramingInstances keeps fill provisional placements out of projection framing bounds", () => {
    const committed = worldSceneContentBounds([{ position: [0, 0, 0] }]);
    const withProvisional = worldSceneContentBounds(
      world3dFramingInstances([
        { id: "a", position: [0, 0, 0] },
        { id: "b", position: [40, -12, 3], provisional: true },
      ]),
    );
    expect(worldSceneContentBoundsKey(withProvisional)).toBe(worldSceneContentBoundsKey(committed));
  });

  it("world3dFrameVisibleOverlayOffered never paints the retired frame-visible overlay", () => {
    expect(world3dFrameVisibleOverlayOffered(null)).toBe(false);
    expect(world3dFrameVisibleOverlayOffered({ enabled: false })).toBe(false);
    expect(world3dFrameVisibleOverlayOffered({ enabled: true })).toBe(false);
    expect(
      world3dFrameVisibleOverlayOffered({
        enabled: true,
        boundsMin: [0, 0, 0],
        boundsMax: [1, 1, 1],
      }),
    ).toBe(false);
  });

  it("world3dFrameDistanceForRadius stands the eye off far enough that the whole bounding sphere projects inside the frustum", () => {
    for (const [fov, aspect] of [
      [45, 1],
      [45, 1.8],
      [45, 0.6],
      [30, 2.2],
      [60, 1.3],
    ] as const) {
      const radius = 1.7320508;
      const distance = world3dFrameDistanceForRadius(radius, fov, aspect);
      const vertical = ((fov * Math.PI) / 180) * 0.5;
      const horizontal = Math.atan(Math.tan(vertical) * aspect);
      expect(Math.asin(radius / distance)).toBeLessThan(Math.min(vertical, horizontal));
      expect(distance).toBeLessThan((radius / Math.sin(Math.min(vertical, horizontal))) * 1.25);
    }
  });

  it("world3dBoundsRadius is the bounding-sphere radius, never half the longest edge", () => {
    expect(world3dBoundsRadius([0, 0, 0], [2, 2, 2])).toBeCloseTo(Math.sqrt(3), 6);
    expect(world3dBoundsRadius([-1.2, -1.2, -1.2], [1.5, 1.5, 1.5])).toBeCloseTo((Math.sqrt(3) * 2.7) / 2, 6);
  });

  it("a camera report never invents a target", () => {
    expect(worldCameraReportTargetV1({ mouseButtons: {}, target: { x: 1.5, y: -2, z: 3 } as never })).toEqual({ x: 1.5, y: -2, z: 3 });
    expect(worldCameraReportTargetV1(null)).toBeNull();
    expect(worldCameraReportTargetV1(undefined)).toBeNull();
    expect(worldCameraReportTargetV1({ mouseButtons: {} })).toBeNull();
    expect(worldCameraReportTargetV1({ mouseButtons: {}, target: { x: Number.NaN, y: 0, z: 0 } as never })).toBeNull();
    expect(worldCameraReportTargetV1({ mouseButtons: {}, target: { x: 0, y: Number.POSITIVE_INFINITY, z: 0 } as never })).toBeNull();
    expect(worldCameraReportTargetV1({ mouseButtons: {}, target: { x: 0, y: 0, z: 0 } as never })).toEqual({ x: 0, y: 0, z: 0 });
  });

  it("world3dAutoFitOwed frames the first delivery of a document and never yanks a camera the user moved on it", () => {
    expect(world3dAutoFitOwed("", "7:a", false)).toBe(true);
    expect(world3dAutoFitOwed("7:a", "7:a", false)).toBe(false);
    expect(world3dAutoFitOwed("", "7:a", true)).toBe(false);
    expect(world3dAutoFitOwed("7:a", "7:b", true)).toBe(false);
    expect(world3dAutoFitOwed("7:a", "9:a", false)).toBe(true);
  });

  it("world3dAutoFitKey tracks document revision and published bounds, not scene mesh roster churn", () => {
    expect(world3dAutoFitKey(7, "seed", null)).toBe("7:seed");
    expect(world3dAutoFitKey(7, "seed", null)).toBe(world3dAutoFitKey(7, "seed", null));
    const bounds: [readonly number[], readonly number[]] = [
      [0, 0, 0],
      [2, 2, 3],
    ];
    expect(world3dAutoFitKey(7, "seed", bounds)).toBe("7:seed:0,0,0:2,2,3");
    expect(world3dAutoFitOwed(world3dAutoFitKey(7, "seed", null), world3dAutoFitKey(7, "seed", null), false)).toBe(false);
  });

  it("a framed camera puts every corner of the delivered box inside the viewport, for every bundled generation3d example", () => {
    const examples: readonly (readonly [string, readonly [number, number, number], readonly [number, number, number]])[] = [
      ["hexagonal-mushroom-column", [-0.5, -0.43301, 0], [0.5, 0.43301, 6]],
      ["rectangle-extrude-volume", [0, 0, 0], [2, 2, 3]],
      ["face-sweep-extrude", [0, 0, 0], [2, 1.5, 4]],
      ["box-shell-preview", [0, 0, 0], [2, 2, 2]],
      ["box-fillet-preview", [0, 0, 0], [2, 2, 2]],
      ["rectangle-wire-preview", [0, 0, 0], [2, 1.5, 0]],
      ["sphere-box-fuse", [-1.2, -1.2, -1.2], [1.5, 1.5, 1.5]],
      ["sphere-cut-with-torus", [-2.2, -2.2, -2.2], [2.2, 2.2, 2.2]],
    ];
    const seed = { position: [4, -4, 3] as [number, number, number], target: [0, 0, 0] as [number, number, number], zoom: 1, projection: "perspective" as const, fov: 45, explicitProjection: false, projectionFrame: "content" as const };
    for (const [name, minimum, maximum] of examples) {
      for (const aspect of [1.7, 1.0, 0.7]) {
        const center: [number, number, number] = [(minimum[0] + maximum[0]) / 2, (minimum[1] + maximum[1]) / 2, (minimum[2] + maximum[2]) / 2];
        const framed = world3dFrameCameraFromBounds(center, world3dBoundsRadius(minimum, maximum), seed, undefined, aspect);
        const forward = [framed.target[0] - framed.position[0], framed.target[1] - framed.position[1], framed.target[2] - framed.position[2]];
        const forwardLength = Math.hypot(...forward);
        const unitForward = forward.map((value) => value / forwardLength);
        const worldUp = [0, 0, 1];
        const right = [unitForward[1] * worldUp[2] - unitForward[2] * worldUp[1], unitForward[2] * worldUp[0] - unitForward[0] * worldUp[2], unitForward[0] * worldUp[1] - unitForward[1] * worldUp[0]];
        const rightLength = Math.hypot(...right);
        const unitRight = right.map((value) => value / rightLength);
        const unitUp = [unitRight[1] * unitForward[2] - unitRight[2] * unitForward[1], unitRight[2] * unitForward[0] - unitRight[0] * unitForward[2], unitRight[0] * unitForward[1] - unitRight[1] * unitForward[0]];
        const halfVertical = Math.tan(((seed.fov * Math.PI) / 180) * 0.5);
        for (const corner of [
          [minimum[0], minimum[1], minimum[2]],
          [minimum[0], minimum[1], maximum[2]],
          [minimum[0], maximum[1], minimum[2]],
          [minimum[0], maximum[1], maximum[2]],
          [maximum[0], minimum[1], minimum[2]],
          [maximum[0], minimum[1], maximum[2]],
          [maximum[0], maximum[1], minimum[2]],
          [maximum[0], maximum[1], maximum[2]],
        ]) {
          const relative = [corner[0] - framed.position[0], corner[1] - framed.position[1], corner[2] - framed.position[2]];
          const depth = relative[0] * unitForward[0] + relative[1] * unitForward[1] + relative[2] * unitForward[2];
          expect(depth).toBeGreaterThan(0);
          const ndcY = (relative[0] * unitUp[0] + relative[1] * unitUp[1] + relative[2] * unitUp[2]) / (depth * halfVertical);
          const ndcX = (relative[0] * unitRight[0] + relative[1] * unitRight[1] + relative[2] * unitRight[2]) / (depth * halfVertical * aspect);
          expect(`${name}@${aspect} x=${ndcX.toFixed(3)}`).toBe(`${name}@${aspect} x=${Math.max(Math.min(ndcX, 0.999), -0.999).toFixed(3)}`);
          expect(`${name}@${aspect} y=${ndcY.toFixed(3)}`).toBe(`${name}@${aspect} y=${Math.max(Math.min(ndcY, 0.999), -0.999).toFixed(3)}`);
        }
      }
    }
  });

  it("buildWorldCameraDispatchArgs carries position/target/zoom/up but never a projection field", () => {
    const withUp = buildWorldCameraDispatchArgs({ position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1], projection: "orthographic" });
    expect(withUp).toEqual({ position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1] });
    expect(withUp).not.toHaveProperty("projection");
    expect(withUp).not.toHaveProperty("projectionSpec");

    const withoutUp = buildWorldCameraDispatchArgs({ position: [1, 2, 3], target: [0, 0, 0], zoom: 1, projection: "perspective" });
    expect(withoutUp).toEqual({ position: [1, 2, 3], target: [0, 0, 0], zoom: 1 });
    expect(withoutUp).not.toHaveProperty("up");
    expect(withoutUp).not.toHaveProperty("projection");
  });

  it("worldCameraSetCameraDispatchArgs nests the camera pose under a `camera` key, never flat alongside windowId", () => {
    const args = worldCameraSetCameraDispatchArgs("puzzle.3d.play.viewport", { position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1], projection: "orthographic" });
    expect(args).toEqual({ windowId: "puzzle.3d.play.viewport", camera: { position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1] } });
    expect(args).not.toHaveProperty("position");
    expect(args).not.toHaveProperty("target");
    expect(args).not.toHaveProperty("zoom");
    expect(args).not.toHaveProperty("up");
  });

  it("worldCameraPoseApproxEqual matches exact poses and float-noise, rejects a genuinely different pose", () => {
    const base = { position: [1, 2, 3] as const, target: [0, 0, 0] as const, zoom: 1 };
    expect(worldCameraPoseApproxEqual(base, { position: [1, 2, 3], target: [0, 0, 0], zoom: 1 })).toBe(true);
    expect(worldCameraPoseApproxEqual(base, { position: [1.0000001, 2.0000001, 3], target: [0, 0, 1e-9], zoom: 1.0000001 })).toBe(true);
    expect(worldCameraPoseApproxEqual(base, { position: [9, 2, 3], target: [0, 0, 0], zoom: 1 })).toBe(false);
    expect(worldCameraPoseApproxEqual(base, { position: [1, 2, 3], target: [0, 0, 0], zoom: 5 })).toBe(false);
  });

  it("shouldReattachWorldViewportCamera suppresses a self-echo of the last dispatched camera but not a genuinely different pose", () => {
    const previous = '{"position":[1,2,3],"target":[0,0,0],"zoom":1}';
    const dispatched = { position: [4, 5, 6] as const, target: [0, 0, 0] as const, zoom: 2 };
    const echoedJson = '{"position":[4.0000001,5,6],"target":[0,0,0],"zoom":2}';
    const differentJson = '{"position":[9,9,9],"target":[0,0,0],"zoom":1}';
    expect(shouldReattachWorldViewportCamera(previous, echoedJson, dispatched)).toBe(false);
    expect(shouldReattachWorldViewportCamera(previous, differentJson, dispatched)).toBe(true);
    expect(shouldReattachWorldViewportCamera(previous, previous, dispatched)).toBe(false);
    expect(shouldReattachWorldViewportCamera(previous, differentJson, null)).toBe(true);
  });

  it("preserves projectionSpec.view from gizmo snaps instead of clobbering to top", () => {
    const merged = mergeWorldViewportCamera(
      { position: [0, 0, 10], target: [0, 0, 0], zoom: 50, projection: "orthographic", fov: 45, explicitProjection: true, projectionFrame: "content", projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } } },
      { position: [0, -600, 0], target: [0, 0, 0], zoom: 50, projection: "orthographic", projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } } },
    );
    expect(merged.projectionSpec).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } });
  });

  it("accepts extended world 3d scene fields", () => {
    const node = {
      type: "componentScene",
      surfaceId: "puzzle.3d.play.viewport",
      controllerId: "puzzle3d-play",
      componentKind: "world-3d",
      world3d: {
        cameraJson: "{}",
        meshesJson: "[]",
        instancesJson: "[]",
        selectionJson: "{}",
        vorticesJson: "[]",
        attractionsJson: "[]",
        targetVolumesJson: "[]",
        referencesJson: "[]",
        brushPreviewJson: undefined,
        interactionJson: '{"activeUtility":"select"}',
        engagementPreviewJson: '[{"kind":"point","role":"origin","position":[0,0,0]},{"kind":"box-preview","role":"preview","cornerA":[0,0,0],"cornerB":[2,2,0]}]',
        // 🖱️ Context menus are no longer pushed as scene JSON — a right-click round-trips through
        // `requestContextMenu`/`ContextMenuItemSpec` on demand instead (see `openSurfaceContextMenu`,
        // renderer `🟦️.tsx`), so `World3dScene` has no `contextMenuJson` field to cover here.
        statusJson: '{"computing":true,"label":"Evaluating"}',
        terrainJson: '{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":1.5,"colorRamp":"hypsometric","minZoom":6,"maxZoom":14}',
      },
    };
    expect(node.world3d?.meshesJson).toBe("[]");
    expect(node.world3d?.vorticesJson).toBe("[]");
    expect(node.world3d?.interactionJson).toContain("select");
    expect(node.world3d?.engagementPreviewJson).toContain("box-preview");
    expect(node.world3d?.statusJson).toContain("computing");
    expect(node.world3d?.terrainJson).toContain("hypsometric");
  });

  it("parses GIS 3D terrain style JSON, defaulting missing fields, and rejects a missing tileUrlTemplate", () => {
    expect(parseWorldTerrainStyle(undefined)).toBeNull();
    expect(parseWorldTerrainStyle("not json")).toBeNull();
    expect(parseWorldTerrainStyle('{"projectOriginLon":1}')).toBeNull();
    const style = parseWorldTerrainStyle('{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":2}');
    expect(style).toMatchObject({
      tileUrlTemplate: "/dem/{z}/{x}/{y}.png",
      projectOriginLon: 9.7382,
      projectOriginLat: 52.3759,
      exaggeration: 2,
      colorRamp: "hypsometric",
      minZoom: 6,
      maxZoom: 14,
    });
  });

  it("blocks instance picking for brush and volume brush engagements but not move", () => {
    expect(worldInstancePickBlocked("brush")).toBe(true);
    expect(worldInstancePickBlocked("volumeBrush")).toBe(true);
    expect(worldInstancePickBlocked("surfaceBrush")).toBe(true);
    expect(worldInstancePickBlocked("move")).toBe(false);
    expect(worldInstancePickBlocked(undefined)).toBe(false);
  });

  it("keeps the vortex hit proxy visible so Three's raycaster does not skip it", () => {
    expect(worldVortexHitProxy(0.36)).toEqual({ visible: true, radius: 0.36 });
    expect(worldVortexHitProxy(0.1).radius).toBeGreaterThanOrEqual(0.36);
    expect(worldVortexHitProxy().visible).toBe(true);
  });

  it("clears instance mesh raycasts when pick is blocked so sibling vortex markers stay hittable", () => {
    expect(worldInstanceMeshRaycast(true)).toBeUndefined();
    expect(worldInstanceMeshRaycast(false)).toEqual(expect.any(Function));
    const mesh = { isMesh: true, raycast: "keep" as unknown };
    const root = { traverse: (fn: (object: typeof mesh) => void) => fn(mesh) };
    applyWorldInstanceMeshRaycast(root, false, "keep");
    expect(mesh.raycast).toEqual(expect.any(Function));
    applyWorldInstanceMeshRaycast(root, true, "keep");
    expect(mesh.raycast).toBe("keep");
  });

  it("resolves vortex pointer-down to select in brush or vertex mode and click-or-drag otherwise", () => {
    expect(resolveVortexPointerDownIntent(true)).toBe("select");
    expect(resolveVortexPointerDownIntent(false, "vertex")).toBe("select");
    expect(resolveVortexPointerDownIntent(false)).toBe("click-or-drag");
    expect(resolveVortexPointerDownIntent(false, "mesh")).toBe("click-or-drag");
  });

  it("scopes suggestion menu ownership to the opening world window so sibling panes stay interactive", () => {
    expect(suggestionMenuOwnsWindow(null, "puzzle3d-main-top")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: false, windowId: "puzzle3d-main-top" }, "puzzle3d-main-top")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: true, windowId: "puzzle3d-main-top" }, "puzzle3d-main-top")).toBe(true);
    expect(suggestionMenuOwnsWindow({ open: true, windowId: "puzzle3d-main-top" }, "puzzle3d-main-perspective")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: true }, "puzzle3d-main-perspective")).toBe(true);
  });

  it("revisions vortex materials when selection or hover state changes", () => {
    expect(worldVortexMaterialRevision()).toBe("neutral");
    expect(worldVortexMaterialRevision(false, true)).toBe("hovered");
    expect(worldVortexMaterialRevision(true, true)).toBe("selected");
  });

  it("revisions world mesh materials when style kind changes so deselection clears selected paint", () => {
    expect(worldMeshMaterialRevision("selected")).toBe("selected");
    expect(worldMeshMaterialRevision("neutral")).toBe("neutral");
    expect(worldMeshMaterialRevision("hovered")).toBe("hovered");
    expect(worldMeshMaterialRevision(resolveMeshStyle({ selected: false, hovered: false }))).toBe("neutral");
  });

  it("uses the world surface selection mode instead of a stale shared invertive mode", () => {
    // 🐚️ "invertive" here plays the role the old page-global `__selectionMode` used to (a shell's
    // persistent toolbar toggle) — passed explicitly now via the `persistentMode` param instead of a
    // `globalThis` singleton, but the priority rule under test is unchanged: an explicitly *configured*
    // world-surface mode still wins over it.
    expect(resolveWorldMergeMode("replace", {}, "invertive")).toBe("replace");
    expect(resolveWorldMergeMode("replace", { shiftKey: true }, "invertive")).toBe("additive");
    expect(resolveWorldMergeMode("invertive", {}, "invertive")).toBe("invertive");
  });

  it("resolves mesh style by priority: disabled > celebrated > selected > highlighted > hovered > neutral", () => {
    expect(resolveMeshStyle({})).toBe("neutral");
    expect(resolveMeshStyle({ hovered: true })).toBe("hovered");
    expect(resolveMeshStyle({ hovered: true, highlighted: true })).toBe("highlighted");
    expect(resolveMeshStyle({ highlighted: true, selected: true })).toBe("selected");
    expect(resolveMeshStyle({ selected: true, celebrating: true })).toBe("celebrated");
    expect(resolveMeshStyle({ celebrating: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ selected: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ disabled: true, selected: true, highlighted: true, hovered: true, celebrating: true })).toBe("disabled");
  });

  it("paints the guest-stamped highlight (a history draft's reference) whether or not the host's own kind hover is on", () => {
    expect(worldInstanceHighlighted(false, { highlighted: true })).toBe(true);
    expect(worldInstanceHighlighted(true, {})).toBe(true);
    expect(worldInstanceHighlighted(false, {})).toBe(false);
    expect(resolveMeshSelectionPreviewStyle({ hovered: true, highlighted: worldInstanceHighlighted(false, { highlighted: true }) })).toBe("highlighted");
  });

  it("celebrateWorldInstances stamps ids and cancel clears them so paint prefers celebrated over selected", () => {
    const cancel = celebrateWorldInstances(["drop-1"], 60_000);
    expect(isWorldInstanceCelebrating("drop-1")).toBe(true);
    expect(resolveMeshStyle({ selected: true, celebrating: isWorldInstanceCelebrating("drop-1") })).toBe("celebrated");
    cancel();
    expect(isWorldInstanceCelebrating("drop-1")).toBe(false);
    expect(resolveMeshStyle({ selected: true, celebrating: isWorldInstanceCelebrating("drop-1") })).toBe("selected");
  });

  it("maps edge hover to line paint so coplanar edges stay distinct from face hover fill", () => {
    const palette = {
      neutral: { meshColor: "#111111", lineColor: "#222222", emissiveIntensity: 0, opacity: 1 },
      hovered: { meshColor: "#aaaaaa", lineColor: "#333333", emissiveIntensity: 0.08, opacity: 1 },
      selected: { meshColor: "#0000ff", lineColor: "#0000ff", emissiveIntensity: 0.35, opacity: 1 },
      highlighted: { meshColor: "#00ff00", lineColor: "#00ff00", emissiveIntensity: 0.2, opacity: 1 },
      celebrated: { meshColor: "#ff00ff", lineColor: "#ff00ff", emissiveIntensity: 0.55, opacity: 1 },
      disabled: { meshColor: "#999999", lineColor: "#888888", emissiveIntensity: 0, opacity: 0.45 },
    } as Parameters<typeof semanticColorsFromPalette>[0];
    const colors = semanticColorsFromPalette(palette);
    expect(colors.hover).toBe("#00ff00");
    expect(colors.edgeHover).toBe("#00ff00");
    expect(colors.edgeHover).toBe(colors.hover);
    expect(colors.select).toBe("#0000ff");
  });

  it("treats centerline meshes without shaded triangles as curve-only instances", () => {
    expect(isCurveOnlyWorldMesh({ indices: [], edgePositions: [0, 0, 0, 1, 0, 0] })).toBe(true);
    expect(isCurveOnlyWorldMesh({ indices: [0, 1, 2], edgePositions: [0, 0, 0, 1, 0, 0] })).toBe(false);
    expect(isCurveOnlyWorldMesh({ indices: [], edgePositions: [] })).toBe(false);
  });

  it("derives marquee bounds from edge samples when positions are empty", () => {
    const corners = meshBoundsCorners({
      positions: [],
      normals: [],
      indices: [],
      edgePositions: [0, 0, 0, 2, 4, 6],
    } as Parameters<typeof meshBoundsCorners>[0]);
    expect(corners).toContainEqual([0, 0, 0]);
    expect(corners).toContainEqual([2, 4, 6]);
  });

  it("renders the new group selection as active and only objects leaving the old selection as highlighted", () => {
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, false)).toBe("highlighted");
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, false)).toBe("neutral");
    expect(resolveMeshSelectionPreviewStyle({ selected: true, disabled: true }, false)).toBe("disabled");
  });

  it("resolves catalogue-drop ghost mesh URLs even when the kind is not yet among scene meshes", () => {
    // 👻️ A dragged catalogue kind must load its own meshUrl directly — requiring a scene mesh match left a
    // kind not placed yet invisible in 3D.
    expect(worldGhostMeshUrl({ meshUrl: "/meshes/new-kind.glb" }, [])).toBe("/meshes/new-kind.glb");
    expect(worldGhostMeshUrl({ meshUrl: "/meshes/placed.glb" }, [{ url: "/meshes/placed.glb" }])).toBe("/meshes/placed.glb");
    expect(worldGhostMeshUrl({}, [{ url: "/meshes/placed.glb" }])).toBeUndefined();
  });

  it("resolves the right-click context menu target by priority: vortex, then object, then reference", () => {
    expect(resolveWorldContextMenuTarget({ hoveredVortexFullId: "seed-left-001:v0" }, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({
      kind: "vortex",
      id: "seed-left-001:v0",
    });
    expect(resolveWorldContextMenuTarget({}, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({ kind: "object", id: "obj-1" });
    expect(resolveWorldContextMenuTarget({}, { hoveredId: "reference:ref-1" })).toEqual({ kind: "reference", id: "ref-1" });
    expect(resolveWorldContextMenuTarget({}, { hoveredId: "obj-1" })).toEqual({ kind: "object", id: "obj-1" });
    expect(resolveWorldContextMenuTarget({}, {})).toBeNull();
  });

  it("carries the right-clicked world entity as the request's own hit, so no target-recording dispatch precedes the menu", () => {
    expect(world3dContextMenuSurfaceV1({ kind: "vortex", id: "seed-left-001:v0" }, { ids: ["seed-left-001"], componentIds: [] })).toEqual({
      hits: [{ domain: "vortex", id: "seed-left-001:v0" }],
      selection: [{ domain: "object", ids: ["seed-left-001"] }],
    });
    expect(world3dContextMenuSurfaceV1(null, { ids: [], componentIds: [7] })).toEqual({ hits: [], selection: [{ domain: "feature", ids: ["7"] }] });
    expect(world3dContextMenuSurfaceV1(null, { ids: [], componentIds: [] })).toEqual({ hits: [], selection: [] });
  });

  it("titles context menus from the specific hit before falling back to the surface", () => {
    const request = (domain?: string, kind = "world3d") => ({
      menu: { id: kind, args: null },
      surface: { surfaceId: "surface", kind, hits: domain ? [{ domain, id: "target" }] : [] },
    });

    expect(surfaceContextMenuTitleKey(request("vortex"))).toBe("ui.surfaceContextMenu.vortex");
    expect(surfaceContextMenuTitleKey(request("object"))).toBe("ui.surfaceContextMenu.object");
    expect(surfaceContextMenuTitleKey(request("reference"))).toBe("ui.surfaceContextMenu.reference");
    expect(surfaceContextMenuTitleKey(request())).toBe("ui.surfaceContextMenu.scene");
    expect(surfaceContextMenuTitleKey(request(undefined, "board2d"))).toBe("ui.surfaceContextMenu.board");
  });

  it("covers every target domain emitted by current surface pickers", () => {
    const domains = ["architecture", "attraction", "block", "edge", "entry", "feature", "group", "handle", "layer", "node", "object", "part", "path", "pixel", "position", "reference", "route", "row", "slider", "vortex"];
    for (const domain of domains) {
      expect(surfaceContextMenuTitleKey({ menu: { id: "surface", args: null }, surface: { surfaceId: "surface", kind: "unknown", hits: [{ domain, id: "target" }] } })).toBe(`ui.surfaceContextMenu.${domain}`);
    }
  });

  it("renders text editor host", () => {
    const markup = renderToStaticMarkup(
      createElement(TextEditorHost, {
        node: {
          type: "componentScene",
          surfaceId: "writer.play.editor",
          controllerId: "writer-play",
          componentKind: "text-editor",
          textEditor: {
            buffer: "hello",
            language: "jack",
            tokensJson: JSON.stringify([{ class: "ident", start: 0, end: 5 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-text-editor-host");
    expect(markup).toContain("hello");
  });

  it("retires a mounted text selection drag without synthesizing an up or a second selection action", async () => {
    const attachCanvas = vi.fn(async () => {});
    const pointerDownScreen = vi.fn();
    const pointerUpScreen = vi.fn();
    const pointerCancelScreen = vi.fn();
    const onAction = vi.fn();
    const session = {
      attachCanvas,
      setCaretVisible: vi.fn(),
      setSize: () => {},
      renderFrame: vi.fn(),

      synchronizeScene: () => {},
      setText: () => {},
      text: () => "hello",
      caret: () => 2,
      anchor: () => 1,
      pointerDownScreen,
      pointerMoveScreen: () => {},
      pointerUpScreen,
      pointerCancelScreen,
      wheelScrollScreen: () => {},
      insertText: () => {},
      backspace: () => {},
      deleteForward: () => {},
      selectAll: () => {},
      replaceSelection: () => {},
      selectionText: () => "e",
      hoverTokenRangeJson: () => "null",
      setHoverRange: () => {},
      cameraJson: () => "{}",
      setCanvasThemeJson: () => {},
      moveLeft: () => {},
      moveRight: () => {},
      moveUp: () => {},
      moveDown: () => {},
      moveLineStart: () => {},
      moveLineEnd: () => {},
      tabInsertText: () => "  ",
      setSelectionRange: () => {},
      selectSpanAt: () => {},
      selectSpanAtScreen: () => {},
      pickTargetsAtScreenJson: () => "[]",
      free: vi.fn(),
    } as unknown as flowSessionLoader.EditorWasmSession;
    const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session);
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const view = render(
      createElement(TextEditorHost, {
        node: { type: "componentScene", surfaceId: "writer.cancel", controllerId: "writer", componentKind: "text-editor", textEditor: { buffer: "hello", language: "jack" } },
        onAction,
      }),
    );
    try {
      await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
      const surfaces = view.container.querySelectorAll(".semio-text-editor-host div.absolute.inset-0");
      const surface = surfaces.item(surfaces.length - 1) as HTMLElement;
      await waitFor(() => {
        fireEvent.pointerDown(surface, { pointerId: 4, button: 0, clientX: 12, clientY: 14 });
        expect(pointerDownScreen).toHaveBeenCalled();
      });
      const downsBeforeCancel = pointerDownScreen.mock.calls.length;
      const actionsBeforeCancel = onAction.mock.calls.length;
      fireEvent.pointerCancel(surface, { pointerId: 4, clientX: 20, clientY: 14 });
      expect(pointerCancelScreen).toHaveBeenCalledOnce();
      expect(pointerUpScreen).not.toHaveBeenCalled();
      expect(onAction).toHaveBeenCalledTimes(actionsBeforeCancel);
      fireEvent.pointerDown(surface, { pointerId: 5, button: 0, clientX: 24, clientY: 14 });
      expect(pointerDownScreen).toHaveBeenCalledTimes(downsBeforeCancel + 1);
    } finally {
      view.unmount();
      factory.mockRestore();
      bounds.mockRestore();
    }
  });

  it("retires only the closed React text editor and remounts a fresh sibling generation", async () => {
    const law = textInputFixture.hostLifecycle;
    const sessions = [law.text, law.text, law.expect.successorText].map((text) => ({
      attachCanvas: vi.fn(async () => {}),
      setCaretVisible: vi.fn(),
      setSize: () => {},
      renderFrame: () => {},

      synchronizeScene: () => {},
      setText: () => {},
      text: () => text,
      caret: () => text.length,
      anchor: () => text.length,
      setCanvasThemeJson: () => {},
      free: vi.fn(),
    }));
    let index = 0;
    const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockImplementation(async () => sessions[index++] as unknown as flowSessionLoader.EditorWasmSession);
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const editor = (window: string, buffer: string) =>
      createElement(TextEditorHost, {
        key: window,
        node: { type: "componentScene", surfaceId: law.surfaceId, controllerId: window, componentKind: "text-editor", textEditor: { buffer, language: "jack" } },
        onAction: noopAction,
      });
    const view = render(createElement("div", null, editor(law.closingWindow, law.text), editor(law.siblingWindow, law.text)));
    try {
      await waitFor(() => expect(sessions[1].attachCanvas).toHaveBeenCalledOnce());
      const before = view.container.querySelectorAll(".semio-text-editor-host").length;
      view.rerender(createElement("div", null, editor(law.siblingWindow, law.text)));
      expect(view.container.querySelectorAll(".semio-text-editor-host").length).toBe(law.expect.survivingHosts);
      expect(before - view.container.querySelectorAll(".semio-text-editor-host").length).toBe(law.expect.closedHosts);
      expect(view.container.querySelector("textarea")?.value).toBe(law.expect.siblingText);
      expect(sessions[1].attachCanvas).toHaveBeenCalledOnce();
      view.rerender(createElement("div", null, editor(law.closingWindow, law.expect.successorText), editor(law.siblingWindow, law.text)));
      await waitFor(() => expect(sessions[2].attachCanvas).toHaveBeenCalledOnce());
      expect(Array.from(view.container.querySelectorAll("textarea"), (area) => area.value)).toEqual([law.expect.successorText, law.expect.siblingText]);
      expect(sessions[1].attachCanvas).toHaveBeenCalledOnce();
      expect(factory).toHaveBeenCalledTimes(3);
    } finally {
      view.unmount();
      factory.mockRestore();
      bounds.mockRestore();
    }
  });

  

  for (const law of editorDeliveryFixture.cases)
    it(`settles actual React editor delivery: ${law.id}`, async () => {
      const { decodePackValue } = await import("@semio-tech/framework-os");
      let text = law.initial;
      let caret = text.length;
      const session = {
        attachCanvas: vi.fn(async () => {}),
        setCaretVisible: vi.fn(),
        setSize: () => {},
        renderFrame: () => {},

        setText: () => {},
        synchronizeScene: (pack: Uint8Array) => {
          const scene = decodePackValue(pack) as { buffer?: string; selection?: { end: number } };
          if (scene.buffer !== undefined) text = scene.buffer;
          if (scene.selection !== undefined) caret = scene.selection.end;
        },
        text: () => text,
        caret: () => caret,
        anchor: () => caret,
        setCanvasThemeJson: () => {},
        free: () => {},
        insertText: (value: string) => {
          text = text.slice(0, caret) + value + text.slice(caret);
          caret += value.length;
        },
      };
      const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session as unknown as flowSessionLoader.EditorWasmSession);
      const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
      const completions: Array<(outcome: unknown) => void> = [];
      const dispatched: string[] = [];
      const onAction = vi.fn((action: ActionDescriptor) => {
        const args = action.args as { readonly text?: string; readonly start?: number; readonly end?: number } | undefined;
        dispatched.push(action.action === "textEdit" ? `textEdit:${args?.text}` : `textSelect:${args?.start}:${args?.end}`);
        return action.action === "textEdit" ? new Promise((resolve) => completions.push(resolve)) : Promise.resolve(undefined);
      });
      const view = render(
        createElement(TextEditorHost, {
          node: {
            type: "componentScene",
            surfaceId: "writer.delivery",
            controllerId: "writer",
            componentKind: "text-editor",
            textEditor: {
              buffer: law.initial,
              selectionJson: JSON.stringify({ start: caret, end: caret }),
            },
          },
          onAction,
        }),
      );
      try {
        await waitFor(() => expect(session.attachCanvas).toHaveBeenCalledOnce());
        await reactAct(async () => {
          await Promise.resolve();
        });
        const area = view.container.querySelector("textarea")!;
        for (const key of law.typed) fireEvent.keyDown(area, { key });
        const typedAfterNewOwner = (law as { readonly typedAfterNewOwner?: readonly string[] }).typedAfterNewOwner ?? [];
        if (typedAfterNewOwner.length > 0) {
          view.rerender(createElement(TextEditorHost, { node: { type: "componentScene", surfaceId: "writer.delivery", controllerId: "writer", componentKind: "text-editor", textEditor: { buffer: law.initial, selectionJson: JSON.stringify({ start: law.initial.length, end: law.initial.length }) } }, onAction: (action: ActionDescriptor) => onAction(action) } as never));
          for (const key of typedAfterNewOwner) fireEvent.keyDown(view.container.querySelector("textarea")!, { key });
        }
        expect(dispatched).toEqual(law.expected[0]);
        await reactAct(async () => {
          completions.shift()!(law.outcome === "accepted" ? undefined : { kind: "refused", reason: law.outcome });
        });
        await waitFor(() => expect(dispatched).toEqual(law.expected[1]));
        if (completions.length)
          await reactAct(async () => {
            completions.shift()!(undefined);
          });
        await waitFor(() => expect(dispatched).toEqual(law.expected[2]));
        expect(area.getAttribute("aria-readonly")).toBe(String(law.readOnly));
        expect(text).toBe(law.outcome === "accepted" ? law.initial + [...law.typed, ...typedAfterNewOwner].join("") : law.initial);
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });

  for (const law of editorDeliveryFixture.spliceCases)
    it(`delivers splice typing against what the actual React editor shows at delivery: ${law.id}`, async () => {
      const { decodePackValue } = await import("@semio-tech/framework-os");
      let text = law.initial;
      let caret = text.length;
      const session = {
        attachCanvas: vi.fn(async () => {}),
        setCaretVisible: vi.fn(),
        setSize: () => {},
        renderFrame: () => {},

        setText: (value: string) => {
          text = value;
        },
        setSelectionRange: (_anchor: number, next: number) => {
          caret = next;
        },
        synchronizeScene: (pack: Uint8Array) => {
          const scene = decodePackValue(pack) as { buffer?: string };
          if (scene.buffer !== undefined) text = scene.buffer;
        },
        text: () => text,
        caret: () => caret,
        anchor: () => caret,
        setCanvasThemeJson: () => {},
        free: () => {},
        insertText: (value: string) => {
          text = text.slice(0, caret) + value + text.slice(caret);
          caret += value.length;
        },
      };
      const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session as unknown as flowSessionLoader.EditorWasmSession);
      const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
      const completions: Array<(outcome: unknown) => void> = [];
      const dispatched: string[] = [];
      const onAction = vi.fn((action: ActionDescriptor) => {
        if (action.action !== "textSplice") return Promise.resolve(undefined);
        const args = action.args as { readonly seq: number; readonly start: number; readonly deleted: string; readonly insert: string; readonly before: string; readonly after: string };
        dispatched.push(`textSplice:${args.seq}:${args.start}:${args.deleted}:${args.insert}:${args.before}:${args.after}`);
        return new Promise((resolve) => completions.push(resolve));
      });
      const node = (buffer: string, applied: number) => ({
        type: "componentScene",
        surfaceId: "writer.splice",
        controllerId: "writer",
        componentKind: "text-editor",
        textEditor: { buffer, selectionJson: JSON.stringify({ start: buffer.length, end: buffer.length, splice: applied }), settingsJson: JSON.stringify({ typing: { mode: "splice" } }) },
      });
      const view = render(createElement(TextEditorHost, { node: node(law.initial, 0), onAction } as never));
      try {
        await waitFor(() => expect(session.attachCanvas).toHaveBeenCalledOnce());
        await reactAct(async () => {
          await Promise.resolve();
        });
        const area = view.container.querySelector("textarea")!;
        for (const key of law.typedFirst) fireEvent.keyDown(area, { key });
        for (const key of law.typedWhileInFlight) fireEvent.keyDown(area, { key });
        await reactAct(async () => {
          view.rerender(createElement(TextEditorHost, { node: node(law.published.buffer, law.published.applied), onAction } as never));
          await Promise.resolve();
        });
        for (let turn = 0; turn < 16 && (completions.length > 0 || dispatched.length < law.expected.length); turn += 1)
          await reactAct(async () => {
            completions.shift()?.({ kind: "applied", inputSeq: 0 });
            await Promise.resolve();
          });
        await waitFor(() => expect(dispatched).toEqual(law.expected));
        expect(text).toBe(law.finalText);
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });

  for (const law of textInputFixture.rendererKeys)
    it(`routes the actual React text editor key: ${law.id}`, async () => {
      const attachCanvas = vi.fn(async () => {});
      let text = law.text;
      let selection = law.selection;
      const operations = Object.fromEntries(
        ["moveLeft", "moveRight", "moveUp", "moveDown", "moveLineStart", "moveLineEnd", "insertText"].map((name) => [
          name,
          vi.fn(() => {
            text = law.expect.text;
            selection = law.expect.selection;
          }),
        ]),
      );
      const session = {
        attachCanvas,
        setCaretVisible: vi.fn(),
        setSize: () => {},
        renderFrame: () => {},

        synchronizeScene: () => {},
        setText: () => {},
        text: () => text,
        caret: () => selection[1],
        anchor: () => selection[0],
        setCanvasThemeJson: () => {},
        free: () => {},
        tabInsertText: () => " ".repeat(law.tabSize ?? 2),
        ...operations,
      } as unknown as flowSessionLoader.EditorWasmSession;
      const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session);
      const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
      const onAction = vi.fn(async (_action: ActionDescriptor) => undefined);
      const view = render(
        createElement(TextEditorHost, {
          node: {
            type: "componentScene",
            surfaceId: "writer.keys",
            controllerId: "writer",
            componentKind: "text-editor",
            textEditor: {
              buffer: law.text,
              language: "jack",
              selectionJson: JSON.stringify({ start: law.selection[0], end: law.selection[1] }),
              newlineGatesJson: law.newlineGates === undefined ? undefined : JSON.stringify(law.newlineGates),
            },
          },
          onAction,
        }),
      );
      try {
        await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
        await reactAct(async () => {
          await Promise.resolve();
        });
        const area = view.container.querySelector("textarea")!;
        fireEvent.keyDown(area, { key: law.key, shiftKey: law.shift ?? false, altKey: law.alt ?? false });
        if (law.operation !== null) {
          expect(operations[law.operation]).toHaveBeenCalledWith(law.argument);
          await waitFor(() => expect(onAction).toHaveBeenCalledWith({ controllerId: "writer", action: "textSelect", args: { surfaceId: "writer.keys", start: law.expect.selection[0], end: law.expect.selection[1] } }));
          const actions = onAction.mock.calls.map(([action]) => action);
          expect(actions.map((action) => action.action)).toEqual(law.operation === "insertText" ? ["textEdit", "textSelect"] : ["textSelect"]);
          if (law.operation === "insertText") expect(actions[0]?.args).toEqual({ surfaceId: "writer.keys", text: law.expect.text, typing: "writer.keys" });
        } else {
          for (const operation of Object.values(operations)) expect(operation).not.toHaveBeenCalled();
          expect(onAction).not.toHaveBeenCalled();
        }
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });

  for (const stepKind of ["paste", "compose"] as const)
    it(`routes the actual React text editor ${stepKind} event through the shared text-input fixture`, async () => {
      await uiI18n.changeLanguage("en");
      const stepText = (step: (typeof textInputFixture.sequences)[number]["steps"][number]): string | undefined => (stepKind === "paste" ? ("paste" in step ? step.paste : undefined) : "compose" in step ? step.compose : undefined);
      const law = textInputFixture.sequences.find((sequence) => sequence.steps.some((step) => stepText(step) !== undefined))!;
      const committed = law.steps.map(stepText).find((text) => text !== undefined)!;
      let text = law.text;
      let selection = law.selection;
      const commit = vi.fn(() => {
        text = law.expect.text;
        selection = law.expect.selection;
      });
      const session = {
        attachCanvas: vi.fn(async () => {}),
        setCaretVisible: vi.fn(),
        setSize: () => {},
        renderFrame: () => {},

        synchronizeScene: () => {},
        setText: () => {},
        text: () => text,
        caret: () => selection[1],
        anchor: () => selection[0],
        setCanvasThemeJson: () => {},
        free: () => {},
        replaceSelection: commit,
        insertText: commit,
      } as unknown as flowSessionLoader.EditorWasmSession;
      const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session);
      const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
      const onAction = vi.fn(async (_action: ActionDescriptor) => undefined);
      const view = render(
        createElement(TextEditorHost, {
          node: { type: "componentScene", surfaceId: `writer.${stepKind}`, controllerId: "writer", componentKind: "text-editor", textEditor: { buffer: law.text, selectionJson: JSON.stringify({ start: law.selection[0], end: law.selection[1] }) } },
          onAction,
        }),
      );
      try {
        await waitFor(() => expect(session.attachCanvas).toHaveBeenCalledOnce());
        const area = view.container.querySelector("textarea")!;
        expect(area.getAttribute("aria-label")).toBe("Artifact");
        expect(area.getAttribute("aria-readonly")).toBe("false");
        const event = new Event(stepKind === "paste" ? "paste" : "compositionend", { bubbles: true, cancelable: true });
        if (stepKind === "paste") Object.defineProperty(event, "clipboardData", { value: { getData: (type: string) => (type === "text/plain" ? committed : "") } });
        else Object.defineProperty(event, "data", { value: committed });
        await reactAct(async () => {
          area.dispatchEvent(event);
        });
        await waitFor(() => expect(onAction).toHaveBeenCalledTimes(2));
        expect(commit).toHaveBeenCalledWith(committed);
        expect(onAction.mock.calls.map(([action]) => action)).toEqual([
          { controllerId: "writer", action: "textEdit", args: { surfaceId: `writer.${stepKind}`, text: law.expect.text, typing: `writer.${stepKind}` } },
          { controllerId: "writer", action: "textSelect", args: { surfaceId: `writer.${stepKind}`, start: law.expect.selection[0], end: law.expect.selection[1] } },
        ]);
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });

  it("renders text editor host with hover/newline/rename scene fields", () => {
    const markup = renderToStaticMarkup(
      createElement(TextEditorHost, {
        node: {
          type: "componentScene",
          surfaceId: "writer.play.editor",
          controllerId: "writer-play",
          componentKind: "text-editor",
          textEditor: {
            buffer: "MATCH (a:Piece) RETURN a.name",
            language: "jack",
            hoverJson: '{"start":0,"end":5}',
            newlineGatesJson: "[30]",
            renameJson: '{"name":"a","occurrences":[{"start":7,"end":8}]}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-text-editor-host");
  });

  it("multiSpanReplace renames every occurrence and remaps spans", () => {
    const result = multiSpanReplace(
      "MATCH (a:Piece) RETURN a.name",
      [
        { start: 7, end: 8 },
        { start: 23, end: 24 },
      ],
      "piece",
    );
    expect(result.text).toBe("MATCH (piece:Piece) RETURN piece.name");
    expect(result.occurrences).toEqual([
      { start: 7, end: 12 },
      { start: 23, end: 28 },
    ]);
  });

  it("lineRangeAt finds the line containing an offset", () => {
    const text = "MATCH (a)\nWHERE a.x = 1\nRETURN a";
    const range = lineRangeAt(text, 15);
    expect(text.slice(range.start, range.end)).toBe("WHERE a.x = 1");
  });

  it("renders table host with ui-react table", () => {
    const markup = renderToStaticMarkup(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.catalogue",
          controllerId: "s-play",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([{ id: "label", label: "Label" }]),
            rowsJson: JSON.stringify([{ label: "Draw" }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-table-host");
    expect(markup).toContain("Draw");
  });

  // 🆔️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 3-F: a plugin-authored
  // row's own `id` (now reachable from `TableWindowKit::render_rows`, `🔌️plugin/🦀️.rs`) must
  // survive all the way to the DOM as `data-row-id` (contract §C0's `"space:<id>"`/`"artifact:<id>"`
  // grammar) — this was already true of `TableHost`/`Table` before lane 3-F; these two tests lock it in.
  it("stamps a row's own id onto the rendered row's data-row-id attribute", () => {
    const { container } = render(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.space.home",
          controllerId: "s-home",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([{ id: "col0", label: "Name" }]),
            rowsJson: JSON.stringify([{ id: "space:abc", col0: { kind: "text", value: "Atelier" } }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(container.querySelector('[data-row-id="space:abc"]')).not.toBeNull();
  });

  it("dispatches a row action button's own ActionDescriptor, unmodified, on click", () => {
    const onAction = vi.fn();
    const { container } = render(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.space.home",
          controllerId: "s-home",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([
              { id: "col0", label: "Name" },
              { id: "actions", label: "" },
            ]),
            rowsJson: JSON.stringify([
              {
                id: "space:abc",
                col0: { kind: "text", value: "Atelier" },
                actions: { kind: "buttons", buttons: [{ iconId: "trash-2", label: "delete", action: { controllerId: "s-home", action: "deleteSpace", args: { spaceId: "abc" } } }] },
              },
            ]),
          },
        },
        onAction,
      }),
    );
    const button = container.querySelector('[data-row-id="space:abc"] button');
    if (!button) throw new Error("row action button not found");
    fireEvent.click(button);
    expect(onAction).toHaveBeenCalledWith({ controllerId: "s-home", action: "deleteSpace", args: { spaceId: "abc" } });
  });

  // 🪜️ Ticket 26/09/18 slice B3f: `TableCell::Stepper` is 🪵️sourcing's ONLY document mutation, and
  // the wgpu table widget drives it by merging `{ delta: ±step }` into the cell's own descriptor and
  // suppressing the press at the bound (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`, `render_table_cell` /
  // `table_cell_hit`). These lock the React host to the same contract, plus the spinbutton keyboard
  // route and the touch-sizing slots the stylesheet grows at phone width.
  const stepperTableNode = (value: number) => ({
    type: "componentScene" as const,
    surfaceId: "window:sourcing-pool",
    controllerId: "sourcing-curation",
    componentKind: "table" as const,
    table: {
      columnsJson: JSON.stringify([
        { id: "name", label: "Name" },
        { id: "curated", label: "Curated" },
      ]),
      rowsJson: JSON.stringify([
        {
          id: "beam-glulam-gl24h",
          name: { kind: "text", value: "Glulam GL24h" },
          curated: { kind: "stepper", value, min: 0, max: 3, step: 1, action: { controllerId: "sourcing-curation", action: "curationSetCount", args: { objectId: "beam-glulam-gl24h" } } },
        },
      ]),
    },
  });
  const renderStepperTable = (value: number, onAction: (action: unknown) => void) =>
    render(createElement(UiDriverProvider, { driver: DEFAULT_UI_DRIVER }, createElement(TableHost, { node: stepperTableNode(value) as never, onAction: onAction as never })));

  it("renders a stepper cell as a spinbutton with decrement, live value and increment controls", () => {
    const { container } = renderStepperTable(1, vi.fn());
    const readout = container.querySelector<HTMLInputElement>('[id="window:sourcing-pool.beam-glulam-gl24h.curated"]');
    expect(readout?.getAttribute("role")).toBe("spinbutton");
    expect(readout?.getAttribute("aria-valuenow")).toBe("1");
    expect(readout?.getAttribute("aria-valuemin")).toBe("0");
    expect(readout?.getAttribute("aria-valuemax")).toBe("3");
    expect(readout?.getAttribute("aria-label")).toBe("Curated");
    expect(container.querySelector('[data-stepper-control="decrement"]')).not.toBeNull();
    expect(container.querySelector('[data-stepper-control="increment"]')).not.toBeNull();
    expect(container.querySelector('[data-slot="table-stepper"]')?.getAttribute("data-stepper-for")).toBe("window:sourcing-pool.beam-glulam-gl24h.curated");
  });

  it("dispatches the cell's own descriptor with the wgpu host's {delta} patch on increment and decrement", () => {
    const onAction = vi.fn();
    const { container } = renderStepperTable(1, onAction);
    fireEvent.click(container.querySelector('[data-stepper-control="increment"]')!);
    expect(onAction).toHaveBeenLastCalledWith({ controllerId: "sourcing-curation", action: "curationSetCount", args: { objectId: "beam-glulam-gl24h", delta: 1 } });
    fireEvent.click(container.querySelector('[data-stepper-control="decrement"]')!);
    expect(onAction).toHaveBeenLastCalledWith({ controllerId: "sourcing-curation", action: "curationSetCount", args: { objectId: "beam-glulam-gl24h", delta: -1 } });
  });

  it("disables the stepper control that would leave the cell's min/max bounds", () => {
    const atMin = renderStepperTable(0, vi.fn());
    expect(atMin.container.querySelector<HTMLButtonElement>('[data-stepper-control="decrement"]')?.disabled).toBe(true);
    expect(atMin.container.querySelector<HTMLButtonElement>('[data-stepper-control="increment"]')?.disabled).toBe(false);
    const atMax = renderStepperTable(3, vi.fn());
    expect(atMax.container.querySelector<HTMLButtonElement>('[data-stepper-control="decrement"]')?.disabled).toBe(false);
    expect(atMax.container.querySelector<HTMLButtonElement>('[data-stepper-control="increment"]')?.disabled).toBe(true);
  });

  it("steps a stepper cell from the keyboard: arrows by one step, Home and End onto the bounds", () => {
    const onAction = vi.fn();
    const { container } = renderStepperTable(1, onAction);
    const readout = container.querySelector('[id="window:sourcing-pool.beam-glulam-gl24h.curated"]')!;
    fireEvent.keyDown(readout, { key: "ArrowUp" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: 1 } }));
    fireEvent.keyDown(readout, { key: "ArrowDown" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: -1 } }));
    fireEvent.keyDown(readout, { key: "End" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: 2 } }));
    fireEvent.keyDown(readout, { key: "Home" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: -1 } }));
    onAction.mockClear();
    fireEvent.keyDown(readout, { key: "a" });
    expect(onAction).not.toHaveBeenCalled();
  });

  it("never dispatches a keyboard step that would cross a bound", () => {
    const onAction = vi.fn();
    const { container } = renderStepperTable(3, onAction);
    const readout = container.querySelector('[id="window:sourcing-pool.beam-glulam-gl24h.curated"]')!;
    fireEvent.keyDown(readout, { key: "PageUp" });
    expect(onAction).not.toHaveBeenCalled();
    fireEvent.keyDown(readout, { key: "PageDown" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: -3 } }));
  });

  it("keeps the stepper buttons on the button-group slots the touch stylesheet sizes at phone width", () => {
    const { container } = renderStepperTable(1, vi.fn());
    const increment = container.querySelector('[data-stepper-control="increment"]');
    expect(increment?.getAttribute("data-slot")).toBe("button-group-item");
    expect(increment?.closest('[data-slot="button-group"]')).not.toBeNull();
    expect(increment?.getAttribute("aria-label")).toBe("Increase Curated");
    expect(container.querySelector('[data-stepper-control="decrement"]')?.getAttribute("aria-label")).toBe("Decrease Curated");
  });

  it("tableStepperKeyDelta and tableStepperClampedDelta agree with the wgpu segment contract", () => {
    const cell = { value: 2, min: 0, max: 5, step: 0.5 };
    expect(tableStepperKeyDelta("ArrowUp", cell)).toBe(0.5);
    expect(tableStepperKeyDelta("ArrowDown", cell)).toBe(-0.5);
    expect(tableStepperKeyDelta("PageUp", cell)).toBe(5);
    expect(tableStepperKeyDelta("Home", cell)).toBe(-2);
    expect(tableStepperKeyDelta("End", cell)).toBe(3);
    expect(tableStepperKeyDelta("Enter", cell)).toBe(0);
    expect(tableStepperClampedDelta(5, cell)).toBe(3);
    expect(tableStepperClampedDelta(-9, cell)).toBe(-2);
    expect(tableStepperClampedDelta(1, { value: 5, min: 0, max: 5 })).toBe(0);
  });

  it("matches the shared Table stepper keyboard fixture through the actual React spinbutton", () => {
    const { cell, action } = tableStepperKeyboardFixture;
    for (const keyboardCase of tableStepperKeyboardFixture.cases) {
      const onAction = vi.fn();
      const node = {
        type: "componentScene" as const,
        surfaceId: "window:sourcing-pool",
        controllerId: action.controllerId,
        componentKind: "table" as const,
        table: {
          columnsJson: JSON.stringify([{ id: cell.columnId, label: cell.columnLabel }]),
          rowsJson: JSON.stringify([{ id: cell.rowId, [cell.columnId]: { kind: "stepper", value: keyboardCase.value, min: cell.min, max: cell.max, step: cell.step, action } }]),
        },
      };
      const mounted = render(createElement(UiDriverProvider, { driver: DEFAULT_UI_DRIVER }, createElement(TableHost, { node: node as never, onAction: onAction as never })));
      const readout = mounted.container.querySelector(`[id="${node.surfaceId}.${cell.rowId}.${cell.columnId}"]`);
      if (!readout) throw new Error(`missing ${keyboardCase.id} spinbutton`);
      const propagated = fireEvent.keyDown(readout, { key: keyboardCase.key });
      expect(!propagated, keyboardCase.id).toBe(keyboardCase.consumed);
      if (keyboardCase.expectedDelta === null) {
        expect(onAction, keyboardCase.id).not.toHaveBeenCalled();
      } else {
        expect(onAction, keyboardCase.id).toHaveBeenCalledOnce();
        expect(onAction, keyboardCase.id).toHaveBeenCalledWith({ ...action, args: { ...action.args, delta: keyboardCase.expectedDelta } });
      }
      mounted.unmount();
    }
  });

  it("renders every GraphTimeline author in authored order with stable overlapping avatars", () => {
    const column = {
      checkpointId: graphTimelineAuthorsFixture.checkpointId,
      timestamp: "1",
      labels: ["main"],
      authors: graphTimelineAuthorsFixture.authors,
      description: "authored checkpoint",
      lane: 0,
      alternativeIds: [],
    };
    const mounted = render(
      createElement(GraphTimelineHost, {
        node: {
          type: "componentScene",
          surfaceId: "vcs.play.history.authors",
          controllerId: "vcs-play",
          componentKind: "graph-timeline",
          graphTimeline: { columnsJson: JSON.stringify([column]) },
        },
        onAction: noopAction,
      }),
    );
    const avatars = [...mounted.container.querySelectorAll<HTMLElement>('[data-slot="avatar"]')];
    expect(avatars.map((avatar) => avatar.id)).toEqual(graphTimelineAuthorsFixture.authors.map((author) => author.id));
    expect(avatars[0]?.parentElement?.classList.contains("-space-x-2")).toBe(true);
    expect(graphTimelineAuthorsFixture.avatarSize - graphTimelineAuthorsFixture.overlap).toBe(graphTimelineAuthorsFixture.advance);
    const image = avatars[0]?.querySelector<HTMLImageElement>('[data-slot="avatar-image"]');
    const fallback = avatars[0]?.querySelector<HTMLElement>('[data-slot="avatar-fallback"]');
    expect(image?.getAttribute("alt")).toBe("Ada Lovelace");
    expect(image?.getAttribute("src")).toBe(graphTimelineAuthorsFixture.authors[0]?.avatar);
    expect(image?.hidden).toBe(true);
    expect(fallback?.hidden).toBe(false);
    reactAct(() => image!.dispatchEvent(new Event("load")));
    expect(image?.hidden).toBe(false);
    expect(fallback?.hidden).toBe(true);
    expect(avatars[1]?.querySelector('[data-slot="avatar-image"]')).toBeNull();
    expect(avatars[1]?.querySelector('[data-slot="avatar-fallback"]')?.getAttribute("aria-label")).toBe("Grace Hopper");
    mounted.rerender(
      createElement(GraphTimelineHost, {
        node: {
          type: "componentScene",
          surfaceId: "vcs.play.history.authors",
          controllerId: "vcs-play",
          componentKind: "graph-timeline",
          graphTimeline: { columnsJson: JSON.stringify([{ ...column, authors: [{ ...column.authors[0], avatar: graphTimelineAuthorsFixture.replacementAvatar }, column.authors[1]] }]) },
        },
        onAction: noopAction,
      }),
    );
    const replacedImage = mounted.container.querySelector<HTMLImageElement>('#ada [data-slot="avatar-image"]');
    expect(replacedImage?.getAttribute("src")).toBe(graphTimelineAuthorsFixture.replacementAvatar);
    expect(replacedImage?.hidden).toBe(true);
  });

  it("renders vcs history host with an ancestor graph fork", () => {
    const columns = [
      {
        checkpointId: "c3",
        timestamp: "3",
        labels: ["feature-b"],
        authors: [],
        parentCheckpointId: "c2",
        description: "branch b",
        lane: 2,
        alternativeIds: ["b"],
      },
      {
        checkpointId: "c2",
        timestamp: "2",
        labels: ["feature-a"],
        authors: [],
        parentCheckpointId: "c1",
        description: "branch a",
        lane: 1,
        alternativeIds: ["a"],
      },
      {
        checkpointId: "c1",
        timestamp: "1",
        labels: ["main"],
        authors: [],
        description: "root",
        lane: 0,
        alternativeIds: [],
      },
    ];
    const markup = renderToStaticMarkup(
      createElement(GraphTimelineHost, {
        node: {
          type: "componentScene",
          surfaceId: "vcs.play.history",
          controllerId: "vcs-play",
          componentKind: "graph-timeline",
          graphTimeline: {
            columnsJson: JSON.stringify(columns),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-graph-timeline-host");
    expect(markup).toContain('id="vcs.play.history.table"');
    expect(markup).toContain('d="M ');
    expect(markup.match(/<circle /g)?.length).toBe(3);
    expect(markup).toContain("branch b");
    expect(markup).toContain("feature-b");
  });

  it.each(["complete", "cancel"])("raster compositor %s settles frame demand and keeps canvas theme", async (mode) => {
    await uiI18n.changeLanguage("en");
    const fixture=(await import("../../🧱️elements/🖌️Paint2dHost/🧫️fixtures/🔁️session/🔣️.json")).default;
    const uploadRasterImageKey=vi.fn(),syncDocumentJson=vi.fn(),attachCanvas=vi.fn(async()=>{}),setCamera=vi.fn();
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    let frames = 0,
      cancelled = false;
    const pending = () => !cancelled && (mode === "cancel" || frames < 12);
    const renderFrame = vi.fn(() => {
      frames++;
      return pending();
    });
    const cancelRender = vi.fn(() => {
      cancelled = true;
    });
    const setCanvasThemeJson = vi.fn();
    const session: flowSessionLoader.RasterWasmSession = {
      gpuReady: () => true,
      attachCanvas,
      setSize: () => {},
      renderFrame,
      setCamera,
      wheelScreen: () => {},
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      pointerCancelScreen: () => {},
      syncDocumentJson,
      uploadLayerImage: () => {},
      uploadRasterImageKey,
      setActiveUtility: () => {},
      setBrushSize: () => {},
      setBrushOpacity: () => {},
      syncInteraction: () => {},
      setCanvasThemeJson,
      cameraJson: () => '{"x":0,"y":0,"zoom":1}',
      setViewMode: () => {},
      pickTargetsAtScreenJson: () => "[]",
      marqueeHitsJson: () => "[]",
      navigatorFitCameraJson: () => '{"x":0,"y":0,"zoom":1}',
      navigatorViewportOverlayJson: () => '{"x":0,"y":0,"width":1,"height":1}',
      free: vi.fn(),
      renderProgressJson: () => JSON.stringify({ pending: pending(), completed: Math.min(frames, 12), total: 12, error: null }),
      cancelRender,
    };
    const factory = vi.spyOn(flowSessionLoader, "createRasterSession").mockResolvedValue(session);
    const originalObserver = globalThis.ResizeObserver;
    vi.stubGlobal(
      "ResizeObserver",
      class {
        observe() {}
        unobserve() {}
        disconnect() {}
      },
    );
    const node:UiComponentSceneNode={type:"componentScene",surfaceId:"raster.play.viewport",controllerId:"raster-play",componentKind:"paint-2d",paint2d:{documentSyncJson:JSON.stringify(fixture.document),assetsJson:JSON.stringify(fixture.assets),cameraJson:JSON.stringify(fixture.camera),selectionJson:"[]",activeUtility:"selectMarquee",brushSize:24,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,viewMode:"composite"}};
    const view=render(createElement(Paint2dHost,{node,onAction:noopAction}));
    try {
      await waitFor(() => expect(setCanvasThemeJson).toHaveBeenCalled());
      expect(setCanvasThemeJson.mock.calls.length).toBeGreaterThanOrEqual(2);
      await waitFor(()=>expect(uploadRasterImageKey).toHaveBeenCalledTimes(fixture.uploads));
      expect(syncDocumentJson).toHaveBeenCalledWith(JSON.stringify(fixture.document));
      expect(uploadRasterImageKey.mock.calls[0]).toEqual(["source",new Uint8Array(Buffer.from(fixture.assets.source.data,"base64"))]);
      expect(setCamera).toHaveBeenCalledWith(fixture.camera.x,fixture.camera.y,fixture.camera.zoom);
      if (mode === "cancel") {
        await waitFor(() => expect(view.getByRole("button", { name: "Cancel rendering" })).toBeTruthy());
        fireEvent.click(view.getByRole("button", { name: "Cancel rendering" }));
        expect(cancelRender).toHaveBeenCalled();
      } else await waitFor(() => expect(frames).toBeGreaterThanOrEqual(12));
      await waitFor(() => expect(view.container.querySelector('progress[aria-label="Rendering image"]')).toBeNull());
      view.rerender(createElement(Paint2dHost,{node:{...node,paint2d:{...node.paint2d!,cameraJson:JSON.stringify(fixture.updatedCamera),selectionJson:'["paint"]'}},onAction:noopAction}));
      await waitFor(()=>expect(setCamera).toHaveBeenCalledWith(fixture.updatedCamera.x,fixture.updatedCamera.y,fixture.updatedCamera.zoom));
      expect(attachCanvas).toHaveBeenCalledTimes(fixture.attachments);
      expect(uploadRasterImageKey).toHaveBeenCalledTimes(fixture.uploads);
      const settled = renderFrame.mock.calls.length;
      await new Promise((resolve) => setTimeout(resolve, 80));
      expect(renderFrame.mock.calls.length).toBe(settled);
    } finally {
      view.unmount();
      factory.mockRestore();
      canvasContext.mockRestore();
      vi.stubGlobal("ResizeObserver", originalObserver);
    }
    expect(session.free).toHaveBeenCalledOnce();
  });

  it("retires a mounted paint gesture on pointer cancel without an up or action commit", async () => {
    const attachCanvas = vi.fn(async () => {});
    const pointerDownScreen = vi.fn();
    const pointerUpScreen = vi.fn();
    const pointerCancelScreen = vi.fn();
    const onAction = vi.fn();
    const session: flowSessionLoader.RasterWasmSession = {
      gpuReady: () => true,
      attachCanvas,
      setSize: () => {},
      renderFrame: vi.fn(() => false),
      setCamera: () => {},
      wheelScreen: () => {},
      pointerDownScreen,
      pointerMoveScreen: () => {},
      pointerUpScreen,
      pointerCancelScreen,
      syncDocumentJson: () => {},
      uploadLayerImage: () => {},
      uploadRasterImageKey: () => {},
      setActiveUtility: () => {},
      setBrushSize: () => {},
      setBrushOpacity: () => {},
      syncInteraction: () => {},
      setCanvasThemeJson: () => {},
      cameraJson: () => '{"x":0,"y":0,"zoom":1}',
      setViewMode: () => {},
      pickTargetsAtScreenJson: () => "[]",
      marqueeHitsJson: () => "[]",
      navigatorFitCameraJson: () => '{"x":0,"y":0,"zoom":1}',
      navigatorViewportOverlayJson: () => '{"x":0,"y":0,"width":1,"height":1}',
      free: vi.fn(),
      renderProgressJson: () => JSON.stringify({ pending: false, completed: 0, total: 0, error: null }),
      cancelRender: () => {},
    };
    const factory = vi.spyOn(flowSessionLoader, "createRasterSession").mockResolvedValue(session);
    const originalObserver = globalThis.ResizeObserver;
    vi.stubGlobal(
      "ResizeObserver",
      class {
        observe() {}
        unobserve() {}
        disconnect() {}
      },
    );
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const view = render(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.cancel",
          controllerId: "raster",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "paintBrush",
            brushSize: 24,
            brushOpacity: 1,
            brushColor: "#2878dc",
            brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,
            viewMode: "composite",
          },
        },
        onAction,
      }),
    );
    try {
      const surface = view.container.querySelector('.semio-paint-2d-canvas-surface [class*="z-30"]') as HTMLElement;
      Object.defineProperty(surface, "hasPointerCapture", { configurable: true, value: () => false });
      await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
      const actionsBeforeCancel = onAction.mock.calls.length;
      fireEvent.pointerDown(surface, { pointerId: 7, button: 0, clientX: 10, clientY: 12 });
      expect(pointerDownScreen).toHaveBeenCalledOnce();
      fireEvent.pointerCancel(surface, { pointerId: 7, button: 0, clientX: 20, clientY: 22 });
      expect(pointerCancelScreen).toHaveBeenCalledOnce();
      expect(pointerUpScreen).not.toHaveBeenCalled();
      expect(onAction.mock.calls.slice(actionsBeforeCancel).map(([action]) => action.action)).toEqual(["interactionHover"]);
      fireEvent.pointerDown(surface, { pointerId: 8, button: 0, clientX: 30, clientY: 32 });
      expect(pointerDownScreen).toHaveBeenCalledTimes(2);
    } finally {
      view.unmount();
      factory.mockRestore();
      bounds.mockRestore();
      vi.stubGlobal("ResizeObserver", originalObserver);
    }
  });

  it("synchronizes raster selection and hover through the current native interaction API", async () => {
    const syncInteraction = vi.fn();
    const session: flowSessionLoader.RasterWasmSession = {
      gpuReady: () => true,
      attachCanvas: async () => {},
      setSize: () => {},
      renderFrame: () => false,
      setCamera: () => {},
      wheelScreen: () => {},
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      pointerCancelScreen: () => {},
      syncDocumentJson: () => {},
      uploadLayerImage: () => {},
      uploadRasterImageKey: () => {},
      setActiveUtility: () => {},
      setBrushSize: () => {},
      setBrushOpacity: () => {},
      syncInteraction,
      setCanvasThemeJson: () => {},
      cameraJson: () => '{"x":0,"y":0,"zoom":1}',
      setViewMode: () => {},
      pickTargetsAtScreenJson: () => "[]",
      marqueeHitsJson: () => "[]",
      navigatorFitCameraJson: () => '{"x":0,"y":0,"zoom":1}',
      navigatorViewportOverlayJson: () => '{"x":0,"y":0,"width":1,"height":1}',
      free: vi.fn(),
      renderProgressJson: () => JSON.stringify({ pending: false, completed: 0, total: 0, error: null }),
      cancelRender: () => {},
    };
    const factory = vi.spyOn(flowSessionLoader, "createRasterSession").mockResolvedValue(session);
    const originalObserver = globalThis.ResizeObserver;
    vi.stubGlobal(
      "ResizeObserver",
      class {
        observe() {}
        unobserve() {}
        disconnect() {}
      },
    );
    const content = (vector: (typeof presenceOverlayFixture.cases)[number]) =>
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.viewport",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: JSON.stringify(vector.expected.selected ? [vector.update.nodeKey] : []),
            hoveredId: vector.expected.hovered ? vector.update.nodeKey : undefined,
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            brushColor: "#2878dc",
            brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,
            viewMode: "composite",
          },
        },
        onAction: noopAction,
      });
    const view = render(content(presenceOverlayFixture.cases[0]!));
    try {
      for (const vector of presenceOverlayFixture.cases) {
        view.rerender(content(vector));
        await waitFor(() => expect(syncInteraction).toHaveBeenLastCalledWith(JSON.stringify(vector.expected.selected ? [vector.update.nodeKey] : []), vector.expected.hovered ? vector.update.nodeKey : null));
      }
    } finally {
      view.unmount();
      factory.mockRestore();
      vi.stubGlobal("ResizeObserver", originalObserver);
    }
    expect(session.free).toHaveBeenCalledOnce();
  });

  it("renders paint-2d host canvas surface from document sync scene", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.viewport",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            brushColor: "#2878dc",
            brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,
            viewMode: "composite",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-canvas-surface");
    expect(markup).toContain('data-surface-id="raster.play.viewport"');
    expect(markup).toContain('data-view-mode="composite"');
  });

  it("renders paint-2d navigator host with the composite viewport overlay channel", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.navigator",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            brushColor: "#2878dc",
            brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,
            viewMode: "navigator",
            compositeViewportJson: '{"width":640,"height":480}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-canvas-surface");
    expect(markup).toContain('data-view-mode="navigator"');
  });

  it("renders paint-2d host empty fallback without a scene", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.composite",
          controllerId: "raster-play",
          componentKind: "paint-2d",
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-empty");
  });

  it("validates and flattens the shared raw virtual file system interaction law", () => {
    for (const vector of virtualFileSystemInteractionFixture.visibility) {
      const rows = buildVirtualFileSystemSceneRows(virtualFileSystemInteractionFixture.rows, new Set(vector.expandedRowIds));
      expect(
        rows.map((row) => row.id),
        vector.id,
      ).toEqual(vector.visibleRowIds);
      expect(
        rows.map((row) => row.level),
        vector.id,
      ).toEqual(vector.levels);
    }
    for (const vector of virtualFileSystemInteractionFixture.navigation) {
      const row = virtualFileSystemInteractionFixture.rows.find((candidate) => candidate.id === vector.rowId)!;
      expect(virtualFileSystemNavigation(row), vector.rowId).toEqual(vector.action ? { action: vector.action, args: vector.args } : null);
    }
  });

  it("mounts localized raw virtual file system controls, owns expansion, and dispatches navigateUri only on double-click", async () => {
    const previousLocale = detectShellLocale(uiI18n.resolvedLanguage || uiI18n.language);
    await uiI18n.changeLanguage("en");
    const onAction = vi.fn();
    const view = render(
      createElement(VirtualFileSystemHost, {
        node: {
          type: "componentScene",
          surfaceId: "vfs.interaction",
          controllerId: "vfs-controller",
          componentKind: "virtual-file-system",
          virtualFileSystem: {
            schemaJson: JSON.stringify({
              fileNodeKinds: {
                root: { id: "root", name: "Root", descriptors: [] },
                branch: { id: "branch", name: "Branch", descriptors: [] },
                leaf: { id: "leaf", name: "Leaf", descriptors: [] },
              },
              descriptorKinds: {},
              descriptorColumnIds: [],
            }),
            rowsJson: JSON.stringify(virtualFileSystemInteractionFixture.rows),
          },
        },
        onAction,
      }),
    );
    try {
      const folder = view.container.querySelector('tr[data-row-id="folder"]') as HTMLElement;
      expect(folder).toBeTruthy();
      expect(view.container.querySelector('tr[data-row-id="instance"]')).toBeTruthy();
      const toggle = folder.querySelector("button[data-vfs-expand]") as HTMLButtonElement;
      const english = virtualFileSystemInteractionFixture.chrome.find((pack) => pack.locale === "en")!;
      expect(toggle.getAttribute("aria-label")).toBe(english.collapse);
      expect(toggle.getAttribute("aria-expanded")).toBe("true");
      fireEvent.click(toggle);
      expect(view.container.querySelector('tr[data-row-id="instance"]')).toBeNull();
      const collapsedToggle = view.container.querySelector('tr[data-row-id="folder"] button[data-vfs-expand]') as HTMLButtonElement;
      expect(collapsedToggle.getAttribute("aria-label")).toBe(english.expand);
      expect(collapsedToggle.getAttribute("aria-expanded")).toBe("false");
      fireEvent.click(collapsedToggle);
      const instance = view.container.querySelector('tr[data-row-id="instance"]') as HTMLElement;
      expect(instance).toBeTruthy();
      fireEvent.click(instance, { detail: 1 });
      fireEvent.click(instance, { detail: 2 });
      expect(onAction).toHaveBeenLastCalledWith({
        controllerId: "vfs-controller",
        action: "openInstance",
        args: { surfaceId: "vfs.interaction", instanceId: "inst-7" },
      });
      expect(onAction.mock.calls.filter(([action]) => action.action === "selectRows")).toHaveLength(1);
      const german = virtualFileSystemInteractionFixture.chrome.find((pack) => pack.locale === "de")!;
      await reactAct(async () => {
        await uiI18n.changeLanguage("de");
      });
      expect((view.container.querySelector('tr[data-row-id="folder"] button[data-vfs-expand]') as HTMLButtonElement).getAttribute("aria-label")).toBe(german.collapse);
    } finally {
      view.unmount();
      await uiI18n.changeLanguage(previousLocale);
    }
  });

  it("interprets virtual file system component scenes", async () => {
    // 🧬️ MIGRATION: `Component::Surface`'s single `SurfaceProps.doc` (a pack-encoded opaque payload)
    // replaces the old `componentScene`/`virtualFileSystem` field pair — `surfacePropsToComponentSceneNode`
    // (Interpreter/🟦️.tsx) decodes `doc.bytes` back into the exact scene sub-field shape
    // `VirtualFileSystemHost` reads.
    const doc = {
      schemaJson: JSON.stringify({
        fileNodeKinds: { instance: { id: "instance", name: "Instance", descriptors: [] } },
        descriptorKinds: {},
        descriptorColumnIds: [],
      }),
      rowsJson: JSON.stringify([{ id: "row-1", fileNodeKindId: "instance", name: "Draw", path: "/draw", level: 0 }]),
    };
    const docBytes = [13, 2];
    const pushLength = (value: number) => {
      for (let remaining = value; ; remaining = Math.floor(remaining / 128)) {
        docBytes.push((remaining % 128) | (remaining >= 128 ? 128 : 0));
        if (remaining < 128) break;
      }
    };
    for (const [key, value] of Object.entries(doc)) {
      const keyBytes = new TextEncoder().encode(key);
      const valueBytes = new TextEncoder().encode(value);
      docBytes.push(6);
      pushLength(keyBytes.length);
      docBytes.push(...keyBytes, 6);
      pushLength(valueBytes.length);
      docBytes.push(...valueBytes);
    }
    const markup = renderContractTree({
      key: "s.play.media-vfs",
      component: {
        type: "surface",
        kind: "virtual-file-system",
        docSchema: "virtual-file-system@1",
        doc: { bytes: docBytes },
        bindings: [],
      },
    });
    expect(markup).toContain("Draw");
  });
});

describe("dag marquee overlay", () => {
  it("computes a rect overlay with numeric bounds for the rectangle method", () => {
    const pointsJson = JSON.stringify([
      { x: 10, y: 20 },
      { x: 30, y: 50 },
    ]);
    const overlay = computeDagMarqueeOverlay(pointsJson, false, "rectangle");
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "full" });
  });

  // Regression: board rust publishes `[[x,y],…]` (not `{x,y}` objects). Object-only parsing yielded
  // NaN bounds so the live select marquee never painted even though selection itself worked.
  it("computes a rect overlay from the rust tuple-array wire format", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 20],
        [30, 50],
        [10, 50],
      ]),
      true,
      "rectangle",
    );
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "partial" });
  });

  it("computes a lasso overlay carrying the raw points for the lasso method", () => {
    const points = [
      { x: 10, y: 20 },
      { x: 30, y: 50 },
      { x: 15, y: 40 },
    ];
    const overlay = computeDagMarqueeOverlay(JSON.stringify(points), true, "lasso");
    expect(overlay).toEqual({ kind: "lasso", points, coverage: "partial" });
  });

  it("computes a lasso overlay from the rust tuple-array wire format", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 50],
        [15, 40],
      ]),
      false,
      "lasso",
    );
    expect(overlay).toEqual({
      kind: "lasso",
      points: [
        { x: 10, y: 20 },
        { x: 30, y: 50 },
        { x: 15, y: 40 },
      ],
      coverage: "full",
    });
  });

  it("infers lasso from a non-rectangular path when method is omitted", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 50],
        [15, 40],
      ]),
      false,
    );
    expect(overlay?.kind).toBe("lasso");
  });

  it("infers rectangle from four axis-aligned corner points when method is omitted", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 20],
        [30, 50],
        [10, 50],
      ]),
      true,
    );
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "partial" });
  });

  it("returns null for fewer than two points", () => {
    expect(computeDagMarqueeOverlay(JSON.stringify([{ x: 0, y: 0 }]), false, "rectangle")).toBeNull();
    expect(computeDagMarqueeOverlay(JSON.stringify([[0, 0]]), false, "rectangle")).toBeNull();
  });

  it("returns null for malformed point entries", () => {
    expect(computeDagMarqueeOverlay(JSON.stringify([[10], [20, 30]]), false, "rectangle")).toBeNull();
    expect(computeDagMarqueeOverlay(JSON.stringify([{ x: 10 }, { x: 20, y: 30 }]), false, "rectangle")).toBeNull();
  });

  // Regression: node-graph-host.tsx used to pass `shape={{ shape: "polygon", points }}` (a single
  // nested-object prop) instead of separate `shape`/`points` props, so `props.shape === "rect"` was
  // always false and the polygon branch read `props.points` as undefined — crashing on every marquee
  // drag and tripping the shell's render error boundary (visible as an interaction "reset").
  it("renders a rect overlay from a computeDagMarqueeOverlay rect result without crashing", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [0, 0],
        [40, 25],
      ]),
      false,
      "rectangle",
    );
    if (!overlay || overlay.kind !== "rect") throw new Error("expected rect overlay");
    const markup = renderToStaticMarkup(
      createElement(SelectionMarquee, {
        coverage: overlay.coverage ?? "full",
        shape: "rect",
        rect: { x: overlay.x ?? 0, y: overlay.y ?? 0, width: overlay.width ?? 0, height: overlay.height ?? 0 },
      }),
    );
    expect(markup).toContain("<rect");
    expect(markup).toContain('width="40"');
    expect(markup).toContain('height="25"');
  });

  it("renders a polygon overlay from a computeDagMarqueeOverlay lasso result without crashing", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [0, 0],
        [40, 25],
        [5, 30],
      ]),
      false,
      "lasso",
    );
    if (!overlay || overlay.kind !== "lasso") throw new Error("expected lasso overlay");
    const markup = renderToStaticMarkup(createElement(SelectionMarquee, { coverage: overlay.coverage ?? "full", shape: "polygon", points: overlay.points ?? [] }));
    expect(markup).toContain("<polygon");
    expect(markup).toContain("0,0 40,25 5,30");
  });
});

describe("ink canvas host", () => {
  const semioInkDocument: InkDocument = {
    schema: "ink.document",
    id: "semio",
    title: "Semio Note",
    camera: { x: 0, y: 0, zoom: 1 },
    gridVisible: true,
    snapEnabled: false,
    pencilWidth: 3,
    eraserRadius: 12,
    blocks: [
      {
        kind: "text",
        id: "welcome-text",
        name: "Welcome",
        x: 80,
        y: 80,
        width: 360,
        height: 120,
        visible: true,
        locked: false,
        paragraphs: [{ runs: [{ text: "Welcome to Note — an infinite canvas for text, images, tables, math, and pencil ink." }] }],
        fontSize: 20,
        fontWeight: "normal",
        align: "left",
      },
      { kind: "math", id: "welcome-math", name: "Equation", x: 80, y: 240, width: 240, height: 80, visible: true, locked: false, tex: "E = mc^2", displayMode: true },
      {
        kind: "table",
        id: "welcome-table",
        name: "Blocks",
        x: 80,
        y: 360,
        width: 360,
        height: 140,
        visible: true,
        locked: false,
        columns: ["Block", "Description"],
        rows: [
          [{ content: "Text" }, { content: "Rich text blocks" }],
          [{ content: "Math" }, { content: "TeX equations" }],
          [{ content: "Ink" }, { content: "Freehand pencil strokes" }],
        ],
      },
    ],
  };

  it("renders the semio example composite scene with rich text, table, and math fallback", () => {
    const markup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: {
          type: "componentScene",
          surfaceId: "note.play.composite",
          controllerId: "note-play",
          componentKind: "ink-canvas",
          inkCanvas: {
            documentJson: JSON.stringify(semioInkDocument),
            selectionJson: "[]",
            activeUtility: "selectDirect",
            viewMode: "composite",
            interactive: true,
          },
        },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(markup).toContain("Welcome to Note");
    expect(markup).toContain("<table");
    expect(markup).toMatch(/\$\$E = mc\^2\$\$|annotation encoding="application\/x-tex">E = mc\^2</);
    expect(markup).toContain('data-surface-id="note.play.composite"');
  });

  it("shows the grid pattern in composite mode but not in navigator mode", () => {
    const baseNode = {
      type: "componentScene" as const,
      surfaceId: "note.play.composite",
      controllerId: "note-play",
      componentKind: "ink-canvas",
    };
    const compositeMarkup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: { ...baseNode, inkCanvas: { documentJson: JSON.stringify(semioInkDocument), selectionJson: "[]", activeUtility: "selectDirect", viewMode: "composite", interactive: true } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(compositeMarkup).toContain("ink-viewport-grid");

    const navigatorMarkup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: { ...baseNode, inkCanvas: { documentJson: JSON.stringify(semioInkDocument), selectionJson: "[]", activeUtility: "selectDirect", viewMode: "navigator", interactive: false } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(navigatorMarkup).not.toContain("ink-viewport-grid");
  });

  it("resizes with a minimum size and scales ink points when a group is resized", () => {
    const fromBounds = { x: 0, y: 0, width: 100, height: 100 };
    const shrunk = inkResizeBounds(fromBounds, "e", -1000, 0);
    expect(shrunk.width).toBe(8);

    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 0,
      y: 0,
      width: 100,
      height: 100,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [100, 100],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    const scaled = inkScaleItemWithinGroup(ink, { x: 0, y: 0, width: 100, height: 100 }, { x: 0, y: 0, width: 200, height: 50 });
    expect(scaled.kind).toBe("stroke");
    if (scaled.kind === "stroke")
      expect(scaled.points).toEqual([
        [0, 0],
        [200, 50],
      ]);
  });

  it("splits an ink stroke into fragments when erasing its middle point", () => {
    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 0,
      y: 0,
      width: 80,
      height: 1,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [40, 0],
        [80, 0],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    const fragments = eraseInkStrokePointsInItem(ink, 40, 0, 5);
    expect(fragments).toHaveLength(0);
    const wideStroke: InkStrokeItem = {
      ...ink,
      points: [
        [0, 0],
        [10, 0],
        [40, 0],
        [70, 0],
        [80, 0],
      ],
    };
    const splitFragments = eraseInkStrokePointsInItem(wideStroke, 40, 0, 5);
    expect(splitFragments).toHaveLength(2);
  });

  it("round-trips bold and link marks between paragraphs and html", () => {
    const html = inkParagraphsToHtml([{ runs: [{ text: "hello", bold: true, link: "https://semio.tech" }] }]);
    expect(html).toContain("<strong>");
    expect(html).toContain('href="https://semio.tech"');
  });

  it("round-trips a clipboard payload of note blocks", () => {
    const payload = inkClipboardPayload([semioInkDocument.blocks[1]!]);
    const parsed = inkItemsFromClipboardPayload(payload);
    expect(parsed).toHaveLength(1);
    expect(parsed?.[0]?.kind).toBe("math");
  });

  it("computes ink block bounds from its local points", () => {
    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 10,
      y: 10,
      width: 1,
      height: 1,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [5, 5],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    expect(inkItemBounds(ink)).toEqual({ x: 10, y: 10, width: 5, height: 5 });
  });

  it("applies the canonical wheel-zoom camera formula symmetrically for screen<->world conversion", () => {
    const camera = { x: 50, y: 50, zoom: 2 };
    const world = screenToWorld(camera, 150, 150);
    expect(world).toEqual([50, 50]);
    expect(worldToScreen(camera, 50, 50)).toEqual({ x: 150, y: 150 });
  });
});

describe("spawned window chrome", () => {
  const app = {
    id: "cad-play",
    label: "CAD",
    breadcrumb: ["semio", "cad"],
    controllerId: "cad-play",
    defaultModeId: "edit",
    modes: [
      {
        id: "edit",
        label: "Edit",
        utilities: [{ id: "static-utility", kind: "button" as const, iconId: "save", controllerId: "cad-play", action: "save" }],
      },
    ],
    windowKinds: [
      {
        id: "cad-window-shape",
        label: "Shape",
        bodyKey: "shape",
        options: {
          engagement: {
            kind: "some" as const,
            value: {
              input: {
                id: "engagement-input",
                placeholder: "Action",
                onChange: { controllerId: "cad-play", action: "engagementInput" },
              },
              possibleEngagements: [{ id: "box", label: "Box", action: { controllerId: "cad-play", action: "startBox" } }],
            },
          },
          measures: [{ id: "render-mode", kind: "select" as const, label: "Render Mode", value: "shaded", items: [], onChange: { controllerId: "cad-play", action: "setRenderMode" } }],
        },
      },
    ],
    panelTabs: [],
    keybindings: [],
  };

  it("builds spawned engagement and measures chrome from program contributions", () => {
    const kind = app.windowKinds[0]!;
    const engagements = {
      [kind.id]: {
        input: {
          id: "engagement-input",
          value: "Box",
          placeholder: "Action",
          onChange: { controllerId: "cad-play", action: "engagementInput" },
        },
        possibleEngagements: [{ id: "box", label: "Box", detail: "b", action: { controllerId: "cad-play", action: "startBox" } }],
      },
    };
    const measures = { [kind.id]: kind.options.measures ?? [] };
    const chrome = spawnedWindowChromeForKind(kind, kind.id, engagements, measures, undefined, noopAction);
    expect(chrome.search?.input?.value).toBe("Box");
    expect(chrome.search?.possibles?.[0]?.label).toBe("Box");
    const measuresMarkup = renderToStaticMarkup(chrome.measures as ReactElement);
    expect(measuresMarkup).toContain("Render Mode");
  });
});

describe("partitionWindowMeasures", () => {
  const utilityGroup = (id: string, activeUtilityId: string | undefined, children: WindowMeasure[] = []): WindowMeasure => ({ kind: "group", id, label: id, activeUtilityId, children });
  const slider = (id: string): WindowMeasure => ({ kind: "slider", id, value: 1, min: 0, max: 2, onChange: { controllerId: "c", action: "a" } });

  it("unwraps a tagged group's children into utilityOptions only when its utility is active", () => {
    const measures = [utilityGroup("brush-params", "brush", [slider("size")]), slider("zoom")];
    const active = partitionWindowMeasures(measures, "brush");
    expect(active.utilityOptions.map((m) => m.id)).toEqual(["size"]);
    expect(active.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("drops a tagged group from both buckets when a different or no utility is active", () => {
    const measures = [utilityGroup("brush-params", "brush", [slider("size")]), slider("zoom")];
    const other = partitionWindowMeasures(measures, "fill");
    expect(other.utilityOptions).toEqual([]);
    expect(other.general.map((m) => m.id)).toEqual(["zoom"]);
    const none = partitionWindowMeasures(measures, undefined);
    expect(none.utilityOptions).toEqual([]);
    expect(none.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("keeps untagged groups and non-group measures in general, unaffected by the active utility", () => {
    const measures = [utilityGroup("grid", undefined), slider("zoom")];
    const { general, utilityOptions } = partitionWindowMeasures(measures, "brush");
    expect(general.map((m) => m.id)).toEqual(["grid", "zoom"]);
    expect(utilityOptions).toEqual([]);
  });

  it("wires a utility-scoped group into spawnedWindowChromeForKind's utilityOptions slot only when its utility is active", () => {
    const kind = { id: "w", label: "W", bodyKey: "b", surfaceKind: "paint-2d", options: { engagement: { kind: "none" as const }, measures: [] } } as unknown as AppWindowKindDefinition;
    const brushGroup: WindowMeasure = {
      kind: "group",
      id: "brush-params",
      label: "Brush",
      defaultOpen: true,
      activeUtilityId: "brush",
      children: [{ kind: "slider", id: "size", label: "Brush size", value: 4, min: 1, max: 10, onChange: { controllerId: "c", action: "setSize" } }],
    };
    const measures = { [kind.id]: [brushGroup] };
    const activeChrome = spawnedWindowChromeForKind(kind, kind.id, {}, measures, "brush", noopAction);
    const activeMarkup = renderToStaticMarkup(activeChrome.utilityOptions as ReactElement);
    expect(activeMarkup).toContain("Brush size");
    expect(activeMarkup).toContain('data-direction="up"');
    expect(activeChrome.measures).toBeUndefined();
    const idleChrome = spawnedWindowChromeForKind(kind, kind.id, {}, measures, "fill", noopAction);
    expect(idleChrome.utilityOptions).toBeUndefined();
    expect(idleChrome.measures).toBeUndefined();
  });

  it("parses the real ui_wgpu camelCase wire JSON and unwraps a utility-scoped fill group into flat utilityOptions (snake_case divergence regression guard)", () => {
    // Verbatim shape of `ui_wgpu::WindowMeasure`'s serde wire after the D-4 `rename_all_fields = "camelCase"`
    // fix: a fill-utility slider group tagged with `activeUtilityId`, plus an untagged toggle. This is the exact
    // class of payload whose snake_case↔camelCase divergence made the puzzle fill slider invisible in React.
    const wireJson =
      '[{"kind":"group","id":"fill-params","label":"Fill","activeUtilityId":"fill","children":[{"kind":"slider","id":"fillCount","label":"Count","value":3,"min":1,"max":9,"step":1,"onChange":{"controllerId":"puzzle","action":"setFillCount"}}]},{"kind":"toggle","id":"grid","iconId":"layout-grid","pressed":true,"onChange":{"controllerId":"puzzle","action":"toggleGrid"}}]';
    const measures = JSON.parse(wireJson) as WindowMeasure[];
    const { general, utilityOptions } = partitionWindowMeasures(measures, "fill");
    expect(utilityOptions.map((m) => m.id)).toEqual(["fillCount"]);
    expect(utilityOptions[0]).toMatchObject({ kind: "slider", id: "fillCount", onChange: { action: "setFillCount" } });
    expect(general.map((m) => m.id)).toEqual(["grid"]);
    const gridToggle = general[0];
    expect(gridToggle.kind === "toggle" && gridToggle.iconId).toBe("layout-grid");

    // Regression guard for the fixed bug: the pre-fix snake_case wire leaves `activeUtilityId` undefined, so the
    // tagged group silently falls through to `general` and the fill slider never reaches the Utility Options rail.
    const legacyJson = wireJson
      .replace(/"activeUtilityId"/g, '"active_utility_id"')
      .replace(/"onChange"/g, '"on_change"')
      .replace(/"iconId"/g, '"icon_id"');
    const legacy = JSON.parse(legacyJson) as WindowMeasure[];
    const legacyPartition = partitionWindowMeasures(legacy, "fill");
    expect(legacyPartition.utilityOptions).toEqual([]);
    expect(legacyPartition.general.map((m) => m.id)).toEqual(["fill-params", "grid"]);
  });
});

describe("utility ribbon", () => {
  it("sorts utility nodes by order", () => {
    const sorted = sortUtilityNodes([
      { id: "b", kind: "button", iconId: "box", order: 2, controllerId: "x", action: "b" },
      { id: "a", kind: "button", iconId: "box", order: 1, controllerId: "x", action: "a" },
    ]);
    expect(sorted.map((node) => node.id)).toEqual(["a", "b"]);
  });

  it("recurses into a collection level only when the path names one of its collections", () => {
    const tree = [
      {
        id: "view",
        kind: "collection",
        iconId: "eye",
        children: [
          {
            id: "view-tools",
            kind: "collection",
            iconId: "zoom-in",
            children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
          },
        ],
      },
      {
        id: "construct",
        kind: "collection",
        iconId: "box",
        children: [
          {
            id: "construct-tools",
            kind: "collection",
            iconId: "box",
            children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
          },
        ],
      },
    ] satisfies UtilityNode[];

    const noActive = buildUtilityRibbonSegments(tree, []);
    expect(noActive).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);

    const oneActive = buildUtilityRibbonSegments(tree, ["construct"]);
    expect(oneActive[0]).toMatchObject({ kind: "picker", depth: 0 });
    expect(oneActive[1]).toMatchObject({ kind: "picker", depth: 1, collections: tree[1].children });
    expect(oneActive).toHaveLength(2);

    const twoActive = buildUtilityRibbonSegments(tree, ["construct", "construct-tools"]);
    const utilitiesSegment = twoActive.find((segment) => segment.kind === "utilities" && segment.items.some((item) => item.id === "box"));
    expect(utilitiesSegment).toMatchObject({ depth: 2 });
  });

  it("ignores a path entry that no longer names an enabled collection at that level", () => {
    const tree: UtilityNode[] = [
      {
        id: "view",
        kind: "collection",
        iconId: "eye",
        children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
      },
    ];
    expect(buildUtilityRibbonSegments(tree, ["nonexistent"])).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);
  });

  it("emits a picker segment alongside loose leaves at the same depth", () => {
    const segments = buildUtilityRibbonSegments(
      [
        { id: "undo", kind: "button", iconId: "undo", controllerId: "x", action: "undo" },
        {
          id: "view",
          kind: "collection",
          iconId: "eye",
          children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
        },
      ],
      [],
    );
    expect(segments).toEqual([
      { kind: "picker", collections: [expect.objectContaining({ id: "view" })], depth: 0 },
      { kind: "utilities", items: [expect.objectContaining({ id: "undo" })], depth: 0 },
    ]);
  });

  it("reconciles an active path by truncating at the first stale entry instead of substituting a default", () => {
    const tree: UtilityNode[] = [
      {
        id: "a",
        kind: "collection",
        iconId: "box",
        children: [
          { id: "x", kind: "collection", iconId: "box", children: [{ id: "leaf", kind: "button", iconId: "box", controllerId: "c", action: "act" }] },
          { id: "y", kind: "collection", iconId: "box", children: [] },
        ],
      },
      { id: "b", kind: "collection", iconId: "box", children: [] },
    ];
    expect(reconcileUtilityPath(tree, ["a", "x"])).toEqual(["a", "x"]);
    expect(reconcileUtilityPath(tree, ["a", "gone"])).toEqual(["a"]);
    expect(reconcileUtilityPath(tree, ["gone"])).toEqual([]);
    expect(reconcileUtilityPath(tree, [])).toEqual([]);
  });

  it("buckets top-level utility nodes into ordered category collections (uncategorized nodes default to tools now that the Actions category is gone)", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "sel", kind: "toggle", iconId: "mouse-pointer", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
      { id: "act", kind: "button", iconId: "sparkles", controllerId: "x", action: "run" },
      { id: "tool", kind: "toggle", iconId: "pencil", controllerId: "x", action: "pen" },
      { id: "sync", kind: "toggle", iconId: "cloud", controllerId: "x", action: "sync", category: "sync" },
    ]);
    expect(grouped.map((node) => node.id)).toEqual(["selection", "utilities", "history", "sync"]);
    expect(grouped.every((node) => node.kind === "collection")).toBe(true);
  });

  it("drops separator-only category buckets so an empty group never appears as a picker option", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "utilities" },
      { id: "sep", kind: "separator" },
    ]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0].id).toBe("utilities");
  });

  it("reuses a category's single already-meaningful collection instead of re-wrapping it, avoiding a duplicate-looking picker level", () => {
    const selectionCollection: Extract<UtilityNode, { kind: "collection" }> = {
      id: "lowpoly-tools-selection",
      kind: "collection" as const,
      iconId: "mouse-pointer",
      label: "Selection",
      category: "selection" as const,
      children: [{ id: "mesh", kind: "toggle" as const, iconId: "box", controllerId: "x", action: "mesh" }],
    };
    const grouped = groupUtilityNodesByCategory([selectionCollection]);
    expect(grouped).toEqual([{ ...selectionCollection, order: 0 }]);
    const segments = buildUtilityRibbonSegments(grouped, ["lowpoly-tools-selection"]);
    const utilitiesSegment = segments.find((segment) => segment.kind === "utilities" && segment.items.some((item) => item.id === "mesh"));
    expect(utilitiesSegment).toBeTruthy();
  });

  it("still wraps a category with multiple top-level nodes in a synthetic collection", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "utilities" },
      { id: "b", kind: "button", iconId: "box", controllerId: "x", action: "b", category: "utilities" },
    ]);
    expect(grouped).toEqual([{ id: "utilities", kind: "collection", iconId: "wrench", text: "utilities", order: 0, category: "utilities", children: expect.any(Array) }]);
  });

  it("scopes grouping to the given categories only", () => {
    const nodes: UtilityNode[] = [
      { id: "sel", kind: "toggle", iconId: "mouse-pointer", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
    ];
    expect(groupUtilityNodesByCategory(nodes, ["selection", "utilities"]).map((node) => node.id)).toEqual(["selection"]);
    expect(groupUtilityNodesByCategory(nodes, ["utilities", "history"]).map((node) => node.id)).toEqual(["history"]);
  });

  it("deduplicates utility nodes by id across window utility lists for a single shared footer entry", () => {
    const history: UtilityNode = { id: "s-play.history", kind: "collection", iconId: "clock", category: "history", children: [] };
    const deduped = dedupeUtilityNodesById([[history, { id: "leaf-a", kind: "button" as const, iconId: "box", controllerId: "x", action: "a" }], [history], []]);
    expect(deduped).toEqual([history, { id: "leaf-a", kind: "button", iconId: "box", controllerId: "x", action: "a" }]);
  });

  it("renders utility ribbon with picker and batched toggles", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        utilities: [
          {
            id: "view",
            kind: "collection",
            iconId: "eye",
            children: [
              {
                id: "view-tools",
                kind: "collection",
                iconId: "eye",
                children: [
                  { id: "show-edges", kind: "toggle", iconId: "box", pressed: true, controllerId: "x", action: "edges" },
                  { id: "show-faces", kind: "toggle", iconId: "square", pressed: false, controllerId: "x", action: "faces" },
                ],
              },
            ],
          },
          {
            id: "construct",
            kind: "collection",
            iconId: "box",
            children: [
              {
                id: "construct-tools",
                kind: "collection",
                iconId: "box",
                children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
              },
            ],
          },
        ],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('id="ui.utilities"');
    expect(markup).toContain('data-slot="toggle-group"');
  });

  it("stacks the window utility bar ribbon upward, showing only the base picker row until a group is activated", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        direction: "up",
        utilities: [
          {
            id: "view",
            kind: "collection",
            iconId: "eye",
            children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
          },
          {
            id: "construct",
            kind: "collection",
            iconId: "box",
            children: [
              {
                id: "construct-tools",
                kind: "collection",
                iconId: "box",
                children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
              },
            ],
          },
        ],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('data-slot="ribbon"');
    expect(markup).toContain('data-direction="up"');
    expect(markup).toContain("flex-col-reverse");
    // No active path given, so neither group is expanded: exactly one ribbon row (the base picker).
    expect(markup.match(/data-slot="ribbon-row"/g)?.length).toBe(1);
    expect(markup).toContain('data-slot="toggle-group"');
    expect(markup).not.toContain('id="zoom-in"');
  });

  it("renders UtilityTree with a custom id for per-window namespacing", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        id: "ui.utilities.model",
        utilities: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('id="ui.utilities.model"');
    expect(markup).not.toContain('id="ui.utilities"');
  });

  it("renders utilityOptions as an extra ribbon row when direction is up", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        id: "ui.utilities.w",
        direction: "up",
        utilities: [{ id: "brush", kind: "toggle", iconId: "paintbrush", pressed: true, controllerId: "x", action: "setActiveUtility" }],
        utilityOptions: createElement("span", { "data-testid": "brush-options" }, "Brush size"),
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('data-testid="brush-options"');
    expect(markup).toContain("Brush size");
    expect(markup).toContain('data-variable-height="true"');
    expect(markup).toContain("h-auto min-h-medium");
    expect(markup).toContain("items-start");
    const variableZoneClass = markup.match(/data-variable-height="true" class="([^"]*)"/)?.[1].split(" ") ?? [];
    expect(variableZoneClass).toContain("h-auto");
    expect(variableZoneClass).not.toContain("h-medium");
    const optionItemClass = [...markup.matchAll(/data-slot="ribbon-item" class="([^"]*)"/g)].at(-1)?.[1].split(" ") ?? [];
    expect(optionItemClass).toEqual(expect.arrayContaining(["h-auto", "items-start", "w-full", "max-w-full"]));
  });
});

describe("s workflow flow routing", () => {
  it("does not render a flow frame before its surface is ready", () => {
    expect(flowSurfaceRenderAllowed(false)).toBe(false);
    expect(flowSurfaceRenderAllowed(true)).toBe(true);
  });

  it("selects the flow engine for scenes with engine flow capabilities", () => {
    expect(isFlowGraphScene('{"engine":"flow","spotlight":false,"noteEdit":false}')).toBe(true);
    expect(isFlowGraphScene('{"spotlight":false,"noteEdit":false,"clusters":false}')).toBe(false);
    expect(isFlowGraphScene(undefined)).toBe(false);
  });

  it("renders presence peers from the scene payload", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
            presencePeersJson: JSON.stringify([{ clientId: "client-b", name: "Ada", selectionCount: 2 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("Ada");
    expect(markup).toContain("2 selected");
  });

  it("parses a catalogue app drag payload, ignoring extra keys", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ pluginId: "s.system", appId: "draw", label: "Draw", extra: "x" }))).toEqual({
      pluginId: "s.system",
      appId: "draw",
      label: "Draw",
    });
  });

  it("rejects catalogue app drag payloads missing pluginId/appId, and garbage", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ appId: "draw" }))).toBeNull();
    expect(parseCatalogueAppDragPayload(JSON.stringify({ kind: "neuron" }))).toBeNull();
    expect(parseCatalogueAppDragPayload("not json")).toBeNull();
  });

  it("builds a ghost neuron descriptor, preferring label over appId", () => {
    expect(JSON.parse(catalogueGhostDescriptorJson({ pluginId: "s.system", appId: "draw", label: "Draw" }))).toEqual({ kind: "neuron", neuronKind: "Draw" });
    expect(JSON.parse(catalogueGhostDescriptorJson({ pluginId: "s.system", appId: "draw" }))).toEqual({ kind: "neuron", neuronKind: "draw" });
  });

  it("builds addWidget descriptors from catalogue items", () => {
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕️", summary: "" }))).toEqual({
      kind: "neuron",
      neuronKind: "math.add",
    });
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "outputExport", format: "svg", name: "Export SVG", abbreviation: "SVG", icon: "emoji:📤️", summary: "" }))).toEqual({
      kind: "outputExport",
      format: "svg",
    });
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "inputSlider", name: "Slider", abbreviation: "Slider", icon: "emoji:🎚️", summary: "" }))).toEqual({ kind: "inputSlider", label: "Slider" });
  });

  it("ranks catalogue suggestions by exact/prefix match with neurons first", () => {
    const sections = [
      {
        id: "inputs",
        title: "Inputs",
        items: [
          { kind: "inputSlider", name: "Slider", abbreviation: "Slider", icon: "emoji:🎚️", summary: "" },
          { kind: "inputNote", name: "Note", abbreviation: "Note", icon: "emoji:📝️", summary: "" },
        ],
      },
      {
        id: "math",
        title: "Math",
        items: [
          { kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕️", summary: "" },
          { kind: "neuron", neuronKind: "math.subtract", name: "Subtract", abbreviation: "Sub", icon: "emoji:➖️", summary: "" },
        ],
      },
    ];
    expect(flowRankCatalogueSuggestions(sections, "add").map((item) => item.neuronKind ?? item.kind)).toEqual(["math.add"]);
    expect(flowRankCatalogueSuggestions(sections, "sl").map((item) => item.kind)).toEqual(["inputSlider"]);
    const brepSections = [
      {
        id: "brep",
        title: "Brep",
        items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦️", summary: "Axis-aligned box" }],
      },
    ];
    expect(flowRankCatalogueSuggestions(brepSections, "brep").map((item) => item.neuronKind ?? item.kind)).toEqual(["brep.prim3d.box"]);
    expect(flowRankCatalogueSuggestions(brepSections, "box").map((item) => item.neuronKind ?? item.kind)).toEqual(["brep.prim3d.box"]);
    const empty = flowRankCatalogueSuggestions(sections, "");
    expect(empty[0]?.kind).toBe("neuron");
    expect(empty.some((item) => item.kind === "inputSlider")).toBe(true);
  });

  it("returns every catalogue item for empty query without a 20-item cap", () => {
    const items = Array.from({ length: 25 }, (_, index) => ({
      kind: "neuron",
      neuronKind: `math.operation${index}`,
      name: `Operation ${index}`,
      abbreviation: `Operation${index}`,
      icon: "emoji:➕️",
      summary: "",
    }));
    const sections = [{ id: "math", title: "Math", items }];
    expect(flowRankCatalogueSuggestions(sections, "")).toHaveLength(25);
  });

  it("enables overflow scrolling only when spotlight suggestions are expanded", () => {
    expect(flowSpotlightSuggestionListScrollClass(false)).toContain("overflow-hidden");
    expect(flowSpotlightSuggestionListScrollClass(false)).not.toContain("overflow-y-auto");
    expect(flowSpotlightSuggestionListScrollClass(true)).toContain("overflow-y-auto");
    expect(flowSpotlightSuggestionListScrollClass(true)).toContain("max-h-[min(24rem,70vh)]");
  });

  it("hosts semantic panel bodies full-width via tree emptyState, not a property-layout control wrapper", () => {
    const config = uiNodeToTreePanelConfig(pendingPanelUiNode(), noopAction, "framework.panel.inspection");
    expect(config.sections).toEqual([]);
    expect(config.emptyState).toBeTruthy();
    expect(config.className).toContain("w-full");
  });

  // (measured 2026-09-09 21:05). The guest cannot render an empty inspection body
  // (`selected_object_inspector_renders_that_object_field_group` in the puzzle3d crate proves a pick
  // yields the object field group, and `render` always falls back to the document summary), and the
  // panel leaf above proves a delivered tree body renders. What was left was the third state: a body
  // the shell has NOT received yet is `pendingPanelUiNode()` — a `tree` node with `activity: "loading"`
  // and no children — and `<Tree sections={[]}/>` drew literally nothing for it. Loading, empty and
  // dropped were one and the same blank rectangle.
  it("renders a panel body that has not arrived yet as a loading surface, never as a silently empty panel", () => {
    const rendered = render(panelTreePanelHost(uiNodeToTreePanelConfig(pendingPanelUiNode(), noopAction, "framework.panel.inspection")));
    expect(rendered.container.querySelector('[data-ui-status="loading"]')).toBeTruthy();
    expect(rendered.container.querySelector('[aria-busy="true"]')).toBeTruthy();
  });

  // 🈳️ The blank-panel state: a tree body that IS settled (`activity: "idle"`) and resolves to zero
  // `treeSection` children used to render `<Tree sections={[]}/>`, i.e. literally nothing — no ring, no
  // text, no marker — which is indistinguishable from a body that was dropped on the way in. It now says
  // so. A guest-rendered panel body always carries at least one section (`PanelTreeBuilder::build`), so
  // reaching this state at all means something upstream lost the body.
  it("renders an idle tree body with no sections as an explicit empty state, distinct from loading", () => {
    const idleEmpty = { ...pendingPanelUiNode(), activity: "idle" as const };
    const empty = render(panelTreePanelHost(uiNodeToTreePanelConfig(idleEmpty, noopAction, "framework.panel.inspection")));
    expect(empty.container.querySelector('[data-ui-status="loading"]')).toBeNull();
    expect(empty.container.textContent?.replace(/\u2026/g, "").trim()).not.toBe("");
    expect(empty.container.textContent).toContain(resolveTranslationLabel(uiI18n.t("ui.common.noData")));
  });

  // 🐢️ The host half of the puzzle3d scope table (`puzzle3d_command_scope_class`): a partial scope that
  // NAMES the inspection panel body is what puts that panel in the batched `refresh-ui` request. A
  // partial scope that names only window bodies — which is what every puzzle3d partial scope used to be
  // — leaves the inspector out of the request entirely, so it keeps whatever it last rendered.
  it("a partial scope naming a panel body requests that panel, and one that omits it does not", () => {
    const leaves = [
      { kind: { kind: "app" as const, id: "framework.panel.artifact" }, bodyKey: "puzzle.3d.play.artifact" },
      { kind: { kind: "app" as const, id: "framework.panel.inspection" }, bodyKey: "puzzle.3d.play.inspector" },
    ];
    const windows = [{ id: "puzzle3d-main", bodyKey: "puzzle3d.play.composite" }];
    const named = buildUiRefreshRequest({ kind: "partial", windowBodies: ["puzzle3d.play.composite"], panelBodies: ["puzzle.3d.play.inspector", "puzzle.3d.play.artifact"], measures: true }, windows, leaves, {}, new Map());
    expect(named?.panels?.map((panel) => panel.key)).toEqual(["framework.panel.artifact", "framework.panel.inspection"]);
    const omitted = buildUiRefreshRequest({ kind: "partial", windowBodies: ["puzzle3d.play.composite"], panelBodies: [] }, windows, leaves, {}, new Map());
    expect(omitted?.panels).toEqual([]);
  });

  it("hosts a semantic tree document inside a panel leaf", () => {
    const config = uiNodeToTreePanelConfig(
      buildContractNode({
        key: "catalogue",
        component: { type: "tree", interactionDomain: null },
        children: [
          {
            key: "catalogue.section",
            component: { type: "treeSection", label: "Catalogue", defaultOpen: true, headerToolbar: null, window: null },
            children: [
              {
                key: "s-play-catalogue.document.draw",
                component: {
                  type: "treeItem",
                  label: "Draw",
                  description: null,
                  icon: null,
                  defaultOpen: null,
                  draggable: true,
                  dragData: { "application/x-semio-catalogue-item": '{"pluginId":"s.system","appId":"draw"}' },
                  dimmed: null,
                  selected: null,
                  window: null,
                  granularity: null,
                  inlineToolbar: null,
                  detail: null,
                  rowActions: [], target: null,
                },
              },
            ],
          },
        ],
      }),
      noopAction,
      "framework.panel.catalogue",
    );
    const rendered = render(panelTreePanelHost(config));
    expect(rendered.getByText("Draw")).toBeTruthy();
  });

  // 🛍️ The catalogue row's OWN add gesture: `object_kind_item` (`📌️panels/🛍️catalogue/🦀️.rs`) authors an
  // expandable `treeItem` — the kind's rim-vortex templates are its children — that ALSO binds
  // `activate` to `addObjectKind` with its own `{objectKind}` args. Battery #48 measured
  // `catalogue-add-object-kind before=1 after=1` while `catalogue-drag-drop` (a different route into the
  // same command) passed, so the question this pins is whether a click on an EXPANDABLE authored row
  // reaches the action channel at all, or is swallowed as a fold toggle.
  it("fires an expandable catalogue row's own target activation, args and all, instead of only folding it", () => {
    const dispatched: ActionDescriptor[] = [];
    const config = uiNodeToTreePanelConfig(
      buildContractNode({
        key: "puzzle3d-play-kinds",
        component: { type: "tree", interactionDomain: null },
        children: [
          {
            key: "puzzle3d-play-kinds.objects",
            component: { type: "treeSection", label: "Objects", defaultOpen: true, headerToolbar: null, window: null },
            children: [
              {
                key: "Hexagonal Cut Concrete Forest Left",
                component: {
                  type: "treeItem",
                  label: "Hexagonal Cut Concrete Forest Left",
                  description: "Hexagonal Cut Concrete Forest Left",
                  icon: "box",
                  defaultOpen: false,
                  draggable: true,
                  dragData: { "application/x-semio-catalogue-item": '{"objectKind":"Hexagonal Cut Concrete Forest Left"}' },
                  dimmed: null,
                  selected: null,
                  window: null,
                  granularity: null,
                  inlineToolbar: null,
                  detail: null,
                  rowActions: [], target: { scope: "puzzle3d-play", version: 1, args: { objectKind: "Hexagonal Cut Concrete Forest Left" }, activation: "addObjectKind" },
                },
                children: [
                  {
                    key: "puzzle3d-kind-vortex.0.b-l",
                    component: { type: "treeItem", label: "b-l", description: "[4,4,3]", icon: "circle-dot", defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null },
                  },
                ],
              },
            ],
          },
        ],
      }),
      (action) => dispatched.push(action),
      "framework.panel.catalogue",
    );
    const rendered = render(panelTreePanelHost(config));
    const row = rendered.container.querySelector("#panel\\:puzzle3d-play-kinds\\/Hexagonal\\ Cut\\ Concrete\\ Forest\\ Left");
    expect(row).toBeTruthy();
    // 🖱️ The row must announce its own activation in the DOM, so a red says WHICH hop broke rather than
    // "nothing happened" — `data-activatable` is `Boolean(TreeDataItem.onClick)` reaching the shell.
    expect(row?.getAttribute("data-activatable")).toBe("true");
    const expected = { controllerId: "puzzle3d-play", action: "addObjectKind", args: { objectKind: "Hexagonal Cut Concrete Forest Left" } };
    fireEvent.click(row as Element);
    expect(dispatched).toEqual([expected]);
    // 🏷️ And through the LABEL, which is what a real pointer lands on: a click on the row's text must
    // reach the same action rather than being spent folding the group open.
    const label = row?.querySelector('[data-slot="tree-label"]');
    expect(label).toBeTruthy();
    fireEvent.click(label as Element);
    expect(dispatched).toEqual([expected, expected]);
  });

  it("bridges semantic intent scope, version, args, and input into the plugin action channel", () => {
    const intent = {
      surface: "panel:settings",
      revision: 1,
      node: 7,
      nodeKey: "spacing",
      trigger: "change",
      action: { scope: "puzzle3d-play", name: "setSpacing", version: 1 },
      args: { axis: "x", value: 1 },
      input: { value: 2 },
      seq: 1n,
    } as UiIntent;
    expect(uiIntentToActionDescriptor(intent)).toEqual({ controllerId: "puzzle3d-play", action: "setSpacing", args: { axis: "x", value: 2 } });
  });

  it("names a scalar intent payload after its trigger and keeps the authored arguments", () => {
    // 🪜️ A `NumberStepper`'s `onChange` reports the number itself, and the node's authored args carry the
    // window the panel is tuning. The scalar must arrive UNDER `value` — every guest command reads named
    // arguments — and must not evict `windowId` on the way (ticket 26/09/02/PUZZLE-3D-END-TO-END B47 §2).
    const base = {
      surface: "panel:puzzle3d-play-settings",
      revision: 1,
      node: 7,
      nodeKey: "puzzle3d-play-settings.grid-spacing",
      action: { scope: "puzzle3d-play", name: "setGridSpacing", version: 1 },
      args: { windowId: "puzzle3d-main-top" },
      seq: 1n,
    };
    expect(uiIntentToActionDescriptor({ ...base, trigger: "change", input: 10.5 } as UiIntent)).toEqual({
      controllerId: "puzzle3d-play",
      action: "setGridSpacing",
      args: { windowId: "puzzle3d-main-top", value: 10.5 },
    });
    // ➕️➖️ …and a relative bump under `delta`, which is the other half of `puzzle3d_absolute_or_delta`.
    expect(uiIntentToActionDescriptor({ ...base, trigger: "delta", input: -0.5 } as UiIntent)).toEqual({
      controllerId: "puzzle3d-play",
      action: "setGridSpacing",
      args: { windowId: "puzzle3d-main-top", delta: -0.5 },
    });
    // 🧾️ A scalar with no authored args is still named, never a bare primitive.
    expect(uiIntentToActionDescriptor({ ...base, args: null, trigger: "commit", input: "fill 12" } as UiIntent)).toEqual({
      controllerId: "puzzle3d-play",
      action: "setGridSpacing",
      args: { value: "fill 12" },
    });
  });

  it("carries an outliner row action's explicit ids to the action channel whatever else is selected", () => {
    // 🙈️ `with_hide_lock_actions` (`📌️panels/🗿️artifact/🦀️.rs`) authors each inline toggle as a set-verb
    // (`setSelectionHidden`/`setSelectionLocked`) over the row's ONE target `{entity, hidden, ids, locked}` — the explicit
    // next state and the entity it flags — so the guest's explicit branch never consults the live selection. Battery #59 measured `outliner-hide-applies` FAIL with the row's
    // object selected (`historyUpserts: 0, effects: 0`) and PASS with nothing selected, which made "the
    // host dropped the args under a selection" a live candidate (26/09/02/PUZZLE-3D-END-TO-END B47 §6).
    // This pins the host half: the same authored map reaches `ActionDescriptor` with and without a
    // selection overlay, so a future red here means the HOST lost them and a red only in the browser
    // means the guest did.
    const targetArgs = { entity: "object", hidden: true, ids: ["seed-left-001"], locked: true };
    const documentNode = buildContractNode({
      key: "puzzle3d-play-document",
      component: { type: "tree", interactionDomain: "puzzle3d" },
      bindings: [{ trigger: "activate", action: { scope: "puzzle3d-play", name: "interactionSelect", version: 1 }, args: { domainId: "puzzle3d" }, capability: null }],
      children: [
        {
          key: "puzzle3d-play-document.objects",
          component: { type: "treeSection", label: "Objects", defaultOpen: true, headerToolbar: null, window: null },
          children: [
            {
              key: "seed-left-001",
              component: {
                type: "treeItem",
                label: "Hexagonal Cut Concrete Forest Left",
                description: null,
                icon: "box",
                defaultOpen: null,
                draggable: null,
                dragData: null,
                dimmed: null,
                selected: null,
                window: null,
                granularity: "object",
                inlineToolbar: null,
                detail: null,
                rowActions: [{ icon: "eye", label: "Hide", verb: "setSelectionHidden", placement: "row", disabled: false }],
                target: { scope: "puzzle3d-play", version: 1, args: targetArgs, activation: null },
              },
            },
          ],
        },
      ],
    });
    const expected = { controllerId: "puzzle3d-play", action: "setSelectionHidden", args: targetArgs };
    for (const presence of [undefined, { "seed-left-001": { selected: true }, "object-1": { selected: true } }]) {
      const dispatched: ActionDescriptor[] = [];
      const config = uiNodeToTreePanelConfig(documentNode, (action) => dispatched.push(action), "framework.panel.document");
      const host = panelTreePanelHost(config);
      const rendered = render(presence ? createElement(UiPresenceOverlayContext.Provider, { value: { byKey: new Map(Object.entries(presence)) as ReadonlyMap<string, UiPresenceOverlayEntry> } }, host) : host);
      const hide = Array.from(rendered.container.querySelectorAll('[data-slot="action"]')).find((candidate) => (candidate.textContent ?? "").includes("Hide"));
      expect(hide).toBeTruthy();
      fireEvent.click(hide as Element);
      expect(dispatched).toEqual([expected]);
      cleanup();
    }
  });

  it("resolves a fixture widget id to its workflow instance id, independent of selection state", () => {
    const snapshotJson = JSON.stringify({
      widgets: [
        { id: "widget-1", params: { instanceId: "app-1" } },
        { id: "widget-2", params: {} },
      ],
    });
    expect(resolveHostSnapshotWidgetInstanceId(snapshotJson, "widget-1")).toBe("app-1");
    expect(resolveHostSnapshotWidgetInstanceId(snapshotJson, "widget-2")).toBeUndefined();
    expect(resolveHostSnapshotWidgetInstanceId(snapshotJson, "missing-widget")).toBeUndefined();
    expect(resolveHostSnapshotWidgetInstanceId(snapshotJson, undefined)).toBeUndefined();
    expect(resolveHostSnapshotWidgetInstanceId(undefined, "widget-1")).toBeUndefined();
    expect(resolveHostSnapshotWidgetInstanceId("not json", "widget-1")).toBeUndefined();
  });

  it("classifies shell routes into landing, space, and notFound", () => {
    expect(parseShellRoute("/")).toEqual({ kind: "landing" });
    expect(parseShellRoute("/spaces/my-studio")).toEqual({ kind: "space", spaceId: "my-studio", instanceId: undefined });
    expect(parseShellRoute("/spaces/my-studio/instances/inst-1")).toEqual({ kind: "space", spaceId: "my-studio", instanceId: "inst-1" });
    expect(parseShellRoute("/unknown/path")).toEqual({ kind: "notFound", path: "/unknown/path" });
    expect(parseShellRoute("/spaces/my-studio/instances/inst-1/extra")).toEqual({ kind: "notFound", path: "/spaces/my-studio/instances/inst-1/extra" });
  });

  // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3/§C6 — pure-function
  // coverage of the identity/directory helpers `ShellHost/🟦️.tsx` exports for this. Full
  // end-to-end coverage (a real `openDocument` binding snapshot with/without a resolved identity, an
  // `os.open-artifact{documentId}` effect resulting in a worker `open` request with hub+folder
  // bindings) would need `fetch`/`Worker`/`DirectoryClient` mocking this already-huge shared suite has
  // no existing pattern for; not added this lane — see `📓️w2-c-report.md`.
  it("shellActorId mints user:{userId}#{sessionId} once identity resolves, else client-{sessionId}", () => {
    expect(shellActorId("sess-1", null)).toBe("client-sess-1");
    expect(shellActorId("sess-1", { userId: "u-1", email: "u1@semio.dev", displayName: "U1", hubBaseUrl: "http://127.0.0.1:8787", issuedAtMs: 0 })).toBe("user:u-1#sess-1");
  });

  it("canonicalSurfaceId formats <kind>@<standard>/<subset>#<role>", () => {
    expect(canonicalSurfaceId({ artifactKind: "s.space.space", standard: "1", subset: "*" }, "editor")).toBe("s.space.space@1/*#editor");
    expect(canonicalSurfaceId({ artifactKind: "s.space.space", standard: "1", subset: "*" }, "viewer")).toBe("s.space.space@1/*#viewer");
  });

  it("reloadRetainsActiveApp accepts extension-only programs and rejects a dropped active app", () => {
    expect(reloadRetainsActiveApp([], undefined)).toBe(true);
    expect(reloadRetainsActiveApp([{ id: "active" }], "active")).toBe(true);
    expect(reloadRetainsActiveApp([], "active")).toBe(false);
  });

  it("directoryCommandFromAction maps all 7 frozen os.directory.* ids, share-link sugaring to create-invite", () => {
    expect(directoryCommandFromAction("os.directory.create-space", { name: "Atelier", spaceKind: "atelier", visibility: "private" })).toEqual({
      kind: "create-space",
      name: "Atelier",
      spaceKind: "atelier",
      visibility: "private",
    });
    expect(directoryCommandFromAction("os.directory.delete-space", { spaceId: "sp-1" })).toEqual({ kind: "delete-space", spaceId: "sp-1" });
    expect(directoryCommandFromAction("os.directory.rename-space", { spaceId: "sp-1", name: "New" })).toEqual({ kind: "rename-space", spaceId: "sp-1", name: "New" });
    expect(directoryCommandFromAction("os.directory.set-visibility", { spaceId: "sp-1", visibility: "public" })).toEqual({ kind: "set-visibility", spaceId: "sp-1", visibility: "public" });
    expect(directoryCommandFromAction("os.directory.upsert-member", { spaceId: "sp-1", email: "a@b.com", role: "author" })).toEqual({
      kind: "upsert-member",
      spaceId: "sp-1",
      email: "a@b.com",
      role: "author",
    });
    expect(directoryCommandFromAction("os.directory.remove-member", { spaceId: "sp-1", userId: "u-1" })).toEqual({ kind: "remove-member", spaceId: "sp-1", userId: "u-1" });
    expect(directoryCommandFromAction("os.directory.share-link", { spaceId: "sp-1", role: "spectator", ttlSecs: 60 })).toEqual({
      kind: "create-invite",
      spaceId: "sp-1",
      role: "spectator",
      ttlSecs: 60,
    });
    expect(directoryCommandFromAction("os.unknownVerb", {})).toBeNull();
  });

  it("mints a 32-hex nonzero correlation and retains bounded, request-id-keyed command results", () => {
    const first = mintDirectoryCommandRequestId();
    expect(first).toMatch(/^(?!0{32}$)[0-9a-f]{32}$/u);
    expect(mintDirectoryCommandRequestId()).not.toBe(first);

    const slots = new Map<string, DirectoryCommandResultSlotV1>();
    const receipt = {
      schema: "semio.directory.command-receipt.v1",
      requestId: first,
      commandSha256: "a".repeat(64),
      outcome: "accepted",
      events: [],
      result: { kind: "invite", inviteToken: "invite.v1.one-shot" },
      receiptSha256: "b".repeat(64),
    } as unknown as DirectoryCommandReceiptV1;
    retainDirectoryCommandResult(slots, first, { kind: "receipt", receipt });
    expect(slots.get(first)).toEqual({ kind: "receipt", receipt });
    retainDirectoryCommandResult(slots, first, { kind: "failed", code: "forbidden" });
    expect(slots.size).toBe(1);
    expect(slots.get(first)).toEqual({ kind: "failed", code: "forbidden" });

    for (let index = 0; index < DIRECTORY_COMMAND_RESULT_SLOTS + 4; index += 1) {
      retainDirectoryCommandResult(slots, index.toString(16).padStart(32, "c"), { kind: "failed", code: "request-conflict" });
    }
    expect(slots.size).toBe(DIRECTORY_COMMAND_RESULT_SLOTS);
    expect(slots.has(first)).toBe(false);
    expect(slots.has((DIRECTORY_COMMAND_RESULT_SLOTS + 3).toString(16).padStart(32, "c"))).toBe(true);
  });

  // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C5 — save/check-in policy
  // (lane 3-A). `AutoCheckinScheduler` is deliberately framework-free (see its own doc) so its
  // debounce/storm-guard behaviour is verifiable with fake timers directly, without mounting
  // `ShellHost` — the same "no Worker/fetch/DirectoryClient mocking pattern exists yet" ceiling
  // `📓️w2-c-report.md` already documented applies here too; `ShellHost`'s own wiring of this
  // scheduler (and the pill's rendering into the sync tab) is reviewed, not click/mount-tested.
  describe("AutoCheckinScheduler (§C5 auto check-in)", () => {
    afterEach(() => {
      vi.useRealTimers();
    });

    it("3 edits then idle ⇒ exactly one commitCheckpoint", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(1);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS - 1);
      scheduler.notify(2);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS - 1);
      scheduler.notify(3);
      expect(onCheckpoint).not.toHaveBeenCalled();
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      // 🎯️ "never a storm": more time passing without a fresh `notify` never fires a second time.
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 3);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
    });

    it("≥ 200 uncommitted edits ⇒ checkpoint without waiting for idle", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD - 1);
      expect(onCheckpoint).not.toHaveBeenCalled();
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      // 🎯️ "never a storm": a second `notify` at/above the threshold before the checkpoint's own
      // `notify(0)` lands must not fire again.
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD + 1);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
    });

    it("notify(0) (a landed checkpoint) clears the pending latch for a fresh idle window later", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      scheduler.notify(0);
      scheduler.notify(1);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS);
      expect(onCheckpoint).toHaveBeenCalledTimes(2);
    });

    it("cancel() stops a pending idle timer (unmount/session-switch)", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(1);
      scheduler.cancel();
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 2);
      expect(onCheckpoint).not.toHaveBeenCalled();
    });

    it("an automatic check-in waits while its document cannot take it and is asked for again, per the shared corpus (live finding O4)", () => {
      for (const row of automaticCheckinCorpus.waits) expect(automaticCheckinWaitsV1(row.document), row.id).toBe(row.waits);
      for (const timeline of automaticCheckinCorpus.timelines) {
        vi.useFakeTimers();
        const start = Date.now();
        let document = { loading: false, attached: false, bound: false };
        const dispatched: number[] = [];
        const deferred: number[] = [];
        const scheduler: AutoCheckinScheduler = new AutoCheckinScheduler(() => {
          if (automaticCheckinWaitsV1(document)) {
            deferred.push(Date.now() - start);
            scheduler.defer();
          } else dispatched.push(Date.now() - start);
        }, automaticCheckinCorpus.idleMs, automaticCheckinCorpus.threshold);
        for (const event of timeline.events as readonly { readonly notify?: number; readonly advance?: number; readonly document?: typeof document }[]) {
          if (event.document !== undefined) document = event.document;
          else if (event.notify !== undefined) scheduler.notify(event.notify);
          else vi.advanceTimersByTime(event.advance ?? 0);
        }
        expect([dispatched, deferred], timeline.id).toEqual([timeline.dispatched, timeline.deferred]);
        scheduler.cancel();
        vi.useRealTimers();
      }
    });
  });

  describe("sync status pill (§C5 status pill, ArtifactSyncStatus → persisted|pending(n)|remote(...))", () => {
    it("persisted: a live remote with nothing pending", () => {
      const state = computeSyncPillState({ persisted: true, pendingMutations: 0, remote: { kind: "live", peerCount: 1 } });
      expect(state).toEqual({ kind: "persisted" });
      expect(syncPillText(state, "en")).toBe("Persisted");
      expect(syncPillText(state, "de")).toBe("Gespeichert");
    });

    it("pending(n): a live remote with unacked mutations", () => {
      const state = computeSyncPillState({ persisted: false, pendingMutations: 3, remote: { kind: "live", peerCount: 1 } });
      expect(state).toEqual({ kind: "pending", count: 3 });
      expect(syncPillText(state, "en")).toBe("Pending (3)");
      expect(syncPillText(state, "de")).toBe("Ausstehend (3)");
    });

    it("remote(connecting|backoff|detached): a non-live remote takes priority over a pending count", () => {
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "connecting" } }), "en")).toBe("Remote: connecting");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "connecting" } }), "de")).toBe("Remote: verbindet");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "backoff", retryInMs: 500 } }), "en")).toBe("Remote: backoff");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 9, remote: { kind: "backoff", retryInMs: 500 } }), "en")).toBe("Remote: backoff");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "detached" } }), "en")).toBe("Remote: detached");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "detached" } }), "de")).toBe("Remote: getrennt");
    });

    it("no status observed yet reads as remote(detached)", () => {
      expect(computeSyncPillState(null)).toEqual({ kind: "remote", remote: "detached" });
    });
  });

  // 🧪️ §C5 item 5: "viewers never checkpoint" — `canCheckIn` is the SAME predicate `ShellHost` gates
  // both the `#s-checkin`/checkpoint footer items (JSX presence, `!canCheckIn(session.app.role)`) and
  // the auto-checkin scheduler's arming (`isEditorSession`) with, so this one test covers both call
  // sites' logic without needing to mount `ShellHost` itself.
  it("canCheckIn is true only for an editor role — viewer gets no affordance and no auto timer", () => {
    expect(canCheckIn("editor")).toBe(true);
    expect(canCheckIn("viewer")).toBe(false);
    expect(canCheckIn(undefined)).toBe(false);
  });

  it("isolates render faults in ShellFaultBoundary", () => {
    function FaultyChild(): ReactElement {
      throw new Error("boom");
    }
    const { getByRole } = render(createElement(ShellFaultBoundary, { boundaryId: "test", fallbackLabel: uiDataLabel("Fault"), children: createElement(FaultyChild) }));
    expect(getByRole("alert").textContent).toContain("boom");
  });

  it("folds spawned focus into viewState so a subsequent host-effect session write keeps activeSpawnedId", async () => {
    const panel = { activePanelTab: "s-play-catalogue", spawnedApps: [] as const };
    const spawned = { id: "app-draw-1", pluginId: "draw", instanceId: 1, appId: "draw", label: "Semio Emblem", breadcrumb: ["draw"] };
    const focused = studioPanelFocusingSpawned(panel, spawned);
    expect(focused.activeSpawnedId).toBe("app-draw-1");
    expect(focused.spawnedApps).toEqual([spawned]);
    // 🐚️ Simulate applyHostEffects: fold into nextViewState, then a final SET_SESSION commits that
    // viewState (the bug was committing the pre-spawn viewState and wiping activeSpawnedId).
    const baseViewState = { panelJson: JSON.stringify(panel) };
    const nextViewState = viewStateWithSpacePanel(baseViewState, focused);
    const { parsePanelState } = await import("../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts");
    expect(parsePanelState(nextViewState)?.activeSpawnedId).toBe("app-draw-1");
    const refocused = studioPanelFocusingSpawned(focused, { ...spawned, label: "Renamed" });
    expect(refocused.spawnedApps).toHaveLength(1);
    expect(refocused.spawnedApps[0]?.label).toBe("Renamed");
    expect(refocused.activeSpawnedId).toBe("app-draw-1");
  });
});

describe("ui search/find (fuse re-export from @semio-tech/ui-react)", () => {
  // Command dialogs render via a Radix Portal into `document.body`, not into the render() container, so assertions query `document.body`.
  // This package's vitest config has no shared setupFile, so tests here clean up their own portal-rendered DOM.
  afterEach(async () => {
    const { cleanup } = await import("@semio-tech/ui-react/test");
    cleanup();
  });

  it("UISearch renders all items and fuzzy-filters them through the owned ranker", async () => {
    const { render, fireEvent } = await import("@semio-tech/ui-react/test");
    const selected = vi.fn();
    const openChange = vi.fn();
    const items: UISearchItem[] = [
      { id: "a", label: "Alpha", category: "Test", onSelect: selected },
      { id: "b", label: "Bravo", category: "Test", onSelect: noopAction },
    ];
    render(createElement(UIFindProvider, null, createElement(UISearch, { items, open: true, onOpenChange: openChange })));
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).toContain("Bravo");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "alp" } });
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).not.toContain("Bravo");
    fireEvent.keyDown(input, { key: "Enter" });
    expect(openChange).toHaveBeenCalledWith(false);
    expect(selected).toHaveBeenCalledTimes(1);
  });

  it("UIFind renders and fuzzy-filters items registered on its context through the owned ranker", async () => {
    const { render, fireEvent, act } = await import("@semio-tech/ui-react/test");
    let contextValue: ReturnType<typeof useUIFind> | undefined;
    const Harness = () => {
      contextValue = useUIFind();
      return createElement(UIFind, { open: true, onOpenChange: noopAction });
    };
    render(createElement(UIFindProvider, null, createElement(Harness)));
    act(() => {
      contextValue!.setFindItems([
        { id: "1", label: "Chair", category: "Test" },
        { id: "2", label: "Table", category: "Test" },
      ]);
    });
    const selected = vi.fn();
    act(() => contextValue!.setOnFindItem(selected));
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).toContain("Table");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "cha" } });
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).not.toContain("Table");
    fireEvent.keyDown(input, { key: "Enter" });
    expect(selected).toHaveBeenCalledWith("1");
  });
});

// 🧰️ Window Actions & Utilities Contract (WS-2): staged argument forms (P1/P2), palette redirect (P3),
// keybinding rule (P4), and registry-derived utility activation (P5).

describe("window action panel — staging and single dispatch (P1/P2)", () => {
  afterEach(() => cleanup());

  const numberArg = (id: string, required: boolean, def?: number): ResolvedActionArgDef => ({ id, label: id[0]!.toUpperCase() + id.slice(1), schema: { kind: "number", integer: false }, required, ...(def === undefined ? {} : { default: def }) });

  const twoArgAction: ResolvedActionDefinition = { id: "extrude", label: "Extrude", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [numberArg("depth", true), numberArg("segments", true)] };
  const zeroArgAction: ResolvedActionDefinition = { id: "flatten", label: "Flatten", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [] };
  const defaultedAction: ResolvedActionDefinition = { id: "bevel", label: "Bevel", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [numberArg("radius", true, 2)] };

  function Harness({ actions, onExecute, disabled }: { actions: readonly ResolvedActionDefinition[]; onExecute: (descriptor: unknown) => void; disabled?: boolean }): ReactElement {
    const [expanded, setExpanded] = useState<string | null>(null);
    const [staged, setStaged] = useState<Record<string, Record<string, unknown>>>({});
    return createElement(WindowActionPane, {
      windowId: "w1",
      controllerId: "c",
      actions,
      expandedActionId: expanded,
      stagedArgsByKey: staged,
      disabled: Boolean(disabled),
      onExpandedChange: setExpanded,
      onStageArg: (actionId, argId, value) => setStaged((prev) => ({ ...prev, [actionStageKey("w1", actionId)]: { ...(prev[actionStageKey("w1", actionId)] ?? {}), [argId]: value } })),
      onResetArgs: (actionId) =>
        setStaged((prev) => {
          const next = { ...prev };
          delete next[actionStageKey("w1", actionId)];
          return next;
        }),
      onExecute,
    });
  }

  const buttonByText = (container: HTMLElement, text: string): HTMLButtonElement => {
    const match = [...container.querySelectorAll("button")].find((button) => button.textContent?.includes(text));
    if (!match) throw new Error(`button "${text}" not found`);
    return match as HTMLButtonElement;
  };

  // 🌳️ Action rows render as Tree items (`role="treeitem"`), not `<button>`s — only the Execute/Reset
  // form actions render as real buttons. Row clicks (fire a zero-arg action, toggle an arg-carrying one)
  // go through this helper instead of `buttonByText`.
  const rowByText = (container: HTMLElement, text: string): HTMLElement => {
    const match = [...container.querySelectorAll('[role="treeitem"]')].find((row) => row.querySelector('[data-slot="tree-label"]')?.textContent?.trim() === text);
    if (!match) throw new Error(`tree row "${text}" not found`);
    return match as HTMLElement;
  };

  it("stages both args locally, dispatches nothing until Execute, then fires exactly one merged descriptor and keeps staged values", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [twoArgAction], onExecute }));
    fireEvent.click(rowByText(container, "Extrude…"));
    const inputs = container.querySelectorAll('input[type="number"]');
    expect(inputs).toHaveLength(2);
    fireEvent.change(inputs[0]!, { target: { value: "3" } });
    fireEvent.change(inputs[1]!, { target: { value: "2" } });
    expect(onExecute).not.toHaveBeenCalled();
    fireEvent.click(buttonByText(container, "Execute"));
    expect(onExecute).toHaveBeenCalledTimes(1);
    expect(onExecute).toHaveBeenCalledWith({ controllerId: "c", action: "extrude", args: { depth: 3, segments: 2 } });
    // staged values survive Execute (tweak-and-repeat): the inputs still hold their values
    expect((container.querySelectorAll('input[type="number"]')[0] as HTMLInputElement).value).toBe("3");
    fireEvent.click(buttonByText(container, "Execute"));
    expect(onExecute).toHaveBeenCalledTimes(2);
  });

  it("gates Execute on required args, but a default-satisfied required arg counts without staging", () => {
    const onExecute = vi.fn();
    const unavailable = (button: HTMLButtonElement) => [button.getAttribute("aria-disabled") === "true", button.disabled];
    const required = render(createElement(Harness, { actions: [twoArgAction], onExecute }));
    fireEvent.click(rowByText(required.container, "Extrude…"));
    expect(unavailable(buttonByText(required.container, "Execute")), "a gated Execute stays focusable, aria-disabled").toEqual([true, false]);
    fireEvent.click(buttonByText(required.container, "Execute"));
    expect(onExecute).not.toHaveBeenCalled();
    const inputs = required.container.querySelectorAll('input[type="number"]');
    fireEvent.change(inputs[0]!, { target: { value: "3" } });
    expect(unavailable(buttonByText(required.container, "Execute"))).toEqual([true, false]);
    fireEvent.change(inputs[1]!, { target: { value: "2" } });
    expect(unavailable(buttonByText(required.container, "Execute"))).toEqual([false, false]);
    cleanup();

    const defaulted = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(rowByText(defaulted.container, "Bevel…"));
    expect(unavailable(buttonByText(defaulted.container, "Execute"))).toEqual([false, false]);
    fireEvent.click(buttonByText(defaulted.container, "Execute"));
    expect(onExecute).toHaveBeenLastCalledWith({ controllerId: "c", action: "bevel", args: { radius: 2 } });
  });

  it("Reset restores defaults while keeping the form expanded", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(rowByText(container, "Bevel…"));
    const input = () => container.querySelector('input[type="number"]') as HTMLInputElement;
    expect(input().value).toBe("2");
    fireEvent.change(input(), { target: { value: "9" } });
    expect(input().value).toBe("9");
    fireEvent.click(buttonByText(container, "Reset"));
    // still expanded (Execute/Reset buttons present) and back to the default effective value
    expect(input().value).toBe("2");
    expect([...container.querySelectorAll("button")].some((b) => b.textContent?.includes("Execute"))).toBe(true);
  });

  it("a zero-arg action row fires immediately with no args object", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute }));
    fireEvent.click(rowByText(container, "Flatten"));
    expect(onExecute).toHaveBeenCalledTimes(1);
    expect(onExecute).toHaveBeenCalledWith({ controllerId: "c", action: "flatten" });
  });

  it.each(["en", "de"] as const)("guest utility assignments preserve armed tools and window scope (%s)", (locale) => {
    const invalid = structuredClone(utilityAssignmentCases);
    Object.assign(invalid.cases[0]!.steps[0]!, { kind: "unknown" });
    for (const row of utilityAssignmentCases.cases) {
      let state = initialShellState({ plugins: [], locks: { locale, terminology: "native" }, storage: createMemoryStoragePort() });
      for (const step of row.steps) {
        const utilityId = step.kind === "press" ? resolveAssignmentPress(state.actionPane.activeUtilityByWindowId[step.windowId], step.utilityId) : step.utilityId || null;
        state = shellReducer(state, { type: "SET_ACTIVE_UTILITY", windowId: step.windowId, utilityId });
        expect(Object.fromEntries(utilityAssignmentCases.windowIds.map((window) => [window, state.actionPane.activeUtilityByWindowId[window] ?? null])), row.id).toEqual(step.expected);
      }
    }
    console.log(`[DEBUG] React ${locale} actual shell reducer preserves twenty explicit assignments and user presses across two window scopes`);
  });

  it("Draw utility action and interruption fixtures conform to their closed contracts", () => {
    const ajv = new Ajv2020({ strict: true });
    const policy = ajv.compile(drawingActionSchema);
    const interruption = ajv.compile(drawingInterruptionSchema);
    expect(policy(drawingActionCases)).toBe(true);
    for (const row of drawingInterruptionCases.cases) {
      expect(interruption(row.press)).toBe(true);
      expect(interruption(row.move)).toBe(true);
      expect(drawingActionCases.utilities.some(utility => utility.id === row.utility)).toBe(true);
    }
    expect(policy({ ...drawingActionCases, utilities: drawingActionCases.utilities.map((utility) => ({ ...utility, allowsActionsWhileActive: false })) })).toBe(false);
    expect(policy({ ...drawingActionCases, utilities: drawingActionCases.utilities.map(() => drawingActionCases.utilities[0]) })).toBe(false);
    const invalid = structuredClone(drawingInterruptionCases);
    Object.assign(invalid.cases[0]!, { utility: "unknown" });
    expect(drawingActionCases.utilities.some(utility => utility.id === invalid.cases[0]!.utility)).toBe(false);
    expect(interruption([1])).toBe(false);
    console.log("[DEBUG] Independent Ajv2020 validates actual Draw producer and gesture-interruption fixtures, rejecting false or duplicate arms and unknown gestures");
  });

  it.each(["en", "de"] as const)("initial window utility decisions follow declared arms and preserve resolved registers (%s)", async (locale) => {
    const { resolveInitialWindowUtility } = await import("../../../../../../../🔨️modules/🛂️manifest/🪛️utilities/🌅️initial/🟦️.ts");
    const { createVersionedRegisterV1 } = await import("../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts");
    const ajv = new Ajv2020({ strict: true, $data: true });
    const fixture: unknown = initialWindowUtilityCases;
    for (const row of fixture.cases) {
      const authored = { utilities: row.utilityIds, ...(row.initialUtilityId === null ? {} : { initialUtilityId: row.initialUtilityId }) };
      const actual = resolveInitialWindowUtility(row);
      expect(actual, row.id).toEqual(row.expected);
      const register = createVersionedRegisterV1<string | null>(row.activeUtilityByWindowId[row.windowId] ?? null);
      const before = register.read();
      if (actual.write) expect(register.write(actual.utilityId, null).kind).toBe("applied");
      else expect(register.read()).toBe(before);
      expect(register.read().value).toBe(row.expected.utilityId);
      if (actual.utilityId !== null) {
        const rendered = register.read();
        expect(register.write(null, rendered.generation).kind).toBe("applied");
        const cleared = { ...row, activeUtilityByWindowId: { ...row.activeUtilityByWindowId, [row.windowId]: null } };
        expect(resolveInitialWindowUtility(cleared)).toEqual({ write: false, utilityId: null });
        expect(register.write(actual.utilityId, rendered.generation).kind).toBe("refused");
        expect(register.read().value).toBeNull();
      }
      const key = row.windowId.replaceAll("~", "~0").replaceAll("/", "~1");
      const oracle = ajv.compile({
        type: "object",
        required: ["activeUtilityByWindowId", "initialUtilityId", "activeToolId", "actual"],
        properties: {
          activeUtilityByWindowId: { type: "object" },
          initialUtilityId: { type: ["string", "null"] },
          activeToolId: { type: ["string", "null"] },
          actual: { type: "object", required: ["write", "utilityId"], properties: { write: { type: "boolean" }, utilityId: { type: ["string", "null"] } } },
        },
        allOf: [{
          if: { properties: { activeUtilityByWindowId: { type: "object", properties: { [row.windowId]: {} }, required: [row.windowId] } } },
          then: { properties: { actual: { type: "object", properties: { write: { const: false }, utilityId: { const: { $data: `/activeUtilityByWindowId/${key}` } } } } } },
          else: {
            properties: { actual: { type: "object", properties: { write: { const: true } } } },
            if: { properties: { activeToolId: { type: "null" } } },
            then: { properties: { actual: { type: "object", properties: { utilityId: { const: { $data: "/initialUtilityId" } } } } } },
            else: { properties: { actual: { type: "object", properties: { utilityId: { type: "null" } } } } },
          },
        }],
      });
      expect(oracle({ ...row, actual }), JSON.stringify(oracle.errors)).toBe(true);
    }
    expect(() => resolveInitialWindowUtility({ ...fixture.cases[0]!, initialUtilityId: "unknown" })).toThrow();
    console.log(`[DEBUG] React ${locale} initial utility decisions match twelve neutral outputs and independent Ajv relational assertions`);
  });

  it("initial utility nullable admission preserves the resolved host context contract", async () => {
    const { testResolvedHostContext } = await import("../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts");
    testResolvedHostContext();
    console.log("[DEBUG] Actual resolved-context admission and independent Ajv agree across the existing valid and invalid corpus after nullable utility admission");
  });

  it.each(["en", "de"] as const)("initial utility registers preserve an explicit clear in the store and guest projection (%s)", (locale) => {
    const row = initialWindowUtilityCases.cases.find((row) => row.id === "fresh-window-without-initial")!;
    const state = initialShellState({ plugins: [], locks: { locale, terminology: "native" }, storage: createMemoryStoragePort() });
    const action = { type: "SET_ACTIVE_UTILITY" as const, windowId: row.windowId, utilityId: null };
    const cleared = shellReducer(state, action);
    expect(Object.hasOwn(cleared.actionPane.activeUtilityByWindowId, row.windowId)).toBe(true);
    expect(cleared.actionPane.activeUtilityByWindowId[row.windowId]).toBeNull();
    expect(shellReducer(cleared, action)).toBe(cleared);
    const projected = buildActiveUtilityByWindowId(cleared.actionPane.activeUtilityByWindowId);
    expect(projected).toEqual({ [row.windowId]: null });
    const view = parseResolvedPluginViewState({ locale, terminology: "native", windowInstances: [{ id: row.windowId, windowKindId: "drawing-composite" }], activeUtilityByWindowId: projected });
    expect(view.activeUtilityByWindowId).toEqual(projected);
    const addressed = windowViewContext({ ...view, activeUtilityId: "pen" }, row.windowId)!;
    expect(addressed.activeUtilityId).toBeUndefined();
    expect(addressed.activeUtilityByWindowId).toEqual(projected);
    console.log(`[DEBUG] React ${locale} actual store and parsed guest context preserve a deliberate clear without confusing it with a new window`);
  });

  it("renders every row disabled when an active utility gates actions", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute, disabled: true }));
    fireEvent.click(rowByText(container, "Flatten"));
    expect(onExecute).not.toHaveBeenCalled();
  });

  it.each(["en", "de"])("neutral utility declarations keep mounted palette commands executable (%s)", (locale) => {
    const actions = utilityActionPolicy.actions.map((id) => ({ id, label: { native: { en: id, de: id }, reuse: { en: id, de: id } }, iconId: "box", semantics: actionSemanticsForKind(id === "exportDocument" ? "view" : "mutation"), kind: id === "exportDocument" ? "view" : "mutation", inPalette: true, args: [] })) as ActionDefinition[];
    const windowKind = { id: utilityActionPolicy.windowKind, actions } as unknown as AppWindowKindDefinition;
    const execute = vi.fn();
    const { container, rerender } = render(createElement("div"));
    for (const policy of [...utilityActionPolicy.utilities, { id: "foreign-modal-tool", allowsActionsWhileActive: false }]) {
      const app = { controllerId: "policy-controller", windowKinds: [windowKind], utilities: [policy] } as unknown as AppDefinition;
      const before = execute.mock.calls.length;
      const node = windowActionPaneNode(app, windowKind, windowKind.id, { expandedByWindowId: {}, stagedArgsByKey: {}, activeUtilityByWindowId: { [windowKind.id]: policy.id } }, execute, vi.fn(), undefined, "native", locale);
      rerender(createElement("div", null, node));
      for (const id of utilityActionPolicy.actions) fireEvent.click(rowByText(container, id));
      expect(execute.mock.calls.slice(before).map(([action]) => action.action)).toEqual(policy.allowsActionsWhileActive ? utilityActionPolicy.actions : []);
    }
    cleanup();
    console.log(`[DEBUG] Mounted React ${locale} neutral palette declarations dispatch 60 commands across twelve armed tools and preserve foreign modal-tool refusal`);
  });

  it("groups actions into category sections like the command panel", () => {
    const createAction: ResolvedActionDefinition = { id: "box", label: "Box", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, category: "create", args: [] };
    const transformAction: ResolvedActionDefinition = { id: "move", label: "Move", iconId: "move", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, category: "transform", args: [] };
    const historyAction: ResolvedActionDefinition = { id: "undo", label: "Undo", iconId: "undo", semantics: actionSemanticsForKind("history"), kind: "history", inPalette: true, args: [] };
    const uncategorizedAction: ResolvedActionDefinition = { id: "flatten2", label: "Flatten2", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [] };
    const { container } = render(createElement(Harness, { actions: [createAction, transformAction, historyAction, uncategorizedAction], onExecute: vi.fn() }));
    const textOf = (text: string) => [...container.querySelectorAll("*")].some((el) => el.textContent?.trim() === text && el.children.length === 0);
    expect(textOf("Create")).toBe(true);
    expect(textOf("Transform")).toBe(true);
    expect(textOf("History")).toBe(true);
    expect(textOf("Actions")).toBe(true);
    expect(rowByText(container, "Box")).toBeTruthy();
    expect(rowByText(container, "Move")).toBeTruthy();
    expect(rowByText(container, "Undo")).toBeTruthy();
    expect(rowByText(container, "Flatten2")).toBeTruthy();
  });

  // 🧹️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B53: the rail is a user-facing surface, so it carries the
  // SAME curation the command palette and the shell fallback menu already apply (`if (!action.inPalette)
  // continue`). Without it the puzzle3d rail rendered 96 rows — `worldPointerDown`, `suggestionsTick`,
  // `registerBrushMesh`, `interactionSelect`, `noteShellCommand`, … — and pushed `Export` to row 79 at
  // y=1990 inside an 807 px band, 1 125 px below the fold (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §3).
  const paneRowIds = (actions: readonly ActionDefinition[]): string[] => {
    const windowKind = { id: "main", actions } as unknown as AppWindowKindDefinition;
    const app = { controllerId: "c", windowKinds: [windowKind], utilities: [] } as unknown as AppDefinition;
    const node = windowActionPaneNode(app, windowKind, "main", { expandedByWindowId: {}, stagedArgsByKey: {}, activeUtilityByWindowId: {} }, vi.fn(), vi.fn());
    if (node === undefined) return [];
    return [...render(createElement("div", null, node)).container.querySelectorAll('[id^="action."]')].map((row) => row.id);
  };

  // 📜️ A LONG rail must still reach its staged-argument form. `Tree` clips itself, so a rail taller than
  // the pane used to hide its own last rows: generation3d's 32-row rail rendered a 960 px tree inside a
  // 901 px pane body at 1600×1000 and put the expanded `addWidget` form's `kind` combobox at y=1038 —
  // outside the viewport, `click` timed out, and the verb dispatched with no staged argument
  // (ticket 26/09/18 B3c §4.3, re-measured by FL1). jsdom lays nothing out, so what this law can hold is
  // the structure that makes the band reachable: the rail is the scroll container, it may shrink inside
  // its pane, and the staged form — control AND execute — renders inside it rather than beside it.
  it("a 32-row rail scrolls its own band and keeps the staged-argument form inside it", () => {
    const rows: ResolvedActionDefinition[] = Array.from({ length: 32 }, (_, index) => ({
      id: `row${index}`,
      label: `Row ${index}`,
      iconId: "box",
      semantics: actionSemanticsForKind("mutation"),
      kind: "mutation",
      inPalette: true,
      args:
        index === 31
          ? [
              {
                id: "kind",
                label: "Kind",
                schema: {
                  kind: "string",
                  options: [
                    { value: "inputSlider", label: "inputSlider" },
                    { value: "note", label: "note" },
                  ],
                },
                required: true,
              },
            ]
          : [],
    }));
    const { container } = render(createElement(Harness, { actions: rows, onExecute: vi.fn() }));
    const pane = container.querySelector('[data-slot="window-action-pane"]');
    expect(pane).toBeTruthy();
    const paneClass = pane!.className;
    expect(paneClass).toContain("overflow-y-auto");
    expect(paneClass).toContain("min-h-0");
    expect(paneClass).toContain("flex-1");
    expect(container.querySelectorAll('[id^="action.row"]').length).toBeGreaterThanOrEqual(32);
    fireEvent.click(rowByText(container, "Row 31…"));
    const control = pane!.querySelector('[id="action.row31.arg.kind"]');
    const execute = [...pane!.querySelectorAll("button")].find((button) => button.id.includes("row31") && button.id.endsWith("execute"));
    expect(control).toBeTruthy();
    expect(execute).toBeTruthy();
  });

  it("renders only palette-eligible rows, and no rail at all when every declared action is dispatch plumbing", () => {
    const action = (id: string, inPalette: boolean, category?: string): ActionDefinition => ({
      id,
      label: id,
      iconId: "box",
      semantics: actionSemanticsForKind("shell"),
      kind: "shell",
      inPalette,
      ...(category === undefined ? {} : { category }),
      args: [],
    });
    expect(paneRowIds([action("worldPointerDown", false), action("exportSnapshot", true, "file"), action("interactionSelect", false)])).toEqual(["action.category.file", "action.exportSnapshot"]);
    cleanup();
    expect(paneRowIds([action("worldPointerDown", false), action("noteShellCommand", false)])).toEqual([]);
  });
});

describe("palette redirect and keybinding rule (P3/P4)", () => {
  const argAction: ActionDefinition = {
    id: "extrude",
    label: "Extrude",
    iconId: "box",
    semantics: actionSemanticsForKind("mutation"),
    kind: "mutation",
    inPalette: true,
    args: [{ id: "depth", label: "Depth", schema: { kind: "number", integer: false }, required: true }],
  };
  const zeroAction: ActionDefinition = { id: "flatten", label: "Flatten", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [] };

  it("only actions with a user-facing arg redirect to a staged form (P3 decision)", () => {
    const hiddenAction: ActionDefinition = { ...zeroAction, id: "toolRunStart", args: [{ id: "toolId", label: "Tool", schema: { kind: "string", options: [] }, presentation: { kind: "hidden" }, required: true }] };
    expect(actionRequiresStagedForm(argAction)).toBe(true);
    expect(actionRequiresStagedForm(zeroAction)).toBe(false);
    expect(actionRequiresStagedForm(hiddenAction)).toBe(false);
    expect(actionRequiresStagedForm({ args: [...hiddenAction.args!, ...argAction.args!] })).toBe(true);
  });

  it("matches chord key tokens against event.key verbatim per ⌨️chord-key-tokens fixture", () => {
    const forbidden = chordKeyTokensFixture.forbiddenKeyNames as string[];
    for (const name of forbidden) {
      expect(chordUsesCanonicalKeyTokens(`mod+${name}`)).toBe(false);
      expect(keyboardEventMatchesChord({ key: ".", ctrlKey: true, metaKey: false, shiftKey: false, altKey: false }, `mod+${name}`)).toBe(false);
    }
    for (const row of chordKeyTokensFixture.canonical as { chord: string; eventKey: string }[]) {
      expect(chordUsesCanonicalKeyTokens(row.chord)).toBe(true);
      expect(keyboardEventMatchesChord({ key: row.eventKey, ctrlKey: true, metaKey: false, shiftKey: false, altKey: false }, row.chord)).toBe(true);
    }
  });

  it("keybinding intent: arg-less fires, arg-action opens unless already expanded and valid then executes (P4)", () => {
    expect(resolveKeybindingIntent(zeroAction, null, {})).toEqual({ kind: "fire" });
    expect(resolveKeybindingIntent(undefined, null, {})).toEqual({ kind: "fire" });
    // not expanded → open
    expect(resolveKeybindingIntent(argAction, null, {})).toEqual({ kind: "open", actionId: "extrude" });
    expect(resolveKeybindingIntent(argAction, "other", { depth: 3 })).toEqual({ kind: "open", actionId: "extrude" });
    // expanded but required arg missing → stays open, never silent-fires
    expect(resolveKeybindingIntent(argAction, "extrude", {})).toEqual({ kind: "open", actionId: "extrude" });
    // expanded and valid → execute with merged effective args
    expect(resolveKeybindingIntent(argAction, "extrude", { depth: 4 })).toEqual({ kind: "execute", actionId: "extrude", args: { depth: 4 } });
  });

  it("retains a clipboardWrite fragment and injects it onto the next paste dispatch", () => {
    const fragment = { objects: [{ id: "slab-1" }] };
    expect(clipboardWriteFragmentFromEffect({ notify: { message: "x" } })).toBeUndefined();
    expect(clipboardWriteFragmentFromEffect({ clipboardWrite: { fragment } })).toBe(fragment);
    expect(pasteActionWithRetainedFragment({ action: "copy" }, fragment)).toEqual({ action: "copy" });
    expect(pasteActionWithRetainedFragment({ action: "paste" }, undefined)).toEqual({ action: "paste" });
    expect(pasteActionWithRetainedFragment({ action: "paste", args: { fragment: { kept: true } } }, fragment)).toEqual({ action: "paste", args: { fragment: { kept: true } } });
    expect(pasteActionWithRetainedFragment({ action: "paste" }, fragment)).toEqual({ action: "paste", args: { fragment } });
    expect(pasteArgsFragment({ args: { fragment } })).toBe(fragment);
    expect(pasteArgsFragment({ args: {} })).toBeUndefined();
  });
});

describe("registry-derived utilities and activation (P5)", () => {
  const utilities: UtilityDefinition[] = [
    { id: "select", label: "Select", iconId: "mouse-pointer", category: "selection", allowsActionsWhileActive: true },
    { id: "brush", label: "Brush", iconId: "paintbrush", group: "paint", category: "utilities", allowsActionsWhileActive: false },
    { id: "erase", label: "Erase", iconId: "eraser", group: "paint", category: "utilities", allowsActionsWhileActive: false },
  ];
  const app = { controllerId: "draw", utilities } satisfies Pick<AppDefinition, "controllerId" | "utilities">;

  it("resolveUtilities scopes to the window kind's refs, falling back to all app utilities when unset", () => {
    expect(resolveUtilities(app, { utilities: ["brush"] } as Pick<AppWindowKindDefinition, "utilities">).map((t) => t.id)).toEqual(["brush"]);
    expect(resolveUtilities(app, { utilities: [] } as unknown as Pick<AppWindowKindDefinition, "utilities">).map((t) => t.id)).toEqual(["select", "brush", "erase"]);
  });

  it("derives grouped utility nodes with the active utility pressed and a setActiveUtility onChange tagged by window", () => {
    const nodes = resolveUtilityNodes(app, { utilities: [] } as unknown as Pick<AppWindowKindDefinition, "utilities">, "brush", "w1");
    const select = nodes.find((node) => node.id === "select");
    expect(select && select.kind === "toggle" ? select.pressed : undefined).toBe(false);
    const paint = nodes.find((node) => node.id === "group:paint");
    expect(paint?.kind).toBe("collection");
    const brush = paint && paint.kind === "collection" ? paint.children.find((child) => child.id === "brush") : undefined;
    expect(brush && brush.kind === "toggle" ? brush.pressed : undefined).toBe(true);
    expect(brush && brush.kind === "toggle" && "onChange" in brush ? brush.onChange : undefined).toEqual({ controllerId: "draw", action: "setActiveUtility", args: { utilityId: "brush", windowId: "w1" } });
  });

  it("deriveUtilityNodes twin marks exactly the active utility pressed", () => {
    const nodes = deriveUtilityNodes(
      "draw",
      [
        { id: "a", label: "A", iconId: "x" },
        { id: "b", label: "B", iconId: "plus" },
      ],
      "b",
    );
    expect(nodes.map((node) => (node.kind === "toggle" ? node.pressed : undefined))).toEqual([false, true]);
  });

  it("deriveUtilityNodes hoists a single-child group to a top-level toggle", () => {
    const nodes = deriveUtilityNodes(
      "puzzle",
      [
        { id: "transform", label: "Transform", iconId: "move-3d", group: "transform" },
        { id: "brush", label: "Brush", iconId: "paintbrush" },
      ],
      "transform",
    );
    expect(nodes.map((node) => node.id)).toEqual(["transform", "brush"]);
    expect(nodes[0]?.kind).toBe("toggle");
    expect(nodes[0] && nodes[0].kind === "toggle" ? nodes[0].pressed : undefined).toBe(true);
  });

  it("resolveUtilityActivation toggles: click activates, re-click or empty deactivates", () => {
    expect(resolveUtilityActivation(null, "brush")).toBe("brush");
    expect(resolveUtilityActivation("brush", "erase")).toBe("erase");
    expect(resolveUtilityActivation("brush", "brush")).toBeNull();
    expect(resolveUtilityActivation("brush", "")).toBeNull();
    expect(resolveUtilityActivation(undefined, "")).toBeNull();
  });

  it("findPressedUtilityLeafId walks nested collections", () => {
    expect(
      findPressedUtilityLeafId([
        {
          id: "group:transform",
          kind: "collection",
          iconId: "move",
          children: [
            { id: "move", kind: "toggle", iconId: "move", pressed: false, onChange: { controllerId: "x", action: "setActiveUtility", args: { utilityId: "move" } } },
            { id: "rotate", kind: "toggle", iconId: "rotate-cw", pressed: true, onChange: { controllerId: "x", action: "setActiveUtility", args: { utilityId: "rotate" } } },
          ],
        },
      ]),
    ).toBe("rotate");
    expect(findPressedUtilityLeafId([{ id: "brush", kind: "toggle", iconId: "paintbrush", pressed: false, onChange: { controllerId: "x", action: "setActiveUtility", args: { utilityId: "brush" } } }])).toBeUndefined();
  });

  it("isWorldTransformGumballMode requires an explicit move/rotate/scale/transform mode", () => {
    expect(isWorldTransformGumballMode("move")).toBe(true);
    expect(isWorldTransformGumballMode("rotate")).toBe(true);
    expect(isWorldTransformGumballMode("scale")).toBe(true);
    expect(isWorldTransformGumballMode("transform")).toBe(true);
    expect(isWorldTransformGumballMode(undefined)).toBe(false);
    expect(isWorldTransformGumballMode("brush")).toBe(false);
    expect(isWorldTransformGumballMode("")).toBe(false);
  });

  it("worldGumballConfigForProjection intersects transform mode with planar window projections", () => {
    expect(worldGumballConfigForProjection("move", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toEqual({
      moveAxes: true,
      movePlanes: true,
      rotate: false,
      scaleAxes: false,
      scalePlanes: false,
      scaleUniform: false,
      plane: "xy",
    });
    expect(worldGumballConfigForProjection("rotate", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } }).plane).toBe("xz");
    expect(worldGumballConfigForProjection("scale", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "left" } }).plane).toBe("yz");
    expect(worldGumballConfigForProjection("move", { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }).plane).toBeUndefined();
    expect(worldGumballConfigForProjection("transform", undefined).plane).toBeUndefined();
  });

  it("gumballIdentityDelta names the handle's verb with a pose delta that moves nothing (a live commit's empty tail, an abort's verb)", () => {
    const base = { mode: "object", ids: ["n1"] };
    expect(gumballIdentityDelta("move", base)).toEqual({ action: "translateSelection", args: { ...base, dx: 0, dy: 0, dz: 0 } });
    expect(gumballIdentityDelta("transform", base, "rotateY")).toEqual({ action: "rotateSelection", args: { ...base, ax: 0, ay: 0, az: 1, angle: 0 } });
    expect(gumballIdentityDelta("scale", base)).toEqual({ action: "scaleSelection", args: { ...base, sx: 1, sy: 1, sz: 1 } });
    expect(gumballTransformDeltaBetweenPoses("move", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, base)).toBeNull();
  });

  it("worldPaintStep streams a gesture's dabs into one transaction, commits on release, swallows the click and aborts with zero trace (the wgpu world engine's press/move/release protocol)", () => {
    const steps = (events: Parameters<typeof worldPaintStep>[1][]) => {
      let gesture = WORLD_PAINT_IDLE;
      const dispatched: (Record<string, unknown> | null)[] = [];
      for (const event of events) {
        const next = worldPaintStep(gesture, event);
        gesture = next.gesture;
        dispatched.push(next.dispatch);
      }
      return { gesture, dispatched };
    };
    const dab = (origin: "press" | "drag" | "click", u: number) => ({ kind: "dab" as const, origin, objectId: "obj-1", u, v: 0.5 });
    const drag = steps([dab("press", 0.1), { kind: "press" }, dab("drag", 0.2), { kind: "release" }, dab("click", 0.2)]);
    expect(drag.dispatched).toEqual([{ objectId: "obj-1", u: 0.1, v: 0.5, phase: "stream" }, null, { objectId: "obj-1", u: 0.2, v: 0.5, phase: "stream" }, { phase: "commit" }, null]);
    expect(drag.gesture).toEqual(WORLD_PAINT_IDLE);
    expect(steps([{ kind: "press" }, { kind: "release" }, dab("click", 0.4)]).dispatched).toEqual([null, null, { objectId: "obj-1", u: 0.4, v: 0.5 }]);
    expect(steps([dab("drag", 0.3), { kind: "cancel", reason: "blur" }, { kind: "release" }]).dispatched).toEqual([{ objectId: "obj-1", u: 0.3, v: 0.5, phase: "stream" }, { phase: "abort", reason: "blur" }, null]);
    expect(steps([{ kind: "cancel", reason: "captureLost" }]).dispatched).toEqual([null]);
  });

  it("gumballTransformDeltaBetweenPoses emits incremental translate/rotate/scale args", () => {
    const base = { mode: "mesh", ids: ["obj-1"] };
    expect(gumballTransformDeltaBetweenPoses("move", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [2, -1, 0.5], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, base)).toEqual({
      action: "translateSelection",
      args: { ...base, dx: 2, dy: -1, dz: 0.5 },
    });
    expect(gumballTransformDeltaBetweenPoses("move", { position: [1, 1, 1], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [1, 1, 1], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, base)).toBeNull();
    const rotate = gumballTransformDeltaBetweenPoses("rotate", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] }, base);
    expect(rotate?.action).toBe("rotateSelection");
    expect(rotate?.args.angle).toBeCloseTo(Math.PI / 2, 5);
    const scale = gumballTransformDeltaBetweenPoses("scale", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 3, 4] }, base);
    expect(scale).toEqual({ action: "scaleSelection", args: { ...base, sx: 2, sy: 3, sz: 4 } });
    expect(gumballTransformDeltaBetweenPoses("transform", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [1, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, base, "moveX")).toEqual({
      action: "translateSelection",
      args: { ...base, dx: 1, dy: 0, dz: 0 },
    });
    const transformRotate = gumballTransformDeltaBetweenPoses(
      "transform",
      { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] },
      base,
      "rotateY",
    );
    expect(transformRotate?.action).toBe("rotateSelection");
    expect(transformRotate?.args.angle).toBeCloseTo(Math.PI / 2, 5);
  });

  it("worldGumballStep answers every step of the language-agnostic gumball live protocol exactly (local one-shot, live stream/commit/abort, no fabricated axis step)", () => {
    for (const testCase of gumballLiveProtocolFixture.cases) {
      const targets = testCase.targets as WorldGumballTargets;
      let gesture = WORLD_GUMBALL_IDLE;
      for (const [index, step] of testCase.steps.entries()) {
        const raw = step.event as Record<string, unknown>;
        const event = (raw.kind === "start" ? { ...raw, targets, live: testCase.live, transformMode: testCase.transformMode } : raw.kind === "release" ? { ...raw, targets } : raw) as WorldGumballEvent;
        const next = worldGumballStep(gesture, event);
        expect(next.dispatch, `${testCase.name} — step ${index} (${raw.kind})`).toEqual(step.dispatch);
        expect(next.skipped, `${testCase.name} — step ${index} skip`).toBe((step as { readonly skipped?: string }).skipped);
        gesture = next.gesture;
      }
      expect(gesture.targets, `${testCase.name} ends at rest`).toBeNull();
    }
  });

  it("gumball gesture targets pin opaque component IDs independently of rendered instances", () => {
    const validate = new Ajv2020({ strict: true }).compile(gumballTargetsSchema);
    for (const fixture of gumballTargetsFixture.cases) {
      expect(validate(fixture.selection)).toBe(true);
      const selection = JSON.parse(JSON.stringify(fixture.selection));
      const pinned = world3dGumballSelectionArgsV1(selection);
      expect(pinned).toEqual(fixture.expected);
      if (selection.ids) selection.ids.splice(0, selection.ids.length, "changed-instance");
      if (selection.gumballSelectionIds) selection.gumballSelectionIds.splice(0, selection.gumballSelectionIds.length, "changed-component");
      expect(pinned).toEqual(fixture.expected);
      const before = { position: gumballTargetsFixture.translation.before as [number, number, number], quaternion: [0, 0, 0, 1] as [number, number, number, number], scale: [1, 1, 1] as [number, number, number] };
      const after = { ...before, position: gumballTargetsFixture.translation.after as [number, number, number] };
      const dispatch = gumballTransformDeltaBetweenPoses("translate", before, after, pinned, "moveX");
      expect(dispatch?.args.ids).toEqual(fixture.expected.ids);
      const delta = new THREE.Vector3(...after.position).sub(new THREE.Vector3(...before.position));
      expect([dispatch?.args.dx, dispatch?.args.dy, dispatch?.args.dz]).toEqual(delta.toArray());
    }
  });

  it("gumballLivePreviewDeltaBetweenPoses applies local start→current deltas for instant mid-drag preview", () => {
    expect(gumballLivePreviewDeltaBetweenPoses("move", { position: [1, 2, 3], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [4, 2, 5], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] })).toEqual({ kind: "translate", dx: 3, dy: 0, dz: 2 });
    const rotate = gumballLivePreviewDeltaBetweenPoses("rotate", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] });
    expect(rotate?.kind).toBe("rotate");
    const scaled = applyGumballLivePreviewDeltaToPose({ position: [10, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 2, 2] }, { kind: "scale", sx: 1.5, sy: 1, sz: 2 });
    expect(scaled).toEqual({ position: [10, 0, 0], quaternion: [0, 0, 0, 1], scale: [3, 2, 4] });
    const translated = applyGumballLivePreviewDeltaToPose({ position: [1, 1, 1], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { kind: "translate", dx: 2, dy: -3, dz: 0.5 });
    expect(translated.position).toEqual([3, -2, 1.5]);
  });

  it("Relocate-utility drag grabs by selection gate and commits one absolute worldRelocate", () => {
    expect(world3dRelocateDragTargetV1("obj-1", [])).toBe("obj-1");
    expect(world3dRelocateDragTargetV1("obj-1", ["obj-1", "obj-2"])).toBe("obj-1");
    expect(world3dRelocateDragTargetV1("obj-3", ["obj-1", "obj-2"])).toBeNull();
    expect(world3dRelocateDragTargetV1(null, [])).toBeNull();
    // 🎯️ Wave B34: an empty-ground press with a live selection is the gesture's BASE POINT, not a miss —
    // `world3dRelocateDispatchArgsV1` below computes `origin + (to - from)`, a travel delta, so the base
    // point never had to be on the object. Measured on `:6013` (wasm #53): the utility was armed, the
    // selection was `["seed-left-001"]`, the ground point resolved — and the press was refused for being
    // 265 px away from the one object on screen, which is why `relocate-pose-delta` sat red at its 30 s
    // budget across four waves (`📓️2026-09-12-wave-B34-interaction-scope.md` §2).
    expect(world3dRelocateDragTargetV1(null, ["seed-left-001"])).toBe("seed-left-001");
    expect(world3dRelocateDragTargetV1(undefined, ["obj-1", "obj-2"])).toBe("obj-1");
    const selected = world3dGumballSelectionArgsV1({ ids: ["obj-1"], selectionMode: "object" });
    expect(world3dRelocateDragTargetV1("obj-1", selected.ids)).toBe("obj-1");
    expect(world3dRelocateDispatchArgsV1("obj-1", [1, 2, 3], [10, 10, 0], [14, 7, 0])).toEqual({ objectId: "obj-1", position: [5, -1, 3] });
    expect(world3dRelocateDispatchArgsV1("obj-2", [0, 0, 0], [0, 0, 0], [1.4, -2.6, 0], { gridSnapEnabled: true, gridFactor: 1 })).toEqual({ objectId: "obj-2", position: [1, -3, 0] });
    expect(world3dRelocateDispatchArgsV1("obj-2", [0, 0, 0], [0, 0, 0], [1.4, -2.6, 0], { gridSnapEnabled: false, gridFactor: 1 })).toEqual({ objectId: "obj-2", position: [1.4, -2.6, 0] });
    expect(world3dRelocateDispatchArgsV1("obj-1", [1, 2, 3], [10, 10, 0], [10, 10, 0])).toBeNull();
    expect(world3dRelocateDispatchArgsV1("obj-1", [1, 2, 3], [10, 10, 0], [10.0000001, 10, 0])).toBeNull();
  });

  // 🏁️ Wave B33: EVERY hover and brush target lane of the world host hands its gate the awaitable twin
  // (`dispatchSettled`), because `dispatch` drops the `onAction` promise the gate measures. Measured on wasm
  // #51: one 70-move brush hover storm enqueued 72 `interactionHover` + 85 `suggestionsTick` guest turns with
  // 11/10 settled, and the `addTargetVolume` behind them lost its 30 s budget while the queue drained at
  // ~3.5 turns/s (`📓️2026-09-12-wave-B33-full-run-vs-fresh-lane.md` §3). The behavioural halves are the
  // "coalescing action dispatcher" and "in-flight skipping interval" laws; this is the wiring half, so a
  // future edit cannot quietly hand a gate the void `dispatch` again.
  it("every self-gating world lane returns the controlled round trip it coalesces", async () => {
    const first = Promise.withResolvers<void>();
    const sent: string[] = [];
    const lane = createCoalescingActionDispatcher<string>((value) => {
      sent.push(value);
      return sent.length === 1 ? first.promise : Promise.resolve();
    });
    lane("marker-a");
    lane("marker-b");
    lane("marker-c");
    expect(sent).toEqual(["marker-a"]);
    first.resolve();
    await waitFor(() => expect(sent).toEqual(["marker-a", "marker-c"]));

    const { default: source } = await import("../../🧱️elements/🌐️World3dHost/🟦️.tsx?raw");
    for (const action of ["interactionHover", "referenceHover", "worldVortexHover"]) expect(source).toMatch(new RegExp(`return dispatchSettled\\(\\"${action}\\"`));
    expect(source).toMatch(/createCoalescingActionDispatcher<string \| null>\(\(fullId\) => dispatchSettled\("targetBrushSuggestions"/);
    expect(source).not.toMatch(/return dispatch\("(?:interactionHover|targetBrushSuggestions|worldVortexHover)"/);
  });

  // 🖌️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS lane W2-C: the brush candidate search is a read-only
  // framework tool run whose trace layer paints every tested candidate, so the host keeps neither a polling
  // tick nor a ghost of its own. Only a change of the armed brush's vortex travels to the guest.
  it("the world host has no brush suggestion tick and no brush ghost; the tool run trace replaces both", async () => {
    const { readFileSync, existsSync } = await import("node:fs");
    const { resolve } = await import("node:path");
    const relative = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx";
    let root = process.cwd();
    for (let hop = 0; hop < 12 && !existsSync(resolve(root, relative)); hop += 1) root = resolve(root, "..");
    const source = readFileSync(resolve(root, relative), "utf8");
    for (const retired of ["suggestionsTick", "createInFlightSkippingInterval(", "brushPreviewJson", "BrushPreviewGhost", "WorldBrushPreviewRecord", "addBrushObject"]) {
      expect(source.includes(retired), `[TRACE] ${retired} is the retired brush ghost path`).toBe(false);
    }
    expect(source).toContain("<WorldToolRunTrace lane={scene.toolRunTrace}");
    expect(source).toContain("if (brushSuggestionsTargetSentRef.current === brushSuggestionsTarget) return;");
    expect(source).toContain('if (fullId) dispatch("acceptSuggestion", { fullId });');
  });

  it("Volume-Brush Alt+click reads the ground through the host, not an occludable canvas plane", () => {
    // 🧊️ The commit predicate is the whole gesture contract: armed + Alt, nothing else. A plain press
    // while the utility is armed must stay a marquee, and Alt while it is NOT armed must stay inert.
    expect(world3dVolumeBrushCommits(true, true)).toBe(true);
    expect(world3dVolumeBrushCommits(true, false)).toBe(false);
    expect(world3dVolumeBrushCommits(false, true)).toBe(false);
    // 🧊️ The origin is ALWAYS grid-snapped (the guest re-snaps by its own spacing), and a ray that
    // misses the ground plane places nothing — which is also what keeps the ghost box off screen.
    expect(world3dVolumeBrushOriginV1([1.4, -2.6, 0], 1)).toEqual([1, -3, 0]);
    expect(world3dVolumeBrushOriginV1([1.4, -2.6, 0], 2)).toEqual([2, -2, 0]);
    expect(world3dVolumeBrushOriginV1(null, 1)).toBeNull();
    // 🧊️ One lattice for every ground gesture: a voxel placement and a catalogue drop at the same point
    // must resolve to the same origin.
    expect(world3dVolumeBrushOriginV1([1.4, -2.6, 0], 1)).toEqual(snapWorldPointToGrid([1.4, -2.6, 0], true, 1));
  });

  it("resolveWindowActions preserves every definition owned by the window", () => {
    const actionsApp = {
      controllerId: "draw",
      windowKinds: [
        {
          actions: [
            { id: "extrude", label: "Extrude", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [] },
            { id: "undo", label: "Undo", iconId: "undo", semantics: actionSemanticsForKind("history"), kind: "history", inPalette: true, args: [] },
            { id: "setActiveUtility", label: "Set Active Utility", iconId: "wrench", semantics: actionSemanticsForKind("view"), kind: "view", inPalette: false, args: [] },
          ] satisfies ActionDefinition[],
        },
      ],
    };
    const resolved = resolveWindowActions(actionsApp, actionsApp.windowKinds[0]!);
    expect(resolved.map((action) => action.id)).toEqual(["extrude", "undo", "setActiveUtility"]);
  });

  it("panelTabDefinitionToNode maps the framework-injected History panel tab through its rendered body", () => {
    // 🕰️ id mirrors Rust `FRAMEWORK_PANEL_TAB_HISTORY_ID` — auto-injected into every app's panelTabs
    // by `AppBuilder::build_definition` (see `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`).
    const emptyAppLabelsOverlay = {
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
    const historyTab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: "History", group: "settings" as const, bodyKey: "framework.body.history", children: [] };
    const historyUiNode = buildContractNode({
      key: "framework.history",
      component: { type: "tree", interactionDomain: null },
      children: [
        {
          key: "framework.history.commands",
          component: { type: "treeSection", label: null, defaultOpen: true, headerToolbar: null, window: null },
          children: [
            { key: "framework.history.entry.1", component: { type: "treeItem", label: "Increment", description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null } },
          ],
        },
      ],
    });
    const historyStore = new UiDocumentStore("panel:framework.panel.history");
    historyStore.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", historyUiNode));
    const node = panelTabDefinitionToNode(historyTab, "settings", { "framework.panel.history": historyStore }, () => {}, 1, emptyAppLabelsOverlay);
    expect(node.kind).toBe("leaf");
    if (node.kind !== "leaf") return;
    expect(node.id).toBe("framework.panel.history");
    const source = node.trees[0].tree;
    const config = "resolveTree" in source ? source.resolveTree() : source;
    const rendered = render(panelTreePanelHost(config));
    expect(rendered.getByText("Increment")).toBeTruthy();
  });

  it("anchors every declarable panel group on one of the four corners the dock fills from the session's own panel tabs", () => {
    // 🧭️ `ShellHost`'s `defaultDock` reads `session.app.panelTabs` for all four corners. It used to read
    // them for the two top corners only, so a `display`- or `settings`-group panel (puzzle3d's own settings
    // section, with its grid-spacing/chunk-size/proximity/overlap steppers) was rendered by the guest,
    // cached as a panel body, and then dropped before any dock node existed — unreachable in every anchor.
    expect(panelAnchorForGroup("workbench")).toBe("top-left");
    expect(panelAnchorForGroup("document")).toBe("top-left");
    expect(panelAnchorForGroup("details")).toBe("top-right");
    expect(panelAnchorForGroup("display")).toBe("bottom-left");
    expect(panelAnchorForGroup("settings")).toBe("bottom-right");
    const dockFilledFromSession = new Set(["top-left", "top-right", "bottom-left", "bottom-right"]);
    for (const group of ["workbench", "document", "details", "display", "settings", "unknown-group"]) {
      expect(dockFilledFromSession.has(panelAnchorForGroup(group))).toBe(true);
    }
  });

  it("docks the framework History tab as its own bottom-right leaf and nests only the app's Settings tabs in the Settings branch", () => {
    // 🕰️ `framework.panel.history` is injected into EVERY app's `panelTabs` (Settings group) and rendered from the guest's own
    // `framework.body.history`; the dock keeps it a sibling leaf beside Settings, exactly like the wgpu dock assembly, so the
    // Settings branch never carries a second History and the leaf is the interpreted body, not a host-built tab.
    const panelTabs = [
      { kind: { kind: "app" as const, id: "puzzle3d.panel.settings" }, group: "settings" as const },
      { kind: { kind: "app" as const, id: "framework.panel.history" }, group: "settings" as const },
    ];
    const { history, rest } = partitionFrameworkHistoryPanelTab(panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "bottom-right"));
    expect(history?.kind.id).toBe("framework.panel.history");
    expect(rest.map((tab) => tab.kind.id)).toEqual(["puzzle3d.panel.settings"]);
    expect(partitionFrameworkHistoryPanelTab([panelTabs[0]!]).history).toBeUndefined();
  });

  it("moves every out-of-hook chrome label when the in-app language switch runs, and restores them on the way back", async () => {
    // 🌐️ A shell owns TWO i18n ports: its `ShellScope` instance (what `useUiTranslation` resolves) and the
    // shared module port (the only one a tree builder outside hook context can read). `ShellHost`'s locale
    // effect used to move the scope instance alone, so picking Deutsch in `#framework.settings.language`
    // relabelled the hook-rendered chrome and left every builder-produced name in English — measured live on
    // :6013 as "Einstellungen"/"Vollbild"/"Remote: getrennt" beside "Settings"/"General"/"Hotkeys"/"History".
    const keys = ["ui.panel.artifact", "ui.settings.tab.general", "ui.settings.tab.language"] as const;
    const english = keys.map((key) => String(shellLabel(key)));
    syncShellLabelLocale("de");
    const german = keys.map((key) => String(shellLabel(key)));
    expect(german).not.toEqual(english);
    for (const [index, label] of german.entries()) expect(label).not.toBe(english[index]);
    syncShellLabelLocale("en");
    expect(keys.map((key) => String(shellLabel(key)))).toEqual(english);
  });
});

describe("resolveCommands / commandCategories (footer command panel registry)", () => {
  const command = (id: string, label: string, category: string): CommandDefinition => ({ id, label, category, iconId: "wrench", semantics: actionSemanticsForKind("shell"), kind: "shell", inPalette: true, args: [], keybindings: [] });
  const osCommands: CommandDefinition[] = [command("os.setThemeId", "Set Theme", "appearance")];
  const pluginManifest = { pluginId: "fixture", commands: [command("export", "Export", "document")] };
  const app = {
    id: "canvas",
    commands: [command("resetGrid", "Reset Grid", "document")],
    modes: [
      { id: "edit", label: "Edit", commands: [command("focus", "Focus", "view")] },
      { id: "paint", label: "Paint", commands: [command("paintOnly", "Paint Only", "view")] },
    ] as AppModeDefinition[],
  };

  it("aggregates os + program + app-scope + active-mode's mode-scope commands, excluding other modes' mode-scope commands", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "edit");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId", "export", "resetGrid", "focus"]);
    expect(resolved.find((entry) => entry.definition.id === "os.setThemeId")?.address).toEqual({ owner: "os", commandId: "os.setThemeId" });
    expect(resolved.find((entry) => entry.definition.id === "resetGrid")?.address).toEqual({ owner: { app: { pluginId: "fixture", appId: "canvas" } }, commandId: "resetGrid" });
    expect(resolved.find((entry) => entry.definition.id === "focus")?.address).toEqual({ owner: { mode: { pluginId: "fixture", appId: "canvas", modeId: "edit" } }, commandId: "focus" });
  });

  it("switching the active mode swaps which mode-scope commands resolve", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "paint");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId", "export", "resetGrid", "paintOnly"]);
  });

  it("resolves only os commands with no session (null program manifest / app)", () => {
    const resolved = resolveCommands(osCommands, null, null, "");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId"]);
  });

  it("owner-qualifies identical local command ids into collision-free UI keys", () => {
    const duplicateId = "refresh";
    const resolved = resolveCommands(
      [command(duplicateId, "Refresh Shell", "general")],
      { pluginId: "fixture", commands: [command(duplicateId, "Refresh Plugin", "general")] },
      {
        id: "canvas",
        commands: [command(duplicateId, "Refresh App", "general")],
        modes: [{ id: "edit", label: "Edit", commands: [command(duplicateId, "Refresh Mode", "general")] }] as AppModeDefinition[],
      },
      "edit",
    );
    const keys = resolved.map((entry) => commandAddressKey(entry.address));
    expect(new Set(keys).size).toBe(4);
    expect(keys).toEqual(["os:refresh", "plugin:fixture:refresh", "app:fixture:canvas:refresh", "mode:fixture:canvas:edit:refresh"]);
  });

  it("commandCategories orders and dedupes categories by first appearance", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "edit");
    expect(commandCategories(resolved)).toEqual([
      { id: "appearance", label: "Appearance" },
      { id: "document", label: "Document" },
      { id: "view", label: "View" },
    ]);
  });
});

describe("resolveModeTools / buildToolTabs (footer tool panel registry)", () => {
  const toolApp = {
    tools: [
      { id: "fill", label: "Fill", iconId: "paint-bucket" },
      { id: "brush", label: "Brush", iconId: "paintbrush" },
    ] satisfies ResolvedToolDefinition[],
    modes: [
      { id: "edit", label: "Edit", tools: ["fill", "brush"] },
      { id: "view", label: "View", tools: [] },
    ] as AppModeDefinition[],
  };

  it("resolves the active mode's tools in declared order", () => {
    expect(resolveModeTools(toolApp, "edit").map((tool) => tool.id)).toEqual(["fill", "brush"]);
  });

  it("tools are opt-in per mode — no orphan fallback for a mode that declares none", () => {
    expect(resolveModeTools(toolApp, "view")).toEqual([]);
  });

  it("resolves nothing for an app/mode that doesn't exist", () => {
    expect(resolveModeTools(undefined, "edit")).toEqual([]);
    expect(resolveModeTools(toolApp, "nonexistent")).toEqual([]);
  });

  it("buildToolTabs builds one leaf per resolved tool, whose lazily-resolved tree carries that tool's own measures", () => {
    const toolMeasuresByToolIdRef = {
      current: { fill: [{ kind: "slider", id: "puzzle3d-fill-count", label: "Count", value: 3, min: 0, max: 100, onChange: { controllerId: "c", action: "setFillCount" } }] } as Readonly<Record<string, readonly WindowMeasure[]>>,
    };
    const onAction = vi.fn();
    const tabs = buildToolTabs(toolApp.tools, toolMeasuresByToolIdRef, onAction);
    expect(tabs.map((tab) => tab.id)).toEqual(["tool.fill", "tool.brush"]);
    const fillTab = tabs[0] as Extract<PanelTabNode, { kind: "leaf" }>;
    const fillTree = fillTab.trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[]; sortableSections: false } };
    const fillResolved = fillTree.resolveTree();
    expect(fillResolved.sortableSections).toBe(false);
    expect(fillResolved.sections).toHaveLength(1);
    expect(fillResolved.sections[0]!.id).toBe("tool.fill.options");
    expect(fillResolved.sections[0]!.items).toHaveLength(1);
    expect(fillResolved.sections[0]!.items![0]!.label).toBe("Count");
    expect(fillResolved.sections[0]!.items![0]!.control).toBeTruthy();

    const brushTab = tabs[1] as Extract<PanelTabNode, { kind: "leaf" }>;
    const brushTree = brushTab.trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[] } };
    // A tool whose program published no measures yet resolves to an EMPTY options section — never to a
    // second activation control (see the element-identity law below).
    const brushResolved = brushTree.resolveTree();
    expect(brushResolved.sections).toHaveLength(1);
    expect(brushResolved.sections[0]!.id).toBe("tool.brush.options");
    expect(brushResolved.sections[0]!.items).toEqual([]);
  });

  // 🛠️ W-G law (browser-verified 2026-09-09): the `tool.<id>` leaf tab IS the activation control. A tool
  // tree that renders its own toggle carried the leaf tab's element id (`tool.fill` appeared twice in the
  // document) and disagreed with it — the tab read "selected" while the toggle read "not pressed", so the
  // first press on a restored Fill tab deactivated instead of arming the tool.
  it("no tool tree carries a control that duplicates its own leaf tab id", () => {
    const toolMeasuresByToolIdRef = { current: {} as Readonly<Record<string, readonly WindowMeasure[]>> };
    const tabs = buildToolTabs(toolApp.tools, toolMeasuresByToolIdRef, vi.fn());
    for (const tab of tabs as Extract<PanelTabNode, { kind: "leaf" }>[]) {
      const tree = tab.trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[] } };
      for (const section of tree.resolveTree().sections) {
        for (const item of section.items ?? []) {
          const control = item.control as ReactElement<{ id?: string }> | undefined;
          expect(control?.props?.id).not.toBe(tab.id);
        }
      }
    }
  });

  // 🛠️ W-G law: tool activation and the selected tool leaf are ONE state, whatever route set the path.
  it("reconcileToolTabSelection arms the tool a restored dock arrangement already selects", () => {
    // Boot: `DockUiStateStore` restores bottom-middle at ["framework.category.tool", "tool.fill"] while no
    // tool is active. Before W-G nothing reconciled this, so the FIRST press on the selected leaf read as a
    // re-press and collapsed it (`progressPanelTabSelection` below) instead of arming Fill.
    const restored = ["framework.category.tool", "tool.fill"];
    const hydrate = reconcileToolTabSelection(null, null, toolIdFromPanelTabId(restored[restored.length - 1]!));
    expect(hydrate.effect).toEqual({ kind: "activate", toolId: "fill" });
    expect(hydrate.next).toEqual({ toolId: "fill", selected: "fill" });
    // Once armed the pass is idle — no bounce between the two representations.
    expect(reconcileToolTabSelection(hydrate.next, "fill", "fill").effect).toEqual({ kind: "idle" });
  });

  it("windowed context-menu view overlays the armed tool like handleAction", () => {
    const view = {
      locale: "en",
      terminology: "native",
      windowInstances: [{ id: "left", windowKindId: "graph" }],
      activeUtilityByWindowId: { left: "pan" },
    };
    expect(windowViewContext(view, "left")?.activeToolId).toBeUndefined();
    expect(hostArmedViewContext(view, "fill", "left")?.activeToolId).toBe("fill");
  });

  it("reconcileToolTabSelection disarms the tool when its leaf collapses, and re-arms on the next press", () => {
    const armed: ToolTabSelection = { toolId: "fill", selected: "fill" };
    const collapsed = reconcileToolTabSelection(armed, "fill", null);
    expect(collapsed.effect).toEqual({ kind: "activate", toolId: null });
    expect(reconcileToolTabSelection(collapsed.next, null, "fill").effect).toEqual({ kind: "activate", toolId: "fill" });
  });

  it("reconcileToolTabSelection selects the leaf of a tool armed by the program, and collapses when a utility clears it", () => {
    const idle: ToolTabSelection = { toolId: null, selected: null };
    const armedByProgram = reconcileToolTabSelection(idle, "fill", null);
    expect(armedByProgram.effect).toEqual({ kind: "select", toolId: "fill" });
    expect(reconcileToolTabSelection(armedByProgram.next, null, "fill").effect).toEqual({ kind: "select", toolId: null });
  });

  // 🛠️ Wave B29 §14 `engagement-fill-verb`: typing `fill 3` answers `Effect::SetActiveTool { fill }`, and
  // the reconciliation above is skipped wholesale while the Tool category is not the active root — so the
  // tool went live with NO `tool.fill` leaf mounted anywhere, its options unreachable and nothing on
  // screen saying it had armed. A program-driven arming reveals its own leaf; a user who armed a tool and
  // then browsed the Command palette is left alone.
  it("a tool the program arms reveals its own leaf tab, and a tool the user browsed away from does not", () => {
    expect(programArmedToolRevealV1(null, "fill", false)).toBe(true);
    expect(programArmedToolRevealV1(null, "fill", true)).toBe(false);
    expect(programArmedToolRevealV1("fill", "fill", false)).toBe(false);
    expect(programArmedToolRevealV1("fill", null, false)).toBe(false);
    expect(programArmedToolRevealV1("fill", "brush", false)).toBe(true);
    expect(programArmedToolRevealV1(null, null, false)).toBe(false);
  });

  it("reconcileToolTabSelection self-heals a refused activation instead of looping", () => {
    const refused = reconcileToolTabSelection(null, null, "fill");
    expect(refused.effect).toEqual({ kind: "activate", toolId: "fill" });
    expect(reconcileToolTabSelection(refused.next, null, "fill").effect).toEqual({ kind: "select", toolId: null });
  });

  // 🛠️ W-G law: one press on the Tool category is enough — the remembered drill-down lands on the tool
  // leaf and the same reconciliation arms it, so its ribbon items are live without a second press.
  it("one press on the Tool category reveals its remembered tool leaf and arms that tool", () => {
    const tabs: PanelTabNode[] = [{ kind: "branch", id: "framework.category.tool", icon: () => null, name: "Tool", children: buildToolTabs(toolApp.tools, { current: {} }, vi.fn()) }];
    const opened = progressPanelTabSelection(tabs, [], ["framework.category.tool"], { "framework.category.tool": "tool.fill" });
    expect(opened.fold).toBe(false);
    expect(opened.path).toEqual(["framework.category.tool", "tool.fill"]);
    expect(reconcileToolTabSelection(null, null, toolIdFromPanelTabId(opened.path[opened.path.length - 1])).effect).toEqual({ kind: "activate", toolId: "fill" });
  });

  it("toolIdFromPanelTabId extracts the mode tool id from a tool leaf tab id", () => {
    expect(toolIdFromPanelTabId("tool.fill")).toBe("fill");
    expect(toolIdFromPanelTabId("tool.brush")).toBe("brush");
    expect(toolIdFromPanelTabId("framework.category.tool")).toBeNull();
    expect(toolIdFromPanelTabId("tool.")).toBeNull();
    expect(toolIdFromPanelTabId(undefined)).toBeNull();
  });

  // 🛠️ W-G3: guest always publishes fill measures on `framework.section.tools`, but the desktop
  // Tool pane is `PanelTreeUnitsPane` (memoized) over lazy `resolveTree`. A first resolve against an
  // empty ref (boot / tools surface not yet retained) stayed empty forever because `buildPanelProps`
  // omitted `treeContentRevision`. This law fails if the revision identity does not change when the
  // packed tools carrier lands, which is what re-calls `resolveTree` on the same tab objects.
  it("late-arriving fill measures change the tool panel tree revision so the memoized pane re-resolves", () => {
    const toolMeasuresByToolIdRef = { current: {} as Readonly<Record<string, readonly WindowMeasure[]>> };
    const tabs = buildToolTabs(toolApp.tools, toolMeasuresByToolIdRef, vi.fn());
    const fillTree = (tabs[0] as Extract<PanelTabNode, { kind: "leaf" }>).trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[] } };
    expect(fillTree.resolveTree().sections[0]!.items).toEqual([]);
    const emptyRevision = toolPanelTreeContentRevision(null, toolMeasuresByToolIdRef.current, {});
    toolMeasuresByToolIdRef.current = { fill: [{ kind: "slider", id: "puzzle3d-fill-count", label: "Count", value: 0, min: 0, max: 100, onChange: { controllerId: "c", action: "setFillCount" } }] };
    expect(fillTree.resolveTree().sections[0]!.items).toHaveLength(1);
    const landedRevision = toolPanelTreeContentRevision(null, toolMeasuresByToolIdRef.current, {});
    expect(landedRevision).not.toEqual(emptyRevision);
    expect(landedRevision.toolMeasuresByToolId.fill).toHaveLength(1);
  });

  // 🛠️ W-G3: a closed Tool category press must not swallow as a root re-select. Opening from an
  // empty current path applies memory (or the first tool leaf) so one press reveals Fill.
  it("one press on an inactive selected Fill leaf arms the tool instead of collapsing it", () => {
    const previous = ["framework.category.tool", "tool.fill"];
    const collapsed = progressPanelTabSelection(
      [{ kind: "branch", id: "framework.category.tool", icon: () => null, name: "Tool", children: buildToolTabs(toolApp.tools, { current: {} }, vi.fn()) }],
      previous,
      ["framework.category.tool", "tool.fill"],
      { "framework.category.tool": "tool.fill" },
    );
    expect(collapsed.path).toEqual(["framework.category.tool"]);
    const repress = toolLeafInactiveRepress(previous, collapsed.path, null);
    expect(repress).toEqual({ path: previous, toolId: "fill" });
    expect(toolLeafInactiveRepress(previous, collapsed.path, "fill")).toBeNull();
  });

  it("one press that opens the Tool category selects Fill when no leaf is remembered", () => {
    const tabs = buildToolTabs(toolApp.tools, { current: {} }, vi.fn());
    const opened = progressPanelTabSelection([{ kind: "branch", id: "framework.category.tool", icon: () => null, name: "Tool", children: tabs }], [], ["framework.category.tool"], {});
    expect(opened.fold).toBe(false);
    expect(opened.path).toEqual(["framework.category.tool"]);
    expect(
      toolCategoryOpenPath(
        opened.path,
        opened.memory,
        tabs.map((tab) => tab.id),
      ),
    ).toEqual(["framework.category.tool", "tool.fill"]);
  });

  it("a Tool category branch still exposes the remembered Fill leaf trees as the panel body", () => {
    const fill = buildToolTabs(toolApp.tools, { current: {} }, vi.fn())[0]!;
    const branch: PanelTabNode = { kind: "branch", id: "framework.category.tool", icon: () => null, name: "Tool", children: [fill] };
    const body = resolvePanelBranchBodyLeaf(branch, { "framework.category.tool": "tool.fill" });
    expect(body?.id).toBe("tool.fill");
    expect(body?.kind).toBe("leaf");
    expect(body && body.kind === "leaf" ? body.trees.length : 0).toBeGreaterThan(0);
    expect(resolvePanelBranchBodyLeaf(branch, {})?.id).toBe("tool.fill");
  });
});

describe("Introduce App command", () => {
  it("is available only for apps with an introduction", () => {
    expect(buildOsCommands([], [], true).find((command) => command.id === "os.introduceApp")).toMatchObject({ label: "Introduce App", category: "app", args: [] });
    expect(buildOsCommands([], [], false).some((command) => command.id === "os.introduceApp")).toBe(false);
  });

  it("starts the introduction at its first step", () => {
    const dispatch = vi.fn();
    dispatchOsCommand("os.introduceApp", undefined, vi.fn(), dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    expect(dispatch).toHaveBeenCalledWith({ type: "SET_INTRODUCTION_STEP", value: 0 });
  });
});

describe("Play/Record Tutorial commands", () => {
  it("os.playTutorial appears only when at least one tutorial is declared, offering each as a Select option", () => {
    expect(buildOsCommands([], [], false).some((command) => command.id === "os.playTutorial")).toBe(false);
    const withTutorials = buildOsCommands([], [], false, undefined, undefined, [{ id: "welcome-tour", title: "Welcome Tour" }]);
    const playTutorial = withTutorials.find((command) => command.id === "os.playTutorial");
    expect(playTutorial).toMatchObject({ label: "Play Tutorial", category: "app" });
    expect(playTutorial?.args[0]).toMatchObject({ id: "tutorialId", required: true, schema: { kind: "string", options: [{ value: "welcome-tour", label: "Welcome Tour" }] } });
    const localized = buildOsCommands([], [], false, undefined, undefined, [{ id: "localized-tour", title: labelResolutionFixture.matrix }], false, "reuse", "de");
    expect(localized.find((command) => command.id === "os.playTutorial")?.args[0]).toMatchObject({ schema: { kind: "string", options: [{ value: "localized-tour", label: "Bauteil" }] } });
  });

  it("os.recordTutorial appears only when the recorder is available (dev/studio), independent of declared tutorials", () => {
    expect(buildOsCommands([], [], false, undefined, undefined, [], false).some((command) => command.id === "os.recordTutorial")).toBe(false);
    expect(buildOsCommands([], [], false, undefined, undefined, [], true).some((command) => command.id === "os.recordTutorial")).toBe(true);
  });

  it("os-scope Play/Record Tutorial commands are NOT handled by dispatchOsCommand (routed earlier, through the shell's own startTutorialRef/toggleTutorialRecordingRef bridge)", () => {
    const dispatch = vi.fn();
    dispatchOsCommand("os.playTutorial", { tutorialId: "welcome-tour" }, vi.fn(), dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    dispatchOsCommand("os.recordTutorial", undefined, vi.fn(), dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    expect(dispatch).not.toHaveBeenCalled();
  });
});

describe("shell option locks (SEMIO_LOCKED_*)", () => {
  it("resolves valid locale/appearance and falls back with a warning on invalid values while staying locked", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    expect(resolveShellLocks({ locale: "de" })).toMatchObject({ locale: "de" });
    expect(resolveShellLocks({ locale: "fr" })).toMatchObject({ locale: "en" });
    expect(resolveShellLocks({ appearance: "dark" })).toMatchObject({ appearance: "dark" });
    expect(resolveShellLocks({ appearance: "bogus" })).toMatchObject({ appearance: "system" });
    expect(warn).toHaveBeenCalled();
    warn.mockRestore();
  });

  it("accepts any non-empty terminology id verbatim (app-declared ids can't be validated at boot)", () => {
    expect(resolveShellLocks({ terminology: "reuse" })).toMatchObject({ terminology: "reuse" });
    expect(resolveShellLocks({ terminology: "some-app-declared-id" })).toMatchObject({ terminology: "some-app-declared-id" });
    expect(resolveShellLocks({ terminology: "" })).toEqual({});
  });

  it("returns an empty object for undefined locks", () => {
    expect(resolveShellLocks(undefined)).toEqual({});
  });

  it("initialShellState applies locked values over stored/default prefs", () => {
    const state = initialShellState({ plugins: [], locks: { exampleId: "concrete-forest", locale: "de", terminology: "reuse", themeId: "semio", appearance: "dark" }, storage: createMemoryStoragePort() });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    expect(state.uiPrefs.uiLocale).toBe("de");
    expect(state.uiPrefs.uiTerminology).toBe("reuse");
    expect(state.uiPrefs.uiThemeId).toBe("semio");
    expect(state.uiPrefs.uiAppearance).toBe("dark");
  });

  it("mergeShellLockSources keeps brand locks locked and lets a defined env lock win per key", () => {
    expect(mergeShellLockSources({ locale: "de", terminology: "reuse" }, { locale: "en", themeId: undefined })).toEqual({ locale: "en", terminology: "reuse" });
    expect(mergeShellLockSources({ locale: "de" }, undefined)).toEqual({ locale: "de" });
    expect(mergeShellLockSources(undefined, { locale: "en" })).toEqual({ locale: "en" });
    expect(mergeShellLockSources(undefined, undefined)).toBeUndefined();
  });

  it("resolveShellDefaults prefers env defaults over brand defaults and initialShellState seeds without locking", () => {
    const brand = { id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", defaults: { exampleId: "concrete-forest" } };
    expect(resolveShellDefaults(brand, { exampleId: "nakagin-capsule-tower" })).toEqual({ exampleId: "nakagin-capsule-tower" });
    expect(resolveShellDefaults(brand, undefined)).toEqual({ exampleId: "concrete-forest" });
    expect(resolveShellDefaults(undefined, undefined)).toEqual({ exampleId: undefined });
    const state = initialShellState({ plugins: [], defaults: { exampleId: "concrete-forest" }, storage: createMemoryStoragePort() });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    const locked = initialShellState({ plugins: [], locks: { exampleId: "nakagin-capsule-tower" }, defaults: { exampleId: "concrete-forest" }, storage: createMemoryStoragePort() });
    expect(locked.layout.activeExampleId).toBe("nakagin-capsule-tower");
  });

  it("resolveBootExampleId seeds the first registered example when nothing is active or defaulted", () => {
    const options = [{ id: "hexagonal-mushroom-column" }, { id: "rectangle-extrude-volume" }, { id: "sphere-cut-with-torus" }];
    expect(resolveBootExampleId("", options)).toBe("hexagonal-mushroom-column");
    expect(resolveBootExampleId("", options, "sphere-cut-with-torus")).toBe("sphere-cut-with-torus");
    expect(resolveBootExampleId("rectangle-extrude-volume", options, "sphere-cut-with-torus")).toBe("rectangle-extrude-volume");
    expect(resolveBootExampleId("missing", options)).toBe("hexagonal-mushroom-column");
    expect(resolveBootExampleId("", [])).toBe("");
  });

  // 🎨️ The REDO half of the navbar label, measured red on :6013 in wave B26: undo relabelled the picker
  // (a popped `Set Active Example` row answers with the boot example, which needs no memory) and redo
  // silently could not, because the only writer of the remembered id was `NavbarExampleSelect`'s own
  // `onValueChange` — `navbar example from history {"navbarExample":"concrete-forest","remembered":""}`.
  // Every dispatch of the verb now teaches the memory, so a row redone from anywhere relabels.
  it("offers examples only to an app that declares setActiveExample on some window kind", () => {
    expect(
      appSwitchesExamples("s.cad.cad@1/*#editor", [
        { id: "cad-play-shape", actions: [{ id: "select" }] },
        { id: "cad-play-energy", actions: [{ id: SET_ACTIVE_EXAMPLE_ACTION_ID }] },
      ]),
    ).toBe(true);
    expect(appSwitchesExamples("s.vcs.vcs@1/*#editor", [{ id: "vcs-editor", actions: [{ id: "commit" }] }, { id: "vcs-history" }])).toBe(false);
    expect(appSwitchesExamples("s.note.note@1/*#editor", [])).toBe(false);
  });

  it("answers neutral example ownership vectors with the independent schema oracle", () => {
    const oracle = new Ajv2020({ strict: true }).compile(exampleOfferSchema);
    for (const { name, expected, ...row } of exampleOfferFixture.cases) {
      const app = { id: name, role: row.role, actions: row.appActions.map((id) => ({ id })), windowKinds: row.windowActions.map((actions, index) => ({ id: String(index), actions: actions.map((id) => ({ id })) })) };
      expect(oracle(row), name).toBe(expected);
      expect(appOffersRegisteredExamples(app, row.exampleCount), name).toBe(expected);
    }
  });

  it("lists registered examples on every editor and admits the switch without a declared action", () => {
    const editor = { id: "s.lowpoly.lowpoly@1/*#editor", role: "editor", windowKinds: [{ id: "model", actions: [{ id: "select" }] }] };
    expect(appOffersRegisteredExamples(editor, 1)).toBe(true);
    expect(appOffersRegisteredExamples(editor, 0)).toBe(false);
    expect(appSwitchesExamples(editor.id, editor.windowKinds)).toBe(false);
    expect(frameworkOwnsExampleSwitch("editor", SET_ACTIVE_EXAMPLE_ACTION_ID)).toBe(true);
    expect(frameworkOwnsExampleSwitch("viewer", SET_ACTIVE_EXAMPLE_ACTION_ID)).toBe(false);
    expect(frameworkOwnsExampleSwitch("editor", "select")).toBe(false);
    const viewer = { id: "s.lowpoly.lowpoly@1/*#viewer", role: "viewer", windowKinds: [{ id: "model", actions: [{ id: "select" }] }] };
    expect(appOffersRegisteredExamples(viewer, 1)).toBe(false);
    expect(appOffersRegisteredExamples({ ...viewer, windowKinds: [{ id: "model", actions: [{ id: SET_ACTIVE_EXAMPLE_ACTION_ID }] }] }, 1)).toBe(true);
  });

  it("remembers the example id of every setActiveExample dispatch, so a redone row can relabel the picker", () => {
    const remembered = rememberedExampleIdFromDispatchV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: "nakagin-capsule-tower" } }, "");
    expect(remembered).toBe("nakagin-capsule-tower");
    expect(rememberedExampleIdFromDispatchV1({ action: "setSpacing", args: { exampleId: "concrete-forest" } }, remembered)).toBe("nakagin-capsule-tower");
    expect(rememberedExampleIdFromDispatchV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: "" } }, remembered)).toBe("nakagin-capsule-tower");
    expect(rememberedExampleIdFromDispatchV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID }, remembered)).toBe("nakagin-capsule-tower");
    // ↩️ undo pops the row → the boot example; ↪️ redo makes it live again → exactly what was remembered.
    const popped = [{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, label: "Set Active Example", revertible: false }];
    const live = [{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, label: "Set Active Example", revertible: true }];
    expect(navbarExampleIdFromHistoryUpserts(popped, remembered, "concrete-forest")).toBe("concrete-forest");
    expect(navbarExampleIdFromHistoryUpserts(live, remembered, "concrete-forest")).toBe("nakagin-capsule-tower");
    expect(navbarExampleIdFromHistoryUpserts(live, "", "concrete-forest")).toBeUndefined();
  });

  it("navbar example id follows Set Active Example revertible, on the action id alone", () => {
    // 🪪️ The row is recognised by its action id, never by its label's English prose — a history
    // label is a locale matrix now, so a German shell must reach the same verdict. Ticket 26/09/18 U3.
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: "shell.windowResize", revertible: false }], "nakagin", "forest")).toBeUndefined();
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, revertible: true }], "nakagin", "forest")).toBe("nakagin");
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, revertible: false }], "nakagin", "forest")).toBe("forest");
  });

  it("leftover InteractionView publication populates selection, lock, and gumball", () => {
    const published = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: { domain: "vortex", channel: "pointer", id: "seed-left-001" },
        locked: { "seed-left-001": false },
        gumball: { active: true, anchorId: "seed-left-001" },
        selection: { vortex: { granularity: "object", ids: ["seed-left-001"] } },
        hover: { vortex: { channel: "pointer", ids: ["seed-left-001"] } },
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
      },
    });
    expect(published?.selectedIds).toEqual(["seed-left-001"]);
    expect(published?.locked["seed-left-001"]).toBe(false);
    expect(published?.gumballActive).toBe(true);
    expect(published?.gumballAnchorId).toBe("seed-left-001");
    expect(leftoverInteractionStateV1(published!).selection.vortex?.ids).toEqual(["seed-left-001"]);
    expect(interactionViewFromLeftoverOutput(null)).toBeNull();
    const selectOnly = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: null,
        locked: {},
        gumball: { active: false, anchorId: null },
        selection: { vortex: { granularity: "object", ids: ["seed-left-001"] } },
        hover: {},
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
        activeUtility: "select",
      },
    });
    expect(selectOnly?.gumballActive).toBe(false);
    expect(leftoverWorldGumballPoseV1({ gumballActive: false, gumballAnchorId: null, ids: ["seed-left-001"] }, [{ id: "seed-left-001", position: [1, 2, 3] }]).transformMode).toBeUndefined();
    const pose = leftoverWorldGumballPoseV1({ gumballActive: true, gumballAnchorId: "seed-left-001", ids: ["seed-left-001"] }, [{ id: "seed-left-001", position: [1, 2, 3] }]);
    expect(pose.transformMode).toBe("transform");
    expect(pose.gumballTarget).toEqual([1, 2, 3]);
    const gumballArgs = world3dGumballSelectionArgsV1({ ids: ["seed-left-001"], componentIds: [9] });
    expect(gumballArgs.ids).toEqual(["seed-left-001"]);
    expect(gumballArgs.mode).toBe("object");
  });

  it("hover-only leftover InteractionView overlays vortex hover without selected ids", () => {
    const published = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: [],
        hoverTarget: { domain: "vortex", channel: "pointer", id: "seed-left-001:v0" },
        locked: {},
        gumball: { active: false, anchorId: null },
        selection: {},
        hover: { vortex: { channel: "pointer", ids: ["seed-left-001:v0"] } },
        activeMode: { vortex: "multiple" },
        activeGranularity: { vortex: "vortex" },
      },
    });
    expect(published?.hoverTarget).toEqual({ domain: "vortex", channel: "pointer", id: "seed-left-001:v0" });
    expect(leftoverWorldOverlayAppliesV1({ ids: published!.selectedIds, hoveredId: published!.hoverTarget?.id ?? null, hoveredDomain: published!.hoverTarget?.domain ?? null, gumballActive: false, gumballAnchorId: null })).toBe(true);
    expect(leftoverHoveredVortexFullIdV1({ hoveredId: published!.hoverTarget!.id, hoveredDomain: published!.hoverTarget!.domain })).toBe("seed-left-001:v0");
    expect(mergeWorldInteractionWithLeftoverV1({}, { ids: [], hoveredId: "seed-left-001:v0", hoveredDomain: "vortex", gumballActive: false, gumballAnchorId: null }).hoveredVortexFullId).toBe("seed-left-001:v0");
    expect(mergeWorldSelectionWithLeftoverV1({ method: "rectangle", ids: [] }, { ids: [], hoveredId: "seed-left-001:v0", hoveredDomain: "vortex", gumballActive: false, gumballAnchorId: null }).hoveredId).toBe("seed-left-001:v0");
    expect(leftoverHoveredVortexFullIdV1({ hoveredId: "seed-left-001", hoveredDomain: "vortex" })).toBeUndefined();
  });

  it("leftover activeUtility overlays guest select without a hover id", () => {
    const leftover = { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "brush" };
    expect(leftoverWorldOverlayAppliesV1(leftover)).toBe(true);
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, leftover).activeUtility).toBe("brush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, { ids: [], hoveredId: null, gumballActive: false, gumballAnchorId: null }).activeUtility).toBe("select");
  });

  it("carries the armed utility across an InteractionView leftover republish", () => {
    const armed = { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "brush" };
    const picked = { ids: ["seed-left-001"], hoveredId: "seed-left-001:v0", hoveredDomain: "vortex", gumballActive: true, gumballAnchorId: "seed-left-001" };
    const carried = leftoverOverlayCarryingUtilityV1(picked, armed);
    expect(carried.activeUtility).toBe("brush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, carried).activeUtility).toBe("brush");
    expect(leftoverOverlayCarryingUtilityV1(picked, null).activeUtility).toBeNull();
    expect(leftoverOverlayArmedBrushUtilityV1("brush")).toBe(true);
    expect(leftoverOverlayArmedBrushUtilityV1("volumeBrush")).toBe(true);
    expect(leftoverOverlayArmedBrushUtilityV1("select")).toBe(false);
    expect(leftoverOverlayCarryingUtilityV1({ ...picked, activeUtility: "select" }, armed).activeUtility).toBe("brush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, leftoverOverlayCarryingUtilityV1({ ...picked, activeUtility: "select" }, armed)).activeUtility).toBe("brush");
    expect(leftoverOverlayCarryingUtilityV1(picked, { ...armed, activeUtility: "select" }).activeUtility).toBe("select");
  });

  it("hover leftover select keeps an armed brush or volumeBrush", () => {
    const hoverSelect = { ids: [] as const, hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" as const, gumballActive: false, gumballAnchorId: null, activeUtility: "select" };
    const brush = leftoverOverlayCarryingSelectionV1(hoverSelect, { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "brush" });
    expect(brush.activeUtility).toBe("brush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, brush).activeUtility).toBe("brush");
    const volume = leftoverOverlayCarryingSelectionV1(hoverSelect, { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "volumeBrush" });
    expect(volume.activeUtility).toBe("volumeBrush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, volume).activeUtility).toBe("volumeBrush");
    const idle = leftoverOverlayCarryingSelectionV1(hoverSelect, { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select" });
    expect(idle.activeUtility).toBe("select");
  });

  it("every pane of one document names its OWN window surface, and publishes the selection it paints", async () => {
    const { default: identity } = await import("../../🧱️elements/🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json");
    const { surfaceHostIdentityV1 } = await import("../../🧱️elements/🗣️Interpreter/🟦️.tsx");
    const { worldSurfaceSelectionDomV1, worldSurfaceGuestSelectionDomV1 } = await import("../../🧱️elements/🌐️World3dHost/🟦️.tsx");

    const identities = identity.panes.map((pane) => surfaceHostIdentityV1(pane.documentSurface, pane.recordKey, pane.recordId));
    for (const [index, pane] of identity.panes.entries()) expect(identities[index], `pane ${pane.window}`).toEqual(pane.identity);
    expect(new Set(identities.map((one) => one.surfaceId)).size, "two panes of one document must not share one surface identity").toBe(identity.panes.length);
    expect(new Set(identity.panes.map((pane) => String(pane.recordId))).size, "the record id they used to be keyed on IS the same in both panes — that is the defect").toBe(1);
    expect(surfaceHostIdentityV1(identity.keylessPane.documentSurface, identity.keylessPane.recordKey, identity.keylessPane.recordId)).toEqual({ ...identity.keylessPane.identity, paneId: undefined });

    const publication = identity.selectionPublication;
    const painted = mergeWorldSelectionWithLeftoverV1(publication.guestSelectionJson as never, publication.leftoverOverlay as never, []);
    const interaction = mergeWorldInteractionWithLeftoverV1(publication.guestInteractionJson as never, publication.leftoverOverlay as never);
    expect(worldSurfaceSelectionDomV1(painted, interaction)).toEqual(publication.published);

    const objectHover = publication.objectHoverPublished;
    const objectPainted = mergeWorldSelectionWithLeftoverV1(publication.guestSelectionJson as never, objectHover.leftoverOverlay as never, []);
    const objectInteraction = mergeWorldInteractionWithLeftoverV1(publication.guestInteractionJson as never, objectHover.leftoverOverlay as never);
    const objectDom = worldSurfaceSelectionDomV1(objectPainted, objectInteraction);
    expect(objectDom.hoverTarget).toEqual(objectHover.hoverTarget);
    expect(objectDom.hoveredVortexFullId).toBe(objectHover.hoveredVortexFullId);
    expect(objectDom.activeUtility).toBe(objectHover.activeUtility);
    const guestPublished = worldSurfaceGuestSelectionDomV1(JSON.stringify(publication.guestSelectionJson));
    const { why: _guestWhy, ...guestExpected } = publication.guestPublished;
    expect(guestPublished, "the guest's own lane must be published UNMERGED, or a guest that lost the pick reads identically to one that kept it").toEqual(guestExpected);
    expect(guestPublished.selectedIds, "this fixture's guest sent nothing").toEqual([]);
    expect(publication.published.selectedIds, "while the pane paints the leftover pick — the two attributes are the discriminator").toEqual(["seed-left-001"]);
    const carried = publication.guestCarriedPick;
    const { why: _carriedWhy, ...carriedExpected } = { why: carried.why, ...carried.guestPublished };
    expect(worldSurfaceGuestSelectionDomV1(JSON.stringify(carried.guestSelectionJson)), "a guest that KEPT the pick must say so on its own lane").toEqual(carriedExpected);
  });

  it("the leftover world overlay is per window INSTANCE: arming one pane leaves its sibling's record untouched", async () => {
    const { default: identity } = await import("../../🧱️elements/🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json");
    const { surfaceHostIdentityV1 } = await import("../../🧱️elements/🗣️Interpreter/🟦️.tsx");
    const world = await import("../../🧱️elements/🌐️World3dHost/🟦️.tsx");
    const [perspective, top] = identity.panes.map((pane) => pane.window);
    const surfaces = identity.panes.map((pane) => surfaceHostIdentityV1(pane.documentSurface, pane.recordKey, pane.recordId).surfaceId);
    expect(new Set(surfaces).size, "two panes of one document bind two records").toBe(2);

    const idle = { ids: [] as readonly string[], hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select", activeToolId: null };
    world.publishLeftoverWorldSelectionV1(idle, { kind: "allWindows" });
    world.publishLeftoverWorldSelectionV1({ ...idle, activeUtility: "brush", hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" }, { kind: "window", windowId: perspective });

    const paneRecord = (windowId: string) => world.worldSurfaceSelectionDomV1({ method: "rectangle", ids: [] }, mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, world.leftoverWorldWindowOverlayV1(windowId)));
    expect(paneRecord(perspective).activeUtility, "the armed pane's own record").toBe("brush");
    expect(paneRecord(top).activeUtility, "its sibling must NOT read the arm back — one shared overlay is what made both panes publish one value").toBe("select");
    expect(paneRecord(perspective)).not.toEqual(paneRecord(top));
    expect(world.leftoverWorldWindowOverlayV1(top)?.hoveredId ?? null, "nor the armed pane's hover").toBeNull();
    expect(world.leftoverWorldArmedWindowOverlayV1()?.activeUtility, "the host still finds the one armed pane").toBe("brush");

    world.publishLeftoverWorldSelectionV1({ ...idle, ids: ["seed-left-001"], gumballActive: true, gumballAnchorId: "seed-left-001" }, { kind: "document" });
    expect(paneRecord(perspective).activeUtility, "a windowless leftover carries the document selection, never another pane's utility").toBe("brush");
    expect(paneRecord(perspective).selectedIds, "while the document's selection reaches every pane").toEqual([]);
    expect(world.leftoverWorldWindowOverlayV1(top)?.ids).toEqual(["seed-left-001"]);
    expect(world.leftoverWorldSelectionOverlayV1()?.ids, "the document-scoped readers (Outliner/Inspection) see it too").toEqual(["seed-left-001"]);

    world.publishLeftoverWorldSelectionV1({ ...idle, activeUtility: "fill", activeToolId: "fill" }, { kind: "allWindows" });
    expect(paneRecord(perspective).activeUtility, "a mode-level tool is the ONE authority that speaks for every pane").toBe("fill");
    expect(paneRecord(top).activeUtility).toBe("fill");
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    expect(world.leftoverWorldWindowOverlayV1(perspective)).toBeNull();
  });

  it("a republished equal leftover keeps every pane's overlay object and notifies nobody; one pane's hover changes only its own", async () => {
    const world = await import("../../🧱️elements/🌐️World3dHost/🟦️.tsx");
    const [a, b] = ["puzzle3d#perspective", "puzzle3d#top"];
    const idle = { ids: ["seed-left-001"] as readonly string[], hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select", activeToolId: null };
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    world.publishLeftoverWorldSelectionV1(idle, { kind: "document" });
    world.publishLeftoverWorldSelectionV1({ ...idle, hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" }, { kind: "window", windowId: a });
    const documentBefore = world.leftoverWorldSelectionOverlayV1();
    const aBefore = world.leftoverWorldWindowOverlayV1(a);
    const bBefore = world.leftoverWorldWindowOverlayV1(b);
    expect(world.leftoverWorldWindowOverlayV1(a), "a pane's overlay is one object between publications").toBe(aBefore);
    let notified = 0;
    const unsubscribe = world.subscribeLeftoverWorldSelectionV1(() => (notified += 1));
    world.publishLeftoverWorldSelectionV1({ ...idle, ids: [...idle.ids], hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" }, { kind: "window", windowId: a });
    expect(notified, "an equal republication with a fresh ids array changes nothing").toBe(0);
    expect(world.leftoverWorldSelectionOverlayV1()).toBe(documentBefore);
    expect(world.leftoverWorldWindowOverlayV1(a)).toBe(aBefore);
    expect(world.leftoverWorldWindowOverlayV1(b)).toBe(bBefore);
    world.publishLeftoverWorldSelectionV1({ ...idle, hoveredId: null, hoveredDomain: null }, { kind: "window", windowId: a });
    expect(notified, "the hover leaving pane a is one change").toBe(1);
    expect(world.leftoverWorldWindowOverlayV1(a)?.hoveredId).toBeNull();
    expect(world.leftoverWorldWindowOverlayV1(a)).not.toBe(aBefore);
    expect(world.leftoverWorldWindowOverlayV1(b), "the sibling pane keeps its object, so it does not re-render").toBe(bBefore);
    expect(world.leftoverWorldSelectionOverlayV1(), "nor do the document readers").toBe(documentBefore);
    unsubscribe();
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
  });

  it("the first leftover a pane publishes never lends its hover or utility to the document slot, so a sibling pane without its own overlay reads none", async () => {
    const world = await import("../../🧱️elements/🌐️World3dHost/🟦️.tsx");
    const [a, b] = ["puzzle3d#perspective", "puzzle3d#top"];
    const hovered = { ids: [] as readonly string[], hoveredId: "seed-left-001", hoveredDomain: "object", gumballActive: false, gumballAnchorId: null, activeUtility: "brush" };
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    world.publishLeftoverWorldSelectionV1(hovered, { kind: "window", windowId: a });
    expect(world.leftoverWorldWindowOverlayV1(a)).toMatchObject({ hoveredId: "seed-left-001", activeUtility: "brush" });
    expect(world.leftoverWorldWindowOverlayV1(b), "the sibling pane paints no hover and no arm of pane a").toMatchObject({ hoveredId: null, hoveredDomain: null, activeUtility: null });
    expect(world.leftoverWorldSelectionOverlayV1()?.hoveredId, "nor do the document readers").toBeNull();
    world.publishLeftoverWorldSelectionV1({ ...hovered, hoveredId: null, hoveredDomain: null }, { kind: "window", windowId: a });
    expect(world.leftoverWorldWindowOverlayV1(b)?.hoveredId, "the hover leaving pane a leaves nothing behind in pane b").toBeNull();
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
  });

  it("one arm authority publishes the armed pane's own overlay through explicit window scope", async () => {
    const world = await import("../../🧱️elements/🌐️World3dHost/🟦️.tsx");
    const a = "generation3d-preview#a";
    const b = "generation3d-preview#b";
    const documentSelection = { ids: ["solid-a"], hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select" } as const;
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    world.publishLeftoverWorldSelectionV1(documentSelection, { kind: "document" });
    world.publishLeftoverWorldSelectionV1({ ...documentSelection, activeUtility: "brush" }, { kind: "window", windowId: a });
    world.publishLeftoverWorldSelectionV1({ ...documentSelection, activeUtility: "volumeBrush" }, { kind: "window", windowId: b });
    expect(world.leftoverWorldWindowOverlayV1(a)).toMatchObject({ ids: ["solid-a"], activeUtility: "brush" });
    expect(world.leftoverWorldWindowOverlayV1(b)).toMatchObject({ ids: ["solid-a"], activeUtility: "volumeBrush" });
    expect(world.leftoverWorldSelectionOverlayV1()).toMatchObject({ ids: ["solid-a"], activeUtility: "select" });
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    expect(world.leftoverWorldWindowOverlayV1(a)).toBeNull();
    expect(world.leftoverWorldWindowOverlayV1(b)).toBeNull();
  });

  it("carries an armed mode tool id across hover leftovers without overriding the published utility", () => {
    const leftover = { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select", activeToolId: "tool-a" };
    expect(leftoverWorldOverlayAppliesV1(leftover)).toBe(true);
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "brush" }, leftover).activeUtility).toBe("select");
    const hoverOnly = { ids: [] as const, hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" as const, gumballActive: false, gumballAnchorId: null };
    const carried = leftoverOverlayCarryingUtilityV1(hoverOnly, leftover);
    expect(carried.activeToolId).toBe("tool-a");
    expect(mergeWorldInteractionWithLeftoverV1({}, carried).activeUtility).toBe("select");
  });

  it("first leftover pick keeps selection across hover leftover and busts the Inspection hash skip", () => {
    const hoverOnly = { ids: [] as const, hoveredId: "seed-left-001", hoveredDomain: "vortex" as const, gumballActive: false, gumballAnchorId: null };
    const priorPick = { ids: ["seed-left-001"], hoveredId: null, gumballActive: true, gumballAnchorId: "seed-left-001" };
    const carried = leftoverOverlayCarryingSelectionV1(hoverOnly, priorPick);
    expect(carried.ids).toEqual(["seed-left-001"]);
    const emptyNoHover = leftoverOverlayCarryingSelectionV1({ ids: [] as const, hoveredId: null, hoveredDomain: null, gumballActive: false, gumballAnchorId: null }, priorPick, true);
    expect(emptyNoHover.ids).toEqual([]);
    expect(mergeWorldSelectionWithLeftoverV1({ ids: ["seed-left-001"], activeObjectId: "seed-left-001", gumballActive: true }, { ids: [], hoveredId: null, gumballActive: false, gumballAnchorId: null, selectionCleared: true }).ids).toEqual([]);
    expect(leftoverTreeItemSelectedV1("seed-left-001", carried.ids)).toBe(true);
    expect(leftoverTreeItemSelectedV1("surface/seed-left-001", carried.ids)).toBe(true);
    expect(leftoverTreeItemSelectedV1("other", carried.ids)).toBe(false);
    const idsOnly = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: null,
        locked: {},
        gumball: { active: true, anchorId: "seed-left-001" },
        selection: {},
        hover: {},
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
      },
    });
    expect(leftoverInteractionStateV1(idsOnly!).selection.vortex?.ids).toEqual(["seed-left-001"]);
    const emptyVortex = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: null,
        locked: {},
        gumball: { active: true, anchorId: "seed-left-001" },
        selection: { vortex: { granularity: "object", ids: [] } },
        hover: {},
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
      },
    });
    expect(leftoverInteractionStateV1(emptyVortex!).selection.vortex?.ids).toEqual(["seed-left-001"]);
    expect(leftoverInspectionRefreshScope(["seed-left-001"])).toEqual({ kind: "full" });
    expect(leftoverInspectionPanelHash(["seed-left-001"], "abc")).toBeUndefined();
    expect(leftoverInspectionPanelHash([], "abc")).toBe("abc");
    expect(uiRefreshSectionUnchanged(leftoverInspectionPanelHash(["seed-left-001"], "abc"), { root: 0, hash: "abc" })).toBe(false);
    expect(uiRefreshSectionUnchanged("abc", { root: 0, hash: "abc" })).toBe(true);
  });

  it("empty-target interactionSelect clears selectedIds while hover may remain; hover leftover does not invent a pick", () => {
    const emptyWhileHover = leftoverSelectIdsMustNameHoverPickV1([], "seed-left-001");
    expect(emptyWhileHover).toBe(false);
    const published = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: { id: "seed-left-001", domain: "vortex", channel: "pointer" },
        locked: {},
        gumball: { active: true, anchorId: "seed-left-001" },
        selection: {},
        hover: {},
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
      },
    });
    expect(published?.selectedIds).toEqual(["seed-left-001"]);
    expect(published?.hoverTarget?.id).toBe("seed-left-001");
    expect(leftoverSelectIdsMustNameHoverPickV1(published?.selectedIds, published?.hoverTarget?.id)).toBe(true);
    const hoverOnly = leftoverOverlayCarryingSelectionV1({ ids: [] as const, hoveredId: "seed-left-001", hoveredDomain: "vortex" as const, gumballActive: false, gumballAnchorId: null }, null);
    expect(hoverOnly.ids).toEqual([]);
    expect(leftoverSelectIdsMustNameHoverPickV1(hoverOnly.ids, hoverOnly.hoveredId)).toBe(false);
  });

  it("retires a stale selection overlay against the pane's own document, and keeps the pick it exists for", async () => {
    // 🧹️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane `selection-prune-interact`. Loading another
    // bundled example replaces the whole document and the guest's own lane goes correctly empty, but
    // the host's leftover overlay is written only by the pick route and no document-replacing command
    // can retire it — a job-routed command answers through `TypedOperationCompletion`, which has no
    // leftover `output` field. Measured on generation3d 2026-09-14: `data-guest-selection-json` `[]`
    // against `data-selection-json` `["shell@solid"]` for six of eight examples, and the next plain
    // REPLACE click then read back `["fuse@solid","shell@solid"]`.
    const world = await import("../../🧱️elements/🌐️World3dHost/🟦️.tsx");
    const previewWindow = "generation3d-play-window-preview#1";
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    const picked = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["shell@solid"],
        hoverTarget: null,
        locked: {},
        gumball: { active: false, anchorId: null },
        selection: { graph: { granularity: "object", ids: ["shell@solid"] } },
        hover: {},
        activeMode: {},
        activeGranularity: {},
        windowId: previewWindow,
      },
    });
    expect(picked?.selectedIds).toEqual(["shell@solid"]);
    world.publishLeftoverWorldSelectionV1(leftoverOverlayCarryingSelectionV1({ ids: picked!.selectedIds, hoveredId: null, hoveredDomain: null, gumballActive: false, gumballAnchorId: null }, null, false), { kind: "window", windowId: previewWindow });

    // 🎯️ The pick's OWN document: the overlay is exactly the cover it exists for, so it must stand
    // even though the guest's lane has not answered yet.
    const boxShell = [{ id: "shell@solid#0", interactionId: "shell@solid", meshId: "eval-shell@solid#0" }];
    const guestSilent = { ids: [] as string[], activeObjectId: null };
    const covered = mergeWorldSelectionWithLeftoverV1(guestSilent, world.leftoverWorldWindowOverlayV1(previewWindow), boxShell);
    expect(covered.ids, "the cover the overlay exists for").toEqual(["shell@solid"]);
    expect(covered.activeObjectId).toBe("shell@solid#0");

    // 🧹️ The NEXT example's document offers none of those ids, so the same overlay is covering nothing.
    const sphereCut = [{ id: "brep_bool_cut_5@solid#0", interactionId: "brep_bool_cut_5@solid", meshId: "eval-brep_bool_cut_5@solid#0" }];
    expect(world.leftoverWorldOverlayIdsInDocumentV1(["shell@solid"], sphereCut)).toEqual([]);
    const afterSwitch = mergeWorldSelectionWithLeftoverV1(guestSilent, world.leftoverWorldWindowOverlayV1(previewWindow), sphereCut);
    expect(afterSwitch.ids, "a selection made in another document is not this pane's").toEqual([]);
    expect(afterSwitch.activeObjectId).toBeNull();
    const published = world.worldSurfaceSelectionDomV1(afterSwitch, mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, world.leftoverWorldWindowOverlayV1(previewWindow)));
    expect(published.selectedIds).toEqual([]);
    expect(published.activeObjectId).toBeNull();

    // 🎯️ A plain click REPLACES: the new pick is the whole selection, the dead id does not ride along.
    const repicked = leftoverOverlayCarryingSelectionV1({ ids: ["brep_bool_cut_5@solid", "shell@solid"], hoveredId: null, hoveredDomain: null, gumballActive: false, gumballAnchorId: null }, world.leftoverWorldWindowOverlayV1(previewWindow), false);
    world.publishLeftoverWorldSelectionV1(repicked, { kind: "window", windowId: previewWindow });
    const afterRepick = mergeWorldSelectionWithLeftoverV1(guestSilent, world.leftoverWorldWindowOverlayV1(previewWindow), sphereCut);
    expect(afterRepick.ids).toEqual(["brep_bool_cut_5@solid"]);
    expect(afterRepick.activeObjectId).toBe("brep_bool_cut_5@solid#0");

    // 🕳️ A pane that has drawn nothing yet has no membership information and keeps every id, the same
    // rule the guest's topology pruning uses for a domain it has no entry for.
    expect(world.leftoverWorldOverlayIdsInDocumentV1(["shell@solid"], [])).toEqual(["shell@solid"]);
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
  });

  it("dirties the whole shell when a direct browser-actor dispatch applied a mutation", () => {
    // 🖼️ Wave B9 lane 4: the actor handoff answers `{outcome, mutationCount}` and carries no
    // `UiDirtyScope`, and the verbs that commit inline on it (`paste`, `undo`, a gumball commit) publish
    // no `OperationCompleted` frame either — so an applied mutation must dirty the shell on the strength
    // of `mutationCount` alone, or the world lane repaints only when some later action happens to.
    expect(browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 1 })).toEqual({ kind: "full" });
    expect(browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 7 })).toEqual({ kind: "full" });
    expect(browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 })).toEqual({ kind: "none" });
    expect(browserActorDispatchUiScopeV1({ outcome: "rejected", mutationCount: 3 })).toEqual({ kind: "none" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "setGridVisible")).toEqual({ kind: "full" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "setGridSpacing")).toEqual({ kind: "full" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "setVortexShow")).toEqual({ kind: "full" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "worldPointerDown")).toEqual({ kind: "none" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "rejected", mutationCount: 0 }, "setGridVisible")).toEqual({ kind: "none" });
  });

  it("refreshes a typed operation on its completion's own scope and pays nothing for a completion that dirtied nothing", () => {
    // 🏁️ Wave B27 §2. Every typed operation is merely ADMITTED on the reply that carries `mutationCount`
    // — its edit publishes and logs on later continuations — so `browserActorDispatchUiScopeV1` above
    // answers `none` for all of them and the refresh has to follow the COMPLETION frame instead. What the
    // completion carries is the mutation's own scope, verbatim; a history patch with an empty scope still
    // owes the reserved history body its row (wave B21 put the patch on the terminal completion).
    const admission = browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 });
    expect(admission).toEqual({ kind: "none" });
    const documentScope = { kind: "partial", panelBodies: ["puzzle.3d.play.artifact", "puzzle.3d.play.inspector"], measures: true } as const;
    expect(typedOperationCompletionRefreshV1({ uiScope: documentScope, historyPatch: { cursor: 3, upserts: [{ seq: 3 }] }, requestedEffects: [] })).toEqual(documentScope);
    expect(typedOperationCompletionRefreshV1({ uiScope: undefined, historyPatch: undefined, requestedEffects: [] })).toEqual({ kind: "full" });
    expect(typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: { cursor: 4, upserts: [{ seq: 4 }] }, requestedEffects: [] })).toEqual({
      kind: "partial",
      panelBodies: [FRAMEWORK_HISTORY_BODY_KEY],
    });
    // 🫥️ …and the other half: a retained operation completes on every drain poll, measured live on the
    // puzzle 3d shell at 18 completions per idle 10 s, 12 of them empty. Those owe NO pass — paying for
    // them is a refresh storm on View-kind completions. An effect still buys the effect pass, with a
    // `none` scope so nothing is re-taken for it.
    expect(typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: undefined, requestedEffects: [] })).toBeNull();
    expect(typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: undefined, requestedEffects: ["requestSync"] })).toEqual({ kind: "none" });
  });

  it("makes an effect that rewrites a guest render input earn its own pass, and leaves host chrome paying nothing", () => {
    // 🧰️ Wave B37, the measured defect. `engagementSubmit`'s typed `brush` verb dirties NOTHING in the
    // guest — it arms `scene.active_utility`, whose only carrier to the next render is the host's own
    // utility map — so the completion declares `none`, `typedOperationCompletionRefreshV1` forwards that
    // `none` (correctly), and `refreshUi({kind:"none"})` asks for nothing at all. Live console on :6013,
    // wasm #53: `engagementSubmit … effects:1` → the host armed `puzzle3d-main-perspective → brush` →
    // the next `refresh-ui` crossing was 30 s later on an unrelated keystroke, with BOTH world panes
    // publishing `activeUtility=select` for the whole interval (`engagement-brush-verb FAIL
    // activeUtility=select waitedMs=30347`).
    const bodyKeys = ["puzzle.3d.play.viewport"] as const;
    const declared = typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: undefined, requestedEffects: [{ setActiveUtility: { windowId: "puzzle3d-main-perspective", utilityId: "brush" } }] });
    expect(declared, "the completion's own scope is still what the GUEST dirtied").toEqual({ kind: "none" });
    const earned = hostEffectRefreshScopeV1([{ setActiveUtility: { windowId: "puzzle3d-main-perspective", utilityId: "brush" } }], declared!, bodyKeys);
    expect(earned, "arming a utility is a host-owned render input — the pass that applies it owes the render that publishes it").toEqual({
      kind: "partial",
      windowBodies: ["puzzle.3d.play.viewport"],
      panelBodies: [],
      utilities: true,
      tools: true,
      engagements: false,
      measures: true,
      labels: false,
    });
    expect(hostEffectRefreshScopeV1([{ setActiveTool: { toolId: "fill" } }], { kind: "none" }, bodyKeys), "a programmatic tool switch is the same shape as a utility arm").toEqual(earned);
    expect(hostEffectRefreshScopeV1([{ setPanel: { panelJson: "{}" } }], { kind: "none" }, bodyKeys), "`setPanel` names no body of its own — `panelJson` feeds every section").toEqual({ kind: "full" });

    // 🫥️ …and the no-storm half, which is why this is a filter and not `{kind:"full"}` for every effect
    // pass: a `notify` banner is host React state, a clipboard write never reaches a projection, and a
    // `dispatchAction` comes back through shell dispatch carrying its own scope. None of them may upgrade
    // a `none` completion into a pass — a retained operation completes on every drain poll (18 per idle
    // 10 s, wave B27).
    expect(hostEffectRefreshScopeV1([{ notify: { message: "refused" } }], { kind: "none" }, bodyKeys)).toEqual({ kind: "none" });
    expect(hostEffectRefreshScopeV1([{ clipboardWrite: { text: "x" } }, { dispatchAction: { actionId: "x" } }, "requestSync"], { kind: "none" }, bodyKeys)).toEqual({ kind: "none" });
    const documentScope = { kind: "partial", panelBodies: ["puzzle.3d.play.artifact"], measures: true } as const;
    expect(hostEffectRefreshScopeV1([{ notify: { message: "done" } }], documentScope, bodyKeys), "an effect that earns nothing must leave a declared scope exactly as it is").toBe(documentScope);
    expect(hostEffectRefreshScopeV1([{ setActiveUtility: { windowId: "w", utilityId: "brush" } }], documentScope, bodyKeys), "the earned scope is UNIONED with the declared one, never substituted for it").toEqual({
      kind: "partial",
      windowBodies: ["puzzle.3d.play.viewport"],
      panelBodies: ["puzzle.3d.play.artifact"],
      utilities: true,
      tools: true,
      engagements: false,
      measures: true,
      labels: false,
    });
  });

  /** 🔁️ A pass's OWED effects are applied under the scope that pass declared — never a wider one.
   *
   * 🐛️ `ShellHost`'s ui-refresh lane handed `{ kind: "full" }` to `applyHostEffects` for every owed
   * application, so each React pick paid a whole extra full guest re-render on top of the narrowed
   * scope the interaction had just derived: `[TRACE] refreshUi lane {"decision":"owed","scope":{"kind":"full"}}`,
   * four of them per pick, which is why narrowing the pick's own scope moved its wall not at all
   * (`📓️interaction-scope-narrowing-2026-09-15.md` §5.3, whose owner this law names). The effects a
   * pass owes are `pending_effects` — `dispatchAction` shapes — and this proves they earn NOTHING of
   * their own, so `full` was the hardcode and not a derivation.
   */
  it("applies a pass's owed effects under that pass's own scope, never a hardcoded full one", () => {
    const bodyKeys = ["procedural.play.main", "procedural.play.preview", "procedural.play.generate-preview"] as const;
    const passScope = { kind: "partial", windowBodies: ["procedural.play.main", "procedural.play.preview"], panelBodies: [], utilities: false, tools: false, engagements: false, measures: false, labels: false } as const;
    const owedEffects = [{ dispatchAction: { req: 103, action: "flowEvalTick", args: {}, delayMs: 0 } }, { dispatchAction: { req: 104, action: "toolRunStart", args: { toolId: "previewEval" }, delayMs: 0 } }];
    expect(hostEffectRefreshScopeV1(owedEffects, passScope, bodyKeys), "a pass's owed pending_effects earn no scope of their own").toBe(passScope);
    expect(hostEffectRefreshScopeV1(owedEffects, { kind: "full" }, bodyKeys), "so the only thing that ever made the owed pass full was the caller").toEqual({ kind: "full" });

    const shellHostRelative = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx";
    let shellHostRoot = process.cwd();
    for (let hop = 0; hop < 12 && !existsSync(`${shellHostRoot}/${shellHostRelative}`); hop += 1) shellHostRoot = `${shellHostRoot}/..`;
    expect(existsSync(`${shellHostRoot}/${shellHostRelative}`)).toBe(true);
    const shellHost = readFileSync(`${shellHostRoot}/${shellHostRelative}`, "utf8");
    const lane = shellHost.slice(shellHost.indexOf("uiRefreshLaneRef.current = createUiRefreshCoalescerV1"), shellHost.indexOf("const refreshUi = useCallback"));
    expect(lane, "the lane applies the scope the pass recorded").toContain("applyHostEffectsRef.current(owedEffects.effects, owedEffects.session, owedEffects.scope, owedEffects.owner)");
    expect(lane.includes('{ kind: "full" }'), "and names no scope of its own").toBe(false);
    expect(shellHost, "the owed slot records the pass's scope when it records the effects").toContain("owedPassEffectsRef.current = { effects: pendingRefreshEffects, session: nextSession, owner: refreshOwner, scope: scopeArg }");
  });

  // 📚️ The language-agnostic half of the picker: which examples a surface may offer at all. Rust
  // `manifest::examples_for_dialect` answers the SAME rows in
  // `🛂️manifest/🧪️tests/🔬️example-picker/🦀️.rs` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  it("resolves the example picker by dialect, so an editor and its viewer offer exactly the same examples", () => {
    
    
    for (const useCase of examplePickerFixture.cases) {
      expect(surfaceAppId(useCase.app.dialect, useCase.app.role as AppRole), useCase.name).toBe(useCase.app.id);
      expect(
        examplesForDialect(examplePickerFixture.examples, useCase.app.dialect).map((example) => example.id),
        useCase.name,
      ).toEqual(useCase.expected);
      expect(
        examplesForApp(examplePickerFixture.examples, useCase.app).map((example) => example.id),
        useCase.name,
      ).toEqual(useCase.expected);
    }
    const editor = examplePickerFixture.cases.find((useCase) => useCase.name === "editor-of-the-dialect-offers-every-example-of-it")!;
    const viewer = examplePickerFixture.cases.find((useCase) => useCase.name === "viewer-of-the-same-dialect-offers-exactly-the-same-picker")!;
    expect(viewer.expected).toEqual(editor.expected);
    expect(viewer.app.id).not.toBe(editor.app.id);
    // 🧭️ An app whose dialect is absent (a pre-registration stub) resolves no picker rather than the
    // whole manifest — the empty-picker branch `ShellHost`'s `exampleOptions` relies on.
    expect(examplesForApp(examplePickerFixture.examples, {})).toEqual([]);
  });

  it("the example picker dispatches the chosen example id and its completion refreshes the whole shell", () => {
    // 🎨️ The picker writes the LABEL through `SET_ACTIVE_EXAMPLE_ID` and reaches the program through
    // exactly one descriptor — boot #11 changed the label while the flow window kept the previous
    // fixture, so both halves are pinned here (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    expect(buildActiveExampleAction("s.procedural.generation3d@1/*#editor", "box-shell-preview")).toEqual({
      controllerId: "s.procedural.generation3d@1/*#editor",
      action: SET_ACTIVE_EXAMPLE_ACTION_ID,
      args: { exampleId: "box-shell-preview" },
    });
    // 👁️ The read-only surface takes the exact same descriptor, addressed at ITS controller — the
    // viewer declares `setActiveExample` as a `View` action and loads the example onto its own
    // config lane (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    expect(buildActiveExampleAction("s.procedural.generation3d@1/*#viewer", "box-shell-preview")).toEqual({
      controllerId: "s.procedural.generation3d@1/*#viewer",
      action: SET_ACTIVE_EXAMPLE_ACTION_ID,
      args: { exampleId: "box-shell-preview" },
    });
    expect(buildActiveExampleAction("c", "")).toEqual({ controllerId: "c", action: "setActiveExample", args: { exampleId: "" } });
    // 🏁️ `setActiveExample` is a retained typed operation: `handleAction` only admits it, and the shell's
    // `subscribeOperationCompletions` handler is the ONLY carrier of its outcome — it must resolve the
    // completion's own scope, and an absent scope must still repaint every surface.
    expect(resolveUiDirtyScope({ kind: "full" })).toEqual({ kind: "full" });
    expect(resolveUiDirtyScope(undefined)).toEqual({ kind: "full" });
    expect(resolveUiDirtyScope({ kind: "none" })).toEqual({ kind: "none" });
  });

  it("shouldReplayIntroductionOnLoad opts a brand into replaying its tour after every window refresh", () => {
    expect(shouldReplayIntroductionOnLoad(undefined)).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain", replayIntroductionOnLoad: false })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "plain", windowTitle: "Plain" })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(false);
  });

  it("shouldStartIntroduction offers an app's tour once per session and never re-arms a veil the user dismissed", () => {
    const base = { appId: "puzzle-3d-play", hasIntroduction: true, tutorialActive: false, suppressed: false, replayOnLoad: false, seenOnDevice: false, dismissedInSession: false };
    expect(shouldStartIntroduction(base)).toBe(true);
    // 🎓️ The blocking veil owns every pointer in the app, so a dismissed tour must stay dismissed for the
    // session even though the app definition is republished (a new object, same content) on every full
    // refresh, hot-swap and re-established session — B47 §5.1 measured a full-viewport
    // `div.ui-veil.z-tutorial` at `pointer-events: auto` swallowing every press after Skip.
    const dismissedAppIds = new Set(["puzzle-3d-play"]);
    const dismissed = { ...base, dismissedInSession: dismissedAppIds.has(base.appId) };
    expect(shouldStartIntroduction(dismissed)).toBe(false);
    // …and a device-local seen flag that cannot be written (ephemeral brand ⇒ in-memory StoragePort ⇒
    // `replayOnLoad`) must not resurrect it either.
    expect(shouldStartIntroduction({ ...dismissed, replayOnLoad: true })).toBe(false);
    // A genuinely DIFFERENT app still introduces itself in the same shell (demonstrator grid, app switch).
    expect(shouldStartIntroduction({ ...dismissed, appId: "puzzle-3d-view", dismissedInSession: dismissedAppIds.has("puzzle-3d-view") })).toBe(true);
    // The device-local seen flag keeps its own meaning, and replay-on-load brands still ignore it.
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true, replayOnLoad: true })).toBe(true);
    // An app id that has not arrived yet must never arm a tour: the "" key persists nothing, so the tour
    // would arm a second time the moment the real id lands.
    expect(shouldStartIntroduction({ ...base, appId: "" })).toBe(false);
    expect(shouldStartIntroduction({ ...base, hasIntroduction: false })).toBe(false);
    expect(shouldStartIntroduction({ ...base, tutorialActive: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, suppressed: true })).toBe(false);
  });

  
  it("shouldAutoStartIntroduction offers an app's tour once per session and never re-arms a veil the user dismissed", () => {
    const base = { appId: "puzzle-3d-play", hasIntroduction: true, tutorialActive: false, suppressed: false, replayOnLoad: false, seenOnDevice: false, dismissedAppIds: new Set<string>() };
    expect(shouldAutoStartIntroduction(base)).toBe(true);
    // 🎓️ The blocking veil owns every pointer in the app, so a dismissed tour must stay dismissed for the
    // session even though the app definition is republished (a new object, same content) on every full
    // refresh, hot-swap and re-established session — B47 §5.1 measured a full-viewport
    // `div.ui-veil.z-tutorial` at `pointer-events: auto` swallowing every press after Skip.
    const dismissed = { ...base, dismissedAppIds: new Set(["puzzle-3d-play"]) };
    expect(shouldAutoStartIntroduction(dismissed)).toBe(false);
    // …and a device-local seen flag that cannot be written (ephemeral brand ⇒ in-memory StoragePort ⇒
    // `replayOnLoad`) must not resurrect it either.
    expect(shouldAutoStartIntroduction({ ...dismissed, replayOnLoad: true })).toBe(false);
    // A genuinely DIFFERENT app still introduces itself in the same shell (demonstrator grid, app switch).
    expect(shouldAutoStartIntroduction({ ...dismissed, appId: "puzzle-3d-view" })).toBe(true);
    // The device-local seen flag keeps its own meaning, and replay-on-load brands still ignore it.
    expect(shouldAutoStartIntroduction({ ...base, seenOnDevice: true })).toBe(false);
    expect(shouldAutoStartIntroduction({ ...base, seenOnDevice: true, replayOnLoad: true })).toBe(true);
    // An app id that has not arrived yet must never arm a tour: the "" key persists nothing, so the tour
    // would arm a second time the moment the real id lands.
    expect(shouldAutoStartIntroduction({ ...base, appId: "" })).toBe(false);
    expect(shouldAutoStartIntroduction({ ...base, hasIntroduction: false })).toBe(false);
    expect(shouldAutoStartIntroduction({ ...base, tutorialActive: true })).toBe(false);
    expect(shouldAutoStartIntroduction({ ...base, suppressed: true })).toBe(false);
  });
it("isEphemeralShellBrand skips durable shell state so a refresh boots from brand defaults only", () => {
    expect(isEphemeralShellBrand(undefined)).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain", ephemeral: true })).toBe(true);
    const ephemeralState = initialShellState({
      plugins: [],
      locks: { locale: "de", terminology: "reuse", themeId: "semio" },
      defaults: { exampleId: "concrete-forest" },
      // 🐚️ An in-memory storage port is now the direct analogue of the old `ephemeral: true` flag —
      // nothing persists, so every unlocked pref reads back its own built-in default, same as before.
      storage: createMemoryStoragePort(),
    });
    expect(ephemeralState.layout.activeExampleId).toBe("concrete-forest");
    expect(ephemeralState.uiPrefs.uiLocale).toBe("de");
    expect(ephemeralState.uiPrefs.uiTerminology).toBe("reuse");
    expect(ephemeralState.uiPrefs.uiThemeId).toBe("semio");
    expect(ephemeralState.uiPrefs.uiAppearance).toBe("system");
    expect(ephemeralState.uiPrefs.uiLayout).toBe("desktop");
    expect(ephemeralState.uiPrefs.uiCustomThemes).toEqual({});
    expect(ephemeralState.layout.dockOverride).toBeNull();
    expect(ephemeralState.layout.shellLayout).toBeNull();
    localStorage.setItem("ui.chrome.appearance", "dark");
    localStorage.setItem("semio.os.dock", "{}");
    localStorage.setItem("ui.introduction.seen.entwerfen-mit-bestand-aggregator:puzzle3d-play", "true");
    clearDurableShellStorage();
    expect(localStorage.getItem("ui.chrome.appearance")).toBeNull();
    expect(localStorage.getItem("semio.os.dock")).toBeNull();
    expect(localStorage.getItem("ui.introduction.seen.entwerfen-mit-bestand-aggregator:puzzle3d-play")).toBeNull();
  });

  it("buildOsCommands omits only the commands for locked prefs", () => {
    const ids = buildOsCommands([], [], false, { locale: "de", appearance: "dark" }).map((c) => c.id);
    expect(ids).not.toContain("os.setLocale");
    expect(ids).not.toContain("os.setAppearance");
    expect(ids).toContain("os.setTerminology");
    expect(ids).toContain("os.setThemeId");
  });

  it("dispatchOsCommand is a no-operation for a locked pref even if invoked directly", () => {
    const dispatch = vi.fn();
    const commit = vi.fn();
    dispatchOsCommand("os.setLocale", { locale: "de" }, commit, dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never, { locale: "en" });
    expect(commit).not.toHaveBeenCalled();
    expect(dispatch).not.toHaveBeenCalled();
  });
});

describe("buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel)", () => {
  const definition = (id: string, label: string, category: string, args: ResolvedActionArgDef[] = []): ResolvedCommand["definition"] => ({
    id,
    label,
    category,
    args,
    iconId: "wrench",
    semantics: actionSemanticsForKind("shell"),
    kind: "shell",
    keybindings: [],
    inPalette: true,
  });
  const zeroArgCommand: ResolvedCommand = { definition: definition("os.resetDock", "Reset Dock", "layout"), address: { owner: "os", commandId: "os.resetDock" } };
  const argCommand: ResolvedCommand = {
    definition: definition("os.setThemeId", "Set Theme", "appearance", [{ id: "themeId", label: "Theme", schema: { kind: "string", options: [] }, required: true }]),
    address: { owner: "os", commandId: "os.setThemeId" },
  };
  const secondArgCommand: ResolvedCommand = {
    definition: definition("os.setAppearance", "Set Appearance", "appearance", [{ id: "appearance", label: "Appearance", schema: { kind: "string", options: [] }, required: true }]),
    address: { owner: "os", commandId: "os.setAppearance" },
  };
  const singletonArgCommand: ResolvedCommand = {
    definition: definition("os.setDriver", "Set Driver", "general", [{ id: "driver", label: "Driver", schema: { kind: "string", options: [] }, required: true }]),
    address: { owner: "os", commandId: "os.setDriver" },
  };

  it("a zero-arg command row fires onExecute directly on click; only one command-list section is present when nothing is expanded", () => {
    const onExecute = vi.fn();
    const tree = buildCommandCategoryTree([zeroArgCommand], null, {}, onExecute, vi.fn(), vi.fn(), vi.fn());
    expect(tree.sections).toHaveLength(1);
    const row = tree.sections[0]!.items!.find((item) => item.id === "command.os.os.resetDock")!;
    expect(row.label).toBe("Reset Dock");
    row.onClick?.(...treeDataActivation());
    expect(onExecute).toHaveBeenCalledWith(zeroArgCommand);
  });

  it("auto-expands a singleton arg-carrying category into a flat form with section actions and no disclosure list", () => {
    const tree = buildCommandCategoryTree([singletonArgCommand], null, {}, vi.fn(), vi.fn(), vi.fn(), vi.fn());
    expect(tree.sections).toHaveLength(1);
    expect(tree.sections[0]!.id).toBe("command.category.general.form");
    expect(tree.sections[0]!.items?.map((item) => item.id)).toEqual(["command.os.os.setDriver.arg.driver"]);
    expect(tree.sections[0]!.actions?.map((action) => action.id)).toEqual(["command-os.os.setDriver-execute", "command-os.os.setDriver-reset"]);
  });

  it("an arg-carrying command row toggles expansion instead of executing, and a synthetic arg-form section only appears while expanded", () => {
    const onToggleExpanded = vi.fn();
    const collapsedTree = buildCommandCategoryTree([argCommand, secondArgCommand], null, {}, vi.fn(), onToggleExpanded, vi.fn(), vi.fn());
    expect(collapsedTree.sections).toHaveLength(1);
    const collapsedRow = collapsedTree.sections[0]!.items!.find((item) => item.id === "command.os.os.setThemeId")!;
    expect(collapsedRow.label).toBe("Set Theme…");
    collapsedRow.onClick?.(...treeDataActivation());
    expect(onToggleExpanded).toHaveBeenCalledWith("os:os.setThemeId");

    const expandedTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os:os.setThemeId", {}, vi.fn(), vi.fn(), vi.fn(), vi.fn());
    expect(expandedTree.sections).toHaveLength(2);
    const formItems = expandedTree.sections[0]!.items!;
    expect(formItems.find((item) => item.id === "command.os.os.setThemeId.arg.themeId")?.label).toBe("Theme");
    expect(expandedTree.sections[0]!.actions?.map((action) => action.id)).toEqual(["command-os.os.setThemeId-execute", "command-os.os.setThemeId-reset"]);
    expect(expandedTree.sections[1]!.items?.map((item) => item.id)).toEqual(["command.os.os.setAppearance"]);
  });

  it("Execute is disabled until the required arg is staged, and calling it passes the effective (staged) args; Reset dispatches onResetArgs", () => {
    const onExecute = vi.fn();
    const onStageArg = vi.fn();
    const onResetArgs = vi.fn();

    const missingTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os:os.setThemeId", {}, onExecute, vi.fn(), onStageArg, onResetArgs);
    const missingExecute = missingTree.sections[0]!.actions!.find((action) => action.id === "command-os.os.setThemeId-execute")!;
    expect(missingExecute.disabled).toBe(true);

    const stagedTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os:os.setThemeId", { "os:os.setThemeId": { themeId: "semio" } }, onExecute, vi.fn(), onStageArg, onResetArgs);
    const stagedExecute = stagedTree.sections[0]!.actions!.find((action) => action.id === "command-os.os.setThemeId-execute")!;
    const stagedReset = stagedTree.sections[0]!.actions!.find((action) => action.id === "command-os.os.setThemeId-reset")!;
    expect(stagedExecute.disabled).toBe(false);
    if (stagedExecute.kind === "checkbox" || stagedReset.kind === "checkbox") throw new Error("Command fixture requires execute and reset buttons");
    stagedExecute.onClick();
    expect(onExecute).toHaveBeenCalledWith(argCommand, { themeId: "semio" });
    stagedReset.onClick();
    expect(onResetArgs).toHaveBeenCalledWith("os:os.setThemeId");
  });

  it("buildCommandCategoryTabs builds one namespaced PanelTabLeaf per category, whose lazily-resolved tree only contains that category's commands", () => {
    const categories = [
      { id: "layout", label: "Layout" },
      { id: "appearance", label: "Appearance" },
    ];
    const expandedRef = { current: null as string | null };
    const stagedRef = { current: {} as Readonly<Record<string, Readonly<Record<string, unknown>>>> };
    const onCommand = vi.fn();
    const dispatch = vi.fn();
    const tabs = buildCommandCategoryTabs([zeroArgCommand, argCommand], categories, expandedRef, stagedRef, onCommand, dispatch);
    expect(tabs.map((tab) => tab.id)).toEqual(["command.category.layout", "command.category.appearance"]);
    expect(tabs.every((tab) => tab.kind === "leaf")).toBe(true);

    const layoutLeaf = tabs[0]!;
    expect(layoutLeaf.kind).toBe("leaf");
    const resolved = layoutLeaf.kind === "leaf" ? (layoutLeaf.trees[0]!.tree as { resolveTree: () => { sections: { items?: { id: string }[] }[] } }).resolveTree() : { sections: [] };
    expect(resolved.sections[0]!.items?.map((item) => item.id)).toEqual(["command.os.os.resetDock"]);

    // Executing routes through the injected onCommand with the command's own source.
    const executeRow = resolved.sections[0]!.items!.find((item: { id: string }) => item.id === "command.os.os.resetDock") as unknown as { onClick: (event: never, context: never) => void };
    executeRow.onClick({} as never, {} as never);
    expect(onCommand).toHaveBeenCalledWith({ owner: "os", commandId: "os.resetDock" }, undefined);
  });
});

describe("host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames)", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it("encodes recursive host-effect actions as fully scoped JSON at the runtime boundary", () => {
    const baseSession = {
      pluginId: "demonstrator",
      app: { id: "cad-play", defaultModeId: "edit", modes: [{ id: "edit" }], windowKinds: [{ id: "shape" }] },
      viewState: { activeModeId: "edit", activeWindowKindId: "shape", windowId: "shape-2" },
    } as unknown as Parameters<typeof encodeEffectActionInvocation>[0];
    expect(JSON.parse(encodeEffectActionInvocation(baseSession, "dispatchNext", { jobId: "job-1" }))).toEqual({
      address: {
        pluginId: "demonstrator",
        appId: "cad-play",
        modeId: "edit",
        windowKindId: "shape",
        windowInstanceId: "shape-2",
        actionId: "dispatchNext",
      },
      arguments: { jobId: "job-1", windowId: "shape-2" },
    });
    expect(JSON.parse(encodeEffectCommandInvocation(baseSession, "flowEvalTick"))).toEqual({
      address: { owner: { app: { pluginId: "demonstrator", appId: "cad-play" } }, commandId: "flowEvalTick" },
      arguments: {},
    });
  });

  it("scheduleDispatchAction (D2): a positive delayMs is a real deadline, honoured to the millisecond", () => {
    const host = createVirtualContinuationHost();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    scheduleDispatchAction("advanceReconstruction", { jobId: "job-1" }, 250, dispatchOne, createContinuationScheduler(host.ports));
    expect(dispatchOne).not.toHaveBeenCalled();
    host.drain(249);
    expect(dispatchOne).not.toHaveBeenCalled();
    host.drain(250);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("advanceReconstruction", { jobId: "job-1" });
  });

  it("scheduleDispatchAction (D2): delayMs 0 defers to a continuation and never arms a host timer", () => {
    const host = createVirtualContinuationHost();
    const armed: number[] = [];
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    scheduleDispatchAction(
      "tick",
      undefined,
      0,
      dispatchOne,
      createContinuationScheduler({
        ...host.ports,
        setTimer: (run, delayMs) => {
          armed.push(delayMs);
          return host.ports.setTimer(run, delayMs);
        },
      }),
    );
    expect(dispatchOne).not.toHaveBeenCalled();
    host.drain();
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("tick", undefined);
    expect(armed).toEqual([]);
    expect(host.nowMs()).toBe(0);
  });

  // 🪃️ THE regression law for ticket 26/09/09/PROCEDURAL-3D-END-TO-END's ~24 s extension hop. The guest
  // re-arms with `Effect::DispatchAction { delay_ms: 0 }` from inside the previous dispatch's own
  // callback; on a nested `setTimeout` chain a hidden/unfocused/headless Chrome renderer clamps that to
  // ~1 tick/s, which is what turned a 0.58 s native chain into 112 s in the browser. This simulates the
  // clamp by making EVERY `globalThis.setTimeout` a second late and then asserts the ten-hop chain still
  // finishes in well under one clamped tick — impossible unless the re-arm avoids timers entirely.
  it("scheduleDispatchAction (D2): a ten-hop re-arm chain beats a throttled setTimeout by two orders of magnitude", async () => {
    vi.useRealTimers();
    const realSetTimeout = globalThis.setTimeout;
    const THROTTLE_MS = 1_000;
    const HOPS = 10;
    let throttledCalls = 0;
    globalThis.setTimeout = ((handler: TimerHandler, timeout?: number, ...args: unknown[]) => {
      throttledCalls += 1;
      return realSetTimeout(handler as () => void, (timeout ?? 0) + THROTTLE_MS, ...(args as []));
    }) as unknown as typeof globalThis.setTimeout;
    try {
      const patchProbe = throttledCalls;
      globalThis.setTimeout(() => {}, 0);
      expect(throttledCalls).toBe(patchProbe + 1);
      const before = throttledCalls;
      const startedAtMs = Date.now();
      const hops: string[] = [];
      await new Promise<void>((resolve) => {
        const dispatchOne = async (action: string): Promise<void> => {
          hops.push(action);
          if (hops.length >= HOPS) return resolve();
          scheduleDispatchAction(`hop#${hops.length + 1}`, undefined, 0, dispatchOne);
        };
        scheduleDispatchAction("hop#1", undefined, 0, dispatchOne);
      });
      const elapsedMs = Date.now() - startedAtMs;
      expect(hops).toEqual(Array.from({ length: HOPS }, (_, index) => `hop#${index + 1}`));
      expect(throttledCalls).toBe(before);
      expect(elapsedMs).toBeLessThan(THROTTLE_MS / 4);
    } finally {
      globalThis.setTimeout = realSetTimeout;
    }
  });

  it("dispatchOpenedFiles (D3): single-file (multiple=false) makes exactly one call with {payload, name} plus the one-chunk envelope and no index/total", async () => {
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    await dispatchOpenedFiles([{ contents: "abc", name: "a.png" }], "importFramePayload", false, dispatchOne);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("importFramePayload", { payload: "abc", name: "a.png", chunk: 0, chunkCount: 1 });
  });

  it("dispatchOpenedFiles (B59): a file above one chunk is dispatched in order as chunks that each stay inside the guest contiguous ceiling", async () => {
    const calls: { readonly payload: string; readonly chunk: number; readonly chunkCount: number; readonly name: string }[] = [];
    const dispatchOne = vi.fn().mockImplementation(async (_action: string, args: unknown) => {
      calls.push(args as (typeof calls)[number]);
    });
    // 📏️ The product's own payload class: the Nakagin Capsule Tower export is 145 714 B (wave B53/B57).
    const contents = `{"schema":"puzzle.3d.fixture","objects":[${"x".repeat(145_000)}]}`;
    await dispatchOpenedFiles([{ contents, name: "nakagin-capsule-tower.json" }], "importSnapshot", false, dispatchOne);
    expect(calls.length).toBe(Math.ceil(contents.length / IMPORT_CHUNK_BYTES));
    expect(calls.map((call) => call.chunk)).toEqual(calls.map((_, index) => index));
    expect(new Set(calls.map((call) => call.chunkCount))).toEqual(new Set([calls.length]));
    expect(new Set(calls.map((call) => call.name))).toEqual(new Set(["nakagin-capsule-tower.json"]));
    for (const call of calls) expect(new TextEncoder().encode(call.payload).length).toBeLessThanOrEqual(IMPORT_CHUNK_BYTES);
    expect(calls.map((call) => call.payload).join("")).toBe(contents);
  });

  it("importPayloadChunks (B59): slices by UTF-8 extent so a non-ASCII label can never overrun the chunk cap, and never splits a code point", () => {
    const label = "Distinct Capsule J · cs_sl1 ōō";
    const contents = label.repeat(4_000);
    const chunks = importPayloadChunks(contents);
    expect(chunks.length).toBeGreaterThan(1);
    for (const chunk of chunks) expect(new TextEncoder().encode(chunk.payload).length).toBeLessThanOrEqual(IMPORT_CHUNK_BYTES);
    expect(chunks.map((chunk) => chunk.payload).join("")).toBe(contents);
    expect(importPayloadChunks("")).toEqual([{ payload: "", chunk: 0, chunkCount: 1 }]);
  });

  it("dispatchOpenedFiles (D3): multiple=true dispatches once per file, in order, each extended with {index, total}", async () => {
    const calls: unknown[][] = [];
    const dispatchOne = vi.fn().mockImplementation(async (action: string, args: unknown) => {
      calls.push([action, args]);
    });
    const opened = [
      { contents: "a", name: "a.png" },
      { contents: "b", name: "b.png" },
      { contents: "c", name: "c.png" },
    ];
    await dispatchOpenedFiles(opened, "importFramePayload", true, dispatchOne);
    expect(dispatchOne).toHaveBeenCalledTimes(3);
    expect(calls).toEqual([
      ["importFramePayload", { payload: "a", name: "a.png", chunk: 0, chunkCount: 1, index: 0, total: 3 }],
      ["importFramePayload", { payload: "b", name: "b.png", chunk: 0, chunkCount: 1, index: 1, total: 3 }],
      ["importFramePayload", { payload: "c", name: "c.png", chunk: 0, chunkCount: 1, index: 2, total: 3 }],
    ]);
  });

  it("sampleMediaFrameTimestampsMs (D5): steps by sampleStride/fpsHint seconds, capped at maxFrames", () => {
    // Expected timestamps mirror the implementation's own `k * stepMs` computation exactly (bit-for-bit)
    // rather than independently-derived literals — plain float division/multiplication isn't perfectly
    // associative, so e.g. `k * (5 / 30 * 1000)` and `(k * 5000) / 30` can differ by a ULP.
    const stepAt5_30 = (5 / 30) * 1000;
    expect(sampleMediaFrameTimestampsMs(1000, 5, 0, 30)).toEqual([0, 1, 2, 3, 4, 5].map((k) => k * stepAt5_30));
    expect(sampleMediaFrameTimestampsMs(1000, 5, 2, 30)).toEqual([0, stepAt5_30]);
    // sampleStride 0 floors to 1, fpsHint 0 falls back to 30 — never divides by zero.
    const stepAt1_30 = (1 / 30) * 1000;
    expect(sampleMediaFrameTimestampsMs(100, 0, 0, 0)).toEqual([0, 1, 2].map((k) => k * stepAt1_30));
    expect(sampleMediaFrameTimestampsMs(0, 5, 10, 30)).toEqual([]);
  });

  //#region 🔌️jsdom media mocks
  /** 🎞️ jsdom has no real media decoder — `<video>`'s `duration`/`videoWidth`/`videoHeight` are
   * read-only getters that never change from a `src` assignment, and `currentTime` is a no-operation setter
   * that never fires `seeked`. This stubs both to the minimum needed for `runTier2VideoFrames`'s
   * seek-and-capture loop: `currentTime` synchronously (via microtask) fires `seeked`, mirroring how a
   * real browser resolves a seek asynchronously without needing fake timers in these tests. */
  function mockVideoElement(durationMs: number, width: number, height: number): HTMLVideoElement {
    const video = document.createElement("video");
    Object.defineProperty(video, "duration", { value: durationMs / 1000, configurable: true });
    Object.defineProperty(video, "videoWidth", { value: width, configurable: true });
    Object.defineProperty(video, "videoHeight", { value: height, configurable: true });
    Object.defineProperty(video, "readyState", { value: 1, configurable: true });
    Object.defineProperty(video, "currentTime", {
      configurable: true,
      get() {
        return 0;
      },
      set() {
        queueMicrotask(() => video.dispatchEvent(new Event("seeked")));
      },
    });
    return video;
  }

  function mockCanvasCapture(): void {
    HTMLCanvasElement.prototype.getContext = vi.fn().mockReturnValue({ drawImage: vi.fn() }) as unknown as typeof HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.toDataURL = vi.fn().mockReturnValue("data:image/jpeg;base64,ZmFrZQ==");
  }
  //#endregion 🔌️jsdom media mocks

  it("runTier2VideoFrames (D5): dispatches frameAction once per sampled timestamp, in order, then doneAction exactly once", async () => {
    mockCanvasCapture();
    const video = mockVideoElement(200, 64, 48);
    const calls: { action: string; args: Record<string, unknown> }[] = [];
    const dispatchOne = vi.fn().mockImplementation(async (action: string, args: Record<string, unknown>) => {
      calls.push({ action, args });
    });
    await runTier2VideoFrames(video, { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 5, maxFrames: 0, maxLongEdgePx: 0, fpsHint: 30, args: { streamId: "s1" } }, "clip.mp4", dispatchOne);
    const frameCalls = calls.filter((call) => call.action === "frame");
    const doneCalls = calls.filter((call) => call.action === "done");
    expect(frameCalls.length).toBeGreaterThan(0);
    expect(doneCalls).toHaveLength(1);
    // frame/done ordering: every frame dispatch precedes the single done dispatch.
    expect(calls.at(-1)!.action).toBe("done");
    expect(frameCalls.map((call) => call.args.index)).toEqual(frameCalls.map((_, index) => index));
    expect(frameCalls[0]!.args).toMatchObject({ payload: "data:image/jpeg;base64,ZmFrZQ==", name: "clip.mp4", streamId: "s1" });
    expect(doneCalls[0]!.args).toMatchObject({ name: "clip.mp4", frameCount: frameCalls.length, sampledCount: frameCalls.length, streamId: "s1" });
  });

  it("runMediaFramesV1 (D5): Tier 2 failure (video element throws mid-seek) ⇒ dispatches fallbackAction exactly once with raw bytes as a data URL, no frame/done calls", async () => {
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    const throwingVideo = mockVideoElement(1000, 16, 16);
    Object.defineProperty(throwingVideo, "currentTime", {
      configurable: true,
      get() {
        return 0;
      },
      set() {
        throw new Error("decode failed");
      },
    });
    const source = await requestMediaFramesSourceV1("video/mp4", payload);
    expect(await runMediaFramesV1({ frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 2, maxLongEdgePx: 0, fpsHint: 30 }, source!, dispatchOne, { actions: [] }, new AbortController().signal, undefined, () => throwingVideo)).toBe("done");
    expect(dispatchOne).toHaveBeenCalledTimes(1);
    const [action, args] = dispatchOne.mock.calls[0]! as [string, Record<string, unknown>];
    expect(action).toBe("fallback");
    expect(args.name).toBe("video");
    expect(String(args.payload)).toMatch(/^data:video\/mp4;base64,/);
  });

  it("runMediaFramesV1 (D5): payload bytes in hand ⇒ Tier 2 seek-capture runs, reporting every frame, ending in doneAction (no picker needed)", async () => {
    mockCanvasCapture();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    const progress: [number, number][] = [];
    const source = await requestMediaFramesSourceV1("video/mp4", payload);
    expect(source?.name).toBe("video");
    expect(await runMediaFramesV1({ frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 2, maxLongEdgePx: 0, fpsHint: 30 }, source!, dispatchOne, { actions: [] }, new AbortController().signal, (completed, total) => progress.push([completed, total]), () => mockVideoElement(1000, 16, 16))).toBe("done");
    const actions = dispatchOne.mock.calls.map((call) => call[0] as string);
    expect(actions).toEqual(["frame", "frame", "done"]);
    expect(progress).toEqual([[1, 2], [2, 2]]);
  });

  it("runMediaFramesV1 (D5): a host cancel stops before the next frame — no done, no fallback; `importAbort` only when a frame reached a declaring guest", async () => {
    mockCanvasCapture();
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    const args = { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 4, maxLongEdgePx: 0, fpsHint: 30 } as const;
    const run = async (declares: boolean, cancelAfter: number | "before") => {
      const controller = new AbortController();
      if (cancelAfter === "before") controller.abort();
      const dispatched: string[] = [];
      const dispatchOne = vi.fn().mockImplementation(async (action: string) => {
        dispatched.push(action);
        if (dispatched.filter((entry) => entry === "frame").length === cancelAfter) controller.abort();
      });
      const outcome = await runMediaFramesV1(args, (await requestMediaFramesSourceV1("video/mp4", payload))!, dispatchOne, { actions: declares ? [{ id: "importAbort" }] : [] } as unknown as Parameters<typeof runMediaFramesV1>[3], controller.signal, undefined, () => mockVideoElement(1000, 16, 16));
      return { outcome, dispatched };
    };
    expect(await run(true, 1)).toEqual({ outcome: "cancelled", dispatched: ["frame", "importAbort"] });
    expect(await run(true, 2)).toEqual({ outcome: "cancelled", dispatched: ["frame", "frame", "importAbort"] });
    expect(await run(false, 1)).toEqual({ outcome: "cancelled", dispatched: ["frame"] });
    expect(await run(true, "before")).toEqual({ outcome: "cancelled", dispatched: [] });
  });
});

describe("Display Windows tab — projection drag templates", () => {
  function windowsTreeSections(windowKinds: DisplayHostApi["windowKinds"]) {
    const host: DisplayHostApi = {
      windowKinds,
      namedLayouts: [],
      userLayouts: [],
      saveCurrentLayout: () => {},
      applyNamedLayout: () => {},
      deleteUserLayout: () => {},
      layoutSaveLabel: "",
      setLayoutSaveLabel: () => {},
    };
    const tabs = createFrameworkDisplayPanelTabs(() => host);
    const windowsTab = tabs.find((tab) => tab.id === "framework.display.windows");
    if (!windowsTab || windowsTab.kind !== "leaf") throw new Error("Display fixture requires the windows tree leaf");
    const source = windowsTab.trees[0]?.tree;
    if (!source) throw new Error("Display fixture requires the windows tree source");
    return ("resolveTree" in source ? source.resolveTree() : source).sections;
  }

  type LabeledTreeItem = { readonly id: string; readonly label?: string; readonly icon?: unknown; readonly items?: readonly LabeledTreeItem[]; readonly dragData?: Record<string, string> };
  const byLabel = (items: readonly LabeledTreeItem[], label: string) => items.find((row) => row.label === label);

  it("shows window kind icons on section headers and kind rows", () => {
    const sections = windowsTreeSections([{ id: "puzzle2d-overview", label: "Overview", iconId: "layout-grid", surfaceKind: "canvas-2d" }]);
    expect(sections[0]!.icon).toBeTruthy();
    const items = sections[0]!.items as LabeledTreeItem[];
    expect(items[0]?.icon).toBeTruthy();
  });

  it("nests the full Parallel/Perspective projection taxonomy under a world-3d window kind", () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", iconId: "puzzle", surfaceKind: "world-3d" }]);
    expect(sections).toHaveLength(1);
    const items = sections[0]!.items as LabeledTreeItem[];
    expect(items).toHaveLength(3);
    expect(items.some((row) => row.id === "framework.display.windows.puzzle3dMain.kind")).toBe(true);
    const parallel = byLabel(items, "Parallel")!;
    const perspective = byLabel(items, "Perspective")!;
    expect(perspective).toBeDefined();
    expect(parallel.items!.map((row) => row.id.split(".").pop()).sort()).toEqual(["axonometric", "oblique", "orthographic"]);
  });

  it("keeps a flat single drag entry for non-world-3d window kinds", () => {
    const sections = windowsTreeSections([{ id: "puzzle2d-overview", label: "Overview", iconId: "layout-grid", surfaceKind: "canvas-2d" }]);
    expect(sections[0]!.items).toHaveLength(1);
  });

  it('pre-reverses every level so the bottom-anchored (direction="up") Tree\'s own sibling-reversal renders Parallel children top-to-bottom', () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", iconId: "puzzle", surfaceKind: "world-3d" }]);
    const items = sections[0]!.items as LabeledTreeItem[];
    const parallel = byLabel(items, "Parallel")!;
    // Raw (pre-render) order is reversed once more so that after the Tree's own "up" reversal on render,
    // Parallel reads Orthographic, Axonometric, Oblique — top to bottom.
    expect([...parallel.items!].reverse().map((row) => row.label)).toEqual(["Orthographic", "Axonometric", "Oblique"]);
    const axonometric = byLabel(parallel.items!, "Axonometric")!;
    expect([...axonometric.items!].reverse().map((row) => row.label)).toEqual(["Isometric", "Dimetric", "Trimetric"]);
  });

  it("each projection leaf's drag payload decodes back to its WorldProjectionSpec", () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", iconId: "puzzle", surfaceKind: "world-3d" }]);
    const items = sections[0]!.items as LabeledTreeItem[];
    const parallel = byLabel(items, "Parallel")!;
    const orthographic = byLabel(parallel.items!, "Orthographic")!;
    const payload = JSON.parse(orthographic.dragData!["application/x-compose-window-template"]!) as { windowKindId: string; templateId: string };
    expect(payload.windowKindId).toBe("puzzle3d-main");
    expect(decodeWorldProjectionTemplateId(payload.templateId)).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } });
    const axonometric = byLabel(parallel.items!, "Axonometric")!;
    const isometric = byLabel(axonometric.items!, "Isometric")!;
    const isoPayload = JSON.parse(isometric.dragData!["application/x-compose-window-template"]!) as { windowKindId: string; templateId: string };
    expect(decodeWorldProjectionTemplateId(isoPayload.templateId)).toMatchObject({ mode: { kind: "axonometric", variant: "isometric" } });
  });
});

describe("createFrameworkSettingsPanelTab", () => {
  it("exposes one Settings toggle whose children are General, Theme, and Hotkeys tabs", () => {
    const settings = createFrameworkSettingsPanelTab(() => null);
    expect(settings.kind).toBe("branch");
    if (settings.kind !== "branch") throw new Error("Settings must be a branch");
    expect(settings.id).toBe("framework.settings");
    expect(settings.children.map((child) => child.id)).toEqual(["framework.settings.general", "framework.settings.theme", "framework.settings.keybindings"]);
    expect(settings.children.map((child) => child.name)).toEqual(["General", "Theme", "Hotkeys"]);
  });
});

describe("integrateAppSettingsPanelTabsIntoFrameworkBranch", () => {
  it("nests app Settings-group tabs inside framework.settings so the bottom-right anchor carries one Settings toggle", () => {
    const framework = createFrameworkSettingsPanelTab(() => null);
    const appTab = singleTreeLeaf({
      id: "puzzle3d.panel.settings",
      icon: shellTabIcon("settings"),
      name: "Einstellungen",
      order: 0,
      tree: { resolveTree: () => ({ sections: [{ id: "puzzle3d-play-settings", items: [] }] }) },
    });
    const integrated = integrateAppSettingsPanelTabsIntoFrameworkBranch(framework, [appTab]);
    expect(integrated.kind).toBe("branch");
    if (integrated.kind !== "branch") throw new Error("integrated settings must be a branch");
    expect(integrated.id).toBe("framework.settings");
    expect(integrated.children[0]?.id).toBe("puzzle3d.panel.settings");
    expect(integrated.children.map((child) => child.id)).toEqual(["puzzle3d.panel.settings", "framework.settings.general", "framework.settings.theme", "framework.settings.keybindings"]);
  });
});

describe("createFrameworkMarketplacePanelTab", () => {
  type LabeledTreeItem = { readonly id: string; readonly label?: string; readonly loading?: boolean; readonly items?: readonly LabeledTreeItem[]; readonly control?: ReactElement };
  type MarketplaceTreeSections = { readonly sections: readonly { readonly id: string; readonly label?: string; readonly items?: readonly LabeledTreeItem[] }[] };
  type LeafWithTrees = { readonly trees: readonly { readonly tree: { readonly resolveTree: () => MarketplaceTreeSections } }[] };

  function marketplaceTreeSections(host: MarketplaceHostApi | null) {
    const marketplaceTab = createFrameworkMarketplacePanelTab(() => host) as unknown as LeafWithTrees;
    return marketplaceTab.trees[0]!.tree.resolveTree().sections;
  }

  it("shows an unavailable placeholder when no host is mounted yet", () => {
    const sections = marketplaceTreeSections(null);
    expect(sections).toHaveLength(1);
    expect(sections[0]!.items?.[0]?.id).toBe("unavailable");
  });

  const plugin = (pluginId: string, status: PluginPanelStatus, canUninstall: boolean): MarketplacePluginEntry => ({ pluginId, label: pluginId, version: "1", status, sourceId: "dev", canUninstall });
  const extension = (extensionId: string, extendsHost: string, enabled = true): MarketplaceExtensionEntry => ({ extensionId, label: extensionId, version: "1", extendsHost, enabled, status: "loaded" });
  const host = (overrides: Partial<MarketplaceHostApi> = {}): MarketplaceHostApi => ({
    plugins: [],
    extensions: [],
    installPlugin: () => {},
    uninstallPlugin: () => {},
    reloadPlugin: () => {},
    installExtensionFromUrl: () => {},
    installExtensionFromFile: () => {},
    uninstallExtension: () => {},
    setExtensionEnabled: () => {},
    ...overrides,
  });

  it("groups plugins into one section per source, sorted by pluginId within a source", () => {
    const sections = marketplaceTreeSections(host({ plugins: [plugin("s", "loaded", false), plugin("note", "loaded", true)] }));
    expect(sections.map((section) => section.id)).toEqual(["framework.marketplace.extensions.install", "framework.marketplace.source.dev"]);
    expect(sections[1]!.items?.map((item) => item.id)).toEqual(["framework.marketplace.plugin.note", "framework.marketplace.plugin.s"]);
  });

  it("integrates extensions as children of their owning plugin", () => {
    const sections = marketplaceTreeSections(
      host({
        plugins: [plugin("flow", "loaded", true), plugin("s", "loaded", false)],
        extensions: [extension("flow.brep", "flow"), extension("flow.math", "flow", false)],
      }),
    );
    const flow = sections[1]!.items?.find((item) => item.id === "framework.marketplace.plugin.flow");
    expect(flow?.items?.map((item) => item.id)).toEqual(["framework.marketplace.plugin.flow.extension.flow.brep", "framework.marketplace.plugin.flow.extension.flow.math"]);
    expect(sections.some((section) => section.id.includes("extensions.host"))).toBe(false);
  });

  it("marks installing/reloading rows as loading, and every status is reflected in the row label", () => {
    const items = marketplaceTreeSections(host({ plugins: [plugin("a", "available", true), plugin("b", "installing", true), plugin("c", "loaded", true), plugin("d", "failed", true), plugin("e", "reloading", true)] }))[1]!.items!;
    const byId = (pluginId: string) => items.find((item) => item.id === `framework.marketplace.plugin.${pluginId}`)!;
    expect(byId("a").loading).toBe(false);
    expect(byId("b").loading).toBe(true);
    expect(byId("c").loading).toBe(false);
    expect(byId("d").loading).toBe(false);
    expect(byId("e").loading).toBe(true);
    expect(byId("a").label).toContain("Available");
    expect(byId("b").label).toContain("Installing");
    expect(byId("c").label).toContain("Loaded");
    expect(byId("d").label).toContain("Failed");
    expect(byId("e").label).toContain("Reloading");
  });

  it("routes install/uninstall/reload clicks for one row back through the host without touching others", () => {
    const calls: string[] = [];
    const sections = marketplaceTreeSections(
      host({
        plugins: [plugin("note", "loaded", true)],
        uninstallPlugin: (pluginId) => calls.push(`uninstall:${pluginId}`),
        reloadPlugin: (pluginId) => calls.push(`reload:${pluginId}`),
      }),
    );
    const noteItem = sections[1]!.items![0]!;
    const { getByText } = render(createElement("div", null, noteItem.control));
    fireEvent.click(getByText("Reload"));
    fireEvent.click(getByText("Uninstall"));
    expect(calls).toEqual(["reload:note", "uninstall:note"]);
    cleanup();
  });

  it("disables uninstall for the host/primary plugin and the active session's plugin (canUninstall: false)", () => {
    const sections = marketplaceTreeSections(host({ plugins: [plugin("s", "loaded", false)] }));
    const sItem = sections[1]!.items![0]!;
    const { getByText } = render(createElement("div", null, sItem.control));
    expect((getByText("Uninstall").closest("button") as HTMLButtonElement).disabled).toBe(true);
    cleanup();
  });
});

describe("introductionTargetsWindow", () => {
  it("matches both the window kind and every open instance of that kind", () => {
    expect(introductionTargetsWindow("puzzle3d-main", "puzzle3d-main", "puzzle3d-main")).toBe(true);
    expect(introductionTargetsWindow("puzzle3d-main-top", "puzzle3d-main", "puzzle3d-main")).toBe(true);
    expect(introductionTargetsWindow("puzzle3d-main-perspective", "puzzle3d-main", "puzzle3d-main")).toBe(true);
    expect(introductionTargetsWindow("other-window", "other-window", "puzzle3d-main")).toBe(false);
  });

  it("matches action-rail segments against the kind and its instances", () => {
    expect(introductionTargetsWindow("puzzle3d-main-top", "puzzle3d-main", null, "puzzle3dMain")).toBe(true);
    expect(introductionTargetsWindow("puzzle3d-main", "puzzle3d-main", null, "puzzle3dMain")).toBe(true);
    expect(introductionTargetsWindow("other-window", "other-window", null, "puzzle3dMain")).toBe(false);
  });
});

describe("windowMeasureTreeContainsId", () => {
  it("finds nested measure ids used as introduction targets", () => {
    const measures: WindowMeasure[] = [
      { kind: "select", id: "puzzle3d-play-vortex-show", value: "selected", items: [], onChange: { controllerId: "puzzle", action: "setVortexShow" } },
      {
        kind: "group" as const,
        id: "group",
        label: "Group",
        children: [{ kind: "toggle", id: "nested-toggle", pressed: false, iconId: "eye", onChange: { controllerId: "puzzle", action: "noOperation" } }],
      },
    ];
    expect(windowMeasureTreeContainsId(measures, "puzzle3d-play-vortex-show")).toBe(true);
    expect(windowMeasureTreeContainsId(measures, "nested-toggle")).toBe(true);
    expect(windowMeasureTreeContainsId(measures, "missing")).toBe(false);
  });
});

describe("renderWindowMeasuresTree", () => {
  it("puts toggle icons before labels and uses checkboxes instead of icon toggles", () => {
    const measures: WindowMeasure[] = [
      {
        kind: "toggle",
        id: "grid-visible",
        iconId: "layout-grid",
        label: "Grid",
        pressed: true,
        onChange: { controllerId: "x", action: "setGridVisible" },
      },
    ];
    const markup = renderToStaticMarkup(renderWindowMeasuresTree(measures, () => undefined) as ReactElement);
    const iconIdx = markup.indexOf('data-slot="tree-icon"');
    const labelIdx = markup.indexOf('data-slot="tree-label"');
    const checkboxIdx = markup.indexOf('data-slot="tree-action-checkbox"');
    expect(iconIdx).toBeGreaterThan(-1);
    expect(labelIdx).toBeGreaterThan(iconIdx);
    expect(checkboxIdx).toBeGreaterThan(labelIdx);
    expect(markup).toContain('type="checkbox"');
    expect(markup).toContain('id="grid-visible"');
    expect(markup).toContain('data-icon="layout-grid"');
    expect(markup).toContain("Grid");
    expect(markup).toContain("checked");
    expect(markup).not.toContain('data-state="on"');
  });
});

describe("per-window element ids", () => {
  const domIds = (markup: string): string[] => Array.from(markup.matchAll(/\sid="([^"]+)"/g)).map((match) => match[1]!);
  const duplicates = (ids: readonly string[]): string[] => ids.filter((id, index) => ids.indexOf(id) !== index);
  const threePoint = { mode: { kind: "threePoint" as const, fov: 50 }, orientation: { type: "free" as const } };
  const mountUnfoldedPane = (windowElementSegment: string): void => {
    render(createElement(WorldOrbitProjectionSwitchPane, { spec: threePoint, onSpecChange: () => undefined, windowElementSegment }) as ReactElement);
    const toggle = document.getElementById(childElementId(world3dProjectionPaneElementId(windowElementSegment), "pane", "fold"));
    expect(toggle).not.toBeNull();
    fireEvent.click(toggle!);
  };
  const mountedDomIds = (): string[] => Array.from(document.querySelectorAll("[id]")).map((element) => element.id);
  const authoredMeasures = (): WindowMeasure[] => [
    {
      kind: "group",
      id: "puzzle3d-measure-projection-parallel",
      label: "Parallel",
      defaultOpen: true,
      children: [
        { kind: "select", id: "puzzle3d-measure-projection-orthographic-view", value: "plan", items: [{ id: "plan", value: "plan", label: "Plan" }], onChange: { controllerId: "puzzle3d-play", action: "setProjection" } },
        { kind: "slider", id: "puzzle3d-measure-projection-oblique-angle", label: "Angle", value: 45, min: 5, max: 90, step: 1, onChange: { controllerId: "puzzle3d-play", action: "setProjectionParam" } },
      ],
    },
    { kind: "toggle", id: "puzzle3d-play-grid-visible", iconId: "layout-grid", label: "Visible", pressed: true, onChange: { controllerId: "puzzle3d-play", action: "setGridVisible" } },
  ];
  const measuresMarkup = (windowId: string): string => renderToStaticMarkup(windowMeasuresChrome(authoredMeasures(), undefined, windowId, () => undefined).measures as ReactElement);

  // 🪪️ Wave B41: `framework.worldOrbit.projection` was a HARDCODED id on a pane rendered once per world
  // surface, and a world surface is mounted once per open window instance — so the two-pane default layout
  // put the pane root and both of `Pane`'s derived fold-control ids in the document TWICE, and every row of
  // the projection switch tree with them. Two surfaces must share no DOM id at all.
  it("renders two world surfaces with no duplicate DOM id", () => {
    mountUnfoldedPane("puzzle3d-main-top");
    const top = mountedDomIds();
    mountUnfoldedPane("puzzle3d-main-perspective");
    const all = mountedDomIds();
    const perspective = all.filter((id) => !top.includes(id));
    // 🌲️ Unfolded, each pane carries its root, its fold toggle, its close control and every projection
    // switch row — the whole id surface the folded rail used to hide.
    expect(top.length).toBeGreaterThan(8);
    expect(perspective.length).toBe(top.length);
    expect(duplicates(all)).toEqual([]);
    expect(all).toContain(world3dProjectionPaneElementId("puzzle3d-main-top"));
    expect(all).toContain(world3dProjectionPaneElementId("puzzle3d-main-perspective"));
    expect(all.every((id) => isElementId(id))).toBe(true);
    cleanup();
  });

  it("styles the projection pane body like window options — transparent payload, no nested ribbon glass", () => {
    mountUnfoldedPane("puzzle3d-main-top");
    const paneId = world3dProjectionPaneElementId("puzzle3d-main-top");
    const body = document.getElementById(paneId)?.querySelector('[data-slot="pane-body"]') as HTMLElement | null;
    const switchRoot = document.querySelector("[data-world-projection-kind-switch]") as HTMLElement | null;
    expect(body?.className).toContain("p-tiny");
    expect(body?.hasAttribute("data-window-silhouette-content")).toBe(true);
    expect(switchRoot?.getAttribute("data-level")).toBeNull();
    expect(switchRoot?.className.includes("rounded-md")).toBe(false);
    cleanup();
  });

  // 🌲️ The pure half of the same invariant, so a regression is reported by the id builder and not only by
  // a mounted pane: every projection-switch row id is a child of the pane that owns it.
  it("qualifies every projection switch row id by its owning pane", () => {
    const templates = createWorldProjectionTemplates({ controllerId: "projection-switch" });
    const rows = (paneId: string): string[] => {
      const walk = (items: readonly { readonly id: string; readonly items?: readonly unknown[] }[]): string[] =>
        items.flatMap((item) => [item.id, ...walk((item.items ?? []) as readonly { readonly id: string; readonly items?: readonly unknown[] }[])]);
      return walk(worldProjectionSwitchTreeItems(paneId, templates, () => undefined) as readonly { readonly id: string; readonly items?: readonly unknown[] }[]);
    };
    const top = rows(world3dProjectionPaneElementId("puzzle3d-main-top"));
    const perspective = rows(world3dProjectionPaneElementId("puzzle3d-main-perspective"));
    expect(top.length).toBeGreaterThan(8);
    expect(duplicates([...top, ...perspective])).toEqual([]);
    expect(top.every((id) => id.startsWith(`${world3dProjectionPaneElementId("puzzle3d-main-top")}.`) && isElementId(id))).toBe(true);
  });

  // 🪪️ The same defect on the measures rail: a window kind's measure tree is authored ONCE for the kind
  // (puzzle3d passes the literal `"puzzle3d"` prefix to `world3d_projection_measures`) and rendered once
  // per instance, so the authored ids collided as soon as a second rail was unfolded.
  it("renders two windows' measures rails with no duplicate DOM id and keeps the authored id as the tail", () => {
    const top = domIds(measuresMarkup("puzzle3d-main-top"));
    const perspective = domIds(measuresMarkup("puzzle3d-main-perspective"));
    expect(top.length).toBeGreaterThan(1);
    expect(duplicates([...top, ...perspective])).toEqual([]);
    expect(top).toContain(windowMeasureDomId("puzzle3d-main-top", "puzzle3d-play-grid-visible"));
    expect(perspective).toContain(windowMeasureDomId("puzzle3d-main-perspective", "puzzle3d-play-grid-visible"));
    for (const authored of ["puzzle3d-play-grid-visible", "puzzle3d-measure-projection-orthographic-view"]) {
      expect(perspective.filter((id) => id.endsWith(`/${authored}`))).toHaveLength(1);
    }
  });

  // 🎓️ Qualification must not reach the AUTHORED tree: `windowMeasureTreeContainsId` (which decides whose
  // window an introduction anchor belongs to) and the program's own `activeUtilityId` routing both speak
  // the authored id, so the qualifier is a pure projection that leaves its input untouched.
  it("qualifies ids without mutating the authored measure tree or any other field", () => {
    const authored = authoredMeasures();
    const qualified = qualifyWindowMeasureIds(authored, "puzzle3d-main-top");
    expect(authored).toEqual(authoredMeasures());
    expect(windowMeasureTreeContainsId(authored, "puzzle3d-play-grid-visible")).toBe(true);
    const group = qualified[0] as Extract<WindowMeasure, { kind: "group" }>;
    expect(group.id).toBe("puzzle3d-main-top/puzzle3d-measure-projection-parallel");
    expect(group.label).toBe("Parallel");
    expect(group.children.map((child) => child.id)).toEqual(["puzzle3d-main-top/puzzle3d-measure-projection-orthographic-view", "puzzle3d-main-top/puzzle3d-measure-projection-oblique-angle"]);
    expect(group.children[0]!.onChange).toEqual(authoredMeasures()[0]!.kind === "group" ? (authoredMeasures()[0] as Extract<WindowMeasure, { kind: "group" }>).children[0]!.onChange : undefined);
  });
});

describe("resolveFrameworkLayoutSeed — multi-pane default layouts", () => {
  const emptyLabels = {
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

  it("does not infer focus when an app has no explicit layout", () => {
    const seed = resolveFrameworkLayoutSeed(undefined, [{ id: "main", label: "Main" }], emptyLabels, "native", "en");
    expect(seed.modeLayout).toEqual({ kind: "stack", children: [{ kind: "window", id: "main" }] });
    expect(seed).not.toHaveProperty("activeWindowId");
  });

  it("hydrates Top (1/3) + Perspective (2/3) instances and projection templates", () => {
    const topTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } });
    const perspectiveTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } });
    const seed = resolveFrameworkLayoutSeed(
      {
        root: {
          kind: "row",
          children: [
            {
              kind: "stack",
              size: 100 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Top", instanceId: "puzzle3d-main-top", templateId: topTemplate }],
            },
            {
              kind: "stack",
              size: 200 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Perspective", instanceId: "puzzle3d-main-perspective", templateId: perspectiveTemplate }],
            },
          ],
        },
      },
      [{ id: "puzzle3d-main", label: "Puzzle 3D" }],
      emptyLabels,
      "native",
      "en",
    );
    expect(seed.modeLayout).toEqual({
      kind: "row",
      children: [
        { kind: "stack", size: 100 / 3, children: [{ kind: "window", id: "puzzle3d-main-top", title: "Top" }] },
        { kind: "stack", size: 200 / 3, children: [{ kind: "window", id: "puzzle3d-main-perspective", title: "Perspective" }] },
      ],
    });
    expect(seed.extraInstances).toEqual([
      { id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "Top" },
      { id: "puzzle3d-main-perspective", windowKindId: "puzzle3d-main", title: "Perspective" },
    ]);
    expect(seed).not.toHaveProperty("activeWindowId");
    expect(seed.pendingProjections).toEqual([
      { windowId: "puzzle3d-main-top", templateId: topTemplate },
      { windowId: "puzzle3d-main-perspective", templateId: perspectiveTemplate },
    ]);
  });

  it("treats instance-id panes as extras so the host fetches bodies keyed by instance id, not only by kind", () => {
    const topTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } });
    const perspectiveTemplate = encodeWorldProjectionTemplateId({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } });
    const seed = resolveFrameworkLayoutSeed(
      {
        root: {
          kind: "row",
          children: [
            {
              kind: "stack",
              size: 100 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Top", instanceId: "puzzle3d-main-top", templateId: topTemplate }],
            },
            {
              kind: "stack",
              size: 200 / 3,
              children: [{ kind: "window", windowKindId: "puzzle3d-main", title: "Perspective", instanceId: "puzzle3d-main-perspective", templateId: perspectiveTemplate }],
            },
          ],
        },
      },
      [{ id: "puzzle3d-main", label: "Puzzle 3D" }],
      emptyLabels,
      "native",
      "en",
    );
    // 🪟️ A refresh that only knows the bare kind id would leave Top/Perspective as "Fehlendes Fenster".
    // Live extras must be in the fetch list: base kind + each default-layout instance.
    const windowInstances = [{ id: "puzzle3d-main", bodyKey: "puzzle3d.play.composite" }, ...seed.extraInstances.map((entry) => ({ id: entry.id, bodyKey: "puzzle3d.play.composite" }))];
    const request = buildUiRefreshRequest({ kind: "full" }, windowInstances, [], {}, new Map());
    expect(request?.windows?.map((window) => window.key)).toEqual(["puzzle3d-main", "puzzle3d-main-top", "puzzle3d-main-perspective"]);
  });

  it("re-derives window titles from localized windowKind labels on locale/terminology switch", () => {
    const windowKinds = [
      {
        id: "main",
        label: {
          native: { en: "Main Window", de: "Hauptfenster" },
          reuse: { en: "Main Component", de: "Hauptkomponente" },
        },
      },
    ];
    const layout = {
      kind: "stack" as const,
      children: [{ kind: "window" as const, id: "main", title: uiDataLabel("Main Window") }],
    };

    const retitledDeNative = retitleWindowLayoutNode(layout, windowKinds, [], "native", "de", {});
    expect(retitledDeNative).toEqual({
      kind: "stack",
      children: [{ kind: "window", id: "main", title: "Hauptfenster" }],
    });

    const retitledDeReuse = retitleWindowLayoutNode(layout, windowKinds, [], "reuse", "de", {});
    expect(retitledDeReuse).toEqual({
      kind: "stack",
      children: [{ kind: "window", id: "main", title: "Hauptkomponente" }],
    });
  });

  it("preserves authored titles for extra window instances on locale changes", () => {
    const windowKinds = [
      {
        id: "puzzle3d-main",
        label: {
          native: { en: "3D Editor", de: "3D-Editor" },
          reuse: { en: "3D Component", de: "3D-Komponente" },
        },
      },
    ];
    const extraInstances = [{ id: "puzzle3d-main-top", windowKindId: "puzzle3d-main", title: "Top" }];
    const layout = {
      kind: "stack" as const,
      children: [{ kind: "window" as const, id: "puzzle3d-main-top", title: uiDataLabel("Top") }],
    };

    const retitled = retitleWindowLayoutNode(layout, windowKinds, extraInstances, "reuse", "de", {});
    expect(retitled).toEqual({
      kind: "stack",
      children: [{ kind: "window", id: "puzzle3d-main-top", title: "Top" }],
    });
  });
});

describe("classifyWindowLayoutChange", () => {
  const twoStackRow = (leftSize: number, rightSize: number) => ({
    kind: "row" as const,
    size: 100,
    children: [
      { kind: "stack" as const, size: leftSize, children: [{ kind: "window" as const, id: "a" }] },
      { kind: "stack" as const, size: rightSize, children: [{ kind: "window" as const, id: "b" }] },
    ],
  });

  it("returns null when the layout is identical (deep-equal, not just same reference)", () => {
    expect(classifyWindowLayoutChange(twoStackRow(50, 50), twoStackRow(50, 50))).toBeNull();
    const same = twoStackRow(50, 50);
    expect(classifyWindowLayoutChange(same, same)).toBeNull();
  });

  it("returns null for a pure active-window-flag change (skeleton and sizes both unchanged)", () => {
    const previous = {
      kind: "stack" as const,
      size: 100,
      activeId: "a",
      children: [
        { kind: "window" as const, id: "a" },
        { kind: "window" as const, id: "b" },
      ],
    };
    const next = { ...previous, activeId: "b" };
    expect(classifyWindowLayoutChange(previous, next)).toBeNull();
  });

  it("returns 'resize' when only pane sizes differ", () => {
    expect(classifyWindowLayoutChange(twoStackRow(50, 50), twoStackRow(30, 70))).toBe("resize");
  });

  it("returns 'rearrange' when window ids/nesting structure differ (drag-to-new-position, split, close)", () => {
    const previous = twoStackRow(50, 50);
    const swapped = { ...previous, children: [previous.children[1]!, previous.children[0]!] };
    expect(classifyWindowLayoutChange(previous, swapped)).toBe("rearrange");
    const closed = { kind: "stack" as const, children: [{ kind: "window" as const, id: "a" }] };
    expect(classifyWindowLayoutChange(previous, closed)).toBe("rearrange");
    expect(classifyWindowLayoutChange(null, previous)).toBe("rearrange");
    expect(classifyWindowLayoutChange(previous, null)).toBe("rearrange");
  });

  it("returns null when both are null", () => {
    expect(classifyWindowLayoutChange(null, null)).toBeNull();
  });
});

describe("noteShellCommand", () => {
  /** ⏪️ A chrome note is an UNDO TARGET only when the caller declares a real inverse. `detail` says
   * where the chrome WENT, never where it came from, so the old unconditional
   * `inverseCommandId: commandId` / `inverseArgs: detail` pair was an identity, not an inverse: the
   * guest pushed every window activation onto the chrome undo stack and the first `undo` after a
   * click popped it and asked the host to replay `shell.windowActivate` into the app, where the
   * window-kind gate refused it `undeclared-action` (ticket 26/09/18 §3.2). */
  it("buildNoteShellCommandAction carries no inverse unless the caller declares one", () => {
    expect(buildNoteShellCommandAction("puzzle3d-play", "shell.windowClose", { en: "Close Window", de: "Fenster schließen" }, { windowId: "w1" })).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "shell.windowClose", label: { en: "Close Window", de: "Fenster schließen" }, detail: { windowId: "w1" } },
    });
    expect(buildNoteShellCommandAction("puzzle3d-play", "os.resetDock", { en: "Reset Panels", de: "Panels zurücksetzen" })).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "os.resetDock", label: { en: "Reset Panels", de: "Panels zurücksetzen" } },
    });
  });

  it("buildNoteShellCommandAction carries a declared inverse, and its arguments only when the inverse has some", () => {
    expect(buildNoteShellCommandAction("puzzle3d-play", "os.setThemeId", { en: "Set Theme", de: "Design festlegen" }, { themeId: "dark" }, { commandId: "os.setThemeId", args: { themeId: "light" } })).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "os.setThemeId", label: { en: "Set Theme", de: "Design festlegen" }, detail: { themeId: "dark" }, inverseCommandId: "os.setThemeId", inverseArgs: { themeId: "light" } },
    });
    expect(buildNoteShellCommandAction("puzzle3d-play", "os.resetDock", { en: "Reset Panels", de: "Panels zurücksetzen" }, undefined, { commandId: "os.resetDock" })).toEqual({
      controllerId: "puzzle3d-play",
      action: "noteShellCommand",
      args: { commandId: "os.resetDock", label: { en: "Reset Panels", de: "Panels zurücksetzen" }, inverseCommandId: "os.resetDock" },
    });
  });

  /** 🐚️ Every chrome id the React shell notes is shell-owned, so none of them may ever reach the
   * guest through `Effect::ReplayShellCommand` — a plugin action id (the `View`-kind rows) must. */
  it("isShellOwnedCommandId separates chrome the shell replays itself from plugin actions the guest replays", () => {
    for (const commandId of [
      "shell.windowActivate",
      "shell.windowResize",
      "shell.windowMove",
      "shell.windowClose",
      "shell.windowSplit",
      "shell.windowOpenInNewWindow",
      "shell.panelToggle",
      "shell.panelTab",
      "shell.dockMove",
      "os.setThemeId",
      "os.resetDock",
      "os.resizeWindow",
    ]) {
      expect(isShellOwnedCommandId(commandId)).toBe(true);
    }
    for (const actionId of ["setActiveExample", "patchNodes", "change-seed", "addNode"]) {
      expect(isShellOwnedCommandId(actionId)).toBe(false);
    }
  });

  /** 🎛️ …and every shell-owned id the shell notes through `dispatchOsCommand` really is routable
   * there, so the replay branch's "no shell route" warning means a genuinely missing route. */
  it("dispatchOsCommand reports whether it routed the command", () => {
    const noop = (): void => {};
    const stores = { reset: noop } as unknown as Parameters<typeof dispatchOsCommand>[4];
    expect(dispatchOsCommand("os.resetDock", undefined, noop, noop, stores, stores as unknown as Parameters<typeof dispatchOsCommand>[5])).toBe(true);
    expect(dispatchOsCommand("os.setThemeId", { themeId: "light" }, noop, noop, stores, stores as unknown as Parameters<typeof dispatchOsCommand>[5])).toBe(true);
    expect(dispatchOsCommand("shell.windowActivate", { windowId: "w1" }, noop, noop, stores, stores as unknown as Parameters<typeof dispatchOsCommand>[5])).toBe(false);
  });

  it("is excluded from tutorial recording, alongside world-navigation/introduction/tutorial-control action ids", () => {
    expect(TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS.has("noteShellCommand")).toBe(true);
  });
});

describe("TutorialRecorder LocalizedLabel synthesis", () => {
  it("synthesizeLocalizedLabel broadcasts a string across all 4 cells (native/reuse x en/de)", () => {
    const label = synthesizeLocalizedLabel("Test Chapter");
    expect(label).toEqual({
      native: { en: "Test Chapter", de: "Test Chapter" },
      reuse: { en: "Test Chapter", de: "Test Chapter" },
    });
    expect(synthesizeLocalizedLabel(label)).toBe(label);
  });

  it("resolves exact language-neutral label cells without a default locale", async () => {
    const validate = rendererExport("ShellLabelResolutionLanguages");
    expect(Object.values(labelResolutionFixture.matrix).every((labels) => validate(labels)), JSON.stringify(validate.errors)).toBe(true);
    const matrix = labelResolutionFixture.matrix;
    const oracle = createTranslationOracle();
    await oracle.init({
      fallbackLng: false,
      resources: {
        en: { native: { label: matrix.native.en }, reuse: { label: matrix.reuse.en } },
        de: { native: { label: matrix.native.de }, reuse: { label: matrix.reuse.de } },
      },
    });
    for (const row of labelResolutionFixture.cases) {
      expect(resolveManifestLabel(row.data ?? matrix, row.terminology, row.locale), row.id).toBe(row.expected);
      const expected = row.data ?? oracle.getResource(row.locale, row.terminology, "label") ?? "";
      expect(expected, row.id).toBe(row.expected);
    }
    for (const malformed of [null, 7, [], {}, { native: { en: "English" }, reuse: { en: "English" } }, { native: { en: 7, de: false }, reuse: { en: "Part", de: "Bauteil" } }]) {
      expect(resolveManifestLabel(malformed, "native", "de")).toBe("");
    }
    expect(validate({ en: "Only English" })).toBe(false);
  });

  it("TutorialRecorder synthesizes LocalizedLabel for addChapter and build titles", () => {
    const recorder = new TutorialRecorder({ activeUtilityByWindowId: {}, activePanelTabByGroup: {}, interactionSelection: {}, expandedTreeIds: [], commandPanelOpen: false }, null);
    recorder.addChapter("Introduction");
    recorder.addChapter();
    const def = recorder.build("rec-1", "Recorded Tutorial");

    expect(def.title).toEqual({
      native: { en: "Recorded Tutorial", de: "Recorded Tutorial" },
      reuse: { en: "Recorded Tutorial", de: "Recorded Tutorial" },
    });
    expect(def.chapters[0].title).toEqual({
      native: { en: "Introduction", de: "Introduction" },
      reuse: { en: "Introduction", de: "Introduction" },
    });
    expect(def.chapters[1].title).toEqual({
      native: { en: "Chapter 2", de: "Chapter 2" },
      reuse: { en: "Chapter 2", de: "Chapter 2" },
    });

    expect(resolveManifestLabel(def.title, "native", "en")).toBe("Recorded Tutorial");
    expect(resolveManifestLabel(def.chapters[0].title, "reuse", "de")).toBe("Introduction");
  });

  it("FrameworkOsShell portal layer is unconstrained by z-tutorial so portaled elements sit above elevated windows", () => {
    if (!window.matchMedia) {
      window.matchMedia = (() => ({
        matches: false,
        media: "",
        onchange: null,
        addListener: () => {},
        removeListener: () => {},
        addEventListener: () => {},
        removeEventListener: () => {},
        dispatchEvent: () => false,
      })) as unknown as typeof window.matchMedia;
    }
    const backboneWorkerFactory = () => Object.assign(new EventTarget(), { onmessage: null, postMessage: vi.fn(), terminate: vi.fn() }) as unknown as Worker;
    const { container } = render(createElement(FrameworkOsShell, { plugins: [], appId: "test", backboneWorkerFactory }));
    const portalLayer = container.querySelector("[data-semio-portal-layer]");
    expect(portalLayer).toBeTruthy();
    expect(portalLayer?.className).not.toContain("z-tutorial");
  });
});



//#region 🥽️SceneMeshKindReferences
import { meshDataFromKind } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";
import sceneMeshKindFixture from "../../../../../../../🔨️modules/🏗️mesh-engine/🧫️fixtures/🥽️scene-mesh-kinds/🔣️.json";

/** 🥽️ The local-space bounding box a resolved kind occupies — the ONE property the fixture pins across
 * the two generators (Rust `mesh_from_kind`, this host's `meshDataFromKind`). */
function meshKindBounds(kind: string): { min: number[]; max: number[] } {
  const positions = meshDataFromKind(kind).positions;
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  for (let index = 0; index < positions.length; index += 3) {
    for (let axis = 0; axis < 3; axis += 1) {
      min[axis] = Math.min(min[axis]!, positions[index + axis]!);
      max[axis] = Math.max(max[axis]!, positions[index + axis]!);
    }
  }
  return { min, max };
}

describe("world-3d scene mesh kind references", () => {
  it("resolves every built-in kind to the language-neutral fixture's local bounding box", () => {
    expect(sceneMeshKindFixture.kinds.length).toBeGreaterThan(0);
    for (const entry of sceneMeshKindFixture.kinds) {
      const bounds = meshKindBounds(entry.kind);
      for (let axis = 0; axis < 3; axis += 1) {
        expect(Math.abs(bounds.min[axis]! - entry.min[axis]!)).toBeLessThanOrEqual(sceneMeshKindFixture.tolerance);
        expect(Math.abs(bounds.max[axis]! - entry.max[axis]!)).toBeLessThanOrEqual(sceneMeshKindFixture.tolerance);
      }
      expect(meshDataFromKind(entry.kind).indices.length).toBeGreaterThan(0);
      expect(meshDataFromKind(entry.kind).normals.length).toBe(meshDataFromKind(entry.kind).positions.length);
    }
  });

  it("falls back to the fixture's fallback kind for a kind it never authored", () => {
    expect(meshKindBounds("totally-unknown-kind")).toEqual(meshKindBounds(sceneMeshKindFixture.fallbackKind));
  });

  it("memoizes each kind so a scene refresh never re-tessellates", () => {
    expect(meshDataFromKind("vortex-marker")).toBe(meshDataFromKind("vortex-marker"));
  });
});
//#endregion 🥽️SceneMeshKindReferences

//#region 🥽️HiddenTabSurfaceSizing
describe("node-graph surface sizing", () => {
  it("adopts its container's size on mount without any animation frame", () => {
    const container = new DOMRect(0, 0, 966, 836);
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(container);
    // 🙈️ A hidden/background tab never runs a frame callback — the regression this guards is a canvas
    // that only ever reached its real size from inside one (`📓️runtime-verification-2026-09-09.md` #3).
    const frame = vi.spyOn(globalThis, "requestAnimationFrame").mockImplementation(() => 0);
    // The wasm surface never settles here either, so nothing but the container observer can size these.
    const session = vi.spyOn(flowSessionLoader, "createFlowSession").mockReturnValue(new Promise(() => {}));
    vi.stubGlobal("devicePixelRatio", 2);
    const view = render(
      createElement(FlowGraphCanvasHost, {
        scene: { nodes: [], edges: [], viewport: { x: 0, y: 0, zoom: 1 }, hostSnapshotJson: '{"schema":"flow.host_snapshot","widgets":[]}' },
        controllerId: "procedural",
        surfaceId: "procedural.main",
        editable: true,
        keyboardPort: { current: null },
        onAction: noopAction,
      }),
    );
    try {
      const canvases = [...view.container.querySelectorAll("canvas")];
      expect(canvases.length).toBe(2);
      for (const canvas of canvases) {
        expect([canvas.width, canvas.height]).toEqual([1932, 1672]);
        expect([canvas.style.width, canvas.style.height]).toEqual(["966px", "836px"]);
      }
      expect(frame.mock.results.every((result) => result.value === 0)).toBe(true);
    } finally {
      view.unmount();
      vi.unstubAllGlobals();
      session.mockRestore();
      frame.mockRestore();
      bounds.mockRestore();
    }
  });

  it("keeps the exact device-pixel store the shared helper computes", () => {
    const canvas = document.createElement("canvas");
    expect([canvas.width, canvas.height]).toEqual([300, 150]);
    expect(resizeCanvasBackingStore(canvas, 966, 836, 1.5)).toBe(true);
    expect([canvas.width, canvas.height]).toEqual([1449, 1254]);
    expect(resizeCanvasBackingStore(canvas, 966, 836, 1.5)).toBe(false);
    expect(resizeCanvasBackingStore(canvas, 0, 0, 1)).toBe(true);
    expect([canvas.width, canvas.height]).toEqual([1, 1]);
    expect(resizeCanvasBackingStore(null, 966, 836, 1)).toBe(false);
  });
});
//#endregion 🥽️HiddenTabSurfaceSizing

//#region 🥽️HiddenTabSurfaceAttach
/** 🍄️ The served `hexagonal-mushroom-column` shape the boot report measured: seven nodes, six edges. */
function hexagonalMushroomColumnScene(): NodeGraphScene {
  const port = (id: string) => [{ id, label: id }];
  const nodes = ["sides", "radius", "height", "profile", "extrusion-axis", "extrude", "column-preview"].map((id, index) => ({
    id,
    label: id,
    x: index * 180,
    y: (index % 2) * 120,
    width: 140,
    height: 64,
    inputs: port("in"),
    outputs: port("out"),
  }));
  const edges = [
    ["sides", "profile"],
    ["radius", "profile"],
    ["height", "extrusion-axis"],
    ["profile", "extrude"],
    ["extrusion-axis", "extrude"],
    ["extrude", "column-preview"],
  ].map(([source, target]) => ({ id: `${source}->${target}`, sourceNodeId: source!, sourcePortId: "out", targetNodeId: target!, targetPortId: "in" }));
  return {
    nodes,
    edges,
    viewport: { x: 0, y: 0, zoom: 1.78 },
    editable: true,
    hostSnapshotJson: JSON.stringify({ schema: "flow.host_snapshot", widgets: nodes.map((node) => ({ id: node.id, x: node.x, y: node.y })) }),
  };
}

/** 🖌️ Recording stand-in for a 2D context — jsdom implements none, so this is the only way to witness
 * that a paint actually reached a canvas rather than merely being scheduled. */
function recordingCanvasContext(operations: string[]): CanvasRenderingContext2D {
  const record =
    (name: string) =>
    (...args: unknown[]) => {
      operations.push(`${name}(${args.join(",")})`);
    };
  return new Proxy({} as CanvasRenderingContext2D, {
    get: (_target, key) => (typeof key === "string" ? record(key) : undefined),
    set: () => true,
  });
}

describe("node-graph surface attachment in a hidden tab", () => {
  it("attaches, feeds the scene and draws without any animation frame", async () => {
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 966, 836));
    // 🙈️ The exact boot condition of `📓️runtime-verification-2026-09-09.md` #4: a hidden tab, so no frame
    // callback ever runs, on a host with no `navigator.gpu` at all (jsdom ships none).
    const hidden = vi.spyOn(document, "hidden", "get").mockReturnValue(true);
    const frame = vi.spyOn(globalThis, "requestAnimationFrame").mockImplementation(() => 0);
    const painted = new Map<HTMLCanvasElement, string[]>();
    const context = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockImplementation(function (this: HTMLCanvasElement) {
      const operations = painted.get(this) ?? [];
      painted.set(this, operations);
      return recordingCanvasContext(operations);
    } as unknown as HTMLCanvasElement["getContext"]);
    const bridge = new MockFlowBridge(new WebAssembly.Memory({ initial: 400 }));
    const runtime = await createFlowBrowserRuntime({ source: bridge.exports });
    const session = vi.spyOn(flowSessionLoader, "createFlowSession").mockImplementation(async () => runtime.openSession() as unknown as flowSessionLoader.FlowWasmSession);
    vi.stubGlobal("devicePixelRatio", 1);
    const view = render(
      createElement(FlowGraphCanvasHost, {
        scene: hexagonalMushroomColumnScene(),
        controllerId: "procedural",
        surfaceId: "procedural.main",
        editable: true,
        keyboardPort: { current: null },
        onAction: noopAction,
      }),
    );
    try {
      expect(globalThis.navigator.gpu).toBeUndefined();
      await waitFor(() => expect(bridge.operations).toContain(flowAbi.operations.renderFrame));
      expect(bridge.operations).toContain(flowAbi.operations.attachSurface);
      expect(bridge.operations).toContain(flowAbi.operations.surfaceStatus);
      expect(bridge.operations).toContain(flowAbi.operations.synchronizeSnapshotJson);
      expect(bridge.operations).toContain(flowAbi.operations.setCamera);
      expect(bridge.operations).toContain(flowAbi.operations.setSize);
      await waitFor(() => expect(bridge.operations).toContain(flowAbi.operations.labelOverlayPaintStateJson));
      const canvases = [...view.container.querySelectorAll("canvas")];
      expect(canvases.length).toBe(2);
      await waitFor(() => {
        for (const canvas of canvases) expect((painted.get(canvas) ?? []).some((operation) => operation.startsWith("clearRect("))).toBe(true);
      });
      // 📏️ A frame that carries no size of its own must not collapse the store the host measured.
      for (const canvas of canvases) expect([canvas.width, canvas.height]).toEqual([966, 836]);
      expect(painted.get(canvases[0]!)).toContain("clearRect(0,0,966,836)");
      // 🚫️ The placeholder this replaced drew one `fillRect(x-60,y-24,120,48)` per widget at RAW
      // world coordinates, in the 2D default black because it never set a fill style. A frame that
      // carries no draw list must now paint nothing at all rather than invent a picture.
      expect((painted.get(canvases[0]!) ?? []).filter((operation) => operation.startsWith("fillRect("))).toEqual([]);
      expect(frame).not.toHaveBeenCalled();
    } finally {
      view.unmount();
      vi.unstubAllGlobals();
      session.mockRestore();
      context.mockRestore();
      frame.mockRestore();
      hidden.mockRestore();
      bounds.mockRestore();
      await runtime.close().catch(() => {});
    }
  });

  it("replaces a canvas that can no longer give a 2D context and re-attaches its surface to the successor", async () => {
    // 🚨️ The live defect this pins: `wgpu` binds a `webgpu` context to the element before it finds
    // out there is no adapter, and a canvas admits exactly one context kind for its whole life — so
    // the frame correctly falls back to `present: "2d"` onto an element that can never give a 2D
    // context, and the graph stays blank forever with nothing in the console. A frame that reports
    // `unpresentable` must retire that element and let the presentation be decided again for a new
    // one. Measured on 6018 before this: canvas 0 `not-2d`, 0 ink pixels, every frame.
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 966, 836));
    const frame = vi.spyOn(globalThis, "requestAnimationFrame").mockImplementation(() => 0);
    const painted = new Map<HTMLCanvasElement, string[]>();
    const poisoned = new Set<HTMLCanvasElement>();
    const context = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockImplementation(function (this: HTMLCanvasElement) {
      if (poisoned.has(this)) return null;
      const operations = painted.get(this) ?? [];
      painted.set(this, operations);
      return recordingCanvasContext(operations);
    } as unknown as HTMLCanvasElement["getContext"]);
    const bridge = new MockFlowBridge(new WebAssembly.Memory({ initial: 400 }));
    // 🖥️ The surface must really be presenting on the GPU, because only a canvas that was handed to
    // the WebGPU presenter can be poisoned — a host that never had a device is a host with no 2D
    // canvas either, and replacing its element would remount forever.
    let boundToTheGpuPresenter = 0;
    const runtime = await createFlowBrowserRuntime({
      source: bridge.exports,
      bindings: {
        flowAttachSurfaceCanvas: async () => {
          boundToTheGpuPresenter += 1;
          return boundToTheGpuPresenter === 1;
        },
      },
    });
    vi.stubGlobal("navigator", { ...globalThis.navigator, gpu: { requestAdapter: async () => ({}) } });
    const session = vi.spyOn(flowSessionLoader, "createFlowSession").mockImplementation(async () => runtime.openSession() as unknown as flowSessionLoader.FlowWasmSession);
    vi.stubGlobal("devicePixelRatio", 1);
    const view = render(
      createElement(FlowGraphCanvasHost, {
        scene: hexagonalMushroomColumnScene(),
        controllerId: "procedural",
        surfaceId: "procedural.main",
        editable: true,
        keyboardPort: { current: null },
        onAction: noopAction,
      }),
    );
    try {
      const first = view.container.querySelectorAll("canvas")[0]!;
      poisoned.add(first);
      const attaches = () => bridge.operations.filter((operation) => operation === flowAbi.operations.attachSurface).length;
      await waitFor(() => expect(attaches()).toBeGreaterThanOrEqual(2));
      const successor = view.container.querySelectorAll("canvas")[0]!;
      expect(successor).not.toBe(first);
      expect(view.container.querySelectorAll("canvas").length).toBe(2);
      // ⏳️ The successor's backing store is sized by its own layout effect, which runs after the
      // attach this test waited on — reading it in the same tick catches the jsdom default 300x150.
      await waitFor(() => expect([successor.width, successor.height]).toEqual([966, 836]));
      await waitFor(() => expect((painted.get(successor) ?? []).some((operation) => operation.startsWith("clearRect("))).toBe(true));
      expect(painted.has(first)).toBe(false);
      // 🔚️ Exactly one successor: it re-attaches without a device, so it stays 2D-capable and the
      // host never asks for another. A replacement loop is the failure mode this bound rules out.
      await new Promise((settle) => setTimeout(settle, 50));
      expect(view.container.querySelectorAll("canvas")[0]).toBe(successor);
      expect(attaches()).toBe(2);
    } finally {
      view.unmount();
      vi.unstubAllGlobals();
      session.mockRestore();
      context.mockRestore();
      frame.mockRestore();
      bounds.mockRestore();
      await runtime.close().catch(() => {});
    }
  });

  /** 🪪️ The host half of the surface-retention law (`🧫️fixtures/🪪️surface-host-retention.json`): a
   * refresh is a SCENE change, never a lifecycle event. Every refresh of `window:procedural-main`
   * carries a different eval status map, and the host must answer each of them by patching the flow
   * session it already owns — one `createFlowSession`, one `attachSurface`, one wasm-side surface —
   * and never by opening a second one. Measured on the live page before the reconciliation-key fix:
   * two mounts, two sessions and `flow surface created surface=2`, costing 5.7 s of attach and a
   * graph that stayed blank for 39 s (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  it("retained surface host — patches the session it already owns across consecutive refreshes, one session and one attach", async () => {
    const retention = (await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪪️surface-host-retention.json")).default;
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 966, 836));
    const context = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockImplementation(() => recordingCanvasContext([]) as never);
    const bridge = new MockFlowBridge(new WebAssembly.Memory({ initial: 400 }));
    const runtime = await createFlowBrowserRuntime({ source: bridge.exports });
    const opened: unknown[] = [];
    const released: ReturnType<typeof vi.spyOn>[] = [];
    const session = vi.spyOn(flowSessionLoader, "createFlowSession").mockImplementation(async () => {
      const next = runtime.openSession();
      opened.push(next);
      released.push(vi.spyOn(next, "free"));
      return next as unknown as flowSessionLoader.FlowWasmSession;
    });
    vi.stubGlobal("devicePixelRatio", 1);
    const sceneFor = (statusJson: string) => ({ ...hexagonalMushroomColumnScene(), statusJson });
    const view = render(
      createElement(FlowGraphCanvasHost, {
        scene: sceneFor(retention.refreshes[0]!.statusJson),
        controllerId: "29",
        surfaceId: retention.surface,
        editable: true,
        keyboardPort: { current: null },
        onAction: noopAction,
      }),
    );
    try {
      await waitFor(() => expect(bridge.operations).toContain(flowAbi.operations.attachSurface));
      for (let index = 1; index < retention.refreshes.length; index += 1) {
        const refresh = retention.refreshes[index]!;
        // 🪪️ `controllerId` is the DFS-minted record id, and it MOVES with the body (29 → 33 on the
        // measured refresh). A host that re-attached on it would fail here and nowhere else.
        view.rerender(
          createElement(FlowGraphCanvasHost, {
            scene: sceneFor(refresh.statusJson),
            controllerId: String(29 + index * 2),
            surfaceId: retention.surface,
            editable: true,
            keyboardPort: { current: null },
            onAction: noopAction,
          }),
        );
        await waitFor(() => expect(bridge.operations).toContain(flowAbi.operations.synchronizeSnapshotJson));
      }
      const attaches = bridge.operations.filter((operation: number) => operation === flowAbi.operations.attachSurface).length;
      expect(attaches, `${retention.refreshes.length} refreshes must dispatch exactly one attach`).toBe(retention.expected.surfaceAttaches);
      expect(opened.length, "one flow session per window instance, never one per refresh").toBe(retention.expected.surfaceHostMounts);
      expect(view.container.querySelectorAll("canvas").length, "the two canvases the host owns, never a second pair").toBe(2);
      for (const release of released) expect(release).not.toHaveBeenCalled();
      view.unmount();
      await waitFor(() => {
        for (const release of released) expect(release).toHaveBeenCalledTimes(1);
      });
    } finally {
      view.unmount();
      vi.unstubAllGlobals();
      session.mockRestore();
      context.mockRestore();
      bounds.mockRestore();
      await runtime.close().catch(() => {});
    }
  });
});
//#endregion 🥽️HiddenTabSurfaceAttach

//#region 🥽️BuiltNodeStoreReload
describe("built-node store reloads", () => {
  const contractNode = (value: string): BuiltNode => buildContractNode({ key: "window.body", component: { type: "text", value, emphasize: null, dataAttributes: null } });

  it("loads a new key inline and defers every later reload to the flush", () => {
    const cache = createBuiltNodeStoreCacheV1();
    const first = contractNode("first");
    const store = cache.storeFor("window:procedural-main", first);
    const rootId = store.getState().root;
    expect(rootId).not.toBeNull();
    const textOf = () => (store.getNodeSnapshot(store.getState().root ?? 0)?.component as { readonly value?: string } | undefined)?.value;
    expect(textOf()).toBe("first");
    let notifications = 0;
    const unsubscribe = store.subscribeNode(rootId ?? 0)(() => {
      notifications += 1;
    });
    try {
      expect(cache.storeFor("window:procedural-main", first)).toBe(store);
      expect(cache.pendingReloadKeys()).toEqual([]);
      expect(notifications).toBe(0);
      const second = contractNode("second");
      expect(cache.storeFor("window:procedural-main", second)).toBe(store);
      expect(cache.pendingReloadKeys()).toEqual(["window:procedural-main"]);
      expect(notifications).toBe(0);
      expect(textOf()).toBe("first");
      const revisionBefore = store.getRevisionSnapshot();
      cache.flushPendingReloads();
      expect(cache.pendingReloadKeys()).toEqual([]);
      expect(textOf()).toBe("second");
      expect(notifications).toBeGreaterThan(0);
      expect(store.getRevisionSnapshot()).toBeGreaterThan(revisionBefore);
      const settled = notifications;
      expect(cache.storeFor("window:procedural-main", second)).toBe(store);
      cache.flushPendingReloads();
      expect(notifications).toBe(settled);
    } finally {
      unsubscribe();
    }
  });

  it("publishes a body no render owns straight into its store: a new key loads, a changed body reloads and notifies at once, an unchanged one is a no-op", () => {
    const cache = createBuiltNodeStoreCacheV1();
    expect(cache.storeOf("spawned:puzzle-2::puzzle3d-main")).toBeNull();
    const first = contractNode("first");
    publishBuiltNodesV1(cache, [["spawned:puzzle-2::puzzle3d-main", first]]);
    const store = cache.storeOf("spawned:puzzle-2::puzzle3d-main");
    expect(store).not.toBeNull();
    const textOf = () => (store!.getNodeSnapshot(store!.getState().root ?? 0)?.component as { readonly value?: string } | undefined)?.value;
    expect(textOf()).toBe("first");
    let notifications = 0;
    const unsubscribe = store!.subscribeNode(store!.getState().root ?? 0)(() => {
      notifications += 1;
    });
    try {
      publishBuiltNodesV1(cache, [["spawned:puzzle-2::puzzle3d-main", first]]);
      expect(notifications).toBe(0);
      const revisionBefore = store!.getRevisionSnapshot();
      publishBuiltNodesV1(cache, [["spawned:puzzle-2::puzzle3d-main", contractNode("hovered")]]);
      expect(cache.storeOf("spawned:puzzle-2::puzzle3d-main")).toBe(store);
      expect(textOf()).toBe("hovered");
      expect(notifications).toBeGreaterThan(0);
      expect(store!.getRevisionSnapshot()).toBeGreaterThan(revisionBefore);
      expect(cache.pendingReloadKeys()).toEqual([]);
    } finally {
      unsubscribe();
    }
  });

  it("forceReload queues loadSnapshot even when the node identity did not change", () => {
    const cache = createBuiltNodeStoreCacheV1();
    const first = contractNode("first");
    const store = cache.storeFor("window:puzzle3d-main-perspective", first);
    const textOf = () => (store.getNodeSnapshot(store.getState().root ?? 0)?.component as { readonly value?: string } | undefined)?.value;
    expect(textOf()).toBe("first");
    const patched = contractNode("preview");
    cache.forceReload("window:puzzle3d-main-perspective", patched);
    expect(cache.pendingReloadKeys()).toEqual(["window:puzzle3d-main-perspective"]);
    expect(textOf()).toBe("first");
    cache.flushPendingReloads();
    expect(textOf()).toBe("preview");
    cache.forceReload("window:puzzle3d-main-perspective", patched);
    expect(cache.pendingReloadKeys()).toEqual(["window:puzzle3d-main-perspective"]);
    cache.flushPendingReloads();
    expect(textOf()).toBe("preview");
  });

  it("never updates a subscribed UiNodeView while another component renders", () => {
    const messages: string[] = [];
    const consoleError = vi.spyOn(console, "error").mockImplementation((...args: unknown[]) => {
      messages.push(args.map(String).join(" "));
    });
    const renderPhaseUpdates = () => messages.filter((message) => message.includes("while rendering a different component"));
    const cache = createBuiltNodeStoreCacheV1();
    const deferred = ({ node }: { readonly node: BuiltNode }) => {
      const store = cache.storeFor("window:procedural-main", node);
      useLayoutEffect(() => cache.flushPendingReloads());
      return createElement(InterpretedUiNode, { store, onAction: noopAction, onIntent: noopAction, requestContextMenu: undefined });
    };
    const eager = ({ node }: { readonly node: BuiltNode }) => {
      const store = cache.storeFor("window:procedural-eager", node);
      store.loadSnapshot(builtNodeToSnapshot("window:procedural-eager", node));
      return createElement(InterpretedUiNode, { store, onAction: noopAction, onIntent: noopAction, requestContextMenu: undefined });
    };
    const view = render(createElement(deferred, { node: contractNode("first") }));
    try {
      expect(view.container.textContent).toContain("first");
      view.rerender(createElement(deferred, { node: contractNode("second") }));
      expect(view.container.textContent).toContain("second");
      expect(renderPhaseUpdates()).toEqual([]);
      // 🧪️ Control: the pre-fix shape (a `loadSnapshot` in the render body of a store that already has
      // mounted subscribers) is exactly what React reports as a cross-component render-phase update.
      const control = render(createElement(eager, { node: contractNode("first") }));
      control.rerender(createElement(eager, { node: contractNode("second") }));
      expect(renderPhaseUpdates().length).toBeGreaterThan(0);
      control.unmount();
    } finally {
      view.unmount();
      consoleError.mockRestore();
    }
  });

  it("panel bodies publish into one store per panel key: a new body reloads it in place, the tab keeps its tree config, and the store answers its node", () => {
    const cache = createBuiltNodeStoreCacheV1();
    const historyTab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: "History", group: "settings" as const, bodyKey: "framework.body.history", children: [] };
    const overlay = { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {}, actionLabels: {}, utilityLabels: {}, exampleLabels: {}, actionArgLabels: {}, dialogLabels: {}, introductionLabels: {}, groupLabels: {} };
    const configs: Parameters<typeof panelTabDefinitionToNode>[9] = new Map();
    const store = publishPanelBodiesV1(cache, [["framework.panel.history", contractNode("first")]])[0]![1];
    const onAction = (): void => undefined;
    const configOf = () => {
      const tab = panelTabDefinitionToNode(historyTab, "settings", { "framework.panel.history": store }, onAction, 1, overlay, undefined, undefined, null, configs);
      const tree = tab.kind === "leaf" ? tab.trees[0]!.tree : null;
      return tree !== null && "resolveTree" in tree ? tree.resolveTree() : tree;
    };
    const before = configOf();
    const second = contractNode("second");
    expect(publishPanelBodiesV1(cache, [["framework.panel.history", second]])[0]![1], "one store per panel key").toBe(store);
    expect(cache.nodeOf(store)).toBe(second);
    expect(configOf(), "the tab's tree config survives the body change, so the mounted tree is never remounted").toBe(before);
    expect(publishPanelBodiesV1(cache, [["framework.panel.history", undefined]])[0]![1]).toBe(store);
    expect(cache.nodeOf(store), "a panel the refresh did not re-serialize keeps its body").toBe(second);
    const pending = publishPanelBodiesV1(cache, [["framework.panel.inspection", undefined]])[0]![1];
    expect(cache.nodeOf(pending)?.key, "a panel with no body yet starts on the pending body").toBe(pendingPanelUiNode().key);
  });
});
//#endregion 🥽️BuiltNodeStoreReload

test("world3d rectangle marquee draws a rectangle and pick draws nothing", () => {
  expect(world3dMarqueeOverlayShape("rectangle")).toBe("rect");
  expect(world3dMarqueeOverlayShape("lasso")).toBe("polygon");
  expect(world3dMarqueeOverlayShape("pick")).toBe("rect");
});

test("InterpretedUiNode hands every surface host the shell's plugin context-menu resolver", () => {
  // 🖱️ No `<InterpretedUiNode>` call site passes `requestContextMenu` as a prop — ShellHost publishes it
  // through `PluginSurfaceActionsContext` instead — so until wave B11 every `ComponentSceneHost` saw
  // `undefined` and `openSurfaceContextMenu` was unreachable in the React renderer: a right-click on a
  // world/board/canvas surface could only ever produce ShellHost's window-level fallback menu. This is a
  // source law because the wiring is one `??` that no render test would notice going missing again.
  const body = String((InterpretedUiNode as unknown as { readonly type: (...args: never[]) => unknown }).type);
  expect(body).toContain("usePluginSurfaceActions");
  expect(body).toMatch(/requestContextMenu:\s*requestContextMenu\s*\?\?/);
});

test("openSurfaceContextMenu keeps an empty plugin answer off the shell fallback", async () => {
  const shell = [{ id: "setActiveExample", label: "Set Active Example" }];
  const guest = async () => [];
  const mapped = await openSurfaceContextMenu(
    guest,
    { menu: { id: "world3d", args: null }, point: { x: 0, y: 0 } } as never,
    (specs) => [...specs] as never,
    () => shell as never,
  );
  expect(mapped.items).toEqual([]);
});

//#region 📄️PublicInvocationPaging
import publicInvocationSchema from "../../../../../../../🔨️modules/🛂️manifest/🎛️public-invocation/🧬️schema/🔣️.json";
import { PUBLIC_INVOCATION_BODY_BYTES, PUBLIC_INVOCATION_DEPTH, PUBLIC_INVOCATION_STRING_BYTES, publicInvocationCharCost, publicInvocationStringPages } from "@semio-tech/framework";

describe("public invocation paging", () => {
  it("mirrors the language-neutral envelope schema both Rust and the shell read", () => {
    expect(PUBLIC_INVOCATION_BODY_BYTES).toBe(publicInvocationSchema.properties.maxBodyBytes.const);
    expect(PUBLIC_INVOCATION_STRING_BYTES).toBe(publicInvocationSchema.properties.maxStringBytes.const);
    expect(PUBLIC_INVOCATION_DEPTH).toBe(publicInvocationSchema.properties.maxDepth.const);
  });

  /** 📐️ The Node oracle for the cost function: what `JSON.stringify` actually writes for this
   * character, minus the leading backslash the guest's counter skips. */
  const oracleCost = (character: string): number => {
    const encoded = JSON.stringify(character).slice(1, -1);
    return encoded.length - (encoded.match(/\\/gu)?.length ?? 0);
  };

  it("never undercharges a character against the JSON.stringify oracle", () => {
    for (const character of ['"', "\\", "\n", "\r", "\t", "\u0001", "a", "\u00e4", "\u26f0", "\u{1f300}"]) {
      expect(publicInvocationCharCost(character)).toBeGreaterThanOrEqual(oracleCost(character));
    }
    expect(publicInvocationCharCost('"')).toBe(1);
    expect(publicInvocationCharCost("\u0001")).toBe(5);
    expect(publicInvocationCharCost("\u{1f300}")).toBe(10);
  });

  it("fills every page to the bound, splits only on code points and loses nothing", () => {
    const payload = '{"id":"brep.extrude","name":"Extrudieren \u26f0\ufe0f"},'.repeat(4_096);
    const pages = publicInvocationStringPages(payload);
    expect(pages.length).toBeGreaterThan(1);
    for (const page of pages) {
      let cost = 0;
      for (const character of page) cost += publicInvocationCharCost(character);
      expect(cost).toBeLessThanOrEqual(PUBLIC_INVOCATION_STRING_BYTES);
      const encoded = JSON.stringify(page).slice(1, -1);
      expect(encoded.length - (encoded.match(/\\/gu)?.length ?? 0)).toBeLessThanOrEqual(PUBLIC_INVOCATION_STRING_BYTES);
    }
    expect(pages.join("")).toBe(payload);
  });

  it("yields one empty page for an empty payload, so a producer always sends an addressed page", () => {
    expect(publicInvocationStringPages("")).toEqual([""]);
  });
});
//#endregion 📄️PublicInvocationPaging

//#region 📄️ContributionsPushDeclaration
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { appCommandTakesPageRun, appOwnsCommand } from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";

/** 📄️ One plugin descriptor, read off disk rather than imported, because the procedural manifest is
 * ~1 MB and belongs in no test bundle. */
const readPluginManifest = (relativeUrl: string): { readonly apps?: readonly { readonly id: string; readonly commands?: readonly { readonly id: string }[] }[] } | undefined => {
  const path = fileURLToPath(new URL(relativeUrl, import.meta.url));
  if (!existsSync(path)) return undefined;
  const descriptor = JSON.parse(readFileSync(path, "utf8")) as { readonly manifest?: unknown };
  return (descriptor.manifest ?? descriptor) as { readonly apps?: readonly { readonly id: string; readonly commands?: readonly { readonly id: string }[] }[] };
};

const PROCEDURAL_SOURCE_DESCRIPTOR = "../../../../../../../../🌎️hub/🧩️compositions/🌀️procedural/🔣️.json";
/** 📄️ The dev server's own copy — a build artifact, so it is asserted only when a dev tree has one.
 * A stale copy is exactly how a served boot ends up receiving one unpaged 293 KiB command the guest
 * refuses as `command contains an oversized string`. */
const PROCEDURAL_SERVED_DESCRIPTOR = "../../../../🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/🔣️.json";

describe("contributions push declaration", () => {
  /** ⚖️ LAW: every procedural app that opts into the host's `setContributions` push declares the
   * `pageCount` argument so the addressed schema matches. The shell sends page 0 of 1 as one
   * pack-encoded `handleCommand` (`PluginRuntime` `encodePackValue`), not 4 KiB JSON pages.
   * The gate is `appCommandTakesPageRun`, so the law runs THAT, never a copy of it
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  it("declares pageCount on every procedural app the shell pushes contributions to", () => {
    for (const [label, relativeUrl] of [
      ["source", PROCEDURAL_SOURCE_DESCRIPTOR],
      ["served", PROCEDURAL_SERVED_DESCRIPTOR],
    ] as const) {
      const manifest = readPluginManifest(relativeUrl);
      if (!manifest) {
        expect(label).toBe("served");
        continue;
      }
      const receivers = (manifest.apps ?? []).filter((app) => appOwnsCommand(app as never, "setContributions"));
      expect(receivers.map((app) => app.id).sort()).toEqual(["s.procedural.generation2d@1/*#editor", "s.procedural.generation3d@1/*#editor", "s.procedural.generation3d@1/*#viewer"]);
      for (const app of receivers) {
        expect(`${label}:${app.id}:${appCommandTakesPageRun(app as never, "setContributions")}`).toBe(`${label}:${app.id}:true`);
      }
    }
  });
});
//#endregion 📄️ContributionsPushDeclaration

//#region 📇️WindowKindActionScoping
type DescriptorWindowKind = { readonly id: string; readonly actions?: readonly { readonly id: string }[] };
type DescriptorApp = { readonly id: string; readonly actions?: readonly { readonly id: string }[]; readonly windowKinds?: readonly DescriptorWindowKind[] };

const windowKindsDeclaring = (app: DescriptorApp, actionId: string): readonly string[] => (app.windowKinds ?? []).filter((kind) => (kind.actions ?? []).some((action) => action.id === actionId)).map((kind) => kind.id);

describe("window-kind action scoping", () => {
  /** ⚖️ LAW: a window-scoped action is declared on the window kinds that dispatch it and on NO other.
   * `WindowKindDefinition.actions` is what `ShellHost`'s `declaredAction` gate reads before it will call
   * `plugin.handleAction`, what the focused-window keybinding table resolves against, and what a window's
   * chrome menu renders — and until this ticket every generation3d/generation2d window carried a copy of
   * the whole app action list, because `build_definition` copies every UNOWNED app action onto every
   * window (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5334-5338`) and no window owned
   * anything. This reads the published descriptor, so it fails on a stale
   * `@semio-tech/procedural-plugin:describe` as well as on a lost `.window_kind_action_refs(...)`
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  it("scopes each generation window's own actions to that window in the published procedural descriptor", () => {
    const manifest = readPluginManifest(PROCEDURAL_SOURCE_DESCRIPTOR) as { readonly apps?: readonly DescriptorApp[] } | undefined;
    expect(manifest).toBeDefined();
    const apps = new Map((manifest?.apps ?? []).map((app) => [app.id, app]));
    const expectations: readonly (readonly [string, string, readonly string[]])[] = [
      ["s.procedural.generation3d@1/*#editor", "addGeneration", ["generation3d-generations"]],
      ["s.procedural.generation3d@1/*#editor", "selectGeneration", ["generation3d-generations"]],
      ["s.procedural.generation3d@1/*#editor", "renameGeneration", ["generation3d-generations"]],
      ["s.procedural.generation3d@1/*#editor", "removeGeneration", ["generation3d-generations"]],
      ["s.procedural.generation3d@1/*#editor", "updateGenerationValues", ["generation3d-generate-form"]],
      ["s.procedural.generation3d@1/*#editor", "setLodMode", ["procedural-main"]],
      ["s.procedural.generation3d@1/*#editor", "nodeGraphEdit", ["procedural-main"]],
      ["s.procedural.generation3d@1/*#editor", "setShowMode", ["generation3d-generate-preview", "procedural-preview"]],
      ["s.procedural.generation3d@1/*#editor", "setSunAzimuth", ["generation3d-generate-preview", "procedural-preview"]],
      ["s.procedural.generation3d@1/*#editor", "translateSelection", ["generation3d-generate-preview", "procedural-preview"]],
      ["s.procedural.generation3d@1/*#viewer", "setShowMode", ["procedural-view-preview"]],
      ["s.procedural.generation3d@1/*#viewer", "setCamera", ["procedural-view-preview"]],
      ["s.procedural.generation2d@1/*#editor", "addGeneration", ["generation2d-generations"]],
      ["s.procedural.generation2d@1/*#editor", "updateGenerationValues", ["generation2d-generate-form"]],
      ["s.procedural.generation2d@1/*#editor", "nodeGraphViewport", ["generation2d-main"]],
      ["s.procedural.generation2d@1/*#editor", "canvasWheel", ["generation2d-generate-preview", "generation2d-preview"]],
    ];
    for (const [appId, actionId, owners] of expectations) {
      const app = apps.get(appId);
      expect(`${appId}:declared`).toBe(app ? `${appId}:declared` : `${appId}:missing`);
      expect(
        `${appId}:${actionId}:${windowKindsDeclaring(app as DescriptorApp, actionId)
          .slice()
          .sort()
          .join(",")}`,
      ).toBe(`${appId}:${actionId}:${owners.slice().sort().join(",")}`);
    }
  });

  /** ⚖️ App-scoped verbs have one app declaration and pass the production dispatch gate from every window. */
  it("declares procedural app-scoped verbs once and accepts them from every window kind", () => {
    const manifest = readPluginManifest(PROCEDURAL_SOURCE_DESCRIPTOR) as { readonly apps?: readonly DescriptorApp[] } | undefined;
    const apps = new Map((manifest?.apps ?? []).map((app) => [app.id, app]));
    for (const [appId, actionId] of [
      ["s.procedural.generation3d@1/*#editor", "setActiveExample"],
      ["s.procedural.generation3d@1/*#editor", "addWidget"],
      ["s.procedural.generation3d@1/*#editor", "undo"],
      ["s.procedural.generation2d@1/*#editor", "addWidget"],
      ["s.procedural.generation2d@1/*#editor", "undo"],
    ] as const) {
      const app = apps.get(appId) as DescriptorApp;
      expect(app, appId).toBeDefined();
      expect(
        app.actions?.filter((action) => action.id === actionId),
        `${appId}:${actionId} app owner`,
      ).toHaveLength(1);
      expect(windowKindsDeclaring(app, actionId), `${appId}:${actionId} explicit window owners`).toEqual([]);
      expect(app.windowKinds?.length).toBeGreaterThan(0);
      for (const windowKindId of [null, ...(app.windowKinds ?? []).map((kind) => kind.id)]) {
        expect(undeclaredActionDiagnostic(appId, actionId, app.windowKinds ?? [], windowKindId, app.actions), `${appId}:${actionId}:${windowKindId}`).toBeNull();
        if (actionId !== "undo")
          expect(
            undeclaredActionDiagnostic(
              appId,
              actionId,
              app.windowKinds ?? [],
              windowKindId,
              app.actions?.filter((action) => action.id !== actionId),
            ),
          ).not.toBeNull();
      }
    }
  });
});
//#endregion 📇️WindowKindActionScoping

//#region 🪪️StableUiNodeDomIds
describe("stable ui node dom ids", () => {
  /** ⚖️ LAW: a bound node's DOM id is a function of the SURFACE and the node's own Rust-authored `key`,
   * never of `UiNodeRecord.id` — that integer is re-minted in DFS order on every full-body
   * reconciliation (`builtNodeToSnapshot`), so an id built from it names a different row after the next
   * refresh and a scripted or assistive click silently targets the wrong node
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  it("derives the dom id from the surface and the authored key, not the reconciliation ordinal", () => {
    expect(uiNodeDomId("procedural.play.generations", "procedural3d-play-generate.add-generation", 5)).toBe("procedural.play.generations/procedural3d-play-generate.add-generation");
    expect(uiNodeDomId("procedural.play.generations", "procedural3d-play-generate.add-generation", 12)).toBe(uiNodeDomId("procedural.play.generations", "procedural3d-play-generate.add-generation", 5));
  });

  /** ⚖️ LAW: the surface is the namespace — two windows of one app rendering the same authored key get
   * two distinct DOM ids. */
  it("namespaces the same authored key per window surface", () => {
    expect(uiNodeDomId("procedural.play.generations", "generation.g1", 1)).not.toBe(uiNodeDomId("procedural.play.generate-form", "generation.g1", 1));
  });

  /** ⚖️ LAW: a keyless node still gets a usable id — the volatile ordinal is the FALLBACK, never the
   * default. */
  it("falls back to the reconciliation ordinal only for a keyless node", () => {
    expect(uiNodeDomId("procedural.play.main", "", 7)).toBe("node-7");
  });

  /** ⚖️ LAW: every authored key of one rendered body mints a distinct DOM id, so `getElementById` is
   * unambiguous across the whole window body. */
  it("mints one distinct dom id per authored key of a body", () => {
    const keys = ["procedural3d-play-generate", "procedural3d-play-generate.generations", "procedural3d-play-generate.generation.generation-1", "procedural3d-play-generate.actions", "procedural3d-play-generate.add-generation"];
    const ids = new Set(keys.map((key, index) => uiNodeDomId("procedural.play.generations", key, index + 1)));
    expect(ids.size).toBe(keys.length);
  });
});
//#endregion 🪪️StableUiNodeDomIds

//#region 🚨️UndeclaredActionDiagnostic
describe("undeclared action diagnostic", () => {
  const windowKinds = [
    { id: "generation3d-generations", actions: [{ id: "addGeneration" }] },
    { id: "procedural-main", actions: [{ id: "nodeGraphEdit" }] },
  ];

  /** ⚖️ LAW: the drop is DESCRIBED, not silent — the message names the app, the action and the window
   * kind the dispatch came from, and is not `[TRACE]`-prefixed, because it is the only signal a fully
   * wired binding died (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
  it("names the app, the action and the dispatching window kind", () => {
    const diagnostic = undeclaredActionDiagnostic("generation3d", "setActiveExample", windowKinds, "procedural-main");
    expect(diagnostic).not.toBeNull();
    expect(diagnostic?.action).toBe("setActiveExample");
    expect(diagnostic?.windowKindId).toBe("procedural-main");
    expect(diagnostic?.windowKindIds).toEqual(["generation3d-generations", "procedural-main"]);
    expect(diagnostic?.message.includes("[TRACE]")).toBe(false);
    for (const fragment of ["generation3d", "setActiveExample", "procedural-main", "window_kind_action_refs"]) expect(diagnostic?.message.includes(fragment)).toBe(true);
  });

  /** ⚖️ LAW: the gate is app-wide — an action owned by ONE window kind is dispatchable from anywhere
   * (a context menu, the palette, a keybinding), so declaring `addGeneration` only on the Generations
   * window must not turn a context-menu dispatch from the flow window into a drop. */
  it("accepts an action declared on any window kind, whatever window dispatched it", () => {
    expect(undeclaredActionDiagnostic("generation3d", "addGeneration", windowKinds, "procedural-main")).toBeNull();
    expect(undeclaredActionDiagnostic("generation3d", "addGeneration", windowKinds, null)).toBeNull();
  });

  /** ⚖️ LAW: the framework's own reserved verbs are never a plugin's to declare. */
  it("never reports a framework-reserved verb", () => {
    for (const action of ["undo", "redo", "setActiveUtility", "setActiveTool"]) expect(undeclaredActionDiagnostic("generation3d", action, windowKinds)).toBeNull();
  });

  /** ⚖️ LAW: an app with no window kinds at all still produces a readable message rather than an empty list. */
  it("says so when the app declares no window kinds", () => {
    expect(undeclaredActionDiagnostic("empty", "addGeneration", [])?.message.includes("window kinds: none")).toBe(true);
  });
});
//#endregion 🚨️UndeclaredActionDiagnostic

describe("history patch apply", () => {
  it("applies an equal-cursor history patch when it carries upserts", () => {
    expect(historyPatchShouldApplyV1(0, { cursor: 0, upserts: [{ seq: 1 }] })).toBe(true);
    expect(historyPatchShouldApplyV1(0, { cursor: 1, upserts: [{ seq: 1 }] })).toBe(true);
    expect(historyPatchShouldApplyV1(1, { cursor: 0, upserts: [{ seq: 1 }] })).toBe(false);
    expect(historyPatchShouldApplyV1(1, { cursor: 1 })).toBe(false);
    expect(historyPatchShouldApplyV1(1, { cursor: 0 }, true)).toBe(true);
    expect(historyRefreshNeededV1("setActiveExample", undefined)).toBe(true);
    expect(historyRefreshNeededV1("setActiveExample", { upserts: [] })).toBe(true);
    expect(historyRefreshNeededV1("setActiveExample", { upserts: [{ seq: 1 }] })).toBe(false);
    expect(historyRefreshNeededV1("undo", undefined)).toBe(false);
  });
});

//#region 🎨️ExampleSwitchHostCaching
// 🎨️ The HOST half of the example-switch runtime law (`🧫️fixtures/🎨️example-switch.json` answers the
// guest half in Rust). Picking an example replaces the artifact's fixture through a RETAINED typed
// operation, so the flow window's new body reaches the shell only through that operation's
// completion — and only if the completion's own `UiDirtyScope` asks for the window body. The trap
// this block pins: a completion that dirties nothing but carries a history patch refreshes ONLY the
// reserved history panel, which repaints the History list while the flow window keeps the previous
// example's graph and nothing faults anywhere (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
describe("example switch — the completion's scope is what re-takes the flow window body", () => {
  const generation3dWindows = [
    { id: "procedural-main", bodyKey: "procedural.play.main" },
    { id: "procedural-preview", bodyKey: "procedural.play.preview" },
  ];
  const historyPanel = [{ kind: { kind: "app" as const, id: "framework.panel.history" }, bodyKey: FRAMEWORK_HISTORY_BODY_KEY }];

  it("a full-scope completion re-takes the flow window body, and a history-only completion never does", () => {
    const full = typedOperationCompletionRefreshV1({ uiScope: { kind: "full" }, historyPatch: { cursor: 4 }, requestedEffects: [] });
    expect(full).toEqual({ kind: "full" });
    expect(buildUiRefreshRequest(full!, generation3dWindows, historyPanel, {}, new Map())?.windows?.map((entry) => entry.bodyKey)).toEqual(["procedural.play.main", "procedural.play.preview"]);
    const historyOnly = typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: { cursor: 4 }, requestedEffects: [] });
    expect(historyOnly).toEqual({ kind: "partial", panelBodies: [FRAMEWORK_HISTORY_BODY_KEY] });
    expect(buildUiRefreshRequest(historyOnly!, generation3dWindows, historyPanel, {}, new Map())?.windows ?? []).toEqual([]);
  });

  it("the guest's new graph replaces the cached flow body while the untouched preview body keeps its identity", () => {
    const cache: UiRefreshCache = new Map([
      ["window:procedural-main", { hash: "hex-column-hash", value: { type: "nodeGraph", value: "hexagonal-mushroom-column" } }],
      ["window:procedural-preview", { hash: "preview-hash", value: { type: "world3d", value: "preview" } }],
    ]);
    const previousBodies = { "procedural-main": cache.get("window:procedural-main")!.value, "procedural-preview": cache.get("window:procedural-preview")!.value } as Readonly<Record<string, unknown>>;
    const request = buildUiRefreshRequest({ kind: "full" }, generation3dWindows, historyPanel, {}, cache);
    expect(request?.windows?.find((entry) => entry.key === "procedural-main")?.hash).toBe("hex-column-hash");
    applyUiRefreshResponseToCache(cache, {
      windows: [
        { key: "procedural-main", hash: "box-shell-hash", value: { type: "nodeGraph", value: "box-shell-preview" } },
        { key: "procedural-preview", hash: "preview-hash" },
      ],
    });
    const merged = mergeRecordPreservingIdentity(
      previousBodies,
      generation3dWindows.map((instance) => [instance.id, cache.get(`window:${instance.id}`)?.value ?? previousBodies[instance.id]] as const),
    );
    expect(merged["procedural-main"]).toEqual({ type: "nodeGraph", value: "box-shell-preview" });
    expect(merged["procedural-preview"]).toBe(previousBodies["procedural-preview"]);
  });

  it("a guest that re-renders the PREVIOUS example answers the same hash, so the shell keeps showing it — the failure this law names", () => {
    const cache: UiRefreshCache = new Map([["window:procedural-main", { hash: "hex-column-hash", value: { type: "nodeGraph", value: "hexagonal-mushroom-column" } }]]);
    applyUiRefreshResponseToCache(cache, { windows: [{ key: "procedural-main", hash: "hex-column-hash" }] });
    expect(cache.get("window:procedural-main")?.value).toEqual({ type: "nodeGraph", value: "hexagonal-mushroom-column" });
  });

  it("a refresh asked for while one is already crossing is coalesced onto the union, never dropped", () => {
    expect(mergeUiDirtyScopeV1({ kind: "partial", windowBodies: ["procedural.play.main"] }, { kind: "partial", windowBodies: ["procedural.play.preview"], measures: true })).toEqual({
      kind: "partial",
      windowBodies: ["procedural.play.main", "procedural.play.preview"],
      panelBodies: [],
      utilities: false,
      tools: false,
      engagements: false,
      measures: true,
      labels: false,
    });
    expect(mergeUiDirtyScopeV1({ kind: "partial", windowBodies: ["procedural.play.main"] }, { kind: "full" })).toEqual({ kind: "full" });
    expect(mergeUiDirtyScopeV1({ kind: "full" }, { kind: "partial", windowBodies: [] })).toEqual({ kind: "full" });
    expect(mergeUiDirtyScopeV1({ kind: "none" }, { kind: "partial", windowBodies: ["procedural.play.main"] })).toEqual({ kind: "partial", windowBodies: ["procedural.play.main"] });
    expect(mergeUiDirtyScopeV1({ kind: "partial", panelBodies: [FRAMEWORK_HISTORY_BODY_KEY] }, { kind: "none" })).toEqual({ kind: "partial", panelBodies: [FRAMEWORK_HISTORY_BODY_KEY] });
    // ⚖️ The union of two window-body passes still asks the guest for BOTH bodies — the whole point
    // of coalescing instead of letting the newer pass supersede (and discard) the older one.
    const union = mergeUiDirtyScopeV1({ kind: "partial", windowBodies: ["procedural.play.main"] }, { kind: "partial", windowBodies: ["procedural.play.preview"] });
    expect(buildUiRefreshRequest(union, generation3dWindows, historyPanel, {}, new Map())?.windows?.map((entry) => entry.bodyKey)).toEqual(["procedural.play.main", "procedural.play.preview"]);
  });
});
//#endregion 🎨️ExampleSwitchHostCaching

//#region 📷️CameraAndLabelFitTwins
import cameraFitFixture from "../../../../../../../🔨️modules/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json" with { type: "json" };
import portTypesFixture from "../../../../🌊️flow/🧫️fixtures/🔌️port-types/🔣️.json" with { type: "json" };
import labelFitFixture from "../../../../../../../🔨️modules/🖼️canvas/🧫️fixtures/🏷️label-fit/🔣️.json" with { type: "json" };

/** 📏️ The fixture's own synthetic advance — the ONE measure both implementations are driven with, so
 * a row pins the clipping algorithm rather than a font file. Mirror of `synthetic_measure` in
 * `🧰️framework/🔨️modules/🖼️canvas/🧪️tests/🏷️label-fit/🦀️.rs`. */
function syntheticLabelMeasure(text: string, charWidth: number): number {
  return [...text].length * charWidth;
}

/** 🖋️ A 2D context that records every call and measures text as `charWidth * glyphs` at the font it
 * was last given — enough to prove what `paintDagLabelOverlays` actually draws, with no font files. */
function recordingLabelContext(charWidthPerPx: number) {
  const calls: { readonly op: string; readonly args: readonly unknown[] }[] = [];
  let fontPx = 0;
  const context = {
    get font() {
      return `${fontPx}px sans-serif`;
    },
    set font(value: string) {
      fontPx = Number.parseFloat(value) || 0;
      calls.push({ op: "font", args: [value] });
    },
    fillStyle: "",
    globalAlpha: 1,
    textAlign: "center",
    textBaseline: "middle",
    measureText: (text: string) => ({ width: [...text].length * fontPx * charWidthPerPx }),
    fillText: (text: string, x: number, y: number) => calls.push({ op: "fillText", args: [text, x, y, fontPx] }),
    clearRect: () => calls.push({ op: "clearRect", args: [] }),
    setTransform: () => calls.push({ op: "setTransform", args: [] }),
    save: () => calls.push({ op: "save", args: [] }),
    restore: () => calls.push({ op: "restore", args: [] }),
    translate: () => calls.push({ op: "translate", args: [] }),
    rotate: () => calls.push({ op: "rotate", args: [] }),
  };
  return { context, calls, drawn: () => calls.filter((call) => call.op === "fillText") };
}

function labelCanvasStub(charWidthPerPx: number) {
  const recorder = recordingLabelContext(charWidthPerPx);
  const canvas = { width: 0, height: 0, style: {} as Record<string, string>, getContext: () => recorder.context } as unknown as HTMLCanvasElement;
  return { canvas, ...recorder };
}

describe("node-graph caption clipping (2D replay twin)", () => {
  it("clips every fixture row to the string the shared law names", () => {
    const rows = labelFitFixture.rows;
    expect(rows.length).toBeGreaterThanOrEqual(8);
    expect(labelFitFixture.provenance.ellipsis).toBe(DAG_LABEL_ELLIPSIS);
    for (const row of rows) {
      const fitted = dagEllipsizeByMeasure(row.text, row.maxWidth, (candidate) => syntheticLabelMeasure(candidate, row.charWidth));
      expect([row.name, fitted]).toEqual([row.name, row.expect]);
      expect(syntheticLabelMeasure(fitted, row.charWidth)).toBeLessThanOrEqual(Math.max(row.maxWidth, 0) + 1e-9);
      expect([...fitted].filter((glyph) => glyph === DAG_LABEL_ELLIPSIS).length).toBeLessThanOrEqual(1);
    }
  });

  it("draws a node title clipped at its measured width instead of shrinking the font into a smudge", () => {
    // 🏷️ The live defect's shape: a 40-world-unit node at zoom 1.78 gives a ~63px caption budget and
    // the title is far wider. Before the fix the overlay binary-searched the font down to 4px.
    const stub = labelCanvasStub(0.6);
    const state = JSON.stringify({
      camera: { x: 0, y: 0, zoom: 1.7844325616011099 },
      width: 483,
      height: 814,
      labels: [{ id: "extrude", text: "Brep.solid.extrude", layout: "horizontal", x: 0, y: 0, nodeW: 40, nodeH: 28, fontScreenPx: 13.085838785074808, ghost: false }],
    });
    paintDagLabelOverlays(state, stub.canvas, 483, 814, 1, { selectedIds: [], preselect: { ids: [], removedIds: [] }, hoveredId: null });
    const drawn = stub.drawn();
    expect(drawn).toHaveLength(1);
    const [text, , , fontPx] = drawn[0]!.args as [string, number, number, number];
    expect(fontPx).toBeGreaterThanOrEqual(8);
    expect(text.endsWith(DAG_LABEL_ELLIPSIS)).toBe(true);
    expect(text.startsWith("Brep")).toBe(true);
    expect([...text].length * fontPx * 0.6).toBeLessThanOrEqual(40 * 1.7844325616011099 * 0.88 + 1e-9);
  });

  it("centres captions on the measured overlay, not on a stale session size", () => {
    // 🏷️ The live defect: the session reported `width: 1, height: 1` (painted at hand-over, before the
    // engine canvas was attached) beside an 852×807 overlay, so a node at the world origin was captioned
    // at (0.5, 0.5) instead of the canvas centre.
    const stub = labelCanvasStub(0.6);
    const state = JSON.stringify({
      camera: { x: 0, y: 0, zoom: 1 },
      width: 1,
      height: 1,
      labels: [{ id: "root", text: "B", layout: "horizontal", x: 0, y: 0, nodeW: 40, nodeH: 28, fontScreenPx: 13.75, ghost: false }],
    });
    paintDagLabelOverlays(state, stub.canvas, 852, 807, 1, { selectedIds: [], preselect: { ids: [], removedIds: [] }, hoveredId: null });
    const [, x, y] = stub.drawn()[0]!.args as [string, number, number];
    expect([x, y]).toEqual([426, 403.5]);
  });

  it("draws a title that fits whole, which is what the live generation3d operator names do", () => {
    const stub = labelCanvasStub(0.6);
    const state = JSON.stringify({
      camera: { x: 0, y: 0, zoom: 1.7844325616011099 },
      width: 483,
      height: 814,
      labels: [{ id: "extrude", text: "Extrude", layout: "horizontal", x: 0, y: 0, nodeW: 40, nodeH: 28, fontScreenPx: 13.085838785074808, ghost: false }],
    });
    paintDagLabelOverlays(state, stub.canvas, 483, 814, 1, { selectedIds: [], preselect: { ids: [], removedIds: [] }, hoveredId: null });
    expect((stub.drawn()[0]!.args as [string])[0]).toBe("Extrude");
  });
});

describe("node-graph opening camera (renderer twin)", () => {
  it("opens on the camera the shared law names for every fixture row", () => {
    const rows = cameraFitFixture.rows;
    expect(rows.length).toBeGreaterThanOrEqual(8);
    expect(cameraFitFixture.provenance.constants.paddingPx).toBe(DAG_CONTENT_FIT_PADDING_PX);
    expect(cameraFitFixture.provenance.constants.minCoverage).toBe(DAG_CONTENT_FRAMED_MIN_COVERAGE);
    expect(cameraFitFixture.provenance.constants.refitMaxCoverage).toBe(DAG_CONTENT_REFIT_MAX_COVERAGE);
    for (const row of rows) {
      const { camera, fitted } = dagStartupCamera(row.stored, row.content, row.viewport.width, row.viewport.height);
      expect([row.name, camera.x, camera.y, camera.zoom, fitted]).toEqual([row.name, row.expect.camera.x, row.expect.camera.y, row.expect.camera.zoom, row.expect.fitted]);
      if (row.content && row.stored) {
        const coverage = dagContentCoverage(row.content, row.stored, row.viewport.width, row.viewport.height);
        expect([row.name, coverage]).toEqual([row.name, row.expect.coverage]);
        expect([row.name, coverage <= DAG_CONTENT_REFIT_MAX_COVERAGE]).toEqual([row.name, row.expect.refits]);
      }
    }
  });

  it("derives the graph bounds a fit frames from the scene's own node records", () => {
    const nodes = [
      { id: "a", x: 0, y: 0, width: 40, height: 20, inputs: [], outputs: [] },
      { id: "b", x: 200, y: -100, width: 40, height: 20, inputs: [], outputs: [] },
    ];
    expect(dagContentBounds(nodes)).toEqual({ minX: -20, minY: -110, maxX: 220, maxY: 10 });
    expect(dagContentBounds([])).toBeNull();
    expect(dagContentBounds(undefined)).toBeNull();
    const fitted = dagFitCamera(dagContentBounds(nodes)!, 483, 814);
    expect(dagContentCoverage(dagContentBounds(nodes)!, fitted, 483, 814)).toBeCloseTo(1, 9);
  });

  it("publishes the camera it computed for every surface row", () => {
    const rows = portSidesSurfaceRows();
    expect(rows.length).toBeGreaterThanOrEqual(4);
    for (const row of rows) {
      const nodes = [{ id: "graph", x: (row.content.minX + row.content.maxX) / 2, y: (row.content.minY + row.content.maxY) / 2, width: row.content.maxX - row.content.minX, height: row.content.maxY - row.content.minY, inputs: [], outputs: [] }];
      const content = dagContentBounds(nodes)!;
      const published = dagFitCamera(content, row.viewport.width, row.viewport.height);
      expect([row.name, published.x, published.y, published.zoom]).not.toEqual([row.name, row.camera.x, row.camera.y, row.camera.zoom]);
      expect([row.name, dagContentCoverage(content, published, row.viewport.width, row.viewport.height)]).toEqual([row.name, row.expect.coverageAfterFit]);
      const again = dagFitCamera(content, row.viewport.width, row.viewport.height);
      expect([row.name, again.x, again.y, again.zoom]).toEqual([row.name, published.x, published.y, published.zoom]);
    }
  });

  it("re-frames only on a layout change, never on hover, selection or evaluation churn", () => {
    const nodes = [{ id: "a", x: 0, y: 0, width: 40, height: 20, inputs: [], outputs: [] }];
    const same = [{ id: "a", x: 0, y: 0, width: 40, height: 20, inputs: [{ id: "a@in", label: "in" }], outputs: [] }];
    const moved = [{ id: "a", x: 900, y: 0, width: 40, height: 20, inputs: [], outputs: [] }];
    expect(nodeGraphContentSignature(nodes)).toBe(nodeGraphContentSignature(same));
    expect(nodeGraphContentSignature(nodes)).not.toBe(nodeGraphContentSignature(moved));
  });
});

/** 🖼️ The `surfaceRows` half of the shared camera-fit fixture — the rows that say a fit must PUBLISH
 * what it computed. The Rust half drives a real `FlowHost`
 * (`🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs`, `a_fitted_flow_surface_publishes_the_camera_it_computed`);
 * this half drives the renderer's own fit rule over the same rows. */
function portSidesSurfaceRows(): {
  name: string;
  viewport: { width: number; height: number };
  camera: { x: number; y: number; zoom: number };
  content: { minX: number; minY: number; maxX: number; maxY: number };
  expect: { coverageAfterFit: number };
}[] {
  return (
    cameraFitFixture as unknown as {
      surfaceRows: { name: string; viewport: { width: number; height: number }; camera: { x: number; y: number; zoom: number }; content: { minX: number; minY: number; maxX: number; maxY: number }; expect: { coverageAfterFit: number } }[];
    }
  ).surfaceRows;
}


//#endregion 📷️CameraAndLabelFitTwins

//#region 📚️BootExampleTwin
/** 📚️ The TypeScript half of the boot-example law: the SAME
 * `🐚️Shell/🧫️fixtures/📚️boot-example/🔣️.json` rows the Rust `🔬️wgpu-shell-chrome-parity` law answers
 * through `resolve_boot_example_id`, answered here by the shipped `resolveBootExampleId` — two
 * independent implementations, one fixture, so the two shells cannot drift on which document a url
 * opens. The `bootQuery` half pins the `?example=` axis itself against the shipped
 * `resolveBootQueryExampleId`, the parser both entries spell the query with.
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
describe("📚️ boot example contract", () => {
  it("validates the shared fixture against its own declared schema", () => {
    expect(bootExampleFixture.rows.map((row) => resolveBootExampleId(row.activeExampleId, row.options.map((id: string) => ({ id })), row.defaultExampleId ?? undefined))).toEqual(bootExampleFixture.rows.map((row) => row.expected));
    expect(bootExampleFixture.bootQueryParam).toBe(BOOT_QUERY_EXAMPLE_PARAM);
  });

  it("resolves every shared fixture row the way the wgpu shell does", () => {
    for (const row of bootExampleFixture.rows) {
      const options = row.options.map((id: string) => ({ id }));
      expect(resolveBootExampleId(row.activeExampleId, options, row.defaultExampleId ?? undefined), row.id).toBe(row.expected);
    }
  });

  it("reads the `?example=` axis the same way on both entries", () => {
    for (const row of bootExampleFixture.bootQuery) {
      expect(resolveBootQueryExampleId(row.search, undefined) ?? null, row.id).toBe(row.expected);
    }
    expect(resolveBootQueryExampleId("?plugin=generation3d", "seeded"), "an absent query keeps the per-server seed").toBe("seeded");
    expect(resolveBootQueryExampleId("?plugin=generation3d&example=", "seeded"), "an empty value is the same as no value at all").toBe("seeded");
    expect(() => resolveBootQueryExampleId("?example=" + "x".repeat(BOOT_QUERY_CAPACITY), undefined)).toThrow(/boot-query-overflow/);
  });
});
//#endregion 📚️BootExampleTwin

//#region 🛑️SurfaceControlCancelTwin
/** 🛑️ The TypeScript half of the surface-control laws: the SAME
 * `🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` rows the Rust
 * `🔬️wgpu-shell-chrome-parity` laws answer through `world3d_cancel_affordance`, answered here by the
 * shipped `world3dComputeStatusV1` parser — two independent implementations, one fixture, so neither
 * renderer can drift into offering a cancel the producer never authorised.
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
describe("🛑️ world3d cancel contract", () => {
  it("validates the shared fixture against its own declared schema", () => {
    expect(surfaceControlsFixture.cancelContract.map((row) => world3dComputeStatusV1(row.statusJson).cancellable)).toEqual(surfaceControlsFixture.cancelContract.map((row) => row.expected.cancellable));
  });

  it("offers a cancel affordance for exactly the rows the shared fixture declares", () => {
    const rows = surfaceControlsFixture.cancelContract;
    for (const row of rows) {
      const status = world3dComputeStatusV1(row.statusJson);
      expect(status.cancellable, row.id).toBe(row.expected.cancellable);
      expect(status.cancelAction, row.id).toBe(row.expected.cancelAction);
      expect(status.cancelArgs, row.id).toEqual(row.expected.cancelArgs);
    }
  });

  it("derives the control ids the wgpu shell paints from the same rows", () => {
    for (const row of surfaceControlsFixture.surfaceControls) {
      const { controlHeightPx, surfaceControlMinimum } = surfaceControlsFixture;
      const fits = (bounds: readonly number[]) => bounds[2] >= surfaceControlMinimum.widthControlHeights * controlHeightPx && bounds[3] >= surfaceControlMinimum.heightControlHeights * controlHeightPx;
      const ids = [
        ...row.worlds.filter((world: { bounds: number[]; statusJson: string | null }) => fits(world.bounds) && world3dComputeStatusV1(world.statusJson).cancellable).map((world: { surfaceId: string }) => `shell.world3d.cancel::${world.surfaceId}`),
      ];
      expect(ids, row.id).toEqual(row.expected);
    }
  });
});
//#endregion 🛑️SurfaceControlCancelTwin

//#region 🛟️ChromePanelSafeAreaTwin
/** 🛟️ The TypeScript half of the chrome-panel SAFE AREA: the SAME
 * `Framework UI 🧫️fixtures/🛟️chrome-panel-safe-area/🔣️.json` `chromePanelSafeArea` rows the Rust
 * `🔬️wgpu-shell-chrome-parity` law answers through `chrome_panel_safe_area`, answered here by the shipped
 * `chromePanelSafeArea` the React world pane's overlay rail and every window's right-edge chrome read —
 * two independent implementations, one fixture, so neither renderer can drift into painting an anchored
 * chrome panel over an affordance a user has to press. The defect it exists for is
 * `📓️react-oracle-hardening-2026-09-14.md` §4.3: the `top-right` Tool runs panel at (1137, 3) 300×120
 * swallowing `Frame visible` and the preview `Cancel`. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
describe("🛟️ chrome panel safe area", () => {
  
  const box = ([x, y, w, h]: readonly number[]) => ({ left: x, top: y, right: x + w, bottom: y + h });

  it("validates the shared safe-area corpus against its own declared schema", () => {
    expect(chromePanelSafeAreaFixture.length).toBeGreaterThanOrEqual(6);
  });

  it("reserves every shared fixture row exactly the way the wgpu shell does", () => {
    for (const row of chromePanelSafeAreaFixture) {
      const safeArea = chromePanelSafeArea(box(row.affordance), box(row.host), row.anchor as Anchor, row.panels.map(box), row.yield as SafeAreaYield, row.gap);
      expect(safeArea.inlinePx, `${row.id}: inline reserve`).toBe(row.expected.inline);
      expect(safeArea.blockPx, `${row.id}: block reserve`).toBe(row.expected.block);
    }
  });

  it("leaves every reserved affordance clear of the panels it yielded to", () => {
    for (const row of chromePanelSafeAreaFixture) {
      const affordance = box(row.affordance);
      const safeArea = chromePanelSafeArea(affordance, box(row.host), row.anchor as Anchor, row.panels.map(box), row.yield as SafeAreaYield, row.gap);
      if (safeArea.inlinePx === 0 && safeArea.blockPx === 0) continue;
      const inlineShift = row.anchor.endsWith("right") ? -safeArea.inlinePx : safeArea.inlinePx;
      const blockShift = row.anchor.startsWith("bottom") ? -safeArea.blockPx : safeArea.blockPx;
      const cleared = { left: affordance.left + inlineShift, right: affordance.right + inlineShift, top: affordance.top + blockShift, bottom: affordance.bottom + blockShift };
      for (const panel of row.panels.map(box)) {
        expect(cleared.left < panel.right && cleared.right > panel.left && cleared.top < panel.bottom && cleared.bottom > panel.top, `${row.id}: still covered by a panel`).toBe(false);
      }
    }
  });
});
//#endregion 🛟️ChromePanelSafeAreaTwin

//#region ⏳️ComputeStatusPaneTwin
/** ⏳️ The TypeScript half of the World3d compute-status PROGRESS laws: the same
 * `🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` `statusPane` rows the Rust
 * `🔬️wgpu-shell-chrome-parity` laws answer through `world3d_compute_status`/`world3d_status_pill_for`,
 * answered here by the shipped `world3dComputeStatusV1` parser React's own `WorldComputeStatusPane`
 * reads. Two independent implementations, one fixture — which is what makes "wgpu shows the same
 * progress React shows" a law rather than a claim. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
describe("⏳️ world3d compute status pane", () => {
  type StatusPaneRow = {
    readonly id: string;
    readonly statusJson: string | null;
    readonly expected: {
      readonly visible: boolean;
      readonly computing: boolean;
      readonly phase: string;
      readonly phaseLabel: { readonly en: string; readonly de: string } | null;
      readonly unitsDone: number;
      readonly unitsTotal: number;
      readonly facesDone: number;
      readonly facesTotal: number;
      readonly inFlight: number;
      readonly ratio: number;
      readonly progressText: string | null;
    };
  };
  const rows = surfaceControlsFixture.statusPane as readonly StatusPaneRow[];

  /** ⏳️ React's own pane gate — a settled producer annotates nothing, so an idle viewport is never
   * covered by chrome (`🌐️World3dHost/🟦️.tsx`'s `WorldComputeStatusPane` first line). */
  const isVisible = (status: ReturnType<typeof world3dComputeStatusV1>) => status.computing || status.cancellable || status.phase === "cancelled";
  /** 📈️ The count React renders beside the phase, and the wgpu pill joins into its one glyph run. */
  const progressText = (status: ReturnType<typeof world3dComputeStatusV1>) => (status.unitsTotal > 0 ? `${status.unitsDone}/${status.unitsTotal} (${Math.round(status.ratio * 100)}%)` : null);

  it("reads every declared field of the shared status rows", () => {
    for (const row of rows) {
      const status = world3dComputeStatusV1(row.statusJson);
      expect(status.computing, row.id).toBe(row.expected.computing);
      expect(status.phase, row.id).toBe(row.expected.phase);
      expect(status.phaseLabel, row.id).toEqual(row.expected.phaseLabel);
      expect(status.unitsDone, row.id).toBe(row.expected.unitsDone);
      expect(status.unitsTotal, row.id).toBe(row.expected.unitsTotal);
      expect(status.facesDone, row.id).toBe(row.expected.facesDone);
      expect(status.facesTotal, row.id).toBe(row.expected.facesTotal);
      expect(status.inFlight, row.id).toBe(row.expected.inFlight);
      expect(status.ratio, row.id).toBeCloseTo(row.expected.ratio, 9);
    }
  });

  it("carries the producer's own empty-surface hint, and an idle surface that HAS one is not silent", () => {
    // 🕳️ `preview_hint` reached `data-status-json` and stopped there: this reader dropped the field
    // and nothing painted it, so a user looking at an empty generate preview was told nothing
    // (`panel-i18n · de:preview_hint`). The pane's own gate is `computing || cancellable ||
    // phase === "cancelled"`; the hint branch is what an IDLE surface with something to say uses.
    const german = "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
    const withHint = world3dComputeStatusV1(JSON.stringify({ computing: false, phase: "idle", hint: german }));
    expect(withHint.hint).toBe(german);
    expect(isVisible(withHint)).toBe(false);
    expect(withHint.hint.length > 0).toBe(true);
    const withoutHint = world3dComputeStatusV1(JSON.stringify({ computing: false, phase: "idle" }));
    expect(withoutHint.hint).toBe("");
    expect(world3dComputeStatusV1(JSON.stringify({ hint: 42 })).hint).toBe("");
    expect(world3dComputeStatusV1(JSON.stringify({ hint: "x".repeat(400) })).hint.length).toBe(200);
    for (const row of rows) expect(typeof world3dComputeStatusV1(row.statusJson).hint, row.id).toBe("string");
  });

  it("shows the pane for exactly the unsettled rows, with exactly the declared progress text", () => {
    let painted = 0;
    for (const row of rows) {
      const status = world3dComputeStatusV1(row.statusJson);
      expect(isVisible(status), row.id).toBe(row.expected.visible);
      if (!row.expected.visible) continue;
      expect(progressText(status), row.id).toBe(row.expected.progressText);
      painted += 1;
    }
    expect(painted).toBeGreaterThan(0);
  });

  it("never defaults to one language: a half-translated phase label is no label at all", () => {
    const halved = rows.find((row) => row.id === "a-half-translated-phase-label-is-no-label-at-all");
    expect(halved, "the fixture declares the half-translated row").toBeTruthy();
    expect(world3dComputeStatusV1(halved!.statusJson).phaseLabel).toBeNull();
    const paired = world3dComputeStatusV1(rows[0].statusJson).phaseLabel;
    expect(paired?.en).not.toBe(paired?.de);
  });

  /** ⏳️⛓️ The TIMELINE law — the TypeScript twin of the Rust
   * `a_long_evaluation_publishes_a_monotone_run_of_non_idle_frames_and_then_settles`, over the same
   * `progressTimeline` frames. No single row can state it: a producer that publishes `idle`
   * throughout satisfies every row assertion above and fails this one, which is exactly what BOTH
   * renderers were showing — one 23 s evaluation, 54 publications, every one of them
   * `phase:"idle" inFlight:0 ratio:1.0` (`📓️wgpu-progress-visibility-2026-09-14.md`). */
  type TimelineFrame = { readonly id: string; readonly statusJson: string; readonly expected: { readonly visible: boolean; readonly phase: string; readonly ratio: number; readonly progressText: string | null } };
  const timeline = surfaceControlsFixture.progressTimeline as { readonly minimumNonIdleFrames: number; readonly evaluation: readonly TimelineFrame[]; readonly cancelled: readonly TimelineFrame[] };

  it("publishes a monotone run of non-idle frames while a long evaluation is in flight, then settles", () => {
    for (const lane of ["evaluation", "cancelled"] as const) {
      const frames = timeline[lane];
      let nonIdle = 0;
      let previousRatio = Number.NEGATIVE_INFINITY;
      for (const frame of frames) {
        const status = world3dComputeStatusV1(frame.statusJson);
        expect(status.phase, `${lane}/${frame.id}`).toBe(frame.expected.phase);
        expect(status.ratio, `${lane}/${frame.id}`).toBeCloseTo(frame.expected.ratio, 9);
        expect(isVisible(status), `${lane}/${frame.id}`).toBe(frame.expected.visible);
        if (!frame.expected.visible) continue;
        expect(progressText(status), `${lane}/${frame.id}`).toBe(frame.expected.progressText);
        if (status.phase === "cancelled") continue;
        expect(status.phase, `${lane}/${frame.id}: a frame that annotates the viewport may not call itself idle`).not.toBe("idle");
        expect(status.ratio, `${lane}/${frame.id}: ratio went backwards`).toBeGreaterThanOrEqual(previousRatio - 1e-9);
        expect(status.cancellable, `${lane}/${frame.id}: work in flight offers a cancel`).toBe(true);
        previousRatio = status.ratio;
        nonIdle += 1;
      }
      expect(isVisible(world3dComputeStatusV1(frames[frames.length - 1].statusJson)), `${lane}: the timeline ends settled`).toBe(false);
      if (lane === "evaluation") expect(nonIdle, `${lane}: non-idle frames`).toBeGreaterThanOrEqual(timeline.minimumNonIdleFrames);
    }
    expect(
      timeline.cancelled.some((frame) => frame.expected.phase === "cancelled"),
      "the cancelled lane settles on `cancelled`, not on `idle`",
    ).toBe(true);
  });
});
//#endregion ⏳️ComputeStatusPaneTwin

//#region 🔌️PortTypeTwin
/** 🔌️ The renderer's half of the port-type law. The oracle is
 * `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧫️fixtures/🔌️port-types/🔣️.json`; the Rust half
 * (`🌊️flow/🧪️tests/🔌️port-types/🦀️.rs`) checks those declared types against the LIVE extension
 * registry and drives a real `FlowHost`, and this half checks that the renderer's own predicate —
 * the one React Flow asks before it paints a snap target as droppable — answers the same rows.
 * Neither implementation owns the verdicts, so a drift on either side fails on both. */
describe("node-graph port types", () => {
  type Channel = { readonly ref: string; readonly side: string; readonly valueTypes: readonly string[] };
  type Row = { readonly source: string; readonly target: string; readonly compatible: boolean; readonly examples?: readonly string[] };
  const fixture = portTypesFixture as unknown as { readonly channels: readonly Channel[]; readonly rows: readonly Row[] };
  const declared = new Map(fixture.channels.map((channel) => [channel.ref, channel.valueTypes] as const));
  const typesOf = (reference: string) =>
    declared.get(reference) ??
    (() => {
      throw new Error(`fixture declares no channel ${reference}`);
    })();

  it("accepts or refuses every fixture pair exactly as the fixture says", () => {
    expect(fixture.rows.length).toBeGreaterThanOrEqual(30);
    expect(fixture.rows.some((row) => row.compatible)).toBe(true);
    expect(fixture.rows.some((row) => !row.compatible)).toBe(true);
    for (const row of fixture.rows) {
      expect([row.source, row.target, portValueTypesCompatible(typesOf(row.source), typesOf(row.target))]).toEqual([row.source, row.target, row.compatible]);
    }
  });

  it("refuses the drag the defect accepted, on the node records a surface actually holds", () => {
    const records = [
      { id: "extrusion-axis", x: 0, y: 0, width: 80, height: 40, inputs: [], outputs: [{ id: "extrusion-axis@vectorOut", label: "V", valueType: typesOf("math.vector@vectorOut").join(",") }] },
      { id: "profile", x: 0, y: 100, width: 80, height: 40, inputs: [], outputs: [{ id: "profile@wire", label: "W", valueType: typesOf("brep.curve.polygon@wire").join(",") }] },
      {
        id: "extrude",
        x: 200,
        y: 0,
        width: 80,
        height: 40,
        inputs: [
          { id: "extrude@wire", label: "W", valueType: typesOf("brep.solid.extrude@wire").join(",") },
          { id: "extrude@vector", label: "V", valueType: typesOf("brep.solid.extrude@vector").join(",") },
        ],
        outputs: [],
      },
    ];
    expect(nodeGraphConnectionIsValid(records, { source: "extrusion-axis", sourceHandle: "vectorOut", target: "extrude", targetHandle: "wire" })).toBe(false);
    expect(nodeGraphConnectionIsValid(records, { source: "extrusion-axis", sourceHandle: "vectorOut", target: "extrude", targetHandle: "vector" })).toBe(true);
    expect(nodeGraphConnectionIsValid(records, { source: "profile", sourceHandle: "wire", target: "extrude", targetHandle: "wire" })).toBe(true);
  });

  it("leaves an undeclared port connectable", () => {
    expect(portValueTypes(undefined)).toEqual([]);
    expect(portValueTypes({ id: "x", valueType: "" })).toEqual([]);
    expect(portValueTypes({ id: "x", valueType: "point,vector" })).toEqual(["point", "vector"]);
    expect(portValueTypesCompatible([], ["geometry"])).toBe(true);
    expect(portValueTypesCompatible(["geometry"], [])).toBe(true);
  });

  it("names both ports and both declared types in the refusal a surface shows", () => {
    const refusal = parseDagWireTypeRefusalJson(JSON.stringify({ refusal: { source: "extrusion-axis@vectorOut", sourceTypes: ["vector"], target: "extrude@wire", targetTypes: ["geometry"] } }));
    expect(refusal).not.toBeNull();
    expect(parseDagWireTypeRefusalJson("null")).toBeNull();
    expect(parseDagWireTypeRefusalJson('{"widgetId":"extrude","port":"wire","direction":"in"}')).toBeNull();
    const options = wireRefusalLabelOptions(refusal, { vector: "Vektor", geometry: "Geometrie" });
    expect(options).toEqual({ source: "extrusion-axis@vectorOut", sourceType: "Vektor", target: "extrude@wire", targetType: "Geometrie" });
  });
});
//#endregion 🔌️PortTypeTwin

//#region 🎫️RemainingReds
/** 🎫️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane `react-remaining-reds`. Four surfaces of the shell
 * that a user reads or clicks and that published nothing — or published over each other — measured on
 * the generation3d React serve (:6018) and closed on the layer that owns each one. */
describe("🎫️ the shell says what it holds", () => {
  it("Escape on a graph surface clears the FRAMEWORK selection, not a deleted media-graph verb", async () => {
    const { readFileSync } = await import("node:fs");
    const { dirname, resolve } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../..");
    const nodeGraph = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx"), "utf8");
    const manifest = readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"), "utf8");
    const { nodeGraphActions } = await import("../../../../../../../🔨️modules/🖱️ui/🎬️scene/🟦️.ts");

    const reserved = /export const CLEAR_SELECTION_ACTION_ID = "([^"]+)";/u.exec(manifest)?.[1];
    expect(reserved, "the framework declares the reserved clear verb").toBe("clearSelection");
    expect(nodeGraphActions.clearSelection, "and the graph scene's action table names that very verb").toBe(reserved);

    const handler = nodeGraph.slice(nodeGraph.indexOf("function handleGraphKeyboard"));
    const body = handler.slice(0, handler.indexOf("\n}"));
    expect(body, "Escape must dispatch the reserved clear verb").toContain("dispatch(nodeGraphActions.clearSelection)");
    expect(nodeGraph, "`setMediaNodeSelection` was deleted with the first-class selection mechanism — no window kind declares it, so every press was dropped").not.toMatch(/dispatch\(\s*"setMediaNodeSelection"/u);
  });

  it("a graph surface publishes the selection it paints, in the same shape the 3D pane does", async () => {
    const { nodeGraphSurfaceSelectionDomV1 } = await import("../../🧱️elements/🕸️NodeGraph/🟦️.tsx");
    expect(nodeGraphSurfaceSelectionDomV1(undefined)).toEqual({ selectedIds: [], highlightedIds: [], hoverTarget: null, editable: true });
    const picked = nodeGraphSurfaceSelectionDomV1({
      nodes: [],
      edges: [],
      editable: true,
      selection: ["height"],
      highlighted: ["height@number"],
      hover: { nodeId: "profile", portId: "radius" },
    });
    expect(picked.selectedIds, "the node the Artifact panel's outline row picked").toEqual(["height"]);
    expect(picked.highlightedIds).toEqual(["height@number"]);
    expect(picked.hoverTarget).toEqual({ nodeId: "profile", portId: "radius" });
    const hoverOnly = nodeGraphSurfaceSelectionDomV1({ nodes: [], edges: [], hover: { nodeId: "extrude" } });
    expect(hoverOnly.hoverTarget).toEqual({ nodeId: "extrude", portId: null });
    expect(hoverOnly.selectedIds, "an unhovered graph publishes an EMPTY lane, never a missing one").toEqual([]);
  });

  it("the shell publishes the framework history cursor undo and redo are decided by, in the locale it is showing", async () => {
    const { shellHistoryCursorDomV1, SHELL_HISTORY_DOM_LABELS } = await import("../../🧱️elements/🛠️ShellHelpers/🟦️.tsx");
    // 🌐️ A history row carries the whole locale matrix, exactly as Rust `LocalizedLabel` serialises
    // it — the projection resolves it, so the SAME ledger publishes German labels for a German shell
    // with no re-read of the guest's history. Ticket 26/09/18 slice U3.
    const bilingual = (en: string, de: string) => ({ native: { en, de }, reuse: { en, de } }) as const;
    const entry = (seq: number, en: string, de: string) => [seq, { seq, label: bilingual(en, de), actionId: `action-${seq}`, kind: "app", timestamp: "0" }] as const;
    const english = { terminology: "native", locale: "en" } as const;
    const german = { terminology: "native", locale: "de" } as const;

    const empty = shellHistoryCursorDomV1({ cursor: 0, entries: {}, canUndo: false, canRedo: false }, english);
    expect(empty).toEqual({ cursor: 0, canUndo: false, canRedo: false, entries: 0, currentCheckpointId: null, labels: [], actionIds: [], undoLabel: null, redoLabel: null });

    const entries = Object.fromEntries([entry(1, "Add Widget", "Widget hinzufügen"), entry(2, "Move Widget", "Widget verschieben"), entry(3, "Delete Selection", "Auswahl löschen")]);
    const midway = shellHistoryCursorDomV1({ cursor: 2, entries, canUndo: true, canRedo: true, currentCheckpointId: "check-1" }, english);
    expect(midway.cursor).toBe(2);
    expect(midway.undoLabel, "undo reverts the newest entry at or below the cursor").toBe("Move Widget");
    expect(midway.redoLabel, "redo re-applies the first entry above it").toBe("Delete Selection");
    expect(midway.labels).toEqual(["Add Widget", "Move Widget", "Delete Selection"]);
    expect(midway.currentCheckpointId).toBe("check-1");

    const auf_deutsch = shellHistoryCursorDomV1({ cursor: 2, entries, canUndo: true, canRedo: true, currentCheckpointId: "check-1" }, german);
    expect(auf_deutsch.undoLabel, "the same ledger, resolved for a German shell").toBe("Widget verschieben");
    expect(auf_deutsch.redoLabel).toBe("Auswahl löschen");
    expect(auf_deutsch.labels, "no English leaks into a German projection").toEqual(["Widget hinzufügen", "Widget verschieben", "Auswahl löschen"]);
    expect(auf_deutsch.actionIds, "ids are locale-invariant").toEqual(midway.actionIds);

    const bounded = shellHistoryCursorDomV1({ cursor: 40, canUndo: true, canRedo: false, entries: Object.fromEntries(Array.from({ length: 40 }, (_, index) => entry(index + 1, `Step ${index + 1}`, `Schritt ${index + 1}`))) }, english);
    expect((bounded.labels as readonly string[]).length, "a DOM attribute carries a bounded tail, never the whole log").toBe(SHELL_HISTORY_DOM_LABELS);
    expect((bounded.labels as readonly string[])[SHELL_HISTORY_DOM_LABELS - 1]).toBe("Step 40");
    expect(bounded.redoLabel, "nothing above the cursor to redo").toBeNull();
  });

  it("a cleared leftover overlay carries no ids, so an outline row it named goes idle on the clearing turn", async () => {
    const { leftoverOverlayCarryingSelectionV1, leftoverTreeItemSelectedV1, leftoverWorldOverlayAppliesV1, mergeWorldSelectionWithLeftoverV1 } = await import("../../🧱️elements/🌐️World3dHost/🟦️.tsx");
    const prior = { ids: ["extrusion-axis"], hoveredId: null, gumballActive: true, gumballAnchorId: "extrusion-axis", activeUtility: "select" };
    /** 🧹️ The publication generation3d's `clearSelection` really sends, measured on 6018:
     * `selectionCleared: true` alongside the ids it has just retired. */
    const cleared = leftoverOverlayCarryingSelectionV1({ ids: ["extrusion-axis"], hoveredId: null, gumballActive: false, gumballAnchorId: null, selectionCleared: true }, prior, true);
    expect(cleared.selectionCleared).toBe(true);
    expect(cleared.ids, "a clear is an ABSENCE — it cannot carry the ids it retired").toEqual([]);
    expect(cleared.gumballActive).toBe(false);
    expect(leftoverWorldOverlayAppliesV1(cleared), "and it still applies, or the one publication that REMOVES a selection would be the one nothing acts on").toBe(true);
    expect(leftoverTreeItemSelectedV1("panel:procedural-play-graph/extrusion-axis", cleared.ids), "the outline row the clear retired reads idle").toBe(false);
    expect(leftoverTreeItemSelectedV1("panel:procedural-play-graph/extrusion-axis", prior.ids), "fails-before: the same row read selected off the overlay's retained ids").toBe(true);
    expect(mergeWorldSelectionWithLeftoverV1({ ids: ["extrusion-axis"], activeObjectId: "extrusion-axis", gumballActive: true }, cleared).ids).toEqual([]);

    const kept = leftoverOverlayCarryingSelectionV1({ ids: ["height"], hoveredId: null, gumballActive: false, gumballAnchorId: null }, prior);
    expect(kept.ids, "an ordinary pick is untouched").toEqual(["height"]);
  });

  it("an open chrome-hosted dock lays its fold control BESIDE the tab strip, never under it", async () => {
    const { chromeHostedPanelCapRowStyle, uiSpacingPx } = await import("@semio-tech/ui-react");
    /** 📐️ The measured 6018 geometry (`🐍️remaining-reds-recon.mjs` → `🗑️generated/react-reds/recon/recon.json`):
     * a 300 px dock body, a 238 px tab strip, a 64 px `Collapse` control and a 92 px navbar trailing-end
     * reserve — 394 px of content for 300 px of body. */
    const body = 300,
      strip = 238,
      controls = 64,
      reserve = 92;
    const style = chromeHostedPanelCapRowStyle("top-right", reserve);
    expect(style.paddingInlineStart, "the navbar's trailing end is still cleared").toBe(`${reserve + uiSpacingPx(1)}px`);
    expect(style.width, "the cap row sizes to its content").toBe("max-content");
    expect(style.minWidth, "and never shrinks below the body it caps").toBe("100%");

    /** 📐️ One inline axis, laid out right-to-left from the dock's own right edge, the way the panel root's
     * `dir="rtl"` lays a right-anchored cap out. Before the fix the row was pinned to `body`, so the strip
     * was allotted `body - controls - reserve` and painted its full width anyway — over the controls. */
    const rowWidth = Math.max(body, reserve + strip + controls);
    const stripBox = { start: rowWidth - reserve - strip, end: rowWidth - reserve };
    const controlsBox = { start: rowWidth - reserve - strip - controls, end: rowWidth - reserve - strip };
    expect(controlsBox.end, "the fold control ends exactly where the strip begins").toBe(stripBox.start);
    expect(Math.max(stripBox.start, controlsBox.start) < Math.min(stripBox.end, controlsBox.end), "no overlap").toBe(false);

    const pinnedStripBox = { start: body - reserve - strip, end: body - reserve };
    const pinnedControlsBox = { start: 0, end: controls };
    expect(Math.max(pinnedStripBox.start, pinnedControlsBox.start) < Math.min(pinnedStripBox.end, pinnedControlsBox.end), "fails-before: pinned to the body, the strip painted over the fold control").toBe(true);
  });

  it("the shared World3d lighting fixture builds React's actual light and standard-material values", async () => {
    
    

    const { sunPositionFromAzimuthElevation } = await import("@semio-tech/ui-react");
    const request = world3dLightingFixture.iconRenderRequest;
    const environment = world3dLightingFixture.worldEnvironment;
    const ambient = new THREE.AmbientLight(request.lights.ambientColor, request.lights.ambientIntensity);
    const sun = new THREE.DirectionalLight(request.lights.sunColor, request.lights.sunIntensity);
    sun.position.fromArray(sunPositionFromAzimuthElevation(request.lights.sunAzimuth, request.lights.sunElevation));
    const material = new THREE.MeshStandardMaterial({
      color: request.material.color,
      metalness: request.material.metalness,
      roughness: request.material.roughness,
      emissive: request.material.emissive,
      emissiveIntensity: request.material.emissiveIntensity,
    });

    expect(ambient.intensity).toBe(environment.ambient.intensity);
    expect(ambient.color.getHexString()).toBe(environment.ambient.color.slice(1));
    expect(sun.intensity).toBe(environment.sun.intensity);
    expect(sun.color.getHexString()).toBe(environment.sun.color.slice(1));
    expect(sun.position.clone().normalize().toArray()).toEqual(
      [
        Math.cos(THREE.MathUtils.degToRad(environment.sun.elevation)) * Math.cos(THREE.MathUtils.degToRad(environment.sun.azimuth)),
        Math.cos(THREE.MathUtils.degToRad(environment.sun.elevation)) * Math.sin(THREE.MathUtils.degToRad(environment.sun.azimuth)),
        Math.sin(THREE.MathUtils.degToRad(environment.sun.elevation)),
      ].map((value) => expect.closeTo(value, 12)),
    );
    expect(material.metalness).toBe(environment.material.metalness);
    expect(material.roughness).toBe(environment.material.roughness);
    expect(material.emissive.getHexString()).toBe(environment.material.emissive.slice(1));
    expect(material.emissiveIntensity).toBe(environment.material.emissiveIntensity);
    expect(
      material.emissive
        .clone()
        .multiplyScalar(material.emissiveIntensity)
        .toArray()
        .some((channel) => channel > 0),
      "the oracle's neutral emissive contributes radiance",
    ).toBe(true);
  });

  it("the shared Tree drag fixture matches React's handle and surface drivers", () => {
    for (const row of treeDragHandleFixture.rows) {
      const dragData = "dragData" in row ? row.dragData : undefined;
      const roles = deriveTreeDragRoles({ draggable: row.draggable, dragData }, false);
      expect(roles).toEqual(row.role ? [row.role] : []);
      // 🧭️ Annotated, not inferred: the two fixture rows differ in which id lists are empty, so the
      // imported JSON types one `labelStarts`/`visibleHandles` as `never[]` and `.includes(id)` refuses.
      const driverCases: readonly { readonly id: string; readonly drag: string; readonly visibleHandles: readonly string[]; readonly labelStarts: readonly string[]; readonly handleStarts: readonly string[] }[] = treeDragHandleFixture.drivers;
      for (const driverCase of driverCases) {
        const driver = driverCase.drag === "surface" ? COMPACT_UI_DRIVER : DEFAULT_UI_DRIVER;
        const markup = renderToStaticMarkup(
          createElement(
            UiDriverProvider,
            { driver },
            createElement(
              TreeContext.Provider,
              { value: { level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 } },
              createElement(TreeItem, {
                id: row.id,
                label: row.label,
                draggable: row.draggable,
                dragRoles: roles,
                dragInitiation: "handle",
                dragData,
              }),
            ),
          ),
        );
        const visible = driverCase.visibleHandles.includes(row.id);
        const labelStarts = driverCase.labelStarts.includes(row.id);
        expect(markup.includes('data-slot="drag-handle"')).toBe(visible);
        expect(/data-slot="tree-item-row"[^>]*\sdraggable="true"/.test(markup), `${driverCase.id}:${row.id}`).toBe(labelStarts);
        if (visible) {
          expect(markup).toContain(`data-drag-role="${row.role}"`);
        }
      }
    }
  });

  it("the shared scene-list transfer fixture matches React data transfer and dnd-kit geometry", async () => {
    const fixture = sceneListTransferFixture;
    
    
    await uiI18n.changeLanguage("en");

    const transferValues = new Map<string, string>();
    const transferTypes: string[] = [];
    const dataTransfer = {
      effectAllowed: "uninitialized",
      dropEffect: "none",
      types: transferTypes,
      setData(type: string, value: string) {
        if (!transferValues.has(type)) transferTypes.push(type);
        transferValues.set(type, value);
      },
      getData(type: string) {
        return transferValues.get(type) ?? "";
      },
      clearData(type?: string) {
        if (type === undefined) {
          transferValues.clear();
          transferTypes.splice(0);
        } else {
          transferValues.delete(type);
          const index = transferTypes.indexOf(type);
          if (index >= 0) transferTypes.splice(index, 1);
        }
      },
      files: [],
      items: [],
      setDragImage() {},
    } as unknown as DataTransfer;
    const tableSource = fixture.table.source;
    const sourceNode = {
      surfaceId: tableSource.surfaceId,
      controllerId: "controller.table-a",
      table: {
        columnsJson: JSON.stringify([{ id: "label", label: "Label" }]),
        rowsJson: JSON.stringify([{ id: tableSource.rowId, label: "Asset 7", _drag: tableSource.payload }]),
        rowDragMime: tableSource.mime,
      },
    };
    const destinationNode = {
      surfaceId: fixture.table.destination.surfaceId,
      controllerId: fixture.table.destination.dropAction.controllerId,
      table: {
        columnsJson: JSON.stringify([{ id: "label", label: "Label" }]),
        rowsJson: JSON.stringify([]),
        dropActionJson: JSON.stringify(fixture.table.destination.dropAction),
      },
    };
    const destinationAction = vi.fn();
    const sourceView = render(createElement(UiDriverProvider, { driver: DEFAULT_UI_DRIVER }, createElement(TableHost, { node: sourceNode as never, onAction: vi.fn() })));
    const destinationView = render(createElement(UiDriverProvider, { driver: DEFAULT_UI_DRIVER }, createElement(TableHost, { node: destinationNode as never, onAction: destinationAction })));
    try {
      const row = sourceView.container.querySelector("tbody tr") as HTMLTableRowElement;
      const handle = row.querySelector('[data-slot="drag-handle"]') as HTMLElement;
      expect(handle).not.toBeNull();
      expect(row.draggable, "Handle leaves the row inert before its handle arms").toBe(false);
      fireEvent.pointerDown(handle, { button: 0, isPrimary: true });
      expect(row.draggable, "the real native-drag arm makes the row draggable").toBe(true);
      fireEvent.dragStart(row, { dataTransfer });
      expect(dataTransfer.types).toEqual([tableSource.mime]);
      expect(JSON.parse(dataTransfer.getData(tableSource.mime))).toEqual(tableSource.payload);
      fireEvent.drop(destinationView.container.querySelector(".semio-table-host") as HTMLElement, { dataTransfer });
      expect(destinationAction).toHaveBeenCalledTimes(1);
      expect(destinationAction.mock.calls[0]![0]).toEqual(fixture.table.expectedAction);
    } finally {
      sourceView.unmount();
      destinationView.unmount();
    }

    const blockNode = {
      surfaceId: fixture.blockList.surfaceId,
      controllerId: "controller.block-list",
      blockList: {
        stepsJson: JSON.stringify(fixture.blockList.steps),
        paletteJson: JSON.stringify(fixture.blockList.palette),
      },
    };
    for (const driverCase of fixture.drivers) {
      const driver = driverCase.drag === "surface" ? COMPACT_UI_DRIVER : DEFAULT_UI_DRIVER;
      const actions = vi.fn();
      const view = render(createElement(UiDriverProvider, { driver }, createElement(BlockListHost, { node: blockNode as never, onAction: actions })));
      try {
        const expectedHandleCount = driverCase.drag === "handle" ? fixture.blockList.steps.length + fixture.blockList.steps.reduce((count, step) => count + step.blocks.length, 0) + fixture.blockList.palette.length : 0;
        expect(view.container.querySelectorAll('[data-slot="drag-handle"]')).toHaveLength(expectedHandleCount);
        const paletteRow = view.container.querySelector(".semio-palette > div") as HTMLDivElement;
        expect(paletteRow.draggable).toBe(driverCase.drag === "surface");
      } finally {
        view.unmount();
      }
    }

    const rect = (top: number) => ({ top, bottom: top + 20, left: 0, right: 100, width: 100, height: 20 });
    const collisionWinner = (ids: readonly string[], targetIndex: number) => {
      const droppableRects = new Map(ids.map((id, index) => [id, rect(index * 40)]));
      const droppableContainers = ids.map((id) => ({ id, disabled: false, data: { current: {} }, node: { current: null }, rect: { current: null } }));
      return closestCenter({ collisionRect: rect(targetIndex * 40), droppableRects, droppableContainers } as never)[0]?.id;
    };
    expect(
      collisionWinner(
        fixture.blockList.steps.map((step) => step.id),
        1,
      ),
    ).toBe("publish");
    expect(
      collisionWinner(
        fixture.blockList.steps[0]!.blocks.map((block) => block.id),
        1,
      ),
    ).toBe("clean");
    expect(fixture.journeys.find((journey) => journey.id === "step-prepare-after-publish")?.expectedAction).toEqual({ controllerId: "controller.block-list", action: "moveStep", args: { stepId: "prepare", index: 1 } });
    expect(fixture.journeys.find((journey) => journey.id === "block-load-after-clean")?.expectedAction).toEqual({
      controllerId: "controller.block-list",
      action: "moveBlock",
      args: { blockId: "load", fromStepId: "prepare", toStepId: "prepare", index: 1 },
    });
  });

  it("the shared shadow fixture matches Three's directional camera and IconRender caster contract", async () => {
    
    

    const { sunPositionFromAzimuthElevation } = await import("@semio-tech/ui-react");
    const environment = world3dShadowFixture.worldEnvironment;
    const sun = new THREE.DirectionalLight(environment.sun.color, environment.sun.intensity);
    sun.position.fromArray(sunPositionFromAzimuthElevation(environment.sun.azimuth, environment.sun.elevation));
    sun.castShadow = environment.shadow.enabled;
    sun.shadow.mapSize.set(world3dShadowFixture.iconRender.mapSize, world3dShadowFixture.iconRender.mapSize);
    sun.updateMatrixWorld();
    sun.target.updateMatrixWorld();
    sun.shadow.updateMatrices(sun);

    expect(sun.castShadow).toBe(true);
    expect(sun.shadow.mapSize.toArray()).toEqual([1024, 1024]);
    expect(sun.shadow.matrix.elements).toEqual(world3dShadowFixture.oracle.shadowMatrix.map((value) => expect.closeTo(value, 12)));
    expect([sun.shadow.camera.left, sun.shadow.camera.right, sun.shadow.camera.top, sun.shadow.camera.bottom, sun.shadow.camera.near, sun.shadow.camera.far]).toEqual([-5, 5, 5, -5, 0.5, 500]);

    const subject = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshStandardMaterial());
    subject.castShadow = environment.shadow.enabled && world3dShadowFixture.iconRender.materialPresent;
    subject.receiveShadow = environment.shadow.enabled && world3dShadowFixture.iconRender.materialPresent;
    expect([subject.castShadow, subject.receiveShadow]).toEqual([true, true]);
  });

  it("the neutral exact-shadow corpus matches React roles and current Three PCF", () => {
    const fixture = world3dShadowParityFixture;
    
    
    const worldSun = new THREE.DirectionalLight();
    expect(worldSun.shadow.mapSize.toArray()).toEqual([fixture.profiles.world.mapSize, fixture.profiles.world.mapSize]);
    expect(worldSun.shadow.bias).toBe(fixture.profiles.world.bias);
    expect(worldSun.shadow.normalBias).toBe(fixture.profiles.world.normalBias);
    expect(worldSun.shadow.radius).toBe(fixture.profiles.world.radius);
    expect(worldSun.shadow.intensity).toBe(fixture.profiles.world.intensity);
    expect(worldSun.shadow.intensity).not.toBe(fixture.profiles.world.opacityInput);
    expect(worldSun.shadow.radius).not.toBe(fixture.profiles.world.softnessInput);
    expect(fixture.profiles.world.consumesOpacity).toBe(false);
    expect(fixture.profiles.world.consumesSoftness).toBe(false);

    const iconSun = new THREE.DirectionalLight();
    iconSun.shadow.mapSize.set(fixture.profiles.iconPng.mapSize, fixture.profiles.iconPng.mapSize);
    expect(iconSun.shadow.mapSize.toArray()).toEqual([1024, 1024]);
    expect(fixture.profiles.iconPng.usesShadowMap).toBe(true);
    expect(fixture.profiles.iconSvg.usesShadowMap).toBe(false);

    for (const row of fixture.roles) {
      const mesh = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshStandardMaterial({ transparent: row.transparent }));
      if (row.kind === "glb") {
        mesh.castShadow = true;
        mesh.receiveShadow = true;
      } else if (row.kind === "terrain") {
        mesh.receiveShadow = true;
      } else if (row.kind === "icon" && row.materialPresent) {
        mesh.castShadow = true;
        mesh.receiveShadow = true;
      }
      expect([mesh.castShadow, mesh.receiveShadow], row.kind).toEqual([row.casts, row.receives]);
    }

    const cameraRow = fixture.frusta.mainCamera;
    const camera = new THREE.PerspectiveCamera(cameraRow.fov, cameraRow.aspect, cameraRow.near, cameraRow.far);
    camera.position.fromArray(cameraRow.position);
    camera.up.fromArray(cameraRow.up);
    camera.lookAt(new THREE.Vector3().fromArray(cameraRow.target));
    camera.updateProjectionMatrix();
    camera.updateMatrixWorld();
    const mainFrustum = new THREE.Frustum().setFromProjectionMatrix(new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse));

    const sunRow = fixture.frusta.sun;
    const azimuth = THREE.MathUtils.degToRad(sunRow.azimuth);
    const elevation = THREE.MathUtils.degToRad(sunRow.elevation);
    const shadowSun = new THREE.DirectionalLight();
    shadowSun.position.set(Math.cos(elevation) * Math.cos(azimuth) * sunRow.distance, Math.cos(elevation) * Math.sin(azimuth) * sunRow.distance, Math.sin(elevation) * sunRow.distance);
    shadowSun.updateMatrixWorld();
    shadowSun.target.updateMatrixWorld();
    shadowSun.shadow.updateMatrices(shadowSun);
    const lightFrustum = shadowSun.shadow.getFrustum();
    const box = (bounds: { readonly min: readonly number[]; readonly max: readonly number[] }) => new THREE.Box3(new THREE.Vector3().fromArray(bounds.min), new THREE.Vector3().fromArray(bounds.max));
    const caster = box(fixture.frusta.offscreenCasterBounds);
    const receiver = box(fixture.frusta.visibleReceiverBounds);
    expect(mainFrustum.intersectsBox(caster)).toBe(fixture.frusta.offscreenCasterMainVisible);
    expect(lightFrustum.intersectsBox(caster)).toBe(fixture.frusta.offscreenCasterLightVisible);
    expect(mainFrustum.intersectsBox(receiver)).toBe(fixture.frusta.visibleReceiverMainVisible);
    expect(lightFrustum.intersectsBox(receiver)).toBe(fixture.frusta.visibleReceiverLightVisible);

    const pcfSource = THREE.ShaderChunk.shadowmap_pars_fragment;
    expect(pcfSource).toContain("interleavedGradientNoise( gl_FragCoord.xy )");
    expect(pcfSource).toContain("vogelDiskSample( 4, 5, phi )");
    expect(pcfSource).toContain("shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0");
    expect(pcfSource).toContain("shadowCoord.z <= 1.0");
    const fract = (value: number) => value - Math.floor(value);
    const [fragmentX, fragmentY] = fixture.pcf.fragmentPosition;
    const noise = fract(52.9829189 * fract(fragmentX * 0.06711056 + fragmentY * 0.00583715));
    const rotation = noise * 6.28318530718;
    const offsets = Array.from({ length: fixture.pcf.sampleCount }, (_, index) => {
      const radius = Math.sqrt((index + 0.5) / fixture.pcf.sampleCount);
      const theta = index * 2.399963229728653 + rotation;
      return [Math.cos(theta) * radius, Math.sin(theta) * radius];
    });
    expect(noise).toBeCloseTo(fixture.pcf.interleavedGradientNoise, 12);
    expect(rotation).toBeCloseTo(fixture.pcf.rotation, 12);
    expect(offsets).toEqual(fixture.pcf.unitOffsets.map((row) => row.map((value) => expect.closeTo(value, 12))));
  });
});
//#endregion 🎫️RemainingReds

it("the neutral gizmo head corpus matches Three sprite scale and circle bounds", () => {
  const camera = new THREE.PerspectiveCamera();
  camera.position.fromArray(gizmoTipBoundsFixture.camera.position);
  camera.up.fromArray(gizmoTipBoundsFixture.camera.up);
  camera.lookAt(new THREE.Vector3().fromArray(gizmoTipBoundsFixture.camera.target));
  camera.updateMatrixWorld(true);
  const viewRotation = new THREE.Matrix4().extractRotation(camera.matrixWorldInverse);
  const axes = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
  ];
  for (const [index, axis] of axes.entries()) {
    const point = new THREE.Vector3().fromArray(axis).applyMatrix4(viewRotation);
    expect([point.x * gizmoTipBoundsFixture.groupScale, -point.y * gizmoTipBoundsFixture.groupScale, point.z]).toEqual(gizmoTipBoundsFixture.axisOffsets[index].map((value) => expect.closeTo(value, 6)));
  }
  for (const row of gizmoTipBoundsFixture.cases) {
    const sprite = new THREE.Sprite();
    const group = new THREE.Group();
    group.scale.setScalar(gizmoTipBoundsFixture.groupScale);
    sprite.scale.setScalar((row.prominent ? 1 : 0.65) * gizmoTipBoundsFixture.axisHeadScale * (row.hovered ? 1.1 : 1));
    group.add(sprite);
    group.updateMatrixWorld(true);
    const radius = (sprite.getWorldScale(new THREE.Vector3()).x * (row.prominent ? 16 : 12)) / gizmoTipBoundsFixture.textureSize;
    expect(radius).toBeCloseTo(row.radius, 8);
    const geometry = new THREE.CircleGeometry(radius, 64);
    geometry.computeBoundingBox();
    const size = geometry.boundingBox!.getSize(new THREE.Vector3());
    expect(size.x).toBeCloseTo(row.radius * 2, 5);
    expect(size.y).toBeCloseTo(row.radius * 2, 5);
    expect(size.x).toBeLessThanOrEqual(gizmoTipBoundsFixture.maxHeadDiameter + 0.00001);
    geometry.dispose();
    sprite.material.dispose();
  }
});

describe("shared VFS descriptor presentation", () => {
  for (const law of vfsDescriptorFixture.cases)
    it(`renders the actual React VFS descriptor ${law.id}`, () => {
      const kind = vfsDescriptorFixture.kinds[law.kind as keyof typeof vfsDescriptorFixture.kinds] as DescriptorKind;
      const value = law.value === null ? undefined : (law.value as FileNodeDescriptorValue);
      const view = render(createElement("div", null, renderVirtualFileSystemDescriptorCell(kind, value, "en-US")));
      try {
        expect(view.container.textContent).toBe(law.expect.text);
        if (value?.presentation === "avatar" && kind.presentation === "avatar") {
          expect(view.container.querySelector('[data-slot="avatar-fallback"]')?.getAttribute("aria-label")).toBe(law.expect.name);
          expect(view.container.querySelector('[data-slot="avatar"]')?.classList.contains("rounded-full")).toBe(true);
          if (value.icon) expect(view.container.querySelector("img")?.getAttribute("src")).toBe(law.expect.icon);
        }
      } finally {
        view.unmount();
      }
    });
});

describe("📣️ replay refusal vocabulary", () => {
  it("names every reason in both tongues from the shared vocabulary the wgpu shell reads", () => {
    expect(Object.keys(REPLAY_REFUSAL_LABELS_V1).sort()).toEqual(Object.keys(replayRefusalVocabulary.reasons).sort());
    for (const label of Object.values(REPLAY_REFUSAL_LABELS_V1)) expect(label.en !== "" && label.de !== "" && label.en !== label.de).toBe(true);
  });

  it("answers the shared vectors the wgpu shell's law answers", () => {
    expect(replayRefusalVocabulary.vectors.length).toBeGreaterThan(0);
    for (const vector of replayRefusalVocabulary.vectors) {
      const reason = vector.reason as ReplayRefusalReasonV1;
      expect([replayRefusalNoticeTextV1(reason, vector.locale), replayRefusalCodeV1(reason)]).toEqual([vector.text, vector.code]);
    }
  });
});
