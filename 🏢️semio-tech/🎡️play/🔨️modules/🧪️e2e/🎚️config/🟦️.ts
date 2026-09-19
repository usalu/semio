// #region 🧲️Header
// 💻️ 🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🎚️config/🟦️.ts
// Specs: Run Playwright acceptance coverage against a live play dev server.
// Summary: Nx prepares an isolated continuous service; the E2E consumer validates its generation
// before supplying PLAYWRIGHT_BASE_URL. Playwright consumes that server and does not start one.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";

import { playwrightTestTimeoutMs } from "@semio-tech/repo-lib";
// #endregion 🔌️Adapters

const playDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const playwrightTimeoutMs = playwrightTestTimeoutMs();
if (!process.env.PLAYWRIGHT_BASE_URL) throw new Error("Run the play test-e2e target through Nx");
const baseURL = process.env.PLAYWRIGHT_BASE_URL.endsWith("/") ? process.env.PLAYWRIGHT_BASE_URL : `${process.env.PLAYWRIGHT_BASE_URL}/`;

/** @emoji 🖥️ Map and wgpu panes need a real adapter; `PLAY_E2E_GPU=swiftshader` forces the software stack for hosts without one. */
function browserLaunchArgs(): readonly string[] {
  if (process.env.PLAY_E2E_GPU === "swiftshader") return ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"];
  return [process.platform === "darwin" ? "--use-angle=metal" : "--use-angle=gl", "--enable-gpu", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"];
}

export default defineConfig({
  testDir: resolve(playDir, "🧪️tests"),
  testMatch: ["🎭️acceptance/🟦️.ts"],
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  timeout: playwrightTimeoutMs,
  expect: { timeout: Math.min(playwrightTimeoutMs, 120_000) },
  workers: 1,
  reporter: [["list"]],
  use: { baseURL, trace: "on-first-retry" },
  projects: [{ name: "chromium", use: { ...devices["Desktop Chrome"], launchOptions: { args: [...browserLaunchArgs()] } } }],
});
