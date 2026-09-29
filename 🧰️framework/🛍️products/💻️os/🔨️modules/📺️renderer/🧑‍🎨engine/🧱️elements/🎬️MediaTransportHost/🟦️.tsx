/// <reference types="vitest/importMeta" />
import { createContext, useContext, useEffect, useMemo, useRef, useState, type ReactElement } from "react";
import { MEDIA_TRANSPORT_EXTENSION_ID, parseMediaTransportProps, type MediaTransportProps } from "../../../../🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts";
import { collectMediaTransportBytes, type MediaTransportPort } from "../../🎬️media/🚚️lifecycle/🟦️.ts";

export { MEDIA_TRANSPORT_EXTENSION_ID };

export type { MediaTransportPort } from "../../🎬️media/🚚️lifecycle/🟦️.ts";

export type MediaTransportOwner = {
  readonly instanceId: number;
  readonly controllerId: string;
  readonly windowId: string;
  readonly port: MediaTransportPort;
};

export const MediaTransportOwnerContext = createContext<MediaTransportOwner | null>(null);

type MediaTransportRun = { cancelRequested: boolean; admitted: boolean };

type TransportState =
  | { readonly kind: "loading"; readonly progress: bigint; readonly detail: string }
  | { readonly kind: "ready"; readonly url: string; readonly mimeType: string }
  | { readonly kind: "unsupported"; readonly detail: string }
  | { readonly kind: "failed"; readonly detail: string };

function parseProps(value: unknown): MediaTransportProps | null {
  try {
    return parseMediaTransportProps(value);
  } catch {
    return null;
  }
}

function invalidContractLabel(value: unknown): string {
  const locale = typeof value === "object" && value !== null && "locale" in value ? (value as { readonly locale?: unknown }).locale : null;
  if (locale === "de") return "Ungültiger Medienvertrag.";
  if (locale === "en") return "Invalid media contract.";
  return `${MEDIA_TRANSPORT_EXTENSION_ID} · invalid-contract`;
}

/** 🎬️ Owns one revision-bound media export and a real browser audio/video element. */
export function MediaTransportHost({ value, nodeId, nodeKey }: { readonly value: unknown; readonly nodeId: string | number; readonly nodeKey: string }): ReactElement {
  const owner = useContext(MediaTransportOwnerContext);
  const props = useMemo(() => parseProps(value), [value]);
  const mediaRef = useRef<HTMLMediaElement>(null);
  const runRef = useRef<MediaTransportRun | null>(null);
  const objectUrlRef = useRef<string | null>(null);
  const [state, setState] = useState<TransportState>(() => props ? { kind: props.capability.status === "unsupported" ? "unsupported" : "loading", progress: 0n, detail: props.capability.reason ?? "" } : { kind: "failed", detail: invalidContractLabel(value) });
  const [positionMs, setPositionMs] = useState(props?.positionMs ?? 0);
  const [durationMs, setDurationMs] = useState<number | null>(props?.durationMs ?? null);
  const [selection, setSelection] = useState<[number, number] | null>(props?.selectionStartMs != null && props.selectionEndMs != null ? [props.selectionStartMs, props.selectionEndMs] : null);
  const [playing, setPlaying] = useState(false);
  const failPlayback = (detail: string): void => {
    const current = mediaRef.current;
    if (current) {
      current.pause();
      current.removeAttribute("src");
      current.load();
    }
    const url = objectUrlRef.current;
    objectUrlRef.current = null;
    if (url) URL.revokeObjectURL(url);
    setState({ kind: "failed", detail });
  };

  useEffect(() => {
    setPositionMs(props?.positionMs ?? 0);
    setDurationMs(props?.durationMs ?? null);
    setSelection(props?.selectionStartMs != null && props.selectionEndMs != null ? [props.selectionStartMs, props.selectionEndMs] : null);
  }, [props]);

  useEffect(() => {
    let active = true;
    const run: MediaTransportRun = { cancelRequested: false, admitted: false };
    runRef.current = run;
    const previousUrl = objectUrlRef.current;
    objectUrlRef.current = null;
    const media = mediaRef.current;
    if (media) {
      media.pause();
      media.removeAttribute("src");
      media.load();
    }
    if (previousUrl) URL.revokeObjectURL(previousUrl);
    setPlaying(false);
    if (!props) {
      setState({ kind: "failed", detail: invalidContractLabel(value) });
      return () => { active = false; };
    }
    if (props.capability.status === "unsupported") {
      setState({ kind: "unsupported", detail: props.capability.reason ?? props.labels.unsupported });
      return () => { active = false; };
    }
    if (!owner || !props.resource) {
      setState(props.capability.status === "loading" ? { kind: "loading", progress: 0n, detail: props.capability.reason ?? props.labels.loading } : { kind: "unsupported", detail: props.labels.unsupported });
      return () => { active = false; };
    }
    if (props.resource.controllerId !== owner.controllerId || props.resource.appInstanceId !== owner.instanceId) {
      setState({ kind: "failed", detail: `${MEDIA_TRANSPORT_EXTENSION_ID} · owner-mismatch` });
      return () => { active = false; };
    }
    const revision = BigInt(props.revision);
    const generation = BigInt(props.resource.generation);
    const parentDocumentId = props.resource.parentDocumentId;
    setState({ kind: "loading", progress: 0n, detail: props.labels.loading });
    void (async () => {
      try {
        const bytes = await collectMediaTransportBytes({
          instanceId: owner.instanceId,
          parentDocumentId,
          revision,
          generation,
          mediaType: props.mediaType,
          port: owner.port,
          cancelled: () => run.cancelRequested || !active,
          progress: (status) => {
            run.admitted = true;
            if (active) setState({ kind: "loading", progress: status.applied_progress, detail: status.detail || props.labels.loading });
          },
        });
        run.admitted = false;
        if (!bytes || run.cancelRequested || !active) return;
        const url = URL.createObjectURL(new Blob(bytes.chunks.map((chunk) => chunk.slice().buffer as ArrayBuffer), { type: bytes.mediaType }));
        if (!active) {
          URL.revokeObjectURL(url);
          return;
        }
        objectUrlRef.current = url;
        setState({ kind: "ready", url, mimeType: bytes.mediaType });
      } catch (error) {
        run.admitted = false;
        if (active) setState({ kind: "failed", detail: error instanceof Error ? error.message : String(error) });
      }
    })();
    return () => {
      active = false;
      run.cancelRequested = true;
      if (runRef.current === run) runRef.current = null;
      const url = objectUrlRef.current;
      objectUrlRef.current = null;
      const current = mediaRef.current;
      if (current) {
        current.pause();
        current.removeAttribute("src");
        current.load();
      }
      if (url) URL.revokeObjectURL(url);
    };
  }, [owner?.controllerId, owner?.instanceId, owner?.port, owner?.windowId, props]);

  if (!props) return <div role="alert" data-media-state="invalid" data-ui-node-id={nodeId} data-ui-node-key={nodeKey}>{invalidContractLabel(value)}</div>;
  if (state.kind === "unsupported" || state.kind === "failed") return <div role="status" data-media-state={state.kind} data-ui-node-id={nodeId} data-ui-node-key={nodeKey}>{state.detail || props.labels.unsupported}</div>;
  if (state.kind === "loading") return (
    <div role="status" aria-live="polite" data-media-state="loading" data-ui-node-id={nodeId} data-ui-node-key={nodeKey} className="flex min-h-0 flex-col gap-single p-single">
      <span>{state.detail || props.labels.loading}</span>
      <progress aria-label={props.labels.progress} />
      {runRef.current?.admitted ? <button type="button" onClick={() => { if (runRef.current) runRef.current.cancelRequested = true; setState({ kind: "loading", progress: state.progress, detail: props.labels.cancel }); }}>{props.labels.cancel}</button> : null}
    </div>
  );
  const knownDuration = durationMs ?? props.durationMs;
  const maximum = Math.max(1, knownDuration ?? 1);
  const currentSelection = selection ?? [0, maximum];
  const mediaProps = {
    src: state.url,
    preload: "metadata" as const,
    "aria-label": props.kind === "audio" ? props.labels.audio : props.labels.video,
    onPlay: () => setPlaying(true),
    onPause: () => setPlaying(false),
    onEnded: () => setPlaying(false),
    onTimeUpdate: () => setPositionMs(Math.round((mediaRef.current?.currentTime ?? 0) * 1000)),
    onLoadedMetadata: () => { if (mediaRef.current) mediaRef.current.currentTime = props.positionMs / 1000; },
    onDurationChange: () => { const duration = mediaRef.current?.duration; if (duration != null && Number.isFinite(duration)) setDurationMs(Math.max(1, Math.round(duration * 1000))); },
    onError: () => failPlayback(props.labels.unsupported),
  };
  return (
    <div data-media-state="ready" data-media-window={owner?.windowId} data-ui-node-id={nodeId} data-ui-node-key={nodeKey} className="flex min-h-0 flex-col gap-single p-single" style={{ height: props.hostContentHeight }}>
      {props.kind === "audio" ? <audio ref={(element) => { mediaRef.current = element; }} {...mediaProps} /> : <video ref={(element) => { mediaRef.current = element; }} {...mediaProps} className="min-h-0 w-full flex-1" />}
      <div className="flex items-center gap-single">
        <button type="button" aria-label={playing ? props.labels.pause : props.labels.play} onClick={() => { const media = mediaRef.current; if (!media) return; if (media.paused) void media.play().catch((error: unknown) => { if (!(error instanceof DOMException && error.name === "AbortError")) failPlayback(props.labels.unsupported); }); else media.pause(); }}>{playing ? props.labels.pause : props.labels.play}</button>
        <label className="min-w-0 flex-1">{props.labels.seek}<input type="range" min={0} max={maximum} value={Math.min(maximum, positionMs)} aria-label={props.labels.seek} onChange={(event) => { const next = Number(event.currentTarget.value); setPositionMs(next); if (mediaRef.current) mediaRef.current.currentTime = next / 1000; }} /></label>
        <output aria-label={props.labels.position}>{positionMs}</output>
        <span aria-label={props.labels.duration}>{knownDuration ?? props.labels.unknownDuration}</span>
      </div>
      {knownDuration !== null ? <div className="grid grid-cols-2 gap-single">
        <label>{props.labels.selectionStart}<input type="range" min={0} max={Math.max(0, currentSelection[1] - 1)} value={currentSelection[0]} aria-label={props.labels.selectionStart} onChange={(event) => setSelection([Number(event.currentTarget.value), currentSelection[1]])} /></label>
        <label>{props.labels.selectionEnd}<input type="range" min={Math.min(maximum, currentSelection[0] + 1)} max={maximum} value={currentSelection[1]} aria-label={props.labels.selectionEnd} onChange={(event) => setSelection([currentSelection[0], Number(event.currentTarget.value)])} /></label>
      </div> : null}
    </div>
  );
}
