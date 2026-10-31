/** 🖼️ Live previews of catalog rows: a figure on the ambient clock when the asset is ready, and a designed state when it is pending, missing, invalid or fails to draw.
 *
 * Previews follow the surface theme (light ink on the dark surface, dark ink on the light one) and never draw dimension lines or labels: the card around them names the asset.
 *
 * @see ../🧍️figure/🟦️.tsx — the figure
 * @see ../🧭️context/🟦️.tsx — the ambient clock
 */
import { Component, useEffect, useMemo, useState, type ErrorInfo, type ReactElement, type ReactNode } from "react";
import type { Asset, EvaluationQuery, View } from "@teaching/staffage";
import { StaffageFigure } from "../🧍️figure/🟦️.tsx";
import { useArchive } from "../🧭️context/🟦️.tsx";
import { type StaffageClock } from "../⏱️clock/🟦️.ts";
import { Button, Notice, cn } from "../🪟️chrome/🟦️.tsx";
import { useCatalog } from "../📚️catalog/🟦️.ts";
import type { Row } from "../🔎️search/🟦️.ts";

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

/** 🖼️ The drawing view that reads on the current surface. */
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
  if (row.status === "pending") return <Notice kind="loading" compact={compact} className={className} title={t("state.loading.title")} />;
  if (row.status === "failed")
    return (
      <Notice kind="error" compact={compact} className={className} title={t("state.categoryFailed.title")} actions={compact ? undefined : <Button icon="rotate-cw" onClick={() => void catalog.retryCategory(row.entry.category)}>{t("action.retry")}</Button>}>
        {compact ? null : snapshot.categories[row.entry.category].failure?.[language]}
      </Notice>
    );
  if (row.status === "missing")
    return (
      <Notice kind="unavailable" compact={compact} className={className} title={t("state.missing.title")}>
        {compact ? null : t("state.missing.body")}
      </Notice>
    );
  const issues = row.issues ?? [];
  return (
    <Notice kind="error" compact={compact} className={className} title={t("state.invalid.title")}>
      {compact ? (
        issues[0]?.message[language]
      ) : (
        <>
          <p className="m-0">{t("state.invalid.body", { count: issues.length })}</p>
          <ul className="m-0 mt-single list-disc pl-4">
            {issues.slice(0, 5).map((issue, index) => (
              <li key={`${issue.path}-${index}`}>
                <code className="font-mono">{issue.path}</code> · {issue.message[language]}
              </li>
            ))}
          </ul>
        </>
      )}
    </Notice>
  );
}

const QUIET = { dimensions: false, scaleBar: false, legend: false, labels: false, ground: true } as const;

function loopedClock(clock: StaffageClock, durationS: number): StaffageClock {
  const project = () => {
    const state = clock.state;
    const wrapped = ((state.timeS % durationS) + durationS) % durationS;
    const frame = Math.floor(wrapped * state.fps + 1e-9);
    return { ...state, timeS: frame / state.fps, frame };
  };
  const snapshot = () => project();
  return new Proxy(clock, {
    get(target, key) {
      if (key === "state") return project();
      if (key === "timeS") return project().timeS;
      if (key === "getSnapshot") return snapshot;
      return Reflect.get(target, key, target);
    },
  });
}

function usePreviewClock(asset: Asset, clock: StaffageClock): StaffageClock {
  const clip = asset.clips.find((candidate) => candidate.id === asset.presentation.defaultClip);
  const durationS = clip !== undefined && clip.loop && clip.durationS > 0 ? clip.durationS : 0;
  return useMemo(() => (durationS > 0 ? loopedClock(clock, durationS) : clock), [clock, durationS]);
}

function ReadyPreview(props: { readonly asset: Asset; readonly solid?: boolean; readonly scale: number; readonly query?: EvaluationQuery; readonly className?: string; readonly compact?: boolean; readonly view?: View }): ReactElement {
  const { asset, solid, scale, query, className, compact, view } = props;
  const { language, ambient } = useArchive();
  const dark = useDarkSurface();
  const clock = usePreviewClock(asset, ambient);
  return (
    <div aria-hidden="true" inert className={cn("relative h-full min-h-0 w-full overflow-hidden", className)} data-preview={asset.id}>
      <FigureBoundary resetKey={asset} fallback={(message) => <Notice kind="error" compact title={asset.title[language]}>{compact ? null : message}</Notice>}>
        <StaffageFigure asset={asset} {...(query === undefined ? {} : { query })} clock={clock} view={view ?? surfaceView(dark, solid)} scale={{ denominator: scale }} language={language} fit="contain" transparent annotations={QUIET} />
      </FigureBoundary>
    </div>
  );
}

/** 🖼️ The live preview of a row, fitted to its box. */
export function RowPreview(props: { readonly row: Row; readonly solid?: boolean; readonly scale?: number; readonly query?: EvaluationQuery; readonly className?: string; readonly compact?: boolean; readonly view?: View }): ReactElement {
  const { row, solid, scale = 100, query, className, compact, view } = props;
  const { language } = useArchive();
  if (row.status !== "ready" || row.asset === undefined) return row.status === "pending" ? <div role="status" aria-busy="true" aria-label={row.entry.title[language]} className={cn("h-full min-h-12 w-full animate-pulse bg-muted/60 motion-reduce:animate-none", className)} /> : <UnavailableNotice row={row} compact className={cn("h-full", className)} />;
  return <ReadyPreview asset={row.asset} {...(solid === undefined ? {} : { solid })} scale={scale} {...(query === undefined ? {} : { query })} {...(className === undefined ? {} : { className })} {...(compact === undefined ? {} : { compact })} {...(view === undefined ? {} : { view })} />;
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
