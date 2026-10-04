/** 📱️ On a phone (375 × 812) home is a list — one section per page, in reading order, each as tall as the screen, its
 * card as tall as what it holds and centred — and a quiz is playable from start to results without anything spilling
 * over the edge of the screen or scrolling the screen sideways. On hard every guess field lies within the screen, is
 * tall enough to hit and takes its guess, and the categories keep their descriptions to themselves.
 * @see ../../🎭️e2e/🎚️config/🟦️.ts — the `phone` project that holds this phone
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive */
import { PAR, QUIZZES, answerRun, answerTask, boxOf, card, enter, expect, expectFeedback, goHome, handle, openTask, playQuiz, quizOf, screen, shownBest, shownTask, shownText, submitRun, taskOf, test, unresolvedLabels, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

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

test("home is a list on a phone and a quiz is playable", async ({ device }) => {
  const learner = await device("de");
  expect(learner.page.viewportSize()).toEqual({ width: 375, height: 812 });
  await enter(learner, { kind: "anonymous" });
  const root = learner.page.locator("[data-layered-overview]");
  await expect(root).toHaveAttribute("data-mode", "list");
  const [first, second, third, fourth] = QUIZZES.map((quiz) => quiz.id);
  expect(await root.locator("[data-layered-section]").evaluateAll((sections) => sections.map((section) => (section as HTMLElement).dataset.layeredSection))).toEqual(["learner", first, "intro", second, "board", third, "badges", fourth, "prefs"]);
  const list = await boxOf(root.locator("[data-layered-list]"));
  const section = await boxOf(root.locator('[data-layered-section="learner"]'));
  expect(Math.round(section.height)).toBe(Math.round(list.height));
  expect(Math.round(section.width)).toBe(375);
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  expect(await unresolvedLabels(learner)).toEqual([]);
  const cards = await root.locator("[data-layered-section]").evaluateAll((sections) =>
    sections.map((section) => {
      const held = section.querySelector<HTMLElement>("[data-layered-card] [data-card]")!;
      return { centred: getComputedStyle(held.closest("[data-layered-card]")!.parentElement!).alignItems, cut: held.scrollHeight - held.clientHeight, fills: held.getBoundingClientRect().height >= section.getBoundingClientRect().height / 2 };
    }),
  );
  expect(cards.map(({ centred, cut }) => ({ centred, cut })), "every card of the list is centred where it fits and cut nowhere").toEqual(cards.map(() => ({ centred: "safe center", cut: 0 })));

  const quiz = quizOf("cooling");
  await card(learner.page, quiz.id).scrollIntoViewIfNeeded();
  await expect(card(learner.page, quiz.id).getByRole("heading")).toHaveText(quiz.title.de);
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
  await expect(root).toHaveAttribute("data-mode", "list");
  await card(learner.page, quiz.id).scrollIntoViewIfNeeded();
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
