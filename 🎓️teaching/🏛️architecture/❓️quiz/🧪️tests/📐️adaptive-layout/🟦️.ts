/** 📐️ The adaptive layout in a real browser: on a phone, a tablet, a desktop and a wide desktop — the devices of the
 * shared vectors — a learner sees every part laid out for the room it has. Whatever fits beside what it belongs to stands
 * on the same line, left and right (the category select of an item, the key and the move buttons of a sorted item on
 * medium — the challenge a new device starts with —, the value select of a matched item); where the room ends they stand below. A wide task shows its pool or its value cards
 * beside what they are dropped on, a narrow card lists the tasks of a run and turns the tables of the results into
 * records, and nothing is ever wider than its card: no screen scrolls sideways. Every observed layout must also be the
 * one the tiers of the shared vectors give for the width its room really has, measured in the root text size.
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🧫️fixtures/📐️adaptive-layout/🔣️.json — the tiers and the devices
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🎨️.css — the adaptive layouts */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Locator, Page } from "@playwright/test";
import { boxOf, card, expect, primary, screen, test, way } from "../../🎭️e2e/🚶️learner/🟦️.ts";

interface Seen {
  readonly device: string;
  readonly width: number;
  readonly height: number;
  readonly touch: boolean;
  readonly steps: "list" | "chips";
  readonly pool: "stacked" | "line";
  readonly classification: "stacked" | "panes";
  readonly sorting: "stacked" | "line";
  readonly matching: "stacked" | "panes";
  readonly slot: "stacked" | "line";
  readonly results: "records" | "table";
}

interface Vectors {
  readonly tiers: readonly { readonly part: string; readonly room: string; readonly from: number }[];
  readonly devices: readonly Seen[];
}

const vectors = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../🧰️framework/🛍️products/❓️quiz/🧫️fixtures/📐️adaptive-layout/🔣️.json"), "utf8")) as Vectors;

/** 📏️ The least width in rem from which `part` takes its roomier layout (its first tier). */
function tier(part: string): number {
  const found = vectors.tiers.find((candidate) => candidate.part === part);
  if (found === undefined) throw new Error(`the vectors name no tier of ${part}`);
  return found.from;
}

/** 📐️ The inline size of `room` in rem of the root text size. */
async function rem(room: Locator): Promise<number> {
  return room.evaluate((element) => element.clientWidth / Number.parseFloat(getComputedStyle(document.documentElement).fontSize));
}

/** ↔️ Whether `right` stands on the line of `left`, right of it — else it stands below it. */
async function beside(left: Locator, right: Locator): Promise<boolean> {
  const [first, second] = [await boxOf(left), await boxOf(right)];
  const side = second.x >= first.x + first.width - 1 && second.y < first.y + first.height;
  const below = second.y >= first.y + first.height - 1;
  expect(side !== below, "a part stands either beside or below what it belongs to").toBe(true);
  return side;
}

/** 🚫️ Nothing of the screen in front is wider than what holds it: neither the page nor any box that could scroll. */
async function expectNoSidewaysScroll(page: Page, where: string): Promise<void> {
  const wider = await page.evaluate(() => {
    const boxes = [document.documentElement, ...document.querySelectorAll<HTMLElement>("#quiz-main, #quiz-main *")].filter((element) => element.closest("[inert]") === null);
    return boxes.filter((element) => element === document.documentElement || ["auto", "scroll"].includes(getComputedStyle(element).overflowX)).filter((element) => element.scrollWidth > element.clientWidth + 1).map((element) => `${element.tagName.toLowerCase()}.${element.className} +${element.scrollWidth - element.clientWidth}`);
  });
  expect(wider, `${where}: boxes that scroll sideways`).toEqual([]);
}

for (const seen of vectors.devices) {
  test(`a ${seen.device} ${seen.width} px wide lays every part out for its room`, async ({ browser }, info) => {
    const context = await browser.newContext({ viewport: { width: seen.width, height: seen.height }, isMobile: seen.touch, hasTouch: seen.touch, baseURL: info.project.use.baseURL, locale: "de-DE" });
    const page = await context.newPage();
    const problems: string[] = [];
    page.on("pageerror", (error) => problems.push(error.message));
    const task = screen(page, "task");
    const steps = screen(page, "run").locator("nav button");
    const where = (place: string): string => `${seen.device}, ${place}`;

    await page.goto("/");
    await primary(screen(page, "introduction")).click();
    await expectNoSidewaysScroll(page, where("the identity step"));
    await primary(screen(page, "identity")).click();
    await expect(page.locator("[data-layered-overview]")).toBeVisible();

    await primary(card(page, "physics")).click();
    await expect(screen(page, "run")).toBeVisible();
    const [firstStep, secondStep] = [await boxOf(steps.nth(0)), await boxOf(steps.nth(1))];
    expect(secondStep.y >= firstStep.y + firstStep.height - 1 ? "list" : "chips", where("the tasks of the run")).toBe(seen.steps);
    expect((await rem(screen(page, "run").locator('[data-slot="quiz-card-content"]'))) >= tier("quiz-steps") ? "chips" : "list", where("the tasks of the run, by their room")).toBe(seen.steps);

    const pool = task.locator('[data-quiz-drop="pool"]');
    const chip = pool.locator(".quiz-chip").first();
    expect((await beside(chip.locator(".quiz-row-label"), chip.locator("select"))) ? "line" : "stacked", where("an item of the pool")).toBe(seen.pool);
    expect((await rem(pool.locator(".quiz-rows"))) >= tier("quiz-chip") ? "line" : "stacked", where("an item of the pool, by its room")).toBe(seen.pool);
    expect((await beside(pool, task.locator(".quiz-bins"))) ? "panes" : "stacked", where("the pool and the bins")).toBe(seen.classification);
    expect((await rem(page.locator(".quiz-task"))) >= tier("quiz-classify") ? "panes" : "stacked", where("the pool and the bins, by their room")).toBe(seen.classification);
    await expectNoSidewaysScroll(page, where("a classification"));

    const items = await task.locator("[data-quiz-item]").evaluateAll((elements) => elements.map((element) => (element as HTMLElement).dataset.quizItem ?? ""));
    for (const item of items) await task.locator(`[data-quiz-item="${item}"] select`).selectOption({ index: 1 });
    const sorted = task.locator('[data-quiz-drop^="category:"] .quiz-chip').first();
    expect(await beside(sorted.locator(".quiz-row-label"), sorted.locator("select")), where("an item in a bin, by its room")).toBe((await rem(sorted.locator(".."))) >= tier("quiz-chip"));
    await expectNoSidewaysScroll(page, where("a classification with every item in a bin"));

    for (const index of [1, 2]) {
      await steps.nth(index).click();
      await expect(steps.nth(index)).toHaveAttribute("aria-current", "step");
      const row = task.locator("ol > .quiz-sort").first();
      expect((await beside(row.locator(".quiz-row-label"), row.locator(".quiz-sort-key"))) ? "line" : "stacked", where("a sorted item")).toBe(seen.sorting);
      expect((await rem(task.locator("ol.quiz-rows"))) >= tier("quiz-sort") ? "line" : "stacked", where("a sorted item, by its room")).toBe(seen.sorting);
      expect(await beside(row.locator(".quiz-sort-key"), row.locator(".quiz-sort-up")), where("the move buttons right of the key")).toBe(true);
      if (seen.touch) expect(Math.min(...Object.values(await boxOf(row.locator(".quiz-sort-up"))).slice(2)), where("a move button for a finger")).toBeGreaterThanOrEqual(40);
      await row.locator(".quiz-sort-down").click();
      await expectNoSidewaysScroll(page, where("a sorting with an item moved"));
      await expect(steps.nth(index).locator("[data-complete]")).toBeVisible();
    }

    await primary(screen(page, "run")).click();
    await primary(page.getByRole("alertdialog")).click();
    await expect(screen(page, "results")).toBeVisible();
    const result = screen(page, "task-result").first().locator("table.quiz-fold").first();
    expect((await result.evaluate((table) => getComputedStyle(table).display)) === "table" ? "table" : "records", where("the table of a classification's result")).toBe(seen.results);
    await expect(result.getByRole("rowheader").first()).toBeVisible();
    await expectNoSidewaysScroll(page, where("the results"));

    await way(page, "overview").click();
    await expect(page.locator("[data-layered-pane][data-opened]")).toHaveCount(0);
    for (const opened of ["board", "learner", "prefs", "physics"]) {
      await page.evaluate((hash) => (window.location.hash = hash), opened);
      await expect(page.locator(`[data-layered-pane="${opened}"][data-opened]`)).toBeVisible();
      await expectNoSidewaysScroll(page, where(`the page ${opened}`));
      await way(page, "overview").click();
      await expect(page.locator("[data-layered-pane][data-opened]")).toHaveCount(0);
    }

    await primary(card(page, "heating")).click();
    await expect(screen(page, "run")).toBeVisible();
    const slot = task.locator(".quiz-slot").first();
    expect((await beside(slot.locator(".quiz-row-label"), slot.locator("select"))) ? "line" : "stacked", where("a matched item")).toBe(seen.slot);
    expect((await rem(task.locator(".quiz-slots").first())) >= tier("quiz-slot") ? "line" : "stacked", where("a matched item, by its room")).toBe(seen.slot);
    expect(await beside(slot.locator("select"), slot.locator("button")), where("the remove button right of the select")).toBe(true);
    expect((await beside(task.locator(".quiz-cards").first(), task.locator(".quiz-slots").first())) ? "panes" : "stacked", where("the value cards and the rows")).toBe(seen.matching);
    expect((await rem(page.locator(".quiz-task"))) >= tier("quiz-match") ? "panes" : "stacked", where("the value cards and the rows, by their room")).toBe(seen.matching);
    await slot.locator("select").selectOption({ index: 1 });
    await expectNoSidewaysScroll(page, where("a matching"));

    await context.close();
    expect(problems, where("page errors")).toEqual([]);
  });
}
