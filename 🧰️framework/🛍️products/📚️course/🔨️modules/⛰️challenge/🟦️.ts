/** ⛰️ The challenges of a run: one rule table, the points a score earns, when a value misses, the time a task allows, the instant a learner acted and the hints an easy run gives.
 *
 * Each challenge is the one before plus one step: easy shows the keys (the numbers a task turns on) and hints, medium
 * shows the keys, hard hides them so the learner guesses, expert also runs every task against a clock. A value misses
 * when it lies farther from the truth than the reach of its set: on a logarithmic scale a factor, {@link REACH_FACTOR}
 * or the square root of the presented true values' max/min ratio where that is less; on a linear scale a distance,
 * half their spread, both widened by a relative {@link REACH_SLACK} of 1e-9 so a decimal typed at exactly the reach
 * stays within. Only `/`, `*`, `+`, `sqrt`, `−`, `abs`, explicit min/max and comparisons are used, all exactly rounded,
 * so an exact ×1000 never misses and every twin agrees bit for bit. The same miss drives the hints (easy) and the scoring
 * of guesses (hard, expert). Everything here is pure, so the device's deputy and the proctor decide alike.
 *
 * @see ../../README.md — the challenges
 * @see ../📏️scoring/🟦️.ts — the scoring of guesses with misses
 * @see ./🦀️.rs — the Rust twin
 */
import type { Answer, Category, Challenge, ClassificationItem, ClassificationTask, Hint, Profile, Scale, Score, SheetTask, Slug, Task, TaskKind, Timestamp, Verdict } from "../../🧬️schema/🟦️.ts";

/** 📋️ What a challenge asks: whether the keys show, whether far-off answers get hints, whether tasks run against a clock, and the points a perfect run earns. */
export type ChallengeRules = { readonly keys: boolean; readonly hints: boolean; readonly timed: boolean; readonly par: number };

/** 🗻️ The rules of every challenge. */
export const CHALLENGE_RULES: { readonly [C in Challenge]: ChallengeRules } = {
  easy: { keys: true, hints: true, timed: false, par: 100 },
  medium: { keys: true, hints: false, timed: false, par: 200 },
  hard: { keys: false, hints: false, timed: false, par: 300 },
  expert: { keys: false, hints: false, timed: true, par: 400 },
};

/** 📖️ The rules of `challenge`. */
export function challengeRules(challenge: Challenge): ChallengeRules {
  return CHALLENGE_RULES[challenge];
}

/** 🔢️ The rank of a challenge: 0 easy, 1 medium, 2 hard, 3 expert. */
export function challengeRank(challenge: Challenge): number {
  switch (challenge) {
    case "easy":
      return 0;
    case "medium":
      return 1;
    case "hard":
      return 2;
    case "expert":
      return 3;
  }
}

/** 🧗️ Whether `challenge` is at least as demanding as `least`. */
export function challengeMeets(challenge: Challenge, least: Challenge): boolean {
  return challengeRank(challenge) >= challengeRank(least);
}

/** 💰️ The points a score earns at a challenge: `score × par`. */
export function points(score: Score, challenge: Challenge): number {
  return score * challengeRules(challenge).par;
}

/** 🔭️ The cap of the reach on a logarithmic scale, a factor: the reach of values that do not spread, widened by {@link REACH_SLACK} before a value misses. */
export const REACH_FACTOR = 1000;

/** 📡️ The reach of a set of presented true values on their scale. Logarithmic: a factor, `sqrt(hi / lo)` capped at {@link REACH_FACTOR}, the cap itself when they do not spread. Linear: a distance, `(hi − lo) / 2`, unbounded when they do not spread. A value that compares to nothing (`NaN`) takes no part. */
export function reach(values: readonly number[], scale: Scale): number {
  let lowest = Infinity;
  let highest = -Infinity;
  for (const value of values) {
    if (value < lowest) lowest = value;
    if (value > highest) highest = value;
  }
  if (scale === "linear") return highest > lowest ? (highest - lowest) / 2 : Infinity;
  if (!(highest > lowest)) return REACH_FACTOR;
  const root = Math.sqrt(highest / lowest);
  return root < REACH_FACTOR ? root : REACH_FACTOR;
}

/** 🪶️ The relative slack a reach is widened by before a value misses: a value typed in decimal at exactly the reach (`0.018` against `18`) lies a hair beyond it in binary and still counts as within. */
export const REACH_SLACK = 1e-9;

/** 🎯️ Whether `value` lies farther from `truth` than `reach` widened by {@link REACH_SLACK}: by the ratio `max / min` on a logarithmic scale, by the distance on a linear one; an infinite reach never misses. */
export function misses(value: number, truth: number, scale: Scale, reach: number): boolean {
  const bound = reach * (1 + REACH_SLACK);
  if (scale === "linear") return Math.abs(value - truth) > bound;
  return (value > truth ? value / truth : truth / value) > bound;
}

/** ⏲️ The seconds a timed task allows: a base and a share per presented item (per item and dimension for a matching). */
export const TASK_SECONDS = { base: 30, classification: 8, sorting: 12, matching: 12 } as const;

/** ⏳️ The seconds a timed task of `kind` allows for `items` presented items (and `dimensions` of a matching). */
export function taskSeconds(kind: TaskKind, items: number, dimensions: number): number {
  return TASK_SECONDS.base + TASK_SECONDS[kind] * items * (kind === "matching" ? dimensions : 1);
}

/** 🛫️ How far, in milliseconds, a device's claim may run ahead of the decider's clock before it is lowered: five minutes, more than an honest device's clock drifts, too little to buy a task more time. */
export const CLOCK_LEAD = 300_000;

/** 🕰️ The instant a learner acted as a decider counts it: the device's `at`, lowered to {@link CLOCK_LEAD} past the decider's `now`, then raised to `floor` (the run's start, or the task's opening; 0 for a start). An honest device decides at its own clock, so the lead never lowers its claims and the deputy and the proctor decide alike. */
export function acted(at: Timestamp, floor: Timestamp, now: Timestamp): Timestamp {
  const lead = now + CLOCK_LEAD;
  const capped = at < lead ? at : lead;
  return capped > floor ? capped : floor;
}

/** 🔑️ An item of a numeric task with the key the learner assigned it, its true value, whether the key misses that value and whether the item is familiar. */
type Keyed = { readonly item: Slug; readonly key: number; readonly value: number; readonly miss: boolean; readonly familiar: boolean };

/** 🔗️ A reference a compare hint may name: the relation the keys claim to it, the true one, whether it is familiar and how far the claim lies from no relation at all (`max(ρ, 1/ρ)`, `|δ|`). */
type Related = { readonly other: Slug; readonly familiar: boolean; readonly claim: number; readonly truth: number; readonly oriented: number };

/** ⏸️ A compare hint before its reference is chosen: the missed item, its dimension, its scale and the references tied for the largest error, in sheet order. */
type Pending = { readonly item: Slug; readonly dimension?: Slug; readonly linear: boolean; readonly tied: readonly Related[] };

/** 🏋️ A hint, or a compare hint whose reference is still to be chosen, with the weight it competes for the {@link HINTS_PER_TASK} places with: the largest error of a compare hint, the gap relative to its reach of a profile hint, none for group and category hints, which come after. */
type Weighted = ({ readonly hint: Hint } | { readonly pending: Pending }) & { readonly weight?: number };

/** 🎟️ How many hints one task gives at most: those with the largest weight first, then those without one, each set in the order they were made. */
export const HINTS_PER_TASK = 3;

/** 🪞️ How a claimed relation stands to the true one around `pivot` (1 for a ratio, 0 for a difference): reversed when the claim lies on one side and the truth at the pivot or on the other, else under when the truth lies beyond the claim (a claim at the pivot: when the truth lies above it), else over. */
export function verdictOf(claim: number, truth: number, pivot: number): Verdict {
  if (claim > pivot ? truth <= pivot : claim < pivot && truth >= pivot) return "reversed";
  return (claim >= pivot ? truth > claim : truth < claim) ? "under" : "over";
}

/** 🤨️ The compare hint of a missed item before its reference is chosen, weighted by the largest error: the relations its key claims to the other keyed items (preferably those whose own key does not miss), rated by the error `max(ρ, τ) / min(ρ, τ)` of the claimed ratio `ρ` and the true ratio `τ` on a logarithmic scale, `|δ − Δ|` of the differences on a linear one; those within the relative {@link REACH_SLACK} of the largest tie — or none when no other item holds a key. */
function compareHint(hinted: Keyed, keyed: readonly Keyed[], scale: Scale, dimension?: Slug): Weighted[] {
  const others = keyed.filter((candidate) => candidate.item !== hinted.item);
  const anchors = others.filter((candidate) => !candidate.miss);
  const linear = scale === "linear";
  const related = (anchors.length > 0 ? anchors : others).map((other) => {
    const claim = linear ? hinted.key - other.key : hinted.key / other.key;
    const truth = linear ? hinted.value - other.value : hinted.value / other.value;
    return { other: other.item, familiar: other.familiar, claim, truth, error: linear ? Math.abs(claim - truth) : claim > truth ? claim / truth : truth / claim, oriented: linear ? Math.abs(claim) : Math.max(claim, 1 / claim) };
  });
  const largest = Math.max(...related.map((candidate) => candidate.error));
  const tied = related.filter((candidate) => candidate.error * (1 + REACH_SLACK) >= largest);
  if (tied.length === 0) return [];
  return [{ pending: { item: hinted.item, ...(dimension === undefined ? {} : { dimension }), linear, tied }, weight: largest }];
}

/** 🪝️ The compare hint of a kept pending one: among the references tied for the largest error a reference not yet `used` by an earlier compare hint of the task (of the same dimension) wins, then a familiar one, then the smallest oriented claim, then the first in sheet order; the chosen one is added to `used`. */
function referenced({ item, dimension, linear, tied }: Pending, used: Map<Slug | undefined, Set<Slug>>): Hint {
  const taken = used.get(dimension) ?? new Set<Slug>();
  const fresh = (candidate: Related) => !taken.has(candidate.other);
  let found = tied[0]!;
  for (const candidate of tied) if (fresh(candidate) !== fresh(found) ? fresh(candidate) : candidate.familiar !== found.familiar ? candidate.familiar : candidate.oriented < found.oriented) found = candidate;
  used.set(dimension, taken.add(found.other));
  const { other, claim, truth } = found;
  return { kind: "compare", item, other, ...(dimension === undefined ? {} : { dimension }), ...(linear ? { difference: claim } : { factor: claim }), verdict: verdictOf(claim, truth, linear ? 0 : 1) };
}

/** 🧐️ The axis on which an item's assigned profile questions it, with its gap relative to its reach: among the task's axes with values in both profiles and a spread among the presented categories, the one with the largest gap relative to its reach (half that spread; the first in axis order on ties), when some gap exceeds its reach widened by {@link REACH_SLACK}. */
function profileAxis(task: ClassificationTask, presented: readonly Category[], assigned: Profile, own: Profile): { readonly axis: Slug; readonly ratio: number } | undefined {
  let found: { readonly axis: Slug; readonly ratio: number } | undefined;
  let beyond = false;
  for (const axis of task.axes ?? []) {
    if (!Object.hasOwn(assigned, axis.id) || !Object.hasOwn(own, axis.id)) continue;
    const values = presented.flatMap((category) => (category.profile && Object.hasOwn(category.profile, axis.id) ? [category.profile[axis.id]!] : []));
    const within = values.length === 0 ? 0 : (Math.max(...values) - Math.min(...values)) / 2;
    if (!(within > 0)) continue;
    const gap = Math.abs(assigned[axis.id]! - own[axis.id]!);
    if (gap > within * (1 + REACH_SLACK)) beyond = true;
    const ratio = gap / within;
    if (!found || ratio > found.ratio) found = { axis: axis.id, ratio };
  }
  return beyond ? found : undefined;
}

/** 🧭️ The item a profile hint measures `item` against on `axis`: among the other items placed in their own category whose value `r` on the axis lies strictly between the assigned value and the own one, the farthest from the own value (the first in sheet order on ties), with whether the placement claims `item` lies above it. */
function profileOther(task: ClassificationTask, placed: readonly { readonly item: ClassificationItem; readonly assigned: Slug }[], item: Slug, axis: Slug, assigned: number, own: number): { readonly other: Slug; readonly above: boolean } | undefined {
  let found: { readonly other: Slug; readonly above: boolean; readonly gap: number } | undefined;
  for (const anchor of placed) {
    if (anchor.item.id === item || anchor.assigned !== anchor.item.category) continue;
    const profile = task.categories.find((category) => category.id === anchor.item.category)?.profile;
    if (!profile || !Object.hasOwn(profile, axis)) continue;
    const value = profile[axis]!;
    const above = assigned > value && own < value;
    if (!above && !(assigned < value && own > value)) continue;
    const gap = Math.abs(own - value);
    if (!found || gap > found.gap) found = { other: anchor.item.id, above, gap };
  }
  return found && { other: found.other, above: found.above };
}

/** ✂️ The first {@link HINTS_PER_TASK} hints by weight (largest first, unweighted last, earlier on ties), in the order they were made. */
function capped(weighted: readonly Weighted[]): Weighted[] {
  const kept = weighted
    .map((entry, index) => ({ entry, index }))
    .sort((left, right) => (left.entry.weight === undefined ? 1 : 0) - (right.entry.weight === undefined ? 1 : 0) || (left.entry.weight! > right.entry.weight! ? -1 : left.entry.weight! < right.entry.weight! ? 1 : 0) || left.index - right.index)
    .slice(0, HINTS_PER_TASK);
  return kept.sort((left, right) => left.index - right.index).map(({ entry }) => entry);
}

/** 💡️ The hints of an answer to a task, none without an answer; each questions one concrete relation the answer claims and never states the truth, and at most {@link HINTS_PER_TASK} are given, those with the largest weight. Sorting (the sheet task shows `keys`), in the learner's order, and matching (per dimension with `cards`, then per item, in sheet order): per item whose assigned key misses its value, a {@link CompareHint} against another keyed item, the references chosen after the cap in that order so that a task's hints (of one dimension) name different references where the tie window allows. Classification, in sheet order, per item assigned to a category not its own: a {@link ProfileHint} when both categories carry profiles (none for a near miss), measured against an item placed in its own category where one lies between, else a {@link GroupHint} with the first item in the same assigned category whose own category differs (`together`), else with the first item of its own category assigned to another category than it, else a {@link CategoryHint}. Entries the task or the sheet task cannot resolve, and assignments to unknown categories, give no hint.
 *
 * @see ../../README.md — the hints
 */
export function hintsOf(task: Task, sheetTask: SheetTask, answer?: Answer): Hint[] {
  const used = new Map<Slug | undefined, Set<Slug>>();
  return capped(weightedHints(task, sheetTask, answer)).map((entry) => ("hint" in entry ? entry.hint : referenced(entry.pending, used)));
}

/** ⚖️ Every hint of an answer to a task with its weight, in the order {@link hintsOf} gives them, before the cap. */
function weightedHints(task: Task, sheetTask: SheetTask, answer?: Answer): Weighted[] {
  if (!answer) return [];
  if (task.kind === "sorting" && sheetTask.kind === "sorting" && answer.kind === "sorting") {
    const keys = sheetTask.keys;
    if (!keys) return [];
    const scale = task.quantity.scale;
    const within = reach(keys, scale);
    const keyed = sheetTask.items.flatMap((presented): Keyed[] => {
      const item = task.items.find((candidate) => candidate.id === presented.id);
      const position = answer.order.indexOf(presented.id);
      if (!item || position < 0 || position >= keys.length) return [];
      return [{ item: item.id, key: keys[position]!, value: item.value, miss: misses(keys[position]!, item.value, scale, within), familiar: item.familiar === true }];
    });
    return answer.order.flatMap((id) => {
      const hinted = keyed.find((candidate) => candidate.item === id);
      return hinted?.miss ? compareHint(hinted, keyed, scale) : [];
    });
  }
  if (task.kind === "matching" && sheetTask.kind === "matching" && answer.kind === "matching") {
    const assignments = answer.assignments;
    if (!assignments) return [];
    const items = sheetTask.items.flatMap((presented) => task.items.find((candidate) => candidate.id === presented.id) ?? []);
    return sheetTask.dimensions.flatMap((dimension) => {
      const cards = dimension.cards;
      const assigned = Object.hasOwn(assignments, dimension.id) ? assignments[dimension.id]! : undefined;
      if (!cards || !assigned) return [];
      const scale = dimension.quantity.scale;
      const valued = items.filter((item) => Object.hasOwn(item.values, dimension.id));
      const within = reach(
        valued.map((item) => item.values[dimension.id]!),
        scale,
      );
      const keyed = valued.flatMap((item): Keyed[] => {
        const card = Object.hasOwn(assigned, item.id) ? assigned[item.id]! : undefined;
        if (card === undefined || !(card >= 0 && card < cards.length)) return [];
        const value = item.values[dimension.id]!;
        return [{ item: item.id, key: cards[card]!, value, miss: misses(cards[card]!, value, scale, within), familiar: item.familiar === true }];
      });
      return keyed.flatMap((hinted) => (hinted.miss ? compareHint(hinted, keyed, scale, dimension.id) : []));
    });
  }
  if (task.kind === "classification" && sheetTask.kind === "classification" && answer.kind === "classification") {
    const presented = sheetTask.categories.flatMap((shown) => task.categories.find((category) => category.id === shown.id) ?? []);
    const placed = sheetTask.items.flatMap((shown) => {
      const item = task.items.find((candidate) => candidate.id === shown.id);
      return item && Object.hasOwn(answer.assignments, shown.id) ? [{ item, assigned: answer.assignments[shown.id]! }] : [];
    });
    return placed.flatMap(({ item, assigned }): Weighted[] => {
      const chosen = task.categories.find((category) => category.id === assigned);
      if (!chosen || assigned === item.category) return [];
      const own = task.categories.find((category) => category.id === item.category);
      if (own?.profile && chosen.profile) {
        const found = profileAxis(task, presented, chosen.profile, own.profile);
        if (!found) return [];
        const relative = profileOther(task, placed, item.id, found.axis, chosen.profile[found.axis]!, own.profile[found.axis]!);
        return [{ hint: { kind: "profile", item: item.id, category: assigned, axis: found.axis, ...relative }, weight: found.ratio }];
      }
      const together = placed.find((other) => other.item.id !== item.id && other.assigned === assigned && other.item.category !== item.category);
      if (together) return [{ hint: { kind: "group", item: item.id, other: together.item.id, together: true } }];
      const apart = placed.find((other) => other.item.id !== item.id && other.item.category === item.category && other.assigned !== assigned);
      if (apart) return [{ hint: { kind: "group", item: item.id, other: apart.item.id, together: false } }];
      return [{ hint: { kind: "category", item: item.id, category: assigned } }];
    });
  }
  return [];
}
