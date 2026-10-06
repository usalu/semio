/** 📱️ On a phone (375 × 812) home is the same grid as on a desktop, one page the size of the screen at a time — the
 * learner swipes along the rows and along the columns from page to page with a real touch, or taps the hint that names
 * the page behind an edge; every card stands on its own page, as tall as what it holds and centred — and a quiz is
 * playable from start to results without anything spilling over the edge of the screen or scrolling the screen sideways. On hard every guess field lies within the screen, is tall
 * enough to hit and takes its guess, and the categories keep their descriptions to themselves.
 * @see ../../🎭️e2e/🎚️config/🟦️.ts — the `phone` project that holds this phone
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive */
import { PAR, QUIZZES, answerRun, answerTask, boxOf, card, enter, expect, expectFeedback, goHome, handle, openTask, playQuiz, quizOf, restingPage, screen, shownBest, shownTask, shownText, submitRun, swipeTo, taskOf, test, touchSwipe, unresolvedLabels, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

/** ↔️ How many pixels the document is wider than the screen. */
async function overflow(device: Device): Promise<number> {
  return device.page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
}

/** ↔️ How many pixels the screen in front scrolls sideways at most: every box of it that scrolls, but for the frames in
 * which a table scrolls on its own. */
async function sideways(device: Device): Promise<number> {
  return device.page.evaluate(() =>
    Math.max(
      0,
      ...[...document.querySelectorAll<HTMLElement>("#quiz-main, #quiz-main *")]
        .filter((box) => box.closest("[inert]") === null && /auto|scroll/u.test(getComputedStyle(box).overflowX) && box.firstElementChild?.tagName !== "TABLE")
        .map((box) => box.scrollWidth - box.clientWidth),
    ),
  );
}

test("home is the same grid on a phone, swiped page by page in both directions, and a quiz is playable", async ({ device }) => {
  const learner = await device("de");
  expect(learner.page.viewportSize()).toEqual({ width: 375, height: 812 });
  await enter(learner, { kind: "anonymous" });
  const root = learner.page.locator("[data-layered-overview]");
  await expect(root).toHaveAttribute("data-mode", "swipe");
  const [first, second, third, fourth] = QUIZZES.map((quiz) => quiz.id);
  const ring = [["learner", 0, 0], [first, 1, 0], ["intro", 2, 0], [second, 0, 1], ["board", 1, 1], [third, 2, 1], ["badges", 0, 2], [fourth, 1, 2], ["prefs", 2, 2]];
  expect(await root.locator("[data-layered-cell]").evaluateAll((cells) => cells.map((cell) => [(cell as HTMLElement).dataset.layeredCell, Number((cell as HTMLElement).style.gridColumn) - 1, Number((cell as HTMLElement).style.gridRow) - 1])), "the cards of the desktop's ring of nine, in reading order").toEqual(ring);
  const view = await boxOf(root);
  const resting = await boxOf(root.locator('[data-layered-cell="learner"]'));
  expect([Math.round(resting.x), Math.round(resting.y), Math.round(resting.width), Math.round(resting.height)], "the learner's page fills the overview").toEqual([Math.round(view.x), Math.round(view.y), 375, Math.round(view.height)]);
  expect(await restingPage(learner.page)).toBe("learner");
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  expect(await unresolvedLabels(learner)).toEqual([]);
  const cards = await root.locator("[data-layered-cell]").evaluateAll((cells) =>
    cells.map((cell) => {
      const held = cell.querySelector<HTMLElement>("[data-layered-card] [data-card]")!;
      return { centred: getComputedStyle(held.closest("[data-layered-card]")!.parentElement!).alignItems, cut: held.scrollHeight - held.clientHeight };
    }),
  );
  expect(cards, "every card is centred on its page where it fits and cut nowhere").toEqual(cards.map(() => ({ centred: "safe center", cut: 0 })));

  await touchSwipe(learner.page, 240, 0);
  await expect.poll(() => restingPage(learner.page), { message: "before the first column lies the last one: the ring wraps" }).toBe("intro");
  await touchSwipe(learner.page, -240, 0);
  await expect.poll(() => restingPage(learner.page), { message: "and back across the edge" }).toBe("learner");
  await touchSwipe(learner.page, 0, 325);
  await expect.poll(() => restingPage(learner.page), { message: "above the top row lies the bottom row" }).toBe("badges");
  await touchSwipe(learner.page, 0, -325);
  await expect.poll(() => restingPage(learner.page), { message: "and below the bottom row the top row" }).toBe("learner");
  await touchSwipe(learner.page, -60, 0, 600);
  await expect.poll(() => restingPage(learner.page), { message: "a short slow swipe stays" }).toBe("learner");
  const hints = () => root.locator("[data-layered-neighbour]").evaluateAll((all) => all.map((hint) => [(hint as HTMLElement).dataset.layeredNeighbour, (hint as HTMLElement).dataset.pane]));
  expect(await hints(), "every edge of the learner's page names the page behind it, across the edges the far side's").toEqual([["up", "badges"], ["left", "intro"], ["right", first], ["down", second]]);
  await expect(root.locator('[data-layered-neighbour="right"]')).toHaveAttribute("aria-label", `Nach rechts zu ${QUIZZES[0]!.title.de}`);
  await expect(root.locator('[data-layered-neighbour="right"]')).toContainText(QUIZZES[0]!.title.de);
  for (const hint of await root.locator("[data-layered-neighbour]").all()) await expect(hint).toBeInViewport({ ratio: 1 });
  await root.locator('[data-layered-neighbour="down"]').tap();
  await expect.poll(() => restingPage(learner.page), { message: "a tap on a hint goes where it points" }).toBe(second);
  await expect.poll(hints).toEqual([["up", "learner"], ["left", third], ["right", "board"], ["down", "badges"]]);
  await swipeTo(learner, "learner");
  const quiz = quizOf("cooling");
  await swipeTo(learner, quiz.id);
  await expect(card(learner.page, quiz.id).getByRole("heading")).toHaveText(quiz.title.de);
  await swipeTo(learner, "learner");
  await swipeTo(learner, quiz.id);
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  await playQuiz(learner, quiz.id);
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  expect(await sideways(learner)).toBe(0);
  await answerRun(learner, quiz, "perfect");
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  expect(await sideways(learner)).toBe(0);
  await submitRun(learner);
  expect((await expectFeedback(learner, quiz, "perfect")).score).toBe(100);
  await expect(screen(learner.page, "results").getByRole("heading", { level: 1 })).toContainText(quiz.title.de);
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  expect(await sideways(learner)).toBe(0);
  await goHome(learner);
  await expect(root).toHaveAttribute("data-mode", "swipe");
  await swipeTo(learner, quiz.id);
  expect(await shownBest(learner, quiz.id)).toEqual({ challenge: "medium", points: PAR.medium, par: PAR.medium });
});

test("a hard run on a phone: every guess field lies within the screen, is tall enough to hit and takes its guess", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Phone Guesser") });
  const quiz = quizOf("physics");
  await playQuiz(learner, quiz.id, "hard");
  const steps = screen(learner.page, "run").locator("nav button");
  for (let index = 0; index < quiz.tasks.length; index++) {
    await openTask(learner, index);
    const fields = screen(learner.page, "task").locator("[data-quiz-item] input");
    for (const field of await fields.all()) {
      const box = await boxOf(field);
      expect(box.x, "a guess field starts on the screen").toBeGreaterThanOrEqual(0);
      expect(box.x + box.width, "a guess field ends on the screen").toBeLessThanOrEqual(375);
      expect(box.height, "a guess field is at least 24 px tall").toBeGreaterThanOrEqual(24);
    }
    const task = taskOf(quiz, await shownTask(learner));
    if (task.kind === "classification") for (const category of task.categories!) if (category.description !== undefined) expect(await shownText(learner)).not.toContain(category.description.en);
    await answerTask(learner, quiz, "perfect");
    await expect(steps.nth(index).locator("[data-complete]")).toBeVisible();
    expect(await overflow(learner)).toBeLessThanOrEqual(0);
    expect(await sideways(learner)).toBe(0);
  }
  await submitRun(learner);
  expect((await expectFeedback(learner, quiz, "perfect", "hard")).points).toBe(PAR.hard);
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  expect(await sideways(learner)).toBe(0);
  expect(await unresolvedLabels(learner)).toEqual([]);
});
