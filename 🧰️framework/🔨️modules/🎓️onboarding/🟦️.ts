/** 🎓️ TypeScript twin of `semio_framework::onboarding` (`🦀️.rs` beside this file): the introduction and tutorial
 * declarations and the reserved action ids that start or record them. Exported through `@semio-tech/framework`. */
import type { IntroductionCursor as GeneratedIntroductionCursor, IntroductionDefinition as GeneratedIntroductionDefinition, IntroductionDemonstration as GeneratedIntroductionDemonstration, IntroductionGesture as GeneratedIntroductionGesture, IntroductionInteraction as GeneratedIntroductionInteraction, IntroductionInteractionKind as GeneratedIntroductionInteractionKind, IntroductionKeyModifier as GeneratedIntroductionKeyModifier, IntroductionLogo as GeneratedIntroductionLogo, IntroductionPlacement as GeneratedIntroductionPlacement, IntroductionPoint as GeneratedIntroductionPoint, IntroductionPointerButton as GeneratedIntroductionPointerButton, IntroductionStepDefinition as GeneratedIntroductionStepDefinition, TutorialAssetSrc as GeneratedTutorialAssetSrc, TutorialBase as GeneratedTutorialBase, TutorialCameraKeyframe as GeneratedTutorialCameraKeyframe, TutorialCameraState as GeneratedTutorialCameraState, TutorialCaption as GeneratedTutorialCaption, TutorialChapter as GeneratedTutorialChapter, TutorialDefinition as GeneratedTutorialDefinition, TutorialDocumentEvent as GeneratedTutorialDocumentEvent, TutorialDocumentEventKind as GeneratedTutorialDocumentEventKind, TutorialEasing as GeneratedTutorialEasing, TutorialEvent as GeneratedTutorialEvent, TutorialEventKind as GeneratedTutorialEventKind, TutorialGestureCue as GeneratedTutorialGestureCue, TutorialNarrationCue as GeneratedTutorialNarrationCue, TutorialOverlayRect as GeneratedTutorialOverlayRect, TutorialUiChange as GeneratedTutorialUiChange, TutorialUiKeyframe as GeneratedTutorialUiKeyframe, TutorialUiSnapshot as GeneratedTutorialUiSnapshot, TutorialVideoCue as GeneratedTutorialVideoCue } from "../🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts";
import type { WindowLayout } from "../🛂️manifest/🗺️window-layout/🟦️.ts";

/** 🎓️ The reserved action id apps dispatch (or the shell auto-injects into the command palette)
 * to (re)start an app's introduction — mirrors Rust `START_INTRODUCTION_ACTION_ID`. */
export const START_INTRODUCTION_ACTION_ID = "startIntroduction";

/** 🎓️ Generated from Rust `Introduction*` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type IntroductionDefinition = GeneratedIntroductionDefinition;
export type IntroductionStepDefinition = GeneratedIntroductionStepDefinition;
export type IntroductionPlacement = GeneratedIntroductionPlacement;
export type IntroductionInteraction = GeneratedIntroductionInteraction;
export type IntroductionInteractionKind = GeneratedIntroductionInteractionKind;
export type IntroductionLogo = GeneratedIntroductionLogo;
export type IntroductionPoint = GeneratedIntroductionPoint;
export type IntroductionGesture = GeneratedIntroductionGesture;
export type IntroductionKeyModifier = GeneratedIntroductionKeyModifier;
export type IntroductionPointerButton = GeneratedIntroductionPointerButton;
export type IntroductionCursor = GeneratedIntroductionCursor;
export type IntroductionDemonstration = GeneratedIntroductionDemonstration;

//#region 🎬️Tutorial
/** 🎬️ The framework-owned action id apps dispatch (or the shell auto-injects into the command palette,
 * with a `tutorialId` Select arg) to (re)start a tutorial — mirrors Rust `START_TUTORIAL_ACTION_ID`.
 * Distinct from the docs-tooltip `tutorial` link field on `UiLabelLeaf` (`framework/ui/js/react`), a URL into the
 * manual — this is the interactive recorded-walkthrough mechanism. */
export const START_TUTORIAL_ACTION_ID = "startTutorial";

/** ⏺️ The framework-owned action id that opens the tutorial recorder chrome — injected into EVERY app
 * unconditionally (recording needs no app-side declaration). Mirrors Rust `RECORD_TUTORIAL_ACTION_ID`. */
export const RECORD_TUTORIAL_ACTION_ID = "recordTutorial";

/** ⏱️ Real-time (rate-independent) duration of the camera glide the player performs when the user
 * presses Play after deviating from an active tutorial's recorded state. Mirrors Rust `TUTORIAL_CONVERGE_MS`. */
export const TUTORIAL_CONVERGE_MS = 600;

// 🔢️ `at`/`durationMs`/`sourceOffsetMs`/`size` are Rust `u64`, which the owned schema exporter
// projects as `bigint` (see `AssetDeclaration.sizeBytes`/`QuotaSchema.*` above) — every consumer in
// `ui/js/react`'s tutorial player/recorder (`createTutorialClock`, `tutorialCameraAt`, `composeTutorialUi`,
// the `TutorialBar`/`TutorialChapterMarker` chrome) treats timeline offsets as plain `number`
// millisecond values, so every such field is narrowed back with `Omit<Generated*, ...> & {...}` here —
// the identical pattern `AppDefinition` already uses to narrow `iconId`/`defaultLayout`/`namedLayouts`
// below. `title`/`body`/`description`/`text` (Rust `LocalizedLabel`) are NOT narrowed: they stay the
// generated `unknown`, exactly like `AppDefinition.label` — every consumer already resolves them via
// `resolveManifestLabel(label: unknown, …)`.
export type TutorialChapter = Omit<GeneratedTutorialChapter, "at"> & { readonly at: number };

/** 📦️ `Blob.size` is the sole `u64` field — narrowed to `number` like every other timeline offset above. */
export type TutorialAssetSrc = Exclude<GeneratedTutorialAssetSrc, { readonly kind: "blob" }> | (Omit<Extract<GeneratedTutorialAssetSrc, { readonly kind: "blob" }>, "size"> & { readonly size: number });

export type TutorialCaption = Omit<GeneratedTutorialCaption, "at" | "durationMs"> & { readonly at: number; readonly durationMs: number };

export type TutorialNarrationCue = Omit<GeneratedTutorialNarrationCue, "at" | "durationMs" | "audio" | "captions"> & {
  readonly at: number;
  readonly durationMs: number;
  readonly audio?: TutorialAssetSrc;
  readonly captions: readonly TutorialCaption[];
};

export type TutorialOverlayRect = GeneratedTutorialOverlayRect;

export type TutorialVideoCue = Omit<GeneratedTutorialVideoCue, "at" | "durationMs" | "sourceOffsetMs" | "src"> & {
  readonly at: number;
  readonly durationMs: number;
  readonly sourceOffsetMs: number;
  readonly src: TutorialAssetSrc;
};

export type TutorialEventKind = GeneratedTutorialEventKind;

export type TutorialEvent = Omit<GeneratedTutorialEvent, "at"> & { readonly at: number };

/** 🧮️ Renderer-neutral restore point for chrome/UI state — see the Rust doc comment on
 * `TutorialUiSnapshot` for why this is deliberately NOT a serialization of `ShellState`. `layout` is
 * narrowed to this file's own hand-refined `WindowLayout` — same reason and same override
 * (`applyFrameworkLayoutSeed` below requires the narrower `"row" | "column" | "stack" | "window"`
 * literal `kind`) as `AppDefinition.defaultLayout`. */
export type TutorialUiSnapshot = Omit<GeneratedTutorialUiSnapshot, "layout"> & { readonly layout?: WindowLayout };

export type TutorialUiChange = Exclude<GeneratedTutorialUiChange, { readonly kind: "layout" }> | { readonly kind: "layout"; readonly layout: WindowLayout };

export type TutorialUiSample = { readonly kind: "snapshot"; readonly state: TutorialUiSnapshot } | { readonly kind: "delta"; readonly changes: readonly TutorialUiChange[] };

export type TutorialUiKeyframe = Omit<GeneratedTutorialUiKeyframe, "at" | "sample"> & { readonly at: number; readonly sample: TutorialUiSample };

/** 🖋️ Mirrors `store::os_store::store_schema::ArtifactCommand` with `Mutation = unknown` (opaque per-app mutation JSON) — the
 * SOLE source of document mutation during playback; `TutorialEvent`s are annotational only. */
export type TutorialDocumentEventKind = GeneratedTutorialDocumentEventKind;

export type TutorialDocumentEvent = Omit<GeneratedTutorialDocumentEvent, "at"> & { readonly at: number };

export type TutorialCameraState = GeneratedTutorialCameraState;

export type TutorialEasing = GeneratedTutorialEasing;

export type TutorialCameraKeyframe = Omit<GeneratedTutorialCameraKeyframe, "at"> & { readonly at: number };

/** 👻️ Reuses the introduction demonstration vocabulary verbatim — see `IntroductionGesture`/`IntroductionPoint`. */
export type TutorialGestureCue = Omit<GeneratedTutorialGestureCue, "at" | "durationMs"> & { readonly at: number; readonly durationMs: number };

export type TutorialTracks = {
  readonly narration: readonly TutorialNarrationCue[];
  readonly video: readonly TutorialVideoCue[];
  readonly events: readonly TutorialEvent[];
  readonly ui: readonly TutorialUiKeyframe[];
  readonly document: readonly TutorialDocumentEvent[];
  readonly camera: readonly TutorialCameraKeyframe[];
  readonly gestures: readonly TutorialGestureCue[];
};

export type TutorialBase = Omit<GeneratedTutorialBase, "ui" | "cameras"> & {
  readonly ui: TutorialUiSnapshot;
  readonly cameras: readonly TutorialCameraKeyframe[];
};

/** 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of `IntroductionDefinition`. A
 * *recording* IS a `TutorialDefinition`; the recorder simply produces a densely-sampled one. */
export type TutorialDefinition = Omit<GeneratedTutorialDefinition, "durationMs" | "chapters" | "base" | "tracks"> & {
  readonly durationMs: number;
  readonly chapters: readonly TutorialChapter[];
  readonly base: TutorialBase;
  readonly tracks: TutorialTracks;
};
//#endregion 🎬️Tutorial
