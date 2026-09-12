// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/📸️remodel/📖️stories/🎭️viewer/🧪️.story.tsx
// Specs: Component-level coverage for remodel's `remodeling-view-model` window — the READ-ONLY viewer surface
// (`👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs`), the fourth window kind, also a `world-3d` host.
// Summary: Same `World3dHost` as the editor's `remodeling-main`, but projected the viewer's way: a hardcoded
// camera (`RemodelingViewer::Config = NoConfig`, so there is no persisted per-session camera), no selection
// overlay, and every point layer unconditionally visible — a viewer keeps no layer-visibility state. The story
// therefore has no layer toolbar, and its `onAction` is a recorder only: `RemodelingViewCommand` has a single
// inert `Noop` variant and `handle` always returns the empty `ViewEmit`, so nothing a viewer surface dispatches
// can change the document or the config. The debug readout proves the editor/viewer difference directly by
// showing the viewer's layer set beside the editor default's.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { World3dHost } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import type { ActionDescriptor } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

import { REMODEL_DEFAULT_CONFIG, REMODEL_EMPTY_SCENE, REMODEL_POPULATED_SCENE, type RemodelScene } from "../🧭️coordination/🧫️fixtures/🧫️model/🟦️.ts";
import { remodelViewerWorldScene, remodelWindowNode } from "../🧭️coordination/🧫️fixtures/🧫️scene/🟦️.ts";

//#region StoryHost
function RemodelViewerStoryHost({ document }: { readonly document: RemodelScene }): ReactElement {
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);
  /** 👁️ Recorder only — a viewer emits no config or artifact mutation by construction (`ViewEmit`). */
  const onAction = useCallback((descriptor: ActionDescriptor): void => setLastAction(descriptor), []);
  const node = useMemo(() => remodelWindowNode("remodeling-view-model", document, REMODEL_DEFAULT_CONFIG), [document]);
  const debug = useMemo(() => {
    const scene = remodelViewerWorldScene(document);
    return JSON.stringify({
      windowKindId: "remodeling-view-model",
      surfaceKind: "world-3d",
      controllerId: "remodeling-view",
      camera: JSON.parse(scene.cameraJson) as unknown,
      pointLayerIds: (JSON.parse(scene.pointsJson ?? "[]") as { readonly id: string }[]).map((layer) => layer.id),
      layerTogglesAvailable: false,
      lastAction,
      dispatchOutcome: "RemodelingViewCommand::Noop — always the empty ViewEmit",
    });
  }, [document, lastAction]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <World3dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="remodel-viewer-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "📸️remodel👁️Viewer",
  component: RemodelViewerStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof RemodelViewerStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 📸️ The populated fixture through the viewer's read-only projection — all four point layers, no toggles, the hardcoded `[4, -4, 3]` camera. */
export const PopulatedScene: Story = { args: { document: REMODEL_POPULATED_SCENE } };

/** 🌱️ `RemodelingViewer::initial_snapshot()` — the same `default_remodeling_scene()` the editor boots on: no point layers at all, only the (unresolvable) placeholder mesh instance. */
export const BootDocument: Story = { args: { document: REMODEL_EMPTY_SCENE } };
