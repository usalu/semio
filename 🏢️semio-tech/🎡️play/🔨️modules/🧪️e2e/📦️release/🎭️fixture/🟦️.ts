import { test as base, type BrowserContext } from "@playwright/test";
import { releaseRequestUrl } from "../🟦️.ts";

export { expect } from "@playwright/test";
export type { BrowserContext, Page } from "@playwright/test";

/** 🛣️ Installs the production acceptance adapter for exact published CDN origins. */
export async function installPublishedPlayRoutes(context: BrowserContext, origins: Readonly<Record<string, string>>): Promise<void> {
  if (Object.keys(origins).length > 0) {
    await context.route(/^https:\/\/(?:map|media|modules)\.assets\.semio-tech\.com\//, async route => {
      const url = releaseRequestUrl(route.request().url(), origins);
      if (url === undefined) throw new Error(`Unmapped published asset: ${route.request().url()}`);
      const response = await route.fetch({ url });
      await route.fulfill({ response });
    });
  }
}

/** 🎭 Exercises baked CDN URLs through the corresponding emitted static page. */
export const test = base.extend<{ publishedPages: void }>({
  publishedPages: [async ({ context }, use) => {
    const origins = JSON.parse(process.env.PLAY_E2E_RELEASE_ORIGINS ?? "{}") as Record<string, string>;
    await installPublishedPlayRoutes(context, origins);
    await use();
  }, { auto: true }],
});
