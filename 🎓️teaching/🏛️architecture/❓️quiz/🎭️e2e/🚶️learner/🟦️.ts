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
  readonly short?: Text;
  readonly unit: string;
  readonly prefixed: boolean;
  readonly additive: boolean;
}

/** 🧩️ An item of a task as authored, with its solution: the category it belongs to, its value, or its value per
 * dimension. */
export interface SourceItem {
  readonly id: string;
  readonly label: Text;
  readonly short?: Text;
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
  readonly categories?: readonly { readonly id: string; readonly label: Text; readonly short?: Text; readonly description?: Text; readonly profile?: Readonly<Record<string, number>> }[];
  readonly axes?: readonly { readonly id: string; readonly label: Text; readonly short?: Text; readonly unit?: string }[];
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

/** ⛰️ The challenge a run is played at, from the least to the most demanding. */
export type Challenge = "easy" | "medium" | "hard" | "expert";

/** ⛰️ Every challenge in rising order. */
export const CHALLENGES: readonly Challenge[] = ["easy", "medium", "hard", "expert"];

/** 💰️ The points a perfect run earns at each challenge (design §1 of the challenge levels). */
export const PAR: Readonly<Record<Challenge, number>> = { easy: 100, medium: 200, hard: 300, expert: 400 };

/** 🔑️ Whether a challenge shows the keys — the values to sort by and the cards to match — or asks to guess them. */
export const KEYS_SHOWN: Readonly<Record<Challenge, boolean>> = { easy: true, medium: true, hard: false, expert: false };

/** 🏷️ The name of every challenge as the site writes it. */
export const CHALLENGE_NAMES: Readonly<Record<Challenge, Text>> = {
  easy: { en: "Easy", de: "Leicht" },
  medium: { en: "Medium", de: "Mittel" },
  hard: { en: "Hard", de: "Schwer" },
  expert: { en: "Expert", de: "Experte" },
};

/** 🏅️ A badge as authored. */
export interface SourceBadge {
  readonly id: string;
  readonly label: Text;
  readonly rule: { readonly kind: string; readonly quiz?: string; readonly taskKind?: string; readonly challenge?: Challenge };
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

/** 🏅️ The badges a learner holds who has played exactly `perfect` (quiz ids) once each, without a mistake, at
 * `challenge`: a badge for perfection counts only from the least challenge its rule names. */
export function badgesFor(perfect: readonly string[], challenge: Challenge = "medium"): readonly SourceBadge[] {
  const played = new Set(perfect);
  return CATALOG.badges.filter((badge) => {
    if (badge.rule.kind === "completed-quizzes") return QUIZZES.every((quiz) => played.has(quiz.id));
    if (badge.rule.challenge !== undefined && CHALLENGES.indexOf(challenge) < CHALLENGES.indexOf(badge.rule.challenge)) return false;
    if (badge.rule.kind === "perfect-quiz") return played.has(badge.rule.quiz!);
    const asked = QUIZZES.filter((quiz) => badge.rule.quiz === undefined || quiz.id === badge.rule.quiz).flatMap((quiz) => quiz.tasks.filter((task) => badge.rule.taskKind === undefined || task.kind === badge.rule.taskKind).map(() => quiz.id));
    return asked.length > 0 && asked.every((quiz) => played.has(quiz));
  });
}

/** 📝️ The task `id` of `quiz` as authored. */
export function taskOf(quiz: SourceQuiz, id: string): SourceTask {
  const task = quiz.tasks.find((candidate) => candidate.id === id);
  if (task === undefined) throw new Error(`quiz ${quiz.id} has no task ${id}`);
  return task;
}

/** 🧩️ The item `id` of `task` as authored. */
export function itemOf(task: SourceTask, id: string): SourceItem {
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

/** 🔎️ Watches `page` for problems. A socket the page leaves while it still connects is closed only once it opened, so
 * Chromium's "WebSocket is closed before the connection is established" is a problem like any other socket error. */
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
  page.on("websocket", (socket) => socket.on("socketerror", (error) => report(`socket error: ${socket.url()} ${error}`)));
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

/** ⛰️ The challenge the device remembers for the next run of `quiz`: the one last chosen on its page, at first `medium`. */
export async function rememberedChallenge(device: Device, quiz: string): Promise<Challenge> {
  return device.page.evaluate(([key, id]) => (JSON.parse(localStorage.getItem(key) ?? "null") as { challenges?: Record<string, Challenge> } | null)?.challenges?.[id] ?? "medium", [`semio.quiz.${CATALOG.id}.preferences`, quiz] as const);
}

/** ⛰️ Opens the page of `quiz` and chooses `challenge` in its chooser; the device remembers it for the next run. */
export async function chooseChallenge(device: Device, quiz: string, challenge: Challenge): Promise<void> {
  await device.page.evaluate((id) => (window.location.hash = id), quiz);
  const page = pane(device.page, quiz);
  await expect(page).toHaveAttribute("data-opened", "");
  const choice = page.locator(`input[type="radio"][value="${challenge}"]`);
  await choice.check();
  await expect(choice).toBeChecked();
  await expect.poll(() => rememberedChallenge(device, quiz)).toBe(challenge);
}

/** ▶️ Plays `quiz` at `challenge` and waits for the run, which names its challenge. At the challenge the device remembers,
 * the main action of the quiz's card (start, resume or start again); at another one, the quiz's page: the challenge
 * chosen there, then its main action — discarding a run still open at another challenge when the page asks. */
export async function playQuiz(device: Device, quiz: string, challenge: Challenge = "medium"): Promise<void> {
  if ((await rememberedChallenge(device, quiz)) === challenge) await primary(card(device.page, quiz)).click();
  else {
    await chooseChallenge(device, quiz, challenge);
    await primary(pane(device.page, quiz).locator('[data-card="quiz"]')).click();
    const discard = device.page.getByRole("alertdialog");
    await expect(screen(device.page, "run").or(discard)).toBeVisible();
    if (await discard.isVisible()) await primary(discard).click();
  }
  await expect(screen(device.page, "run")).toBeVisible();
  await expect(screen(device.page, "run")).toContainText(CHALLENGE_NAMES[challenge][device.locale]);
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
  const slots = await task.locator('.quiz-slot[data-presence-anchor^="item:"]').evaluateAll((elements) => elements.map((element) => ((element as HTMLElement).dataset.presenceAnchor ?? "").replace(/^item:/u, "")));
  return [...new Set(slots)];
}

/** ⏱️ Starts the clock of the timed task on screen and waits until the task shows; a task without a clock, or whose
 * clock runs already, is left as it is. */
export async function startClock(device: Device): Promise<void> {
  const task = screen(device.page, "task");
  const closed = task.locator('[data-clock="closed"]');
  if ((await closed.count()) === 0) return;
  await closed.getByRole("button").first().click();
  await expect(task.locator('[data-clock]:not([data-clock="closed"])')).toBeVisible();
}

/** 🚦️ The stage of the clock of the task on screen (`closed`, `running`, `thirty`, `ten`, `up`), or `undefined` on a task
 * without one. */
export async function clockOf(device: Device): Promise<string | undefined> {
  const clock = screen(device.page, "task").locator("[data-clock]");
  return (await clock.count()) === 0 ? undefined : ((await clock.first().getAttribute("data-clock")) ?? undefined);
}

/** ⌨️ The text a learner types for `value` in the base `unit`: a whole mantissa with an exponent (`3828e23 W`), which
 * reads the same in every language and needs no SI prefix. */
export function typedValue(value: number, unit: string): string {
  const [mantissa = "", exponent = "0"] = value.toExponential().split("e");
  const [whole = "", fraction = ""] = mantissa.split(".");
  return `${whole}${fraction}e${Number(exponent) - fraction.length} ${unit}`;
}

async function typeGuess(field: Locator, value: number, unit: string): Promise<void> {
  const typed = typedValue(value, unit);
  await field.fill(typed);
  await field.press("Enter");
  await expect(field).not.toHaveValue(typed);
  await expect(field).toHaveAttribute("aria-invalid", "false");
}

/** ⌨️ Types `value` (in the base unit of `quantity`) as the guess for `item` of the sorting on screen that hides its keys,
 * and commits it. */
export async function guessSorting(device: Device, item: string, value: number, quantity: SourceQuantity): Promise<void> {
  await typeGuess(screen(device.page, "task").locator(`[data-quiz-item="${item}"] input`), value, quantity.unit);
}

/** ⌨️ Types `value` (in the base unit of `quantity`) as the guess for `item` in the dimension at `dimension` (its place
 * in the task) of the matching on screen that hides its cards, and commits it. */
export async function guessMatching(device: Device, dimension: number, item: string, value: number, quantity: SourceQuantity): Promise<void> {
  await typeGuess(screen(device.page, "task").locator("section.quiz-match").nth(dimension).locator(`.quiz-slot[data-presence-anchor="item:${item}"] input`), value, quantity.unit);
}

/** 🔑️ Whether the task on screen hides its keys: a sorting or matching that asks for typed guesses. */
export async function keysHidden(device: Device): Promise<boolean> {
  const task = screen(device.page, "task");
  return (await task.locator('[data-quiz-item] input, section[data-keys="hidden"]').count()) > 0;
}

/** 🙋️ A hint as shown: its kind (`compare`, `profile`, `group` or `category`) and the question it asks. */
export interface ShownHint {
  readonly kind: string;
  readonly question: string;
}

/** 💡️ The hints shown in the task on screen, by the item each stands beside (`""` for one beside no item; a matching
 * shows one per item and dimension, so ask it while a single dimension carries hints). */
export async function shownHints(device: Device): Promise<Readonly<Record<string, ShownHint>>> {
  return screen(device.page, "task")
    .locator("[data-hint]")
    .evaluateAll((elements) =>
      Object.fromEntries(
        elements.map((element) => {
          const anchor = element.closest<HTMLElement>("[data-quiz-item], [data-presence-anchor^='item:']");
          const question = element.querySelector<HTMLElement>(".quiz-hint-question")?.innerText ?? "";
          return [anchor?.dataset.quizItem ?? (anchor?.dataset.presenceAnchor ?? "").replace(/^item:/u, ""), { kind: (element as HTMLElement).dataset.hint ?? "", question }];
        }),
      ),
    );
}

/** 🤨️ How every hint question opens (design §8.4 of the challenge levels): it asks, never states. */
export const HINT_OPENING: Readonly<Record<Locale, string>> = { en: "Are you sure ", de: "Bist du sicher, dass " };

/** 🚫️ The wording of the general hints the specific questions replaced: a bare direction ("far too", "viel zu") or a
 * count of misplaced items ("wrong category", "Kategorie: 1"). */
export const GENERIC_HINT = /far too|wrong category|viel zu|Kategorie: \d/iu;

/** 🖋️ A label in the quotation marks of `locale`, as the hint questions quote items. */
export function quoted(label: string, locale: Locale): string {
  return locale === "en" ? `“${label}”` : `„${label}“`;
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

/** ↕️ The items of the sorting on screen, by id, from the smallest place to the largest. */
export async function sortedItems(device: Device): Promise<readonly string[]> {
  return screen(device.page, "task")
    .locator("ol > [data-quiz-item]")
    .evaluateAll((elements) => elements.map((element) => (element as HTMLElement).dataset.quizItem ?? ""));
}

/** 🫴️ The action of a sorting that keeps the order it was dealt. */
const KEEP_ORDER: Text = { en: "Keep this order", de: "Reihenfolge übernehmen" };

/** ↕️ Brings the sorting on screen into `order` (item ids, smallest first) with the move buttons — on the ladder of
 * keys where the challenge shows them. An order the sheet dealt already is kept with the sorting's own "keep" action,
 * so the task counts as answered either way. */
export async function sortInto(device: Device, order: readonly string[]): Promise<void> {
  const task = screen(device.page, "task");
  let moved = false;
  for (let place = 0; place < order.length; place++) {
    const item = order[place]!;
    for (let at = (await sortedItems(device)).indexOf(item); at > place; at--) {
      await task.locator(`[data-quiz-item="${item}"] .quiz-sort-up`).click();
      await expect.poll(async () => (await sortedItems(device)).indexOf(item)).toBe(at - 1);
      moved = true;
    }
  }
  const keep = task.getByRole("button", { name: KEEP_ORDER[device.locale], exact: true });
  if (!moved && (await keep.count()) > 0) await keep.click();
  expect(await sortedItems(device)).toEqual(order);
}

/** ↕️ Moves `item` of the sorting on screen one place down. */
export async function moveDown(device: Device, item: string): Promise<void> {
  await screen(device.page, "task").locator(`[data-quiz-item="${item}"] .quiz-sort-down`).click();
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

/** 🎯️ How far off a flawed guess lies: ten thousand times the true value, beyond the reach of every set (a factor of
 * 1000 at most). */
export const FAR_OFF = 1e4;

/** ✍️ Answers the task on screen from the catalog's solution, as its challenge asks — on a timed task the clock is
 * started first: `perfect` gives every item its true answer (the place of its key, its card, or its true value typed as
 * the guess where the keys are hidden); `flawed` does the same and then gets exactly one thing wrong (one item dragged
 * into another category, two neighbours of different value swapped, the cards of the smallest and the largest item
 * exchanged, or one guess typed far off), which costs some credit but never all. */
export async function answerTask(device: Device, quiz: SourceQuiz, how: "perfect" | "flawed"): Promise<void> {
  await startClock(device);
  const task = taskOf(quiz, await shownTask(device));
  const items = await shownItems(device);
  expect(items.length).toBe(Math.min(task.draw ?? task.items.length, task.items.length));
  if (task.kind === "classification") {
    for (const item of items) await classify(device, item, itemOf(task, item).category!);
    if (how === "flawed") await dragInto(device, items[0]!, task.categories!.find((category) => category.id !== itemOf(task, items[0]!).category)!.id);
    return;
  }
  const hidden = await keysHidden(device);
  if (task.kind === "sorting") {
    const order = ascending(task, items);
    if (hidden) {
      for (const item of items) await guessSorting(device, item, itemOf(task, item).value!, task.quantity!);
      await expect.poll(() => sortedItems(device)).toEqual(order);
      if (how === "flawed") await guessSorting(device, order[0]!, itemOf(task, order[0]!).value! * FAR_OFF, task.quantity!);
      return;
    }
    await sortInto(device, order);
    if (how === "flawed") await moveDown(device, order.find((item, place) => place + 1 < order.length && itemOf(task, item).value !== itemOf(task, order[place + 1]!).value)!);
    return;
  }
  if (hidden) {
    for (const [index, dimension] of task.dimensions!.entries()) for (const item of items) await guessMatching(device, index, item, itemOf(task, item).values![dimension.id]!, dimension.quantity);
    if (how === "flawed") await guessMatching(device, 0, items[0]!, itemOf(task, items[0]!).values![task.dimensions![0]!.id]! * FAR_OFF, task.dimensions![0]!.quantity);
    return;
  }
  for (const dimension of task.dimensions!) {
    for (const item of items) await match(device, dimension.id, item, await shownQuantity(device, itemOf(task, item).values![dimension.id]!, dimension.quantity));
    if (how === "flawed" && dimension === task.dimensions![0]) await swapExtremes(device, task, 0, items);
  }
}

/** 🔀️ Exchanges the cards of the smallest and the largest of `items` in the dimension at `dimension` of the matching on
 * screen, whose every item holds its true card; returns the two, smallest first. */
export async function swapExtremes(device: Device, task: SourceTask, dimension: number, items: readonly string[]): Promise<readonly [string, string]> {
  const { id, quantity } = task.dimensions![dimension]!;
  const value = (item: string): number => itemOf(task, item).values![id]!;
  const ranked = [...items].sort((left, right) => value(left) - value(right));
  const [smallest, largest] = [ranked[0]!, ranked[ranked.length - 1]!];
  expect(value(smallest)).toBeLessThan(value(largest));
  await unmatch(device, id, smallest);
  await unmatch(device, id, largest);
  await match(device, id, smallest, await shownQuantity(device, value(largest), quantity));
  await match(device, id, largest, await shownQuantity(device, value(smallest), quantity));
  return [smallest, largest];
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

/** 🔢️ The amounts the site wrote in `text` (`261`, `87.5`, `87,5`), in order; amounts below 1000 never group. */
export function amounts(text: string): readonly number[] {
  return [...text.matchAll(/\d+(?:[.,]\d+)?/gu)].map((found) => Number(found[0].replace(",", ".")));
}

/** ⛰️ The challenge `text` names in the device's language. */
export function namedChallenge(device: Device, text: string): Challenge {
  const named = CHALLENGES.filter((challenge) => new RegExp(`(^|[^\\p{L}])${CHALLENGE_NAMES[challenge][device.locale]}([^\\p{L}]|$)`, "u").test(text));
  if (named.length !== 1) throw new Error(`no single challenge in ${JSON.stringify(text)}`);
  return named[0]!;
}

/** 💰️ Points as the site writes them, "Medium · Points: 174.5 of 200": the challenge, the points earned and the most. */
export function scoredPoints(device: Device, text: string): { readonly challenge: Challenge; readonly points: number; readonly par: number } {
  const found = amounts(text);
  if (found.length < 2) throw new Error(`no points in ${JSON.stringify(text)}`);
  return { challenge: namedChallenge(device, text), points: found[found.length - 2]!, par: found[found.length - 1]! };
}

/** 🏁️ What the results on screen say: the score of the run in percent, its challenge with the points it earned of the
 * most, and per task its score and the rows of its result tables (the item a row is about, by its label without its icon, everything the
 * row says, and whether it marks a guess as far off) — not those of the figures of what everyone answered, which are
 * tables too. */
export async function shownResults(device: Device): Promise<{ readonly score: number; readonly challenge: Challenge; readonly points: number; readonly par: number; readonly tasks: readonly { readonly task: string; readonly score: number; readonly rows: readonly { readonly label: string; readonly text: string; readonly miss: boolean }[] }[] }> {
  const summary = screen(device.page, "results");
  const score = percent(await summary.locator("p").first().innerText());
  const scored = scoredPoints(device, await summary.locator("p").nth(1).innerText());
  const cards = screen(device.page, "task-result");
  const tasks = [];
  for (let index = 0; index < (await cards.count()); index++) {
    const result = cards.nth(index);
    const rows = await result.locator("table:not(.quiz-plot) tbody tr").evaluateAll((elements) => elements.map((element) => ({ label: (element.querySelector<HTMLElement>("th[scope=row]")?.innerText ?? "").trim(), text: (element as HTMLElement).innerText, miss: element.querySelector("[data-miss]") !== null })));
    tasks.push({ task: ((await result.getAttribute("data-presence-anchor")) ?? "").replace(/^result:/u, ""), score: percent(await result.locator("p").first().innerText()), rows: rows.map((row) => ({ ...row, label: row.label.replace(/^[^\p{L}\p{N}]+/u, "") })) });
  }
  return { score, ...scored, tasks };
}

/** 🏅️ The best run of `quiz` its card shows: its challenge and its points of the most that challenge gives. */
export async function shownBest(device: Device, quiz: string): Promise<{ readonly challenge: Challenge; readonly points: number; readonly par: number }> {
  return scoredPoints(device, await card(device.page, quiz).locator("ul > li").nth(1).innerText());
}

/** 🧾️ Checks the per-item feedback of the results on screen against the catalog: the run was played at `challenge` and
 * earned its score times the points of that challenge, every task of the quiz has its card, every drawn item its row
 * (per dimension for a matching) with its label and its sourced explanation, and — where the answer was `perfect` — a
 * score of 100 % and every point of the challenge. Returns what the results say. */
export async function expectFeedback(device: Device, quiz: SourceQuiz, how: "perfect" | "flawed", challenge: Challenge = "medium"): Promise<Awaited<ReturnType<typeof shownResults>>> {
  const results = await shownResults(device);
  expect(results.challenge).toBe(challenge);
  expect(results.par).toBe(PAR[challenge]);
  expect(results.points, "points are the score times the most points of the challenge").toBeCloseTo((results.score / 100) * PAR[challenge], 0);
  if (how === "perfect") expect(results.points).toBe(PAR[challenge]);
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
