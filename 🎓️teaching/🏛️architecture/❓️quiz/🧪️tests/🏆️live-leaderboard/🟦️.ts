/** 🏆️ The leaderboards: every learner with a submitted run has a row, the rows are ordered by total, the own row is
 * marked, and a device that only watches sees new rows and new totals arrive without ever reloading; the same learners
 * are on today's, this week's and this month's leaderboard, on the one of the quiz they played and on no other quiz's,
 * and a click on a column heading puts the table in that column's order. Other specs play on the same proctor at the
 * same time, so everything here is asserted about the rows of this spec's own learners and about the order of whatever
 * the board holds.
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive */
import { PAR, answerRun, card, enter, expect, goHome, handle, pane, playQuiz, primary, quizOf, shownResults, submitRun, test, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

/** 📋️ One row of the leaderboard: rank, the cell naming the learner, total, and whether it is the own row. */
interface BoardRow {
  readonly rank: number;
  readonly name: string;
  readonly total: number;
  readonly own: boolean;
}

/** 📋️ The opened leaderboard page at one instant: its rows in the order shown, and how many learners it says it ranks. */
async function standings(device: Device): Promise<{ readonly rows: readonly BoardRow[]; readonly learners: number }> {
  return pane(device.page, "board").evaluate((page) => ({
    rows: [...page.querySelectorAll("tbody:not([data-board-own]) tr")].map((row) => {
      const cells = [...row.children] as HTMLElement[];
      return { rank: Number(cells[0]!.innerText), name: cells[1]!.innerText.trim(), total: Number(cells[2]!.innerText.replace(",", ".")), own: row.getAttribute("aria-current") === "true" };
    }),
    learners: Number(/\d+/u.exec(page.querySelector<HTMLElement>("[data-board-count]")?.innerText ?? "")?.[0] ?? Number.NaN),
  }));
}

async function rows(device: Device): Promise<readonly BoardRow[]> {
  return (await standings(device)).rows;
}

async function play(device: Device, how: "perfect" | "flawed"): Promise<number> {
  const quiz = quizOf("heating");
  await playQuiz(device, quiz.id);
  await answerRun(device, quiz, how);
  await submitRun(device);
  const { points } = await shownResults(device);
  await goHome(device);
  return points;
}

test("the leaderboard orders learners by total and updates on a watching device without a reload", async ({ device }) => {
  const [watcherName, strongName, partialName] = [handle("Watcher"), handle("Strong"), handle("Partial")];
  const watcher = await device("en");
  await enter(watcher, { kind: "pseudonym", handle: watcherName });
  await primary(card(watcher.page, "board")).click();
  const board = pane(watcher.page, "board");
  await expect(board).toHaveAttribute("data-opened", "");
  await watcher.page.evaluate(() => ((window as unknown as { neverReloaded: boolean }).neverReloaded = true));
  const row = (name: string): ReturnType<typeof board.locator> => board.locator("tbody tr", { hasText: name });
  await expect(row(strongName)).toHaveCount(0);
  await expect(row(watcherName)).toHaveCount(0);

  const strong = await device("de");
  await enter(strong, { kind: "pseudonym", handle: strongName });
  expect(await play(strong, "perfect")).toBe(PAR.medium);
  await expect(row(strongName)).toBeVisible({ timeout: 40_000 });

  const partial = await device("en");
  await enter(partial, { kind: "pseudonym", handle: partialName });
  const partialPoints = await play(partial, "flawed");
  expect(partialPoints).toBeGreaterThan(0);
  expect(partialPoints).toBeLessThan(PAR.medium);
  await expect(row(partialName)).toBeVisible({ timeout: 40_000 });

  const { rows: listed, learners } = await standings(watcher);
  expect(listed.length, "the page shows the top hundred of the learners it counts").toBe(Math.min(learners, 100));
  expect(listed.map((entry) => entry.rank)).toEqual([...listed.map((entry) => entry.rank)].sort((left, right) => left - right));
  for (let index = 1; index < listed.length; index++) expect(listed[index]!.total, `row ${index + 1} never outranks the row above`).toBeLessThanOrEqual(listed[index - 1]!.total);
  const strongRow = listed.find((entry) => entry.name.startsWith(strongName))!;
  const partialRow = listed.find((entry) => entry.name.startsWith(partialName))!;
  expect(strongRow.total).toBe(PAR.medium);
  expect(partialRow.total).toBe(partialPoints);
  expect(strongRow.rank).toBeLessThan(partialRow.rank);
  expect(listed.filter((entry) => entry.own)).toEqual([]);

  await primary(card(strong.page, "board")).click();
  await expect(pane(strong.page, "board").locator('tbody tr[aria-current="true"]')).toContainText(strongName);
  expect((await rows(strong)).filter((entry) => entry.own).map((entry) => entry.total)).toEqual([PAR.medium]);

  expect(await play(partial, "perfect")).toBe(PAR.medium);
  await expect.poll(async () => (await rows(watcher)).find((entry) => entry.name.startsWith(partialName))?.total, { timeout: 40_000 }).toBe(PAR.medium);

  const choice = (group: string, name: string | RegExp): ReturnType<typeof board.locator> => board.getByRole("group", { name: group }).getByRole("button", { name });
  const category = (quiz: string): RegExp => new RegExp(`${quizOf(quiz).title.en.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}$`, "u");
  const mine = async (): Promise<readonly number[]> => (await rows(watcher)).filter((entry) => entry.name.startsWith(strongName) || entry.name.startsWith(partialName)).map((entry) => entry.total);
  await expect(choice("Period", "All time")).toHaveAttribute("aria-pressed", "true");
  await expect(choice("Category", "All")).toHaveAttribute("aria-pressed", "true");
  await expect(board.locator("[data-board-window]")).toHaveCount(0);
  for (const period of ["Today", "This week", "This month"]) {
    await choice("Period", period).click();
    await expect(choice("Period", period)).toHaveAttribute("aria-pressed", "true");
    await expect(board.locator("[data-board-window]")).toBeVisible();
    await expect(row(strongName)).toBeVisible();
    await expect(row(partialName)).toBeVisible();
    expect(await mine(), `${period}: both played heating to a perfect score a moment ago`).toEqual([PAR.medium, PAR.medium]);
  }
  await choice("Category", category("heating")).click();
  await expect(choice("Category", category("heating"))).toHaveAttribute("aria-pressed", "true");
  await expect(board.getByRole("columnheader", { name: "Points" })).toBeVisible();
  await expect(row(strongName)).toBeVisible();
  expect(await mine()).toEqual([PAR.medium, PAR.medium]);
  await choice("Category", category("cooling")).click();
  await expect(choice("Category", category("cooling"))).toHaveAttribute("aria-pressed", "true");
  await expect(board.locator("[data-board-count], [data-board-empty]")).toBeVisible();
  await expect(row(strongName), "nobody of this spec played cooling").toHaveCount(0);
  await expect(row(partialName)).toHaveCount(0);
  await choice("Category", "All").click();
  await choice("Period", "All time").click();
  await expect(row(strongName)).toBeVisible();

  const total = board.getByRole("columnheader", { name: "Total" });
  await expect(board.getByRole("columnheader", { name: "Rank" })).toHaveAttribute("aria-sort", "ascending");
  await total.getByRole("button").click();
  await expect(total).toHaveAttribute("aria-sort", "descending");
  await total.getByRole("button").click();
  await expect(total).toHaveAttribute("aria-sort", "ascending");
  const ascending = (await rows(watcher)).map((entry) => entry.total);
  expect(ascending.length).toBeGreaterThan(1);
  expect(ascending, "a click on a heading orders the table by its column").toEqual([...ascending].sort((left, right) => left - right));
  expect(ascending.at(-1)).toBeGreaterThanOrEqual(PAR.medium);
  expect(await watcher.page.evaluate(() => (window as unknown as { neverReloaded?: boolean }).neverReloaded)).toBe(true);
});
