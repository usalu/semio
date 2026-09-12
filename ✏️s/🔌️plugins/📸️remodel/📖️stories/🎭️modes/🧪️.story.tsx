// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/📸️remodel/📖️stories/🎭️modes/🧪️.story.tsx
// Specs: The three modes `create_remodeling_app` declares — `capture`, `model` (the default) and `analyze` —
// each shown through the window kind its layout puts in the main slot: `capture` → `remodeling-frames`
// (Canvas2d), `model` → `remodeling-main` (World3d, `default_layout`), `analyze` → `remodeling-report` (Table).
// Summary: One story per mode, each mounting the same host `🗣️Interpreter`'s `componentKind` switch would
// resolve for that window (`resolveComponentSceneHost`, `🗣️Interpreter/🟦️.tsx`), driven by the same populated
// document and the same story-local config reducer the per-window stories use. This is the mode-level view; the
// per-window stories (`📸️remodel🧊️Model` / `🖼️Frames` / `📊️Report`) carry the per-window toolbars and the
// window-specific caveats.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Canvas2dHost, TableHost, World3dHost } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import type { ActionDescriptor } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

import { REMODEL_DEFAULT_CONFIG, REMODEL_POPULATED_SCENE, remodelLabelsFor } from "../🧭️coordination/🧫️fixtures/🧫️model/🟦️.ts";
import { REMODEL_MODES, reduceRemodelStoryAction, remodelWindowNode } from "../🧭️coordination/🧫️fixtures/🧫️scene/🟦️.ts";

type RemodelModeId = (typeof REMODEL_MODES)[number]["id"];

//#region StoryHost
function RemodelModeStoryHost({ mode }: { readonly mode: RemodelModeId }): ReactElement {
  /** 🎞️ `capture` is the only mode whose window has anything to show without an asset store, and only on the frame that carries a GCP observation. */
  const [config, setConfig] = useState({ ...REMODEL_DEFAULT_CONFIG, frameCursor: mode === "capture" ? { streamId: "stream-b", frameIndex: 0 } : REMODEL_DEFAULT_CONFIG.frameCursor });
  const onAction = useCallback((descriptor: ActionDescriptor): void => setConfig((current) => reduceRemodelStoryAction(current, descriptor)), []);

  const windowKindId = REMODEL_MODES.find((entry) => entry.id === mode)?.windowKindId ?? "remodeling-main";
  const node = useMemo(() => remodelWindowNode(windowKindId, REMODEL_POPULATED_SCENE, config, windowKindId === "remodeling-frames"), [windowKindId, config]);
  const labels = remodelLabelsFor(config.locale);
  const debug = useMemo(() => JSON.stringify({ mode, modeLabel: mode === "capture" ? labels.capture : mode === "analyze" ? labels.analyze : labels.model, windowKindId, componentKind: node.componentKind }), [mode, labels, windowKindId, node]);

  /** 🧭️ The same `componentKind` → host resolution `resolveComponentSceneHost` performs, written out per branch (a union-typed component value is not callable in JSX). */
  const surface = node.componentKind === "canvas-2d" ? <Canvas2dHost node={node} onAction={onAction} /> : node.componentKind === "table" ? <TableHost node={node} onAction={onAction} /> : <World3dHost node={node} onAction={onAction} />;

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>{surface}</div>
      <pre data-testid={`remodel-mode-${mode}-debug`} style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "📸️remodel🕒️Modes",
  component: RemodelModeStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof RemodelModeStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 📷️ `capture` — its named layout puts `remodeling-frames` (Canvas2d) in the main slot; shown here through the story's host-shaped adapter so the `Corner` GCP observation is actually visible (see `📸️remodel🖼️Frames` for why the plugin's own payload is not). */
export const Capture: Story = { args: { mode: "capture" } };

/** 🧊️ `model` — the DEFAULT mode; its `default_layout` puts `remodeling-main` (World3d) in the main slot. */
export const Model: Story = { args: { mode: "model" } };

/** 🔍️ `analyze` — its named layout puts `remodeling-report` (Table) in the main slot, on the config's default `frames` dataset. */
export const Analyze: Story = { args: { mode: "analyze" } };
