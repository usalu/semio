import type { Browser, LaunchOptions } from "playwright";

/** 🌐️ Launches Chromium for tests through the new headless mode (`channel: "chromium"`): the legacy headless shell never delivers wheel or key input and cannot start WebGPU or downloads on macOS, while the full binary behaves the same on every platform. Playwright loads lazily so suites without a browser stay cheap. Every ui and os browser test launches through here, so the mode is decided once. */
export async function launchChromium(options: Omit<LaunchOptions, "channel"> = {}): Promise<Browser> {
  const { chromium } = await import("playwright");
  return chromium.launch({ headless: true, ...options, channel: "chromium" });
}

/** 🩺️ Whether the Playwright Chromium binary is provisioned, so a suite can skip instead of failing without a browser. */
export async function chromiumProvisioned(): Promise<boolean> {
  const { chromium } = await import("playwright");
  const { existsSync } = await import("node:fs");
  return existsSync(chromium.executablePath());
}
