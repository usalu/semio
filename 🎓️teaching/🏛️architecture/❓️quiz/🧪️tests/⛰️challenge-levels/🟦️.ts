/** ⛰️ The physics quiz at each of the four challenges. Easy shows the keys and, beside a value placed far off, asks
 * whether the relation the learner's keys claim between it and another item is meant ("Are you sure 180 million ×
 * “Car engine at full throttle” together only add up to the power of 1 × “The Sun”?"), until it is fixed; a perfect run earns 100
 * points and no badge,
 * since every badge for perfection asks for medium or harder. Medium shows the keys without hints, and a perfect run
 * earns 200 points and the physics badges. Hard hides the keys: the true values typed as guesses earn 300 points, and
 * one guess far off costs score and is marked as a miss in the results while the best run stays. Expert puts every task
 * behind its own clock: a task shows only its title, the time it allows and the start of its clock until it is
 * started, takes no more answers once the page's clock has passed its deadline, and a run submitted with tasks left
 * open scores what was answered, of 400 points.
 *
 * The hints are asserted by their structure where the arrangement decides it (design §8.4a), not sentence by sentence:
 * beside the item each question stands, naming items, quantities and axes by their short label where they have one; a
 * pair exchanged across all others is asked for its order alone, without a number, the one the keys make larger first
 * ("Are you sure “Person sitting still” is higher in power than “The Sun”?"), against one item placed right; powers placed the
 * true way round but too close together are asked how many of an item placed right "together only add up to" the one,
 * or "it takes" to add up to the other, the count without long digit runs; every compare question names its quantity,
 * so a matching of several names the one it asks about ("in puncto Heizlast"); a misplaced standard is asked whether it lies above or below the standard
 * placed right that lies between on the axis its profile questions, or whether it fits that profile; a sorting placed
 * upside down shows only three questions; nothing general ("far too", "wrong category") is said anywhere. The exact
 * sentences and numbers are pinned by the core's shared vectors and the React tests. The other kinds and situations:
 * on easy a matching asks beside both items whose cards were exchanged, and a classification with spider profiles
 * asks beside the misplaced item about the profile it was put into; on hard a matching takes typed guesses
 * and its results mark a miss under the line that says how far a guess may lie, and the spider diagrams show their
 * shape only. Expert scores a sorting and a matching answered in time fully and a task whose time ran out not at all.
 * German speaks German in the chooser, every kind of hint and the results. Choosing another challenge for a quiz with
 * an open run asks before discarding it, and is remembered for that quiz alone. A reload in the middle of an expert run
 * keeps its clock running from the instant the task was opened. Other specs play on the same proctor at the same time,
 * so everything here is asserted about this spec's own learners.
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../🔣️.json — the catalog, whose badges ask for medium or harder
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/⛰️challenge/🟦️.ts — the rules of the challenges and the hints
 * @see ../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-CHALLENGE-LEVELS/📓️design.md — §8, the specific hints
 * https://playwright.dev/docs/clock */
import {
  CHALLENGES,
  CHALLENGE_NAMES,
  FAR_OFF,
  GENERIC_HINT,
  HINT_OPENING,
  PAR,
  answerRun,
  answerTask,
  ascending,
  badgesFor,
  card,
  chooseChallenge,
  classify,
  clockOf,
  enter,
  expect,
  expectFeedback,
  goHome,
  guessMatching,
  guessSorting,
  handle,
  itemOf,
  keysHidden,
  match,
  openTask,
  openTaskById,
  pane,
  playQuiz,
  primary,
  quizOf,
  quoted,
  rememberedChallenge,
  screen,
  shownBest,
  shownHints,
  shownItems,
  shownQuantity,
  shownResults,
  shownTask,
  sortInto,
  startClock,
  submitRun,
  swapExtremes,
  taskOf,
  test,
  unmatch,
  unresolvedLabels,
  type Device,
  type Locale,
  type SourceQuantity,
  type SourceTask,
  type Text,
} from "../../🎭️e2e/🚶️learner/🟦️.ts";

const physics = quizOf("physics");
const powers = taskOf(physics, "powers");
const energies = taskOf(physics, "energies");
const heating = quizOf("heating");
const uValues = taskOf(heating, "u-values");
const loads = taskOf(heating, "heating-load-and-demand");
const demand = quizOf("demand");
const profiles = taskOf(demand, "standard-profiles");

/** ⏲️ The whole seconds a clock text shows first (`2:30` → 150). */
function seconds(text: string): number {
  const found = /(\d+):(\d\d)/u.exec(text);
  if (found === null) throw new Error(`no m:ss in ${JSON.stringify(text)}`);
  return Number(found[1]) * 60 + Number(found[2]);
}

/** ⏲️ The seconds the clock of the task on screen shows: allowed while closed, left while it runs. */
async function clockSeconds(device: Device): Promise<number> {
  return seconds(await screen(device.page, "task").locator("[data-clock]").first().innerText());
}

/** 🕸️ The text of every spider diagram of the task on screen, with its folded value table, and the columns of that table. */
async function diagrams(device: Device): Promise<readonly { readonly text: string; readonly columns: number }[]> {
  return screen(device.page, "task")
    .locator("figure.quiz-radar")
    .evaluateAll((figures) => figures.map((figure) => ({ text: figure.textContent ?? "", columns: figure.querySelectorAll("thead th").length })));
}

/** 🎯️ The result card of `task` on screen. */
function resultOf(device: Device, task: string): ReturnType<typeof screen> {
  return screen(device.page, "task-result").and(device.page.locator(`[data-presence-anchor="result:${task}"]`));
}

/** 🏅️ The labels of the badges the results on screen say the run earned. */
async function earnedLabels(device: Device): Promise<readonly string[]> {
  return screen(device.page, "results").locator("[data-earned] h3").allInnerTexts();
}

/** 🪶️ The relative slack of the core's reach. */
const SLACK = 1e-9;

/** 🧢️ The most hints a task gives at once. */
const HINTS_PER_TASK = 3;

/** 🔭️ The reach of a set of true values on a logarithmic scale, as the challenge core takes it: the square root of their
 * spread, at most a factor of 1000; a key farther off its value than this (widened by {@link SLACK}) is questioned. */
function logReach(values: readonly number[]): number {
  return Math.min(Math.sqrt(Math.max(...values) / Math.min(...values)), 1000);
}

/** 🏷️ How every hint names `named` in `locale`: by its short label where it has one, else by its label (design §8.4a.4). */
function nameOf(named: { readonly label: Text; readonly short?: Text }, locale: Locale): string {
  return (named.short ?? named.label)[locale];
}

/** 🖋️ How a hint quotes `id` of `task` in `locale`: its {@link nameOf} in the language's quotation marks. */
function cited(task: SourceTask, id: string, locale: Locale): string {
  return quoted(nameOf(itemOf(task, id), locale), locale);
}

/** ✔️ An assertion about the question shown beside one item. */
type Asks = (question: string) => void;

/** ⚓️ The one of `anchors` (items holding their own keys) the question quotes beside its own item — its reference. */
function referenceIn(task: SourceTask, anchors: readonly string[], question: string, locale: Locale): string {
  const named = anchors.filter((anchor) => question.includes(cited(task, anchor, locale)));
  expect(named, `${JSON.stringify(question)} compares with one item placed right`).toHaveLength(1);
  return named[0]!;
}

/** ✂️ The question without the item names it quotes, so what remains is the hint's own wording and numbers. */
function unquoted(task: SourceTask, question: string, locale: Locale): string {
  return task.items.reduce((rest, item) => rest.replaceAll(cited(task, item.id, locale), "“”"), question);
}

/** 🔢️ A count stays brief (design §8.4a.3): never a run of four or more digits outside the names the question quotes. */
function briefCounts(task: SourceTask, question: string, locale: Locale): void {
  expect(unquoted(task, question, locale), `${JSON.stringify(question)} writes its count briefly`).not.toMatch(/\d{4,}/u);
}

/** 📏️ Every compare question names the quantity it is about (design §8.4b.1; in German "in puncto"); the case of the
 * name is the client's to choose. */
function namesQuantity(question: string, locale: Locale, quantity: string): void {
  expect(question.toLowerCase(), `${JSON.stringify(question)} names ${quantity}`).toContain(`${locale === "en" ? "" : "in puncto "}${quantity}`.toLowerCase());
}

/** 🔃️ The question of a reversed claim (design §8.4a.1) beside `item`: whether the one the learner's keys make the larger
 * (`item` itself where `itemLarger`, else its reference among `anchors`) is larger than the other in `quantity` — by
 * the order alone, without a number. */
function orderAsks(task: SourceTask, item: string, itemLarger: boolean, anchors: readonly string[], locale: Locale, quantity: string): Asks {
  return (question) => {
    expect(question).toMatch(locale === "en" ? / is (?:larger|higher)\b.* than /u : / (?:größer|höher) ist als /u);
    const [self, other] = [cited(task, item, locale), cited(task, referenceIn(task, anchors, question, locale), locale)];
    const [first, second] = itemLarger ? [self, other] : [other, self];
    expect(question.indexOf(first) < question.indexOf(second), `${JSON.stringify(question)} names ${first} as the larger`).toBe(true);
    expect(unquoted(task, question, locale), `${JSON.stringify(question)} asks for the order alone`).not.toMatch(/\d/u);
    namesQuantity(question, locale, quantity);
  };
}

/** ➕️ The question of a claim about amounts that add up, pointing the true way, beside `large`: `under`, whether so many
 * of a reference among `anchors` "together only add up to" one `large`; `over`, whether "it takes" so many; the count
 * brief ({@link briefCounts}), the quantity named. */
function adds(task: SourceTask, verdict: "under" | "over", large: string, anchors: readonly string[], locale: Locale, quantity: string): Asks {
  return (question) => {
    expect(question).toMatch({ en: { under: / together only add up to /u, over: / it takes /u }, de: { under: / zusammen nur /u, over: / braucht, um /u } }[locale][verdict]);
    const one = `1 × ${cited(task, large, locale)}`;
    const few = `× ${cited(task, referenceIn(task, anchors, question, locale), locale)}`;
    expect(question.includes(one) && question.includes(few) && question.indexOf(few) < question.indexOf(one), `${JSON.stringify(question)} asks how many ${few} make ${one}`).toBe(true);
    briefCounts(task, question, locale);
    namesQuantity(question, locale, quantity);
  };
}

/** 🔀️ The questions beside `low` and `high` of `task` when they hold each other's keys and every one of `anchors` its
 * own: each claim points the wrong way, so each is asked for its order in `quantity` ({@link orderAsks}). */
function exchanged(task: SourceTask, low: string, high: string, anchors: readonly string[], locale: Locale, quantity: string): Readonly<Record<string, Asks>> {
  return { [high]: orderAsks(task, high, false, anchors, locale, quantity), [low]: orderAsks(task, low, true, anchors, locale, quantity) };
}

/** 💬️ Waits until the task on screen shows hints of `kind` beside exactly the items of `expected`, no more than
 * {@link HINTS_PER_TASK}, each a question in the learner's language that opens as one, names its own item as
 * {@link cited}, and passes its assertion; nothing general is said anywhere in the task. The numbers are left to the core
 * and client tests, which pin them over shared vectors. */
async function expectQuestions(device: Device, task: SourceTask, kind: string, expected: Readonly<Record<string, Asks>>): Promise<void> {
  const opening = HINT_OPENING[device.locale];
  await expect(async () => {
    const shown = await shownHints(device);
    expect(Object.keys(shown).sort(), "the items questioned").toEqual(Object.keys(expected).sort());
    expect(Object.keys(shown).length).toBeLessThanOrEqual(HINTS_PER_TASK);
    for (const [item, check] of Object.entries(expected)) {
      const { kind: shownKind, question } = shown[item]!;
      expect(shownKind).toBe(kind);
      expect(question.startsWith(opening), `${JSON.stringify(question)} opens with ${JSON.stringify(opening)}`).toBe(true);
      expect(question.includes(cited(task, item, device.locale)), `${JSON.stringify(question)} names ${item}`).toBe(true);
      check(question);
    }
    expect(await screen(device.page, "task").innerText()).not.toMatch(GENERIC_HINT);
  }).toPass({ timeout: 20_000 });
}

/** ↕️ Places the physics `powers` sorting on screen (true order `order`) wrongly twice and expects the questions, in the
 * learner's language. With the extremes exchanged every other item keeps its own key, so both extremes are asked for
 * their order ({@link exchanged}). With the two largest exchanged they are questioned when they lie farther apart than
 * the reach (always when the Sun is drawn); then every anchor lies below both, so both claims point the true way, the
 * largest's understated and the other's overstated, and powers add up: it is asked whether so many of the reference
 * "together only add up to" the largest, and whether "it takes" so many to add up to the other. Otherwise nothing is
 * asked. */
async function expectPowersQuestions(device: Device, order: readonly string[]): Promise<void> {
  expect(powers.quantity!.additive, "powers add up").toBe(true);
  const { locale } = device;
  const value = (id: string): number => itemOf(powers, id).value!;
  const n = order.length;
  const [smallest, next, largest] = [order[0]!, order[n - 2]!, order[n - 1]!];
  await sortInto(device, [largest, ...order.slice(1, -1), smallest]);
  const quantity = nameOf(powers.quantity!, locale);
  await expectQuestions(device, powers, "compare", exchanged(powers, smallest, largest, order.slice(1, -1), locale, quantity));
  await sortInto(device, [...order.slice(0, -2), largest, next]);
  const below = order.slice(0, -2);
  const far = value(largest) / value(next) > logReach(order.map(value)) * (1 + SLACK);
  await expectQuestions(device, powers, "compare", far ? { [largest]: adds(powers, "under", largest, below, locale, quantity), [next]: adds(powers, "over", next, below, locale, quantity) } : {});
}

/** 🧢️ Places the physics `powers` sorting on screen (true order `order`) upside down, so more items miss their keys than
 * a task questions at once, and expects questions beside exactly {@link HINTS_PER_TASK} of the items that miss. */
async function expectCappedPowers(device: Device, order: readonly string[]): Promise<void> {
  const value = (id: string): number => itemOf(powers, id).value!;
  const upsideDown = [...order].reverse();
  const reach = logReach(order.map(value)) * (1 + SLACK);
  const missed = upsideDown.filter((item, place) => Math.max(value(order[place]!) / value(item), value(item) / value(order[place]!)) > reach);
  expect(missed.length, "more items miss than a task questions").toBeGreaterThan(HINTS_PER_TASK);
  await sortInto(device, upsideDown);
  await expect(async () => {
    const shown = await shownHints(device);
    expect(Object.keys(shown)).toHaveLength(HINTS_PER_TASK);
    for (const [item, { kind, question }] of Object.entries(shown)) {
      expect(missed, `${item} misses its key`).toContain(item);
      expect(kind).toBe("compare");
      expect(question.startsWith(HINT_OPENING[device.locale]) && question.includes(cited(powers, item, device.locale)), `${JSON.stringify(question)} asks about ${item}`).toBe(true);
    }
  }).toPass({ timeout: 20_000 });
}

/** 🃏️ Matches every shown item of the matching `task` on screen to its true card in every quantity, exchanges the
 * cards of the extremes in the first and expects beside both the question for their order ({@link exchanged}), naming
 * that quantity where the task has several, in the learner's language; the true cards back, nothing is asked. */
async function expectExchangeQuestions(device: Device, task: SourceTask): Promise<void> {
  const [dimension] = task.dimensions!;
  expect(dimension!.quantity.additive, `${dimension!.id} does not add up`).toBe(false);
  const items = await shownItems(device);
  const fit = async ({ id, quantity }: { readonly id: string; readonly quantity: SourceQuantity }, item: string): Promise<void> => match(device, id, item, await shownQuantity(device, itemOf(task, item).values![id]!, quantity));
  for (const each of task.dimensions!) for (const item of items) await fit(each, item);
  await expect(screen(device.page, "task").locator("[data-hint]")).toHaveCount(0);
  const [smallest, largest] = await swapExtremes(device, task, 0, items);
  const quantity = nameOf(dimension!.quantity, device.locale);
  await expectQuestions(device, task, "compare", exchanged(task, smallest, largest, items.filter((item) => item !== smallest && item !== largest), device.locale, quantity));
  for (const item of [smallest, largest]) await unmatch(device, dimension!.id, item);
  for (const item of [smallest, largest]) await fit(dimension!, item);
  await expect.poll(() => shownHints(device)).toEqual({});
}

/** 🧭️ The profile of the demand `standard-profiles` task that questions `item` most surely, and the axis its hint names:
 * among the other categories, the one whose largest gap to the item's own profile on an axis, relative to that axis's
 * reach (half the spread of every category's value on it), is the largest; on ties the first category, and the first
 * axis in the task's order (design §8.3). */
function farthestProfile(item: string): { readonly category: string; readonly axis: string; readonly ratio: number } {
  const categories = profiles.categories!;
  const home = itemOf(profiles, item).category;
  const own = categories.find((category) => category.id === home)!.profile!;
  const reach = (axis: string): number => {
    const values = categories.map((category) => category.profile![axis]!);
    return (Math.max(...values) - Math.min(...values)) / 2;
  };
  let found = { category: "", axis: "", ratio: -Infinity };
  for (const category of categories) {
    if (category.id === home) continue;
    for (const axis of profiles.axes!) {
      const ratio = Math.abs(category.profile![axis.id]! - own[axis.id]!) / reach(axis.id);
      if (ratio > found.ratio) found = { category: category.id, axis: axis.id, ratio };
    }
  }
  return found;
}

/** 🧭️ The question beside `item` of the demand `standard-profiles` task put into `category`, whose profile questions it
 * on `axis`, while every other of `placed` sits in its own profile (design §8.4a.6): where some of them lies strictly
 * between the assigned and the own value on the axis, whether `item` lies above (or below, as the placement claims) the
 * one farthest from its own value; else whether it fits `category`, with the axis at about the profile's value. */
function profileAsks(item: string, category: string, axis: string, placed: readonly string[], locale: Locale): Asks {
  const categories = profiles.categories!;
  const on = (id: string): number => categories.find((candidate) => candidate.id === id)!.profile![axis]!;
  const at = (id: string): number => on(itemOf(profiles, id).category!);
  const [assigned, own] = [on(category), at(item)];
  const between = placed.filter((other) => other !== item && Math.min(assigned, own) < at(other) && at(other) < Math.max(assigned, own));
  const axisName = nameOf(profiles.axes!.find((candidate) => candidate.id === axis)!, locale).toLowerCase();
  if (between.length === 0) {
    const profile = nameOf(categories.find((candidate) => candidate.id === category)!, locale);
    return (question) => {
      expect(question).toMatch(locale === "en" ? / fits .*, with .* at about /u : / passt, mit .* bei rund /u);
      expect(question.includes(profile) && question.toLowerCase().includes(axisName), `${JSON.stringify(question)} names ${profile} and ${axisName}`).toBe(true);
    };
  }
  const other = between.reduce((farthest, candidate) => (Math.abs(own - at(candidate)) > Math.abs(own - at(farthest)) ? candidate : farthest));
  const above = assigned > at(other);
  const side = { en: above ? / (?:lies above|is higher) /u : / (?:lies below|is lower) /u, de: above ? / (?:über|höher ist als) /u : / (?:unter|niedriger ist als) /u }[locale];
  return (question) => {
    const [self, named, at] = [cited(profiles, item, locale), cited(profiles, other, locale), question.search(side)];
    expect(at >= 0 && question.indexOf(self) < at && at < question.indexOf(named), `${JSON.stringify(question)} asks whether ${self} lies ${above ? "above" : "below"} ${named}`).toBe(true);
    expect(question.toLowerCase()).toContain(locale === "en" ? ` in ${axisName}` : `in puncto ${axisName}`);
  };
}

/** 🗂️ Puts every shown standard of the demand `standard-profiles` classification on screen into its own profile, then
 * the first into {@link farthestProfile}, and expects one question beside it, in the learner's language
 * ({@link profileAsks}). Back in its own profile, nothing is asked. */
async function expectProfileQuestion(device: Device): Promise<void> {
  const task = screen(device.page, "task");
  const items = await shownItems(device);
  for (const item of items) await classify(device, item, itemOf(profiles, item).category!);
  await expect(task.locator("[data-hint]")).toHaveCount(0);
  const moved = items[0]!;
  const { category, axis, ratio } = farthestProfile(moved);
  expect(ratio, "the profile lies beyond the reach on its axis").toBeGreaterThan(1 + SLACK);
  await classify(device, moved, category);
  await expectQuestions(device, profiles, "profile", { [moved]: profileAsks(moved, category, axis, items, device.locale) });
  await classify(device, moved, itemOf(profiles, moved).category!);
  await expect(task.locator("[data-hint]")).toHaveCount(0);
}

test("easy shows the keys and questions a value placed far off against another item until it is fixed, and a perfect run earns 100 points and no badge", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Easy Player") });
  await playQuiz(learner, physics.id, "easy");
  await openTaskById(learner, powers.id);
  const task = screen(learner.page, "task");
  const items = await shownItems(learner);
  await expect(task.locator(".quiz-sort-key")).toHaveCount(items.length);
  await expect(task.locator("[data-quiz-item] input")).toHaveCount(0);
  const order = ascending(powers, items);
  await expectPowersQuestions(learner, order);
  await expectCappedPowers(learner, order);
  await sortInto(learner, order);
  await expect(task.locator("[data-hint]")).toHaveCount(0);

  await answerRun(learner, physics, "perfect");
  await submitRun(learner);
  expect((await expectFeedback(learner, physics, "perfect", "easy")).points).toBe(PAR.easy);
  expect(badgesFor([physics.id], "easy")).toEqual([]);
  await expect(screen(learner.page, "results").locator("[data-earned]")).toHaveCount(0);
  await goHome(learner);
  expect(await shownBest(learner, physics.id)).toEqual({ challenge: "easy", points: PAR.easy, par: PAR.easy });
});

test("medium shows the keys without hints, and a perfect run earns 200 points and the physics badges", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Medium Player") });
  await playQuiz(learner, physics.id, "medium");
  await openTaskById(learner, powers.id);
  const items = await shownItems(learner);
  await expect(screen(learner.page, "task").locator(".quiz-sort-key")).toHaveCount(items.length);
  const order = ascending(powers, items);
  await sortInto(learner, [order[order.length - 1]!, ...order.slice(0, -1)]);
  await expect(screen(learner.page, "task").locator("[data-hint]")).toHaveCount(0);

  await answerRun(learner, physics, "perfect");
  await submitRun(learner);
  expect((await expectFeedback(learner, physics, "perfect", "medium")).points).toBe(PAR.medium);
  const badges = badgesFor([physics.id], "medium");
  expect(badges.map((badge) => badge.id)).toContain("physics-expert");
  expect(await earnedLabels(learner)).toEqual(badges.map((badge) => badge.label.en));
  await goHome(learner);
  expect(await shownBest(learner, physics.id)).toEqual({ challenge: "medium", points: PAR.medium, par: PAR.medium });
});

test("hard hides the keys: true guesses earn 300 points, and one guess far off costs score and shows as a miss", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Hard Player") });
  await playQuiz(learner, physics.id, "hard");
  await openTaskById(learner, powers.id);
  const items = await shownItems(learner);
  await expect(screen(learner.page, "task").locator(".quiz-sort-key")).toHaveCount(0);
  await expect(screen(learner.page, "task").locator("[data-quiz-item] input")).toHaveCount(items.length);

  await answerRun(learner, physics, "perfect");
  await submitRun(learner);
  expect((await expectFeedback(learner, physics, "perfect", "hard")).points).toBe(PAR.hard);
  expect(await earnedLabels(learner)).toEqual(badgesFor([physics.id], "hard").map((badge) => badge.label.en));
  await goHome(learner);

  await playQuiz(learner, physics.id, "hard");
  await answerRun(learner, physics, "perfect");
  await openTaskById(learner, powers.id);
  const far = ascending(powers, await shownItems(learner))[0]!;
  await guessSorting(learner, far, itemOf(powers, far).value! * FAR_OFF, powers.quantity!);
  await submitRun(learner);
  const results = await expectFeedback(learner, physics, "flawed", "hard");
  expect(results.score).toBeLessThan(100);
  expect(results.points).toBeLessThan(PAR.hard);
  const sorted = results.tasks.find((entry) => entry.task === powers.id)!;
  expect(sorted.score).toBeLessThan(100);
  expect(sorted.rows.filter((row) => row.miss).map((row) => row.label)).toEqual([itemOf(powers, far).label.en]);
  expect(sorted.rows.find((row) => row.miss)!.text).toContain("Far off");
  await goHome(learner);
  expect(await shownBest(learner, physics.id), "the best run stays the perfect one").toEqual({ challenge: "hard", points: PAR.hard, par: PAR.hard });
});

test("expert runs every task against its own clock: a task past its deadline takes no more answers, and a run with open tasks scores what was answered", async ({ device }) => {
  const learner = await device("en");
  await learner.page.clock.install();
  await enter(learner, { kind: "pseudonym", handle: handle("Expert Player") });
  await playQuiz(learner, physics.id, "expert");
  const task = screen(learner.page, "task");

  await openTask(learner, 0);
  await expect(task.locator('[data-clock="closed"]')).toBeVisible();
  await expect(task.locator("[data-quiz-item]")).toHaveCount(0);
  await answerTask(learner, physics, "perfect");
  await expect(screen(learner.page, "run").locator("nav button").nth(0).locator("[data-complete]")).toBeVisible();

  await openTask(learner, 1);
  const late = taskOf(physics, await shownTask(learner));
  const allowed = /(\d+):(\d\d)/u.exec(await task.locator('[data-clock="closed"]').innerText());
  expect(allowed, "a closed clock says how long the task allows").not.toBeNull();
  await startClock(learner);
  await expect(task.locator('[data-clock="running"]')).toBeVisible();
  const first = (await shownItems(learner))[0]!;
  await guessSorting(learner, first, itemOf(late, first).value!, late.quantity!);
  await learner.page.clock.fastForward((Number(allowed![1]) * 60 + Number(allowed![2]) + 2) * 1000);
  await expect(task.locator('[data-clock="up"]')).toBeVisible();
  for (const field of await task.locator("[data-quiz-item] input").all()) await expect(field).toHaveAttribute("readonly", "");

  await openTask(learner, 2);
  const untouched = taskOf(physics, await shownTask(learner));
  await expect(task.locator('[data-clock="closed"]')).toBeVisible();
  await primary(screen(learner.page, "run")).click();
  const confirmation = learner.page.getByRole("alertdialog");
  await expect(confirmation).toContainText(late.title.en);
  await expect(confirmation).toContainText(untouched.title.en);
  await primary(confirmation).click();
  await expect(screen(learner.page, "results")).toBeVisible();

  const results = await shownResults(learner);
  expect({ challenge: results.challenge, par: results.par }).toEqual({ challenge: "expert", par: PAR.expert });
  expect(Object.fromEntries(results.tasks.map((entry) => [entry.task, entry.score]))).toEqual({ [physics.tasks[0]!.id]: 100, [late.id]: 0, [untouched.id]: 0 });
  expect(results.score).toBe(33.3);
  expect(results.points).toBe(133.3);
});

test("easy on a matching: beside both items whose cards were exchanged, a question asks for their order against another item, in the quantity where there are several, until the true cards are back", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Easy Matcher") });
  await playQuiz(learner, heating.id, "easy");
  await openTaskById(learner, uValues.id);
  await expectExchangeQuestions(learner, uValues);
  await openTaskById(learner, loads.id);
  await expectExchangeQuestions(learner, loads);

  await answerRun(learner, heating, "perfect");
  await submitRun(learner);
  expect((await expectFeedback(learner, heating, "perfect", "easy")).points).toBe(PAR.easy);
});

test("hard on a matching: typed guesses, and one far off is marked as a miss under the line that says how far a guess may lie", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Hard Matcher") });
  await playQuiz(learner, heating.id, "hard");
  await openTaskById(learner, uValues.id);
  const items = await shownItems(learner);
  await expect(screen(learner.page, "task").locator("select")).toHaveCount(0);
  await expect(screen(learner.page, "task").locator(".quiz-slot input")).toHaveCount(items.length * uValues.dimensions!.length);

  await answerRun(learner, heating, "perfect");
  await openTaskById(learner, uValues.id);
  const far = items[0]!;
  const dimension = uValues.dimensions![0]!;
  await guessMatching(learner, 0, far, itemOf(uValues, far).values![dimension.id]! * FAR_OFF, dimension.quantity);
  await submitRun(learner);

  const results = await expectFeedback(learner, heating, "flawed", "hard");
  expect(results.points).toBeLessThan(PAR.hard);
  const guessed = results.tasks.find((entry) => entry.task === uValues.id)!;
  expect(guessed.score).toBeLessThan(100);
  expect(guessed.rows.filter((row) => row.miss).map((row) => row.label)).toEqual([itemOf(uValues, far).label.en]);
  expect(guessed.rows.find((row) => row.miss)!.text).toContain("Far off");
  await expect(resultOf(learner, uValues.id).locator("[data-tolerance]")).toHaveText(/^A guess counts within [×±]\d.* of the true value\.$/u);
  for (const entry of results.tasks.filter((other) => other.task !== uValues.id)) expect(entry.score, `${entry.task} was answered with its true values`).toBe(100);
});

test("easy on a classification with spider profiles: beside a misplaced standard a question doubts the profile it was put into on one named axis", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Easy Sorter") });
  await playQuiz(learner, demand.id, "easy");
  await openTaskById(learner, profiles.id);
  await expectProfileQuestion(learner);

  await answerRun(learner, demand, "perfect");
  await submitRun(learner);
  expect((await expectFeedback(learner, demand, "perfect", "easy")).points).toBe(PAR.easy);
});

test("hard on a classification with spider profiles: the diagrams show their shape only, as shares without units, ranges or values", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Shape Reader") });
  await playQuiz(learner, demand.id, "medium");
  await openTaskById(learner, profiles.id);
  const shown = await diagrams(learner);
  expect(shown.length, "every profile has its diagram").toBe(profiles.categories!.length);
  expect(shown.every((diagram) => diagram.columns === 4 && diagram.text.includes("kWh/(m²·a)")), "medium names values, units and ranges").toBe(true);
  await goHome(learner);

  await playQuiz(learner, demand.id, "hard");
  await openTaskById(learner, profiles.id);
  const shapes = await diagrams(learner);
  expect(shapes.length).toBe(profiles.categories!.length);
  for (const shape of shapes) {
    expect(shape.columns, "an axis and its share").toBe(2);
    expect(shape.text).toContain("%");
    for (const unit of ["kWh", "€", "m²"]) expect(shape.text, `no ${unit} on a diagram that hides the keys`).not.toContain(unit);
  }
  const text = await screen(learner.page, "task").innerText();
  for (const category of profiles.categories!) if (category.description !== undefined) expect(text).not.toContain(category.description.en);

  await answerRun(learner, demand, "perfect");
  await submitRun(learner);
  expect((await expectFeedback(learner, demand, "perfect", "hard")).points).toBe(PAR.hard);
});

test("expert: a sorting and a matching answered within their time score fully, and a task whose time runs out scores nothing", async ({ device }) => {
  const learner = await device("en");
  await learner.page.clock.install();
  await enter(learner, { kind: "pseudonym", handle: handle("Expert Finisher") });
  const task = screen(learner.page, "task");

  await playQuiz(learner, demand.id, "expert");
  await answerRun(learner, demand, "perfect");
  for (let index = 0; index < demand.tasks.length; index++) {
    await openTask(learner, index);
    expect(await clockOf(learner), `${await shownTask(learner)} was answered in time`).not.toBe("up");
  }
  await submitRun(learner);
  expect((await expectFeedback(learner, demand, "perfect", "expert")).points).toBe(PAR.expert);
  await goHome(learner);

  await playQuiz(learner, physics.id, "expert");
  for (const id of [physics.tasks[0]!.id, powers.id]) {
    await openTaskById(learner, id);
    await answerTask(learner, physics, "perfect");
    expect(await clockOf(learner), `${id} was answered in time`).not.toBe("up");
  }
  await openTaskById(learner, energies.id);
  const allowed = await clockSeconds(learner);
  await startClock(learner);
  await learner.page.clock.fastForward((allowed + 2) * 1000);
  await expect(task.locator('[data-clock="up"]')).toBeVisible();
  await primary(screen(learner.page, "run")).click();
  const confirmation = learner.page.getByRole("alertdialog");
  await expect(confirmation).toContainText(energies.title.en);
  await primary(confirmation).click();
  await expect(screen(learner.page, "results")).toBeVisible();
  const timed = await shownResults(learner);
  expect(Object.fromEntries(timed.tasks.map((entry) => [entry.task, entry.score]))).toEqual({ [physics.tasks[0]!.id]: 100, [powers.id]: 100, [energies.id]: 0 });
  expect({ challenge: timed.challenge, score: timed.score, points: timed.points, par: timed.par }).toEqual({ challenge: "expert", score: 66.7, points: 266.7, par: PAR.expert });
});

test("in German the chooser, the hints and the results of a challenge speak German", async ({ device }) => {
  const learner = await device("de");
  await enter(learner, { kind: "pseudonym", handle: handle("Leichte Spielerin") });
  await learner.page.evaluate((id) => (window.location.hash = id), physics.id);
  const page = pane(learner.page, physics.id);
  await expect(page).toHaveAttribute("data-opened", "");
  const chooser = page.getByRole("group", { name: "Herausforderung" });
  await expect(chooser.getByRole("radio")).toHaveCount(CHALLENGES.length);
  for (const challenge of CHALLENGES) await expect(chooser).toContainText(`${CHALLENGE_NAMES[challenge].de}`);
  for (const challenge of CHALLENGES) await expect(chooser).toContainText(`Höchstpunktzahl: ${PAR[challenge]}`);
  await expect(chooser).toContainText("Die Werte sind verborgen und jede Aufgabe läuft gegen die Uhr.");

  await playQuiz(learner, physics.id, "easy");
  await openTaskById(learner, powers.id);
  const order = ascending(powers, await shownItems(learner));
  await expectPowersQuestions(learner, order);
  expect(await unresolvedLabels(learner)).toEqual([]);

  await answerRun(learner, physics, "perfect");
  await submitRun(learner);
  expect((await expectFeedback(learner, physics, "perfect", "easy")).points).toBe(PAR.easy);
  await expect(screen(learner.page, "results")).toContainText("Leicht · Punkte: 100 von 100");
  expect(await unresolvedLabels(learner)).toEqual([]);
});

test("in German a matching, one of two quantities named in puncto, and a classification with spider profiles ask their easy hints in German", async ({ device }) => {
  const learner = await device("de");
  await enter(learner, { kind: "pseudonym", handle: handle("Fragende Spielerin") });
  await playQuiz(learner, heating.id, "easy");
  await openTaskById(learner, uValues.id);
  await expectExchangeQuestions(learner, uValues);
  expect(await unresolvedLabels(learner)).toEqual([]);
  await openTaskById(learner, loads.id);
  await expectExchangeQuestions(learner, loads);
  expect(await unresolvedLabels(learner)).toEqual([]);
  await goHome(learner);

  await playQuiz(learner, demand.id, "easy");
  await openTaskById(learner, profiles.id);
  await expectProfileQuestion(learner);
  expect(await unresolvedLabels(learner)).toEqual([]);
});

test("choosing another challenge for a quiz with an open run asks first, and discarding starts the new run on that challenge", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Switcher") });
  await playQuiz(learner, heating.id, "medium");
  await openTask(learner, 0);
  await answerTask(learner, heating, "perfect");
  await goHome(learner);

  await chooseChallenge(learner, heating.id, "hard");
  await primary(pane(learner.page, heating.id).locator('[data-card="quiz"]')).click();
  const dialog = learner.page.getByRole("alertdialog");
  await expect(dialog).toContainText("Discard the open run?");
  await expect(dialog).toContainText("Your run on Medium is still open. Starting on Hard discards it with its answers.");
  await primary(dialog).click();
  const run = screen(learner.page, "run");
  await expect(run).toContainText(CHALLENGE_NAMES.hard.en);
  await expect(run.locator("nav button [data-complete]")).toHaveCount(0);
  await openTask(learner, 0);
  expect(await keysHidden(learner)).toBe(true);
  expect(await rememberedChallenge(learner, heating.id)).toBe("hard");
  expect(await rememberedChallenge(learner, physics.id), "the challenge is remembered per quiz").toBe("medium");
});

test("an expert run survives a reload: its clock keeps running from the instant the task was opened", async ({ device }) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Reloader") });
  await playQuiz(learner, physics.id, "expert");
  await openTaskById(learner, powers.id);
  const allowed = await clockSeconds(learner);
  await startClock(learner);
  const first = (await shownItems(learner))[0]!;
  await guessSorting(learner, first, itemOf(powers, first).value!, powers.quantity!);
  await expect.poll(() => clockSeconds(learner), { timeout: 10_000 }).toBeLessThanOrEqual(allowed - 3);
  const before = await clockSeconds(learner);

  await learner.page.reload();
  const overview = learner.page.locator("[data-layered-overview]");
  await expect(screen(learner.page, "run").or(overview)).toBeVisible();
  if (!(await screen(learner.page, "run").isVisible())) await primary(card(learner.page, physics.id)).click();
  await expect(screen(learner.page, "run")).toContainText(CHALLENGE_NAMES.expert.en);
  await openTaskById(learner, powers.id);
  expect(["running", "thirty", "ten"], "the clock runs on, it neither closed nor started again").toContain(await clockOf(learner));
  expect(await clockSeconds(learner)).toBeLessThanOrEqual(before);
  await expect(screen(learner.page, "task").locator(`[data-quiz-item="${first}"] input`)).not.toHaveValue("");
});
