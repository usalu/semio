/** 🪪️ The first visit: the introduction, then the identity step with its three ways to appear — anonymous, pseudonym or
 * name — and never a password. A new pseudonym creates a learner; the same pseudonym on a fresh device (whatever its
 * capitals and spaces) recalls that learner with its progress; a name works like a pseudonym; anonymous creates a new
 * learner every time — on another device, and on the same one after switching identity, which asks first.
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive */
import { CATALOG, answerTask, arrive, card, enter, expect, expectSaved, goHome, handle, identify, learnerId, openTask, playQuiz, primary, quizOf, readIntroduction, screen, secondary, shownItems, shownName, shownTask, test } from "../../🎭️e2e/🚶️learner/🟦️.ts";

test("a first visit leads through the introduction to the identity step, which never asks for a password", async ({ device }) => {
  const visitor = await device("en");
  await arrive(visitor);
  await expect(screen(visitor.page, "introduction").getByRole("heading", { level: 1 })).toHaveText(CATALOG.introduction.title.en);
  await expect(screen(visitor.page, "identity")).toHaveCount(0);
  await readIntroduction(visitor);
  const form = screen(visitor.page, "identity");
  await expect(form.getByRole("radio")).toHaveCount(3);
  for (const kind of ["anonymous", "pseudonym", "name"]) await expect(form.locator(`input[type="radio"][value="${kind}"]`)).toBeVisible();
  await expect(form.locator('input[type="radio"][value="anonymous"]')).toBeChecked();
  await expect(form.locator('input[type="text"]')).toHaveCount(0);
  await form.locator('input[type="radio"][value="pseudonym"]').check();
  await expect(form.locator('input[type="text"]')).toBeVisible();
  await expect(visitor.page.locator('input[type="password"]')).toHaveCount(0);
  await primary(form).click();
  await expect(form.getByRole("alert")).toBeVisible();
  await expect(visitor.page.locator("[data-layered-overview]")).toHaveCount(0);
});

test("a new pseudonym creates a learner, and the same pseudonym on a fresh device recalls it with its progress", async ({ device }) => {
  const pseudonym = handle("Ada Lovelace");
  const quiz = quizOf("physics");
  const first = await device("en");
  await enter(first, { kind: "pseudonym", handle: pseudonym });
  expect(await shownName(first)).toBe(pseudonym);
  const learner = await learnerId(first);
  expect(learner).toMatch(/^[0-9a-f]{32}$/u);
  await playQuiz(first, quiz.id);
  await openTask(first, 0);
  const task = await shownTask(first);
  const items = await shownItems(first);
  await answerTask(first, quiz, "perfect");
  await expectSaved(first);
  await goHome(first);

  const second = await device("de");
  await arrive(second);
  await readIntroduction(second);
  await identify(second, { kind: "pseudonym", handle: `  ${pseudonym.toUpperCase().replace(" ", "   ")} ` });
  expect(await shownName(second)).toBe(pseudonym);
  expect(await learnerId(second)).toBe(learner);
  await playQuiz(second, quiz.id);
  const steps = screen(second.page, "run").locator("nav button");
  await expect(steps).toHaveCount(quiz.tasks.length);
  await expect(steps.locator("[data-complete]")).toHaveCount(1);
  await steps.filter({ has: second.page.locator("[data-complete]") }).click();
  expect(await shownTask(second)).toBe(task);
  expect([...(await shownItems(second))].sort()).toEqual([...items].sort());
});

test("a name identifies like a pseudonym", async ({ device }) => {
  const name = handle("Grace Hopper");
  const first = await device("de");
  await enter(first, { kind: "name", handle: name });
  expect(await shownName(first)).toBe(name);
  const second = await device("en");
  await enter(second, { kind: "name", handle: name });
  expect(await learnerId(second)).toBe(await learnerId(first));
});

test("anonymous creates a new learner every time", async ({ device }) => {
  const learners = [];
  for (const locale of ["en", "de"] as const) {
    const visitor = await device(locale);
    await enter(visitor, { kind: "anonymous" });
    await expect(card(visitor.page, "learner").getByRole("heading")).toContainText("#");
    learners.push({ id: await learnerId(visitor), name: (await shownName(visitor)).replace(/^.*#/u, "") });
  }
  expect(learners[0]!.id).not.toBe(learners[1]!.id);
  expect(learners[0]!.name).toMatch(/^[0-9a-f]{8}$/u);
  expect(learners[0]!.name).not.toBe(learners[1]!.name);

  const again = await device("en");
  await enter(again, { kind: "anonymous" });
  const before = await learnerId(again);
  await secondary(card(again.page, "learner")).click();
  const question = again.page.getByRole("alertdialog");
  await expect(question.locator("[data-autofocus]")).toBeFocused();
  await again.page.keyboard.press("Escape");
  await expect(question).toHaveCount(0);
  expect(await learnerId(again)).toBe(before);
  await secondary(card(again.page, "learner")).click();
  await primary(question).click();
  await identify(again, { kind: "anonymous" });
  expect(await learnerId(again)).toMatch(/^[0-9a-f]{32}$/u);
  expect(await learnerId(again)).not.toBe(before);
});
