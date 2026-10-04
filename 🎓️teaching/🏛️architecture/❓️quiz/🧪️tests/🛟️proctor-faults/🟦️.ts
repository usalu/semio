/** 🛟️ A proctor that fails, whenever and however often it fails. It dies and comes back between every two tasks of an
 * expert run and once more in the very moment the run is submitted: the clock keeps running on the device, the
 * submission is decided wherever it can be, nothing is lost and nothing is told — and a fresh device finds the run.
 * A proctor of another contract is sent nothing: while it is older the device decides everything, while it is newer the
 * page says it is out of date and offers a reload, and once the two agree everything arrives. The contract is changed
 * by answering the proctor's `GET /instance` in the browser with every version one lower or higher; the deaths go
 * through the gate's control endpoint, so the specs run alone; whatever a spec does, the proctor runs again once it
 * ends.
 * @see ../../🎭️e2e/🟦️.ts — the control endpoint
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts — the agreement on the contract */
import { PAR, PROCTOR_ORIGIN, answerTask, connection, enter, expect, expectFeedback, handle, learnerId, openTask, playQuiz, proctor, quizOf, screen, shownBest, submitRun, test, type Device } from "../../🎭️e2e/🚶️learner/🟦️.ts";

test.afterEach(() => proctor("start"));

/** 📇️ One command or query kind as a proctor declares it. */
interface Declared {
  readonly kind: string;
  readonly version: number;
}

/** 🏛️ What the proctor of the stack declares at `GET /instance`, with every version moved `by`. */
async function shiftedInstance(by: number): Promise<unknown> {
  const instance = (await (await fetch(`${PROCTOR_ORIGIN}/instance`)).json()) as { readonly modules: readonly { readonly commands: readonly Declared[]; readonly queries: readonly Declared[] }[] };
  const shift = (declared: Declared): Declared => ({ ...declared, version: declared.version + by });
  return { ...instance, modules: instance.modules.map((module) => ({ ...module, commands: module.commands.map(shift), queries: module.queries.map(shift) })) };
}

/** 🔎️ The requests for a proctor's contract. */
const instanceRoute = (url: URL): boolean => url.pathname === "/instance";

/** 🪞️ From now on `device` meets a proctor whose contract is `by` versions away from the site's — answered with the
 * headers a proctor on another origin sends, its preflight included —; `0` meets the real one. */
async function contractAway(device: Device, by: number): Promise<void> {
  await device.context.unrouteAll({ behavior: "wait" });
  if (by === 0) return;
  const answer = await shiftedInstance(by);
  await device.context.route(instanceRoute, (route) => {
    const request = route.request();
    const origin = request.headers()["origin"];
    const cors: Record<string, string> = origin === undefined ? {} : { "access-control-allow-origin": origin, "access-control-allow-methods": "GET", "access-control-allow-headers": request.headers()["access-control-request-headers"] ?? "content-type", vary: "origin" };
    return request.method() === "OPTIONS" ? route.fulfill({ status: 204, headers: cors }) : route.fulfill({ json: answer, headers: cors });
  });
}

/** 📨️ The commands and queries `device` sends from now on, by path. */
function sentCalls(device: Device): string[] {
  const sent: string[] = [];
  device.page.on("request", (request) => {
    const path = new URL(request.url()).pathname;
    if (path === "/commands" || path === "/queries") sent.push(`${request.method()} ${path}`);
  });
  return sent;
}

test("a proctor that dies between every two tasks of an expert run and as it is submitted loses nothing and tells nothing", async ({ device }) => {
  test.setTimeout(480_000);
  const quiz = quizOf("heating");
  const name = handle("Unshaken");
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: name });
  await playQuiz(learner, quiz.id, "expert");
  const failures = await learner.expectingFailures(async () => {
    for (let index = 0; index < quiz.tasks.length; index++) {
      await proctor(index % 2 === 0 ? "stop" : "start");
      await openTask(learner, index);
      await answerTask(learner, quiz, "perfect");
      await expect(screen(learner.page, "run").locator("nav button").nth(index).locator("[data-complete]")).toBeVisible();
    }
    await proctor("start");
    await Promise.all([proctor("stop"), submitRun(learner)]);
    expect((await expectFeedback(learner, quiz, "perfect", "expert")).points).toBe(PAR.expert);
    await proctor("start");
    await expect(connection(learner)).toHaveAttribute("data-tone", "calm", { timeout: 120_000 });
    await expect(learner.page.locator("[data-presence-status]")).toBeVisible({ timeout: 90_000 });
  });
  expect(failures, "the proctor was really away: requests failed").toBeGreaterThan(0);
  await expect(learner.page.getByRole("alert")).toHaveCount(0);
  expect((await expectFeedback(learner, quiz, "perfect", "expert")).points).toBe(PAR.expert);

  const elsewhere = await device("en");
  await enter(elsewhere, { kind: "pseudonym", handle: name });
  expect(await learnerId(elsewhere)).toBe(await learnerId(learner));
  await expect.poll(async () => (await shownBest(elsewhere, quiz.id)).points, { timeout: 60_000 }).toBe(PAR.expert);
});

test("a proctor of another contract is sent nothing: the device decides while it is older, the page is out of date while it is newer, and everything arrives once they agree", async ({ device }) => {
  test.setTimeout(480_000);
  const quiz = quizOf("demand");
  const name = handle("Versioned");
  const learner = await device("de");
  const sent = sentCalls(learner);
  await contractAway(learner, -1);
  await enter(learner, { kind: "pseudonym", handle: name });
  await expect(connection(learner)).toHaveAttribute("data-tone", "alert", { timeout: 30_000 });
  await expect(connection(learner)).toContainText(/Quiz-Server nicht erreichbar – (?:alles wird )?auf diesem Gerät gespeichert/u);
  await playQuiz(learner, quiz.id);
  for (let index = 0; index < quiz.tasks.length; index++) {
    await openTask(learner, index);
    await answerTask(learner, quiz, "perfect");
  }
  await submitRun(learner);
  expect((await expectFeedback(learner, quiz, "perfect")).points).toBe(PAR.medium);
  expect(sent, "nothing goes to a proctor of an older contract").toEqual([]);

  await contractAway(learner, 1);
  await learner.page.reload();
  const outdated = learner.page.locator("#quiz-outdated");
  await expect(outdated).toContainText("Diese Seite ist älter als der Quiz-Server", { timeout: 30_000 });
  await expect(connection(learner)).toContainText("Seite veraltet");
  await expect(learner.page.locator("[data-layered-overview]")).toBeVisible();
  expect(sent, "nothing goes to a proctor of a newer contract").toEqual([]);

  await contractAway(learner, 0);
  await outdated.getByRole("button", { name: "Seite neu laden" }).click();
  await expect(connection(learner)).toHaveAttribute("data-tone", "calm", { timeout: 120_000 });
  await expect(learner.page.locator("#quiz-outdated")).toHaveCount(0);
  await expect(learner.page.getByRole("alert")).toHaveCount(0);
  expect(sent.length, "once they agree the device hands everything over").toBeGreaterThan(0);

  const elsewhere = await device("de");
  await enter(elsewhere, { kind: "pseudonym", handle: name });
  expect(await learnerId(elsewhere)).toBe(await learnerId(learner));
  await expect.poll(async () => (await shownBest(elsewhere, quiz.id)).points, { timeout: 60_000 }).toBe(PAR.medium);
});
