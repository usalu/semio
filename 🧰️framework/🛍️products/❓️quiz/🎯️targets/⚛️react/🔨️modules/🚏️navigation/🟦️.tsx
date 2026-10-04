/** 🚏️ Finding the way through the client. The navbar carries four ways on its left: the overview, back and forward along
 * the trail of the steps the learner went through, and up to the place above — each named with the place it leads to,
 * and kept focusable but inactive where it leads nowhere, so the bar never changes its shape and focus is never lost.
 * The address names the page of the overview that shows (`#board`) and nothing anywhere else: on arrival a hash that
 * names a page opens it instead of the overview, a hash that changes later (typed, a card's heading link) opens its
 * page like any opening, and the address follows the step without writing entries into the browser's history.
 *
 * @see ../🧭️session/🟦️.ts — the trail and the place above a step
 * @see ../../../../🧫️fixtures/🚏️navigation/🔣️.json — the shared vectors
 * @see https://www.w3.org/WAI/ARIA/apg/patterns/toolbar/ — controls that stay focusable while unavailable
 */

import { useEffect, useLayoutEffect, useMemo, useRef, type ReactElement } from "react";
import { Icon, cn, overviewCardChipClass, type IconName } from "@semio-tech/ui-react/chrome";
import { localized, type QuizLabelKey, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { CHALLENGE_LABELS } from "../⛰️challenge/🟦️.tsx";
import { homePages, pageLabel } from "../🏠️home/🟦️.tsx";
import { stepAbove, stepAfter, stepBefore, type QuizSession, type QuizState, type QuizStep } from "../🧭️session/🟦️.ts";

//#region 🏷️Places
/** 🏷️ What the place a step shows is called: the overview, one of its pages, a run by its quiz and its challenge,
 * results by their quiz; nothing for a step that is no place, an unknown page and a run whose view has not arrived. */
export function placeName(step: QuizStep, state: Pick<QuizState, "catalog" | "learner" | "runs">, locale: QuizLocale, text: QuizText): string | undefined {
  if (step.screen === "home") return step.page === undefined ? text("quiz.nav.home") : pageLabel(step.page, state, locale, text);
  if (step.screen !== "run" && step.screen !== "results") return undefined;
  const view = state.runs[step.run];
  if (view === undefined) return undefined;
  const quiz = localized(view.sheet.title, locale);
  return step.screen === "run" ? text("quiz.nav.run", { quiz, challenge: text(CHALLENGE_LABELS[view.sheet.challenge]) }) : text("quiz.results.title", { quiz });
}
//#endregion 🏷️Places

//#region 🚏️Ways
/** 🚏️ The ways the navbar offers, in the order it shows them. */
export const NAVIGATION_WAYS = ["overview", "back", "forward", "up"] as const;

/** 🚏️ One way of the navbar. */
export type NavigationWay = (typeof NAVIGATION_WAYS)[number];

const WAY_ICONS: { readonly [W in NavigationWay]: IconName } = { overview: "layout-grid", back: "arrow-left", forward: "arrow-right", up: "arrow-up" };

const WAY_LABELS: { readonly [W in Exclude<NavigationWay, "overview">]: { readonly bare: QuizLabelKey; readonly to: QuizLabelKey } } = {
  back: { bare: "quiz.nav.back", to: "quiz.nav.backTo" },
  forward: { bare: "quiz.nav.forward", to: "quiz.nav.forwardTo" },
  up: { bare: "quiz.nav.up", to: "quiz.nav.upTo" },
};

/** 🚏️ Where each way leads from `state`; nothing where it leads nowhere. */
export function navigationWays(state: QuizState): { readonly [W in NavigationWay]: QuizStep | undefined } {
  const atOverview = state.step.screen === "home" && state.step.page === undefined;
  return { overview: atOverview ? undefined : { screen: "home" }, back: stepBefore(state), forward: stepAfter(state), up: stepAbove(state) };
}

/** 🚏️ The ways of the navbar as one group of buttons: the overview with its word beside the icon from tablet width,
 * the others as icons named — also for the pointer that rests on them — with the place they lead to. A way that leads
 * nowhere stays in its place and in the tab order and says that it is unavailable; Escape is named as the overview's
 * key while a page of the overview is open. */
export function NavigationControls(props: { readonly session: QuizSession; readonly state: QuizState; readonly locale: QuizLocale; readonly text: QuizText }): ReactElement {
  const { session, state, locale, text } = props;
  const ways = navigationWays(state);
  const go: { readonly [W in NavigationWay]: () => void } = { overview: () => session.open({ screen: "home" }), back: () => session.back(), forward: () => session.forward(), up: () => session.up() };
  const label = (way: NavigationWay): string => {
    if (way === "overview") return text("quiz.nav.home");
    const target = ways[way];
    const place = target === undefined ? undefined : placeName(target, state, locale, text);
    return place === undefined ? text(WAY_LABELS[way].bare) : text(WAY_LABELS[way].to, { place });
  };
  return (
    <div role="group" aria-label={text("quiz.nav.ways")} className="flex w-fit border border-normal">
      {NAVIGATION_WAYS.map((way) => (
        <button
          key={way}
          type="button"
          data-quiz-nav={way}
          aria-label={label(way)}
          title={label(way)}
          aria-disabled={ways[way] === undefined}
          aria-keyshortcuts={way === "overview" && state.step.screen === "home" && ways.overview !== undefined ? "Escape" : undefined}
          onClick={() => ways[way] !== undefined && go[way]()}
          className={cn(overviewCardChipClass, "quiz-target min-w-[1.5rem] cursor-pointer justify-center text-muted-foreground transition-colors hover:text-foreground aria-disabled:cursor-not-allowed aria-disabled:opacity-50 aria-disabled:hover:text-muted-foreground")}
        >
          <Icon icon={WAY_ICONS[way]} size="small" />
          {way === "overview" ? <span className="max-md:sr-only">{label(way)}</span> : null}
        </button>
      ))}
    </div>
  );
}
//#endregion 🚏️Ways

//#region 🔗️Address
/** 🔗️ The page of `pages` that `hash` names, if any. */
export function addressPage(hash: string, pages: readonly string[]): string | undefined {
  const named = hash.startsWith("#") ? hash.slice(1) : hash;
  return pages.includes(named) ? named : undefined;
}

/** 🔗️ The hash that names `step`: its page of the overview; empty on the overview itself and on every other screen. */
export function stepAddress(step: QuizStep): string {
  return step.screen === "home" && step.page !== undefined ? `#${step.page}` : "";
}

/** 🔗️ Binds the address to the step of an identified learner whose catalog is known. On arrival a hash that names a
 * page opens that page instead of the overview; afterwards the hash follows the step — replaced in place, never a new
 * entry of the browser's history, and left alone while it names no page and none shows (a skip link's target) — and a
 * hash changed from outside opens the page it names, or the overview when it was emptied. */
export function useAddress(session: QuizSession, state: QuizState): void {
  const quizzes = state.introduced && state.learner !== undefined ? state.catalog?.quizzes.map((quiz) => quiz.id).join(" ") : undefined;
  const pages = useMemo(() => (quizzes === undefined ? undefined : homePages(quizzes.split(" ").filter(Boolean))), [quizzes]);
  const arrived = useRef(false);
  const { step } = state;
  useLayoutEffect(() => {
    if (pages === undefined) return;
    const named = addressPage(window.location.hash, pages);
    const shown = session.getSnapshot().state.step;
    if (!arrived.current) {
      arrived.current = true;
      if (named !== undefined && shown.screen === "home" && shown.page === undefined) return session.open({ screen: "home", page: named }, true);
    }
    const wanted = stepAddress(shown);
    if (wanted === "" ? named === undefined : window.location.hash === wanted) return;
    window.history.replaceState(window.history.state, "", wanted === "" ? window.location.pathname + window.location.search : wanted);
  }, [pages, session, step]);
  useEffect(() => {
    if (pages === undefined) return;
    const follow = (): void => {
      const page = addressPage(window.location.hash, pages);
      if (page !== undefined) session.open({ screen: "home", page });
      else if (window.location.hash === "") session.open({ screen: "home" });
    };
    window.addEventListener("hashchange", follow);
    return () => window.removeEventListener("hashchange", follow);
  }, [pages, session]);
}
//#endregion 🔗️Address
