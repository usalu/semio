/** 🖼️ Live previews of catalog rows: a figure on the ambient clock when the asset is ready, and a designed state when it is pending, missing, invalid or fails to draw.
 *
 * Previews follow the surface theme (light ink on the dark surface, dark ink on the light one) and never draw dimension lines or labels: the card around them names the asset.
 *
 * @see ../🧮️pipeline/🟦️.ts — the figure drawing
 * @see ../🧭️context/🟦️.tsx — the ambient clock
 */
import { Component, useEffect, useLayoutEffect, useMemo, useRef, useState, type ErrorInfo, type ReactElement, type ReactNode } from "react";
import type { Asset, EvaluationQuery, Issue, Language, View, PlanarViewpoint } from "@teaching/staffage";
import { defaultQuery, depictionOptions, queryAt } from "../🧮️pipeline/🟦️.ts";
import { drawFigure, type Drawing } from "../🧮️pipeline/🏁️drawing/🟦️.ts";
import { retainDrawing } from "../🧮️pipeline/🏁️drawing/queue.ts";
import { writeSvgMarkup } from "../🧮️pipeline/🧬️svg/🟦️.ts";
import { driveFigure } from "../⏱️clock/🏁️frames/🟦️.ts";
import { word } from "../🗣️labels/🟦️.ts";
import { useArchive } from "../🧭️context/🟦️.tsx";
import { type StaffageClock } from "../⏱️clock/🟦️.ts";
import { Button, Notice, cn } from "../🪟️chrome/🟦️.tsx";
import { useCatalog } from "../📚️catalog/🟦️.ts";
import type { Row } from "../🔎️search/🟦️.ts";
import { VIEWPOINT_PHRASES, resolveViewpoint, viewpointWord } from "../🔭️viewpoints/🟦️.ts";

/** 🌑️ Whether the surface is dark now, following the theme switch and the device. */
export function useDarkSurface(): boolean {
  const [dark, setDark] = useState(() => document.documentElement.classList.contains("dark"));
  useEffect(() => {
    const update = (): void => setDark(document.documentElement.classList.contains("dark"));
    update();
    const observer = new MutationObserver(update);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });
    return () => observer.disconnect();
  }, []);
  return dark;
}

/** 🔭️ The drawing view that reads on the current surface. */
export const surfaceView = (dark: boolean, solid = false): View => (dark ? (solid ? "dark-solid" : "dark-line") : solid ? "light-solid" : "light-line");

/** 🧯️ Catches a figure that fails to draw and shows why, so one broken asset never takes a page down. */
export class FigureBoundary extends Component<{ readonly fallback: (message: string) => ReactNode; readonly children: ReactNode; readonly resetKey?: unknown }, { readonly message: string | null }> {
  override state = { message: null as string | null };

  static getDerivedStateFromError(error: unknown): { message: string } {
    return { message: error instanceof Error ? error.message : String(error) };
  }

  override componentDidUpdate(previous: { readonly resetKey?: unknown }): void {
    if (previous.resetKey !== this.props.resetKey && this.state.message !== null) this.setState({ message: null });
  }

  override componentDidCatch(_error: unknown, _info: ErrorInfo): void {}

  override render(): ReactNode {
    return this.state.message === null ? this.props.children : this.props.fallback(this.state.message);
  }
}

/** 🧾️ The reason a row is unavailable, in words. */
export function UnavailableNotice(props: { readonly row: Row; readonly compact?: boolean; readonly className?: string }): ReactElement {
  const { t, language, catalog } = useArchive();
  const snapshot = useCatalog(catalog);
  const { row, compact, className } = props;
  if (row.status === "pending") return <Notice kind="loading" compact={compact} plain={compact === true} className={className} title={t("state.loading.title")} />;
  if (row.status === "failed")
    return (
      <Notice kind="error" compact={compact} plain={compact === true} className={className} title={t("state.categoryFailed.title")} actions={compact ? undefined : <Button icon="rotate-cw" onClick={() => void catalog.retryCategory(row.entry.category)}>{t("action.retry")}</Button>}>
        {compact ? null : snapshot.categories[row.entry.category].failure?.[language]}
      </Notice>
    );
  if (row.status === "missing")
    return (
      <Notice kind="unavailable" compact={compact} plain={compact === true} className={className} title={t("state.missing.title")}>
        {compact ? null : t("state.missing.body")}
      </Notice>
    );
  const issues = row.issues ?? [];
  return (
    <Notice kind="error" compact={compact} plain={compact === true} className={className} title={t("state.invalid.title")}>
      {compact ? (
        issues[0]?.message[language]
      ) : (
        <>
          <p className="m-0">{t("state.invalid.body", { count: issues.length })}</p>
          <IssueList issues={issues.slice(0, 5)} language={language} />
        </>
      )}
    </Notice>
  );
}

function IssueList({ issues, language }: { readonly issues: readonly Issue[]; readonly language: Language }): ReactElement {
  return (
    <ul className="m-0 mt-single list-disc pl-4">
      {issues.map((issue, index) => (
        <li key={`${issue.path}-${index}`}>
          <code className="font-mono">{issue.path}</code> · {issue.message[language]}
        </li>
      ))}
    </ul>
  );
}

/** 🩺️ The findings of a check that ended after its asset was offered: the first of them, and all of them on request. The asset stays shown beside this notice.
 * @see ../📚️catalog/🟦️.ts — `Row.findings` */
export function FindingsNotice({ findings }: { readonly findings: readonly Issue[] }): ReactElement {
  const { t, language } = useArchive();
  return (
    <Notice kind="error" compact data={{ "data-asset-state": "findings" }} title={t("state.findings.title", { count: findings.length })}>
      <p className="m-0">{t("state.findings.body")}</p>
      <IssueList issues={findings.slice(0, 5)} language={language} />
      {findings.length <= 5 ? null : (
        <details>
          <summary className="cursor-pointer">{t("state.findings.all", { count: findings.length })}</summary>
          <div className="max-h-[40vh] overflow-auto"><IssueList issues={findings} language={language} /></div>
        </details>
      )}
    </Notice>
  );
}

const QUIET = { dimensions: false, scaleBar: false, legend: false, labels: false, ground: true } as const;

type PreviewDrawingProps = { readonly asset: Asset; readonly clock: StaffageClock; readonly query?: EvaluationQuery; readonly view: View; readonly viewpoint: PlanarViewpoint; readonly scale: number; readonly language: Language };

/** 🖌️ Updates the ink of an inert archive preview. Its card names the asset; the drawing needs no per-frame layout or accessibility attributes. */
function PreviewDrawing({ asset, clock, query, view, viewpoint, scale, language }: PreviewDrawingProps): ReactElement {
  const ink = useRef<HTMLDivElement>(null);
  const [failure, setFailure] = useState<{ error: unknown } | null>(null);
  const [fidelity, setFidelity] = useState<Drawing["fidelity"]>();
  const drawing = useMemo(() => {
    const options = depictionOptions({ view, viewpoint, scale: { denominator: scale }, language, transparent: true, annotations: QUIET });
    const initial = query ?? defaultQuery(asset, 0);
    const clip = asset.clips.find(candidate => candidate.id === asset.presentation.defaultClip);
    const duration = clip?.loop ? clip.durationS : 0;
    const time = () => {
      const { frame, fps, timeS } = clock.state;
      return duration > 0 ? Math.floor((((timeS % duration) + duration) % duration) * fps + 1e-9) / fps : frame / fps;
    };
    return { initial, options, time };
  }, [asset, clock, query, view, viewpoint, scale, language]);
  useLayoutEffect(() => {
    const element = ink.current;
    if (element === null) return;
    let shown: Drawing["fidelity"] | undefined;
    const receive = (next: Drawing) => {
      writeSvgMarkup(element, next.markup);
      if (shown !== next.fidelity) setFidelity((shown = next.fidelity));
    };
    const fail = (error: unknown) => setFailure({ error });
    const lease = clock.drawBudgetMs > 0 ? retainDrawing(asset, drawing.options, false, receive, fail) : null;
    let lastTime = Number.NaN;
    const step = () => {
      const now = drawing.time();
      if (now === lastTime) return;
      lastTime = now;
      const query = queryAt(drawing.initial, now);
      try {
        if (lease) lease.request(query);
        else receive(drawFigure(asset, drawing.options, query));
      } catch (error) {
        fail(error);
      }
    };
    step();
    const cancelPending = () => {
      lastTime = Number.NaN;
      lease?.cancelPending();
    };
    const stop = drawing.initial.kind === "pose" ? undefined : driveFigure(clock, { element, animated: () => true, step, cancelPending });
    return () => { stop?.(); lease?.release(); };
  }, [asset, clock, drawing]);
  if (failure !== null) throw failure.error;
  return (
    <div data-staffage-figure="" data-fit="contain" data-asset={asset.id} data-view={view} data-viewpoint={viewpoint} data-fidelity={fidelity} className="absolute inset-0 min-h-0 min-w-0 overflow-hidden">
      <div ref={ink} className="absolute inset-0" />
      {fidelity === "conventional-symbol" ? <span data-fidelity-note="" style={{ position: "absolute", left: 2, bottom: 2, padding: "0 3px", fontSize: 10, lineHeight: 1.4, background: "rgba(127,127,127,0.25)", color: "inherit" }}>{word("conventionalSymbol", language)}</span> : null}
    </div>
  );
}

function ReadyPreview(props: { readonly asset: Asset; readonly viewpoint: PlanarViewpoint; readonly solid?: boolean; readonly scale: number; readonly query?: EvaluationQuery; readonly className?: string; readonly compact?: boolean; readonly view?: View }): ReactElement {
  const { asset, viewpoint, solid, scale, query, className, compact, view } = props;
  const { language, ambient } = useArchive();
  const dark = useDarkSurface();
  const resolution = useMemo(() => resolveViewpoint(asset, viewpoint, scale, language), [asset, viewpoint, scale, language]);
  if (!resolution.available) return <Notice kind="unavailable" compact plain className={cn("h-full overflow-hidden", className)} title={viewpointWord("viewpoints.unavailable", language, { view: VIEWPOINT_PHRASES[viewpoint][language] })}>{compact ? null : resolution.message[language]}</Notice>;
  return (
    <div aria-hidden="true" inert className={cn("relative h-full min-h-0 w-full overflow-hidden", className)} data-preview={asset.id}>
      <FigureBoundary resetKey={asset} fallback={(message) => <Notice kind="error" compact plain title={asset.title[language]}>{compact ? null : message}</Notice>}>
        <PreviewDrawing asset={asset} {...(query === undefined ? {} : { query })} clock={ambient} view={view ?? surfaceView(dark, solid)} viewpoint={viewpoint} scale={scale} language={language} />
      </FigureBoundary>
    </div>
  );
}

/** 👁️ The live preview of a row, fitted to its box. */
export function RowPreview(props: { readonly row: Row; readonly viewpoint: PlanarViewpoint; readonly solid?: boolean; readonly scale?: number; readonly query?: EvaluationQuery; readonly className?: string; readonly compact?: boolean; readonly view?: View }): ReactElement {
  const { row, viewpoint, solid, scale = 100, query, className, compact, view } = props;
  const { language } = useArchive();
  if (row.status !== "ready" || row.asset === undefined) return row.status === "pending" ? <div role="status" aria-busy="true" aria-label={row.entry.title[language]} className={cn("h-full min-h-12 w-full animate-pulse bg-muted/60 motion-reduce:animate-none", className)} /> : <UnavailableNotice row={row} compact className={cn("h-full", className)} />;
  return <ReadyPreview asset={row.asset} viewpoint={viewpoint} {...(solid === undefined ? {} : { solid })} scale={scale} {...(query === undefined ? {} : { query })} {...(className === undefined ? {} : { className })} {...(compact === undefined ? {} : { compact })} {...(view === undefined ? {} : { view })} />;
}

/** 🚨️ The failure of the manifest or of a category, in words, with the way to try again; nothing while all is well. */
export function CatalogProblem(props: { readonly category?: import("@teaching/staffage").CategoryId; readonly children?: ReactNode }): ReactElement | null {
  const { t, language, catalog } = useArchive();
  const snapshot = useCatalog(catalog);
  if (snapshot.manifestState === "failed")
    return (
      <Notice kind="error" title={t("state.manifestFailed.title")} actions={<Button icon="rotate-cw" onClick={() => void catalog.loadManifest()}>{t("action.retry")}</Button>}>
        {t("state.manifestFailed.body")}
      </Notice>
    );
  const failed = props.category === undefined ? undefined : snapshot.categories[props.category];
  if (props.category !== undefined && failed?.state === "failed")
    return (
      <Notice kind="error" title={t("state.categoryFailed.title")} actions={<Button icon="rotate-cw" onClick={() => void catalog.retryCategory(props.category!)}>{t("action.retry")}</Button>}>
        {failed.failure?.[language]}
      </Notice>
    );
  return <>{props.children}</>;
}
