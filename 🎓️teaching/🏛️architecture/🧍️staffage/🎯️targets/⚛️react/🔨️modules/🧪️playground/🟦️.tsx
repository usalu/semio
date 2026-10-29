/** 🧪️ The playground: figures on one ground line at one true scale, dragged or moved by keyboard, each with its own pose or clip, exported as one SVG at correct paper offsets.
 *
 * One pixel-per-millimetre factor serves every figure, so relative scale is exact whatever drawing scale the line weights use; zoom is a separate screen factor (1 = print size on screen).
 * Edits are events on the local store (persisted, undoable in memory); the address carries a shareable composition that only the open page reads and only applies when the reader agrees.
 * A little play, kept restrained: figures idle on the ambient clock, a dropped figure squashes and hops once, the ground takes a soft pat; reduced motion and the "playful" switch both silence it.
 *
 * @see ./🧱️compose/🟦️.ts — SVG composition
 * @see ./🧲️keys/🟦️.ts — keyboard model
 * @see ../📍️placed/🟦️.tsx — the figure on a ground line
 */
import { useCallback, useEffect, useMemo, useRef, useState, type PointerEvent as ReactPointerEvent, type ReactElement, type KeyboardEvent as ReactKeyboardEvent } from "react";
import { CSS_PX_PER_MM, SCALE_PRESETS, VIEWS, formatNumber, mmPerMetre } from "@teaching/staffage";
import type { Asset, View } from "@teaching/staffage";
import { useArchive, useLocal, useRows } from "../🧭️context/🟦️.tsx";
import { Button, CheckField, Notice, NumberField, PageFrame, ProgressNotice, Segmented, SelectField, Tag, TextField, cn } from "../🪟️chrome/🟦️.tsx";
import { useSize, type PaneView } from "../🥞️levels/🟦️.tsx";
import { RowPreview } from "../🖼️previews/🟦️.tsx";
import { PlacedFigure } from "../📍️placed/🟦️.tsx";
import { BACKGROUNDS, type Background, type Placement, type Playground, type Selection } from "../💾️persistence/🟦️.ts";
import { formatAddress, navigate, playgroundFromParams, playgroundToParams, useAddress } from "../🗺️routing/🟦️.ts";
import { matches, namedPoses, NO_FILTERS, type Row } from "../🔎️search/🟦️.ts";
import { useAssetLoading } from "../📚️catalog/🟦️.ts";
import { snap } from "./🧱️compose/🟦️.ts";
import { downloadText, exportComposition } from "./📤️export/🟦️.ts";
import { keyAction } from "./🧲️keys/🟦️.ts";
import { useHistory } from "./📜️history/🟦️.ts";

const PAD_PX = 24;
const GROUND_GAP_PX = 34;
const MIN_HEIGHT_M = 2;
const HOP_MS = 520;
const DARK_VIEWS: readonly View[] = ["dark-line", "dark-solid"];

/** 🎨️ The ground the figures really stand on: "paper" follows the view, so light ink never lands on cream and dark ink never on night. */
export const effectiveBackground = (view: View, background: Background): Background => (background === "paper" ? (DARK_VIEWS.includes(view) ? "dark" : "paper") : background);

const SURFACE: Readonly<Record<Background, string>> = { paper: "bg-[#f4efe1] text-[#1b1b1b]", dark: "bg-[#0b1a1f] text-[#e8e4d4]", light: "bg-white text-[#1b1b1b]", transparent: "staffage-checker text-[#1b1b1b]" };

const newId = (): string => `p-${(globalThis.crypto?.randomUUID?.() ?? Math.random().toString(36).slice(2)).slice(0, 8)}`;
const pickValue = (pick: Selection | null): string => (pick === null ? "default" : pick.kind === "pose" ? `pose:${pick.pose}` : `clip:${pick.clip}`);
const pickFrom = (value: string): Selection | null => (value === "default" ? null : value.startsWith("pose:") ? { kind: "pose", pose: value.slice(5) } : { kind: "clip", clip: value.slice(5) });
const clampM = (value: number, lengthM: number): number => Math.min(Math.max(value, 0), lengthM);
const sameComposition = (a: Playground, b: Playground): boolean => playgroundToParams(a).toString() === playgroundToParams(b).toString();

type Reaction = { readonly id: string; readonly token: number };

function Placed(props: { readonly placement: Placement; readonly row: Row | undefined; readonly xM: number; readonly selected: boolean; readonly reaction: Reaction | null; readonly playful: boolean; readonly pg: Playground; readonly pxPerMm: number; readonly metrePx: number; readonly groundPx: number; readonly interactive: boolean; readonly onSelect: () => void; readonly onKey: (event: ReactKeyboardEvent) => void; readonly onDown: (event: ReactPointerEvent) => void; readonly onMove: (event: ReactPointerEvent) => void; readonly onUp: (event: ReactPointerEvent) => void }): ReactElement {
  const { placement, row, xM, selected, reaction, playful, pg, pxPerMm, metrePx, groundPx, interactive } = props;
  const { ambient, language, t } = useArchive();
  const hop = useRef<HTMLDivElement>(null);
  const pat = useRef<HTMLSpanElement>(null);
  const xPx = PAD_PX + xM * metrePx;
  const asset: Asset | undefined = row?.asset;
  const title = row?.entry.title[language] ?? placement.asset;
  const heightPx = Math.max(asset?.dimensions.heightM ?? 1, 0.4) * metrePx;
  const widthPx = Math.max(asset?.dimensions.widthM ?? 0.5, 0.5) * metrePx;
  useEffect(() => {
    if (reaction === null || reaction.id !== placement.id || !playful) return;
    hop.current?.animate?.([{ transform: "scale(1.06, 0.88) translateY(0)" }, { transform: "scale(0.97, 1.05) translateY(-6px)", offset: 0.45 }, { transform: "scale(1, 1) translateY(0)" }], { duration: HOP_MS, easing: "cubic-bezier(.3,.7,.4,1)" });
    pat.current?.animate?.([{ opacity: 0.55, transform: "scaleX(0.4)" }, { opacity: 0, transform: "scaleX(1.4)" }], { duration: HOP_MS * 1.4, easing: "ease-out" });
  }, [reaction, placement.id, playful]);
  return (
    <>
      <div ref={hop} className="pointer-events-none absolute inset-0" style={{ transformOrigin: `${xPx}px ${groundPx}px` }}>
        {asset === undefined ? null : <PlacedFigure asset={asset} selection={placement.pick} clock={ambient} offsetS={placement.offsetS} view={pg.view} scale={pg.scale} language={language} pxPerMm={pxPerMm} xPx={xPx} groundPx={groundPx} mirrored={placement.mirrored} label={title} />}
        <span ref={pat} aria-hidden="true" className="absolute h-1.5 w-12 -translate-x-1/2 rounded-[50%] bg-current opacity-0" style={{ left: xPx, top: groundPx - 3 }} />
      </div>
      {interactive ? (
        <div
          role="button"
          tabIndex={0}
          aria-pressed={selected}
          aria-label={t("playground.placementLabel", { title, x: formatNumber(xM, 2, language) })}
          data-placement={placement.id}
          data-x-m={xM}
          onFocus={props.onSelect}
          onKeyDown={props.onKey}
          onPointerDown={props.onDown}
          onPointerMove={props.onMove}
          onPointerUp={props.onUp}
          onPointerCancel={props.onUp}
          className={cn("absolute cursor-grab touch-none active:cursor-grabbing", selected && "outline-2 outline-dashed outline-offset-2 outline-[var(--accent)]")}
          style={{ left: xPx - widthPx / 2, top: groundPx - heightPx, width: widthPx, height: heightPx }}
        />
      ) : null}
    </>
  );
}

function nextX(pg: Playground, rows: ReadonlyMap<string, Row>): number {
  const end = Math.max(1.5, ...pg.placements.map((placement) => placement.xM + (rows.get(placement.asset)?.asset?.dimensions.widthM ?? 0.6) + 0.5));
  return snap(end > pg.lengthM - 1 ? pg.lengthM / 2 : end);
}

/** 🧪️ The playground page; the background copy is a cheap, inert preview of the local composition. */
export function PlaygroundPage(props: { readonly view: PaneView }): ReactElement {
  const { view } = props;
  const { t, language, store, ambient, reducedMotion, catalog } = useArchive();
  const { playground: pg, preferences } = useLocal();
  const { rows } = useRows();
  const address = useAddress();
  const [attach, size] = useSize({ width: 1000, height: 600 });
  const [selected, setSelected] = useState<string | null>(null);
  const [zoom, setZoom] = useState<"fit" | number>(() => {
    const value = Number(address.params.get("pz"));
    return view.opened && Number.isFinite(value) && value > 0 ? Math.min(value, 64) : "fit";
  });
  useEffect(() => {
    if (!view.opened) return;
    const params = new URLSearchParams(window.location.hash.split("?")[1] ?? "");
    const wanted = zoom === "fit" ? null : String(Number(zoom.toFixed(3)));
    if (params.get("pz") === wanted) return;
    if (wanted === null) params.delete("pz");
    else params.set("pz", wanted);
    navigate({ page: "playground" }, params, true);
  }, [zoom, view.opened]);
  const [drag, setDrag] = useState<{ readonly id: string; readonly xM: number } | null>(null);
  const [reaction, setReaction] = useState<Reaction | null>(null);
  const [message, setMessage] = useState("");
  const [search, setSearch] = useState("");
  const [dismissed, setDismissed] = useState("");
  const [link, setLink] = useState<{ readonly href: string; readonly copied: boolean } | null>(null);
  const [job, setJob] = useState<{ readonly done: number; readonly total: number; readonly controller: AbortController } | null>(null);
  const [outcome, setOutcome] = useState<{ readonly kind: "done" | "cancelled" | "error"; readonly text: string } | null>(null);
  const gesture = useRef<{ readonly id: string; readonly startX: number; readonly startM: number; moved: boolean } | null>(null);
  const playful = preferences.playful && !reducedMotion;
  const byId = useMemo(() => new Map(rows.map((row) => [row.entry.id, row])), [rows]);
  const categories = useMemo(() => [...new Set(pg.placements.flatMap((placement) => (byId.get(placement.asset) === undefined ? [] : [byId.get(placement.asset)!.entry.category])))], [pg.placements, byId]);
  useAssetLoading(catalog, true, categories, categories.join());
  const everything = useAssetLoading(catalog, view.opened, "all");
  const current = useCallback(() => store.getSnapshot().playground, [store]);
  const history = useHistory(current, useCallback((next: Playground) => store.dispatch({ type: "playground-replaced", playground: next }), [store]));
  const say = (text: string): void => setMessage(text);
  const k = mmPerMetre({ denominator: pg.scale });
  const fitZoom = Math.max(0.05, (size.width - 2 * PAD_PX) / (pg.lengthM * k * CSS_PX_PER_MM));
  const zoomValue = zoom === "fit" ? fitZoom : zoom;
  const pxPerMm = CSS_PX_PER_MM * zoomValue;
  const metrePx = k * pxPerMm;
  const tallestM = Math.max(MIN_HEIGHT_M, ...pg.placements.map((placement) => byId.get(placement.asset)?.asset?.dimensions.heightM ?? 0));
  const stageW = 2 * PAD_PX + pg.lengthM * metrePx;
  const stageH = Math.max(160, tallestM * metrePx + GROUND_GAP_PX + 24);
  const groundPx = stageH - GROUND_GAP_PX;
  const surface = effectiveBackground(pg.view, pg.background);
  const mismatch = (DARK_VIEWS.includes(pg.view) && pg.background === "light") || (!DARK_VIEWS.includes(pg.view) && pg.background === "dark");
  const linked = view.opened && address.route.page === "playground" ? playgroundFromParams(address.params) : null;
  const offered = linked !== null && !sameComposition(linked, pg) && dismissed !== address.hash;
  const selectedPlacement = pg.placements.find((placement) => placement.id === selected) ?? null;
  const selectedRow = selectedPlacement === null ? undefined : byId.get(selectedPlacement.asset);
  const titleOf = (id: string): string => byId.get(id)?.entry.title[language] ?? id;
  const act = (event: Parameters<typeof store.dispatch>[0], text: string): void => {
    history.record();
    store.dispatch(event);
    say(text);
  };
  const config = (patch: Partial<Pick<Playground, "scale" | "view" | "background" | "lengthM">>): void => act({ type: "playground-configured", patch }, t("playground.said.configured"));
  const react = (id: string): void => setReaction((previous) => ({ id, token: (previous?.token ?? 0) + 1 }));
  const add = (row: Row): void => {
    const placement: Placement = { id: newId(), asset: row.entry.id, xM: nextX(pg, byId), pick: null, offsetS: 0, mirrored: false };
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
    } else if (action.kind === "remove") remove(placement.id);
    else if (action.kind === "duplicate") duplicate(placement.id);
    else select(action.step, placement.id);
  };
  const onDown = (placement: Placement) => (event: ReactPointerEvent): void => {
    setSelected(placement.id);
    event.currentTarget.setPointerCapture?.(event.pointerId);
    gesture.current = { id: placement.id, startX: event.clientX, startM: placement.xM, moved: false };
  };
  const onMove = (event: ReactPointerEvent): void => {
    const g = gesture.current;
    if (g === null) return;
    const xM = clampM(snap(g.startM + (event.clientX - g.startX) / metrePx), pg.lengthM);
    g.moved ||= Math.abs(event.clientX - g.startX) > 2;
    if (g.moved) setDrag({ id: g.id, xM });
  };
  const onUp = (event: ReactPointerEvent): void => {
    const g = gesture.current;
    gesture.current = null;
    event.currentTarget.releasePointerCapture?.(event.pointerId);
    if (g === null || !g.moved || drag === null) return setDrag(null);
    const placement = pg.placements.find((candidate) => candidate.id === g.id);
    setDrag(null);
    if (placement === undefined || drag.xM === placement.xM) return;
    change(placement.id, { xM: drag.xM }, t("playground.said.dropped", { title: titleOf(placement.asset), x: formatNumber(drag.xM, 2, language) }));
    react(placement.id);
  };
  const copyLink = async (): Promise<void> => {
    const href = new URL(formatAddress({ page: "playground" }, playgroundToParams(pg)), window.location.href).href;
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
    const assets = new Map(pg.placements.flatMap((placement) => (byId.get(placement.asset)?.asset === undefined ? [] : [[placement.asset, byId.get(placement.asset)!.asset!] as const])));
    try {
      const svg = await exportComposition({ placements: pg.placements, assets, scale: pg.scale, view: pg.view, background: surface, lengthM: pg.lengthM, language, timeS: ambient.state.frame / ambient.state.fps, title: t("playground.title") }, { signal: controller.signal, progress: (done, total) => setJob({ done, total, controller }) });
      const filename = `staffage-playground_1-${pg.scale}_${pg.view}.svg`;
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
  const unplaceable = pg.placements.filter((placement) => byId.get(placement.asset)?.asset === undefined);
  return (
    <PageFrame
      page="playground"
      wide
      eyebrow={t("playground.eyebrow")}
      title={t("playground.title")}
      lede={t("playground.lede")}
      actions={
        interactive ? (
          <>
            <Button icon="rotate-ccw" disabled={!history.canUndo} onClick={() => { history.undo(); say(t("playground.said.undone")); }}>{t("playground.undo")}</Button>
            <Button icon="rotate-cw" disabled={!history.canRedo} onClick={() => { history.redo(); say(t("playground.said.redone")); }}>{t("playground.redo")}</Button>
            <Button icon="trash-2" disabled={pg.placements.length === 0} onClick={() => act({ type: "playground-cleared" }, t("playground.said.cleared"))}>{t("playground.clear")}</Button>
          </>
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
        <div className="flex flex-wrap items-end gap-double border border-border bg-panel p-double">
          <Segmented label={t("playground.scale")} value={(SCALE_PRESETS as readonly number[]).includes(pg.scale) ? pg.scale : null} onChange={(scale) => config({ scale })} options={SCALE_PRESETS.map((scale) => ({ value: scale as number, label: `1:${scale}` }))} />
          <NumberField label={t("playground.scaleCustom")} value={pg.scale} min={1} max={100000} step={1} onChange={(scale) => scale !== null && scale >= 1 && config({ scale })} className="w-28" />
          <SelectField label={t("playground.view")} value={pg.view} onChange={(next: View) => config({ view: next })} options={VIEWS.map((v) => ({ value: v, label: t(`playground.view.${v}`) }))} className="w-48" />
          <Segmented label={t("playground.background")} value={pg.background} onChange={(background) => config({ background })} options={BACKGROUNDS.map((b) => ({ value: b, label: t(`playground.background.${b}`) }))} />
          <NumberField label={t("playground.length")} unit="m" value={pg.lengthM} min={4} max={200} step={1} onChange={(lengthM) => lengthM !== null && lengthM >= 4 && lengthM <= 200 && config({ lengthM })} className="w-28" />
          <div className="flex flex-col gap-single">
            <CheckField label={t("playground.printSize")} checked={zoom === 1} onChange={(on) => setZoom(on ? 1 : "fit")} />
            <Button onClick={() => setZoom("fit")} aria-pressed={zoom === "fit"} icon="maximize-2">{t("playground.fit")}</Button>
          </div>
          <p className="m-0 font-mono text-xs text-muted-foreground">{t("playground.zoomNow", { percent: Math.round(zoomValue * 100) })}</p>
        </div>
      ) : null}
      {mismatch && interactive ? <Notice kind="info" compact title={t("playground.contrast.title")}>{t("playground.contrast.body")}</Notice> : null}
      <div className={cn("grid gap-double", interactive && "xl:grid-cols-[minmax(0,1fr)_20rem]")}>
        <div className="flex min-w-0 flex-col gap-double">
          <div ref={attach} className={cn("relative min-h-40 overflow-x-auto overflow-y-hidden border border-border", SURFACE[surface])} data-surface={surface}>
            {pg.placements.length === 0 ? (
              <div className="p-4"><Notice kind="empty" title={t("playground.empty.title")}>{t("playground.empty.body")}</Notice></div>
            ) : (
              <div role="group" aria-label={t("playground.stageLabel", { count: pg.placements.length, scale: pg.scale })} className="relative" style={{ width: stageW, height: stageH }} data-metre-px={metrePx}>
                <div aria-hidden="true" className="absolute border-t border-current opacity-70" style={{ left: PAD_PX, width: pg.lengthM * metrePx, top: groundPx }} />
                <div aria-hidden="true" className="absolute font-mono text-[0.65rem] opacity-80" style={{ left: PAD_PX, top: groundPx + 2 }}>
                  {Array.from({ length: Math.floor(pg.lengthM) + 1 }, (_, metre) => (
                    <span key={metre} className="absolute -translate-x-1/2 border-l border-current pl-px" style={{ left: metre * metrePx, height: metre % 5 === 0 ? 10 : 5 }}>
                      {metre % 5 === 0 ? <span className="relative left-1 top-2.5 whitespace-nowrap">{metre} m</span> : null}
                    </span>
                  ))}
                </div>
                {pg.placements.map((placement) => (
                  <Placed key={placement.id} placement={placement} row={byId.get(placement.asset)} xM={drag?.id === placement.id ? drag.xM : placement.xM} selected={selected === placement.id} reaction={reaction} playful={playful} pg={pg} pxPerMm={pxPerMm} metrePx={metrePx} groundPx={groundPx} interactive={interactive} onSelect={() => setSelected(placement.id)} onKey={onKey(placement)} onDown={onDown(placement)} onMove={onMove} onUp={onUp} />
                ))}
              </div>
            )}
          </div>
          {unplaceable.length > 0 ? <Notice kind="unavailable" compact title={t("playground.unavailable.title", { count: unplaceable.length })}>{t("playground.unavailable.body")}</Notice> : null}
          {interactive ? (
            <>
              <p className="m-0 text-xs text-muted-foreground">{t("playground.keys")}</p>
              {selectedPlacement !== null ? (
                <section aria-label={t("playground.selected", { title: titleOf(selectedPlacement.asset) })} className="grid gap-double border border-border bg-panel p-double sm:grid-cols-2 lg:grid-cols-4">
                  <h2 className="col-span-full m-0 text-sm font-semibold">{titleOf(selectedPlacement.asset)}</h2>
                  <SelectField label={t("playground.pick")} value={pickValue(selectedPlacement.pick)} onChange={(value) => change(selectedPlacement.id, { pick: pickFrom(value) }, t("playground.said.picked"))} options={[{ value: "default", label: t("playground.pick.default") }, ...(selectedRow?.asset === undefined ? [] : namedPoses(selectedRow.asset)).map((pose) => ({ value: `pose:${pose.id}`, label: `${t("playground.pick.pose")}: ${pose.title[language]}` })), ...(selectedRow?.asset?.clips ?? []).map((clip) => ({ value: `clip:${clip.id}`, label: `${t("playground.pick.clip")}: ${clip.title[language]}` }))]} />
                  <NumberField label={t("playground.x")} unit="m" step={0.05} value={selectedPlacement.xM} onChange={(xM) => xM !== null && change(selectedPlacement.id, { xM }, t("playground.said.moved", { title: titleOf(selectedPlacement.asset), x: formatNumber(clampM(xM, pg.lengthM), 2, language) }))} />
                  <NumberField label={t("playground.offset")} unit="s" step={0.1} value={selectedPlacement.offsetS} onChange={(offsetS) => offsetS !== null && change(selectedPlacement.id, { offsetS }, t("playground.said.offset"))} />
                  <div className="flex flex-wrap items-end gap-single">
                    <CheckField label={t("playground.mirror")} checked={selectedPlacement.mirrored} onChange={(mirrored) => change(selectedPlacement.id, { mirrored }, t("playground.said.mirrored"))} />
                    <Button icon="copy" onClick={() => duplicate(selectedPlacement.id)}>{t("playground.duplicate")}</Button>
                    <Button icon="trash-2" onClick={() => remove(selectedPlacement.id)}>{t("playground.remove")}</Button>
                  </div>
                </section>
              ) : null}
              <section aria-label={t("playground.share")} className="flex flex-col gap-double border border-border bg-panel p-double">
                <div className="flex flex-wrap items-center gap-single">
                  <Button icon="link" onClick={() => void copyLink()}>{t("playground.copyLink")}</Button>
                  <Button icon="file-image" variant="primary" disabled={pg.placements.length === 0 || job !== null} onClick={() => void runExport()}>{t("playground.exportSvg")}</Button>
                  <Tag>{t("playground.exportNote", { scale: pg.scale })}</Tag>
                </div>
                {link === null ? null : link.copied ? <p role="status" className="m-0 text-xs">{t("playground.copied")}</p> : (
                  <Notice kind="unsupported" compact title={t("playground.copyFailed.title")}>
                    <TextField readOnly label={t("playground.copyFailed.field")} value={link.href} onChange={() => undefined} inputRef={(element) => element?.select()} />
                  </Notice>
                )}
                {job === null ? null : <ProgressNotice label={t("playground.export.progress")} done={job.done} total={job.total} cancelLabel={t("action.cancel")} onCancel={() => job.controller.abort()} />}
                {outcome === null ? null : <Notice kind={outcome.kind === "error" ? "error" : outcome.kind === "cancelled" ? "info" : "info"} compact title={outcome.text} />}
              </section>
            </>
          ) : null}
        </div>
        {interactive ? (
          <aside aria-label={t("playground.palette")} className="flex min-w-0 flex-col gap-double border border-border bg-panel p-double">
            <h2 className="m-0 text-sm font-semibold">{t("playground.palette")}</h2>
            <TextField type="search" label={t("playground.paletteSearch")} value={search} onChange={setSearch} />
            {everything.busy ? <ProgressNotice label={t("state.loadingAssets")} done={everything.done} total={everything.total} cancelLabel={t("action.cancel")} onCancel={everything.cancel} /> : null}
            {palette.length === 0 ? <Notice kind="empty" compact title={t(everything.busy ? "state.loading.title" : "playground.paletteEmpty")} /> : (
              <ul className="m-0 grid max-h-[32rem] list-none grid-cols-2 gap-single overflow-y-auto p-0">
                {palette.map((row) => (
                  <li key={row.entry.id}>
                    <button type="button" onClick={() => add(row)} aria-label={t("playground.add", { title: row.entry.title[language] })} className="flex h-full min-h-24 w-full flex-col gap-single border border-border bg-background p-single text-left text-xs hover:bg-hover-panel">
                      <span className="h-16 w-full"><RowPreview row={row} compact /></span>
                      <span className="font-medium">{row.entry.title[language]}</span>
                      <span className="font-mono text-[0.65rem] text-muted-foreground">{formatNumber(row.entry.heightM, 2, language)} m</span>
                    </button>
                  </li>
                ))}
              </ul>
            )}
          </aside>
        ) : null}
      </div>
    </PageFrame>
  );
}
