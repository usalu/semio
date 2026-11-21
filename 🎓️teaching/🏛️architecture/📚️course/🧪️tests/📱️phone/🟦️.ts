/** 📱️ On a phone (375 × 812) home is the same grid as on a desktop, one page the size of the screen at a time — the
 * learner swipes along the rows and along the columns from page to page with a real touch, or taps the hint that names
 * the page behind an edge; every card stands on its own page, as tall as what it holds and centred — and a quiz is
 * playable from start to results without anything spilling over the edge of the screen or scrolling the screen sideways. On hard every guess field lies within the screen, is tall
 * enough to hit and takes its guess, and the categories keep their descriptions to themselves.
 * @see ../../🎭️e2e/🎚️config/🟦️.ts — the `phone` project that holds this phone
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive */
import { PAR, activeOverview, answerRun, answerTask, authorPreview, boxOf, card, enter, expect, expectFeedback, goHome, handle, openTask, openTaskById, playQuiz, quizOf, restingPage, screen, shownBest, shownTask, shownText, submitRun, swipeTo, taskOf, test, touchAssignClassification, touchSwipe, unresolvedLabels, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

import phone from "./🔣️.json" with { type: "json" };

/** ↔️ How many pixels the document is wider than the screen. */
async function overflow(device: Device): Promise<number> {
  return device.page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
}

/** 📜️ How many pixels the screen in front scrolls sideways at most: every box of it that scrolls, but for the frames in
 * which a table scrolls on its own. */
async function sideways(device: Device): Promise<number> {
  return device.page.evaluate(() =>
    Math.max(
      0,
      ...[...document.querySelectorAll<HTMLElement>("#course-main, #course-main *")]
        .filter((box) => box.closest("[inert]") === null && /auto|scroll/u.test(getComputedStyle(box).overflowX) && box.firstElementChild?.tagName !== "TABLE")
        .map((box) => box.scrollWidth - box.clientWidth),
    ),
  );
}

test("home is the same grid on a phone, swiped page by page in both directions, and a quiz is playable", { tag: "@hierarchy" }, async ({ device }) => {
  const learner = await device("de");
  expect(learner.page.viewportSize()).toEqual({ width: 375, height: 812 });
  await enter(learner, { kind: "anonymous" });
  const root = activeOverview(learner.page);
  await expect(root).toHaveAttribute("data-mode", "swipe");
  const [first, second, third, fourth] = ["quizzes", "clips", "board", "prefs"];
  const ring = [["learner", 0, 0], ["quizzes", 1, 0], ["assignments", 2, 0], ["clips", 0, 1], ["board", 1, 1], ["badges", 2, 1], ["prefs", 0, 2]];
  expect(await root.locator(":scope > [data-layered-overlay] > [data-layered-cell]").evaluateAll((cells) => cells.map((cell) => [(cell as HTMLElement).dataset.layeredCell, Number((cell as HTMLElement).style.gridColumn) - 1, Number((cell as HTMLElement).style.gridRow) - 1])), "the seven cards of the root grid, in reading order").toEqual(ring);
  const view = await boxOf(root);
  const resting = await boxOf(root.locator('[data-layered-cell="learner"]'));
  expect([Math.round(resting.x), Math.round(resting.y), Math.round(resting.width), Math.round(resting.height)], "the learner's page fills the overview").toEqual([Math.round(view.x), Math.round(view.y), 375, Math.round(view.height)]);
  expect(await restingPage(learner.page)).toBe("learner");
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  expect(await unresolvedLabels(learner)).toEqual([]);
  const cards = await root.locator(":scope > [data-layered-overlay] > [data-layered-cell]").evaluateAll((cells) =>
    cells.map((cell) => {
      const held = cell.querySelector<HTMLElement>("[data-layered-card] [data-card]")!;
      return { centred: getComputedStyle(cell).alignItems, cut: held.scrollHeight - held.clientHeight };
    }),
  );
  expect(cards, "every card fills its page (the cell stretches it, as the layered overview does) and is cut nowhere").toEqual(cards.map(() => ({ centred: "stretch", cut: 0 })));

  await touchSwipe(learner.page, 240, 0);
  await expect.poll(() => restingPage(learner.page), { message: "before the first column lies the last one: the ring wraps" }).toBe("assignments");
  await touchSwipe(learner.page, -240, 0);
  await expect.poll(() => restingPage(learner.page), { message: "and back across the edge" }).toBe("learner");
  await touchSwipe(learner.page, 0, 325);
  await expect.poll(() => restingPage(learner.page), { message: "above the top row lies the bottom row" }).toBe("prefs");
  await touchSwipe(learner.page, 0, -325);
  await expect.poll(() => restingPage(learner.page), { message: "and below the bottom row the top row" }).toBe("learner");
  await touchSwipe(learner.page, -60, 0, 600);
  await expect.poll(() => restingPage(learner.page), { message: "a short slow swipe stays" }).toBe("learner");
  const hints = () => root.locator(":scope > [data-layered-neighbour]").evaluateAll((all) => all.map((hint) => [(hint as HTMLElement).dataset.layeredNeighbour, (hint as HTMLElement).dataset.pane]));
  expect(await hints(), "every edge of the learner's page names the page behind it, across the edges the far side's").toEqual([["up", "prefs"], ["left", "assignments"], ["right", first], ["down", second]]);
  await expect(root.locator(':scope > [data-layered-neighbour="right"]')).toHaveAttribute("aria-label", "Nach rechts zu Quizze");
  await expect(root.locator(':scope > [data-layered-neighbour="right"]')).toContainText("Quizze");
  for (const hint of await root.locator(":scope > [data-layered-neighbour]").all()) await expect(hint).toBeInViewport({ ratio: 1 });
  await root.locator(':scope > [data-layered-neighbour="down"]').tap();
  await expect.poll(() => restingPage(learner.page), { message: "a tap on a hint goes where it points" }).toBe(second);
  await expect.poll(hints).toEqual([["up", "learner"], ["left", "badges"], ["right", third], ["down", fourth]]);
  await swipeTo(learner, "learner");
  await swipeTo(learner, "quizzes");
  await card(learner.page, "quizzes").getByRole("link").tap();
  await expect(learner.page.locator('[data-course-grid="quizzes"][data-grid-active]')).toBeVisible();
  await swipeTo(learner, "energy");
  await card(learner.page, "energy").getByRole("link").tap();
  await expect(learner.page.locator('[data-course-grid="energy"][data-grid-active]')).toBeVisible();
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  const quiz = quizOf("cooling");
  await swipeTo(learner, quiz.id);
  await expect(card(learner.page, quiz.id).getByRole("heading")).toHaveText(quiz.title.de);
  await swipeTo(learner, "demand");
  expect(await restingPage(learner.page), "the sparse final row rests on demand").toBe("demand");
  await swipeTo(learner, quiz.id);
  expect(await restingPage(learner.page), "the cooling page is reachable from a row with only one occupied cell").toBe(quiz.id);
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
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
  await expect(root).toHaveAttribute("data-mode", "swipe");
  await swipeTo(learner, quiz.id);
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
    await expect(screen(learner.page, "task").locator('[aria-busy="true"]')).toHaveCount(0);
    const fields = screen(learner.page, "task").locator("[data-course-item] input");
    for (const field of await fields.all()) {
      const box = await boxOf(field);
      expect(box.x, "a guess field starts on the screen").toBeGreaterThanOrEqual(0);
      expect(box.x + box.width, "a guess field ends on the screen").toBeLessThanOrEqual(375);
      expect(box.height, "a guess field is at least 44 px tall").toBeGreaterThanOrEqual(44);
      await expect(field).toHaveCSS("font-size", "16px");
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

test("a narrow phone can jump to course pages without loading hidden media or quiz crowds", async ({ device }) => {
  const learner = await device("en");
  const { page } = learner;
  const crowdRequests: string[] = [];
  const pdfRequests: string[] = [];
  const pdfPath = new RegExp(phone.pdfPattern, "u");
  page.on("request", (request) => {
    const url = new URL(request.url());
    if (url.pathname === "/queries" && request.method() === "POST") {
      const envelope = request.postDataJSON() as { readonly kind?: string; readonly arguments?: readonly number[] } | null;
      if (envelope?.kind === "course.crowd") {
        const query = JSON.parse(new TextDecoder().decode(new Uint8Array(envelope.arguments ?? []))) as { readonly quiz: string };
        crowdRequests.push(query.quiz);
      }
    }
    if (pdfPath.test(url.pathname)) pdfRequests.push(request.url());
  });
  await page.setViewportSize({ width: phone.width, height: phone.height });
  await enter(learner, { kind: "anonymous" });
  await authorPreview(learner);
  const picker = page.getByRole("combobox", { name: "Open a course page" });
  const jumpTo = async (destination: string) => {
    await picker.selectOption(destination);
    await picker.locator("..").getByRole("button").tap();
  };
  await expect(picker).toBeVisible();
  await expect(page.locator("[data-course-grid]")).toHaveCount(phone.rootGrids);
  expect(await overflow(learner)).toBeLessThanOrEqual(0);
  const navbar = await boxOf(page.locator('[data-slot="navbar"]'));
  expect(navbar.height).toBeGreaterThanOrEqual(phone.minimumTouchTarget);
  expect((await boxOf(picker)).y).toBeGreaterThanOrEqual(navbar.y + navbar.height);
  for (const control of await page.locator("[data-course-nav]").all()) {
    const box = await boxOf(control);
    expect(box.width).toBeGreaterThanOrEqual(phone.minimumTouchTarget);
    expect(box.height).toBeGreaterThanOrEqual(phone.minimumTouchTarget);
    expect(box.y).toBeGreaterThanOrEqual(navbar.y);
    expect(box.y + box.height).toBeLessThanOrEqual(navbar.y + navbar.height);
    await expect(control).toBeInViewport({ ratio: 1 });
  }

  await jumpTo("clips");
  const clips = activeOverview(page);
  const poster = card(page, phone.visibleClip).locator("img");
  await expect(poster).toBeVisible();
  await expect.poll(() => poster.evaluate((image: HTMLImageElement) => image.naturalWidth)).toBeGreaterThan(0);
  await expect(clips.locator("[data-layered-overlay] [data-clip-poster] img")).toHaveCount(phone.maximumClipCardPosters);
  await jumpTo(phone.directClip);
  await expect(page.locator(`[data-page="clip:${phone.directClip}"] img`)).toBeVisible();
  await page.locator('[data-course-nav="overview"]').tap();
  await jumpTo("assignments");
  await jumpTo(phone.assignment);
  const assignment = page.locator(`[data-page="assignment:${phone.assignment}"]`);
  await expect(assignment).toBeVisible();
  await expect(assignment.locator("iframe")).toHaveCount(0);
  await expect(assignment.getByRole("link", { name: "Open in new tab" })).toHaveAttribute("href", pdfPath);
  await expect(assignment.getByRole("link", { name: "Download" })).toHaveAttribute("href", pdfPath);
  expect(pdfRequests).toEqual([]);
  expect(crowdRequests).toHaveLength(phone.idleCrowdRequests);
  expect(await overflow(learner)).toBeLessThanOrEqual(0);

  await page.setViewportSize(phone.landscape);
  await expect(activeOverview(page)).toHaveAttribute("data-mode", "swipe");
  await expect(picker).toBeVisible();
  expect(await overflow(learner)).toBeLessThanOrEqual(0);

  await page.locator('[data-course-nav="overview"]').tap();
  await jumpTo("quizzes");
  await jumpTo("energy");
  await jumpTo("physics");
  await expect(page.locator('[data-page="quiz:physics"]')).toBeVisible();
  await expect.poll(() => [...new Set(crowdRequests)]).toEqual(["physics"]);
});

test("holding a dragged quiz item at the phone edge reaches and assigns a distant category", async ({ device }) => {
  const learner = await device("en");
  const { page } = learner;
  await page.setViewportSize({ width: 375, height: 560 });
  await enter(learner, { kind: "anonymous" });
  const quiz = quizOf("physics");
  await playQuiz(learner, quiz.id);
  await openTaskById(learner, quiz.tasks.find((task) => task.kind === "classification")!.id);
  await touchAssignClassification(page);
});
