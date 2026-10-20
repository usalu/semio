/** 🏠️ The layered home (design §16–§17): the pages of the overview in reading order and the cell of each (shared
 * vectors), five general cards over the glass and nested topic and quiz grids, each heading linking its page, every
 * page live behind the cards at rest with its card in its cell, opening a page by its card or the step in front and
 * closing it with Escape (the address and the navbar's way back are the client's, `🚏️navigation`), revealing a page behind its hovered card without running any command, the leaderboard polled and the crowd of
 * every quiz asked for while home shows and again with every new submission, the others inside the pages behind the cards, the actions of every quiz card by
 * run state, and the tracks of the live grid the card layer shares at the design system's breakpoints — read from the
 * stylesheet and the overview's track lists as parsed by `lightningcss` (the CSS engine Tailwind itself runs on) as the
 * third-party oracle.
 *
 * @see ../../🧫️fixtures/🏠️home-grid/🔣️.json
 * @see ../../../../🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx — the layered overview and its hooks
 */
import { describe } from "vitest";
import { registerReplaysLanguageAgnosticHierarchyTests } from "./🔶️replays-language-agnostic-hierarchy/🟦️.tsx";
import { registerOpensPageStepFrontTests } from "./🔸️opens-page-step-front/🟦️.tsx";
import { registerZoomsHoveredTopicPansTests } from "./🟠️zooms-hovered-topic-pans/🟦️.tsx";
import { registerHomeOverviewTests } from "./🟢️home-overview/🟦️.tsx";

describe("🏠️ home overview", () => {
  registerReplaysLanguageAgnosticHierarchyTests();
  registerOpensPageStepFrontTests();
  registerZoomsHoveredTopicPansTests();
  registerHomeOverviewTests();
});
