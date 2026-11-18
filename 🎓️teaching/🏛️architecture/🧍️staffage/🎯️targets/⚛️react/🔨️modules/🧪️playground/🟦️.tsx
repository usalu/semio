/** 🧪️ The playgrounds: one shared scene of placed assets, seen in three playgrounds — the section (side or front elevation), the floor plan (top view) and the 3D model — each at its own address.
 *
 * The scene is one thing: placements with a position (x ahead, z sideways), a turn, a pose or clip and a window on one timeline, a scale, a representation and a ground. Switching playgrounds keeps all of it; the tab only chooses how it is looked at. One shared clock plays the timeline (play, pause, scrub, rate, loop); every placement can also be paused, scrubbed, looped, sped up and moved on the timeline on its own.
 * One pixel-per-millimetre factor serves every figure of a 2D playground, so relative scale is exact whatever drawing scale the line weights use; zoom is a separate screen factor (1 = print size on screen). The 3D playground is loaded on demand.
 * Edits are events on the local store (persisted, undoable in memory); the address carries a shareable scene (and the tab) that only the open page reads and only applies when the reader agrees. The pages follow the catalog: placements read the current asset of their row, so a new revision replaces a figure's geometry at once and a pose or clip it no longer has is named.
 * A little play, kept restrained: a dropped figure squashes and hops once, the ground takes a soft pat; reduced motion and the "playful" switch both silence it.
 *
 * @see ./🧱️compose/🟦️.ts — SVG composition
 * @see ./📐️geometry/🟦️.ts — positions of the 2D stages
 * @see ./🎞️timeline/🟦️.ts — the shared timeline rules
 * @see ./🖼️stage/🟦️.tsx — the 2D stage
 * @see ./🧊️model/🟦️.tsx — the 3D playground
 * @see ../📍️placed/🟦️.tsx — the figure on a ground line
 */
import { useCallback, useEffect, useMemo, useRef, useState, type ReactElement, type KeyboardEvent as ReactKeyboardEvent } from "react";
import { CSS_PX_PER_MM, SCALE_PRESETS, VIEWS, formatNumber, mmPerMetre } from "@teaching/staffage";
import type { Asset, PlanarViewpoint, View } from "@teaching/staffage";
import { StaffageClock } from "../⏱️clock/🟦️.ts";
import { useArchive, useLocal, useRows } from "../🧭️context/🟦️.tsx";
import { Bar, Button, ButtonGroup, ButtonLink, CheckField, Cell, Column, Note, Notice, NumberField, PageFrame, ProgressNotice, Segmented, SelectField, Tag, TextField, cn } from "../🪟️chrome/🟦️.tsx";
import { useSize, type PaneView } from "../🥞️levels/🟦️.tsx";
import { RowPreview } from "../🖼️previews/🟦️.tsx";
import { isViewpoint } from "../🔭️viewpoints/🟦️.ts";
import { ViewpointSelector } from "../🔭️viewpoints/🎚️selector/🟦️.tsx";
import { BACKGROUNDS, SECTION_VIEWPOINTS, TRACK_DEFAULTS, wrapYaw, type Background, type Placement, type Playground, type SectionViewpoint } from "../💾️persistence/🟦️.ts";
import { addressProblems, formatAddress, navigate, playgroundFromParams, playgroundToParams, useAddress, type PlaygroundTab, PLAYGROUND_TABS } from "../🗺️routing/🟦️.ts";
import { matches, NO_FILTERS, type Row } from "../🔎️search/🟦️.ts";
import { useAssetLoading } from "../📚️catalog/🟦️.ts";
import { SpatialPlayground } from "../🧊️spatial/🚪️gate/🟦️.tsx";
import { AssetPanel } from "./🎛️asset/🟦️.tsx";
import { TimelinePanel } from "./🎞️timeline/🖼️panel/🟦️.tsx";
import { clipDurationS, timelineLengthS } from "./🎞️timeline/🟦️.ts";
import { PlanarStage, type Reaction } from "./🖼️stage/🟦️.tsx";
import { snap } from "./🧱️compose/🟦️.ts";
import { extentM, fitZoomOf, stageGeometry, MIN_HEIGHT_M } from "./📐️geometry/🟦️.ts";
import { downloadText, exportComposition } from "./📤️export/🟦️.ts";
import { keyAction } from "./🧲️keys/🟦️.ts";
import { useHistory } from "./📜️history/🟦️.ts";

const DARK_VIEWS: readonly View[] = ["dark-line", "dark-solid"];

/** 🎨️ The ground the figures really stand on: "paper" follows the view, so light ink never lands on cream and dark ink never on night. */
export const effectiveBackground = (view: View, background: Background): Background => (background === "paper" ? (DARK_VIEWS.includes(view) ? "dark" : "paper") : background);

const SURFACE: Readonly<Record<Background, string>> = { paper: "bg-[#f4efe1] text-[#1b1b1b]", dark: "bg-[#0b1a1f] text-[#e8e4d4]", light: "bg-white text-[#1b1b1b]", transparent: "staffage-checker text-[#1b1b1b]" };
const SURFACE_COLOUR: Readonly<Record<Background, string>> = { paper: "#f4efe1", dark: "#0b1a1f", light: "#ffffff", transparent: "#ffffff" };

const newId = (): string => `p-${(globalThis.crypto?.randomUUID?.() ?? Math.random().toString(36).slice(2)).slice(0, 8)}`;
const clampM = (value: number, lengthM: number): number => Math.min(Math.max(value, 0), lengthM);
const sameComposition = (a: Playground, b: Playground): boolean => playgroundToParams(a).toString() === playgroundToParams(b).toString();
const TAB_ICONS = { section: "layout", plan: "layout-grid", model: "box" } as const;

function nextX(pg: Playground, rows: ReadonlyMap<string, Row>): number {
  const end = Math.max(1.5, ...pg.placements.map((placement) => placement.xM + (rows.get(placement.asset)?.asset?.dimensions.widthM ?? 0.6) + 0.5));
  return snap(end > pg.lengthM - 1 ? pg.lengthM / 2 : end);
}

/** 🔭️ The playgrounds page; the background copy is a cheap, inert preview of the local scene. */
export function PlaygroundPage(props: { readonly view: PaneView }): ReactElement {
  const { view } = props;
  const { t, language, store, ambient, reducedMotion, catalog } = useArchive();
  const { playground: pg, preferences } = useLocal();
  const snapshot = useRows();
  const { rows } = snapshot;
  const address = useAddress();
  const tab: PlaygroundTab = view.opened && address.route.page === "playground" ? (address.route.tab ?? "section") : "section";
  const viewpoint: PlanarViewpoint = tab === "plan" ? "plan" : pg.viewpoint;
  const [attach, size] = useSize({ width: 1000, height: 600 });
  const [selected, setSelected] = useState<string | null>(null);
  const [zoom, setZoom] = useState<"fit" | number>(() => {
    const value = Number(address.params.get("pz"));
    return view.opened && Number.isFinite(value) && value > 0 ? Math.min(value, 64) : "fit";
  });
  useEffect(() => {
    if (!view.opened) return;
    const params = new URLSearchParams(window.location.hash.split("?")[1] ?? "");
    const wanted = zoom === "fit" || tab === "model" ? null : String(Number(zoom.toFixed(3)));
    if (params.get("pz") === wanted) return;
    if (wanted === null) params.delete("pz");
    else params.set("pz", wanted);
    navigate({ page: "playground", tab }, params, true);
  }, [zoom, view.opened, tab]);
  const [reaction, setReaction] = useState<Reaction | null>(null);
  const [message, setMessage] = useState("");
  const [search, setSearch] = useState("");
  const [dismissed, setDismissed] = useState("");
  const [link, setLink] = useState<{ readonly href: string; readonly copied: boolean } | null>(null);
  const [job, setJob] = useState<{ readonly done: number; readonly total: number; readonly controller: AbortController } | null>(null);
  const [outcome, setOutcome] = useState<{ readonly kind: "done" | "cancelled" | "error"; readonly text: string } | null>(null);
  const playful = preferences.playful && !reducedMotion;
  const byId = useMemo(() => new Map(rows.map((row) => [row.entry.id, row])), [rows]);
  const assets = useMemo(() => new Map<string, Asset | undefined>(pg.placements.map((placement) => [placement.asset, byId.get(placement.asset)?.asset])), [pg.placements, byId]);
  const readyAssets = useMemo(() => new Map(pg.placements.flatMap((placement) => (assets.get(placement.asset) === undefined ? [] : [[placement.asset, assets.get(placement.asset)!] as const]))), [pg.placements, assets]);
  const categories = useMemo(() => [...new Set(pg.placements.flatMap((placement) => (byId.get(placement.asset) === undefined ? [] : [byId.get(placement.asset)!.entry.category])))], [pg.placements, byId]);
  useAssetLoading(catalog, true, categories, categories.join());
  const everything = useAssetLoading(catalog, view.opened, "all");
  const timelineS = useMemo(() => timelineLengthS(pg.placements.map((placement) => ({ ...placement, clipS: readyAssets.has(placement.asset) ? clipDurationS(readyAssets.get(placement.asset)!, placement.pick) : 0 })), pg.timelineS), [pg.placements, pg.timelineS, readyAssets]);
  const clock = useMemo(() => new StaffageClock({ fps: 30, durationS: 1, loop: true, playing: false, reducedMotion }), []);
  useEffect(() => clock.setDuration(timelineS), [clock, timelineS]);
  useEffect(() => {
    if (view.opened && !reducedMotion) clock.play();
    return () => clock.pause();
  }, [clock, view.opened, reducedMotion]);
  const current = useCallback(() => store.getSnapshot().playground, [store]);
  const history = useHistory(current, useCallback((next: Playground) => store.dispatch({ type: "playground-replaced", playground: next }), [store]));
  const say = (text: string): void => setMessage(text);
  const k = mmPerMetre({ denominator: pg.scale });
  const { acrossM } = extentM(viewpoint, pg);
  const zoomFit = fitZoomOf(size.width, acrossM, k, CSS_PX_PER_MM);
  const zoomValue = zoom === "fit" ? zoomFit : zoom;
  const pxPerMm = CSS_PX_PER_MM * zoomValue;
  const metrePx = k * pxPerMm;
  const tallestM = Math.max(MIN_HEIGHT_M, ...pg.placements.map((placement) => byId.get(placement.asset)?.asset?.dimensions.heightM ?? 0));
  const geometry = stageGeometry(viewpoint, pg, tallestM, metrePx);
  const sizes = { pxPerMm, metrePx, ...geometry };
  const surface = effectiveBackground(pg.view, pg.background);
  const mismatch = (DARK_VIEWS.includes(pg.view) && pg.background === "light") || (!DARK_VIEWS.includes(pg.view) && pg.background === "dark");
  const linked = view.opened && address.route.page === "playground" ? playgroundFromParams(address.params) : null;
  const offered = linked !== null && !sameComposition(linked, pg) && dismissed !== address.hash;
  const selectedPlacement = pg.placements.find((placement) => placement.id === selected) ?? null;
  const titleOf = (id: string): string => byId.get(id)?.entry.title[language] ?? id;
  const mirrorAllowed = tab === "plan" || (tab === "section" && pg.viewpoint === "elevation");
  const act = (event: Parameters<typeof store.dispatch>[0], text: string): void => {
    history.record();
    store.dispatch(event);
    say(text);
  };
  const config = (patch: Partial<Pick<Playground, "scale" | "view" | "viewpoint" | "camera" | "background" | "lengthM" | "depthM" | "timelineS">>): void => act({ type: "playground-configured", patch }, t("playground.said.configured"));
  const react = (id: string): void => setReaction((previous) => ({ id, token: (previous?.token ?? 0) + 1 }));
  const add = (row: Row): void => {
    const placement: Placement = { id: newId(), asset: row.entry.id, xM: nextX(pg, byId), pick: null, offsetS: 0, mirrored: false, zM: 0, yawDeg: 0, ...TRACK_DEFAULTS };
    act({ type: "placement-added", placement }, t("playground.said.added", { title: row.entry.title[language] }));
    setSelected(placement.id);
    react(placement.id);
  };
  const change = (id: string, patch: Partial<Omit<Placement, "id">>, text: string): void => act({ type: "placement-changed", id, patch }, text);
  const select = (step: -1 | 1, from: string): void => {
    const index = pg.placements.findIndex((placement) => placement.id === from);
    const target = pg.placements[(index + step + pg.placements.length) % pg.placements.length];
    if (target === undefined) return;
    setSelected(target.id);
    document.querySelector<HTMLElement>(`[data-placement="${target.id}"]`)?.focus();
  };
  const remove = (id: string): void => {
    const index = pg.placements.findIndex((placement) => placement.id === id);
    const after = pg.placements.filter((placement) => placement.id !== id);
    act({ type: "placement-removed", id }, t("playground.said.removed", { title: titleOf(pg.placements[index]?.asset ?? id) }));
    const neighbour = after[Math.min(index, after.length - 1)];
    setSelected(neighbour?.id ?? null);
    if (neighbour !== undefined) setTimeout(() => document.querySelector<HTMLElement>(`[data-placement="${neighbour.id}"]`)?.focus(), 0);
  };
  const duplicate = (id: string): void => {
    const newOne = newId();
    act({ type: "placement-duplicated", id, newId: newOne }, t("playground.said.duplicated", { title: titleOf(pg.placements.find((placement) => placement.id === id)?.asset ?? id) }));
    setSelected(newOne);
    react(newOne);
  };
  const onKey = (placement: Placement) => (event: ReactKeyboardEvent): void => {
    const action = keyAction(event);
    if (action === null) return;
    event.preventDefault();
    event.stopPropagation();
    if (action.kind === "move") {
      const xM = clampM(snap(placement.xM + action.deltaM, Math.abs(action.deltaM) < 0.05 ? 0.01 : 0.05), pg.lengthM);
      change(placement.id, { xM }, t("playground.said.moved", { title: titleOf(placement.asset), x: formatNumber(xM, 2, language) }));
    } else if (action.kind === "to") {
      const xM = action.end === "start" ? 0 : pg.lengthM;
      change(placement.id, { xM }, t("playground.said.moved", { title: titleOf(placement.asset), x: formatNumber(xM, 2, language) }));
    } else if (action.kind === "lateral") {
      const zM = Math.min(Math.max(snap(placement.zM + action.deltaM, Math.abs(action.deltaM) < 0.05 ? 0.01 : 0.05), -pg.depthM / 2), pg.depthM / 2);
      change(placement.id, { zM }, t("playground.said.lateral", { title: titleOf(placement.asset), z: formatNumber(zM, 2, language) }));
    } else if (action.kind === "yaw") {
      const yawDeg = wrapYaw(placement.yawDeg + action.deltaDeg);
      change(placement.id, { yawDeg }, t("playground.said.turned", { title: titleOf(placement.asset), yaw: formatNumber(yawDeg, 0, language) }));
    } else if (action.kind === "remove") remove(placement.id);
    else if (action.kind === "duplicate") duplicate(placement.id);
    else select(action.step, placement.id);
  };
  const onDrop = (placement: Placement, position: { readonly xM?: number; readonly zM?: number }): void => {
    const title = titleOf(placement.asset);
    change(placement.id, position, position.xM !== undefined && position.zM !== undefined ? t("playground.said.droppedPlan", { title, x: formatNumber(position.xM, 2, language), z: formatNumber(position.zM, 2, language) }) : position.xM !== undefined ? t("playground.said.dropped", { title, x: formatNumber(position.xM, 2, language) }) : t("playground.said.lateral", { title, z: formatNumber(position.zM ?? 0, 2, language) }));
    react(placement.id);
  };
  const copyLink = async (): Promise<void> => {
    const href = new URL(formatAddress({ page: "playground", tab }, playgroundToParams(pg)), window.location.href).href;
    try {
      await navigator.clipboard.writeText(href);
      setLink({ href, copied: true });
    } catch {
      setLink({ href, copied: false });
    }
  };
  const runExport = async (): Promise<void> => {
    const controller = new AbortController();
    setOutcome(null);
    setJob({ done: 0, total: pg.placements.length, controller });
    try {
      const svg = await exportComposition({ placements: pg.placements, assets: readyAssets, scale: pg.scale, view: pg.view, viewpoint, background: surface, lengthM: pg.lengthM, depthM: pg.depthM, language, timeS: clock.state.frame / clock.state.fps, timelineS, title: t("playground.title") }, { signal: controller.signal, progress: (done, total) => setJob({ done, total, controller }) });
      const filename = `staffage-playground_${viewpoint}_${pg.view}_1-${pg.scale}.svg`;
      downloadText(filename, svg);
      setOutcome({ kind: "done", text: t("playground.export.done", { filename, kb: Math.max(1, Math.round(svg.length / 1024)) }) });
    } catch (error) {
      setOutcome(controller.signal.aborted ? { kind: "cancelled", text: t("playground.export.cancelled") } : { kind: "error", text: t("playground.export.failed", { reason: error instanceof Error ? error.message : String(error) }) });
    } finally {
      setJob(null);
    }
  };
  useEffect(() => () => job?.controller.abort(), []);
  const palette = useMemo(() => rows.filter((row) => row.status === "ready" && matches(row, { ...NO_FILTERS, q: search })).slice(0, 60), [rows, search]);
  const interactive = view.opened;
  const unplaceable = pg.placements.filter((placement) => assets.get(placement.asset) === undefined);
  const catalogBusy = snapshot.manifestState === "loading" || Object.values(snapshot.categories).some((category) => category.state === "loading");
  const tabLink = (next: PlaygroundTab): string => formatAddress({ page: "playground", tab: next }, new URLSearchParams(next === "model" || zoom === "fit" ? "" : { pz: String(Number((zoom as number).toFixed(3))) }));
  return (
    <PageFrame
      page="playground"
      wide
      eyebrow={t("playground.eyebrow")}
      title={t("playground.title")}
      lede={t("playground.lede")}
      actions={
        interactive ? (
          <ButtonGroup>
            <Button grouped icon="rotate-ccw" disabled={!history.canUndo} onClick={() => { history.undo(); say(t("playground.said.undone")); }}>{t("playground.undo")}</Button>
            <Button grouped icon="rotate-cw" disabled={!history.canRedo} onClick={() => { history.redo(); say(t("playground.said.redone")); }}>{t("playground.redo")}</Button>
            <Button grouped icon="trash-2" disabled={pg.placements.length === 0} onClick={() => act({ type: "playground-cleared" }, t("playground.said.cleared"))}>{t("playground.clear")}</Button>
          </ButtonGroup>
        ) : undefined
      }
    >
      <p role="status" aria-live="polite" className="sr-only">{message}</p>
      {offered ? (
        <Notice kind="info" title={t("playground.linked.title", { count: linked.placements.length })} actions={<><Button variant="primary" onClick={() => act({ type: "playground-replaced", playground: linked }, t("playground.said.linked"))}>{t("playground.linked.use")}</Button><Button onClick={() => setDismissed(address.hash)}>{t("playground.linked.keep")}</Button></>}>
          {t("playground.linked.body")}
        </Notice>
      ) : null}
      {interactive ? (
        <Column className="bg-panel">
          <Bar below="tablet" as="nav" aria-label={t("playground.tabs")}>
            {PLAYGROUND_TABS.map((next) => (
              <ButtonLink key={next} grouped href={tabLink(next)} icon={TAB_ICONS[next]} variant={tab === next ? "primary" : "default"} aria-current={tab === next ? "page" : undefined} data-tab={next}>{t(`playground.tab.${next}`)}</ButtonLink>
            ))}
          </Bar>
          <Bar below="tablet">
            <Segmented inline label={t("playground.scale")} value={(SCALE_PRESETS as readonly number[]).includes(pg.scale) ? pg.scale : null} onChange={(scale) => config({ scale })} options={SCALE_PRESETS.map((scale) => ({ value: scale as number, label: `1:${scale}` }))} />
            <NumberField label={t("playground.scaleCustom")} value={pg.scale} min={1} max={100000} step={1} onChange={(scale) => scale !== null && scale >= 1 && config({ scale })} className="lg:max-w-40" />
          </Bar>
          <Bar below="tablet">
            {tab === "section" ? <ViewpointSelector inline value={pg.viewpoint} language={language} choices={SECTION_VIEWPOINTS} onChange={(choice) => isViewpoint(choice) && (SECTION_VIEWPOINTS as readonly string[]).includes(choice) && config({ viewpoint: choice as SectionViewpoint })} /> : null}
            <SelectField label={t("playground.view")} value={pg.view} onChange={(next: View) => config({ view: next })} options={VIEWS.map((v) => ({ value: v, label: t(`playground.view.${v}`) }))} />
          </Bar>
          <Bar below="tablet">
            <Segmented inline label={t("playground.background")} value={pg.background} onChange={(background) => config({ background })} options={BACKGROUNDS.map((b) => ({ value: b, label: t(`playground.background.${b}`) }))} />
            <NumberField label={t("playground.length")} unit="m" value={pg.lengthM} min={4} max={200} step={1} onChange={(lengthM) => lengthM !== null && lengthM >= 4 && lengthM <= 200 && config({ lengthM })} className="lg:max-w-40" />
            <NumberField label={t("playground.depth")} unit="m" value={pg.depthM} min={2} max={200} step={1} onChange={(depthM) => depthM !== null && depthM >= 2 && depthM <= 200 && config({ depthM })} className="lg:max-w-40" />
          </Bar>
          {tab === "model" ? null : (
            <Bar below="tablet">
              <CheckField label={t("playground.printSize")} checked={zoom === 1} onChange={(on) => setZoom(on ? 1 : "fit")} />
              <Button grouped onClick={() => setZoom("fit")} aria-pressed={zoom === "fit"} icon="maximize-2">{t("playground.fit")}</Button>
              <Cell className="flex-1 py-single"><span className="font-mono text-xs text-muted-foreground">{t("playground.zoomNow", { percent: Math.round(zoomValue * 100) })}</span></Cell>
            </Bar>
          )}
        </Column>
      ) : null}
      {view.opened ? addressProblems(address.params, "playground", tab).map((problem) => <Notice key={problem.en} kind="info" compact title={problem[language]} />) : null}
      {!mirrorAllowed && pg.placements.some((placement) => placement.mirrored) ? <Notice kind="info" compact title={t("playground.mirroredIgnored", { count: pg.placements.filter((placement) => placement.mirrored).length })} /> : null}
      {mismatch && interactive && tab !== "model" ? <Notice kind="info" compact title={t("playground.contrast.title")}>{t("playground.contrast.body")}</Notice> : null}
      <Bar below="tablet" className="items-stretch">
        <Column className="min-w-0 flex-1">
          {tab === "model" && interactive ? (
            <div className="relative min-h-80" data-surface={surface} data-stage="model">
              <SpatialPlayground placements={pg.placements} assets={readyAssets} scale={pg.scale} view={pg.view} camera={pg.camera} surface={SURFACE_COLOUR[surface]} language={language} clock={clock} timelineS={timelineS} reducedMotion={reducedMotion} onSelect={setSelected} onCamera={(camera) => camera !== pg.camera && config({ camera })} />
            </div>
          ) : (
            <div ref={attach} data-shared-border-cell="" data-border-probe-art="" className={cn("relative min-h-40 overflow-x-auto overflow-y-hidden", SURFACE[surface])} data-surface={surface} data-stage={tab}>
              {pg.placements.length === 0 ? (
                <div className="p-4"><Notice kind="empty" plain title={t("playground.empty.title")}>{t(tab === "plan" ? "playground.empty.bodyPlan" : "playground.empty.body")}</Notice></div>
              ) : (
                <PlanarStage pg={pg} viewpoint={viewpoint} byId={byId} sizes={sizes} interactive={interactive} selected={selected} reaction={reaction} playful={playful} clock={interactive ? clock : ambient} timelineS={timelineS} onSelect={setSelected} onKey={onKey} onDrop={onDrop} />
              )}
            </div>
          )}
          {unplaceable.length > 0 ? <Notice kind={catalogBusy ? "loading" : "unavailable"} compact title={t("playground.unavailable.title", { count: unplaceable.length })}>{t("playground.unavailable.body")}</Notice> : null}
          {interactive ? (
            <>
              <Note>{t(tab === "plan" ? "playground.keysPlan" : tab === "model" ? "playground.keys3d" : "playground.keys")}</Note>
              {tab === "model" ? (
                <Column as="section" aria-label={t("playground.placements")} className="bg-panel" data-placement-list="">
                  <Cell><h2 className="m-0 text-sm font-semibold">{t("playground.placements")}</h2></Cell>
                  {pg.placements.length === 0 ? <Cell className="py-single"><span className="text-xs text-muted-foreground">{t("playground.timeline.empty")}</span></Cell> : (
                    <Bar className="flex-wrap">
                      {pg.placements.map((placement) => (
                        <Button key={placement.id} grouped data-placement={placement.id} aria-pressed={selected === placement.id} aria-label={t("playground.placementLabelPlan", { title: titleOf(placement.asset), x: formatNumber(placement.xM, 2, language), z: formatNumber(placement.zM, 2, language) })} onClick={() => setSelected(placement.id)} onFocus={() => setSelected(placement.id)} onKeyDown={onKey(placement)}>
                          {titleOf(placement.asset)}
                        </Button>
                      ))}
                    </Bar>
                  )}
                </Column>
              ) : null}
              <TimelinePanel placements={pg.placements} assets={assets} titleOf={titleOf} timelineS={timelineS} chosenS={pg.timelineS} clock={clock} selected={selected} onSelect={setSelected} onChange={change} onTimeline={(next) => config({ timelineS: next })} />
              {selectedPlacement !== null ? <AssetPanel placement={selectedPlacement} asset={assets.get(selectedPlacement.asset)} title={titleOf(selectedPlacement.asset)} clock={clock} timelineS={timelineS} mirrorAllowed={mirrorAllowed} change={(patch, text) => change(selectedPlacement.id, patch, text)} duplicate={() => duplicate(selectedPlacement.id)} remove={() => remove(selectedPlacement.id)} /> : null}
              <Column as="section" aria-label={t("playground.share")} className="bg-panel">
                <Bar below="tablet">
                  <Button grouped className="flex-1" icon="link" onClick={() => void copyLink()}>{t("playground.copyLink")}</Button>
                  <Button grouped className="flex-1" icon="file-image" variant="primary" disabled={tab === "model" || pg.placements.length === 0 || job !== null} onClick={() => void runExport()}>{t("playground.exportSvg")}</Button>
                  <Cell className="py-single"><Tag>{t(tab === "model" ? "playground.exportNote3d" : "playground.exportNote", { scale: pg.scale })}</Tag></Cell>
                </Bar>
                {link === null ? null : link.copied ? <Cell role="status"><span className="text-xs">{t("playground.copied")}</span></Cell> : (
                  <>
                    <Notice kind="unsupported" compact title={t("playground.copyFailed.title")} />
                    <Bar><TextField readOnly label={t("playground.copyFailed.field")} value={link.href} onChange={() => undefined} inputRef={(element) => element?.select()} /></Bar>
                  </>
                )}
                {job === null ? null : <ProgressNotice label={t("playground.export.progress")} done={job.done} total={job.total} cancelLabel={t("action.cancel")} onCancel={() => job.controller.abort()} />}
                {outcome === null ? null : <Notice kind={outcome.kind === "error" ? "error" : "info"} compact title={outcome.text} />}
              </Column>
            </>
          ) : null}
        </Column>
        {interactive ? (
          <Column as="aside" aria-label={t("playground.palette")} className="w-full bg-panel lg:w-80 lg:shrink-0">
            <Cell><h2 className="m-0 text-sm font-semibold">{t("playground.palette")}</h2></Cell>
            <Bar><TextField type="search" label={t("playground.paletteSearch")} value={search} onChange={setSearch} /></Bar>
            {everything.busy ? <ProgressNotice label={t("state.loadingAssets")} done={everything.done} total={everything.total} cancelLabel={t("action.cancel")} onCancel={everything.cancel} /> : null}
            {palette.length === 0 ? <Notice kind="empty" compact title={t(everything.busy ? "state.loading.title" : "playground.paletteEmpty")} /> : (
              <Column className="max-h-[32rem] overflow-y-auto">
                {Array.from({ length: Math.ceil(palette.length / 2) }, (_, at) => palette.slice(at * 2, at * 2 + 2)).map((pair) => (
                  <Bar key={pair[0]!.entry.id}>
                    {pair.map((row) => (
                      <button key={row.entry.id} type="button" data-shared-border-cell="" onClick={() => add(row)} aria-label={t("playground.add", { title: row.entry.title[language] })} className="flex min-h-24 min-w-0 flex-1 flex-col gap-single border-0 bg-background p-single text-left text-xs hover:bg-hover-panel">
                        <span className="h-16 w-full"><RowPreview row={row} viewpoint={viewpoint} compact /></span>
                        <span className="font-medium">{row.entry.title[language]}</span>
                        <span className="font-mono text-[0.65rem] text-muted-foreground">{formatNumber(row.entry.heightM, 2, language)} m</span>
                      </button>
                    ))}
                    {pair.length === 1 ? <div data-shared-border-cell="" className="min-w-0 flex-1" /> : null}
                  </Bar>
                ))}
              </Column>
            )}
          </Column>
        ) : null}
      </Bar>
    </PageFrame>
  );
}
