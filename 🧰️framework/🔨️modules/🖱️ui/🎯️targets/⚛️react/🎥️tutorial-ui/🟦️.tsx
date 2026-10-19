import type { TutorialCameraState, TutorialOverlayRect, TutorialGestureCue } from "../../../../🎓️onboarding/🏷️declaration/🟦️.ts";
import { ephemeralMap } from "@semio-tech/framework/transient";
import * as React from "react";
import { useSurface, SurfaceScope, Surface } from "../../../🧱️elements/🌈️Surface/🟦️.tsx";
import { shellFloorPaints, shellFloorFillClass } from "../../../🔨️modules/🏠️shell-floor-presentation/🟦️.ts";
import { useLabel } from "../../../🧱️elements/🏷️Label/🟦️.tsx";
import { Button } from "../../../🧱️elements/🔘️Button/🟦️.tsx";
import { Slider } from "../../../🧱️elements/🎚️Slider/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { GLASS_OVERLAY_BOX_CLASS } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { reactHostPort } from "../../../🧱️elements/🔌️Ports/⚛️react/🟦️.ts";
import { useShellScopeOptional } from "../../../🧱️elements/🐚️ShellScope/🟦️.tsx";
import { type TutorialClockPort, formatTutorialTime, useTutorialClock } from "../🎞️tutorial-engine/🟦️.ts";
import { resolveIntroductionPoint } from "../🧭️introduction-resolve/🟦️.ts";
import { INTRODUCTION_DEMO_GESTURE_DEFAULT_CURSOR, introductionDemoArcPoint } from "../🎬️introduction-demo/🟦️.tsx";

//#region 🎬️TutorialCameraDriver
/** @emoji 🎥️ A live 3D/2D window's imperative camera bridge for tutorial playback — modeled exactly on {@link registerIntroductionSurfaceResolver}. `get` reads the surface's current live pose (for deviation-then-play convergence); `set` writes a pose during playback/seek. */
export type TutorialCameraDriver = {
  readonly get: () => TutorialCameraState | null;
  readonly set: (camera: TutorialCameraState) => void;
};

const tutorialCameraDrivers = ephemeralMap<string, TutorialCameraDriver>("framework.modules.ui.packages.typescript.targets.react.index.tsx.tutorialCameraDrivers");

/** @emoji 🎥️ Registers the tutorial camera driver for window instance `windowId` — call from the window's own host component (e.g. `World3dHost`), unregister on unmount via the returned disposer. */
export function registerTutorialCameraDriver(windowId: string, driver: TutorialCameraDriver): () => void {
  tutorialCameraDrivers.set(windowId, driver);
  return () => {
    if (tutorialCameraDrivers.get(windowId) === driver) tutorialCameraDrivers.delete(windowId);
  };
}

/** @emoji 🎥️ Looks up a registered {@link TutorialCameraDriver}, or `undefined` if that window hasn't mounted/registered one yet. */
export function getTutorialCameraDriver(windowId: string): TutorialCameraDriver | undefined {
  return tutorialCameraDrivers.get(windowId);
}
//#endregion 🎬️TutorialCameraDriver

//#region 🎬️TutorialBar
const TUTORIAL_RATES = [0.5, 1, 1.5, 2] as const;

/** @emoji ⏭️ Cycles through the fixed rate ladder (0.5→1→1.5→2→0.5→…). */
function nextTutorialRate(rate: number): number {
  const index = TUTORIAL_RATES.indexOf(rate as (typeof TUTORIAL_RATES)[number]);
  const next=TUTORIAL_RATES[(index===-1?0:index+1)%TUTORIAL_RATES.length];if(next===undefined)throw Error("Complete tutorial rate ladder required");return next;
}

export type TutorialChapterMarker = { readonly id: string; readonly title: string; readonly atMs: number };

export type TutorialBarProps = {
  readonly title: string;
  readonly durationMs: number;
  readonly playing: boolean;
  readonly rate: number;
  readonly muted: boolean;
  readonly captionsOn: boolean;
  readonly recording: boolean;
  readonly recordAvailable: boolean;
  readonly chapters: readonly TutorialChapterMarker[];
  readonly clock: TutorialClockPort;
  readonly onPlayPause: () => void;
  readonly onStop: () => void;
  readonly onSeek: (ms: number) => void;
  readonly onRateChange: (rate: number) => void;
  readonly onMutedChange: (muted: boolean) => void;
  readonly onCaptionsChange: (captionsOn: boolean) => void;
  readonly onRecordToggle: () => void;
  readonly onAddChapter: () => void;
};

/** @emoji 🎥️ Navbar-style timeline/controls row shown as {@link LayoutProps.subnavbar} whenever a tutorial is active — markup mirrors `Navbar`/`Footer` (`id="ui.tutorial.bar"`, `data-slot="tutorial-bar"`, `data-ui-reveal-region`, `data-elevation-root`). Only this component subscribes to the per-frame `clock` (see {@link useTutorialClock}), so the 60fps scrubber/time readout never re-renders the rest of the shell. */
export const TutorialBar: React.FC<TutorialBarProps> = ({
  title,
  durationMs,
  playing,
  rate,
  muted,
  captionsOn,
  recording,
  recordAvailable,
  chapters,
  clock,
  onPlayPause,
  onStop,
  onSeek,
  onRateChange,
  onMutedChange,
  onCaptionsChange,
  onRecordToggle,
  onAddChapter,
}) => {
  const parent = useSurface();
  const paints = shellFloorPaints(parent);
  const bgClass = shellFloorFillClass(parent);
  const timeMs = useTutorialClock(clock);
  const playLabel = useLabel("tutorial.play");
  const pauseLabel = useLabel("tutorial.pause");
  const stopLabel = useLabel("tutorial.stop");
  const muteLabel = useLabel("tutorial.mute");
  const captionsLabel = useLabel("tutorial.captions");
  const recordLabel = useLabel("tutorial.record");
  const recordingLabel = useLabel("tutorial.recording");
  const addChapterLabel = useLabel("tutorial.addChapter");

  const body = (
    <>
      <div className="p-single flex gap-single items-center min-w-0 h-full">
        <Button id="ui.tutorial.play" icon={playing ? "pause" : "play"} text={playing ? pauseLabel : playLabel} onClick={onPlayPause} />
        <Button id="ui.tutorial.stop" icon="square" text={stopLabel} onClick={onStop} />
        <span id="ui.tutorial.time" data-slot="tutorial-time" className="shrink-0 whitespace-nowrap px-single text-xs tabular-nums text-muted-foreground">
          {formatTutorialTime(timeMs)} / {formatTutorialTime(durationMs)}
        </span>
        <div className="relative min-w-0 flex-1 px-single">
          <Slider id="ui.tutorial.scrubber" min={0} max={Math.max(durationMs, 1)} value={[timeMs]} onValueChange={(values) => onSeek(values[0] ?? 0)} showValue={false} />
          {chapters.map((chapter) => (
            <button
              key={chapter.id}
              id={`ui.tutorial.chapter.${chapter.id}`}
              type="button"
              data-slot="tutorial-chapter-tick"
              title={chapter.title}
              className="pointer-events-auto absolute top-1/2 size-1 -translate-x-1/2 -translate-y-1/2 rounded-full bg-foreground/50 hover:bg-foreground"
              style={{ left: `${durationMs > 0 ? (chapter.atMs / durationMs) * 100 : 0}%` }}
              onClick={() => onSeek(chapter.atMs)}
            />
          ))}
        </div>
        <Button id="ui.tutorial.rate" icon="bar-chart-3" text={`${rate}x`} onClick={() => onRateChange(nextTutorialRate(rate))} />
        <Button id="ui.tutorial.mute" icon="x" text={muteLabel} aria-pressed={muted} onClick={() => onMutedChange(!muted)} />
        <Button id="ui.tutorial.captions" icon="message-square" text={captionsLabel} aria-pressed={captionsOn} onClick={() => onCaptionsChange(!captionsOn)} />
        {recordAvailable && (
          <>
            <Button id="ui.tutorial.record" icon="circle-dot" text={recordLabel} aria-pressed={recording} onClick={onRecordToggle} />
            {recording && (
              <span id="ui.tutorial.recordingIndicator" data-slot="tutorial-recording-indicator" className="whitespace-nowrap px-single text-xs text-destructive">
                {recordingLabel}
              </span>
            )}
          </>
        )}
        <Button id="ui.tutorial.addChapter" icon="plus" text={addChapterLabel} onClick={onAddChapter} />
        <span className="min-w-0 flex-1 truncate px-single text-right text-xs text-muted-foreground">{title}</span>
      </div>
    </>
  );

  return (
    <nav id="ui.tutorial.bar" data-slot="tutorial-bar" data-level="base" data-ui-reveal-region="tutorial-bar" data-elevation-root="" className={cn("relative h-large z-navbar border-t border-border", bgClass)}>
      {paints ? (
        <SurfaceScope level="base" fill="surface">
          {body}
        </SurfaceScope>
      ) : (
        body
      )}
    </nav>
  );
};
//#endregion 🎬️TutorialBar

//#region 🎬️TutorialCaptions
export type TutorialCaptionsProps = {
  readonly text: string | null;
  readonly visible: boolean;
};

/** @emoji 💬️ Bottom-center glass strip showing the active narration cue's caption text — hidden entirely when `visible` is false (captions toggled off) or no cue currently covers the playhead. Reuses {@link GLASS_OVERLAY_BOX_CLASS}'s panel glass tier, positioned bottom-center instead of anchored. */
export const TutorialCaptions: React.FC<TutorialCaptionsProps> = ({ text, visible }) => {
  if (!visible || !text) return null;
  return (
    <Surface id="ui.tutorial.captions.text" data-slot="tutorial-captions" level="dialog" fill="glass" className={cn(GLASS_OVERLAY_BOX_CLASS, "bottom-double left-1/2 max-w-2xl -translate-x-1/2 text-center text-sm")}>
      {text}
    </Surface>
  );
};
//#endregion 🎬️TutorialCaptions

//#region 🎬️TutorialVideoOverlay
export type TutorialVideoOverlayProps = {
  readonly src: string | null;
  readonly rect: TutorialOverlayRect;
  readonly muted: boolean;
  readonly playing: boolean;
  readonly rate: number;
  /** ⏱️ Offset (ms) into THIS cue's own source — the overlay resyncs `video.currentTime` to this whenever it drifts. */
  readonly localTimeMs: number;
};

/** @emoji 📹️ Fixed `<video>` positioned via the active cue's normalized {@link TutorialOverlayRect}, resynced to the playhead every render (hard reseek past a 300ms drift, matching the plan's tolerance) rather than played independently. */
export const TutorialVideoOverlay: React.FC<TutorialVideoOverlayProps> = ({ src, rect, muted, playing, rate, localTimeMs }) => {
  const videoRef = reactHostPort.useRef<HTMLVideoElement | null>(null);
  reactHostPort.useEffect(() => {
    const video = videoRef.current;
    if (!video) return;
    video.muted = muted;
    video.playbackRate = rate;
    const targetSeconds = localTimeMs / 1000;
    if (Math.abs(video.currentTime - targetSeconds) > 0.3) video.currentTime = targetSeconds;
    if (playing) void video.play().catch(() => {});
    else video.pause();
  }, [muted, rate, playing, localTimeMs]);
  if (!src) return null;
  return (
    <video
      id="ui.tutorial.video"
      ref={videoRef}
      data-slot="tutorial-video-overlay"
      src={src}
      className="pointer-events-none fixed z-tutorial rounded-lg object-cover shadow-lg"
      style={{ left: `${rect.x * 100}%`, top: `${rect.y * 100}%`, width: `${rect.width * 100}%`, height: `${rect.height * 100}%` }}
      muted={muted}
      playsInline
    />
  );
};
//#endregion 🎬️TutorialVideoOverlay

export type TutorialGhostPointerProps = {
  /** 🎬️ The gesture cue currently covering the playhead, or `null` between cues. */
  readonly cue: TutorialGestureCue | null;
  /** 📈️ 0–1 progress through `cue`'s own `durationMs`, driven by the tutorial playhead (never the ghost pointer's own clock, unlike `IntroductionDemonstrationOverlay`). */
  readonly progress: number;
};

/** @emoji 👻️ Tutorial-playback ghost cursor — a lean, parallel implementation of `IntroductionDemonstrationOverlay`'s point resolution (reuses {@link resolveIntroductionPoint}/{@link introductionDemoArcPoint}/{@link introductionDemoResolveVisual} verbatim rather than re-deriving them) driven by the tutorial playhead's `progress` instead of its own internal rAF phase machine — click-family/scroll gestures render statically at their point, `drag` lerps linearly from→to, `orbit` bulges along the same quadratic arc the introduction overlay uses. */
export const TutorialGhostPointer: React.FC<TutorialGhostPointerProps> = ({ cue, progress }) => {
  const [point, setPoint] = reactHostPort.useState<{ readonly x: number; readonly y: number } | null>(null);
  const shellScope = useShellScopeOptional();

  reactHostPort.useEffect(() => {
    if (!cue) {
      setPoint(null);
      return;
    }
    const root: Element = shellScope?.rootRef.current ?? document.documentElement;
    const gesture = cue.gesture;
    let next: { readonly x: number; readonly y: number } | null = null;
    switch (gesture.kind) {
      case "leftClick":
      case "rightClick":
      case "doubleClick":
      case "scroll":
        next = resolveIntroductionPoint(gesture.at, root);
        break;
      case "drag": {
        const from = resolveIntroductionPoint(gesture.from, root);
        const to = resolveIntroductionPoint(gesture.to, root);
        next = from && to ? { x: from.x + (to.x - from.x) * progress, y: from.y + (to.y - from.y) * progress } : (from ?? to ?? null);
        break;
      }
      case "orbit": {
        const from = resolveIntroductionPoint(gesture.from, root);
        const to = resolveIntroductionPoint(gesture.to, root);
        next = from && to ? introductionDemoArcPoint(from, to, progress) : (from ?? to ?? null);
        break;
      }
      default:
        next = null;
    }
    setPoint(next);
  }, [cue, progress, shellScope]);

  if (!cue || !point) return null;
  const glyph = cue.cursor ?? INTRODUCTION_DEMO_GESTURE_DEFAULT_CURSOR[cue.gesture.kind];
  return (
    <div
      id="ui.tutorial.ghostPointer"
      data-slot="tutorial-ghost-pointer"
      aria-hidden
      className="pointer-events-none fixed z-tutorial size-double -translate-x-1/2 -translate-y-1/2 bg-contain bg-no-repeat opacity-90"
      style={{ left: point.x, top: point.y, backgroundImage: `var(--cursor-ghost-${glyph})` }}
    />
  );
};
