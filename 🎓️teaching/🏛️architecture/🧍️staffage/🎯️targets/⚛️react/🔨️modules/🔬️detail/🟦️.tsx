/** 🔬️ The page of one asset: header and actions, the viewer, poses and clips with their timeline, dimensions, the model assumptions inspector, provenance and the export panel.
 *
 * The viewer's state is the address while this page is open (`#/humans/walking-commuter?view=thermal&scale=50&clip=walk-cycle&t=1.2`); in the background it shows the default state and touches no address.
 * An asset that is not authored or fails validation gets the reason in place of the viewer, never a crash.
 *
 * @see ../🎛️viewer/🟦️.tsx — the viewer
 * @see ./📐️dimensions/🟦️.tsx — and the other sections
 */
import { useMemo, useRef, useState, type ReactElement } from "react";
import type { Asset, EvaluationQuery } from "@teaching/staffage";
import { useClock } from "../⏱️clock/🟦️.ts";
import { ViewerBoundary } from "../🎛️viewer/🛡️boundary/🟦️.tsx";
import { Viewer, type ViewerFrame } from "../🎛️viewer/🟦️.tsx";
import { useArchive, useLocal, useRows } from "../🧭️context/🟦️.tsx";
import { Button, ButtonLink, Notice, PageFrame, Tag } from "../🪟️chrome/🟦️.tsx";
import type { PaneView } from "../🥞️levels/🟦️.tsx";
import { FavoriteButton, RowFacts } from "../🗃️browse/🟦️.tsx";
import { UnavailableNotice } from "../🖼️previews/🟦️.tsx";
import { freshId, placeAllEvents } from "../⭐️collections/🧮️actions/🟦️.ts";
import type { Row } from "../🔎️search/🟦️.ts";
import { DEFAULT_VIEWER, compareToParams, formatAddress, navigate, useAddress, viewerFromParams, viewerToParams, type ViewerState } from "../🗺️routing/🟦️.ts";
import { PosesAndClips, ClipTimeline } from "./🎞️clips/🟦️.tsx";
import { DimensionsSection } from "./📐️dimensions/🟦️.tsx";
import { ExportPanel } from "./📤️export-panel/🟦️.tsx";
import { ModelSection } from "./🧬️model/🟦️.tsx";
import { ProvenanceSection } from "./📚️provenance/🟦️.tsx";

function Timeline(props: { readonly asset: Asset; readonly frame: ViewerFrame | null; readonly viewer: ViewerState; readonly onSeek: (timeS: number) => void }): ReactElement | null {
  const { asset, frame, viewer } = props;
  const clock = useClock(frame?.clock);
  const clipId = viewer.selection?.kind === "clip" ? viewer.selection.clip : asset.presentation.defaultClip;
  const clip = asset.clips.find((candidate) => candidate.id === clipId);
  if (clip === undefined || viewer.selection?.kind === "pose") return null;
  return <ClipTimeline asset={asset} clip={clip} timeS={clock?.timeS ?? viewer.timeS ?? 0} onSeek={(timeS) => { frame?.clock.pause(); frame?.clock.seek(timeS); props.onSeek(timeS); }} />;
}

/** 🔬️ The asset page. */
export function AssetPage(props: { readonly row: Row; readonly view: PaneView }): ReactElement {
  const { row, view } = props;
  const { t, language, store, catalog } = useArchive();
  const { manifest } = useRows();
  const { playground } = useLocal();
  const address = useAddress();
  const active = view.opened && address.route.page === "asset" && address.route.asset === row.entry.id;
  const viewer = useMemo(() => (active ? viewerFromParams(address.params) : DEFAULT_VIEWER), [active, address.hash]);
  const latest = useRef(viewer);
  latest.current = viewer;
  const [frame, setFrame] = useState<ViewerFrame | null>(null);
  const [copied, setCopied] = useState<"idle" | "done" | "blocked">("idle");
  const [announce, setAnnounce] = useState("");
  const route = { page: "asset", category: row.entry.category, asset: row.entry.id } as const;
  const change = (patch: Partial<ViewerState>, replace = true): void => {
    if (!active) return;
    latest.current = { ...latest.current, ...patch };
    navigate(route, viewerToParams(latest.current), replace);
  };
  const asset = row.asset;
  const title = row.entry.title[language];
  const category = manifest?.categories.find((entry) => entry.id === row.entry.category)?.title[language] ?? row.entry.category;
  const query: EvaluationQuery = frame?.scene.metadata.query ?? { kind: "pose", pose: asset?.presentation.defaultPose ?? "" };
  const share = async (): Promise<void> => {
    const href = `${window.location.origin}${window.location.pathname}${active ? window.location.hash : formatAddress(route)}`;
    try {
      await navigator.clipboard.writeText(href);
      setCopied("done");
    } catch {
      setCopied("blocked");
    }
  };
  const actions = (
    <>
      <FavoriteButton asset={row.entry.id} title={title} className="border border-border" />
      {asset === undefined ? null : (
        <>
          <ButtonLink href={formatAddress({ page: "compare" }, compareToParams({ panes: [{ asset: asset.id, view: "dark-line", selection: null }, { asset: asset.id, view: "thermal", selection: null }], scale: viewer.scale, timeS: null }))} icon="combine">{t("detail.compare")}</ButtonLink>
          <Button icon="plus" onClick={() => { for (const event of placeAllEvents([asset], playground, () => freshId("placed"))) store.dispatch(event); setAnnounce(t("detail.placed", { title })); }}>{t("detail.place")}</Button>
        </>
      )}
      <Button icon="link" onClick={() => void share()}>{t("detail.share")}</Button>
    </>
  );
  return (
    <PageFrame page={`asset:${row.entry.id}`} wide eyebrow={category} title={title} lede={asset?.summary[language] ?? row.entry.distinction[language]} actions={actions}>
      <p role="status" className="sr-only">{announce}</p>
      {copied === "idle" ? null : <Notice kind="info" compact title={t(copied === "done" ? "detail.shareDone" : "detail.shareBlocked")} />}
      <div className="flex flex-wrap items-center gap-single">
        <RowFacts row={row} />
        {asset === undefined ? null : asset.tags.map((tag) => <Tag key={tag}>{tag.replaceAll("-", " ")}</Tag>)}
        <Tag>{t(`completion.${asset?.completion ?? row.entry.completion}`)}</Tag>
        {row.entry.analytics.map((analytic) => <Tag key={analytic}>{t(`analytic.${analytic}`)}</Tag>)}
      </div>
      {asset === undefined ? (
        <>
          <UnavailableNotice row={row} />
          <Notice kind="info" compact title={t("detail.planned")}>{row.entry.distinction[language]}</Notice>
        </>
      ) : (
        <>
          <h2 className="sr-only">{t("detail.viewerHeading")}</h2>
          <ViewerBoundary resetKey={asset}>
            <Viewer asset={asset} viewer={viewer} onChange={change} onFrame={setFrame} />
          </ViewerBoundary>
          <section aria-labelledby={`poses-${asset.id}`} className="flex flex-col gap-double">
            <PosesAndClips asset={asset} selection={viewer.selection} onSelect={(selection) => change({ selection, timeS: null })} />
            <Timeline asset={asset} frame={frame} viewer={viewer} onSeek={(timeS) => change({ timeS })} />
          </section>
          <DimensionsSection asset={asset} row={row} />
          <ModelSection asset={asset} state={frame?.state ?? null} thermal={frame?.thermal ?? null} structure={frame?.structure ?? null} environment={frame?.environment ?? asset.thermal.environment} language={language} />
          <ProvenanceSection asset={asset} />
          <ExportPanel asset={asset} query={query} language={language} options={{ sheet: viewer.sheet, fps: viewer.fps, durationS: viewer.durationS }} onOptions={(settings) => change({ sheet: settings.sheet, fps: settings.fps, durationS: settings.durationS })} request={{ view: viewer.view, scale: { denominator: viewer.scale }, language, theme: viewer.theme === "auto" ? (document.documentElement.classList.contains("dark") ? "dark" : "light") : viewer.theme, thermalQuantity: viewer.thermalQuantity, forceQuantity: viewer.forceQuantity, range: viewer.range, patterns: viewer.patterns, diagrams: viewer.diagrams, ...(Object.keys(viewer.environment).length > 0 ? { environment: frame?.environment ?? { ...asset.thermal.environment, ...viewer.environment } } : {}) }} />
        </>
      )}
    </PageFrame>
  );
}
