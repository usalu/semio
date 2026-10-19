// #region 🧲️Header
// 💻️ ✏️s/🧑‍💻dev/📸️remodel/📖️stories/🧭️coordination/🟦️.ts
// Specs: The `📸️remodel` scope's story coordination: the typed story model and labels and the scene, panel and report projections built on it — everything the `🎭️*` stories need that is not itself a story.
// Specs: Story-local, self-contained copies of the remodel plugin's own populated document fixture and of
// its editor config defaults + complete en/de label set — the data half of the `📸️remodel` scope's stories.
// Summary: `REMODEL_POPULATED_SCENE` is a verbatim transcription of the shared mutation-fixture document
// `🗿️artifacts/📸️remodeling/…/🧬️schema/🧬️mutations/*/🧪️tests/<toy case>/📸️snapshot/⬅️before/🔣️.json` (the shared
// toy scene every `toy`-role vector starts from — two streams, three assets with their durable leaves, two
// calibrated cameras, two GCPs in canonical id order, and finished sparse/dense/trajectory/tracks/geo/qc
// results; a reconstruction run is framework tool-run state, never document state). It is copied rather than
// imported because a `?raw`/`?json` import of a plugin-owned asset would couple this scope to that plugin
// directory's layout, which is actively being renamed. `REMODEL_EMPTY_SCENE` mirrors `default_remodeling_scene()`
// (`🗿️artifacts/📸️remodeling/🦀️.rs`) — everything empty, the placeholder mesh handle seeded — which is also what
// the shipped `📚️examples/🎬️demo` document contains (its `🗣️.dsl.semio` declares zero streams/gcps/cameras).
// `REMODEL_LABELS` transcribes every field of `app_labels! { RemodelingLabels }` (`✏️editor/🗣️terminology/🦀️.rs`)
// in both native locales, so a story renders the same strings the Rust panels would for `locale: "en-US"/"de-DE"`.
// ⚠️ The populated document now carries its `durableArtifacts` store, so every `assets[key]` handle in it
// resolves to real content — but no leaf exists for the composed `s.stdio.semio@v1/mesh` CHILD handle in
// `results.mesh.mesh`, which resolves to nothing, matching `resolve_bounded_remodeling_mesh`'s documented
// "unavailable content renders no mesh entity" behavior. Every mesh-shaped readout below is therefore honestly
// empty, exactly as the plugin's own render path would be for these documents.
//
// Specs: The projection + reducer half of the `📸️remodel` scope — turns the story-local document/config
// fixtures (`./🗂️documents`) into exactly the payloads remodel's own Rust render functions build, and folds the
// host-dispatched `ActionDescriptor`s back into config the way `RemodelingCommand::dispatch` does.
// Summary: One projection per window kind, mirroring its Rust source line for line —
//   `remodelMainWorldScene`   ⇔ `✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️.rs` (`remodeling-main`, World3d)
//   `remodelFramesCanvasScene`⇔ `✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs` (`remodeling-frames`, Canvas2d)
//   `remodelReportTableScene` ⇔ `✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs` (`remodeling-report`, Table)
//   `remodelViewerWorldScene` ⇔ `👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs` (`remodeling-view-model`, World3d)
// plus one `BuiltNode` projection per panel tab (`✏️editor/📌️panels/*/🦀️.rs`), rendered through the REAL
// `InterpretedUiNode`. The panels' Rust still emits the legacy `UiNode` (`ui_stack_vertical`/`ui_text`/
// `ui_import_drop_zone`); no TS adapter from that shape to the retained `BuiltNode` document exists, so these
// helpers do the Stack→`container(plain)` / Text→`text` translation themselves and note it — the strings
// themselves are formatted exactly as the Rust `format!` calls do, in whichever locale the labels came from.
// Reducers cover the config-only command subset the hosts can actually reach without a plugin runtime
// (`setCamera`/`setLayerVisibility`/`setFrameCursor`/`setReportTable`/`setActiveUtility`/`setLocale`);
// an unrecognized action is ignored, matching `command_from_action` returning `None`.
// Layout: this file keeps only the reducer; its children own the rest — `./📐️scene-model` (types + config defaults),
// `./🗂️documents` (populated + empty scene, data in `./🗂️documents/🔣️.json`), `./🗣️labels` (en/de sets, data in
// `./🗣️labels/🔣️.json`), `./🪟️window-scenes` (constants + per-window scenes) and `./🧩️panel-documents` (panel tabs).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { ActionDescriptor } from "@semio-tech/framework-renderer-react";
import type { RemodelConfig } from "./📐️scene-model/🟦️.ts";
import { remodelActionArgs } from "./🪟️window-scenes/🟦️.ts";

//#region 🔖️Reducer
/**
 * 🎮️ Story-local mirror of `command_from_action` → `RemodelingCommand::dispatch` for the config-only subset a
 * host surface can reach without a plugin runtime. Document-mutating commands (`addGcp`, `importFramePayload`,
 * the reconstruction tool run, …) are deliberately NOT emulated: they emit artifact mutations through the event-sourced
 * store, which no story owns. An unrecognized action is ignored, exactly as `command_from_action` returning
 * `None` leaves the dispatch a no-op.
 */
export function reduceRemodelStoryAction(config: RemodelConfig, descriptor: ActionDescriptor): RemodelConfig {
  const args = remodelActionArgs(descriptor.args);
  switch (descriptor.action) {
    case "setCamera": {
      const camera = args.camera as { readonly position?: readonly [number, number, number]; readonly target?: readonly [number, number, number]; readonly fov?: number } | undefined;
      if (!camera) return config;
      return { ...config, camera: { position: camera.position ?? config.camera.position, target: camera.target ?? config.camera.target, fov: camera.fov ?? config.camera.fov } };
    }
    case "setLayerVisibility": {
      const layer = String(args.layer ?? "");
      if (!(layer in config.layers)) return config;
      return { ...config, layers: { ...config.layers, [layer]: args.visible === undefined ? !config.layers[layer as keyof RemodelConfig["layers"]] : Boolean(args.visible) } };
    }
    case "setFrameCursor":
      return { ...config, frameCursor: { streamId: typeof args.streamId === "string" ? args.streamId : null, frameIndex: Number(args.frameIndex ?? 0) } };
    case "setReportTable":
      return { ...config, reportTable: String(args.table ?? config.reportTable) };
    case "setActiveUtility":
      return { ...config, activeUtilityId: String(args.utilityId ?? config.activeUtilityId) };
    case "setLocale":
      return { ...config, locale: String(args.locale ?? config.locale) };
    default:
      return config;
  }
}
//#endregion 🔖️Reducer
