/** 🗣️ English and German are both complete, and neither is the default: the language follows the browser, a browser
 * that names neither is asked in both, an explicit choice wins and is kept, and on every screen a learner visits — the
 * introduction, the identity step, the overview, every page behind it, a run with every kind of task, the confirmation
 * and the results — no label is left unresolved, the catalog's texts are those of the chosen language, and every action
 * and heading the client itself words reads differently in the other language.
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive */
import { CATALOG, LOCALES, QUIZZES, answerRun, arrive, card, expect, handle, openTask, pane, playQuiz, primary, quizOf, readIntroduction, screen, shownName, shownTask, test, unresolvedLabels, type Device, type Locale } from "../../🎭️e2e/🚶️learner/🟦️.ts";

const other = (locale: Locale): Locale => (locale === "en" ? "de" : "en");

/** 📄️ The document itself speaks `locale`: its language, and a title that is or ends with the catalog's title in it. */
async function expectDocumentIn(device: Device, locale: Locale): Promise<void> {
  await expect(device.page.locator(".quiz-app")).toHaveAttribute("lang", locale);
  await expect(device.page.locator("html")).toHaveAttribute("lang", locale);
  await expect.poll(async () => (await device.page.title()).split(" · ").at(-1)).toBe(CATALOG.title[locale]);
}

/** 🔀️ Chooses `locale` with the language switch of the navigation. */
async function choose(device: Device, locale: Locale): Promise<void> {
  const choice = device.page.locator(`header button[lang="${locale}"]`);
  await choice.click();
  await expect(choice).toHaveAttribute("aria-pressed", "true");
  await expectDocumentIn(device, locale);
}

/** 🔏️ Opens the notice of what the site stores from the footer, reads it, and closes it with Escape: it is a modal
 * dialog that takes the focus and gives it back to the button that opened it. Returns its texts in order. */
async function storedNotice(device: Device): Promise<readonly string[]> {
  const opener = device.page.locator("footer nav button");
  await opener.click();
  const notice = device.page.getByRole("dialog");
  await expect(notice).toHaveAttribute("aria-modal", "true");
  await expect(notice.locator("[data-autofocus]")).toBeFocused();
  expect(await unresolvedLabels(device), "the notice of what is stored").toEqual([]);
  const texts = await notice.locator("h1, h2, h3, p, button").evaluateAll((parts) => parts.map((part) => (part as HTMLElement).innerText.replace(/\s+/gu, " ").trim()));
  await device.page.keyboard.press("Escape");
  await expect(notice).toHaveCount(0);
  await expect(opener).toBeFocused();
  return texts.filter((text) => text !== "");
}

/** 🔤️ What the screen in front words: the names of its actions, its headings, labels and column heads, in order. What
 * no language changes (numbers, symbols, the names of the languages themselves) is left out. */
async function wording(device: Device): Promise<readonly string[]> {
  const texts = await device.page.locator("#quiz-main, header, footer").evaluateAll((roots) =>
    roots.flatMap((root) => [...root.querySelectorAll<HTMLElement>("button, h1, h2, h3, h4, legend, label, th[scope=col], summary")].filter((element) => element.closest("[inert]") === null && element.closest("[aria-hidden=true]") === null).map((element) => (element.getAttribute("aria-label") ?? element.innerText).replace(/\s+/gu, " ").trim())),
  );
  return texts.filter((text) => /\p{L}{2,}/u.test(text) && !["English", "Deutsch"].includes(text));
}

/** ⚖️ Checks the screen in front in `locale`, switches to the other language and back, and checks that everything the
 * client words changed with it while `data` (texts of the catalog or the learner in `locale`) is not held against it. */
async function expectBothLanguages(device: Device, locale: Locale, where: string, data: readonly string[]): Promise<void> {
  expect(await unresolvedLabels(device), `${where} in ${locale}`).toEqual([]);
  const mine = await wording(device);
  await choose(device, other(locale));
  expect(await unresolvedLabels(device), `${where} in ${other(locale)}`).toEqual([]);
  const theirs = await wording(device);
  await choose(device, locale);
  expect(theirs.length, `${where}: the same controls in both languages`).toBe(mine.length);
  const neutral = new Set(["Pseudonym", "Name", "Quiz", "Status", "Normal", "Minimum", "Maximum", ...data]);
  const untranslated = mine.filter((text, index) => text === theirs[index] && !/^[\d.,\s]+[\p{L}/()·²³]+$/u.test(text) && !neutral.has(text) && !data.some((entry) => entry !== "" && text.includes(entry)));
  expect(untranslated, `${where}: worded the same in ${locale} and ${other(locale)}`).toEqual([]);
}

for (const locale of LOCALES) {
  test(`a ${locale} browser reads every screen in ${locale}, and each screen switches completely`, async ({ device }) => {
    const learner = await device(locale);
    const name = handle("Polyglot");

    await arrive(learner);
    await expectDocumentIn(learner, locale);
    await expect(screen(learner.page, "introduction").getByRole("heading", { level: 1 })).toHaveText(CATALOG.introduction.title[locale]);
    await expect(screen(learner.page, "introduction")).not.toContainText(CATALOG.introduction.paragraphs[0]![other(locale)]);
    await expectBothLanguages(learner, locale, "introduction", []);

    await readIntroduction(learner);
    await screen(learner.page, "identity").locator('input[type="radio"][value="pseudonym"]').check();
    await expectBothLanguages(learner, locale, "identity", []);
    await screen(learner.page, "identity").locator('input[type="text"]').fill(name);
    await primary(screen(learner.page, "identity")).click();
    await expect(learner.page.locator("[data-layered-overview]")).toBeVisible();
    expect(await shownName(learner)).toBe(name);

    for (const quiz of QUIZZES) {
      await expect(card(learner.page, quiz.id).getByRole("heading")).toHaveText(quiz.title[locale]);
      await expect(card(learner.page, quiz.id)).toContainText(quiz.description[locale]);
    }
    await expectBothLanguages(learner, locale, "overview", [name]);

    await expect(learner.page.getByRole("navigation")).toHaveCount(2);
    const mine = await storedNotice(learner);
    await choose(learner, other(locale));
    const theirs = await storedNotice(learner);
    await choose(learner, locale);
    expect(mine.length, "the notice of what is stored says something").toBeGreaterThanOrEqual(5);
    expect(theirs.length).toBe(mine.length);
    expect(mine.filter((text, index) => text === theirs[index]), `the notice of what is stored: worded the same in ${locale} and ${other(locale)}`).toEqual([]);

    for (const id of ["learner", ...QUIZZES.map((quiz) => quiz.id), "intro", "board", "badges", "prefs"]) {
      await learner.page.evaluate((page) => (window.location.hash = page), id);
      await expect(pane(learner.page, id)).toHaveAttribute("data-opened", "");
      await expectBothLanguages(learner, locale, `page ${id}`, [name]);
      await learner.page.keyboard.press("Escape");
      await expect(pane(learner.page, id)).not.toHaveAttribute("data-opened", "");
    }

    for (const id of ["physics", "heating", "demand"]) {
      const quiz = quizOf(id);
      await playQuiz(learner, quiz.id);
      await expect(screen(learner.page, "run").getByRole("heading", { level: 1 })).toHaveText(quiz.title[locale]);
      for (let index = 0; index < quiz.tasks.length; index++) {
        await openTask(learner, index);
        await expectBothLanguages(learner, locale, `run of ${id}, task ${await shownTask(learner)}`, [name]);
      }
      await answerRun(learner, quiz, "perfect");
      await primary(screen(learner.page, "run")).click();
      await expect(learner.page.getByRole("alertdialog")).toBeVisible();
      expect(await unresolvedLabels(learner)).toEqual([]);
      await primary(learner.page.getByRole("alertdialog")).click();
      await expect(screen(learner.page, "results")).toBeVisible();
      await expect(screen(learner.page, "results").getByRole("heading", { level: 1 })).toContainText(quiz.title[locale]);
      await expectBothLanguages(learner, locale, `results of ${id}`, [name]);
      await primary(screen(learner.page, "results")).click();
      await expect(learner.page.locator("[data-layered-overview]")).toBeVisible();
    }
  });
}

test("the language follows the browser, is asked for when the browser names neither, and an explicit choice is kept", async ({ device }) => {
  for (const [language, expected] of [
    ["de-CH", "de"],
    ["en-US", "en"],
  ] as const) {
    const visitor = await device(expected, language);
    await arrive(visitor);
    await expect(visitor.page.locator(".quiz-app")).toHaveAttribute("lang", expected);
    await expect(screen(visitor.page, "introduction").getByRole("heading", { level: 1 })).toHaveText(CATALOG.introduction.title[expected]);
    await expect(visitor.page.locator(`header button[lang="${expected}"]`)).toHaveAttribute("aria-pressed", "true");
  }

  const stranger = await device("de", "fr-FR");
  await stranger.page.goto("/");
  const question = screen(stranger.page, "language");
  await expect(question).toBeVisible();
  await expect(screen(stranger.page, "introduction")).toHaveCount(0);
  await expect(stranger.page.locator(".quiz-app")).not.toHaveAttribute("lang", /.*/u);
  await expect(stranger.page.locator("html")).not.toHaveAttribute("lang", /.*/u);
  const asked = await question.getByRole("heading", { level: 1 }).locator("[lang]").evaluateAll((parts) => parts.map((part) => [part.getAttribute("lang"), (part as HTMLElement).innerText] as const));
  expect(asked.map(([locale]) => locale)).toEqual([...LOCALES]);
  await expect.poll(() => stranger.page.title()).toBe(asked.map(([, text]) => text).join(" · "));
  for (const locale of LOCALES) await expect(question.locator(`li[lang="${locale}"] button`)).toHaveCount(1);
  expect(await question.locator("li[lang]").evaluateAll((offers) => offers.map((offer) => offer.getAttribute("lang")))).toEqual([...LOCALES]);
  expect(await unresolvedLabels(stranger)).toEqual([]);
  await question.locator('li[lang="de"] button').click();
  await expect(screen(stranger.page, "introduction").getByRole("heading", { level: 1 })).toHaveText(CATALOG.introduction.title.de);
  await expect(stranger.page.locator(".quiz-app")).toHaveAttribute("lang", "de");
  await stranger.page.reload();
  await expect(screen(stranger.page, "introduction").getByRole("heading", { level: 1 })).toHaveText(CATALOG.introduction.title.de);

  const chooser = await device("en");
  await arrive(chooser);
  await choose(chooser, "de");
  await chooser.page.reload();
  await expect(chooser.page.locator(".quiz-app")).toHaveAttribute("lang", "de");
  await expect(screen(chooser.page, "introduction").getByRole("heading", { level: 1 })).toHaveText(CATALOG.introduction.title.de);
  await expect(chooser.page.locator('header button[lang="de"]')).toHaveAttribute("aria-pressed", "true");
});
