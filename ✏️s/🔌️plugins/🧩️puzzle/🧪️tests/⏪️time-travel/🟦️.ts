/** 🕰️ The puzzle 2d scenario of `time-travel` — the end-to-end gate of non-destructive history editing, driven the way a person drives it in the
 * puzzle 2d dev serve (the reference app of the history-editing design, React `serve puzzle2d react dev` on :6012 or the wgpu
 * browser shell on :6112). One headless Chromium, one tab at a time, one fresh browser context per UI locale (`en`, then
 * `de`, chosen through `navigator.language`), numbered steps per locale (promoted from the ticket probe of 26/09/30
 * NON-DESTRUCTIVE-HISTORY-EDITING, work package W3-E2E):
 *
 * 1. boot: `data-board-snapshot-parsed` first, then a board with nodes and the locale of the history panel; then the
 *    document is bound to a fresh local folder through the sync card (`--folder-at=1`, the default), so every edit persists;
 * 2. select two nodes, drag them by (+80,+40) world units → exactly one new document row labelled from its
 *    `drag-selection` mutation, `data-board-positions-json` moved by the offset; then one downstream drag of a third
 *    node, so "downstream is not applied" is observable;
 * 3. Edit the drag mutation (`historyEditBegin`) → the React band (`role=status`) in `editing`, the preview = state
 *    before the drag + the draft (design §4), the downstream drag not applied, the editor's dx/dy steppers (snap step)
 *    and the targets reference list;
 * 4. dx → 120 through the stepper (keyboard: fill, Enter, ArrowUp, ArrowDown) → preview; Accept → replay → review
 *    `ready` → head at +120 with the downstream drag re-applied;
 * 5. Finalize → dialog (destructive Overwrite, New alternative + name) → Overwrite → band gone, an
 *    "History edited — overwrite" row; after step 8 a page reload plus a re-attach of the same folder must restore the head
 *    and every document row;
 * 6. Undo (chord, then button) → +80; Redo → +120; the undone/redone rows;
 * 7. a second session on dy → New alternative with a name → the alternative row, the head and the Alternatives section
 *    (`framework.history.alternative.<id>`); switching to the trunk and back when the trunk is listed, else a second
 *    alternative and switching between the two;
 * 8. fatal path: duplicate a node, drag the clone, withdraw the clone's `create-node` → review `blocked`, the drag row
 *    reads Error "Target missing", Finalize disabled; Next problem → Withdraw the drag → `ready`; Exit → zero trace;
 * 9. console: uncaught page errors, hard faults and lines with AGENTS.md's temporary-log tag fail the run; errors and warnings are digested;
 * 10. input controls (G6, design §18): a rotate command → Edit → the angle dial (ticks at 0, ±90, 180°, degrees, Arrow = one
 *     step, PageUp = next detent); a scale command → Edit → the factor slider (log ticks at its snaps, a typed value beyond
 *     the hard bound refused naming the bound, the draft kept); Exit leaves zero trace;
 * 11. keep editing after a clean review: `ready` → Begin another mutation → Accept → `ready` with two accepted drafts →
 *     Finalize overwrite of both;
 * 12. fatal loop resolved by EDITING targets (G3, design §16.4): withdraw an upstream `create-node` → the downstream drag is
 *     `mutation.target-missing`, review `blocked` → Next problem → select two remaining nodes → Use selection → chips read
 *     labels, the board highlights them → Accept → `ready` → Finalize overwrite → head equals the expectation;
 * 13. a warning introduced by an upstream edit (G4, design §16.5, S2-W2D scenario `warning-from-an-upstream-edit`): lock and
 *     unlock a node, drag it with another, withdraw the unlock → the drag reads Warning "Partially applied" (new since this
 *     edit), review `ready` → Finalize overwrite → still visible; the reload check (after step 13) re-reads it;
 * 14. phone width (G13): a fresh 375 px touch context — band, editor and finalize prompt inside the viewport, touch-sized,
 *     reachable by touch and keyboard, no horizontal page scroll;
 * 15. tablet width (G13): the same journey in a fresh 768 × 1024 touch context (the tablet breakpoint of `📱️device`);
 * 16. long history (G9, N1, N15): a fresh document whose example load is one transaction of several hundred mutations — its
 *     row is a tree window whose `total` (React `data-tree-window-total`) counts every mutation and whose LAST one opens for
 *     editing; edit its first mutation, Accept, replay progress over at least `--long-history` mutations, Cancel (review
 *     "Replay needed"), Replay again with an Edit pressed while it replays (disabled, naming why; the session keeps its
 *     target), Exit with zero trace; the history grows through the palette's Set Active Example until the replay is observable;
 * 17. two peers on one local folder (G10): two fresh contexts attached to the same folder — the second reads the first's
 *     document, the first begins a history edit, the second's drag arrives as a base move (the session survives, the remote
 *     edit stays downstream and unapplied), Accept replays it too, Finalize reaches the second peer, whose history body shows
 *     what it replays (`HistoryPatch.reprojection`). Presence (the ⏪ roster badge, "is editing") travels only through a hub
 *     and is recorded as such;
 * 18. history steps and the stepped document load (N17, §20.8): a fresh document, bound to a fresh folder, grown by a second
 *     example load, its first mutation finalized as a new alternative; switching back to the main line is a deferred history
 *     step — the body's `framework.history.reprojection` section ("History step", "Replaying history: d of t mutations") —
 *     during which Undo is refused (`history.replaying`) and Cancel replay drops the step with zero trace; switching again
 *     completes it. A second fresh page attaching that folder loads the archive stepwise ("Document load", "Loading
 *     document: d of t"), refuses a command meanwhile (`document.loading`) and Cancel replay keeps its previous document;
 * 19. configuration is not history (L4, §20.13), windows count rows (N1), list inputs (N2), transaction labels (§19.1): camera
 *     zoom and pan add no history row; a drag adds exactly one (the commands window `total` grows by one) labelled by its
 *     intent leaf; Undo after a camera move takes back the drag and leaves the camera; Edit of a duplicated node's
 *     `create-node` → Add item / Remove item on its handles list (the row names its item count) → Exit with zero trace;
 * 20. row actions (design §22.1, §22.20): every mutation row offers Edit and Withdraw; Withdraw pressed on a row opens (or
 *     stacks) a session with a withdrawn draft — a duplicated node's `create-node` withdrawn from its row leaves its drag
 *     blocking, the blocking drag is withdrawn from ITS row (no editor needed) → review `ready`; a row holding an accepted
 *     draft offers Restore in place of Withdraw, and Restore drops that draft; Exit leaves zero trace;
 * 21. the goal's own sentence as one verdict set (puzzle 2d: dragging the selection yields ONE drag mutation whose selection AND
 *     offset stay editable): one drag of two nodes = one row; Edit → the selection as a reference list (Use selection, a chip
 *     with Remove per target) and dx / dy as steppers stepping by the grid snap; the offset is stepped, a target removed, the
 *     targets replaced by the board selection — the preview follows each — and Accept re-applies the downstream drag; Exit.
 *
 * Step 3 also asserts the G13 transitions (`history-panel-reveals-on-session-start`, `focus-moves-to-the-editor`), step 4
 * focus on the band and the N15 refusal of Edit on another mutation while a changed draft is open (disabled, its reason named
 * and shown), step 5 focus in the prompt and Edit refused while choosing; steps 3, 5, 14 and 15 run the structural ARIA oracle (`aria-query`'s WAI-ARIA
 * role model + `dom-accessibility-api`'s accessible names, bundled into the page) over the band, the history panel and the
 * prompt (wgpu: over its ARIA mirror). The reload check (G5) runs after step 13 and is attributed to step 5: every pre-reload
 * row with its localized label, the alternatives (main line + alternatives) and the current alternative.
 *
 * Every control is the real one, reached the way a user reaches it: the windowed history body is scrolled until the
 * section holds the row, steppers take keyboard input, arg-carrying commands run from the palette (`mod+p`) through the
 * command panel's staged form. Counts come from the windows' own `total` (the rows a window holds, materialised or not),
 * never from the rows the DOM happens to hold (W1E-6); the wgpu mirror publishes no window size, so wgpu pages until the
 * expected rows were all seen.
 *
 * `--renderer wgpu` drives the puzzle 2d wgpu shell (default serve :6112) through the same steps: rows, buttons, inputs and
 * the dialog through its ARIA mirror (`#semio-wgpu-accessibility`), chrome through `dumpChrome` hit rects, the board with the
 * pointer and keyboard on the canvas at the positions `semioWgpuIntrospection.dumpBoard2d` publishes (region `🔖️Wgpu`).
 *
 * Run: `bun repo run @semio-tech/framework-os-dev:time-travel -- --scenario ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts [--serve <url>] [--renderer react|wgpu] [--locales en,de] [--chords en,de] [--only 1,2,3]
 * [--folder-at 1|5] [--long-history <mutations>] [--out <dir>] [--universal] [--variant <declared variant>] [--explore]` — the serve is reused when it answers, else
 * started for the run and stopped after it ({@link ensureDevServe}). `--only` selects steps (each selection boots its own
 * document, so a later step needs the earlier ones it builds on: 3–7 and 11 build on 2, 6 and 7 on 4–5); `reload` selects the
 * reload check (it needs step 5's overwrite or step 13's warning), which runs on its own only when named. `--chords <locales>` drives Accept/Discard/Exit through
 * the band chords (`alt+enter`, `alt+backspace`, `alt+shift+backspace`) in those locales and through the band buttons
 * elsewhere; step 11 always takes the other way once, so chords and buttons are both driven. `--explore` boots each locale,
 * opens the history panel, dumps the DOM inventory and exits. Outputs under `--out` (default `🤖️generated/🧪️time-travel`):
 * `probe-<renderer>-<stamp>.md` (step lines, verdicts, notes, timeline, console digest), `.ndjson` (one record per
 * verdict/note/dump) and `probe-<renderer>-<stamp>-<locale>-s<step>-<tag>.png|json`; the acceptance record `time-travel`.
 * @see ../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts */
import type { TimeTravelBlockV1, TimeTravelScenarioV1 } from "@semio-tech/framework/time-travel/testing/scenario";
import { renderer, setCurrentStep, reloadSelected, invalidatedBy, cancelled } from "@semio-tech/framework/time-travel/testing/state";
import { wgpuBoard, WGPU_BOARD_CONTRACT, PLUGIN_VARIANT, DEFAULT_SERVES, vitals } from "./🎲️board/🟦️.ts";
import { waitUntil, shot } from "@semio-tech/framework/time-travel/testing/page";
import { verdict } from "@semio-tech/framework/time-travel/testing/report";
import { mirrorFind, mirror, wgpuPress, mirrorActivate } from "@semio-tech/framework/time-travel/testing/wgpu-mirror";
import { step1, step2, step3, step4, step5 } from "./🌄️drag-journey/🟦️.ts";
import type { Ctx } from "./🌄️drag-journey/🟦️.ts";
import { reloadCheck } from "./🔂️reload-check/🟦️.ts";
import { step6, step7, step8 } from "./🌿️alternatives/🟦️.ts";
import { step10, step11, step12, step13, step14, step15 } from "./🧮️gaps/🟦️.ts";
import { step16, step17, step18, step19 } from "./🛣️lanes/🟦️.ts";
import { step20, step21, step22, step23 } from "./🕹️interaction/🟦️.ts";

/** 🔁️ The contexts whose reload check already ran. */
const reloaded = new WeakSet<object>();

/** 🧱️ Once per locale after boot: on wgpu the board must project its introspection and carry the example; otherwise the board steps are blocked. */
const prepare = async (): Promise<TimeTravelBlockV1 | null> => {
  let boardEmpty = false;
  if (renderer === "wgpu" && (await wgpuBoard()) !== null) {
    setCurrentStep(1);
    const atBoot = await waitUntil(wgpuBoard, (surface) => (surface?.nodes ?? 0) > 0, 15000, 1000);
    verdict("wgpu-loads-the-example-at-boot", atBoot.ok, { nodes: atBoot.value?.nodes ?? null, edges: atBoot.value?.edges ?? null, waitedMs: atBoot.waitedMs, fixture: mirrorFind(await mirror(), "playground.navbar.fixture")?.label ?? null, reading: "the playground route names its example in the navbar and the React shell boots with it loaded (one \"Set Active Example\" row); the wgpu board must hold its nodes too" });
    if (!atBoot.ok) {
      const opened = await wgpuPress("playground.navbar.fixture");
      const option = await waitUntil(async () => (await mirror()).find((node) => /(^|[./])shell\.example\.[\w-]+$/u.test(node.key)) ?? null, (node) => node !== null, 6000, 300);
      const chosen = option.value ? await mirrorActivate(option.value.key, option.value.window) : false;
      const loaded = await waitUntil(wgpuBoard, (surface) => (surface?.nodes ?? 0) > 0, 30000, 1000);
      verdict("wgpu-navbar-example-loads-the-board", loaded.ok, { opened, option: option.value ? `${option.value.key} = ${option.value.label}` : null, chosen, nodes: loaded.value?.nodes ?? null, waitedMs: loaded.waitedMs, reading: "choosing the example in the navbar's example control (`playground.navbar.fixture` → `shell.example.<id>`, the `setActiveExample` action) must fill the board" });
      boardEmpty = !loaded.ok;
      if (boardEmpty) await shot("wgpu-empty-board");
    }
    setCurrentStep(0);
  }
  const boardBlocked = renderer === "wgpu" && (boardEmpty || (await wgpuBoard()) === null);
  if (boardBlocked && !boardEmpty) verdict("wgpu-board-introspection-present", false, { prerequisite: WGPU_BOARD_CONTRACT, reading: "the wgpu Board2d surface projects no accessibility nodes and no dump carries its positions, camera or selection, so no pointer gesture can be aimed at a node; every board step is blocked until the export exists" });
  return boardBlocked ? { name: boardEmpty ? "blocked-by-the-empty-board" : "blocked-by-missing-board-introspection", prerequisite: boardEmpty ? "a board with nodes (wgpu-loads-the-example-at-boot / wgpu-navbar-example-loads-the-board)" : "semioWgpuIntrospection.dumpBoard2d", exempt: [1] } : null;
};

/** 🔁️ Takes step 5's reload half before the fresh-context steps (14) and the console digest (9). */
const beforeStep = async (step: number, ctx: Ctx, blocked: TimeTravelBlockV1 | null): Promise<void> => {
  if ((step === 14 || step === 9) && !reloaded.has(ctx) && reloadSelected && blocked === null && invalidatedBy === null) {
    reloaded.add(ctx);
    setCurrentStep(5);
    try {
      await reloadCheck(ctx);
    } catch (error) {
      verdict("reload-check-threw", false, { error: String(error).slice(0, 400) });
    }
  }
};

/** 🔚️ Takes the reload half after the last step when no earlier step already did. */
const afterSteps = async (ctx: Ctx, blocked: TimeTravelBlockV1 | null): Promise<void> => {
  if (reloadSelected && !reloaded.has(ctx) && blocked === null && !cancelled.aborted && invalidatedBy === null) {
    setCurrentStep(5);
    await reloadCheck(ctx);
  }
};

/** ⏪️ The reference scenario of the history-editing design: the puzzle 2d playground driven through a drag, its edit, alternatives, input controls, phone and tablet journeys, long history, two peers and the row actions. */
export const timeTravelScenarioV1: TimeTravelScenarioV1<Ctx> = {
  variant: PLUGIN_VARIANT,
  defaultServes: DEFAULT_SERVES,
  bootWindowPrefix: "2d-",
  editorNumbers: ["dx", "dy"],
  paletteQuery: { en: "Rotate", de: "Drehen" },
  stepNumbers: [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23],
  order: [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 19, 20, 21, 22, 23, 14, 15, 16, 17, 18],
  steps: { 1: step1, 2: step2, 3: step3, 4: step4, 5: step5, 6: step6, 7: step7, 8: step8, 10: step10, 11: step11, 12: step12, 13: step13, 14: step14, 15: step15, 16: step16, 17: step17, 18: step18, 19: step19, 20: step20, 21: step21, 22: step22, 23: step23 },
  context: () => ({}),
  prepare,
  beforeStep,
  afterSteps,
  diagnostics: async () => ({ vitals: await vitals() }),
  exploreInventory: async () => ({ board: await wgpuBoard(), boardContract: WGPU_BOARD_CONTRACT }),
};
