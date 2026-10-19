import type { PresenceAppearance, PresenceHsl, PresencePeer, TutorialCameraKeyframe, TutorialClockPort, TutorialDefinition, UiTranslationKey } from "@semio-tech/ui-react/source";
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerTutorialEngineAndMoreTests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { composeTutorialUi, createTutorialClock, formatTutorialTime, interpolateTutorialCamera, PresenceBar, presenceColor, presenceCssVar, React, renderToStaticMarkup, resolveTranslationLabel, TutorialBar, tutorialCameraAt, tutorialCuesBetween, tutorialSlice, uiI18n, validateTutorial } = dependencies;
  const { describe, expect, it, vi } = vitest;

  
    describe("tutorial engine", () => {
      const minimalTutorial = (): TutorialDefinition => ({
        id: "welcome-tour",
        title: "Welcome Tour",
        durationMs: 10_000,
        chapters: [{ id: "start", at: 0, title: "Start" }],
        base: { exampleId: "concrete-forest", ui: { activeUtilityByWindowId: {}, activePanelTabByGroup: {}, interactionSelection: {}, expandedTreeIds: [], commandPanelOpen: false }, cameras: [] },
        tracks: { narration: [], video: [], events: [], ui: [], document: [], camera: [], gestures: [] },
      });
  
  
      it("formatTutorialTime formats mm:ss and floors sub-second/negative offsets", () => {
        expect(formatTutorialTime(0)).toBe("0:00");
        expect(formatTutorialTime(1_500)).toBe("0:01");
        expect(formatTutorialTime(65_000)).toBe("1:05");
        expect(formatTutorialTime(-100)).toBe("0:00");
      });
  
      it("tutorialCuesBetween returns only cues whose [at, at+durationMs) window covers atMs", () => {
        const cues = [
          { at: 0, durationMs: 100 },
          { at: 100, durationMs: 50 },
        ];
        expect(tutorialCuesBetween(cues, 50)).toEqual([cues[0]]);
        expect(tutorialCuesBetween(cues, 100)).toEqual([cues[1]]);
        expect(tutorialCuesBetween(cues, 200)).toEqual([]);
      });
  
      it("interpolateTutorialCamera lerps position/target and clamps to endpoints", () => {
        const prev: TutorialCameraKeyframe = { at: 0, windowId: "w", camera: { kind: "orbit", position: [0, 0, 0], target: [0, 0, 0], up: [0, 0, 1], fov: 40 }, easing: "linear" };
        const next: TutorialCameraKeyframe = { at: 1000, windowId: "w", camera: { kind: "orbit", position: [10, 0, 0], target: [0, 0, 0], up: [0, 0, 1], fov: 60 }, easing: "linear" };
        const mid = interpolateTutorialCamera(prev, next, 500);
        expect(mid.kind).toBe("orbit");
        if (mid.kind === "orbit") {
          expect(mid.position[0]).toBeCloseTo(5, 9);
          expect(mid.fov).toBe(50);
        }
        expect(interpolateTutorialCamera(prev, next, 0)).toEqual(prev.camera);
        expect(interpolateTutorialCamera(prev, next, 1000)).toEqual(next.camera);
      });
  
      it("interpolateTutorialCamera zooms canvas cameras in log space", () => {
        const prev: TutorialCameraKeyframe = { at: 0, windowId: "w", camera: { kind: "canvas", x: 0, y: 0, zoom: 1 }, easing: "linear" };
        const next: TutorialCameraKeyframe = { at: 1000, windowId: "w", camera: { kind: "canvas", x: 0, y: 0, zoom: 4 }, easing: "linear" };
        const mid = interpolateTutorialCamera(prev, next, 500);
        expect(mid.kind).toBe("canvas");
        if (mid.kind === "canvas") expect(mid.zoom).toBeCloseTo(2, 9);
      });
  
      it("interpolateTutorialCamera holds the previous pose until the keyframe, then snaps", () => {
        const prev: TutorialCameraKeyframe = { at: 0, windowId: "w", camera: { kind: "canvas", x: 0, y: 0, zoom: 1 }, easing: "hold" };
        const next: TutorialCameraKeyframe = { at: 1000, windowId: "w", camera: { kind: "canvas", x: 0, y: 0, zoom: 4 }, easing: "hold" };
        expect(interpolateTutorialCamera(prev, next, 999)).toEqual(prev.camera);
        expect(interpolateTutorialCamera(prev, next, 1000)).toEqual(next.camera);
      });
  
      it("tutorialCameraAt holds the first pose before the first keyframe and the last pose after", () => {
        const def = {
          ...minimalTutorial(),
          tracks: {
            ...minimalTutorial().tracks,
            camera: [
              { at: 100, windowId: "w", camera: { kind: "canvas" as const, x: 0, y: 0, zoom: 1 }, easing: "linear" as const },
              { at: 900, windowId: "w", camera: { kind: "canvas" as const, x: 0, y: 0, zoom: 9 }, easing: "linear" as const },
            ],
          },
        };
        expect(tutorialCameraAt(def, "w", 0)).toEqual({ kind: "canvas", x: 0, y: 0, zoom: 1 });
        expect(tutorialCameraAt(def, "w", 10_000)).toEqual({ kind: "canvas", x: 0, y: 0, zoom: 9 });
        expect(tutorialCameraAt(def, "other-window", 500)).toBeUndefined();
      });
  
      it("composeTutorialUi applies the latest snapshot then replays deltas after it", () => {
        const base = minimalTutorial();
        const def: TutorialDefinition = {
          ...base,
          base: { ...base.base, ui: { ...base.base.ui, activeToolId: "fill" } },
          tracks: {
            ...base.tracks,
            ui: [
              { at: 100, sample: { kind: "snapshot", state: { activeModeId: "edit", activeUtilityByWindowId: {}, activePanelTabByGroup: {}, interactionSelection: {}, expandedTreeIds: [], commandPanelOpen: false } } },
              { at: 200, sample: { kind: "delta", changes: [{ kind: "activeTool", id: "brush" }] } },
              { at: 300, sample: { kind: "delta", changes: [{ kind: "panelTab", group: "top-left", tabId: "catalogue" }] } },
            ],
          },
        };
        expect(composeTutorialUi(def, 0).activeToolId).toBe("fill");
        const at100 = composeTutorialUi(def, 100);
        expect(at100.activeModeId).toBe("edit");
        expect(at100.activeToolId).toBeUndefined();
        expect(composeTutorialUi(def, 250).activeToolId).toBe("brush");
        const at300 = composeTutorialUi(def, 300);
        expect(at300.activeToolId).toBe("brush");
        expect(at300.activePanelTabByGroup["top-left"]).toBe("catalogue");
      });
  
      it("tutorialSlice crosses document events forward oldest-first and backward newest-first", () => {
        const base = minimalTutorial();
        const def: TutorialDefinition = {
          ...base,
          tracks: {
            ...base.tracks,
            document: [
              { at: 100, kind: { kind: "edit", forwards: [{ op: "add", id: "a" }], backwards: [{ op: "remove", id: "a" }] } },
              { at: 200, kind: { kind: "edit", forwards: [{ op: "add", id: "b" }], backwards: [{ op: "remove", id: "b" }] } },
            ],
          },
        };
        const forward = tutorialSlice(def, 0, 250);
        expect(forward.forward).toBe(true);
        expect(forward.document).toHaveLength(2);
        expect((forward.document[0].kind as { forwards: readonly { id: string }[] }).forwards[0].id).toBe("a");
  
        const backward = tutorialSlice(def, 250, 0);
        expect(backward.forward).toBe(false);
        expect(backward.document).toHaveLength(2);
        expect((backward.document[0].kind as { backwards: readonly { id: string }[] }).backwards[0].id).toBe("b");
  
        expect(tutorialSlice(def, 250, 250).document).toHaveLength(0);
      });
  
      it("validateTutorial rejects unsorted/out-of-range tracks and passes a minimal valid tutorial", () => {
        const unsorted = {
          ...minimalTutorial(),
          tracks: {
            ...minimalTutorial().tracks,
            narration: [
              { id: "b", at: 500, durationMs: 100, text: "b", rate: 1, captions: [] },
              { id: "a", at: 100, durationMs: 100, text: "a", rate: 1, captions: [] },
            ],
          },
        };
        expect(validateTutorial(unsorted)).not.toBeNull();
  
        const outOfRange = { ...minimalTutorial(), tracks: { ...minimalTutorial().tracks, narration: [{ id: "a", at: 999_999, durationMs: 100, text: "a", rate: 1, captions: [] }] } };
        expect(validateTutorial(outOfRange)).not.toBeNull();
  
        const dupChapters = { ...minimalTutorial(), chapters: [...minimalTutorial().chapters, { id: "start", at: 0, title: "Dup" }] };
        expect(validateTutorial(dupChapters)).not.toBeNull();
  
        const badBaseCamera = { ...minimalTutorial(), base: { ...minimalTutorial().base, cameras: [{ at: 5, windowId: "w", camera: { kind: "canvas" as const, x: 0, y: 0, zoom: 1 }, easing: "linear" as const }] } };
        expect(validateTutorial(badBaseCamera)).not.toBeNull();
  
        expect(validateTutorial(minimalTutorial())).toBeNull();
      });
    });

  
    describe("TutorialBar", () => {
      it("renders its element ids and chapter ticks", () => {
        const clock: TutorialClockPort = { getTimeMs: () => 1000, subscribe: () => () => {} };
        const markup = renderToStaticMarkup(
          <TutorialBar
            title="Welcome Tour"
            durationMs={10_000}
            playing={false}
            rate={1}
            muted={false}
            captionsOn
            recording
            recordAvailable
            chapters={[{ id: "start", title: "Start", atMs: 0 }]}
            clock={clock}
            onPlayPause={() => {}}
            onStop={() => {}}
            onSeek={() => {}}
            onRateChange={() => {}}
            onMutedChange={() => {}}
            onCaptionsChange={() => {}}
            onRecordToggle={() => {}}
            onAddChapter={() => {}}
          />,
        );
        expect(markup).toContain('id="ui.tutorial.bar"');
        expect(markup).toContain('id="ui.tutorial.play"');
        expect(markup).toContain('id="ui.tutorial.stop"');
        expect(markup).toContain('id="ui.tutorial.scrubber"');
        expect(markup).toContain('id="ui.tutorial.chapter.start"');
        expect(markup).toContain('id="ui.tutorial.record"');
        expect(markup).toContain('id="ui.tutorial.recordingIndicator"');
      });
    });

  
    describe("TutorialClock", () => {
      it("seek/setRate/play/pause update state synchronously (rAF ticking itself is not exercised here)", () => {
        const clock = createTutorialClock(1000);
        expect(clock.getTimeMs()).toBe(0);
        expect(clock.isPlaying()).toBe(false);
        clock.seek(400);
        expect(clock.getTimeMs()).toBe(400);
        clock.setRate(2);
        expect(clock.getRate()).toBe(2);
        clock.play();
        expect(clock.isPlaying()).toBe(true);
        clock.pause();
        expect(clock.isPlaying()).toBe(false);
        clock.seek(-50);
        expect(clock.getTimeMs()).toBe(0);
        clock.seek(5000);
        expect(clock.getTimeMs()).toBe(1000);
        clock.dispose();
      });
  
      it("notifies subscribers on seek/rate/play/pause", () => {
        const clock = createTutorialClock(1000);
        const listener = vi.fn();
        const unsubscribe = clock.subscribe(listener);
        clock.seek(100);
        clock.setRate(1.5);
        expect(listener).toHaveBeenCalledTimes(2);
        unsubscribe();
        clock.seek(200);
        expect(listener).toHaveBeenCalledTimes(2);
      });
    });

  
    describe("PresenceBar", () => {
      const peers = (n: number): PresencePeer[] => Array.from({ length: n }, (_, i) => ({ actor: `user:p${i}#s`, label: `Peer ${i}`, role: i % 2 === 0 ? ("author" as const) : ("spectator" as const) }));
  
      it("renders one listitem per peer under max, each carrying its own peer data-row-id", async () => {
        const { render } = await import("@testing-library/react");
        const { container } = render(<PresenceBar id="s-presence-peers" peers={peers(3)} />);
        const root = container.querySelector("#s-presence-peers")!;
        expect(root.getAttribute("role")).toBe("list");
        expect(root.querySelectorAll('[role="listitem"]')).toHaveLength(3);
        expect(container.querySelector('[data-row-id="peer:user:p0#s"]')).not.toBeNull();
        expect(container.querySelector('[data-row-id="peer:user:p2#s"]')).not.toBeNull();
      });
  
      it("collapses past max into a single overflow chip", async () => {
        const { render } = await import("@testing-library/react");
        const { container } = render(<PresenceBar id="s-presence-peers" peers={peers(7)} max={5} />);
        expect(container.querySelectorAll('[role="listitem"]')).toHaveLength(6);
        expect(container.querySelector('[data-row-id="peer:overflow"]')?.textContent).toBe("+2");
      });
  
      it("renders an empty state with no listitems when there are no peers", async () => {
        const { render } = await import("@testing-library/react");
        const { container } = render(<PresenceBar id="s-presence-peers" peers={[]} />);
        expect(container.querySelectorAll('[role="listitem"]')).toHaveLength(0);
        expect(container.querySelector("#s-presence-peers")?.textContent).toBeTruthy();
      });
  
      it("resolves every ui.presence.* key in both en and de", () => {
        // 🐚️ Mirrors the ribbon-category coverage test above: reads `uiI18n.t` directly instead of round-tripping
        // through a React render, since this huge in-source suite shares one global i18next instance and a
        // full-render assertion racing another test's own in-flight `changeLanguage` is exactly the kind of
        // cross-test flake that pattern avoids.
        const keys: UiTranslationKey[] = ["ui.presence.roster", "ui.presence.empty", "ui.presence.overflow", "ui.presence.role.author", "ui.presence.role.spectator"];
        const seenByLocale: Record<string, string[]> = {};
        for (const locale of ["en", "de"] as const) {
          void uiI18n.changeLanguage(locale);
          seenByLocale[locale] = keys.map((key) => {
            const label = resolveTranslationLabel(uiI18n.t(key));
            expect(label, `${locale}:${key}`).toBeTruthy();
            expect(label, `${locale}:${key}`).not.toBe(key);
            return label!;
          });
        }
        expect(seenByLocale.en).not.toEqual(seenByLocale.de);
        void uiI18n.changeLanguage("en");
      });
  
      // 👥️ Pinned index/appearance table (contract freeze §C7.5) — the Rust twin's
      // `presence_color_wraps_after_twelve_with_lightness_then_saturation_shift` (`🧊️component.rs`)
      // pins the same formula against the same `ui_styling::presence`-generated constants
      // (`hues`/`light`/`dark` from `🎨️styling/🔣️tokens.json`), so identical inputs here prove the two
      // implementations compute byte-identical HSL — this is the "TS twin" half of that cross-check.
      it("presenceColor matches the Rust twin's pinned index/appearance table", () => {
        const cases: readonly [index: number, appearance: PresenceAppearance, expected: PresenceHsl][] = [
          [0, "light", { h: 0, s: 0.68, l: 0.32 }],
          [5, "light", { h: 180, s: 0.68, l: 0.32 }],
          [11, "dark", { h: 90, s: 0.72, l: 0.62 }],
          [12, "light", { h: 0, s: 0.68, l: 0.46 }], // k=1 (odd, <2): lightness +0.14, no desaturation
          [12, "dark", { h: 0, s: 0.72, l: 0.48 }], // k=1: dark shifts lightness DOWN instead
          [24, "light", { h: 0, s: 0.43, l: 0.32 }], // k=2 (even, >=2): desaturate -0.25, no lightness shift
          [25, "dark", { h: 210, s: 0.47, l: 0.62 }], // k=2
          [37, "light", { h: 210, s: 0.43, l: 0.46 }], // k=3 (odd, >=2): both shifts apply
        ];
        for (const [index, appearance, expected] of cases) {
          const resolved = presenceColor(index, appearance);
          expect(resolved.h, `index=${index} ${appearance} h`).toBe(expected.h);
          expect(resolved.s, `index=${index} ${appearance} s`).toBeCloseTo(expected.s, 10);
          expect(resolved.l, `index=${index} ${appearance} l`).toBeCloseTo(expected.l, 10);
        }
      });
  
      it("presenceCssVar addresses only the base cycle, wrapping modulo 12", () => {
        expect(presenceCssVar(0)).toBe("var(--presence-0)");
        expect(presenceCssVar(11)).toBe("var(--presence-11)");
        expect(presenceCssVar(12)).toBe("var(--presence-0)");
      });
    });
}
