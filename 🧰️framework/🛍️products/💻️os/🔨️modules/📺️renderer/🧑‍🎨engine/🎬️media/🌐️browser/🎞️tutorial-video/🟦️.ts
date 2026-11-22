import type { MediaRect } from "../🟦️.ts";

/** 🎞️ Drift past which the tutorial video hard-reseeks to the playhead, in milliseconds — React `TutorialVideoOverlay`'s tolerance. */
export const TUTORIAL_VIDEO_RESYNC_TOLERANCE_MS = 300;

/** 📹️ One resolved tutorial video cue as the shell computes it each frame (`tutorial_video_overlay`): viewport-pixel rect, source URL, mute, transport and the offset into the cue's own source. */
export type TutorialVideoOverlayPlan = {
  readonly src: string;
  readonly rect: MediaRect;
  readonly muted: boolean;
  readonly playing: boolean;
  readonly rate: number;
  readonly localTimeMs: number;
};

const SHELL_SLOT_KEYS = ["token", "windowId", "nodeId", "nodeKey", "rect", "clip", "paintOrder", "occluded", "pluginId", "controllerId", "appInstanceId", "parentDocumentId", "props"] as const;
const PROPS_KEYS = ["kind", "src", "muted", "playing", "rate", "localTimeMs"] as const;
/** 🪪️ The shell's own identity on the presented-slot contract (`presented-media-slots` schema, `shellTutorialVideoProps`). */
export const SHELL_TUTORIAL_VIDEO_SLOT = { token: "shell.tutorial.video", windowId: "shell", nodeId: "0", nodeKey: "ui.tutorial.video", pluginId: "shell", controllerId: "shell.tutorial", parentDocumentId: "shell", kind: "tutorialVideo", srcCodeUnits: 32_768 } as const;

function isShellTutorialVideoSlot(candidate: unknown): candidate is Record<string, unknown> {
  const props = (candidate as { props?: unknown } | null)?.props;
  return !!candidate && typeof candidate === "object" && !!props && typeof props === "object" && (props as { kind?: unknown }).kind === SHELL_TUTORIAL_VIDEO_SLOT.kind;
}

function plain(value: unknown, keys: readonly string[]): value is Record<string, unknown> {
  return !!value && typeof value === "object" && !Array.isArray(value) && Object.keys(value).length === keys.length && keys.every((key) => Object.hasOwn(value, key));
}

function finiteRect(value: unknown): value is MediaRect {
  if (!plain(value, ["x", "y", "width", "height"])) return false;
  return [value.x, value.y, value.width, value.height].every((number) => typeof number === "number" && Number.isFinite(number)) && (value.width as number) > 0 && (value.height as number) > 0;
}

/** 📹️ Admits one shell-owned tutorial slot descriptor exactly (identity constants, bounded source, positive rate) and returns its plan. */
export function parseShellTutorialVideoSlot(candidate: Record<string, unknown>): TutorialVideoOverlayPlan {
  const props = candidate.props;
  const owned = SHELL_TUTORIAL_VIDEO_SLOT;
  if (!plain(candidate, SHELL_SLOT_KEYS) || !plain(props, PROPS_KEYS)) throw new Error("browser-media.shell-descriptor");
  if (candidate.token !== owned.token || candidate.windowId !== owned.windowId || candidate.nodeId !== owned.nodeId || candidate.nodeKey !== owned.nodeKey || candidate.pluginId !== owned.pluginId || candidate.controllerId !== owned.controllerId || candidate.parentDocumentId !== owned.parentDocumentId || candidate.appInstanceId !== 0 || candidate.occluded !== false) throw new Error("browser-media.shell-owner");
  if (!finiteRect(candidate.rect) || !finiteRect(candidate.clip)) throw new Error("browser-media.shell-rect");
  if (typeof props.src !== "string" || props.src.length === 0 || props.src.length > owned.srcCodeUnits || typeof props.muted !== "boolean" || typeof props.playing !== "boolean" || typeof props.rate !== "number" || !(props.rate > 0) || !Number.isFinite(props.rate) || typeof props.localTimeMs !== "number" || !Number.isFinite(props.localTimeMs)) throw new Error("browser-media.shell-props");
  return { src: props.src, rect: candidate.rect, muted: props.muted, playing: props.playing, rate: props.rate, localTimeMs: props.localTimeMs };
}

/** ✂️ Splits the worker's presented slots into the plugin-owned transport slots and the one shell-owned tutorial video plan; two shell slots are a protocol violation. */
export function splitShellMediaSlots(value: unknown): { readonly transport: readonly unknown[]; readonly tutorialVideo: TutorialVideoOverlayPlan | null } {
  if (!Array.isArray(value)) return { transport: [], tutorialVideo: null };
  const shell = value.filter(isShellTutorialVideoSlot);
  if (shell.length > 1) throw new Error("browser-media.shell-duplicate");
  return { transport: value.filter((candidate) => !isShellTutorialVideoSlot(candidate)), tutorialVideo: shell.length === 1 ? parseShellTutorialVideoSlot(shell[0]!) : null };
}

/** 🧾️ Re-admits a plan the worker posted to the page (the page never trusts a posted plan unchecked). */
export function parseTutorialVideoPlan(value: unknown): TutorialVideoOverlayPlan | null {
  if (value === null || value === undefined) return null;
  if (!plain(value, ["src", "rect", "muted", "playing", "rate", "localTimeMs"]) || !finiteRect(value.rect) || typeof value.src !== "string" || value.src.length === 0 || value.src.length > SHELL_TUTORIAL_VIDEO_SLOT.srcCodeUnits || typeof value.muted !== "boolean" || typeof value.playing !== "boolean" || typeof value.rate !== "number" || !(value.rate > 0) || typeof value.localTimeMs !== "number" || !Number.isFinite(value.localTimeMs)) throw new Error("browser-media.shell-plan");
  return value as unknown as TutorialVideoOverlayPlan;
}

/** 🎯️ Whether a video sitting at `currentMs` must be hard-reseeked to `targetMs`. */
export function tutorialVideoNeedsReseek(currentMs: number, targetMs: number): boolean {
  return Math.abs(currentMs - targetMs) > TUTORIAL_VIDEO_RESYNC_TOLERANCE_MS;
}

/** 📐️ A cue's normalized `[x, y, width, height]` rect in viewport pixels. */
export function tutorialVideoRect(normalized: readonly [number, number, number, number], viewport: { readonly width: number; readonly height: number }): MediaRect {
  return { x: normalized[0] * viewport.width, y: normalized[1] * viewport.height, width: normalized[2] * viewport.width, height: normalized[3] * viewport.height };
}

/** ⏱️ Offset into a cue's own source at `playheadMs`. */
export function tutorialVideoLocalTimeMs(playheadMs: number, cueAtMs: number, sourceOffsetMs: number): number {
  return playheadMs - cueAtMs + sourceOffsetMs;
}

/** 📹️ Hosts React's `<video id="ui.tutorial.video">` for the wgpu shell inside the media overlay root: muted playsInline, `pointer-events:none`, `object-fit:cover`, `rounded-lg`, slaved to the playhead rather than played independently. */
export function createBrowserTutorialVideoOverlay(root: HTMLElement): { readonly accept: (plan: TutorialVideoOverlayPlan | null) => void; readonly dispose: () => void } {
  const video = document.createElement("video");
  video.id = "ui.tutorial.video";
  video.dataset.slot = "tutorial-video-overlay";
  video.playsInline = true;
  video.setAttribute("aria-hidden", "true");
  video.style.cssText = "position:absolute;pointer-events:none;object-fit:cover;border-radius:var(--radius-lg,0.5rem);box-shadow:var(--shadow-lg,0 10px 15px -3px rgb(0 0 0 / 0.1));display:none;";
  root.append(video);
  let source = "";
  let disposed = false;
  return {
    accept: (plan) => {
      if (disposed) return;
      if (!plan) {
        video.pause();
        video.style.display = "none";
        return;
      }
      if (plan.src !== source) {
        source = plan.src;
        video.src = plan.src;
      }
      Object.assign(video.style, { left: `${plan.rect.x}px`, top: `${plan.rect.y}px`, width: `${plan.rect.width}px`, height: `${plan.rect.height}px`, display: "block" });
      video.muted = plan.muted;
      video.playbackRate = plan.rate;
      const targetSeconds = plan.localTimeMs / 1000;
      if (tutorialVideoNeedsReseek(video.currentTime * 1000, plan.localTimeMs)) video.currentTime = targetSeconds;
      if (plan.playing) void video.play().catch(() => {});
      else video.pause();
    },
    dispose: () => {
      if (disposed) return;
      disposed = true;
      video.pause();
      video.removeAttribute("src");
      video.load();
      video.remove();
    },
  };
}
