/** ⚖️ The comparison: two to four panes on one shared clock at one scale — other assets, or one asset in other views, styles and analytic states.
 *
 * The opened page reads and writes its state in the address (`#/compare?p=walking-commuter~thermal~clip.walk-cycle&scale=50&t=1.2`); the page behind the glass shows a cheap default built from real ready assets and does not touch the address.
 * All panes follow one `StaffageClock`: seeking, stepping or changing the rate moves every figure on the same frame.
 *
 * @see ./🧮️panes/🟦️.ts — the pure rules
 * @see ../⏯️transport/🟦️.tsx — the transport
 * @see ../🧍️figure/🟦️.tsx — the figure
 */
import { useEffect, useMemo, useState, type ReactElement } from "react";
import { SCALE_PRESETS, VIEWS, formatNumber } from "@teaching/staffage";
import type { Asset, ItemValue, View } from "@teaching/staffage";
import { APP_CONFIG } from "../⚙️config/🟦️.ts";
import { StaffageClock } from "../⏱️clock/🟦️.ts";
import { useArchive, useRows } from "../🧭️context/🟦️.tsx";
import { Button, Notice, NumberField, PageFrame, ProgressNotice, SelectField, Segmented, Tag } from "../🪟️chrome/🟦️.tsx";
import { FigureBoundary, UnavailableNotice, useDarkSurface } from "../🖼️previews/🟦️.tsx";
import { StaffageFigure } from "../🧍️figure/🟦️.tsx";
import { depictionOptions, evaluateState, frameOf, sceneOf, type SceneFrame } from "../🧮️pipeline/🟦️.ts";
import { VIEW_NAMES, valueText } from "../🗣️labels/🟦️.ts";
import { Transport } from "../⏯️transport/🟦️.tsx";
import { useAssetLoading } from "../📚️catalog/🟦️.ts";
import { COMPARE_LIMITS, compareFromParams, compareToParams, navigate, useAddress, type ComparePane, type CompareState } from "../🗺️routing/🟦️.ts";
import type { PaneView } from "../🥞️levels/🟦️.tsx";
import { namedPoses, type Row } from "../🔎️search/🟦️.ts";
import { ACROSS_VIEWS, clockSpan, defaultPanes, differentAssetsSameView, nextPane, paneQuery, sameAssetAcrossViews, selectionFromKey, selectionKey, swapFirstTwo } from "./🧮️panes/🟦️.ts";

type PaneProps = {
  readonly pane: ComparePane;
  readonly index: number;
  readonly row: Row | undefined;
  readonly ready: readonly Row[];
  readonly clock: StaffageClock;
  readonly scale: number;
  readonly frame: SceneFrame | undefined;
  readonly theme: "light" | "dark";
  readonly editable: boolean;
  readonly canRemove: boolean;
  readonly onChange: (pane: ComparePane) => void;
  readonly onRemove: () => void;
};

function ComparePaneCard(props: PaneProps): ReactElement {
  const { pane, index, row, ready, clock, scale, frame, theme, editable, canRemove, onChange, onRemove } = props;
  const { t, language } = useArchive();
  const [inspected, setInspected] = useState<{ readonly value: ItemValue; readonly item: string } | null>(null);
  const asset: Asset | undefined = row?.asset;
  const title = row?.entry.title[language] ?? pane.asset;
  const options = ready.some((candidate) => candidate.entry.id === pane.asset) ? ready : row === undefined ? ready : [row, ...ready];
  const query = useMemo(() => (asset === undefined ? undefined : paneQuery(asset, pane.selection)), [asset, pane.selection]);
  return (
    <section aria-labelledby={`compare-pane-${index}`} data-compare-pane={index} className="flex min-w-0 flex-col gap-double border border-border bg-panel p-double">
      <div className="flex items-start justify-between gap-double">
        <h2 id={`compare-pane-${index}`} className="m-0 font-serif text-lg leading-tight">
          {title}
          <span className="block font-sans text-xs font-normal text-muted-foreground">{VIEW_NAMES[pane.view][language]}</span>
        </h2>
        {editable && canRemove ? <Button variant="quiet" icon="x" aria-label={t("compare.removePane", { index: index + 1 })} title={t("compare.removePane", { index: index + 1 })} onClick={onRemove} /> : null}
      </div>
      <div className="relative flex h-72 min-h-0 items-center justify-center overflow-hidden border border-border sm:h-80" data-compare-figure={index} style={{ backgroundColor: pane.view === "dark-line" || pane.view === "dark-solid" || (theme === "dark" && (pane.view === "thermal" || pane.view === "forces" || pane.view === "moments")) ? "#001117" : "#f7f3e3" }}>
        {asset === undefined ? (
          row === undefined ? <Notice kind="unavailable" compact title={t("compare.unknownAsset")} /> : <UnavailableNotice row={row} compact />
        ) : (
          <FigureBoundary resetKey={asset} fallback={(message) => <Notice kind="error" compact title={title}>{message}</Notice>}>
            <StaffageFigure asset={asset} {...(query === undefined ? {} : { query })} clock={clock} view={pane.view} scale={{ denominator: scale }} language={language} theme={theme} {...(frame === undefined ? {} : { frame })} fit="contain" onInspect={(value, item) => setInspected(value === null ? null : { value, item: item?.region ?? item?.bone ?? "" })} />
          </FigureBoundary>
        )}
      </div>
      <p role="status" aria-live="polite" className="m-0 min-h-4 font-mono text-[0.65rem] text-muted-foreground">
        {inspected === null ? t("compare.inspectHint") : `${inspected.value.label[language]}: ${valueText(inspected.value, language)}`}
      </p>
      {asset === undefined ? null : (
        <p className="m-0 flex flex-wrap gap-single">
          <Tag title={t("fact.height")}>{formatNumber(asset.dimensions.heightM, 2, language)} m</Tag>
          <Tag title={t("compare.width")}>{formatNumber(asset.dimensions.widthM, 2, language)} m</Tag>
          <Tag title={t("fact.mass")}>{formatNumber(asset.dimensions.massKg, asset.dimensions.massKg < 10 ? 1 : 0, language)} kg</Tag>
        </p>
      )}
      {editable ? (
        <div className="grid gap-double sm:grid-cols-2">
          <SelectField label={t("compare.asset")} value={pane.asset} onChange={(next) => onChange({ asset: next, view: pane.view, selection: null })} options={options.map((candidate) => ({ value: candidate.entry.id, label: candidate.entry.title[language] }))} />
          <SelectField label={t("compare.view")} value={pane.view} onChange={(next: View) => onChange({ ...pane, view: next })} options={VIEWS.map((candidate) => ({ value: candidate, label: VIEW_NAMES[candidate][language] }))} />
          <SelectField
            label={t("compare.pick")}
            value={selectionKey(pane.selection)}
            onChange={(key) => onChange({ ...pane, selection: selectionFromKey(key) })}
            className="sm:col-span-2"
            options={[{ value: "default", label: t("compare.pickDefault") }, ...(asset === undefined ? [] : namedPoses(asset)).map((pose) => ({ value: `pose:${pose.id}`, label: `${t("compare.pose")}: ${pose.title[language]}` })), ...(asset?.clips ?? []).map((clip) => ({ value: `clip:${clip.id}`, label: `${t("compare.clip")}: ${clip.title[language]}` }))]}
          />
        </div>
      ) : null}
    </section>
  );
}

/** ⚖️ The comparison page. */
export function ComparePage(props: { readonly view: PaneView }): ReactElement {
  const { view } = props;
  const { t, language, catalog, reducedMotion } = useArchive();
  const { rows, manifest } = useRows();
  const address = useAddress();
  const loading = useAssetLoading(catalog, view.opened || view.revealed, "all");
  const ready = useMemo(() => rows.filter((row) => row.status === "ready"), [rows]);
  const readyIds = useMemo(() => ready.map((row) => row.entry.id), [ready]);
  const parsed = useMemo<CompareState | null>(() => (view.opened ? compareFromParams(address.params) : null), [view.opened, address.hash]);
  const featured = manifest?.collections.find((collection) => collection.id === APP_CONFIG.featuredCollection)?.assets.find((asset) => readyIds.includes(asset));
  const fallback = useMemo(() => defaultPanes(readyIds, featured), [readyIds.join(), featured]);
  const panes = parsed !== null && parsed.panes.length >= COMPARE_LIMITS.min ? parsed.panes : fallback;
  const scale = parsed?.scale ?? 100;
  const find = (id: string): Asset | undefined => rows.find((row) => row.entry.id === id)?.asset;
  const span = clockSpan(panes, find);
  const clock = useMemo(() => new StaffageClock({ fps: 30, durationS: null, loop: true, playing: false, reducedMotion: true }), []);
  useEffect(() => () => clock.dispose(), [clock]);
  useEffect(() => clock.setDuration(span > 0 ? span : null), [clock, span]);
  useEffect(() => {
    if (view.opened && !reducedMotion) clock.play();
    else clock.pause();
  }, [clock, view.opened, reducedMotion]);
  useEffect(() => {
    clock.setRate(parsed?.rate ?? 1);
    clock.setLoop(parsed?.loop ?? true);
  }, [clock, parsed?.rate, parsed?.loop]);
  const seeded = parsed?.timeS ?? null;
  useEffect(() => {
    if (seeded === null || span <= 0) return;
    clock.pause();
    clock.seek(seeded);
  }, [clock, seeded === null, span > 0]);
  const write = (next: Partial<CompareState>): void => {
    if (!view.opened) return;
    const state: CompareState = { panes, scale, timeS: clock.state.playing ? null : clock.timeS, ...(parsed?.rate === undefined ? {} : { rate: parsed.rate }), ...(parsed?.loop === undefined ? {} : { loop: parsed.loop }), ...next };
    navigate({ page: "compare" }, compareToParams(state), true);
  };
  const setPanes = (next: readonly ComparePane[]): void => write({ panes: next.slice(0, COMPARE_LIMITS.max) });
  const rowOf = (id: string): Row | undefined => rows.find((row) => row.entry.id === id);
  const first = panes[0];
  const dark = useDarkSurface();
  const theme = dark ? "dark" : "light";
  const frame = useMemo(() => {
    try {
      const scenes = panes.flatMap((pane) => {
        const asset = find(pane.asset);
        if (asset === undefined) return [];
        const query = paneQuery(asset, pane.selection) ?? { kind: "clip" as const, clip: asset.presentation.defaultClip, timeS: 0 };
        return [sceneOf(evaluateState(asset, query), depictionOptions({ view: pane.view, scale: { denominator: scale }, language, theme, transparent: true }))];
      });
      return scenes.length === 0 ? undefined : frameOf(scenes);
    } catch {
      return undefined;
    }
  }, [panes, rows, scale, language, theme]);
  const columns = panes.length >= 4 ? "sm:grid-cols-2 2xl:grid-cols-4" : "sm:grid-cols-2";
  const body =
    panes.length < COMPARE_LIMITS.min ? (
      loading.busy ? null : (
        <Notice kind="unavailable" title={t("compare.needTwo.title")}>
          {t("compare.needTwo.body", { count: ready.length })}
        </Notice>
      )
    ) : (
      <>
        <div className="flex flex-wrap items-end gap-double">
          <Segmented label={t("compare.scale")} value={(SCALE_PRESETS as readonly number[]).includes(scale) ? scale : null} onChange={(next) => write({ scale: next })} options={SCALE_PRESETS.map((preset) => ({ value: preset, label: `1:${preset}` }))} />
          <NumberField label={t("compare.scaleCustom")} min={1} max={100000} step={1} value={scale} onChange={(next) => next !== null && next >= 1 && write({ scale: next })} className="w-32" unit="1:" />
          <div className="flex flex-wrap gap-single" role="group" aria-label={t("compare.quick")}>
            <Button icon="layout-grid" disabled={first === undefined} onClick={() => first !== undefined && setPanes(sameAssetAcrossViews(first.asset, panes.length, first.selection))}>{t("compare.sameAsset")}</Button>
            <Button icon="users" disabled={first === undefined} onClick={() => first !== undefined && setPanes(differentAssetsSameView(readyIds, first.asset, first.view, panes.length))}>{t("compare.sameView")}</Button>
            <Button icon="plus" disabled={panes.length >= COMPARE_LIMITS.max} onClick={() => setPanes([...panes, nextPane(panes)])}>{t("compare.addPane")}</Button>
            <Button icon="rotate-cw" onClick={() => setPanes(swapFirstTwo(panes))}>{t("compare.swap")}</Button>
            <Button icon="rotate-ccw" onClick={() => navigate({ page: "compare" }, new URLSearchParams(), true)}>{t("compare.reset")}</Button>
          </div>
        </div>
        <Transport clock={clock} label={t("compare.transport")} onUserTime={(timeS) => write({ timeS })} onRate={(rate) => write({ rate })} onLoop={(loop) => write({ loop })} />
        <div className={`grid gap-double ${columns}`}>
          {panes.map((pane, index) => (
            <ComparePaneCard key={index} pane={pane} index={index} row={rowOf(pane.asset)} ready={ready} clock={clock} scale={scale} frame={frame} theme={theme} editable={view.opened} canRemove={panes.length > COMPARE_LIMITS.min} onChange={(next) => setPanes(panes.map((candidate, at) => (at === index ? next : candidate)))} onRemove={() => setPanes(panes.filter((_, at) => at !== index))} />
          ))}
        </div>
        <p className="m-0 text-xs text-muted-foreground">{t("compare.sharedClock", { views: ACROSS_VIEWS.length })}</p>
      </>
    );
  return (
    <PageFrame page="compare" wide eyebrow={t("compare.eyebrow")} title={t("compare.title")} lede={t("compare.lede")}>
      {loading.busy ? <ProgressNotice label={t("state.loadingAssets")} done={loading.done} total={loading.total} cancelLabel={t("action.cancel")} onCancel={loading.cancel} /> : null}
      {loading.cancelled ? <Notice kind="info" compact title={t("state.cancelled.title")} actions={<Button onClick={loading.retry} icon="rotate-cw">{t("action.retry")}</Button>}>{t("state.cancelled.body")}</Notice> : null}
      {body}
    </PageFrame>
  );
}
