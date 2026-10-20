/** 📡️ What the pet layer sees of a page: the survey turns element boxes into the `surveyed` event of the stage (shared
 * vectors, expected by a second reading of the design written in Python), names every surface once and for good, reads
 * without writing, knows when a measurement went stale and turns the pointer into stage events without ever touching
 * an event. Which elements count as controls is judged against the WAI-ARIA role model of `aria-query`, queried through
 * `@testing-library`'s own role lookup.
 *
 * @see ../../🧫️fixtures/📡️surface-survey/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts
 * @see https://www.w3.org/TR/wai-aria-1.2/#widget_roles
 */
import { afterEach, describe, vi } from "vitest";
import { registerShipsVectorsRuleSurveyTests } from "./🔶️ships-vectors-rule-survey/🟦️.tsx";
import { registerTellsSurveyWentStaleTests } from "./🔸️tells-survey-went-stale/🟦️.tsx";
import { registerTurnsPointerStageEventsTests } from "./🟠️turns-pointer-stage-events/🟦️.tsx";
import { registerNamesFixtureOnceGoodTests } from "./🟢️names-fixture-once-good/🟦️.tsx";

afterEach(() => {
  document.body.replaceChildren();
  Reflect.deleteProperty(document.documentElement, "clientWidth");
  Reflect.deleteProperty(document.documentElement, "clientHeight");
  vi.restoreAllMocks();
});

describe("📡️ surface survey", () => {
  registerShipsVectorsRuleSurveyTests();
  registerTellsSurveyWentStaleTests();
  registerTurnsPointerStageEventsTests();
  registerNamesFixtureOnceGoodTests();
});
