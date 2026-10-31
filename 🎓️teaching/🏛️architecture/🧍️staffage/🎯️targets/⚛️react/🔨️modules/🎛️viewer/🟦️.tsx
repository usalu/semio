/** 🎛️ The viewer: one asset on a pannable stage with every control synchronized to one state — scale, view, pose or clip, time, background, zoom, analytic options and environment — plus inspector and legends.
 *
 * The whole state is a {@link ViewerState}: the page keeps it in the address, so a link reproduces the view, the instant and the analytic settings. Time is the state of one {@link StaffageClock}; the address follows it only while it is
 * paused or scrubbed, never per frame. Scale (paper) and zoom (screen) are separate controls. Analytic views follow the site theme unless the reader overrides it.
 *
 * @see ./🗺️stage/🟦️.tsx — pan and zoom
 * @see ../🗺️routing/🟦️.ts — `ViewerState` and its parameters
 */
import { useEffect, useMemo, useRef, useState, useSyncExternalStore, type ReactElement } from "react";
import { FORCE_QUANTITIES, SCALE_PRESETS, THERMAL_QUANTITIES, VIEWS, formatNumber, mmPerMetre } from "@teaching/staffage";
import type { Asset, DrawingScene, Environment, EvaluatedState, EvaluationQuery, ItemValue, StructuralResult, ThermalField, View } from "@teaching/staffage";
import { StaffageClock } from "../⏱️clock/🟦️.ts";
import { useArchive } from "../🧭️context/🟦️.tsx";
import { Button, CheckField, Notice, NumberField, Segmented, SelectField, SliderField, Tag, cn } from "../🪟️chrome/🟦️.tsx";
import { useDarkSurface } from "../🖼️previews/🟦️.tsx";
import { Transport } from "../⏯️transport/🟦️.tsx";
import { depictionOptions, evaluateState, frameOf, sceneOf, type SceneFrame } from "../🧮️pipeline/🟦️.ts";
import type { Selection } from "../💾️persistence/🟦️.ts";
import type { ViewerState } from "../🗺️routing/🟦️.ts";
import { VIEW_NAMES } from "../🗣️labels/🟦️.ts";
import { namedPoses } from "../🔎️search/🟦️.ts";
import { Stage } from "./🗺️stage/🟦️.tsx";
import { Inspector, Legends, type Inspected } from "./🧾️reading/🟦️.tsx";

/** 🧾️ What the page needs back from the viewer: the evaluated instant with its analyses and the effective environment and clock. */
export type ViewerFrame = { readonly state: EvaluatedState; readonly thermal: ThermalField; readonly structure: StructuralResult; readonly scene: DrawingScene; readonly environment: Environment; readonly clock: StaffageClock };

/** ⚙️ The props of {@link Viewer}. */
export type ViewerProps = { readonly asset: Asset; readonly viewer: ViewerState; readonly onChange: (patch: Partial<ViewerState>, replace?: boolean) => void; readonly onFrame?: (frame: ViewerFrame) => void; readonly interactive?: boolean };

const ANALYTIC_VIEWS: readonly View[] = ["thermal", "forces", "moments"];
const DARK_VIEWS: readonly View[] = ["dark-line", "dark-solid"];
const selectionKey = (selection: Selection | null): string => (selection === null ? "default" : selection.kind === "pose" ? `pose:${selection.pose}` : `clip:${selection.clip}`);
const keyToSelection = (key: string): Selection | null => (key === "default" ? null : key.startsWith("pose:") ? { kind: "pose", pose: key.slice(5) } : { kind: "clip", clip: key.slice(5) });

/** 🕰️ The clip whose time the clock follows. */
function clipOf(asset: Asset, selection: Selection | null) {
  const id = selection?.kind === "clip" ? selection.clip : asset.presentation.defaultClip;
  return asset.clips.find((clip) => clip.id === id) ?? asset.clips[0]!;
}

/** 🎛️ The viewer. */
export function Viewer(props: ViewerProps): ReactElement {
  const { asset, viewer, onChange } = props;
  const { t, language, reducedMotion } = useArchive();
  const dark = useDarkSurface();
  const [scene, setScene] = useState<DrawingScene | null>(null);
  const [inspected, setInspected] = useState<Inspected | null>(null);
  const clip = clipOf(asset, viewer.selection);
  const isPose = viewer.selection?.kind === "pose";
  const clipKey = `${asset.id}:${clip.id}`;
  const clock = useMemo(() => new StaffageClock({ fps: Math.min(clip.fps, 30), durationS: clip.durationS, loop: clip.loop && viewer.loop, rate: viewer.rate, playing: false, reducedMotion: true }), [clipKey]);
  const initial = useRef(viewer.timeS);
  useEffect(() => {
    if (initial.current !== null) {
      clock.seek(initial.current);
      clock.pause();
    } else if (!reducedMotion) clock.play();
    return () => clock.dispose();
  }, [clock]);
  useEffect(() => {
    if (reducedMotion) clock.pause();
  }, [reducedMotion, clock]);
  useEffect(() => {
    clock.setRate(viewer.rate);
    clock.setLoop(clip.loop && viewer.loop);
  }, [clock, viewer.rate, viewer.loop]);
  const playing = useSyncExternalStore(clock.subscribe, () => clock.state.playing, () => false);
  useEffect(() => {
    if (playing && viewer.timeS !== null) onChange({ timeS: null }, true);
  }, [playing]);
  const poseId = viewer.selection?.kind === "pose" ? viewer.selection.pose : null;
  const query: EvaluationQuery = useMemo(() => (poseId === null ? { kind: "clip", clip: clip.id, timeS: 0 } : { kind: "pose", pose: poseId }), [poseId, clip.id]);
  const analytic = ANALYTIC_VIEWS.includes(viewer.view);
  const theme = viewer.theme === "auto" ? (dark ? "dark" : "light") : viewer.theme;
  const frames = useMemo(() => {
    try {
      const state = evaluateState(asset, query);
      const baseScene = sceneOf(state, depictionOptions({ view: "dark-line", scale: { denominator: viewer.scale }, language, theme, transparent: true }));
      const activeScene = viewer.view === "dark-line" ? baseScene : sceneOf(state, depictionOptions({ view: viewer.view, scale: { denominator: viewer.scale }, language, theme, transparent: true }));
      return { base: frameOf([baseScene]), byView: { [viewer.view]: frameOf([baseScene, activeScene]) } as Record<View, SceneFrame> };
    } catch {
      return undefined;
    }
  }, [asset, query, viewer.scale, viewer.view, language, theme]);
  const reasoned = viewer.environment;
  const base = asset.thermal.environment;
  const environment: Environment = useMemo(() => ({ ...base, ...reasoned }), [base, JSON.stringify(reasoned)]);
  const overridden = Object.keys(reasoned).length > 0;
  const bg = viewer.background === "auto" ? (DARK_VIEWS.includes(viewer.view) ? "dark" : analytic && theme === "dark" ? "dark" : "light") : viewer.background;
  const explicit = viewer.background !== "auto";
  const mismatch = explicit && viewer.background !== "transparent" && ((DARK_VIEWS.includes(viewer.view) && viewer.background !== "dark") || (viewer.view.startsWith("light") && viewer.background === "dark"));
  const latest = useRef<ViewerFrame | null>(null);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const fitZoom = useRef(1);
  const hasValues = analytic || (scene?.legends.length ?? 0) > 0;
  const emit = (): void => {
    timer.current = null;
    const current = latest.current;
    if (current === null) return;
    setScene((previous) => (previous !== null && previous.metadata.view === current.scene.metadata.view && JSON.stringify(previous.legends) === JSON.stringify(current.scene.legends) ? previous : current.scene));
    props.onFrame?.(current);
  };
  useEffect(
    () => () => {
      if (timer.current !== null) clearTimeout(timer.current);
      timer.current = null;
    },
    [],
  );
  const onEvaluated = (frame: { state: EvaluatedState; thermal: ThermalField; structure: StructuralResult; scene: DrawingScene }): void => {
    latest.current = {
      get state() {
        return frame.state;
      },
      get thermal() {
        return frame.thermal;
      },
      get structure() {
        return frame.structure;
      },
      scene: frame.scene,
      environment,
      clock,
    };
    if (scene === null || scene.metadata.view !== frame.scene.metadata.view) setScene(frame.scene);
    if (timer.current === null) timer.current = setTimeout(emit, 400);
  };
  const zoomText = viewer.zoom === "fit" ? t("viewer.zoomFit") : `${Math.round(viewer.zoom * 100)} %`;
  const printSize = viewer.zoom === 1;
  const reference100 = asset.dimensions.heightM * mmPerMetre({ denominator: viewer.scale });
  const customScale = !(SCALE_PRESETS as readonly number[]).includes(viewer.scale);
  const poseOptions = [{ value: "default", label: t("viewer.selection.default", { title: clip.title[language] }) }, ...asset.clips.map((candidate) => ({ value: `clip:${candidate.id}`, label: `${t("viewer.selection.clip")}: ${candidate.title[language]}` })), ...namedPoses(asset).map((pose) => ({ value: `pose:${pose.id}`, label: `${t("viewer.selection.pose")}: ${pose.title[language]}` }))];
  const patch = (next: Partial<ViewerState>, replace = true): void => onChange(next, replace);
  const setEnv = (key: keyof Environment, value: number): void => patch({ environment: { ...viewer.environment, [key]: value } });
  return (
    <div className="grid min-w-0 gap-double lg:grid-cols-[minmax(0,1fr)_21rem]" data-viewer="">
      <div className="flex min-w-0 flex-col gap-double">
        <div className="flex flex-wrap items-center gap-single text-xs">
          <Tag>{`1:${formatNumber(viewer.scale, Number.isInteger(viewer.scale) ? 0 : 1, language)}`}</Tag>
          <Tag>{VIEW_NAMES[viewer.view][language]}</Tag>
          <span className="text-muted-foreground">{t("viewer.onPaper", { height: formatNumber(reference100, 1, language), mm: "mm", metres: formatNumber(asset.dimensions.heightM, 2, language) })}</span>
          <span className="ml-auto flex flex-wrap items-center gap-single">
            <Button icon="minus" aria-label={t("viewer.zoomOut")} title={t("viewer.zoomOut")} onClick={() => patch({ zoom: Math.max(0.05, Number(((viewer.zoom === "fit" ? fitZoom.current : viewer.zoom) / 1.25).toFixed(3))) })} />
            <output className="min-w-16 text-center font-mono" aria-label={t("viewer.zoom")}>{zoomText}</output>
            <Button icon="plus" aria-label={t("viewer.zoomIn")} title={t("viewer.zoomIn")} onClick={() => patch({ zoom: Math.min(64, Number(((viewer.zoom === "fit" ? fitZoom.current : viewer.zoom) * 1.25).toFixed(3))) })} />
            <Button aria-pressed={viewer.zoom === "fit"} onClick={() => patch({ zoom: "fit", pan: { xMm: 0, yMm: 0 } })} icon="maximize-2">{t("viewer.fit")}</Button>
            <Button aria-pressed={printSize} onClick={() => patch({ zoom: printSize ? "fit" : 1 })} icon="scaling" title={t("viewer.printSizeHelp")}>{t("viewer.printSize")}</Button>
            <Button icon="crosshair" aria-label={t("viewer.centre")} title={t("viewer.centre")} onClick={() => patch({ pan: { xMm: 0, yMm: 0 } })} />
          </span>
        </div>
        <Stage
          asset={asset}
          query={query}
          clock={clock}
          view={viewer.view}
          scale={viewer.scale}
          language={language}
          theme={theme}
          ground={bg}
          transparent={explicit}
          frame={frames?.byView[viewer.view]}
          anchor={frames?.base}
          zoom={viewer.zoom}
          pan={viewer.pan}
          onZoom={(zoom) => patch({ zoom })}
          onPan={(pan) => patch({ pan })}
          onFit={(value) => void (fitZoom.current = value)}
          label={t("viewer.stage", { title: asset.title[language] })}
          help={t("viewer.stageHelp")}
          thermalQuantity={viewer.thermalQuantity}
          forceQuantity={viewer.forceQuantity}
          range={viewer.range}
          patterns={viewer.patterns}
          diagrams={viewer.diagrams}
          environment={overridden ? environment : undefined}
          onEvaluated={onEvaluated}
          onInspect={(value: ItemValue | null, item) => setInspected(value === null ? null : { value, item })}
        />
        {mismatch ? <Notice kind="info" compact title={t("viewer.contrast")} /> : null}
        {isPose ? (
          <Notice kind="info" compact title={t("viewer.poseStatic", { title: (asset.poses.find((pose) => pose.id === (viewer.selection as { pose: string }).pose)?.title ?? { en: "", de: "" })[language] })} />
        ) : (
          <Transport clock={clock} label={t("viewer.transport")} onUserTime={(timeS) => patch({ timeS })} onRate={(rate) => patch({ rate })} onLoop={(loop) => patch({ loop })} />
        )}
        <Inspector inspected={inspected} language={language} hasValues={hasValues} asset={asset} />
        <Legends scene={scene} language={language} patterns={viewer.patterns} />
      </div>
      <form aria-label={t("viewer.controls")} onSubmit={(event) => event.preventDefault()} className="flex min-w-0 flex-col gap-double border border-border bg-panel p-double">
        <Segmented label={t("viewer.view")} value={viewer.view} onChange={(view) => patch({ view })} options={VIEWS.map((view) => ({ value: view, label: VIEW_NAMES[view][language], title: VIEW_NAMES[view][language] }))} />
        <div className="flex flex-col gap-single">
          <Segmented label={t("viewer.scale")} value={customScale ? -1 : viewer.scale} onChange={(scale) => scale > 0 && patch({ scale })} options={[...SCALE_PRESETS.map((scale) => ({ value: scale as number, label: `1:${scale}` })), { value: -1, label: t("viewer.scaleCustom"), title: t("viewer.scaleCustomHelp") }]} />
          <NumberField label={t("viewer.scaleDenominator")} unit={t("viewer.scaleUnit")} min={1} max={100000} value={viewer.scale} onChange={(scale) => scale !== null && scale >= 1 && patch({ scale })} />
        </div>
        <SelectField label={t("viewer.selection")} value={selectionKey(viewer.selection)} options={poseOptions} onChange={(key) => patch({ selection: keyToSelection(key), timeS: null, pan: viewer.pan })} />
        <Segmented label={t("viewer.background")} value={viewer.background} onChange={(background) => patch({ background })} options={[{ value: "auto", label: t("viewer.background.auto") }, { value: "dark", label: t("viewer.background.dark") }, { value: "light", label: t("viewer.background.light") }, { value: "paper", label: t("viewer.background.paper") }, { value: "transparent", label: t("viewer.background.transparent") }]} />
        <Segmented label={t("viewer.theme")} value={viewer.theme} onChange={(next) => patch({ theme: next })} options={[{ value: "auto", label: t("viewer.theme.auto") }, { value: "light", label: t("viewer.theme.light") }, { value: "dark", label: t("viewer.theme.dark") }]} />
        <fieldset className={cn("m-0 flex min-w-0 flex-col gap-double border border-border p-double", !analytic && "opacity-70")}>
          <legend className="px-single text-[0.65rem] uppercase tracking-wide text-muted-foreground">{t("viewer.analytics")}</legend>
          {!analytic ? <p className="m-0 text-[0.65rem] text-muted-foreground">{t("viewer.analyticsOnly")}</p> : null}
          <SelectField label={t("viewer.thermalQuantity")} value={viewer.thermalQuantity} disabled={viewer.view !== "thermal"} onChange={(thermalQuantity) => patch({ thermalQuantity })} options={THERMAL_QUANTITIES.map((quantity) => ({ value: quantity, label: t(`viewer.thermal.${quantity}`) }))} />
          <SelectField label={t("viewer.forceQuantity")} value={viewer.forceQuantity} disabled={viewer.view !== "forces"} onChange={(forceQuantity) => patch({ forceQuantity })} options={FORCE_QUANTITIES.map((quantity) => ({ value: quantity, label: t(`viewer.force.${quantity}`) }))} />
          <Segmented label={t("viewer.range")} value={viewer.range.mode} onChange={(mode) => patch({ range: mode === "auto" ? { mode: "auto" } : { mode: "fixed", min: scene?.legends[0]?.min ?? 0, max: scene?.legends[0]?.max ?? 1 } })} options={[{ value: "auto", label: t("viewer.range.auto") }, { value: "fixed", label: t("viewer.range.fixed") }]} />
          {viewer.range.mode === "fixed" ? (
            <div className="flex gap-double">
              <NumberField label={t("viewer.range.min")} value={viewer.range.min} onChange={(min) => min !== null && viewer.range.mode === "fixed" && min < viewer.range.max && patch({ range: { mode: "fixed", min, max: viewer.range.max } })} />
              <NumberField label={t("viewer.range.max")} value={viewer.range.max} onChange={(max) => max !== null && viewer.range.mode === "fixed" && max > viewer.range.min && patch({ range: { mode: "fixed", min: viewer.range.min, max } })} />
            </div>
          ) : null}
          <CheckField label={t("viewer.patterns")} checked={viewer.patterns} onChange={(patterns) => patch({ patterns })} />
          <fieldset className="m-0 flex flex-wrap gap-x-double border-0 p-0">
            <legend className="mb-single p-0 text-[0.65rem] text-muted-foreground">{t("viewer.diagrams")}</legend>
            {(["axial", "shear", "moment", "freeBody"] as const).map((key) => (
              <CheckField key={key} label={t(`viewer.diagram.${key}`)} checked={viewer.diagrams[key]} onChange={(checked) => patch({ diagrams: { ...viewer.diagrams, [key]: checked } })} />
            ))}
          </fieldset>
        </fieldset>
        <fieldset className="m-0 flex min-w-0 flex-col gap-double border border-border p-double">
          <legend className="px-single text-[0.65rem] uppercase tracking-wide text-muted-foreground">{t("viewer.environment")}</legend>
          <p className="m-0 text-[0.65rem] text-muted-foreground">{t(viewer.view === "thermal" ? "viewer.environmentHelp" : "viewer.environmentOnlyThermal")}</p>
          <SliderField label={t("viewer.env.air")} min={-30} max={45} step={0.5} value={environment.airTempC} text={`${formatNumber(environment.airTempC, 1, language)} °C`} onChange={(value) => setEnv("airTempC", value)} />
          <SliderField label={t("viewer.env.radiant")} min={-30} max={60} step={0.5} value={environment.meanRadiantTempC} text={`${formatNumber(environment.meanRadiantTempC, 1, language)} °C`} onChange={(value) => setEnv("meanRadiantTempC", value)} />
          <SliderField label={t("viewer.env.humidity")} min={0} max={100} step={1} value={Math.round(environment.relativeHumidity * 100)} text={`${Math.round(environment.relativeHumidity * 100)} %`} onChange={(value) => setEnv("relativeHumidity", value / 100)} />
          <SliderField label={t("viewer.env.wind")} min={0} max={15} step={0.1} value={environment.windMS} text={`${formatNumber(environment.windMS, 1, language)} m/s`} onChange={(value) => setEnv("windMS", value)} />
          <SliderField label={t("viewer.env.solar")} min={0} max={1000} step={10} value={environment.solarWM2} text={`${Math.round(environment.solarWM2)} W/m²`} onChange={(value) => setEnv("solarWM2", value)} />
          <Button icon="rotate-ccw" disabled={!overridden} onClick={() => patch({ environment: {} })}>{t("viewer.env.reset")}</Button>
        </fieldset>
      </form>
    </div>
  );
}
