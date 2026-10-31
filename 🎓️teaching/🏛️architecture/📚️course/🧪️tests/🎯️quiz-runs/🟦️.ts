/** 🎯️ Every quiz of the course, played as a whole: a run is started, every task of its sheet answered (classification,
 * sorting, matching — whatever was drawn), nothing of the solution shows before the run is submitted, and the results
 * give a score and feedback per item. Answers come from the course's own sources by item id. One learner plays every
 * quiz perfectly and earns every badge exactly when its rule is met; another makes one mistake per task and earns
 * partial credit, strictly between nothing and everything, and no badge.
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../🔣️.json — the course and its badges */
import { COURSE, PAR, authorPreview, openGrid, openQuizGrid, QUIZZES, answerRun, badgesFor, card, enter, expect, expectFeedback, expectNothingRevealed, goHome, handle, layoutQuiet, pane, playQuiz, primary, quizOf, screen, shownBest, shownItems, startClock, openTaskById, expectSaved, submitRun, test } from "../../🎭️e2e/🚶️learner/🟦️.ts";

test("every quiz is playable to a perfect score, and the perfect runs earn every badge of the course", async ({ device }) => {
  test.setTimeout(20 * 60_000); const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Perfect Player") });
  await authorPreview(learner);
  const played: string[] = [];
  for (const quiz of QUIZZES) {
    await playQuiz(learner, quiz.id);
    await expect(screen(learner.page, "run").getByRole("heading", { level: 1 })).toHaveText(quiz.title.en);
    await expect(primary(screen(learner.page, "run"))).toBeEnabled();
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

  expect(badgesFor(played)).toEqual(COURSE.badges);
  await openQuizGrid(learner, "heating");
  await expect(card(learner.page, quizOf("heating").id)).toContainText(COURSE.badges.find((badge) => badge.id === "heating-expert")!.label.en);
  await openGrid(learner);
  await primary(card(learner.page, "badges")).click();
  const badges = pane(learner.page, "badges");
  await expect(badges).toHaveAttribute("data-opened", "");
  expect(await badges.locator("li[data-earned] h3").allInnerTexts()).toEqual(COURSE.badges.map((badge) => badge.label.en));
  await expect(badges.locator("li:not([data-earned])")).toHaveCount(0);
});

for (const id of ["physics", "demand"]) {
  test(`one mistake per task in ${id} earns partial credit and no badge`, async ({ device }) => {
    const quiz = quizOf(id);
    const learner = await device("de");
    await enter(learner, { kind: "pseudonym", handle: handle("Partial Player") });
    await authorPreview(learner);
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

/** 🔄️ The server rotates balanced sheets and persists individually scored answers in every quiz. */
for (const quiz of QUIZZES) {
  test(`balanced pools rotate ${quiz.id} and preserve individual answers`, async ({ device }) => {
    const learner = await device("en");
    await enter(learner, { kind: "pseudonym", handle: handle("Pool Player") });
    await authorPreview(learner);
    const seen = new Map<string, Set<string>>();
    for (const challenge of ["easy", "medium", "hard", "expert"] as const) {
      await playQuiz(learner, quiz.id, challenge);
      for (const task of quiz.tasks) {
        await openTaskById(learner, task.id);
        if (challenge === "expert") {
          await expect(screen(learner.page, "task").locator("[data-clock]").first()).toContainText("5:00");
          await startClock(learner);
        }
        const ids = await shownItems(learner);
        expect(ids).toHaveLength(task.draw!);
        const previous = seen.get(task.id) ?? new Set<string>();
        expect(ids.filter((id) => previous.has(id))).toEqual([]);
        for (const slot of task.pool!.slots) expect(ids.filter((id) => slot.includes(id))).toHaveLength(1);
        for (const id of ids) previous.add(id);
        seen.set(task.id, previous);
      }
      await answerRun(learner, quiz, "perfect");
      await submitRun(learner);
      const result = await expectFeedback(learner, quiz, "perfect", challenge);
      expect(result.score).toBe(100);
      for (const task of result.tasks) {
        const source = quiz.tasks.find(({ id }) => id === task.task)!;
        expect(task.rows).toHaveLength(source.draw! * (source.dimensions?.length ?? 1));
        const crowd = learner.page.locator(`[data-presence-anchor="result:${task.task}"] [data-crowd-figure="answers"]`);
        await expect(crowd.locator("[data-crowd-item]")).toHaveCount(task.rows.length);
        await expect(crowd.locator("[data-own]")).toHaveCount(task.rows.length);
        const counts = await crowd.locator("[data-own]").evaluateAll((cells) => cells.map((cell) => Number(cell.getAttribute("data-count"))));
        expect(counts.every((count) => count >= 1)).toBe(true);
      }
      await expectSaved(learner);
      await learner.page.reload();
      await layoutQuiet(learner.page);
      await card(learner.page, "learner").getByRole("link").click({ noWaitAfter: true });
      await expect(pane(learner.page, "learner")).toHaveAttribute("data-opened", "");
      await pane(learner.page, "learner").locator('[data-card="runs"] tbody tr').first().getByRole("button", { name: "View result", exact: true }).click();
      await expect(screen(learner.page, "results")).toBeVisible();
      expect(await expectFeedback(learner, quiz, "perfect", challenge)).toEqual(result);
      await goHome(learner);
      await expectSaved(learner);
    }
  });
}

/** 🧑‍💻️ Real authored software previews remain keyboard reachable beside the authoritative answer controls. */
for (const locale of ["en", "de"] as const) test(`software mockups render and grade in ${locale}`, async ({ device }) => {
  const learner = await device(locale);
  await enter(learner, { kind: "anonymous" });
  await authorPreview(learner);
  for (const [id, task, kind] of [["excel-workbook", "workbook-terminology", "excel"], ["grasshopper-data", "canvas-terminology", "grasshopper"]] as const) {
    const quiz = quizOf(id);
    await playQuiz(learner, id);
    await openTaskById(learner, task);
    const preview = screen(learner.page, "task").locator(`figure[data-mockup="${kind}"]`);
    await expect(preview.first()).toBeVisible();
    const region = preview.first().locator('.course-mockup-scroll[role="region"]');
    await region.focus();
    await expect(region).toBeFocused();
    await expect(region).toHaveAttribute("aria-labelledby", /.+/u);
    if (kind === "excel") await expect(preview.first().getByRole("table")).toBeVisible();
    else {
      await expect(preview.first().locator(".course-graph-node").first()).toBeVisible();
      await openTaskById(learner, "tree-operations");
      await preview.first().locator("summary").click();
      await expect(preview.first().locator(".course-graph-relations")).toContainText(locale === "en" ? "Connections" : "Verbindungen");
    }
    await answerRun(learner, quiz, "perfect");
    await submitRun(learner);
    expect((await expectFeedback(learner, quiz, "perfect")).score).toBe(100);
    await goHome(learner);
  }
});
