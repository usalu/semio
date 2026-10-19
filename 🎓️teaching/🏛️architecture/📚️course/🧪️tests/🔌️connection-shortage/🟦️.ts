/** 🔌️ Short connection shortages: a reload in the middle of a run keeps every answer, and while the proctor is away for
 * a few seconds the app keeps working — answers are taken, tasks open, the connection indicator says what happens — and
 * the answers given meanwhile reach the proctor once it is back, where a fresh device finds them. The spec stops and
 * starts the proctor of its stack through the gate's control endpoint, so it runs alone, last.
 * @see ../../🎭️e2e/🟦️.ts — the control endpoint
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive */
import { answerRun, answerTask, connection, enter, expect, expectFeedback, expectSaved, handle, openTask, openTaskById, playQuiz, proctor, quizOf, screen, shownItems, shownTask, submitRun, test, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

/** 📝️ What the task on screen holds as answers: every choice by the name of its control, and the order of the items. */
async function shownAnswers(device: Device): Promise<{ readonly choices: Readonly<Record<string, string>>; readonly order: readonly string[] }> {
  const task = screen(device.page, "task");
  const choices = await task.locator("select").evaluateAll((selects) => Object.fromEntries(selects.map((select) => [select.getAttribute("aria-label") ?? "", (select as HTMLSelectElement).selectedOptions[0]?.text ?? ""])));
  return { choices, order: await shownItems(device) };
}

test("a reload in the middle of a run keeps the answers", async ({ device }) => {
  const quiz = quizOf("cooling");
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Reloader") });
  await playQuiz(learner, quiz.id);
  await openTask(learner, 0);
  const task = await shownTask(learner);
  await answerTask(learner, quiz, "perfect");
  const before = await shownAnswers(learner);
  expect(Object.values(before.choices).every((choice) => choice !== "")).toBe(true);

  await learner.page.reload();
  await playQuiz(learner, quiz.id);
  await openTaskById(learner, task);
  expect(await shownAnswers(learner)).toEqual(before);
  await expect(screen(learner.page, "run").locator("nav [data-complete]")).toHaveCount(1);
  await expectSaved(learner);
});

test("the app keeps working while the proctor is away, and the answers arrive once it is back", async ({ device }) => {
  test.setTimeout(300_000);
  const quiz = quizOf("physics");
  const name = handle("Patient");
  const learner = await device("de");
  await enter(learner, { kind: "pseudonym", handle: name });
  await playQuiz(learner, quiz.id);
  await openTask(learner, 0);
  await answerTask(learner, quiz, "perfect");
  await expectSaved(learner);

  let given: Awaited<ReturnType<typeof shownAnswers>> | undefined;
  let task = "";
  const failures = await learner.expectingFailures(async () => {
    await proctor("stop");
    await openTask(learner, 1);
    task = await shownTask(learner);
    await answerTask(learner, quiz, "perfect");
    given = await shownAnswers(learner);
    await expect(screen(learner.page, "run").locator("nav [data-complete]")).toHaveCount(2);
    await expect(connection(learner)).toHaveAttribute("data-tone", "alert", { timeout: 30_000 });
    await openTask(learner, 0);
    await openTask(learner, 1);
    expect(await shownAnswers(learner)).toEqual(given);
    await proctor("start");
    await expect(connection(learner)).toHaveAttribute("data-tone", "calm", { timeout: 90_000 });
    await expect(learner.page.locator("[data-presence-status]")).toBeVisible({ timeout: 90_000 });
  });
  expect(failures, "the shortage was real: requests failed while the proctor was away").toBeGreaterThan(0);

  const elsewhere = await device("de");
  await enter(elsewhere, { kind: "pseudonym", handle: name });
  await playQuiz(elsewhere, quiz.id);
  await expect(screen(elsewhere.page, "run").locator("nav [data-complete]")).toHaveCount(2);
  await openTaskById(elsewhere, task);
  expect(await shownAnswers(elsewhere)).toEqual(given);

  await openTask(learner, 2);
  await answerRun(learner, quiz, "perfect");
  await submitRun(learner);
  expect((await expectFeedback(learner, quiz, "perfect")).score).toBe(100);
});
