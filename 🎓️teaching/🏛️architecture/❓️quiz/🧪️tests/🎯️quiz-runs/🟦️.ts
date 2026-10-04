/** 🎯️ Every quiz of the catalog, played as a whole: a run is started, every task of its sheet answered (classification,
 * sorting, matching — whatever was drawn), nothing of the solution shows before the run is submitted, and the results
 * give a score and feedback per item. Answers come from the catalog's own sources by item id. One learner plays every
 * quiz perfectly and earns every badge exactly when its rule is met; another makes one mistake per task and earns
 * partial credit, strictly between nothing and everything, and no badge.
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../🔣️.json — the catalog and its badges */
import { CATALOG, PAR, QUIZZES, answerRun, badgesFor, card, enter, expect, expectFeedback, expectNothingRevealed, goHome, handle, pane, playQuiz, primary, quizOf, screen, shownBest, submitRun, test } from "../../🎭️e2e/🚶️learner/🟦️.ts";

test("every quiz is playable to a perfect score, and the perfect runs earn every badge of the catalog", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Perfect Player") });
  const played: string[] = [];
  for (const quiz of QUIZZES) {
    await playQuiz(learner, quiz.id);
    await expect(screen(learner.page, "run").getByRole("heading", { level: 1 })).toHaveText(quiz.title.en);
    await expect(primary(screen(learner.page, "run"))).toBeDisabled();
    await expectNothingRevealed(learner, quiz);
    await answerRun(learner, quiz, "perfect");
    await expectNothingRevealed(learner, quiz);
    await submitRun(learner);
    await expect(screen(learner.page, "results").getByRole("heading", { level: 1 })).toContainText(quiz.title.en);
    expect((await expectFeedback(learner, quiz, "perfect")).score).toBe(100);

    const before = badgesFor(played);
    played.push(quiz.id);
    const earned = badgesFor(played).filter((badge) => !before.includes(badge));
    expect(await screen(learner.page, "results").locator("[data-earned] h3").allInnerTexts()).toEqual(earned.map((badge) => badge.label.en));

    await goHome(learner);
    expect(await shownBest(learner, quiz.id)).toEqual({ challenge: "medium", points: PAR.medium, par: PAR.medium });
    await expect(card(learner.page, "badges").locator("li[data-earned]")).toHaveCount(badgesFor(played).length);
  }

  expect(badgesFor(played)).toEqual(CATALOG.badges);
  await expect(card(learner.page, quizOf("heating").id)).toContainText(CATALOG.badges.find((badge) => badge.id === "heating-expert")!.label.en);
  await primary(card(learner.page, "badges")).click();
  const badges = pane(learner.page, "badges");
  await expect(badges).toHaveAttribute("data-opened", "");
  expect(await badges.locator("li[data-earned] h3").allInnerTexts()).toEqual(CATALOG.badges.map((badge) => badge.label.en));
  await expect(badges.locator("li:not([data-earned])")).toHaveCount(0);
});

for (const id of ["physics", "demand"]) {
  test(`one mistake per task in ${id} earns partial credit and no badge`, async ({ device }) => {
    const quiz = quizOf(id);
    const learner = await device("de");
    await enter(learner, { kind: "pseudonym", handle: handle("Partial Player") });
    await playQuiz(learner, quiz.id);
    await answerRun(learner, quiz, "flawed");
    await expectNothingRevealed(learner, quiz);
    await submitRun(learner);
    const results = await expectFeedback(learner, quiz, "flawed");
    expect(results.score).toBeGreaterThan(0);
    expect(results.score).toBeLessThan(100);
    for (const task of results.tasks) {
      expect(task.score, `${task.task}: partial credit`).toBeLessThan(100);
      const right = task.rows.filter((row) => row.text.includes("✓")).length;
      const wrong = task.rows.filter((row) => /[✗◐]/u.test(row.text)).length;
      expect(wrong, `${task.task}: the mistake is marked`).toBeGreaterThan(0);
      expect(right, `${task.task}: the correct items are marked`).toBeGreaterThan(wrong);
    }
    await expect(screen(learner.page, "results").locator("[data-earned]")).toHaveCount(0);
    await goHome(learner);
    expect(await shownBest(learner, quiz.id)).toEqual({ challenge: "medium", points: results.points, par: PAR.medium });
    await expect(card(learner.page, "badges").locator("li[data-earned]")).toHaveCount(0);
  });
}
