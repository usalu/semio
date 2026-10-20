/** 🟠️ The handle template drop phase of useDirectoryLaneAndLayout. */
import { useCallback, useMemo, useRef } from "react";
import { type LocalizedLabel, createNamedLayout } from "@semio-tech/framework";
import { setNamedLayout } from "@semio-tech/framework/config/mutations";
import { decodeWorldProjectionTemplateId, worldProjectionSpecIconId, worldProjectionSpecLabel } from "@semio-tech/framework/infinite/world/r3f";
import { type IconName, insertWindowAtDropZone, type ModeCanvasDropTarget, type WindowTemplateDropPayload } from "@semio-tech/ui-react";
import { registerPendingWorldProjection } from "../../../🌐️World3dHost/🪟️window-view-store/🟦️.tsx";
import { captureCurrentFrameworkLayout, resolveFrameworkLayoutSeed } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { shellLabelTextV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { resolveAppLabel, resolveManifestLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { useNamedLayoutHost } from "../../../📌️ChromePanels/🟦️.tsx";
import { type DisplayHostApi } from "../../../📌️ChromePanels/🖥️display-panel/🟦️.tsx";
import { withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { useScopeState } from "../🔶️scope/🟦️.ts";
import type { useApplyNamedLayoutState } from "../🔸️apply-named-layout/🟦️.ts";

/** 🟡 Builds handleTemplateDrop, displayHostRef, displayLayoutAppId, displayUserLayouts of useDirectoryLaneAndLayout. */
export function useHandleTemplateDropState({ session, focusedSpawnedId, extraWindowCounterRef, appLabelsOverlay, uiTerminology, uiLocale, extraWindowInstancesRef, dispatch, refreshUi, reportRefreshFault, noteShellCommand, focusedApp, uiNamedLayouts, shellLayout, extraWindowInstances, applyNamedLayout, commitUiPreferenceRef }: Pick<ReturnType<typeof useScopeState>, "session" | "focusedSpawnedId" | "extraWindowCounterRef" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "extraWindowInstancesRef" | "dispatch" | "refreshUi" | "reportRefreshFault" | "noteShellCommand" | "focusedApp" | "uiNamedLayouts" | "shellLayout" | "extraWindowInstances" | "commitUiPreferenceRef"> & Pick<ReturnType<typeof useApplyNamedLayoutState>, "applyNamedLayout">) {
  const handleTemplateDrop = useCallback(
    (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => {
      if (!session) return;
      if (focusedSpawnedId !== null) {
        console.warn(`[os-shell] window template drop refused: ${focusedSpawnedId} is a spawned program and has no split lane yet (kind ${payload.windowKindId})`);
        return;
      }
      const kind = session.app.windowKinds.find((entry) => entry.id === payload.windowKindId);
      if (!kind) return;
      extraWindowCounterRef.current += 1;
      const instanceId = `${payload.windowKindId}-${extraWindowCounterRef.current}`;
      const projectionSpec = decodeWorldProjectionTemplateId(payload.templateId);
      if (projectionSpec) registerPendingWorldProjection(instanceId, projectionSpec);
      const title = projectionSpec ? worldProjectionSpecLabel(projectionSpec) : resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale));
      const nextExtraInstances = [...extraWindowInstancesRef.current, { id: instanceId, windowKindId: payload.windowKindId, title }];
      extraWindowInstancesRef.current = nextExtraInstances;
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: nextExtraInstances });
      if (projectionSpec) {
        dispatch({ type: "SET_WINDOW_TITLE", windowId: instanceId, title });
        dispatch({ type: "SET_WINDOW_ICON", windowId: instanceId, iconId: worldProjectionSpecIconId(projectionSpec) as IconName });
      }
      void refreshUi(session, { kind: "full" }, nextExtraInstances).catch((error: unknown) => reportRefreshFault(session.pluginId, error));
      dispatch({
        type: "SET_SHELL_LAYOUT",
        value: (current) => {
          const base =
            current ??
            resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout;
          return insertWindowAtDropZone(base, instanceId, target);
        },
      });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: instanceId });
      noteShellCommand("shell.windowSplit", shellLabelTextV1("ui.shellCommand.windowSplit"), { windowKindId: payload.windowKindId, instanceId });
    },
    [appLabelsOverlay, refreshUi, session, focusedSpawnedId, noteShellCommand, uiTerminology, uiLocale],
  );

  const displayHostRef = useRef<DisplayHostApi | null>(null);
  const displayLayoutAppId = focusedApp?.id ?? "framework-os";
  const displayUserLayouts = useMemo(
    () => Object.entries(uiNamedLayouts[displayLayoutAppId] ?? {}).sort(([left], [right]) => left.localeCompare(right)).map(([layoutId, saved]) => createNamedLayout(layoutId, saved.label, saved.layout, "user")),
    [displayLayoutAppId, uiNamedLayouts],
  );
  const displayHost = useNamedLayoutHost({
    appId: displayLayoutAppId,
    windowKinds: focusedApp?.windowKinds.map((kind) => ({ ...kind, label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale)) })) ?? [],
    builtinLayouts: focusedApp?.namedLayouts ?? [],
    currentLayout: captureCurrentFrameworkLayout(shellLayout, extraWindowInstances, session?.app.defaultLayout),
    onApplyLayout: applyNamedLayout,
    userLayouts: displayUserLayouts,
    onSaveUserLayout: (layoutId, label, layout) => commitUiPreferenceRef.current(setNamedLayout(displayLayoutAppId, layoutId, { label, layout })),
    onDeleteUserLayout: (layoutId) => commitUiPreferenceRef.current(setNamedLayout(displayLayoutAppId, layoutId, null)),
  });
  displayHostRef.current = displayHost;
  return { handleTemplateDrop, displayHostRef, displayHost };
}
