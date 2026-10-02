/** 📴️ The site without its proctor. A learner who has never been here arrives while the proctor is away: the catalog
 * shows at once, a pseudonym is taken, a quiz is played to its score, its feedback and its badges, the overview shows
 * the result and the device's own standing, and a reload keeps all of it — everything is decided and saved on the
 * device. Once the proctor is back it hears of everything: the connection calms, the leaderboard is the proctor's
 * again, and a fresh device that enters the pseudonym finds the same learner with the same result. A pseudonym the
 * proctor already knew, taken on a device while the proctor was away, turns out to be its holder: the device continues
 * as that learner and its run is added to theirs. The specs stop and start the proctor of their stack through the
 * gate's control endpoint, so they run alone; whatever a spec does, the proctor runs again once it ends.
 * @see ../../🎭️e2e/🟦️.ts — the control endpoint
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../📚️catalog/🟦️.ts — the material the device decides with */
import { answerRun, badgesFor, card, connection, enter, expect, expectFeedback, expectSaved, goHome, handle, learnerId, percent, playQuiz, proctor, quizOf, screen, shownName, submitRun, test, type Device, type Locale, type SourceQuiz } from "../../🎭️e2e/🚶️learner/🟦️.ts";

/** 📶️ What the connection indicator says while everything stays on the device. */
const ON_DEVICE: Readonly<Record<Locale, RegExp>> = { en: /Quiz server not reachable – (?:everything is )?saved on this device/u, de: /Quiz-Server nicht erreichbar – (?:alles wird )?auf diesem Gerät gespeichert/u };

test.afterEach(() => proctor("start"));

/** 💯️ The best score of `quiz` in percent as its card on the overview shows it. */
async function shownBest(device: Device, quiz: SourceQuiz): Promise<number> {
  return percent(await card(device.page, quiz.id).locator("li", { hasText: "%" }).innerText());
}

/** 🏅️ The labels of the badges the results on screen say the run earned. */
async function earnedLabels(device: Device): Promise<readonly string[]> {
  return screen(device.page, "results").locator("[data-earned] h3").allInnerTexts();
}

test("a first visit while the proctor is away is decided on the device, survives a reload and reaches the proctor once it is back", async ({ device }) => {
  test.setTimeout(300_000);
  const quiz = quizOf("cooling");
  const name = handle("Away Player");
  const badges = badgesFor([quiz.id]);
  const learner = await device("en");
  const failures = await learner.expectingFailures(async () => {
    await proctor("stop");
    await enter(learner, { kind: "pseudonym", handle: name });
    expect(await shownName(learner)).toBe(name);
    await expect(connection(learner)).toHaveAttribute("data-tone", "alert", { timeout: 30_000 });
    await expect(connection(learner)).toContainText(ON_DEVICE.en);
    await expect(card(learner.page, "board").locator("[data-board-local]")).toBeVisible({ timeout: 30_000 });

    await playQuiz(learner, quiz.id);
    await expect(screen(learner.page, "run").getByRole("heading", { level: 1 })).toHaveText(quiz.title.en);
    await answerRun(learner, quiz, "perfect");
    await submitRun(learner);
    expect((await expectFeedback(learner, quiz, "perfect")).score).toBe(100);
    expect(await earnedLabels(learner)).toEqual(badges.map((badge) => badge.label.en));
    await goHome(learner);
    expect(await shownBest(learner, quiz)).toBe(100);
    await expect(card(learner.page, "badges").locator("li[data-earned]")).toHaveCount(badges.length);
    await expect(card(learner.page, "board")).toContainText(name);

    await learner.page.reload();
    await expect(learner.page.locator("[data-layered-overview]")).toBeVisible();
    expect(await shownName(learner)).toBe(name);
    expect(await shownBest(learner, quiz)).toBe(100);
    await expect(card(learner.page, "badges").locator("li[data-earned]")).toHaveCount(badges.length);
    await expect(connection(learner)).toContainText(ON_DEVICE.en, { timeout: 30_000 });

    await proctor("start");
    await expect(connection(learner)).toHaveAttribute("data-tone", "calm", { timeout: 90_000 });
    await expect(learner.page.locator("[data-presence-status]")).toBeVisible({ timeout: 90_000 });
  });
  expect(failures, "the proctor was really away: requests failed").toBeGreaterThan(0);
  await expect(card(learner.page, "board").locator("[data-board-local]")).toHaveCount(0, { timeout: 60_000 });
  await expect(card(learner.page, "board")).toContainText(name);
  expect(await shownBest(learner, quiz)).toBe(100);
  await expect(learner.page.getByRole("alert")).toHaveCount(0);

  const elsewhere = await device("en");
  await enter(elsewhere, { kind: "pseudonym", handle: name });
  expect(await learnerId(elsewhere)).toBe(await learnerId(learner));
  expect(await shownBest(elsewhere, quiz)).toBe(100);
  await expect(card(elsewhere.page, "badges").locator("li[data-earned]")).toHaveCount(badges.length);
});

test("a pseudonym the proctor already knows, taken while it is away, continues as its holder once it is back", async ({ device }) => {
  test.setTimeout(300_000);
  const [before, during] = [quizOf("demand"), quizOf("physics")];
  const name = handle("Returning Player");
  const home = await device("de");
  await enter(home, { kind: "pseudonym", handle: name });
  await playQuiz(home, before.id);
  await answerRun(home, before, "perfect");
  await submitRun(home);
  await goHome(home);
  await expectSaved(home);
  const holder = await learnerId(home);

  const away = await device("de");
  let seen = 0;
  const watched = await home.expectingFailures(async () => {
    seen = await away.expectingFailures(async () => {
      await proctor("stop");
      await enter(away, { kind: "pseudonym", handle: name });
      expect(await learnerId(away)).not.toBe(holder);
      await playQuiz(away, during.id);
      await answerRun(away, during, "perfect");
      await submitRun(away);
      expect((await expectFeedback(away, during, "perfect")).score).toBe(100);
      await goHome(away);
      await expect(connection(away)).toContainText(ON_DEVICE.de, { timeout: 30_000 });
      await proctor("start");
      await expect(connection(away)).toHaveAttribute("data-tone", "calm", { timeout: 90_000 });
      await expect(connection(home)).toHaveAttribute("data-tone", "calm", { timeout: 90_000 });
      await expect(away.page.locator("[data-presence-status]")).toBeVisible({ timeout: 90_000 });
      await expect(home.page.locator("[data-presence-status]")).toBeVisible({ timeout: 90_000 });
    });
  });
  expect(seen + watched, "the proctor was really away: requests failed").toBeGreaterThan(0);
  expect(await learnerId(away)).toBe(holder);
  await expect(away.page.getByRole("alert")).toContainText(name);
  expect(await shownName(away)).toBe(name);
  await expect.poll(() => shownBest(away, before), { timeout: 60_000 }).toBe(100);
  expect(await shownBest(away, during)).toBe(100);

  await home.page.reload();
  await expect(home.page.locator("[data-layered-overview]")).toBeVisible();
  await expect.poll(() => shownBest(home, during), { timeout: 60_000 }).toBe(100);
  expect(await shownBest(home, before)).toBe(100);
});
