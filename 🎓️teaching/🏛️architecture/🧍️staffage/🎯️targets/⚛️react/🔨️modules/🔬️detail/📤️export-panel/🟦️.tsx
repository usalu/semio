/** 📤️ The export controls of an asset: every format with its honest state in this browser, options, progress with cancel, and a spoken result.
 *
 * Capabilities are probed once from the real runtime: an unsupported browser feature (WebM, MP4, canvas) is shown as `unsupported` with the reason, a missing writer (the DWG WebAssembly writer) as `unavailable`.
 * One export runs at a time; cancelling aborts it through the job's signal. `capabilities`, `host`, `exporter` and `save` are seams for hosts and tests.
 *
 * @see ../../📤️downloads/🟦️.ts — `exportFigure`, `exportCapabilities`, `saveExport`
 */
import { useEffect, useRef, useState, type ReactElement } from "react";
import { formatNumber } from "@teaching/staffage";
import type { Asset, EvaluationQuery, Language, Text } from "@teaching/staffage";
import { exportCapabilities, exportFigure, saveExport } from "../../📤️downloads/🟦️.ts";
import type { ExportCapabilities, ExportFormat, ExportHost, ExportResult } from "../../📤️downloads/🟦️.ts";
import type { DepictionRequest } from "../../🧮️pipeline/🟦️.ts";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { Button, Notice, NumberField, ProgressNotice, SelectField, Tag } from "../../🪟️chrome/🟦️.tsx";

const FORMATS: readonly ExportFormat[] = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm"];
const FRAMED: readonly ExportFormat[] = ["png-sequence", "mp4", "webm"];

type Outcome = { readonly kind: "done"; readonly result: Extract<ExportResult, { status: "done" }> } | { readonly kind: "cancelled"; readonly format: ExportFormat } | { readonly kind: "unavailable"; readonly format: ExportFormat; readonly reason: Text } | { readonly kind: "error"; readonly message: string };
type Running = { readonly format: ExportFormat; readonly done: number; readonly total: number; readonly label: string };

/** 📤️ The props of {@link ExportPanel}. */
export type ExportPanelProps = {
  readonly asset: Asset;
  readonly query: EvaluationQuery;
  readonly request: DepictionRequest;
  readonly language: Language;
  readonly capabilities?: ExportCapabilities;
  readonly host?: ExportHost;
  readonly exporter?: typeof exportFigure;
  readonly save?: (result: ExportResult) => void;
  readonly options?: ExportSettings;
  readonly onOptions?: (settings: ExportSettings) => void;
};

/** 🎛️ The export settings a link may carry: PDF sheet, frame rate and duration (seconds, `null` = the clip). */
export type ExportSettings = { readonly sheet: "single" | "all-views"; readonly fps: number | null; readonly durationS: number | null };

/** 📤️ The export panel. */
export function ExportPanel(props: ExportPanelProps): ReactElement {
  const { asset, query, request, language, host, exporter = exportFigure, save = saveExport } = props;
  const { t } = useArchive();
  const [probed, setProbed] = useState<ExportCapabilities | null>(props.capabilities ?? null);
  const [running, setRunning] = useState<Running | null>(null);
  const [copied, setCopied] = useState<"idle" | "done" | "blocked">("idle");
  const [outcome, setOutcome] = useState<Outcome | null>(null);
  const [sheet, setSheetState] = useState<"single" | "all-views">(props.options?.sheet ?? "single");
  const clipDuration = query.kind === "clip" ? (asset.clips.find((clip) => clip.id === query.clip)?.durationS ?? 1) : 1;
  const [fps, setFpsState] = useState<number | null>(props.options?.fps ?? 30);
  const [duration, setDurationState] = useState<number | null>(props.options?.durationS ?? null);
  const setSheet = (next: "single" | "all-views"): void => { setSheetState(next); props.onOptions?.({ sheet: next, fps, durationS: duration }); };
  const setFps = (next: number | null): void => { setFpsState(next); props.onOptions?.({ sheet, fps: next, durationS: duration }); };
  const setDuration = (next: number | null): void => { setDurationState(next); props.onOptions?.({ sheet, fps, durationS: next }); };
  const controller = useRef<AbortController | null>(null);
  useEffect(() => {
    if (props.capabilities !== undefined) return setProbed(props.capabilities);
    let live = true;
    void exportCapabilities(host).then((found) => live && setProbed(found));
    return () => {
      live = false;
    };
  }, [host, props.capabilities]);
  useEffect(() => () => controller.current?.abort(), []);
  const run = async (format: ExportFormat): Promise<void> => {
    if (running !== null) return;
    const job = new AbortController();
    controller.current = job;
    setOutcome(null);
    setRunning({ format, done: 0, total: 1, label: "" });
    const options = { ...request, sheet, ...(FRAMED.includes(format) ? { fps: Math.min(60, Math.max(12, fps ?? 30)), durationS: duration ?? clipDuration } : {}) };
    try {
      const result = await exporter(asset, query, options, format, { signal: job.signal, progress: (done, total, label) => setRunning({ format, done, total, label: label[language] }) }, host);
      if (result.status === "done") {
        save(result);
        setOutcome({ kind: "done", result });
      } else if (result.status === "cancelled") setOutcome({ kind: "cancelled", format });
      else setOutcome({ kind: "unavailable", format, reason: result.reason });
    } catch (error) {
      setOutcome(job.signal.aborted ? { kind: "cancelled", format } : { kind: "error", message: "" });
    } finally {
      setRunning(null);
    }
  };
  const offered = FORMATS.filter((format) => asset.exports.includes(format));
  const snippet = `<staffage-figure asset="${asset.id}" view="${request.view}" scale="${request.scale.denominator}" lang="${language}"></staffage-figure>`;
  const needsClip = query.kind === "pose";
  return (
    <section aria-labelledby="detail-export" className="flex flex-col gap-double" data-section="export">
      <h2 id="detail-export" className="m-0 font-serif text-lg">{t("detail.export.title")}</h2>
      <p className="m-0 max-w-[70ch] text-xs leading-normal text-muted-foreground">{t("detail.export.lede", { view: request.view, scale: request.scale.denominator })}</p>
      {probed === null ? <Notice kind="loading" compact title={t("detail.export.probing")} /> : (
        <ul className="m-0 grid list-none gap-double p-0 md:grid-cols-2" data-formats="">
          {offered.map((format) => {
            const capability = probed[format];
            const kind = capability.available ? null : format === "dwg" ? "unavailable" : "unsupported";
            const blocked = capability.available && FRAMED.includes(format) && needsClip;
            return (
              <li key={format} data-format={format} data-state={kind ?? (blocked ? "needs-clip" : "available")} className="flex min-w-0 flex-col gap-single border border-border bg-panel p-double">
                <div className="flex flex-wrap items-center justify-between gap-single">
                  <h3 className="m-0 text-sm font-semibold">{t(`detail.format.${format}`)}</h3>
                  <Tag>{t(capability.available ? "detail.export.available" : kind === "unavailable" ? "detail.export.unavailable" : "detail.export.unsupported")}</Tag>
                </div>
                <p className="m-0 text-xs leading-normal text-muted-foreground">{t(`detail.format.${format}.body`)}</p>
                {kind === null ? null : <Notice kind={kind} compact title={t(kind === "unavailable" ? "detail.export.unavailableTitle" : "detail.export.unsupportedTitle")}>{capability.reason?.[language]}</Notice>}
                {blocked ? <Notice kind="info" compact title={t("detail.export.needsClip")} /> : null}
                {capability.available && format === "pdf" ? <SelectField label={t("detail.export.sheet")} value={sheet} onChange={setSheet} options={[{ value: "single", label: t("detail.export.sheetSingle") }, { value: "all-views", label: t("detail.export.sheetAll") }]} /> : null}
                {capability.available && FRAMED.includes(format) && !needsClip ? (
                  <div className="flex gap-double">
                    <NumberField label={t("detail.export.fps")} value={fps} min={12} max={60} step={1} onChange={setFps} />
                    <NumberField label={t("detail.export.duration")} unit="s" value={duration ?? Number(formatNumber(clipDuration, 2, "en"))} min={0.1} step={0.1} onChange={setDuration} />
                  </div>
                ) : null}
                {capability.available && capability.codec !== undefined ? <p className="m-0 font-mono text-[0.65rem] text-muted-foreground">{t("detail.export.codec", { codec: capability.codec, alpha: t(capability.alpha === true ? "detail.yes" : "detail.no") })}</p> : null}
                <div className="mt-auto pt-single">
                  <Button variant="primary" icon="file-archive" disabled={!capability.available || blocked || running !== null} onClick={() => void run(format)}>
                    {t("detail.export.run", { format: t(`detail.format.${format}`) })}
                  </Button>
                </div>
              </li>
            );
          })}
        </ul>
      )}
      {asset.exports.includes("component") ? (
        <div className="flex flex-col gap-single border border-border bg-panel p-double" data-format="component">
          <h3 className="m-0 text-sm font-semibold">{t("detail.format.component")}</h3>
          <p className="m-0 text-xs text-muted-foreground">{t("detail.format.component.body")}</p>
          <code className="block overflow-x-auto whitespace-pre border border-border bg-background p-double font-mono text-xs">{snippet}</code>
          <div className="flex flex-wrap items-center gap-double"><Button icon="copy" onClick={() => void (navigator.clipboard?.writeText(snippet).then(() => setCopied("done"), () => setCopied("blocked")) ?? setCopied("blocked"))}>{t("detail.export.copy")}</Button><span role="status" aria-live="polite" className="text-xs">{copied === "done" ? t("detail.export.copied") : copied === "blocked" ? t("detail.export.copyBlocked") : ""}</span></div>
        </div>
      ) : null}
      {running === null ? null : <ProgressNotice label={t("detail.export.running", { format: t(`detail.format.${running.format}`) })} done={running.done} total={running.total} cancelLabel={t("action.cancel")} onCancel={() => controller.current?.abort()} />}
      <div role="status" aria-live="polite" data-result="">
        {outcome === null ? null : outcome.kind === "done" ? (
          <Notice kind="info" compact title={t("detail.export.done", { name: outcome.result.filename })}>
            <p className="m-0 font-mono">{t("detail.export.size", { kb: formatNumber(outcome.result.bytes.length / 1024, 1, language) })}{outcome.result.details.codec === undefined ? "" : ` · ${outcome.result.details.codec}`}{outcome.result.details.frames === undefined ? "" : ` · ${t("detail.export.frames", { frames: outcome.result.details.frames })}`}{outcome.result.details.pages === undefined ? "" : ` · ${t("detail.export.pages", { pages: outcome.result.details.pages })}`}{outcome.result.details.alpha === undefined ? "" : ` · ${t("detail.export.alpha", { alpha: t(outcome.result.details.alpha ? "detail.yes" : "detail.no") })}`}</p>
            {outcome.result.details.notes.map((note, index) => <p key={index} className="m-0">{note[language]}</p>)}
          </Notice>
        ) : outcome.kind === "cancelled" ? (
          <Notice kind="info" compact title={t("detail.export.cancelled")} />
        ) : outcome.kind === "unavailable" ? (
          <Notice kind="unavailable" compact title={t("detail.export.unavailableTitle")}>{outcome.reason[language]}</Notice>
        ) : (
          <Notice kind="error" compact title={t("detail.export.failed")}>{t("detail.export.failedBody")}</Notice>
        )}
      </div>
    </section>
  );
}
