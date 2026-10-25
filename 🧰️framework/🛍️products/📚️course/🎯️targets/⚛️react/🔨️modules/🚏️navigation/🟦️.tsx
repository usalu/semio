/** 🚏️ Finding the way through the client. The navbar carries four ways on its left: the overview, back and forward along
 * the trail of the steps the learner went through, and up to the place above — each named with the place it leads to,
 * and kept focusable but inactive where it leads nowhere, so the bar never changes its shape and focus is never lost.
 * The address names the page of the overview that shows (`#board`) and nothing anywhere else: on arrival a hash that
 * names a page opens it instead of the overview, a hash that changes later (typed, a card's heading link) opens its
 * page like any opening, and the address follows the step without writing entries into the browser's history.
 *
 * @see ../🧭️session/🧮️state/🟦️.ts — the trail and the place above a step
 * @see ../../../../🧫️fixtures/🚏️navigation/🔣️.json — the shared vectors
 * @see https://www.w3.org/WAI/ARIA/apg/patterns/toolbar/ — controls that stay focusable while unavailable
 */

import { useEffect, useLayoutEffect, useMemo, useRef, type ReactElement } from "react";
import { Icon, cn, overviewCardChipClass, type IconName } from "@semio-tech/ui-react/chrome";
import { localized, type CourseLabelKey, type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";
import { CHALLENGE_LABELS } from "../⛰️challenge/🏷️labels/🟦️.ts";
import { addressPages, pageLabel } from "../🏠️home/🗺️pages/🟦️.ts";
import { sameStep, stepAbove, stepAfter, stepBefore, type CourseState, type CourseStep } from "../🧭️session/🧮️state/🟦️.ts";
import { type CourseSession } from "../🧭️session/🟦️.ts";

//#region 🏷️Places
/** 🏷️ What the place a step shows is called: the overview, one of its pages, a run by its quiz and its challenge,
 * results by their quiz; nothing for a step that is no place, an unknown page and a run whose view has not arrived. */
export function placeName(step: CourseStep, state: Pick<CourseState, "course" | "learner" | "runs">, locale: CourseLocale, text: CourseText): string | undefined {
  if (step.screen === "home") return step.page === undefined ? text("course.nav.home") : pageLabel(step.page, state, locale, text);
  if (step.screen !== "run" && step.screen !== "results") return undefined;
  const view = state.runs[step.run];
  if (view === undefined) return undefined;
  const quiz = localized(view.sheet.title, locale);
  return step.screen === "run" ? text("course.nav.run", { quiz, challenge: text(CHALLENGE_LABELS[view.sheet.challenge]) }) : text("course.results.title", { quiz });
}
//#endregion 🏷️Places

//#region 🚏️Ways
/** 🧭️ The ways the navbar offers, in the order it shows them. */
export const NAVIGATION_WAYS = ["overview", "back", "forward", "up"] as const;

/** 🔃️ One way of the navbar. */
export type NavigationWay = (typeof NAVIGATION_WAYS)[number];

const WAY_ICONS: { readonly [W in NavigationWay]: IconName } = { overview: "layout-grid", back: "arrow-left", forward: "arrow-right", up: "arrow-up" };

const WAY_LABELS: { readonly [W in Exclude<NavigationWay, "overview">]: { readonly bare: CourseLabelKey; readonly to: CourseLabelKey } } = {
  back: { bare: "course.nav.back", to: "course.nav.backTo" },
  forward: { bare: "course.nav.forward", to: "course.nav.forwardTo" },
  up: { bare: "course.nav.up", to: "course.nav.upTo" },
};

/** 🛤️ Where each way leads from `state`; nothing where it leads nowhere. */
export function navigationWays(state: CourseState): { readonly [W in NavigationWay]: CourseStep | undefined } {
  const atOverview = state.step.screen === "home" && state.step.page === undefined;
  return { overview: atOverview ? undefined : { screen: "home" }, back: stepBefore(state), forward: stepAfter(state), up: stepAbove(state) };
}

/** 🎛️ The ways of the navbar as one group of buttons: each an icon and its word (Overview, Back, Forward, Up), and
 * named — also for the pointer that rests on them — with the place it leads to. A way that leads nowhere stays in its
 * place and in the tab order and says that it is unavailable; Escape is named as Up's key while a home page is open. */
export function NavigationControls(props: { readonly session: CourseSession; readonly state: CourseState; readonly locale: CourseLocale; readonly text: CourseText }): ReactElement {
  const { session, state, locale, text } = props;
  const ways = navigationWays(state);
  const go: { readonly [W in NavigationWay]: () => void } = { overview: () => session.open({ screen: "home" }), back: () => session.back(), forward: () => session.forward(), up: () => session.up() };
  const caption = (way: NavigationWay): string => (way === "overview" ? text("course.nav.home") : text(WAY_LABELS[way].bare));
  const label = (way: NavigationWay): string => {
    if (way === "overview") return caption(way);
    const target = ways[way];
    const place = target === undefined ? undefined : placeName(target, state, locale, text);
    return place === undefined ? caption(way) : text(WAY_LABELS[way].to, { place });
  };
  return (
    <div role="group" aria-label={text("course.nav.ways")} className="flex w-fit border border-normal">
      {NAVIGATION_WAYS.map((way) => (
        <button
          key={way}
          type="button"
          data-course-nav={way}
          aria-label={label(way)}
          title={label(way)}
          aria-disabled={ways[way] === undefined}
          aria-keyshortcuts={way === "up" && state.step.screen === "home" && ways.up !== undefined ? "Escape" : undefined}
          onClick={() => ways[way] !== undefined && go[way]()}
          className={cn(overviewCardChipClass, "course-target min-w-[1.5rem] cursor-pointer justify-center text-muted-foreground transition-colors hover:text-foreground aria-disabled:cursor-not-allowed aria-disabled:opacity-50 aria-disabled:hover:text-muted-foreground")}
        >
          <Icon icon={WAY_ICONS[way]} size="small" />
          <span className="whitespace-nowrap">{caption(way)}</span>
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

/** 🪜️ The hash that names `step`: its page of the overview; empty on the overview itself and on every other screen. */
export function stepAddress(step: CourseStep): string {
  return step.screen === "home" && step.page !== undefined ? `#${step.page}` : "";
}

/** 🔂️ Binds the address to the current overview once its course is known. On arrival a hash that names a
 * page opens that page instead of the overview; afterwards the hash follows the step — replaced in place, never a new
 * entry of the browser's history, and left alone while it names no page and none shows (a skip link's target) — and a
 * hash changed from outside opens the page it names, or the overview when it was emptied. The hash is written only
 * when the step moved: a course that arrives again (the proctor's answer after the one held on the device) changes
 * nothing about the step, and a hash set from outside whose change has not been delivered yet is not a stale address. */
export function useAddress(session: CourseSession, state: CourseState): void {
  const course = state.course;
  const pages = useMemo(() => (course === undefined ? undefined : addressPages(course)), [course]);
  const arrived = useRef(false);
  const mirrored = useRef<CourseStep | undefined>(undefined);
  const { step } = state;
  useLayoutEffect(() => {
    if (pages === undefined) return;
    const named = addressPage(window.location.hash, pages);
    const shown = session.getSnapshot().state.step;
    if (!arrived.current) {
      arrived.current = true;
      if (named !== undefined && shown.screen === "home" && shown.page === undefined) return session.open({ screen: "home", page: named }, true);
    }
    const moved = mirrored.current === undefined || !sameStep(mirrored.current, shown);
    mirrored.current = shown;
    if (!moved) return;
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
