/** 🪜️ The mischief, the ladder and the phone in a real browser: a pet lifts a copy of a quiz task, a climber raises its ladder or a grappler shoots its rope, and a phone shows at most two small pets.
 *
 * @see ../🟦️.ts — the pets in a real browser, as the second round made them
 */
import { HANG_ROD } from "@semio-tech/pets";
import {QUIZZES, enter, expect, pane, playQuiz, swipeTo, test, type Device, activeOverview} from "../../../🎭️e2e/🚶️learner/🟦️.ts";
import { ANSWER_MS, APPEAR_MS, bodies, type Body, CARD_PARTS, carry, castOf, choosePets, CONTROLS, enterWithPets, expectCast, expectPerched, floorSpot, framesPass, HASTE, hasten, HOME, hoverOf, instrumented, type Instruments, KINDS, LANDING_MS, layer, MISCHIEF_HASTE, MISCHIEF_QUIZ, misplaced, pets, reachable, RECAST_MS, ROPE_MS, ROPE_REACH, ROPE_ROUND_MS, standing, TEMPO, trailOf, WALK_MS } from "../🧰️pet-kit/🟦️.ts";

for (const reclaim of ["hover", "focus"] as const) test(`a pet lifts a copy of a task of a quiz's page out of its place, beside its transparent original; the original is back at once on ${reclaim}, and the pet is thrown off`, async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await page.addInitScript(() => { Math.random = () => 0.5; });
  await page.addInitScript(hasten, { attribute: TEMPO, tempo: MISCHIEF_HASTE });
  await enterWithPets(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await choosePets(learner, "lively");
  await page.evaluate((id) => (window.location.hash = id), MISCHIEF_QUIZ);
  await expect(pane(page, MISCHIEF_QUIZ)).toHaveAttribute("data-opened", "");
  await instrumented(page);
  const lifted = page.locator("[data-pet-prop]").and(page.locator('[style*="opacity: 0"]'));
  const copy = layer(learner).locator(':scope > [inert][aria-hidden="true"]');
  await page.mouse.move(4, 4);
  await expect(lifted, `a task row lifted, to be taken back by ${reclaim}`).toHaveCount(1, { timeout: 4 * RECAST_MS });
  await expect(copy).toHaveCount(1);
  const original = await lifted.boundingBox();
  expect(original, "the original keeps its box").not.toBeNull();
  await expect.poll(() => copy.boundingBox().then((shown) => (shown === null ? 0 : Math.hypot(shown.x - original!.x, shown.y - original!.y))), { message: "the copy stands out of the original's place", timeout: ANSWER_MS }).toBeGreaterThan(1);
  const [key, pets] = await Promise.all([lifted.getAttribute("data-pet-prop"), bodies(page)]);
  const pusher = pets.find((body) => body.activity === "push");
  expect(pusher, "a pet pushes the copy").toBeDefined();
  const pushed = (await trailOf(page, pusher!.id)).length - 1;
  if (reclaim === "hover") await page.mouse.move(original!.x + original!.width / 2, original!.y + original!.height / 2);
  else await lifted.evaluate((element) => (element.matches("a[href], button, input, select, textarea, [tabindex]") ? (element as HTMLElement) : (element.querySelector<HTMLElement>("a[href], button, input, select, textarea, [tabindex]") ?? Object.assign(element as HTMLElement, { tabIndex: -1 }))).focus());
  await expect(lifted, `the original is back at once on ${reclaim}`).toHaveCount(0, { timeout: ANSWER_MS });
  await expect(copy).toHaveCount(0, { timeout: ANSWER_MS });
  expect(key, "a task row of the opened quiz's page").toMatch(new RegExp(`^${MISCHIEF_QUIZ}/[^/]+$`));
  await expect.poll(async () => (await trailOf(page, pusher!.id)).slice(pushed).join(" → "), { message: `${pusher!.id} is thrown off`, timeout: ANSWER_MS }).toMatch(/^push\/[a-z]+\/[^ ]* → tumble\/air\//u);
  test.info().annotations.push({ type: "mischief", description: `${reclaim}: ${pusher!.id} pushed ${key}, then ${(await trailOf(page, pusher!.id)).slice(pushed).join(" → ")}` });
  if (reclaim === "focus") await page.evaluate(() => (document.activeElement as HTMLElement | null)?.blur());
});

test("a pet that cannot hop to a perch above raises its ladder or shoots its grapple there: on a quiz's page a climber raises its ladder against the lowest card of the column, and on the home screen a grappler on the footer line reels itself up a rope to a card", async ({ device }) => {
  test.setTimeout(WALK_MS + ROPE_MS + 4 * RECAST_MS);
  const learner = await device("en");
  const page = learner.page;
  await page.addInitScript(hasten, { attribute: TEMPO, tempo: HASTE });
  await enterWithPets(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await choosePets(learner, "lively");
  await page.keyboard.press("Escape");
  await expect(pane(page, "prefs")).not.toHaveAttribute("data-opened", "");
  const climber = (id: string): boolean => ["climb", "ladder"].every((gear) => KINDS[id]?.gear.includes(gear) === true);
  const gutters = QUIZZES.find((quiz) => castOf(quiz.id).core.some(climber));
  expect(gutters, "a quiz whose cast has a climber with a ladder in its core").toBeDefined();
  await page.evaluate((id) => (window.location.hash = id), gutters!.id);
  await expect(pane(page, gutters!.id)).toHaveAttribute("data-opened", "");
  await expectCast(learner, gutters!.id, true);
  await instrumented(page);
  const carried = (footing: string): Promise<readonly string[]> => page.evaluate((wanted) => Object.entries((window as unknown as { petWalk: Instruments }).petWalk.trails).flatMap(([id, trail]) => (trail.some((line) => line.split("/")[1] === wanted) ? [id] : [])), footing);
  await expect.poll(() => carried("ladder"), { message: `a pet on a ladder on the page of ${gutters!.id}`, timeout: WALK_MS }).not.toEqual([]);
  await page.keyboard.press("Escape");
  await expect(pane(page, gutters!.id)).not.toHaveAttribute("data-opened", "");
  await expectCast(learner, HOME, true);
  await instrumented(page);
  const grappling = (body: Body): boolean => KINDS[body.id]?.gear.includes("grapple") === true;
  const moved: string[] = [];
  const held: string[] = [];
  await expect(async () => {
    if ((await carried("rope")).length > 0) return;
    const footer = (await page.locator(".course-app > footer").boundingBox())!;
    const raised = (body: Body): number => footer.y - body.bottom - hoverOf(body.id);
    for (const body of (await bodies(page)).filter((candidate) => grappling(candidate) && candidate.footing === "perch" && raised(candidate) > 2)) {
      const pet = await reachable(page, (candidate) => candidate.id === body.id).catch(() => undefined);
      if (pet === undefined) continue;
      const height = pet.bottom - pet.top;
      const hover = hoverOf(pet.id);
      const spot = await page.evaluate(floorSpot, { parts: CARD_PARTS, controls: CONTROLS, reach: (pet.right - pet.left) / 2 + 12, room: height + hover + 12 });
      if (spot === null) continue;
      await carry(page, pet, { x: spot.x, y: spot.top - HANG_ROD * height - hover - 8 });
      await page.mouse.up();
      moved.push(pet.id);
      await expectPerched(page, pet.id, LANDING_MS).catch(() => undefined);
      if ((await bodies(page)).some((landed) => landed.id === pet.id && landed.footing === "perch" && raised(landed) <= 2)) break;
    }
    const shown = (await bodies(page)).filter((body) => body.opacity > 0.99);
    const grounded = shown.filter((body) => grappling(body) && body.footing === "perch" && raised(body) <= 2);
    expect(grounded.length, `a grappler on the footer line among ${shown.map((body) => `${body.id} ${body.footing}`).join(", ")}`).toBeGreaterThan(0);
    const reach = Math.max(...grounded.map((body) => ROPE_REACH * KINDS[body.id]!.size.height));
    const sitting = (body: Body): boolean => !grappling(body) && raised(body) > 2 && raised(body) <= reach;
    const until = Date.now() + ROPE_ROUND_MS;
    let holding = false;
    try {
      while (Date.now() < until && (await carried("rope")).length === 0) {
        const sitter = await reachable(page, sitting).catch(() => undefined);
        if (sitter !== undefined) {
          if (holding) await page.mouse.up();
          await carry(page, sitter, { x: 50, y: footer.y / 2 });
          holding = true;
          held.push(sitter.id);
        }
        await page.evaluate(framesPass, 30);
      }
    } finally {
      if (holding) await page.mouse.up();
    }
    expect(await carried("rope"), "a grappler on the footer line reels itself up a rope").not.toEqual([]);
  }).toPass({ timeout: ROPE_MS, intervals: [0] });
  const [laddered, roped] = await Promise.all([carried("ladder"), carried("rope")]);
  test.info().annotations.push({ type: "gear", description: `on a ladder: ${laddered.join(", ")}; on a rope: ${roped.join(", ")}; taken to the footer line by the hand: ${moved.join(", ") || "nobody"}; held off a perch in reach of a rope: ${held.join(", ") || "nobody"}` });
  for (const id of roped) test.info().annotations.push({ type: "trail", description: `${id}: ${(await trailOf(page, id)).join(" → ")}` });
  expect(roped.every((id) => KINDS[id]?.gear.includes("grapple") === true), `only grapplers are on a rope: ${roped.join(", ")}`).toBe(true);
  expect(laddered.every((id) => KINDS[id]!.gear.some((gear) => gear !== "parachute")), `only pets with gear are on a ladder: ${laddered.join(", ")}`).toBe(true);
});

test("a phone shows at most two small pets, on edges it shows and never on a control", async ({ browser }, info) => {
  const context = await browser.newContext({ viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true, baseURL: info.project.use.baseURL, locale: "en-GB" });
  const page = await context.newPage();
  const problems: string[] = [];
  page.on("console", (message) => {
    if (message.type() === "error") problems.push(`console error: ${message.text()}`);
  });
  page.on("pageerror", (error) => problems.push(`page error: ${error.message}`));
  const phone: Device = { context, page, locale: "en", problems, expectingFailures: async () => 0 };
  const few = async (): Promise<void> => {
    const shown = await standing(page);
    expect(shown.length, "a newcomer waits until whoever it replaces is gone").toBeLessThanOrEqual(2);
    for (const pet of shown) expect(pet.size).toBe(0.8);
  };
  await enter(phone, { kind: "anonymous" });
  await expect(activeOverview(page)).toHaveAttribute("data-mode", "swipe");
  await expect(pets(phone)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
  await few();

  const quiz = QUIZZES[0]!;
  await page.evaluate((id) => (window.location.hash = id), quiz.id);
  await expect(pane(page, quiz.id)).toHaveAttribute("data-opened", "");
  await expectCast(phone, quiz.id, false);
  await expect(pets(phone)).not.toHaveCount(0, { timeout: RECAST_MS });
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
  await few();

  await page.keyboard.press("Escape");
  await expect(pane(page, quiz.id)).not.toHaveAttribute("data-opened", "");
  await swipeTo(phone, quiz.id);
  await playQuiz(phone, quiz.id);
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
  await few();
  expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
  await context.close();
  expect(problems).toEqual([]);
});
