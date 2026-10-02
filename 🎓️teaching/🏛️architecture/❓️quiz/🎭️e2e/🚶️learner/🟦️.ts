/** 🚶️ The learner the end-to-end specs drive: a device (one browser context, as one person's browser) that fails its test
 * on any console error, page error or failed request it did not announce, the steps of the journey (arrive, read the
 * introduction, identify, start, answer, submit), and the catalog's own sources, from which every answer is computed by
 * item id — sheets are drawn and shuffled per run, so nothing here ever answers by position.
 *
 * Elements are found by the hooks the client renders for styles, presence and tests (`data-card`, `data-layered-*`,
 * `data-quiz-item`, `data-overview-card-action`, …) and by role; what is asserted as text comes from the catalog sources
 * in the learner's language.
 * @see ../🟦️.ts — the gate that boots the stack
 * @see ../../🔣️.json — the catalog; its quizzes hold the solutions
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🟦️.tsx — the client under test */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test as base, type Browser, type BrowserContext, type Locator, type Page } from "@playwright/test";

//#region 🧾️Stack
const here = dirname(fileURLToPath(import.meta.url));
const site = resolve(here, "../..");

/** 🗺️ The topology the specs run in: `dev` (one origin through the dev proxy) or `rehearsal` (static site, proctor on
 * another origin). */
export const TOPOLOGY = (process.env.TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY ?? "dev") as "dev" | "rehearsal";

/** 🛂️ The proctor origin of the stack under test. */
export const PROCTOR_ORIGIN = process.env.TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR ?? "";

const CONTROL = process.env.TEACHING_ARCHITECTURE_QUIZ_E2E_CONTROL ?? "";

/** 🎛️ Takes the stack's proctor away (`stop`) or brings it back and waits until it is ready (`start`). */
export async function proctor(action: "stop" | "start"): Promise<void> {
  const answer = await fetch(`${CONTROL}/proctor/${action}`, { method: "POST" });
  if (answer.status !== 204) throw new Error(`the stack could not ${action} its proctor: ${answer.status} ${await answer.text()}`);
}
//#endregion 🧾️Stack

//#region 📚️Catalog
/** 🗣️ A language of the site. */
export type Locale = "en" | "de";

/** 🗣️ Both languages, in no order of preference. */
export const LOCALES: readonly Locale[] = ["en", "de"];

/** 🌍️ A text of the catalog in both languages. */
export type Text = Readonly<Record<Locale, string>>;

/** 📏️ A quantity of a sorting task or a matching dimension. */
export interface SourceQuantity {
  readonly label: Text;
  readonly unit: string;
  readonly prefixed: boolean;
}

/** 🧩️ An item of a task as authored, with its solution: the category it belongs to, its value, or its value per
 * dimension. */
export interface SourceItem {
  readonly id: string;
  readonly label: Text;
  readonly explanation: Text;
  readonly category?: string;
  readonly value?: number;
  readonly values?: Readonly<Record<string, number>>;
}

/** 📝️ A task as authored. */
export interface SourceTask {
  readonly kind: "classification" | "sorting" | "matching";
  readonly id: string;
  readonly title: Text;
  readonly items: readonly SourceItem[];
  readonly draw?: number;
  readonly categories?: readonly { readonly id: string; readonly label: Text }[];
  readonly quantity?: SourceQuantity;
  readonly dimensions?: readonly { readonly id: string; readonly quantity: SourceQuantity }[];
}

/** ❓️ A quiz as authored. */
export interface SourceQuiz {
  readonly id: string;
  readonly title: Text;
  readonly description: Text;
  readonly tasks: readonly SourceTask[];
}

/** 🏅️ A badge as authored. */
export interface SourceBadge {
  readonly id: string;
  readonly label: Text;
  readonly rule: { readonly kind: string; readonly quiz?: string; readonly taskKind?: string };
}

function json<T>(path: string): T {
  return JSON.parse(readFileSync(path, "utf8")) as T;
}

/** 📚️ The catalog the site offers, as authored. */
export const CATALOG = json<{ readonly id: string; readonly title: Text; readonly introduction: { readonly title: Text; readonly paragraphs: readonly Text[] }; readonly quizzes: readonly string[]; readonly badges: readonly SourceBadge[] }>(resolve(site, "🔣️.json"));

/** ❓️ The quizzes of the catalog in catalog order, with their solutions. */
export const QUIZZES: readonly SourceQuiz[] = CATALOG.quizzes.map((path) => json<SourceQuiz>(resolve(site, path)));

/** ❓️ The quiz `id` of the catalog. */
export function quizOf(id: string): SourceQuiz {
  const quiz = QUIZZES.find((candidate) => candidate.id === id);
  if (quiz === undefined) throw new Error(`the catalog has no quiz ${id}`);
  return quiz;
}

/** 🏅️ The badge `id` of the catalog. */
export function badgeOf(id: string): SourceBadge {
  const badge = CATALOG.badges.find((candidate) => candidate.id === id);
  if (badge === undefined) throw new Error(`the catalog has no badge ${id}`);
  return badge;
}

/** 🏅️ The badges a learner holds who has played exactly `perfect` (quiz ids) once, each without a mistake. */
export function badgesFor(perfect: readonly string[]): readonly SourceBadge[] {
  const played = new Set(perfect);
  return CATALOG.badges.filter((badge) => {
    if (badge.rule.kind === "perfect-quiz") return played.has(badge.rule.quiz!);
    if (badge.rule.kind === "completed-quizzes") return QUIZZES.every((quiz) => played.has(quiz.id));
    const asked = QUIZZES.filter((quiz) => badge.rule.quiz === undefined || quiz.id === badge.rule.quiz).flatMap((quiz) => quiz.tasks.filter((task) => badge.rule.taskKind === undefined || task.kind === badge.rule.taskKind).map(() => quiz.id));
    return asked.length > 0 && asked.every((quiz) => played.has(quiz));
  });
}

function taskOf(quiz: SourceQuiz, id: string): SourceTask {
  const task = quiz.tasks.find((candidate) => candidate.id === id);
  if (task === undefined) throw new Error(`quiz ${quiz.id} has no task ${id}`);
  return task;
}

function itemOf(task: SourceTask, id: string): SourceItem {
  const item = task.items.find((candidate) => candidate.id === id);
  if (item === undefined) throw new Error(`task ${task.id} has no item ${id}`);
  return item;
}

/** 📶️ `ids` in the true ascending order of a sorting task: by value, ties in the order they were authored. */
export function ascending(task: SourceTask, ids: readonly string[]): readonly string[] {
  const authored = new Map(task.items.map((item, index) => [item.id, index]));
  return [...ids].sort((left, right) => itemOf(task, left).value! - itemOf(task, right).value! || authored.get(left)! - authored.get(right)!);
}
//#endregion 📚️Catalog

//#region 🖥️Device
/** 🚧️ What a device saw go wrong, as one line each. */
type Problems = string[];

/** 🖥️ One person's browser: its page, its language, and everything that went wrong in it — console errors, page
 * errors, failed requests, and every violation of the document's content security policy. `expectingFailures` runs
 * `body` while failed requests and console errors are expected (the proctor is away) and returns how many it saw; a
 * policy violation is never expected. */
export interface Device {
  readonly context: BrowserContext;
  readonly page: Page;
  readonly locale: Locale;
  readonly problems: Problems;
  expectingFailures(body: () => Promise<void>): Promise<number>;
}

const BROWSER_LOCALES: Readonly<Record<Locale, string>> = { en: "en-GB", de: "de-DE" };

/** ✋️ What Chromium says when the page itself closes a socket it was still opening (the learner left the room): an
 * intended abort, like a cancelled request. */
const SOCKET_ABANDONED = "WebSocket is closed before the connection is established";

function watch(page: Page, problems: Problems, tolerated: { on: boolean; seen: number }): void {
  const report = (line: string): void => {
    if (tolerated.on) tolerated.seen += 1;
    else problems.push(line);
  };
  page.on("console", (message) => {
    if (message.type() === "error") report(`console error: ${message.text()} (${message.location().url})`);
  });
  page.on("pageerror", (error) => report(`page error: ${error.message}`));
  page.on("requestfailed", (request) => {
    const reason = request.failure()?.errorText ?? "";
    if (reason !== "net::ERR_ABORTED") report(`request failed: ${request.method()} ${request.url()} ${reason}`);
  });
  page.on("response", (response) => {
    if (response.status() >= 400) report(`request answered ${response.status()}: ${response.request().method()} ${response.url()}`);
  });
  page.on("websocket", (socket) =>
    socket.on("socketerror", (error) => {
      if (!error.includes(SOCKET_ABANDONED)) report(`socket error: ${socket.url()} ${error}`);
    }),
  );
}

async function openDevice(browser: Browser, options: Parameters<Browser["newContext"]>[0], locale: Locale, language: string): Promise<Device> {
  const context = await browser.newContext({ ...options, locale: language });
  const problems: Problems = [];
  await context.exposeBinding("quizSecurityViolation", (_source, violation: string) => void problems.push(`content security policy: ${violation}`));
  await context.addInitScript(() =>
    document.addEventListener("securitypolicyviolation", (event) => (window as unknown as { quizSecurityViolation(violation: string): void }).quizSecurityViolation(`${event.effectiveDirective} blocked ${event.blockedURI || "inline code"} in ${event.documentURI}`)),
  );
  const page = await context.newPage();
  const tolerated = { on: false, seen: 0 };
  watch(page, problems, tolerated);
  return {
    context,
    page,
    locale,
    problems,
    expectingFailures: async (body) => {
      tolerated.on = true;
      tolerated.seen = 0;
      try {
        await body();
      } finally {
        tolerated.on = false;
      }
      return tolerated.seen;
    },
  };
}

/** 🧪️ The test of the gate: `device(locale)` opens a fresh device (empty storage) in the window of the running project,
 * whose browser asks for `language` (by default the British or German of `locale`, the language the site then speaks);
 * when the test ends every device is closed and the test fails if any of them saw a console error, a page error or a
 * failed request outside an announced shortage. */
export const test = base.extend<{ device: (locale?: Locale, language?: string) => Promise<Device> }>({
  device: async ({ browser }, use, info) => {
    const opened: Device[] = [];
    const { viewport, isMobile, hasTouch, baseURL } = info.project.use;
    await use(async (locale = "en", language = BROWSER_LOCALES[locale]) => {
      const device = await openDevice(browser, { viewport, isMobile, hasTouch, baseURL }, locale, language);
      opened.push(device);
      return device;
    });
    for (const device of opened) await device.context.close();
    expect(opened.flatMap((device, index) => device.problems.map((problem) => `device ${index + 1}: ${problem}`))).toEqual([]);
  },
});

export { expect };
//#endregion 🖥️Device

//#region 🔎️Places
/** 🃏️ The card of `id` on the overview (`learner`, a quiz id, `intro`, `board`, `badges`, `prefs`). */
export function card(page: Page, id: string): Locator {
  return page.locator(`[data-layered-card="${id}"]`);
}

/** 📄️ The page behind the card of `id`. */
export function pane(page: Page, id: string): Locator {
  return page.locator(`[data-layered-pane="${id}"]`);
}

/** 🎬️ The main action of a card (start, submit, continue, …). */
export function primary(scope: Locator | Page): Locator {
  return scope.locator('[data-overview-card-action="primary"]');
}

/** 🎬️ The other actions of a card. */
export function secondary(scope: Locator | Page): Locator {
  return scope.locator('[data-overview-card-action="secondary"]');
}

/** 🚏️ A way of the navbar (`overview`, `back`, `forward`, `up`); it carries `aria-disabled` where it leads nowhere. */
export function way(page: Page, name: "overview" | "back" | "forward" | "up"): Locator {
  return page.locator(`header [data-quiz-nav="${name}"]`);
}

/** 🪧️ A card of the screen in front (`introduction`, `identity`, `run`, `task`, `results`, `task-result`, `dialog`). */
export function screen(page: Page, name: string): Locator {
  return page.locator(`#quiz-main [data-card="${name}"]`).and(page.locator(":not([inert] *)"));
}

/** 🏷️ A random handle no other test uses. */
export function handle(prefix: string): string {
  return `${prefix} ${Math.random().toString(36).slice(2, 8)}`;
}
//#endregion 🔎️Places

//#region 🚶️Journey
/** 🪪️ How a learner identifies. */
export type Identity = { readonly kind: "anonymous" } | { readonly kind: "pseudonym" | "name"; readonly handle: string };

/** 🚪️ Opens the site on a device that has never been there: the introduction shows (within `timeout`, which a cold dev
 * server needs more of). */
export async function arrive(device: Device, timeout?: number): Promise<void> {
  await device.page.goto("/");
  await expect(screen(device.page, "introduction")).toBeVisible({ timeout });
}

/** 👋️ Continues from the introduction to the identity step. */
export async function readIntroduction(device: Device): Promise<void> {
  await primary(screen(device.page, "introduction")).click();
  await expect(screen(device.page, "identity")).toBeVisible();
}

/** 🪪️ Chooses an identity on the identity step and waits for the overview. The step is submitted with a click, which
 * leaves the pointer where the button was: whichever card comes to lie there shows its page clear. `atRest` leaves the
 * overview at rest instead — the pointer is moved to the corner of the navbar first and the step is submitted from the
 * keyboard, so the overview mounts with no card under the pointer and sees no pointer move its camera would follow. */
export async function identify(device: Device, identity: Identity, atRest = false): Promise<void> {
  const form = screen(device.page, "identity");
  const kind = form.locator(`input[type="radio"][value="${identity.kind}"]`);
  await kind.check();
  if (identity.kind !== "anonymous") await form.locator('input[type="text"]').fill(identity.handle);
  if (atRest) {
    await device.page.mouse.move(1, 1);
    await (identity.kind === "anonymous" ? kind : form.locator('input[type="text"]')).press("Enter");
  } else await primary(form).click();
  await expect(device.page.locator("[data-layered-overview]")).toBeVisible();
}

/** 🚶️ The whole first visit: arrive, read the introduction, identify — onto an overview `atRest` when asked for. */
export async function enter(device: Device, identity: Identity, atRest = false): Promise<void> {
  await arrive(device);
  await readIntroduction(device);
  await identify(device, identity, atRest);
}

/** 🧑‍🎓️ The learner's name as the overview shows it. */
export async function shownName(device: Device): Promise<string> {
  return (await card(device.page, "learner").getByRole("heading").innerText()).trim();
}

/** 🆔️ The id of the learner this device acts as (its only credential), as the client keeps it. */
export async function learnerId(device: Device): Promise<string> {
  return device.page.evaluate((key) => (JSON.parse(localStorage.getItem(key) ?? "null") as { id: string } | null)?.id ?? "", `semio.quiz.${CATALOG.id}.learner`);
}

/** ▶️ Presses the main action of a quiz's card (start, resume or start again) and waits for the run. */
export async function playQuiz(device: Device, quiz: string): Promise<void> {
  await primary(card(device.page, quiz)).click();
  await expect(screen(device.page, "run")).toBeVisible();
}

/** 👉️ Opens the task at `index` of the run on screen. */
export async function openTask(device: Device, index: number): Promise<void> {
  const step = screen(device.page, "run").locator("nav button").nth(index);
  await step.click();
  await expect(step).toHaveAttribute("aria-current", "step");
}

/** 👉️ Opens the task `task` of the run on screen, wherever this sheet put it. */
export async function openTaskById(device: Device, task: string): Promise<void> {
  const steps = screen(device.page, "run").locator("nav button");
  for (let index = 0; index < (await steps.count()); index++) {
    await openTask(device, index);
    if ((await shownTask(device)) === task) return;
  }
  throw new Error(`the run on screen has no task ${task}`);
}

/** 🏷️ The id of the task on screen. */
export async function shownTask(device: Device): Promise<string> {
  return ((await screen(device.page, "task").getAttribute("data-presence-anchor")) ?? "").replace(/^task:/u, "");
}

/** 🧺️ The items of the task on screen, by id, in the order they are shown. */
export async function shownItems(device: Device): Promise<readonly string[]> {
  const task = screen(device.page, "task");
  const ids = await task.locator("[data-quiz-item]").evaluateAll((elements) => elements.map((element) => (element as HTMLElement).dataset.quizItem ?? ""));
  if (ids.length > 0) return ids;
  const slots = await task.locator('[data-quiz-drop^="slot:"]').evaluateAll((elements) => elements.map((element) => ((element as HTMLElement).dataset.quizDrop ?? "").split(":")[2] ?? ""));
  return [...new Set(slots)];
}

/** 📐️ Where every item of the task on screen lies, by item id: its box in whole viewport pixels (x, y, width, height). */
export async function itemBoxes(device: Device): Promise<Readonly<Record<string, readonly number[]>>> {
  return screen(device.page, "task")
    .locator("[data-quiz-item]")
    .evaluateAll((elements) =>
      Object.fromEntries(
        elements.map((element) => {
          const box = element.getBoundingClientRect();
          return [(element as HTMLElement).dataset.quizItem ?? "", [box.x, box.y, box.width, box.height].map(Math.round)];
        }),
      ),
    );
}

/** 🗂️ Puts `item` of the classification on screen into `category`. */
export async function classify(device: Device, item: string, category: string): Promise<void> {
  const choice = screen(device.page, "task").locator(`[data-quiz-item="${item}"] select`);
  await choice.selectOption(category);
  await expect(choice).toHaveValue(category);
}

/** 🤏️ Drags `item` of the classification on screen by its grip, with the mouse, into the bin of `category`, the way a
 * hand does it: it takes the grip where it is, carries a ghost that shows what the item shows, and follows the bin
 * until it lights up under the pointer, then lets go. */
export async function dragInto(device: Device, item: string, category: string): Promise<void> {
  const task = screen(device.page, "task");
  const chip = task.locator(`[data-quiz-item="${item}"]`);
  const bin = task.locator(`[data-quiz-drop="category:${category}"]`);
  const held = await chip.locator("select").inputValue();
  await chip.locator("[data-quiz-grip]").hover();
  await device.page.mouse.down();
  await expect(async () => {
    const [target, view] = [await boxOf(bin), await boxOf(device.page.locator("#quiz-main"))];
    const [top, bottom] = [Math.max(target.y, view.y), Math.min(target.y + target.height, view.y + view.height)];
    await device.page.mouse.move(target.x + target.width / 2, (top + bottom) / 2, { steps: 5 });
    await expect(device.page.locator(".quiz-drag-ghost select")).toHaveValue(held, { timeout: 1_000 });
    await expect(bin).toHaveClass(/(^| )quiz-drop-active( |$)/u, { timeout: 1_000 });
  }).toPass({ timeout: 20_000 });
  await device.page.mouse.up();
  await expect(device.page.locator(".quiz-drag-ghost")).toHaveCount(0);
  await expect(bin.locator(`[data-quiz-item="${item}"] select`)).toHaveValue(category);
}

/** ↕️ Brings the sorting on screen into `order` (item ids, smallest first) with the move buttons. */
export async function sortInto(device: Device, order: readonly string[]): Promise<void> {
  const task = screen(device.page, "task");
  const shown = async (): Promise<readonly string[]> => task.locator("ol > [data-quiz-item]").evaluateAll((elements) => elements.map((element) => (element as HTMLElement).dataset.quizItem ?? ""));
  for (let place = 0; place < order.length; place++) {
    const item = order[place]!;
    for (let at = (await shown()).indexOf(item); at > place; at--) {
      await task.locator(`[data-quiz-item="${item}"] button`).first().click();
      await expect.poll(async () => (await shown()).indexOf(item)).toBe(at - 1);
    }
  }
  expect(await shown()).toEqual(order);
}

/** ↕️ Moves `item` of the sorting on screen one place down. */
export async function moveDown(device: Device, item: string): Promise<void> {
  await screen(device.page, "task").locator(`[data-quiz-item="${item}"] button`).nth(1).click();
}

/** 🏷️ How the site writes `value` of an unprefixed `quantity` in the device's language. */
export async function shownQuantity(device: Device, value: number, quantity: SourceQuantity): Promise<string> {
  if (quantity.prefixed) throw new Error(`matching by a prefixed quantity (${quantity.unit}) is not driven yet`);
  return device.page.evaluate(([amount, unit, locale]) => `${new Intl.NumberFormat(locale as string, { maximumFractionDigits: 6 }).format(amount as number)} ${unit}`, [value, quantity.unit, device.locale] as const);
}

/** 🃏️ Gives `item` of the matching on screen a card showing `label` in `dimension`: the one it holds already, else one
 * no item uses. */
export async function match(device: Device, dimension: string, item: string, label: string): Promise<void> {
  const choice = screen(device.page, "task").locator(`[data-quiz-drop="slot:${dimension}:${item}"] select`);
  const card = await choice.evaluate((select, wanted) => {
    const options = [...(select as HTMLSelectElement).options].filter((option) => option.value !== "" && option.text === wanted);
    return (options.find((option) => option.selected) ?? options.find((option) => !option.disabled))?.value;
  }, label);
  if (card === undefined) throw new Error(`no free card shows ${label} for ${item} in ${dimension}`);
  await choice.selectOption(card);
  await expect(choice).toHaveValue(card);
}

/** 🃏️ Takes the card of `item` of the matching on screen away again in `dimension`. */
export async function unmatch(device: Device, dimension: string, item: string): Promise<void> {
  const row = screen(device.page, "task").locator(`[data-quiz-drop="slot:${dimension}:${item}"]`);
  await row.getByRole("button").click();
  await expect(row.locator("select")).toHaveValue("");
}

/** ✍️ Answers the task on screen from the catalog's solution: `perfect` gives every item its true answer; `flawed` does
 * the same and then gets exactly one thing wrong (one item dragged into another category, two neighbours of different
 * value swapped, the cards of the smallest and the largest item exchanged), which costs some credit but never all. */
export async function answerTask(device: Device, quiz: SourceQuiz, how: "perfect" | "flawed"): Promise<void> {
  const task = taskOf(quiz, await shownTask(device));
  const items = await shownItems(device);
  expect(items.length).toBe(Math.min(task.draw ?? task.items.length, task.items.length));
  if (task.kind === "classification") {
    for (const item of items) await classify(device, item, itemOf(task, item).category!);
    if (how === "flawed") await dragInto(device, items[0]!, task.categories!.find((category) => category.id !== itemOf(task, items[0]!).category)!.id);
    return;
  }
  if (task.kind === "sorting") {
    const order = ascending(task, items);
    await sortInto(device, order);
    if (how === "flawed") await moveDown(device, order.find((item, place) => place + 1 < order.length && itemOf(task, item).value !== itemOf(task, order[place + 1]!).value)!);
    return;
  }
  for (const dimension of task.dimensions!) {
    const value = (item: string): number => itemOf(task, item).values![dimension.id]!;
    for (const item of items) await match(device, dimension.id, item, await shownQuantity(device, value(item), dimension.quantity));
    if (how === "flawed" && dimension === task.dimensions![0]) {
      const ranked = [...items].sort((left, right) => value(left) - value(right));
      const [smallest, largest] = [ranked[0]!, ranked[ranked.length - 1]!];
      expect(value(smallest)).toBeLessThan(value(largest));
      await unmatch(device, dimension.id, smallest);
      await unmatch(device, dimension.id, largest);
      await match(device, dimension.id, smallest, await shownQuantity(device, value(largest), dimension.quantity));
      await match(device, dimension.id, largest, await shownQuantity(device, value(smallest), dimension.quantity));
    }
  }
}

/** ✍️ Answers every task of the run on screen. */
export async function answerRun(device: Device, quiz: SourceQuiz, how: "perfect" | "flawed"): Promise<void> {
  const steps = screen(device.page, "run").locator("nav button");
  await expect(steps).toHaveCount(quiz.tasks.length);
  for (let index = 0; index < quiz.tasks.length; index++) {
    await openTask(device, index);
    await answerTask(device, quiz, how);
    await expect(steps.nth(index).locator("[data-complete]")).toBeVisible();
  }
}

/** 🙈️ Checks that the run on screen gives nothing of the solution away: no results, and none of the explanations the
 * catalog keeps for the items of `quiz`. */
export async function expectNothingRevealed(device: Device, quiz: SourceQuiz): Promise<void> {
  await expect(screen(device.page, "results")).toHaveCount(0);
  await expect(screen(device.page, "task-result")).toHaveCount(0);
  const text = await shownText(device);
  const revealed = quiz.tasks.flatMap((task) => task.items.filter((item) => text.includes(item.explanation[device.locale])).map((item) => `${task.id}/${item.id}`));
  expect(revealed, "explanations shown before the run was submitted").toEqual([]);
}

/** 📨️ Submits the run on screen as a whole — the submit action, then the confirmation — and waits for its results. */
export async function submitRun(device: Device): Promise<void> {
  const submit = primary(screen(device.page, "run"));
  await expect(submit).toBeEnabled();
  await submit.click();
  await primary(device.page.getByRole("alertdialog")).click();
  await expect(screen(device.page, "results")).toBeVisible();
}

/** 💯️ A percentage the site wrote (`87.5%`, `87,5 %`) as a number. */
export function percent(text: string): number {
  const found = /(\d+(?:[.,]\d+)?)\s*%/u.exec(text);
  if (found === null) throw new Error(`no percentage in ${JSON.stringify(text)}`);
  return Number(found[1]!.replace(",", "."));
}

/** 🏁️ What the results on screen say: the score of the run in percent, and per task its score and the rows of its result
 * tables (the item a row is about, by its label, and everything the row says) — not those of the figures of what
 * everyone answered, which are tables too. */
export async function shownResults(device: Device): Promise<{ readonly score: number; readonly tasks: readonly { readonly task: string; readonly score: number; readonly rows: readonly { readonly label: string; readonly text: string }[] }[] }> {
  const summary = screen(device.page, "results");
  const score = percent(await summary.locator("p").first().innerText());
  const cards = screen(device.page, "task-result");
  const tasks = [];
  for (let index = 0; index < (await cards.count()); index++) {
    const result = cards.nth(index);
    const rows = await result.locator("table:not(.quiz-plot) tbody tr").evaluateAll((elements) => elements.map((element) => ({ label: (element.querySelector<HTMLElement>("th[scope=row]")?.innerText ?? "").trim(), text: (element as HTMLElement).innerText })));
    tasks.push({ task: ((await result.getAttribute("data-presence-anchor")) ?? "").replace(/^result:/u, ""), score: percent(await result.locator("p").first().innerText()), rows });
  }
  return { score, tasks };
}

/** 🧾️ Checks the per-item feedback of the results on screen against the catalog: every task of the quiz has its card, every
 * drawn item its row (per dimension for a matching) with its label and its sourced explanation, and — where the answer
 * was `perfect` — a score of 100 %. Returns what the results say. */
export async function expectFeedback(device: Device, quiz: SourceQuiz, how: "perfect" | "flawed"): Promise<Awaited<ReturnType<typeof shownResults>>> {
  const results = await shownResults(device);
  expect(results.tasks.map((entry) => entry.task).sort()).toEqual(quiz.tasks.map((task) => task.id).sort());
  for (const entry of results.tasks) {
    const task = taskOf(quiz, entry.task);
    const drawn = Math.min(task.draw ?? task.items.length, task.items.length);
    expect(entry.rows.length, `${task.id}: one row per drawn item${task.kind === "matching" ? " and dimension" : ""}`).toBe(drawn * (task.dimensions?.length ?? 1));
    for (const row of entry.rows) {
      const item = task.items.find((candidate) => candidate.label[device.locale] === row.label);
      expect(item, `${task.id}: the row of ${JSON.stringify(row.label)} is about an item of the task`).toBeDefined();
      expect(row.text, `${task.id}: the row of ${item!.id} explains it`).toContain(item!.explanation[device.locale]);
    }
    if (how === "perfect") expect(entry.score, `${task.id}: a perfect answer scores 100 %`).toBe(100);
    else expect(entry.score, `${task.id}: one mistake costs some credit, not all`).toBeGreaterThan(0);
  }
  return results;
}

/** 🏠️ Goes back to the overview from a run or its results, by the navbar. */
export async function goHome(device: Device): Promise<void> {
  await way(device.page, "overview").click();
  await expect(device.page.locator("[data-layered-overview]")).toBeVisible();
  await expect(device.page.locator("[data-layered-pane][data-opened]")).toHaveCount(0);
}

/** 📶️ The connection indicator of the navigation; its `data-tone` is `calm` (saved), `busy` (saving) or `alert`
 * (the proctor does not answer). */
export function connection(device: Device): Locator {
  return device.page.locator("header [data-tone]");
}

/** 💾️ Waits until the connection indicator says every answer is saved with the proctor. */
export async function expectSaved(device: Device): Promise<void> {
  await expect(connection(device)).toHaveAttribute("data-tone", "calm");
}

/** 🔢️ The first whole number in `text`. */
export function count(text: string): number {
  const found = /\d+/u.exec(text);
  if (found === null) throw new Error(`no number in ${JSON.stringify(text)}`);
  return Number(found[0]);
}

/** 📐️ The box of the one element `target` matches, in viewport pixels. */
export async function boxOf(target: Locator): Promise<{ readonly x: number; readonly y: number; readonly width: number; readonly height: number }> {
  const box = await target.boundingBox();
  if (box === null) throw new Error("the element has no box");
  return box;
}

/** 🙅️ The text of the screen in front. */
export async function shownText(device: Device): Promise<string> {
  return device.page.locator("#quiz-main").innerText();
}

/** 🕳️ Every label the client failed to resolve on the screen in front: a raw key (`quiz.run.submit`) or an unfilled
 * placeholder (`{{count}}`). */
export async function unresolvedLabels(device: Device): Promise<readonly string[]> {
  const text = await device.page.locator("body").evaluate((body) => [(body as HTMLElement).innerText, ...[...body.querySelectorAll("[aria-label], [title]")].flatMap((element) => [element.getAttribute("aria-label") ?? "", element.getAttribute("title") ?? ""])].join("\n"));
  return [...text.matchAll(/\bquiz\.[a-z][A-Za-z]*\.[a-z][A-Za-z.]*|\{\{[^}]*\}\}/gu)].map((found) => found[0]);
}
//#endregion 🚶️Journey
