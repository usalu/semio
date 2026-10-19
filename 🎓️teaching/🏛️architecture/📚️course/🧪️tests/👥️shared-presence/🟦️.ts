/** 👥️ Presence between two learners on two devices: each counts the other online and reads where the other is, sees the
 * other's cursor on the same card of the overview and on the same item inside a run of the same quiz (whatever place
 * that item has on each sheet). What the other answers stays out of a run until it is submitted: nothing of it shows
 * unless the learner asks — then the task's figure shows the other thinking along as a dot on the answer given, below
 * the task, so no item moves when the figure opens, the other joins, answers and leaves. Afterwards what everyone
 * answered shows on the results, and on the quiz's page to whoever submitted or asked. This spec runs alone on its
 * proctor, so the counts are exact.
 * @see ../../🎭️e2e/🎚️config/🟦️.ts — why this spec runs after the others
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learners these specs drive */
import { COURSE, answerRun, boxOf, card, classify, count, enter, expect, goHome, handle, itemBoxes, openTaskById, pane, playQuiz, primary, quizOf, restingPage, screen, shownItems, shownTask, submitRun, test, touchSwipe, swipeTo, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

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
      const observed = device.page.locator("[data-follow-screen]");
      const targetScope = await observed.count() > 0 ? observed.locator("[data-course-grid][data-grid-active]") : device.page.locator("#course-main");
      const target = await boxOf(targetScope.locator(`[data-presence-anchor="${anchor}"]`).filter({ visible: true }));
      const tip = await boxOf(cursor);
      return tip.x >= target.x - 1 && tip.x <= target.x + target.width + 1 && tip.y >= target.y - 1 && tip.y <= target.y + target.height + 1;
    })
    .toBe(true);
}

/** 🕳️ Finds a current overview gap by hit testing, optionally from the bottom for a nonzero camera pan. */
async function overviewGap(device: Device, bottom = false): Promise<{ readonly x: number; readonly y: number }> {
  return device.page.evaluate((fromBottom) => {
    const root = document.querySelector<HTMLElement>('[data-course-grid="root"][data-grid-active] > [data-layered-overview]')!;
    const box = root.getBoundingClientRect();
    const rows: number[] = [];
    for (let y = box.top + 8; y < box.bottom - 8; y += 16) rows.push(y);
    if (fromBottom) rows.reverse();
    for (const y of rows) for (let x = box.left + 8; x < box.right - 8; x += 16) {
      const target = document.elementFromPoint(x, y);
      if (target !== null && root.contains(target) && target.closest("[data-overview-card], [data-presence-anchor^='home:']") === null) return { x, y };
    }
    throw new Error("the overview has no gap between its cards");
  }, bottom);
}
/** 💾️ The learner's persisted runs, excluding all ephemeral observation state. */
async function savedRuns(device: Device): Promise<Readonly<Record<string, string>>> {
  return device.page.evaluate(() => Object.fromEntries(Object.keys(localStorage).filter((key) => key.includes(".runs/")).sort().map((key) => [key, localStorage.getItem(key) ?? ""])));
}

/** 👣️ Follows the named learner through the footer's accessible control. */
async function follow(device: Device, name: string): Promise<void> {
  const button = device.page.locator("[data-presence-footer] [data-follow-user]").filter({ hasText: name });
  await expect(button).toBeEnabled();
  await button.click();
  await expect(device.page.locator("[data-following]")).toBeVisible();
}

/** 🪪️ The exact named learner roster, including the observer's own entry. */
async function expectNamedRoster(device: Device, names: readonly string[]): Promise<void> {
  await expect.poll(() => online(device)).toBe(names.length);
  const entries = device.page.locator("[data-presence-footer] > ul > li");
  await expect(entries).toHaveCount(names.length);
  for (const name of names) await expect(entries.filter({ hasText: name })).toHaveCount(1);
}

/** 🔌️ A browser-only gate for this observer's course roster, leaving every other presence scope connected. */
interface RosterGate {
  paused: boolean;
  refused: number;
  opened: number;
  retained?: { readonly tag: string | null; readonly stop: boolean; readonly board: boolean };
  close(durationMs?: number): void;
}

test("cooperative reconnect preserves bounded follow and restores own task after timeout", async ({ device }) => {
  const quiz = quizOf("physics");
  const task = quiz.tasks.find((candidate) => candidate.kind === "classification")!;
  const [annaName, benName] = [handle("Reconnect Observer"), handle("Connected Leader")];
  const anna = await device("en");
  const ben = await device("de");
  await anna.page.addInitScript((course) => {
    const NativeSocket = window.WebSocket;
    const sockets = new Set<WebSocket>();
    const gate: RosterGate = { paused: false, refused: 0, opened: 0, close: (durationMs) => {
      const open = [...sockets].filter((socket) => socket.readyState === NativeSocket.OPEN);
      if (open.length !== 1) throw new Error(`expected one observer roster socket, got ${open.length}`);
      gate.paused = true;
      open[0]!.close(1000, "test observer roster shortage");
      console.debug("[DEBUG] Closed only observer course roster socket; reconnect gate paused");
      if (durationMs !== undefined) {
        setTimeout(() => {
          const stop = document.querySelector<HTMLButtonElement>("[data-follow-stop]");
          gate.retained = { tag: document.querySelector("[data-following]")?.getAttribute("data-following") ?? null, stop: stop !== null && !stop.disabled, board: document.querySelector('[data-follow-screen] [data-layered-pane="board"][data-opened]') !== null };
        }, durationMs / 2);
        setTimeout(() => { gate.paused = false; console.debug("[DEBUG] Resumed observer roster reconnect after browser-timed shortage", durationMs); }, durationMs);
      }
    } };
    (window as unknown as { quizRosterGate: RosterGate }).quizRosterGate = gate;
    window.WebSocket = class extends NativeSocket {
      constructor(url: string | URL, protocols?: string | string[]) {
        const address = new URL(url, window.location.href);
        const roster = address.pathname.endsWith(`/scopes/${encodeURIComponent(course)}/presence/ws`) && address.searchParams.get("surface") === "course" && protocols === "semio.presence.v1";
        if (roster && gate.paused) { gate.refused += 1; throw new Error("test observer roster reconnect paused"); }
        super(url, protocols);
        if (roster) {
          sockets.add(this);
          this.addEventListener("open", () => { gate.opened += 1; console.debug("[DEBUG] Observer course roster socket opened", gate.opened); });
          this.addEventListener("close", () => sockets.delete(this));
        }
      }
    };
  }, COURSE.id);
  await enter(anna, { kind: "pseudonym", handle: annaName });
  await enter(ben, { kind: "pseudonym", handle: benName }, true);
  await playQuiz(anna, quiz.id);
  await openTaskById(anna, task.id);
  const item = (await shownItems(anna))[0]!;
  await classify(anna, item, task.items.find((candidate) => candidate.id === item)!.category!);
  await expect(anna.page.locator(".course-connection")).toHaveAttribute("data-tone", "calm");
  await expect.poll(async () => Object.keys(await savedRuns(anna)).length).toBe(1);
  const before = await savedRuns(anna);
  const ownItems = await shownItems(anna);
  await expectNamedRoster(anna, [annaName, benName]);
  await expectNamedRoster(ben, [annaName, benName]);
  await ben.page.evaluate(() => { window.location.hash = "board"; });
  await expect(pane(ben.page, "board")).toHaveAttribute("data-opened", "");
  await follow(anna, benName);
  const tag = await anna.page.locator("[data-following]").getAttribute("data-following");
  const observed = anna.page.locator("[data-follow-screen]");
  await expect(observed.locator('[data-layered-pane="board"]').last()).toHaveAttribute("data-opened", "");
  await anna.page.evaluate(() => (window as unknown as { quizRosterGate: RosterGate }).quizRosterGate.close(2_000));
  await ben.page.evaluate(() => { window.location.hash = "badges"; });
  await expect(pane(ben.page, "badges")).toHaveAttribute("data-opened", "");
  await expect.poll(() => anna.page.evaluate(() => (window as unknown as { quizRosterGate: RosterGate }).quizRosterGate.retained)).toEqual({ tag, stop: true, board: true });
  expect(await savedRuns(anna)).toEqual(before);
  expect(await anna.page.evaluate(() => (window as unknown as { quizRosterGate: RosterGate }).quizRosterGate.refused)).toBeGreaterThan(0);
  await expectNamedRoster(anna, [annaName, benName]);
  await expectNamedRoster(ben, [annaName, benName]);
  await expect(anna.page.locator("[data-following]")).toHaveAttribute("data-following", tag!);
  await expect(observed.locator('[data-layered-pane="badges"]').last()).toHaveAttribute("data-opened", "");
  expect(await savedRuns(anna)).toEqual(before);
  await anna.page.evaluate(() => (window as unknown as { quizRosterGate: RosterGate }).quizRosterGate.close());
  await expect(anna.page.locator("[data-follow-stop]")).toBeEnabled();
  await expect(observed.locator('[data-layered-pane="badges"]').last()).toHaveAttribute("data-opened", "");
  await expect(anna.page.locator("[data-following]")).toHaveCount(0);
  await expect(observed).toHaveCount(0);
  await expect.poll(() => shownTask(anna)).toBe(task.id);
  expect(await shownItems(anna)).toEqual(ownItems);
  await expect(screen(anna.page, "task").locator(`[data-course-item="${item}"] select`)).toHaveValue(task.items.find((candidate) => candidate.id === item)!.category!);
  expect(await savedRuns(anna)).toEqual(before);
  await anna.page.evaluate(() => { (window as unknown as { quizRosterGate: RosterGate }).quizRosterGate.paused = false; console.debug("[DEBUG] Resumed observer roster after bounded follow expired"); });
  await expectNamedRoster(anna, [annaName, benName]);
  await expectNamedRoster(ben, [annaName, benName]);
  await expect(anna.page.locator("[data-following]")).toHaveCount(0);
  await expect(observed).toHaveCount(0);
  await expect.poll(() => shownTask(anna)).toBe(task.id);
  expect(await savedRuns(anna)).toEqual(before);
  expect(await anna.page.evaluate(() => (window as unknown as { quizRosterGate: RosterGate }).quizRosterGate.opened)).toBe(3);
  console.log("[DEBUG] Actual observer roster outage retained cached follow and Stop; reconnect mirrored same target; timeout restored own saved task without restarting follow");
});

test("cooperative roster and answers", async ({ device }) => {
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
  const crowd = page.locator('[data-card="course-crowd"]');
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

test("cooperative footer, cursors and pages", async ({ device }) => {
  const [annaName, benName] = [handle("Following Anna"), handle("Leading Ben")];
  const anna = await device("en");
  const ben = await device("de");
  await ben.page.setViewportSize({ width: 1600, height: 1000 });
  await enter(anna, { kind: "pseudonym", handle: annaName }, true);
  await enter(ben, { kind: "pseudonym", handle: benName }, true);
  await expect.poll(() => online(anna)).toBe(2);
  const footer = anna.page.locator("[data-presence-footer]");
  await expect(footer).toContainText(annaName);
  await expect(footer).toContainText(benName);
  const before = await savedRuns(anna);

  const gap = await overviewGap(ben);
  await ben.page.mouse.move(gap.x, gap.y);
  const global = anna.page.locator('[data-presence-layer] [data-peer="cursor"]').filter({ hasText: benName });
  await expect(global).toBeVisible();
  await expect(global).toHaveAttribute("data-anchor", "viewport");
  const first = await boxOf(global);
  await ben.page.mouse.move(gap.x + 3, gap.y + 3);
  await expect.poll(async () => {
    const next = await boxOf(global);
    return Math.abs(next.x - first.x) + Math.abs(next.y - first.y);
  }).toBeGreaterThan(1);

  await card(ben.page, "board").locator("[data-overview-card]").hover({ position: { x: 12, y: 12 } });
  await expectInside(anna, peerCursor(anna, "home:board"), "home:board");
  await expect(card(anna.page, "board").locator("[data-card-presence]")).toContainText(benName);
  await follow(anna, benName);
  await expect(anna.page.locator("[data-follow-screen] [data-layered-card=board]").last()).toHaveAttribute("data-revealed", "");
  await expectInside(anna, peerCursor(anna, "home:board"), "home:board");
  const cameraGap = await overviewGap(ben, true);
  await ben.page.mouse.move(cameraGap.x, cameraGap.y);
  await expect(anna.page.locator("[data-follow-screen] [data-layered-card=board]").last()).not.toHaveAttribute("data-revealed", "");
  const stripSelector = '[data-course-grid="root"] > [data-layered-overview] > [data-layered-strip]';
  await expect.poll(async () => {
    const owner = await ben.page.locator(stripSelector).evaluate((element) => (element as HTMLElement).style.transform);
    const observed = await anna.page.locator("[data-follow-screen]").locator(stripSelector).evaluate((element) => (element as HTMLElement).style.transform);
    const values = (transform: string) => (transform.replace("translate3d", "").match(/-?\d+(?:\.\d+)?/gu) ?? []).map(Number);
    const ownerValues = values(owner);
    const observedValues = values(observed);
    expect(ownerValues).toHaveLength(3);
    expect(observedValues).toHaveLength(3);
    expect([...ownerValues, ...observedValues].every(Number.isFinite)).toBe(true);
    expect(ownerValues.slice(0, 2).some((value) => Math.abs(value) > 1)).toBe(true);
    return Math.max(...ownerValues.map((value, index) => Math.abs(value - observedValues[index]!)));
  }).toBeLessThan(0.05);
  await ben.page.evaluate(() => { window.location.hash = "board"; });
  await expect(pane(ben.page, "board")).toHaveAttribute("data-opened", "");
  await expect(anna.page.locator("[data-follow-screen]").locator('[data-layered-pane="board"]').last()).toHaveAttribute("data-opened", "");
  await anna.page.locator("[data-follow-stop]").click();
  await expect(anna.page.locator("[data-following]")).toHaveCount(0);
  await expect(anna.page.locator('[data-course-grid="root"][data-grid-active]')).toBeVisible();
  expect(await savedRuns(anna)).toEqual(before);

  await follow(anna, benName);
  await ben.context.close();
  await expect.poll(() => online(anna)).toBe(1);
  await expect(anna.page.locator("[data-following]")).toHaveCount(0);
  await expect(anna.page.locator("[data-follow-screen]")).toHaveCount(0);
  console.log("[DEBUG] Cooperative footer, gap/card cursors, page follow, cancellation and departure verified");
});

test("cooperative run, answers and results", async ({ device }) => {
  const quiz = quizOf("physics");
  const task = quiz.tasks.find((candidate) => candidate.kind === "classification")!;
  const [annaName, benName] = [handle("Observer Anna"), handle("Player Ben")];
  const anna = await device("en");
  const ben = await device("de");
  await enter(anna, { kind: "pseudonym", handle: annaName });
  await enter(ben, { kind: "pseudonym", handle: benName });
  await playQuiz(anna, quiz.id);
  await openTaskById(anna, task.id);
  const ownItems = await shownItems(anna);
  await expect.poll(async () => Object.keys(await savedRuns(anna)).length).toBe(1);
  await expect(anna.page.locator(".course-connection")).toHaveAttribute("data-tone", "calm");
  const before = await savedRuns(anna);
  await playQuiz(ben, quiz.id);
  await openTaskById(ben, task.id);
  await follow(anna, benName);
  await expect(anna.page.locator("[data-follow-screen]")).toHaveAttribute("data-follow-screen", "run");
  await expect.poll(() => shownTask(anna)).toBe(task.id);
  await expect.poll(() => shownItems(anna)).toEqual(await shownItems(ben));
  const item = (await shownItems(ben))[0]!;
  const category = task.items.find((candidate) => candidate.id === item)!.category!;
  await classify(ben, item, category);
  const answer = screen(anna.page, "task").locator(`[data-course-item="${item}"] select`);
  await expect(answer).toHaveValue(category);
  await expect(answer).toBeDisabled();
  expect(await savedRuns(anna)).toEqual(before);
  const other = quiz.tasks.find((candidate) => candidate.id !== task.id)!;
  await openTaskById(ben, other.id);
  await expect.poll(() => shownTask(anna)).toBe(other.id);
  await anna.page.keyboard.press("Escape");
  await expect(anna.page.locator("[data-following]")).toHaveCount(0);
  await expect.poll(() => shownTask(anna)).toBe(task.id);
  expect(await shownItems(anna)).toEqual(ownItems);
  expect(await savedRuns(anna)).toEqual(before);

  await follow(anna, benName);
  await answerRun(ben, quiz, "perfect");
  await primary(screen(ben.page, "run")).click();
  const observedDialog = anna.page.locator("[data-observed-dialog]");
  await expect(observedDialog).toBeVisible();
  await expect(primary(observedDialog)).toBeDisabled();
  await expect(anna.page.locator("[data-follow-stop]")).toBeEnabled();
  await primary(ben.page.getByRole("alertdialog")).click();
  await expect(screen(ben.page, "results")).toBeVisible();
  await expect(anna.page.locator("[data-follow-screen]")).toHaveAttribute("data-follow-screen", "results");
  await expect(screen(anna.page, "results")).toBeVisible();
  await expect(anna.page.locator("[data-follow-screen]")).toHaveAttribute("lang", "de");
  await expect.poll(() => screen(anna.page, "results").locator(".course-summary p").allTextContents()).toEqual(await screen(ben.page, "results").locator(".course-summary p").allTextContents());
  await expect.poll(() => screen(anna.page, "task-result").evaluateAll((elements) => elements.map((element) => element.getAttribute("data-presence-anchor")))).toEqual(await screen(ben.page, "task-result").evaluateAll((elements) => elements.map((element) => element.getAttribute("data-presence-anchor"))));
  expect(await savedRuns(anna)).toEqual(before);
  console.log("[DEBUG] Remote randomized sheet, task navigation, draft answer, results and observer persistence verified");
});

test("cooperative mobile swipe", async ({ device }) => {
  const [annaName, benName] = [handle("Desktop Observer"), handle("Phone Player")];
  const anna = await device("en");
  const ben = await device("de");
  await ben.page.setViewportSize({ width: 375, height: 812 });
  await enter(anna, { kind: "pseudonym", handle: annaName }, true);
  await enter(ben, { kind: "pseudonym", handle: benName }, true);
  await expect.poll(() => online(anna)).toBe(2);
  const before = await savedRuns(anna);
  await follow(anna, benName);
  let overview = anna.page.locator('[data-follow-screen] [data-course-grid="root"][data-grid-active] > [data-layered-overview]');
  await expect(overview).toHaveAttribute("data-mode", "swipe");
  const observedPage = () => overview.evaluate((root) => {
    const view = root.getBoundingClientRect();
    const cell = [...root.querySelectorAll<HTMLElement>(":scope > [data-layered-overlay] > [data-layered-cell]")].find((candidate) => {
      const box = candidate.getBoundingClientRect();
      return Math.abs(box.left - view.left) < 1 && Math.abs(box.top - view.top) < 1;
    });
    return cell?.dataset.layeredCell ?? null;
  });
  await expect.poll(observedPage).toBe("learner");
  await touchSwipe(ben.page, 240, 0);
  await expect.poll(() => restingPage(ben.page)).toBe("board");
  await expect.poll(observedPage).toBe("board");
  await touchSwipe(ben.page, -240, 0);
  await expect.poll(() => restingPage(ben.page)).toBe("learner");
  await expect.poll(observedPage).toBe("learner");
  await touchSwipe(ben.page, 0, 325);
  await expect.poll(() => restingPage(ben.page)).toBe("board");
  await expect.poll(observedPage).toBe("board");
  await ben.page.evaluate(() => { window.location.hash = "quizzes"; });
  await expect(ben.page.locator('[data-course-grid="quizzes"][data-grid-active]')).toBeVisible();
  await expect.poll(() => restingPage(ben.page)).not.toBeNull();
  overview = anna.page.locator('[data-follow-screen] [data-course-grid="quizzes"][data-grid-active] > [data-layered-overview]');
  await expect(overview).toHaveAttribute("data-mode", "swipe");
  await expect.poll(observedPage).toBe(await restingPage(ben.page));
  const cells = await ben.page.locator('[data-course-grid="quizzes"][data-grid-active] > [data-layered-overview] > [data-layered-overlay] > [data-layered-cell]').evaluateAll((elements) => elements.map((element) => (element as HTMLElement).dataset.layeredCell!));
  const currentPage = await restingPage(ben.page);
  const next = cells.find((id) => id !== currentPage)!;
  await swipeTo(ben, next);
  await expect.poll(observedPage).toBe(next);
  await anna.page.locator("[data-follow-stop]").click();
  await expect(anna.page.locator("[data-follow-screen]")).toHaveCount(0);
  expect(await savedRuns(anna)).toEqual(before);
  console.log("[DEBUG] Phone swipe position and mode followed on a desktop across wrapped columns and rows");
});

test("cooperative card scroll", async ({ device }) => {
  const [annaName, benName] = [handle("Scroll Observer"), handle("Scroll Player")];
  const anna = await device("en");
  const ben = await device("de");
  await anna.page.setViewportSize({ width: 1440, height: 100 });
  await ben.page.setViewportSize({ width: 375, height: 200 });
  await enter(anna, { kind: "pseudonym", handle: annaName }, true);
  await enter(ben, { kind: "pseudonym", handle: benName }, true);
  await expect.poll(() => online(anna)).toBe(2);
  await swipeTo(ben, "badges");
  await follow(anna, benName);
  const selector = '[data-course-grid="root"][data-grid-active] > [data-layered-overview] > [data-layered-overlay] > [data-layered-cell="badges"]';
  const owner = ben.page.locator(selector);
  const observed = anna.page.locator("[data-follow-screen]").locator(selector);
  await expect.poll(() => owner.evaluate((element) => element.scrollHeight - element.clientHeight)).toBeGreaterThan(1);
  await expect.poll(() => observed.evaluate((element) => element.scrollHeight - element.clientHeight)).toBeGreaterThan(1);
  await owner.evaluate((element) => { element.scrollTop = (element.scrollHeight - element.clientHeight) * 0.65; });
  const fraction = (element: Element) => element.scrollTop / Math.max(1, element.scrollHeight - element.clientHeight);
  await expect.poll(() => owner.evaluate(fraction)).toBeCloseTo(0.65, 1);
  await expect.poll(() => observed.evaluate(fraction)).toBeCloseTo(await owner.evaluate(fraction), 1);
  await swipeTo(ben, "board");
  const boardSelector = selector.replace('data-layered-cell="badges"', 'data-layered-cell="board"');
  const observedBoard = anna.page.locator("[data-follow-screen]").locator(boardSelector);
  await expect.poll(() => observedBoard.evaluate((element) => {
    const root = element.closest("[data-layered-overview]")!.getBoundingClientRect();
    const cell = element.getBoundingClientRect();
    return Math.abs(root.left - cell.left) < 1 && Math.abs(root.top - cell.top) < 1;
  })).toBe(true);
  await expect.poll(() => observedBoard.evaluate((element) => element.scrollTop)).toBe(0);
  await anna.page.locator("[data-follow-stop]").click();
  await expect(anna.page.locator("[data-follow-screen]")).toHaveCount(0);
  console.log("[DEBUG] Current phone card scroll mirrored across viewport sizes and recaptured after swipe");
});
