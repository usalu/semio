/** 🥞️ Home on a desktop: nine cards in reading order with the leaderboard at the centre, and behind the glass the real
 * page of every card, each as large as the screen, mounted and live but inert at rest. Between the cards the mouse pans
 * those pages under the glass, as on semio-tech play; hovering or keyboard-focusing a card brings its page to the
 * screen, clear, and never starts a run — on a device that reports reduced motion too. A page opens
 * full size by its hash or its card
 * and closes with Escape or the navbar's overview button. The navbar leads the way from its left — the overview, back
 * and forward along the trail, up from a run to its quiz — around what the quizzes are about in its middle.
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🚏️navigation/🟦️.tsx — the ways and the address
 * @see ../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx — the layered overview */
import { CATALOG, QUIZZES, boxOf, card, enter, expect, handle, pane, playQuiz, primary, screen, test, way, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

const [first, second, third, fourth] = QUIZZES.map((quiz) => quiz.id);
const PAGES = ["learner", first!, "intro", second!, "board", third!, "badges", fourth!, "prefs"];

/** 📏️ The grid of the desktop overview: three columns and three rows whose middle ones are the larger (the weights the
 * client lays its cards and pages out with). */
const GRID = { columns: [1, 1.5, 1], rows: [1, 1.4, 1] } as const;

/** 📐️ Where the tracks of `weights` start and end along an axis that starts at `origin` and is `length` long. */
function tracks(weights: readonly number[], origin: number, length: number): readonly { readonly start: number; readonly end: number }[] {
  const total = weights.reduce((sum, weight) => sum + weight, 0);
  return weights.map((weight, index) => {
    const start = origin + (length * weights.slice(0, index).reduce((sum, before) => sum + before, 0)) / total;
    return { start, end: start + (length * weight) / total };
  });
}

/** 🏷️ The `data-page` every page behind a card carries. */
function pageOf(id: string): string {
  return QUIZZES.some((quiz) => quiz.id === id) ? `quiz:${id}` : id;
}

/** ⏳️ How long a pan needs to leave its place visibly: asserting stillness earlier would pass for a pan that had only
 * not started yet. */
const PAN_SETTLE_MS = 800;

/** 📍️ Where the page of `id` lies relative to the overview, in pixels: its left and top edge and its size. */
async function placeOf(device: Device, id: string): Promise<{ readonly x: number; readonly y: number; readonly width: number; readonly height: number }> {
  const [frame, box] = [await boxOf(device.page.locator("[data-layered-overview]")), await boxOf(pane(device.page, id))];
  return { x: box.x - frame.x, y: box.y - frame.y, width: box.width, height: box.height };
}

async function overview(device: Device): Promise<ReturnType<Device["page"]["locator"]>> {
  const root = device.page.locator("[data-layered-overview]");
  await expect(root).toHaveAttribute("data-mode", "strip");
  return root;
}

test("home is a grid of nine cards around the leaderboard, with every real page live behind the glass", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Home Grid") }, true);
  const root = await overview(learner);
  await expect(root).toHaveAttribute("data-pan", "pointer");
  expect(await root.locator("[data-layered-card]").evaluateAll((cards) => cards.map((element) => (element as HTMLElement).dataset.layeredCard))).toEqual(PAGES);
  expect(PAGES.indexOf("board")).toBe(4);

  const frame = await boxOf(root);
  const [columns, rows] = [tracks(GRID.columns, frame.x, frame.width), tracks(GRID.rows, frame.y, frame.height)];
  for (const [index, id] of PAGES.entries()) {
    const box = await boxOf(card(learner.page, id).locator("[data-overview-card]"));
    const [column, row] = [columns[index % 3]!, rows[Math.floor(index / 3)]!];
    expect({ id, left: box.x >= column.start - 1, right: box.x + box.width <= column.end + 1, top: box.y >= row.start - 1, bottom: box.y + box.height <= row.end + 1 }, `${id} lies in the cell of column ${index % 3}, row ${Math.floor(index / 3)}`).toEqual({ id, left: true, right: true, top: true, bottom: true });
    if (id !== "board") continue;
    expect(Math.abs(box.x + box.width / 2 - (frame.x + frame.width / 2)), "the leaderboard is centred across").toBeLessThan(2);
    expect(Math.abs(box.y + box.height / 2 - (frame.y + frame.height / 2)), "the leaderboard is centred down").toBeLessThan(2);
  }
  expect(columns[1]!.end - columns[1]!.start).toBeGreaterThan(columns[0]!.end - columns[0]!.start);
  expect(rows[1]!.end - rows[1]!.start).toBeGreaterThan(rows[0]!.end - rows[0]!.start);

  await expect(root.locator("[data-layered-pane]")).toHaveCount(PAGES.length);
  for (const id of PAGES) {
    const page = pane(learner.page, id);
    await expect(page).toHaveAttribute("inert", "");
    await expect(page).not.toHaveAttribute("data-opened", "");
    await expect(page.locator(`[data-page="${pageOf(id)}"]`)).toHaveCount(1);
  }
  for (const quiz of QUIZZES) for (const task of quiz.tasks) await expect(pane(learner.page, quiz.id)).toContainText(task.title.en);
  for (const paragraph of CATALOG.introduction.paragraphs) await expect(pane(learner.page, "intro")).toContainText(paragraph.en);
  for (const badge of CATALOG.badges) await expect(pane(learner.page, "badges")).toContainText(badge.label.en);
  await expect(pane(learner.page, "board").locator('[data-card="leaderboard"]')).toHaveCount(1);
  await expect(pane(learner.page, "learner")).toContainText(await card(learner.page, "learner").getByRole("heading").innerText());
  await expect(root.locator("[data-layered-veil]")).toHaveAttribute("data-veil", "whole");
});

test("between the cards the mouse pans the screen-sized pages under the glass; on a card its page comes to the screen, clear; whatever the device says about motion", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Home Pan") }, true);
  const root = await overview(learner);
  await expect(root).toHaveAttribute("data-pan", "pointer");
  const veil = root.locator("[data-layered-veil]");
  const frame = await boxOf(root);
  const cards = await Promise.all(PAGES.map((id) => boxOf(card(learner.page, id).locator("[data-overview-card]"))));
  for (const [index, id] of PAGES.entries()) {
    const place = await placeOf(learner, id);
    expect([Math.round(place.width), Math.round(place.height)], `${id} is as large as the overview`).toEqual([Math.round(frame.width), Math.round(frame.height)]);
    expect(Math.max(Math.abs(place.x - (index % 3) * frame.width), Math.abs(place.y - Math.floor(index / 3) * frame.height)), `${id} lies in its cell of the strip, the first page on the screen`).toBeLessThan(1.5);
  }

  const offPan = async (id: string, at: { readonly x: number; readonly y: number }): Promise<number> => {
    const index = PAGES.indexOf(id);
    const [place, across, down] = [await placeOf(learner, id), ((at.x - frame.x) / frame.width) * 2, ((at.y - frame.y) / frame.height) * 2];
    return Math.max(Math.abs(place.x - ((index % 3) - across) * frame.width), Math.abs(place.y - (Math.floor(index / 3) - down) * frame.height));
  };
  for (const [fx, fy, shown] of [[1, 1, "prefs"], [1, 0, "intro"], [0.5, 1, fourth!], [0, 0, "learner"]] as const) {
    const at = { x: frame.x + Math.min(frame.width - 2, Math.max(2, fx * frame.width)), y: frame.y + Math.min(frame.height - 2, Math.max(2, fy * frame.height)) };
    await learner.page.mouse.move(at.x, at.y, { steps: 4 });
    await expect.poll(() => offPan(shown, at), { message: `the pointer at ${fx}, ${fy} pans to ${shown}` }).toBeLessThan(1.5);
    const place = await placeOf(learner, shown);
    expect(Math.max(Math.abs(place.x), Math.abs(place.y)), `${shown} fills the screen behind the glass`).toBeLessThan(6);
    await expect(veil).toHaveAttribute("data-veil", "whole");
    await expect(veil).toBeVisible();
    expect(await Promise.all(PAGES.map((id) => boxOf(card(learner.page, id).locator("[data-overview-card]")))), "the cards stay where they are").toEqual(cards);
  }

  const onScreen = async (id: string): Promise<number> => {
    const place = await placeOf(learner, id);
    return Math.max(Math.abs(place.x), Math.abs(place.y));
  };
  const board = card(learner.page, "board").locator("[data-overview-card]");
  await board.hover();
  await expect(card(learner.page, "board")).toHaveAttribute("data-revealed", "");
  await expect.poll(() => onScreen("board"), { message: "the page of the hovered card glides to the screen" }).toBeLessThan(0.5);
  await expect(veil).toHaveAttribute("data-veil", "clear");
  await expect(veil).toBeHidden();
  const held = await boxOf(board);
  await learner.page.mouse.move(held.x + 8, held.y + held.height - 8, { steps: 3 });
  await learner.page.waitForTimeout(PAN_SETTLE_MS);
  expect(await onScreen("board"), "the page stays while the mouse moves on its card").toBeLessThan(0.5);
  const corner = { x: frame.x + frame.width - 2, y: frame.y + 2 };
  await learner.page.mouse.move(corner.x, corner.y, { steps: 4 });
  await expect(veil).toHaveAttribute("data-veil", "whole");
  await expect.poll(() => offPan("intro", corner), { message: "between the cards the pages pan again" }).toBeLessThan(1.5);

  await learner.page.emulateMedia({ reducedMotion: "reduce" });
  await expect(root, "a device that reports reduced motion — every browser in a Remote Desktop session — pans all the same").toHaveAttribute("data-pan", "pointer");
  const last = { x: frame.x + frame.width - 2, y: frame.y + frame.height - 2 };
  await learner.page.mouse.move(last.x, last.y, { steps: 4 });
  await expect.poll(() => offPan("prefs", last), { message: "the pages pan under reduced motion too" }).toBeLessThan(1.5);
  await expect(veil).toHaveAttribute("data-veil", "whole");
  await board.hover();
  await expect.poll(() => onScreen("board"), { message: "and the page of a hovered card comes to the screen" }).toBeLessThan(0.5);
  await expect(veil).toHaveAttribute("data-veil", "clear");
});

test("hovering or focusing a card shows its page clear and never starts a run", async ({ device }) => {
  const learner = await device("de");
  await enter(learner, { kind: "pseudonym", handle: handle("Home Hover") });
  const root = await overview(learner);
  const veil = root.locator("[data-layered-veil]");
  const frame = await boxOf(root);
  for (const id of [second!, "board"]) {
    await card(learner.page, id).locator("[data-overview-card]").hover();
    await expect(card(learner.page, id)).toHaveAttribute("data-revealed", "");
    await expect(veil).toHaveAttribute("data-veil", "clear");
    await expect(veil).toBeHidden();
    await expect.poll(async () => Object.values(await boxOf(pane(learner.page, id))).map(Math.round), { message: `the page of ${id} fills the overview` }).toEqual(Object.values(frame).map(Math.round));
    await expect(pane(learner.page, id)).toHaveAttribute("inert", "");
    await learner.page.mouse.move(frame.x + 2, frame.y + 2);
    await expect(card(learner.page, id)).not.toHaveAttribute("data-revealed", "");
    await expect(veil).toHaveAttribute("data-veil", "whole");
  }

  const heading = card(learner.page, third!).getByRole("link");
  for (let presses = 0; presses < 60 && !(await heading.evaluate((link) => link === document.activeElement)); presses++) await learner.page.keyboard.press("Tab");
  await expect(heading).toBeFocused();
  await expect(card(learner.page, third!)).toHaveAttribute("data-revealed", "");
  await expect(veil).toHaveAttribute("data-veil", "clear");

  await expect(screen(learner.page, "run")).toHaveCount(0);
  await expect(pane(learner.page, "learner").locator('[data-card="runs"] tbody tr')).toHaveCount(0);
});

test("a page opens by its hash or its card and closes with Escape or the navbar's overview button", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Home Hash") });
  const root = await overview(learner);
  const board = pane(learner.page, "board");
  await expect(way(learner.page, "overview")).toHaveAttribute("aria-disabled", "true");

  await learner.page.goto("/#board");
  await expect(board).toHaveAttribute("data-opened", "");
  await expect(board).toHaveRole("region");
  await expect(board).not.toHaveAttribute("inert", "");
  await expect(root.locator("[data-layered-veil]")).toHaveCount(0);
  await expect(root.locator("[data-layered-overview-button]")).toHaveCount(0);
  await expect(way(learner.page, "overview")).toHaveAttribute("aria-disabled", "false");
  await expect(way(learner.page, "overview")).toHaveAttribute("aria-keyshortcuts", "Escape");
  await learner.page.keyboard.press("Escape");
  await expect(board).not.toHaveAttribute("data-opened", "");
  await expect(board).toHaveAttribute("inert", "");
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("");
  await expect(card(learner.page, "board").locator(":focus")).toHaveCount(1);

  await primary(card(learner.page, "board")).click();
  await expect(board).toHaveAttribute("data-opened", "");
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("#board");
  await way(learner.page, "overview").click();
  await expect(board).not.toHaveAttribute("data-opened", "");
  await expect.poll(() => new URL(learner.page.url()).hash).toBe("");

  await learner.page.evaluate((id) => (window.location.hash = id), second!);
  await expect(pane(learner.page, second!)).toHaveAttribute("data-opened", "");
  await expect(pane(learner.page, second!)).toContainText(QUIZZES[1]!.description.en);
  await learner.page.keyboard.press("Escape");
  await expect(pane(learner.page, second!)).not.toHaveAttribute("data-opened", "");

  await learner.page.goto("/#board");
  await learner.page.reload();
  await expect(board).toHaveAttribute("data-opened", "");
  await expect(way(learner.page, "back"), "a page the address named on arrival has nothing behind it").toHaveAttribute("aria-disabled", "true");
  await expect(way(learner.page, "up")).toHaveAttribute("aria-disabled", "false");
});

test("the navbar leads the way: the overview, back and forward along the trail, and up from a run to its quiz, around what the quizzes are about", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Home Ways") });
  await overview(learner);
  const { page } = learner;
  const ways = ["overview", "back", "forward", "up"] as const;
  for (const name of ways) await expect(way(page, name)).toHaveAttribute("aria-disabled", "true");

  const bar = await boxOf(page.locator("header nav"));
  const boxes = await Promise.all(ways.map((name) => boxOf(way(page, name))));
  expect(boxes.map((box) => box.x)).toEqual([...boxes.map((box) => box.x)].sort((left, right) => left - right));
  expect(boxes[0]!.x - bar.x, "the ways start the navbar").toBeLessThan(16);
  const brand = page.locator("header [data-quiz-brand]");
  await expect(brand).toContainText(CATALOG.title.en);
  const middle = await boxOf(brand);
  expect(Math.abs(middle.x + middle.width / 2 - (bar.x + bar.width / 2)), "what the quizzes are about stands in the middle").toBeLessThanOrEqual(1);
  expect(middle.x).toBeGreaterThan(boxes[3]!.x + boxes[3]!.width);

  const board = pane(page, "board");
  await primary(card(page, "board")).click();
  await expect(board).toHaveAttribute("data-opened", "");
  await expect(way(page, "back")).toHaveAccessibleName("Back: Overview");
  await way(page, "back").click();
  await expect(board).not.toHaveAttribute("data-opened", "");
  await expect(way(page, "forward")).toHaveAccessibleName("Forward: Leaderboard");
  await way(page, "forward").click();
  await expect(board).toHaveAttribute("data-opened", "");
  await expect(way(page, "forward")).toHaveAttribute("aria-disabled", "true");
  await way(page, "overview").click();
  await expect(board).not.toHaveAttribute("data-opened", "");

  await playQuiz(learner, second!);
  await expect(way(page, "up")).toHaveAccessibleName(`Up: ${QUIZZES[1]!.title.en}`);
  await way(page, "up").click();
  await expect(pane(page, second!)).toHaveAttribute("data-opened", "");
  await expect.poll(() => new URL(page.url()).hash).toBe(`#${second!}`);
  await way(page, "back").click();
  await expect(screen(page, "run")).toBeVisible();
  await expect.poll(() => new URL(page.url()).hash).toBe("");
  await way(page, "overview").click();
  await expect(screen(page, "run")).toHaveCount(0);
  await expect(page.locator("[data-layered-pane][data-opened]")).toHaveCount(0);
  await expect(way(page, "back")).toHaveAccessibleName(`Back: ${QUIZZES[1]!.title.en}`);
});
