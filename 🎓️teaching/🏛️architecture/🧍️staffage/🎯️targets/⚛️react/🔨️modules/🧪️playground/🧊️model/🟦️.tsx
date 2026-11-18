/** 🧊️ The 3D playground: every placed asset in one neutral 3D scene, driven by the playground's own clock.
 *
 * Each placement's model is prepared by the model pipeline of the 3D view (`prepareModel`, one scale, its pose or clip; kept per asset object, so a new revision of an asset is a new model, never a stale one), then the parts are composed into one document (`composePlayground`): positions and turns from the shared scene, every clip laid on the master animation `timeline` with the window, speed, loop and hold of its placement.
 * The shared clock only seeks that animation; time is never the viewer's own. A change of scale, placements or timeline prepares and composes again after a short settle, with progress and a cancel button; a change of representation or camera only switches the scene, the variant or the preset of the loaded document.
 * Picking a part selects its placement. Assets without a model at the scale are named, never replaced by another scale.
 *
 * Loaded on demand through `../../🧊️spatial/🚪️gate/🟦️.tsx`, so the viewer, the pipeline and the kernel stay out of the entry chunk.
 *
 * @see ./🧩️compose/🟦️.ts — the composition
 * @see ./🎞️retime/🟦️.ts — the clips on the timeline
 * @see ../../🧊️spatial/🏭️prepare/🟦️.ts — the model pipeline
 */
import { useEffect, useMemo, useRef, useState, type ReactElement } from "react";
import { authoredScales, modeOf, sceneIdOf } from "@teaching/staffage";
import type { Asset, EvaluationQuery, Language, View } from "@teaching/staffage";
import { ModelViewer, type ViewerStatus } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🟦️.ts";
import type { StaffageClock } from "../../⏱️clock/🧱️core/🟦️.ts";
import { translator, type Key } from "../../🌐️i18n/🟦️.ts";
import type { Placement } from "../../💾️persistence/🟦️.ts";
import { prepareModel } from "../../🧊️spatial/🏭️prepare/🟦️.ts";
import { CAMERA_PRESETS, variantOf, type CameraPresetName } from "../../🧊️spatial/🟦️.ts";
import { Bar, Button, Notice, ProgressNotice } from "../../🪟️chrome/🟦️.tsx";
import { clipOf, passS, playable, windowEndS } from "../🎞️timeline/🟦️.ts";
import { composePlayground, placementOf, TIMELINE_ANIMATION, type Floor, type Part } from "./🧩️compose/🟦️.ts";

/** 🎯️ The query of the model of a placement: its pose, its clip (its first instant is the static state), else the asset's default pose. */
export function modelQueryOf(asset: Asset, pick: Placement["pick"]): EvaluationQuery {
  const chosen = playable(asset, pick);
  if (chosen?.kind === "pose") return { kind: "pose", pose: chosen.pose };
  const clip = clipOf(asset, chosen);
  return clip === undefined ? { kind: "pose", pose: asset.presentation.defaultPose } : { kind: "clip", clip: clip.id, timeS: 0 };
}

/** 🟫️ The floor under the placements: their footprints and a margin, at least 4 m by 4 m, a grid line every metre. */
export function floorOf(placements: readonly Placement[], assets: ReadonlyMap<string, Asset>): Floor {
  const margin = 1.5;
  const boxes = placements.map((placement) => {
    const asset = assets.get(placement.asset);
    const reach = Math.max(asset?.dimensions.widthM ?? 0.5, asset?.dimensions.depthM ?? 0.5) / 2;
    return { minX: placement.xM - reach, maxX: placement.xM + reach, minZ: placement.zM - reach, maxZ: placement.zM + reach };
  });
  const minX = Math.min(...boxes.map((box) => box.minX), 0) - margin;
  const maxX = Math.max(...boxes.map((box) => box.maxX), 0) + margin;
  const minZ = Math.min(...boxes.map((box) => box.minZ), 0) - margin;
  const maxZ = Math.max(...boxes.map((box) => box.maxZ), 0) + margin;
  const grow = (low: number, high: number): readonly [number, number] => (high - low >= 4 ? [low, high] : [(low + high) / 2 - 2, (low + high) / 2 + 2]);
  const [lowX, highX] = grow(minX, maxX);
  const [lowZ, highZ] = grow(minZ, maxZ);
  return { minXM: Math.floor(lowX), maxXM: Math.ceil(highX), minZM: Math.floor(lowZ), maxZM: Math.ceil(highZ), stepM: 1 };
}

/** 🧾️ What {@link ModelPlayground} shows. */
export type ModelPlaygroundProps = {
  readonly placements: readonly Placement[];
  readonly assets: ReadonlyMap<string, Asset>;
  readonly scale: number;
  readonly view: View;
  readonly camera: CameraPresetName;
  readonly surface: string;
  readonly language: Language;
  readonly clock: StaffageClock;
  readonly timelineS: number;
  readonly onSelect?: (id: string) => void;
  readonly onCamera?: (camera: CameraPresetName) => void;
  readonly reducedMotion?: boolean;
  readonly settleMs?: number;
  readonly className?: string;
};

type Stage = { readonly phase: "idle" | "preparing" | "ready" | "cancelled" | "error"; readonly fraction: number; readonly message: string };

let serial = 0;
const serials = new WeakMap<Asset, number>();
const serialOf = (asset: Asset): number => {
  let known = serials.get(asset);
  if (known === undefined) serials.set(asset, (known = ++serial));
  return known;
};

const isAbort = (error: unknown): boolean => (error instanceof DOMException || error instanceof Error) && error.name === "AbortError";
const WEIGHTS = { prepare: 0.8, compose: 0.05 } as const;

/** 🧊️ The 3D playground. */
export function ModelPlayground(props: ModelPlaygroundProps): ReactElement {
  const { placements, assets, scale, view, camera, surface, language, clock, timelineS, settleMs = 200 } = props;
  const t = useMemo(() => translator(language), [language]);
  const canvas = useRef<HTMLCanvasElement | null>(null);
  const viewer = useRef<ModelViewer | null>(null);
  const latest = useRef(props);
  latest.current = props;
  const [stage, setStage] = useState<Stage>({ phase: "idle", fraction: 0, message: "" });
  const [attempt, setAttempt] = useState(0);
  const job = useRef<AbortController | null>(null);
  const loaded = useRef("");
  const order = useRef<readonly string[]>([]);
  const drawn = useMemo(() => placements.filter((placement) => {
    const asset = assets.get(placement.asset);
    return asset !== undefined && authoredScales(asset.volumes).includes(scale);
  }), [placements, assets, scale]);
  const skipped = useMemo(() => placements.filter((placement) => !drawn.includes(placement) && assets.has(placement.asset)), [placements, drawn, assets]);
  const key = JSON.stringify([scale, timelineS, drawn.map((placement) => [placement.id, serialOf(assets.get(placement.asset)!), placement.xM, placement.zM, placement.yawDeg, placement.pick, placement.offsetS, placement.startS, placement.durationS, placement.speed, placement.loop, placement.holdS])]);
  useEffect(() => {
    const element = canvas.current;
    if (element === null) return;
    const status = (s: ViewerStatus): string => {
      switch (s.kind) {
        case "ready": return t("model.status.ready");
        case "preset": return t("model.status.preset", { camera: t(`model.camera.${s.preset}` as Key) });
        case "fit": return t("model.status.fit");
        case "orbit": return t("model.status.orbit");
        case "pan": return t("model.status.pan");
        case "zoom": return t("model.status.zoom");
        default: return "";
      }
    };
    const created = new ModelViewer({ canvas: element, strings: { label: t("playground.model.label", { count: latest.current.placements.length }), roleDescription: t("model.roleDescription"), description: t("playground.model.instructions"), status }, fallback: true, background: null, autoplay: false, ...(latest.current.reducedMotion === undefined ? {} : { reducedMotion: latest.current.reducedMotion }), preset: latest.current.camera });
    viewer.current = created;
    created.on("pick", ({ hit }) => {
      if (hit === null) return;
      const id = placementOf(hit.node, order.current);
      if (id !== null) latest.current.onSelect?.(id);
    });
    created.on("preset", ({ preset }) => {
      if (preset !== null && preset !== latest.current.camera) latest.current.onCamera?.(preset as CameraPresetName);
    });
    return () => {
      created.dispose();
      viewer.current = null;
      loaded.current = "";
    };
  }, [t]);
  const apply = (): void => {
    const current = viewer.current;
    if (current === null || loaded.current === "") return;
    const wanted = latest.current;
    current.setVariant(variantOf(wanted.view));
    current.setScene(sceneIdOf(wanted.scale, modeOf(wanted.view)));
    current.setAnimation(TIMELINE_ANIMATION, wanted.clock.state.frame / wanted.clock.state.fps);
    current.setLoop(false);
    current.setFollow("off");
    current.setPreset(wanted.camera);
  };
  useEffect(() => {
    if (canvas.current === null) return;
    canvas.current.style.setProperty("--semio-viewer-background", surface);
    viewer.current?.requestRender();
  }, [surface]);
  useEffect(() => {
    if (loaded.current !== "" && viewer.current !== null) apply();
  }, [view, camera]);
  useEffect(() => {
    const seek = (): void => viewer.current?.seek(clock.state.frame / clock.state.fps);
    seek();
    return clock.subscribe(seek);
  }, [clock]);
  useEffect(() => {
    job.current?.abort();
    job.current = null;
    if (drawn.length === 0) {
      loaded.current = "";
      setStage({ phase: "idle", fraction: 0, message: "" });
      return;
    }
    const timer = setTimeout(() => {
      const controller = new AbortController();
      job.current = controller;
      void (async () => {
        setStage({ phase: "preparing", fraction: 0, message: t("model.phase.model") });
        try {
          const parts: Part[] = [];
          for (const [index, placement] of drawn.entries()) {
            const asset = assets.get(placement.asset)!;
            const chosen = playable(asset, placement.pick);
            const query = modelQueryOf(asset, chosen);
            const document = await prepareModel(asset, { view, scale, language, query, scales: [scale] }, {
              signal: controller.signal,
              progress: (fraction, phase) => setStage({ phase: "preparing", fraction: ((index + fraction) / drawn.length) * WEIGHTS.prepare, message: t(`model.phase.${phase}` as Key) }),
            });
            const clip = clipOf(asset, chosen);
            parts.push({
              id: placement.id,
              document,
              xM: placement.xM,
              zM: placement.zM,
              yawDeg: placement.yawDeg,
              animation: query.kind === "clip" ? query.clip : null,
              pose: query.kind === "pose" ? query.pose : null,
              window: clip === undefined ? null : { startS: placement.startS, endS: windowEndS(placement, timelineS), speed: placement.speed, offsetS: placement.offsetS, loop: placement.loop, holdS: placement.holdS, clipS: clip.durationS },
              plantNode: asset.rig.root,
            });
          }
          controller.signal.throwIfAborted();
          setStage({ phase: "preparing", fraction: WEIGHTS.prepare, message: t("playground.model.composing") });
          order.current = parts.map((part) => part.id);
          const document = composePlayground(parts, { id: "playground", timelineS, floor: floorOf(drawn, assets) });
          const target = viewer.current;
          if (target === null) return;
          let ready = false;
          const stopProgress = target.on("progress", ({ fraction }) => setStage({ phase: "preparing", fraction: WEIGHTS.prepare + WEIGHTS.compose + (1 - WEIGHTS.prepare - WEIGHTS.compose) * fraction, message: t("model.phase.scene") }));
          const stopReady = target.on("ready", () => (ready = true));
          try {
            await target.setDocument(document, controller.signal);
          } finally {
            stopProgress();
            stopReady();
          }
          if (controller.signal.aborted || !ready) return;
          loaded.current = key;
          job.current = null;
          apply();
          setStage({ phase: "ready", fraction: 1, message: "" });
        } catch (error) {
          if (controller.signal.aborted && isAbort(controller.signal.reason ?? error)) return setStage({ phase: "cancelled", fraction: 0, message: "" });
          if (isAbort(error)) return setStage({ phase: "cancelled", fraction: 0, message: "" });
          setStage({ phase: "error", fraction: 0, message: error instanceof Error ? error.message : String(error) });
        }
      })();
    }, settleMs);
    return () => {
      clearTimeout(timer);
    };
  }, [key, attempt, t]);
  useEffect(() => () => job.current?.abort(), []);
  const cancel = (): void => {
    job.current?.abort();
    job.current = null;
    setStage({ phase: "cancelled", fraction: 0, message: "" });
  };
  const names = skipped.map((placement) => assets.get(placement.asset)?.title[language] ?? placement.asset);
  return (
    <div className={props.className} data-model-playground="" data-state={stage.phase} data-scale={scale} data-parts={drawn.length} lang={language} style={{ display: "flex", flexDirection: "column", gap: "0.5rem", blockSize: "100%", minBlockSize: 0 }}>
      <Bar below="tablet">
        {CAMERA_PRESETS.map((name) => (
          <Button key={name} grouped aria-pressed={camera === name} onClick={() => props.onCamera?.(name)} data-preset={name}>{t(`model.camera.${name}` as Key)}</Button>
        ))}
      </Bar>
      {stage.phase === "preparing" ? <ProgressNotice label={`${stage.message === "" ? t("model.progress") : stage.message}`} done={Math.round(stage.fraction * 100)} total={100} cancelLabel={t("model.cancel")} onCancel={cancel} /> : null}
      {stage.phase === "cancelled" ? <Notice kind="info" compact title={t("model.cancelled")} actions={<Button onClick={() => setAttempt((count) => count + 1)}>{t("model.retry")}</Button>} /> : null}
      {stage.phase === "error" ? <Notice kind="error" compact title={t("model.error")} actions={<Button onClick={() => setAttempt((count) => count + 1)}>{t("model.retry")}</Button>}>{t("model.errorBody", { message: stage.message })}</Notice> : null}
      {skipped.length > 0 ? <Notice kind="unavailable" compact title={t("playground.model.skipped", { count: skipped.length, scale })}>{names.join(", ")}</Notice> : null}
      {drawn.length === 0 && skipped.length === 0 ? <Notice kind="empty" compact title={t("playground.empty.title")}>{t("playground.model.empty")}</Notice> : null}
      <div style={{ position: "relative", flex: "1 1 auto", minBlockSize: "20rem", border: "1px solid currentColor" }}>
        <canvas ref={canvas} data-model-canvas="" style={{ display: "block", inlineSize: "100%", blockSize: "100%", minBlockSize: "20rem" }} />
      </div>
    </div>
  );
}
