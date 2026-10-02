import type { SvgVideoBrowserV1 } from "../🟦️.ts";

export interface ChromiumSvgVideoRuntimeV1 {
  readonly openBrowser: () => Promise<SvgVideoBrowserV1>;
  close(): Promise<void>;
}

/** 🌐️Shares one Chromium process while each export owns and closes its isolated frame session. */
export async function createChromiumSvgVideoRuntimeV1(): Promise<ChromiumSvgVideoRuntimeV1> {
  const tooling = await import("playwright") as unknown as { chromium: { launch(options: { headless: boolean }): Promise<{ newContext(): Promise<SvgVideoBrowserV1>; close(): Promise<void> }> } };
  const browser = await tooling.chromium.launch({ headless: true });
  return Object.freeze({ openBrowser: () => browser.newContext(), close: () => browser.close() });
}
