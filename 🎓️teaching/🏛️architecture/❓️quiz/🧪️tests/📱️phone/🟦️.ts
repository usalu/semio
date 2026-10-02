/** 📱️ On a phone (375 × 812) home is a list — one section per page, in reading order, each as tall as the screen, its
 * card as tall as what it holds and centred — and a quiz is playable from start to results without anything spilling
 * over the edge of the screen or scrolling the screen sideways.
 * @see ../../🎭️e2e/🎚️config/🟦️.ts — the `phone` project that holds this phone
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive */
import { QUIZZES, answerRun, boxOf, card, enter, expect, expectFeedback, goHome, percent, playQuiz, quizOf, screen, submitRun, test, unresolvedLabels, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

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
  expect(percent(await card(learner.page, quiz.id).locator("li", { hasText: "%" }).innerText())).toBe(100);
});
