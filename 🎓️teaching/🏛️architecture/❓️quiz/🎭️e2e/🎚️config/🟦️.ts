/** 🎚️ Playwright configuration of the end-to-end gate. The gate boots the stack and passes its site origin as
 * `PLAYWRIGHT_BASE_URL`; Playwright starts no server. The projects run in this order:
 * - `boot` loads the site once, so a cold dev server has transformed and optimized everything before the others start;
 * - `desktop` (every test beside every other) and `phone` run side by side on one proctor, so every assertion there
 *   holds whoever else is learning;
 * - `presence`, then `shortage`, then `away`, run alone afterwards: they count who is online, take the proctor away
 *   for a moment, and take it away before a learner ever arrives;
 * - `pets` runs last and nothing waits for it: the companions are decoration, so they never hold up another project.
 *
 * Four workers is what one topology gets: a page of the quiz keeps a processor core busy while a test drives it, and
 * more workers than cores for them only make every test slower. The gate gives each of two topologies running at once
 * fewer (`--workers`). Every step waits at most twenty seconds for what it expects, so a hang fails at once; the limit of
 * a whole test only bounds the longest journeys (every screen in both languages: two minutes on an idle machine, five
 * beside other builds).
 * @see ../🟦️.ts — the gate that boots the stacks and runs this configuration
 * @see ../../🧪️tests — the specs
 * https://playwright.dev/docs/test-projects#dependencies */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "@playwright/test";

const site = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const baseURL = process.env.PLAYWRIGHT_BASE_URL;
if (!baseURL) throw new Error("Run the end-to-end gate through `bun nx run @teaching/architecture-quiz:test-e2e`; it boots the stack these specs need");

/** 🖥️ The desktop window every spec but the phone's opens. */
export const DESKTOP_VIEWPORT = { width: 1440, height: 900 } as const;

/** 📱️ The phone the phone spec holds. */
export const PHONE_VIEWPORT = { width: 375, height: 812 } as const;

export default defineConfig({
  testDir: resolve(site, "🧪️tests"),
  fullyParallel: false,
  forbidOnly: true,
  retries: 0,
  workers: 4,
  timeout: 600_000,
  expect: { timeout: 20_000 },
  reporter: [["list"]],
  use: { baseURL, viewport: DESKTOP_VIEWPORT, trace: "retain-on-failure", screenshot: "only-on-failure", actionTimeout: 20_000, navigationTimeout: 60_000 },
  projects: [
    { name: "boot", testMatch: ["🚀️site-boot/🟦️.ts"] },
    { name: "desktop", dependencies: ["boot"], fullyParallel: true, testMatch: ["🪪️first-visit/🟦️.ts", "🥞️layered-home/🟦️.ts", "🎯️quiz-runs/🟦️.ts", "🏆️live-leaderboard/🟦️.ts", "🗣️both-languages/🟦️.ts"] },
    { name: "phone", dependencies: ["boot"], testMatch: ["📱️phone/🟦️.ts"], use: { viewport: PHONE_VIEWPORT, isMobile: true, hasTouch: true } },
    { name: "presence", dependencies: ["desktop", "phone"], testMatch: ["👥️shared-presence/🟦️.ts"] },
    { name: "shortage", dependencies: ["presence"], testMatch: ["🔌️connection-shortage/🟦️.ts"] },
    { name: "away", dependencies: ["shortage"], testMatch: ["📴️proctor-away/🟦️.ts"] },
    { name: "pets", dependencies: ["away"], fullyParallel: true, testMatch: ["🐕️pet-walk/🟦️.ts"] },
  ],
});
