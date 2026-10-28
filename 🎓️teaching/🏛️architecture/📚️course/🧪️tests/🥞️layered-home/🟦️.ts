/** 🥞️ Nested overview grids share live inert previews, glass, pan, zoom and accessible navigation. */
import { COURSE, QUIZZES, activeOverview, boxOf, card, enter, expect, handle, openGrid, pane, playQuiz, screen, test, way, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

const SCOPES = [{ id: undefined, pages: ["learner", "quizzes", "board", "badges", "prefs"] }, { id: "quizzes", pages: ["energy", "parametric", "grasshopper", "excel", "bim", "optimization"] }, { id: "energy", pages: ["physics", "ventilation", "heating", "cooling", "demand"] }];

/** 🔬️ The immediate layer, excluding the nested preview grids. */
function layer(device: Device, name: string) {
  return activeOverview(device.page).locator(`:scope > [data-layered-${name}]`);
}

/** 📍️ Distance between preview and active overview corners. */
async function displacement(device: Device, id: string): Promise<number> {
  const [frame, preview] = [await boxOf(activeOverview(device.page)), await boxOf(pane(device.page, id))];
  return Math.max(Math.abs(preview.x - frame.x), Math.abs(preview.y - frame.y));
}

test("home, topics and energy preserve the layered glass grid and immediate reading order", { tag: "@hierarchy" }, async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Nested Grid") }, true);
  for (const scope of SCOPES) {
    if (scope.id !== undefined) await openGrid(learner, scope.id);
    const root = activeOverview(learner.page);
    await expect(root).toHaveAttribute("data-mode", "strip");
    await expect(root).toHaveAttribute("data-pan", "pointer");
    expect(await layer(learner, "overlay").locator("[data-layered-card]").evaluateAll((cards) => cards.map((node) => node.getAttribute("data-layered-card")))).toEqual(scope.pages);
    const frame = await boxOf(root);
    for (const [index, id] of scope.pages.entries()) {
      const box = await boxOf(card(learner.page, id).locator("[data-overview-card]"));
      const [column, row] = [index % 3, Math.floor(index / 3)];
      expect(box.x).toBeGreaterThanOrEqual(frame.x + column * frame.width / 3 - 1);
      expect(box.x + box.width).toBeLessThanOrEqual(frame.x + (column + 1) * frame.width / 3 + 1);
      expect(box.y).toBeGreaterThanOrEqual(frame.y + row * frame.height / 2 - 1);
      expect(box.y + box.height).toBeLessThanOrEqual(frame.y + (row + 1) * frame.height / 2 + 1);
      await expect(pane(learner.page, id)).toHaveAttribute("inert", "");
      await expect(pane(learner.page, id)).not.toHaveAttribute("data-opened", "");
    }
  }
  for (const quiz of QUIZZES) for (const task of quiz.tasks) await expect(pane(learner.page, quiz.id)).toContainText(task.title.en);
  await expect(screen(learner.page, "run")).toHaveCount(0);
});

test("each nested grid pans under glass, reveals crisp screen-sized previews and zooms into a child", { tag: "@hierarchy" }, async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "anonymous" }, true);
  for (const scope of SCOPES) {
    await openGrid(learner, scope.id);
    const root = activeOverview(learner.page);
    const frame = await boxOf(root);
    const strip = layer(learner, "strip");
    const veil = layer(learner, "veil");
    const before = await strip.getAttribute("style");
    await learner.page.mouse.move(frame.x + frame.width - 2, frame.y + frame.height - 2, { steps: 4 });
    await expect(veil).toHaveAttribute("data-veil", "whole");
    await expect(veil).toBeVisible();
    expect(await veil.evaluate((glass) => getComputedStyle(glass).backdropFilter)).toContain("blur(");
    await expect.poll(() => strip.getAttribute("style")).not.toBe(before);
    const id = scope.id === "quizzes" ? "energy" : scope.pages[1]!;
    await card(learner.page, id).locator("[data-layered-zoom]").hover();
    await expect(card(learner.page, id)).toHaveAttribute("data-revealed", "");
    await expect.poll(() => displacement(learner, id)).toBeLessThan(0.5);
    const preview = await boxOf(pane(learner.page, id));
    expect([Math.round(preview.width), Math.round(preview.height)]).toEqual([Math.round(frame.width), Math.round(frame.height)]);
    await expect(veil).toHaveAttribute("data-veil", "clear");
    await expect(veil).toBeHidden();
    await expect(pane(learner.page, id)).toHaveAttribute("inert", "");
    await learner.page.emulateMedia({ reducedMotion: "reduce" });
    await expect(root).toHaveAttribute("data-pan", "pointer");
    await card(learner.page, id).getByRole("link").click();
    await expect(pane(learner.page, id)).toHaveAttribute("data-opened", "");
    await expect(pane(learner.page, id)).not.toHaveAttribute("inert", "");
    await expect.poll(() => new URL(learner.page.url()).hash).toBe(`#${id}`);
    await expect(screen(learner.page, "run")).toHaveCount(0);
  }
});

test("nested addresses preserve parents; Escape and up ascend one level and the trail supports back and forward", { tag: "@hierarchy" }, async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "anonymous" }, true);
  await learner.page.goto("/#physics");
  await learner.page.reload();
  for (const id of ["quizzes", "energy", "physics"]) await expect(pane(learner.page, id)).toHaveAttribute("data-opened", "");
  await expect(way(learner.page, "back")).toHaveAttribute("aria-disabled", "true");
  await expect(way(learner.page, "up")).toHaveAttribute("aria-keyshortcuts", "Escape");
  await learner.page.keyboard.press("Escape");
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("#energy");
  await expect(pane(learner.page, "physics")).toHaveAttribute("inert", "");
  await expect(card(learner.page, "physics").getByRole("link")).toBeFocused();
  await way(learner.page, "up").click();
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("#quizzes");
  await expect(card(learner.page, "energy").getByRole("link")).toBeFocused();
  await way(learner.page, "back").click();
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("#energy");
  await way(learner.page, "forward").click();
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("#quizzes");
  await learner.page.keyboard.press("Escape");
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("");
  await expect(card(learner.page, "quizzes").getByRole("link")).toBeFocused();
  await expect(way(learner.page, "up")).toHaveAttribute("aria-disabled", "true");
});

test("keyboard enters Energy and locked topic previews speak both languages without opening", { tag: "@hierarchy" }, async ({ device }) => {
  const learner = await device("de");
  await enter(learner, { kind: "anonymous" }, true);
  const quizzes = card(learner.page, "quizzes").getByRole("link");
  for (let count = 0; count < 60 && !await quizzes.evaluate((node) => node === document.activeElement); count++) await learner.page.keyboard.press("Tab");
  await expect(quizzes).toBeFocused();
  await quizzes.press("Enter");
  await expect(layer(learner, "overlay").locator("[data-layered-card]")).toHaveCount(SCOPES[1]!.pages.length);
  const energy = card(learner.page, "energy").getByRole("link");
  for (let count = 0; count < 60 && !await energy.evaluate((node) => node === document.activeElement); count++) await learner.page.keyboard.press("Tab");
  await expect(energy).toBeFocused();
  await energy.press("Enter");
  await expect(layer(learner, "overlay").locator("[data-layered-card]")).toHaveCount(SCOPES[2]!.pages.length);
  const physics = card(learner.page, "physics").getByRole("link");
  for (let count = 0; count < 60 && !await physics.evaluate((node) => node === document.activeElement); count++) await learner.page.keyboard.press("Tab");
  await expect(physics).toBeFocused();
  await physics.press("Enter");
  await expect(pane(learner.page, "physics")).toHaveAttribute("data-opened", "");
  await learner.page.keyboard.press("Escape");
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("#energy");
  await learner.page.keyboard.press("Escape");
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("#quizzes");
  for (const topic of COURSE.topics.filter((topic) => topic.id !== "energy")) {
    const locked = card(learner.page, topic.id);
    await expect(locked.getByRole("link")).toHaveCount(0);
    await expect(locked.locator(".course-access-preview[inert]")).toHaveCount(1);
    await expect(locked.getByRole("status")).toContainText("Noch nicht freigeschaltet");
    await locked.locator(".course-access-tint").click();
    const placeholder = pane(learner.page, topic.id);
    await expect(placeholder).not.toHaveAttribute("data-opened", "");
    await expect(placeholder).toContainText(topic.title.de);
    for (const id of topic.quizzes) await expect(placeholder).toContainText(QUIZZES.find(quiz => quiz.id === id)!.title.de);

    await learner.page.locator('header button[lang="en"]').click();
    await expect(locked.getByRole("status")).toContainText("Not available yet");
    await expect(placeholder).toContainText(topic.title.en);
    for (const id of topic.quizzes) await expect(placeholder).toContainText(QUIZZES.find(quiz => quiz.id === id)!.title.en);
    await learner.page.locator('header button[lang="de"]').click();
    await expect.poll(() => new URL(learner.page.url()).hash).toBe("#quizzes");
  }
});

test("locked topic addresses require the hidden author preview argument", { tag: "@hierarchy" }, async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "anonymous" }, true);
  await openGrid(learner, "quizzes");
  await learner.page.evaluate(() => { location.hash = "parametric"; });
  await expect(pane(learner.page, "parametric")).not.toHaveAttribute("data-opened", "");
  await expect(learner.page.locator('[data-course-grid="quizzes"][data-grid-active]')).toBeVisible();
  const url = new URL(learner.page.url());
  url.searchParams.set("preview", "1");
  await learner.page.goto(url.href);
  await expect(pane(learner.page, "parametric")).toHaveAttribute("data-opened", "");
  await expect(pane(learner.page, "parametric").locator("[data-course-locked]")).toHaveCount(0);
  url.searchParams.delete("preview");
  await learner.page.goto(url.href);
  await expect(pane(learner.page, "parametric")).not.toHaveAttribute("data-opened", "");
});

test("navbar keeps ordered ways and reaches the containing quiz from a run", { tag: "@hierarchy" }, async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "anonymous" });
  const ways = ["overview", "back", "forward", "up"] as const;
  for (const name of ways) await expect(way(learner.page, name)).toHaveAttribute("aria-disabled", "true");
  const boxes = await Promise.all(ways.map((name) => boxOf(way(learner.page, name))));
  expect(boxes.map((box) => box.x)).toEqual([...boxes.map((box) => box.x)].sort((a, b) => a - b));
  await expect(learner.page.locator("header [data-course-brand]")).toContainText(COURSE.title.en);
  await playQuiz(learner, "heating");
  await way(learner.page, "up").click();
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("#heating");
  await way(learner.page, "up").click();
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("#energy");
  await way(learner.page, "overview").click();
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("");
  await expect(activeOverview(learner.page)).toBeVisible();
});
