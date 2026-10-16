/** 🔭️ What the learner looks at and how they find the way: the screen and the trail of steps around it, the crowd of a quiz and the leaderboard, each asking the proctor for what it shows. */

import { type Slug } from "@semio-tech/quiz";
import { type HeldState } from "../🗃️held-state/🟦️.ts";
import { type ProctorReads } from "../📖️proctor-reads/🟦️.ts";
import { HOME_PAGES, stepAbove, type BoardChoice, type QuizStep, type QuizTrail } from "../🧮️state/🟦️.ts";

//#region 🔭️Attention
/** 👀️ The attention of one session. */
export class Attention {
  constructor(
    private readonly state: HeldState,
    private readonly reads: ProctorReads,
  ) {}

  /** 🚪️ Opens a screen and refreshes what it shows from the proctor; a cached run shows at once, also offline. The
   * step left stays on the trail behind — unless the new one shows `instead` of it, as the page the address named on
   * arrival does. */
  open(step: QuizStep, instead = false): void {
    this.state.dispatch({ type: "step-opened", step, instead });
    this.refresh(step);
  }

  /** ◀️ Goes back along the trail to the nearest step that still stands. */
  back(): void {
    this.retrace("back");
  }

  /** ⏭️ Goes forward again along the trail to the nearest step that still stands. */
  forward(): void {
    this.retrace("forward");
  }

  /** 🔼️ Opens the place above the step in front, if there is one. */
  up(): void {
    const above = stepAbove(this.state.read());
    if (above !== undefined) this.open(above);
  }

  /** 👆️ The learner chooses the leaderboard to look at — a period, of every quiz or of one: what is held of it shows at
   * once, and it is asked for again. A quiz the catalog does not list is no choice. */
  chooseBoard(board: BoardChoice): void {
    const before = this.state.read().board;
    this.state.dispatch({ type: "board-chosen", board });
    if (this.state.read().board !== before) void this.reads.refreshLeaderboard();
  }

  /** 👁️ The learner asks to see what the others answered in `quiz` now, before submitting: the crowd held shows at
   * once — also without a connection — and is asked for again. The ask ends with the run. */
  askCrowd(quiz: Slug): void {
    this.state.dispatch({ type: "crowd-asked", quiz });
    void this.reads.refreshCrowd(quiz);
  }

  /** 🫣️ The learner hides the others of `quiz` again until submitting. */
  unaskCrowd(quiz: Slug): void {
    this.state.dispatch({ type: "crowd-unasked", quiz });
  }

  private retrace(to: keyof QuizTrail): void {
    const left = this.state.read().step;
    this.state.dispatch({ type: "step-retraced", to });
    if (this.state.read().step !== left) this.refresh(this.state.read().step);
  }

  private refresh(step: QuizStep): void {
    if (step.screen === "home" && (step.page === undefined || step.page === HOME_PAGES.learner || step.page === HOME_PAGES.badges)) void this.reads.refreshLearner();
    if (step.screen === "home" && step.page === HOME_PAGES.leaderboard) void this.reads.refreshLeaderboard();
    if (step.screen === "run" || step.screen === "results")
      void this.reads.loadRun(step.run).then((view) => {
        if (view !== undefined) void this.reads.refreshCrowd(view.quiz);
      });
  }
}
//#endregion 🔭️Attention
