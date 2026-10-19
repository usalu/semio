/** 🧭️ The cast and the learner's choices in a real browser: the cast follows the learner, `still` and `off` and their return, reduced motion, forced colours and the footer switch of every screen.
 *
 * @see ../🟦️.ts — the pets in a real browser, as the second round made them
 */
import {QUIZZES, answerRun, arrive, enter, expect, identify, pane, playQuiz, readIntroduction, screen, submitRun, test, activeOverview, COURSE} from "../../../🎭️e2e/🚶️learner/🟦️.ts";
import { APPEAR_MS, CAST_LINE, castLine, CHOICES, choosePets, CURSOR, enterWithPets, expectCast, expectMotionless, expectMoving, expectWalk, FORCED_NOTE, HASTE, hasten, HOME, instrumented, layer, middleOf, pets, PETS_LABEL, PLAY_LABEL, reachable, RECAST_MS, REDUCED_NOTE, shownPets, STILL_FRAMES, SWITCH_LABEL, takenTargets, TEMPO, trailOf, watchStill } from "../🧰️pet-kit/🟦️.ts";

for (const locale of ["en", "de"] as const) test(`topic casts show only their own pets in ${locale}`, async ({ device }) => {
  const learner = await device(locale);
  await enterWithPets(learner);
  for (const topic of COURSE.topics) {
    await learner.page.evaluate((id) => (window.location.hash = id), topic.id);
    await expect(pane(learner.page, topic.id)).toHaveAttribute("data-opened", "");
    await expectCast(learner, `topic-${topic.id}`, true);
    expect(await takenTargets(learner)).toEqual([]);
    await learner.page.keyboard.press("Escape");
    await expect(pane(learner.page, topic.id)).not.toHaveAttribute("data-opened", "");
    await expectCast(learner, HOME, true);
  }
});

test("the cast follows the learner from quiz pages to each active question", async ({ device }) => {
  const learner = await device("de");
  await enterWithPets(learner);
  for (const quiz of QUIZZES) {
    await learner.page.evaluate((id) => (window.location.hash = id), quiz.id);
    await expect(pane(learner.page, quiz.id)).toHaveAttribute("data-opened", "");
    await expectCast(learner, quiz.id, true);
    expect(await takenTargets(learner)).toEqual([]);
    await learner.page.keyboard.press("Escape");
    await expect(pane(learner.page, quiz.id)).not.toHaveAttribute("data-opened", "");
    await expectCast(learner, HOME, true);
  }

  const played = QUIZZES[0]!;
  await playQuiz(learner, played.id);
  for (const task of played.tasks) {
    await learner.page.locator(".course-run .course-steps button").filter({ hasText: task.title[learner.locale] }).click();
    await expect(learner.page.locator('.course-run [data-card="task"]')).toContainText(task.title[learner.locale]);
    await expect(learner.page.locator(".pet-layer")).toHaveAttribute("data-pet-scene", `question-${played.id}-${task.id}`);
    await expectCast(learner, `question-${played.id}-${task.id}`, false);
  }
  await expect(async () => {
    if ((await shownPets(learner)).length > 0) expect(await learner.page.evaluate(watchStill, { frames: 2 * STILL_FRAMES, places: true }), "pets stay where they are while a run is on screen").toBe(true);
  }).toPass({ timeout: RECAST_MS });
  expect(await takenTargets(learner)).toEqual([]);
  await expect(screen(learner.page, "run")).toBeVisible();
});

test("the learner's choice stops the pets, removes them and brings them back, and is kept", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  const app = learner.page.locator(".course-app");
  const preferences = pane(learner.page, "prefs");
  const choices = preferences.getByRole("group", { name: PETS_LABEL[learner.locale], exact: true }).getByRole("button");
  const choose = (choice: (typeof CHOICES)[number]): Promise<void> => choosePets(learner, choice);
  await learner.page.evaluate(() => (window.location.hash = "prefs"));
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(choices).toHaveCount(CHOICES.length);
  await expect(choices.nth(CHOICES.indexOf("lively"))).toHaveAttribute("aria-pressed", "true");
  await expectMoving(learner);
  await expect(async () => expect(await preferences.locator("p", { hasText: CAST_LINE[learner.locale] }).innerText()).toBe(await castLine(learner))).toPass({ timeout: RECAST_MS });

  await choose("still");
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expectMotionless(learner);
  await expect(preferences.getByRole("group", { name: PLAY_LABEL[learner.locale], exact: true }), "still pets do not play").toHaveCount(0);

  await choose("off");
  await expect(layer(learner)).toHaveCount(0);
  await expect(learner.page.locator("svg.pet")).toHaveCount(0);
  await expect(preferences.locator("p", { hasText: CAST_LINE[learner.locale] })).toHaveCount(0);

  await learner.page.reload();
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(app).toHaveAttribute("data-pets", "off");
  await expect(choices.nth(CHOICES.indexOf("off"))).toHaveAttribute("aria-pressed", "true");
  await expect(layer(learner)).toHaveCount(0);

  await choose("lively");
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect(layer(learner)).toHaveAttribute("aria-hidden", "true");
  await expectMoving(learner);
});

test("a device that asks for reduced motion leaves the pets as the learner set them, lively until they choose otherwise, also after a reload", async ({ device }) => {
  const learner = await device("de");
  const page = learner.page;
  const app = page.locator(".course-app");
  const preferences = pane(page, "prefs");
  const note = preferences.getByText(REDUCED_NOTE[learner.locale], { exact: true });
  const choices = preferences.getByRole("group", { name: PETS_LABEL[learner.locale], exact: true }).getByRole("button");
  const playing = preferences.getByRole("group", { name: PLAY_LABEL[learner.locale], exact: true });
  const pressed = (choice: (typeof CHOICES)[number]): Promise<void> => expect(choices.nth(CHOICES.indexOf(choice))).toHaveAttribute("aria-pressed", "true");
  await page.addInitScript(hasten, { attribute: TEMPO, tempo: HASTE });
  await page.emulateMedia({ reducedMotion: "reduce" });
  await enterWithPets(learner);
  await expect(page.locator("html")).toHaveAttribute(TEMPO, "8");
  await expect(app).toHaveAttribute("data-pets", "lively");
  await expectCast(learner, HOME, true);
  await expectMoving(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(note).toHaveCount(0);
  await pressed("lively");
  await expect(playing).toBeVisible();

  await page.emulateMedia({ reducedMotion: "no-preference" });
  await expect(app).toHaveAttribute("data-pets", "lively");
  await pressed("lively");
  await page.emulateMedia({ reducedMotion: "reduce" });
  await expect(note).toHaveCount(0);
  await expect(app).toHaveAttribute("data-pets", "lively");
  await pressed("lively");

  await choosePets(learner, "still");
  await expect(playing).toHaveCount(0);
  await instrumented(page);
  await expectMotionless(learner);
  const still = await reachable(page);
  const middle = middleOf(still);
  await page.mouse.move(middle.x, middle.y, { steps: 5 });
  await expect(page.locator("html"), "the hand offers no still pet").not.toHaveAttribute(CURSOR, /.+/u);
  await page.mouse.click(middle.x, middle.y);
  await page.mouse.move(900, 600, { steps: 5 });
  await expectMotionless(learner);
  expect(await trailOf(page, still.id), `${still.id} answers no click`).toEqual([`idle/perch/${still.state}`]);
  expect(await takenTargets(learner)).toEqual([]);

  await page.reload();
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(app).toHaveAttribute("data-pets", "still");
  await pressed("still");
  await expect(note).toHaveCount(0);
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expectMotionless(learner);
  expect(await page.evaluate(() => window.matchMedia("(prefers-reduced-motion: reduce)").matches), "the device still asks for reduced motion").toBe(true);
});

test("a device that forces its own colours shows no pets, and the preferences say why", async ({ device }) => {
  const learner = await device("de");
  const note = pane(learner.page, "prefs").getByText(FORCED_NOTE[learner.locale], { exact: true });
  await learner.page.emulateMedia({ forcedColors: "active" });
  await enter(learner, { kind: "anonymous" });
  await expect(learner.page.locator(".course-app")).toHaveAttribute("data-pets", "lively");
  await learner.page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(learner.page, "prefs")).toHaveAttribute("data-opened", "");
  await expect(note).toBeVisible();
  await expect(layer(learner)).toHaveCount(1, { timeout: APPEAR_MS });
  await expect(pets(learner)).toHaveCount(0);
  await expect(pane(learner.page, "prefs").locator("p", { hasText: CAST_LINE[learner.locale] })).toHaveCount(0);

  await learner.page.emulateMedia({ forcedColors: "none" });
  await expect(note).toHaveCount(0);
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect(pane(learner.page, "prefs").locator("p", { hasText: CAST_LINE[learner.locale] })).toHaveCount(1, { timeout: RECAST_MS });
});

test("a switch on the footer of every screen hides the pets and brings them back as lively as they were, by keyboard", async ({ device }) => {
  const learner = await device("de");
  const page = learner.page;
  const app = page.locator(".course-app");
  const toggle = page.locator(".course-app > footer").getByRole("checkbox", { name: SWITCH_LABEL[learner.locale], exact: true });
  const press = async (shown: boolean, choice: (typeof CHOICES)[number]): Promise<void> => {
    await toggle.focus();
    await expect(toggle).toBeFocused();
    await page.keyboard.press("Space");
    await expect(toggle).toBeChecked({ checked: shown });
    await expect(app).toHaveAttribute("data-pets", choice);
    await expect(layer(learner)).toHaveCount(shown ? 1 : 0);
    expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
  };

  await arrive(learner);
  await expect(toggle).toBeChecked();
  await press(false, "off");
  await press(true, "lively");
  await readIntroduction(learner);
  await expect(toggle).toBeChecked();
  await identify(learner, { kind: "anonymous" });
  await expect(activeOverview(page)).toBeVisible();
  await expect(toggle).toBeChecked();
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });

  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await choosePets(learner, "lively");
  await press(false, "off");
  await expect(pane(page, "prefs").getByRole("group", { name: PETS_LABEL[learner.locale], exact: true }).getByRole("button").nth(CHOICES.indexOf("off"))).toHaveAttribute("aria-pressed", "true");
  await page.keyboard.press("Escape");
  await expect(pane(page, "prefs")).not.toHaveAttribute("data-opened", "");

  const quiz = QUIZZES[0]!;
  await playQuiz(learner, quiz.id);
  await expect(toggle).not.toBeChecked();
  await press(true, "lively");
  await press(false, "off");
  await answerRun(learner, quiz, "perfect");
  await submitRun(learner);
  await expect(screen(page, "results")).toBeVisible();
  await expect(toggle).not.toBeChecked();
  await press(true, "lively");

  await page.reload();
  await expect(app).toHaveAttribute("data-pets", "lively");
  await expect(toggle).toBeChecked();
});
