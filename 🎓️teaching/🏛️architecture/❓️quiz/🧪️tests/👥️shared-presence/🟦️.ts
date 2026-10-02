/** 👥️ Presence between two learners on two devices: each counts the other online and reads where the other is, sees the
 * other's cursor on the same card of the overview and on the same item inside a run of the same quiz (whatever place
 * that item has on each sheet). What the other answers stays out of a run until it is submitted: nothing of it shows
 * unless the learner asks — then the task's figure shows the other thinking along as a dot on the answer given, below
 * the task, so no item moves when the figure opens, the other joins, answers and leaves. Afterwards what everyone
 * answered shows on the results, and on the quiz's page to whoever submitted or asked. This spec runs alone on its
 * proctor, so the counts are exact.
 * @see ../../🎭️e2e/🎚️config/🟦️.ts — why this spec runs after the others
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learners these specs drive */
import { answerRun, boxOf, card, classify, count, enter, expect, goHome, handle, itemBoxes, openTaskById, pane, playQuiz, quizOf, screen, shownItems, submitRun, test, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

/** 🔢️ How many learners the device counts online. */
async function online(device: Device): Promise<number> {
  return count(await device.page.locator("[data-presence-status]").innerText());
}

/** 🖱️ The cursor of another learner the device draws on `anchor`. */
function peerCursor(device: Device, anchor: string): ReturnType<Device["page"]["locator"]> {
  return device.page.locator(`[data-presence-layer] [data-peer="cursor"][data-anchor="${anchor}"]`);
}

/** 📍️ Whether the tip of `cursor` lies inside the element the device draws for `anchor`. */
async function expectInside(device: Device, cursor: ReturnType<Device["page"]["locator"]>, anchor: string): Promise<void> {
  await expect(cursor).toBeVisible();
  await expect
    .poll(async () => {
      const [tip, target] = [await boxOf(cursor), await boxOf(device.page.locator(`[data-presence-anchor="${anchor}"]`).and(device.page.locator(":not([inert] *)")))];
      return tip.x >= target.x - 1 && tip.x <= target.x + target.width + 1 && tip.y >= target.y - 1 && tip.y <= target.y + target.height + 1;
    })
    .toBe(true);
}

test("two learners see each other online, each other's cursors and what the other thinks and answered", async ({ device }) => {
  const quiz = quizOf("physics");
  const task = quiz.tasks.find((candidate) => candidate.kind === "classification")!;
  const [annaName, benName] = [handle("Anna"), handle("Ben")];

  const anna = await device("en");
  await enter(anna, { kind: "pseudonym", handle: annaName });
  await expect.poll(() => online(anna)).toBe(1);

  const ben = await device("de");
  await enter(ben, { kind: "pseudonym", handle: benName });
  await expect.poll(() => online(anna)).toBe(2);
  await expect.poll(() => online(ben)).toBe(2);
  const who = card(anna.page, "learner").locator("[data-presence-list]");
  await who.locator("summary").click();
  await expect(who.locator("li")).toHaveCount(2);
  await expect(who.locator("li", { hasText: benName })).toBeVisible();
  await expect(who.locator("li", { hasText: annaName })).toBeVisible();

  await card(ben.page, "board").locator("[data-overview-card]").hover({ position: { x: 12, y: 12 } });
  const onBoard = peerCursor(anna, "home:board");
  await expectInside(anna, onBoard, "home:board");
  await expect(onBoard).toContainText(benName);

  await playQuiz(anna, quiz.id);
  await openTaskById(anna, task.id);
  const figure = screen(anna.page, "task").locator('[data-crowd-figure="answers"]');
  await expect(screen(anna.page, "task").locator("[data-crowd-gate]")).toHaveAttribute("data-crowd-gate", "locked");
  await expect(figure).toHaveCount(0);
  await screen(anna.page, "task").getByRole("button", { name: "Show it now" }).scrollIntoViewIfNeeded();
  const alone = await itemBoxes(anna);
  await playQuiz(ben, quiz.id);
  await openTaskById(ben, task.id);
  const drawnForBen = await shownItems(ben);
  const item = (await shownItems(anna)).find((candidate) => drawnForBen.includes(candidate))!;
  expect(item, `both sheets draw ${task.draw} of ${task.items.length} items, so they share one`).toBeDefined();

  await screen(ben.page, "task").locator(`[data-presence-anchor="item:${item}"]`).hover();
  const onItem = peerCursor(anna, `item:${item}`);
  await expectInside(anna, onItem, `item:${item}`);
  await expect(onItem).toContainText(benName);

  const category = task.categories!.find((candidate) => candidate.id === task.items.find((entry) => entry.id === item)!.category)!;
  await classify(ben, item, category.id);
  await expect(onItem).toBeVisible();
  await expect(anna.page.locator("[data-crowd-figure], [data-crowd-dot]"), "what the other answers stays out of the run by default").toHaveCount(0);
  expect(await itemBoxes(anna), "no item moved when the other learner joined the task and answered").toEqual(alone);

  await screen(anna.page, "task").getByRole("button", { name: "Show it now" }).click();
  await expect(figure).toHaveAttribute("data-thinkers", "1");
  const thought = figure.locator(`[data-crowd-item="${item}"] > [data-key="${category.id}"]`);
  await expect(thought).toHaveAttribute("data-live", "1");
  await expect(thought.locator("[data-crowd-dot]")).toBeVisible();
  await expect(screen(anna.page, "task").getByRole("button", { name: "Hide it again" })).toBeVisible();
  await expect(ben.page.locator("[data-crowd-figure], [data-crowd-dot]"), "the other learner did not ask").toHaveCount(0);
  expect(await itemBoxes(anna), "no item moved when the figure opened below the task").toEqual(alone);

  await answerRun(ben, quiz, "perfect");
  await submitRun(ben);
  await expect(figure.locator("[data-crowd-dot]")).toHaveCount(0);
  expect(await itemBoxes(anna), "no item moved when the other learner left the task").toEqual(alone);
  await goHome(anna);
  await anna.page.evaluate((page) => (window.location.hash = page), quiz.id);
  const page = pane(anna.page, quiz.id);
  await expect(page).toHaveAttribute("data-opened", "");
  const crowd = page.locator('[data-card="quiz-crowd"]');
  const scored = crowd.locator('[data-crowd-figure="scores"]');
  await expect(crowd.locator("[data-crowd-gate]"), "the ask of the open run holds on the quiz's page too").toHaveAttribute("data-crowd-gate", "asked");
  await expect.poll(async () => Number(await scored.getAttribute("data-runs"))).toBeGreaterThanOrEqual(1);
  const runs = Number(await scored.getAttribute("data-runs"));
  const answers = crowd.locator("section").filter({ has: anna.page.getByRole("heading", { name: task.title.en, exact: true }) });
  await expect.poll(async () => Number(await answers.locator(`[data-crowd-item="${item}"] > [data-key="${category.id}"]`).getAttribute("data-count"))).toBeGreaterThanOrEqual(1);
  await crowd.getByRole("button", { name: "Hide it again" }).click();
  await expect(crowd.locator("[data-crowd-gate]")).toHaveAttribute("data-crowd-gate", "locked");
  await expect(crowd.locator("[data-crowd-figure]")).toHaveCount(0);
  await anna.page.keyboard.press("Escape");

  const everyone = screen(ben.page, "results").locator('[data-crowd-figure="scores"]');
  await expect.poll(async () => Number(await everyone.getAttribute("data-runs"))).toBe(runs);
  await expect(everyone.locator("[data-own]")).toHaveCount(1);
  const given = ben.page.locator(`[data-presence-anchor="result:${task.id}"] [data-crowd-figure="answers"] [data-crowd-item="${item}"] > [data-key="${category.id}"]`);
  await expect(given).toHaveAttribute("data-own", "");
  await expect(given).toHaveAttribute("data-correct", "");
  await expect.poll(async () => Number(await given.getAttribute("data-count"))).toBeGreaterThanOrEqual(1);
  await expect(ben.page.locator("[data-crowd-gate]"), "a submitted run shows the others without an ask").toHaveCount(0);

  await ben.context.close();
  await expect.poll(() => online(anna)).toBe(1);
  await expect(anna.page.locator(`[data-peer][data-tag]`)).toHaveCount(0);
});
