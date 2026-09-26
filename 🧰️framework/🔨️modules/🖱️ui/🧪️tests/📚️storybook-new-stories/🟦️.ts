// #region 🧲️Header
// 💻️ .storybook/ui-new-stories.spec.ts
// Specs: End-to-end smoke coverage for the "ui" scope's newly authored stories (PanelTabBar/PanelChromeTabBar, Scene, SelectionMarquee, Shell*Panel/Dialog, Skeletons, SortableTreeItems, UIDialog, UIIntroduction, UnifiedGumball, NavbarExampleSelect, ActionDropdown, Providers).
// Summary: Drives each story id through the aggregated Storybook static build's `iframe.html` and asserts a mounted root plus zero page/console errors — no visual/pixel assertions, this is boot-health coverage, not per-component behavior testing (each component's own barrel-level unit tests already cover behavior).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Page } from "@playwright/test";
import dockAxisGeometry from "../../🧫️fixtures/📐️dock-axis-geometry/🔣️.json" with { type: "json" };

const NEW_UI_STORY_IDS: readonly string[] = [
  "🖱️ui⚛️react-mobilepanel--default",
  "🖱️ui⚛️react-mobilepanel--hidden",
  "🖱️ui⚛️react-paneltabbar--panel-variant",
  "🖱️ui⚛️react-paneltabbar--mobile-variant",
  "🖱️ui⚛️react-paneltabbar--chrome-hosted",
  "🖱️ui⚛️react-scene--default",
  "🖱️ui⚛️react-scene--orthographic",
  "🖱️ui⚛️react-scene--without-chrome",
  "🖱️ui⚛️react-selectionmarquee--rect-full",
  "🖱️ui⚛️react-selectionmarquee--rect-partial",
  "🖱️ui⚛️react-selectionmarquee--polygon",
  "🖱️ui⚛️react-shelldisplaypanel--default",
  "🖱️ui⚛️react-shelldisplaypanel--compact-on",
  "🖱️ui⚛️react-shellsettingspanel--default",
  "🖱️ui⚛️react-shellsettingspanel--expert-mode",
  "🖱️ui⚛️react-skeletons--table",
  "🖱️ui⚛️react-skeletons--diagram",
  "🖱️ui⚛️react-skeletons--scene",
  "🖱️ui⚛️react-sortabletreeitems--default",
  "🖱️ui⚛️react-sortabletreeitems--single-item",
  "🖱️ui⚛️react-uidialog--staged-form",
  "🖱️ui⚛️react-uidialog--confirm-only",
  "🖱️ui⚛️react-uiintroduction--first-step",
  "🖱️ui⚛️react-uiintroduction--last-step",
  "🖱️ui⚛️react-unifiedgumball--move-rotate-scale",
  "🖱️ui⚛️react-unifiedgumball--move-only",
  "🖱️ui⚛️react-navbarexampleselect--default",
  "🖱️ui⚛️react-navbarexampleselect--without-no-example-option",
  "🖱️ui⚛️react-actiondropdown--default",
  "🖱️ui⚛️react-actiondropdown--with-transaction",
  "🖱️ui⚛️react-providers--ghost",
  "🖱️ui⚛️react-providers--interaction",
  "🖱️ui⚛️react-providers--panel-dock-story",
  "🖱️ui⚛️react-providers--transaction",
  "🖱️ui⚛️react-providers--tree-state",
  "🖱️ui⚛️react-providers--chrome-compact",
  "🖱️ui⚛️react-providers--chrome-label-policy",
  "🖱️ui⚛️react-providers--flow",
  "🖱️ui⚛️react-providers--glass-tier",
  "🖱️ui⚛️react-mode--content-through-glass",
  "🖱️ui⚛️react-mode--geometry-oracle",
];

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function expectStoryMounts(page: Page, storyId: string): Promise<void> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto(`iframe.html?id=${storyId}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForFunction(() => {
    const root = document.querySelector("#storybook-root");
    return !!root && root.childElementCount > 0;
  });

  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
}

for (const storyId of NEW_UI_STORY_IDS) {
  test(`ui scope story "${storyId}" mounts with no console/page errors`, async ({ page }) => {
    await expectStoryMounts(page, storyId);
  });
}

test("Mode reserves themed separators before distributing nested React panel extents", async ({ page }) => {
  await expectStoryMounts(page, "🖱️ui⚛️react-mode--geometry-oracle");
  const geometry = await page.evaluate(() => {
    const oracle = document.querySelector('[data-testid="dock-axis-geometry-oracle"]');
    const modeBody = oracle?.querySelector('[data-slot="mode-body"]');
    const groups = modeBody ? Array.from(modeBody.querySelectorAll<HTMLElement>('[data-slot="resizable-panel-group"]')) : [];
    if (!(oracle instanceof HTMLElement) || !(modeBody instanceof HTMLElement) || groups.length !== 2) return null;
    const rect = (element: Element) => {
      const value = element.getBoundingClientRect();
      const origin = oracle.getBoundingClientRect();
      return { x: value.x - origin.x, y: value.y - origin.y, width: value.width, height: value.height };
    };
    const parts = (group: HTMLElement) => Array.from(group.children).filter((child): child is HTMLElement => child instanceof HTMLElement && (child.dataset.slot === "resizable-panel" || child.dataset.slot === "resizable-handle"));
    const root = parts(groups[0]!);
    const nested = parts(groups[1]!);
    const rootHandle = root.find((part) => part.dataset.slot === "resizable-handle");
    const nestedHandle = nested.find((part) => part.dataset.slot === "resizable-handle");
    const rootPanels = root.filter((part) => part.dataset.slot === "resizable-panel");
    const nestedPanels = nested.filter((part) => part.dataset.slot === "resizable-panel");
    const bodies = Array.from(modeBody.querySelectorAll<HTMLElement>('[data-slot="mode-dock-stack-body"]'));
    if (!rootHandle || !nestedHandle || rootPanels.length !== 2 || nestedPanels.length !== 2 || bodies.length !== 3) return null;
    const bodyStyle = getComputedStyle(modeBody);
    const stackBodyPadding = bodies.map((body) => {
      const style = getComputedStyle(body);
      return {
        inline: [style.paddingLeft, style.paddingRight].map(Number.parseFloat),
        block: [style.paddingTop, style.paddingBottom].map(Number.parseFloat),
        clearance: [style.getPropertyValue("--window-silhouette-top-clearance"), style.getPropertyValue("--window-silhouette-bottom-clearance")].map(Number.parseFloat),
        margin: [style.marginTop, style.marginBottom].map(Number.parseFloat),
      };
    });
    return {
      viewport: rect(oracle),
      canvasPadding: [bodyStyle.paddingLeft, bodyStyle.paddingRight, bodyStyle.paddingTop, bodyStyle.paddingBottom].map(Number.parseFloat),
      rootGroup: rect(groups[0]!),
      nestedGroup: rect(groups[1]!),
      rootPanels: rootPanels.map(rect),
      nestedPanels: nestedPanels.map(rect),
      rootHandle: rect(rootHandle),
      nestedHandle: rect(nestedHandle),
      stackBodyPadding,
    };
  });
  expect(geometry).not.toBeNull();
  const close = (actual: number, expected: number) => expect(actual).toBeCloseTo(expected, 1);
  const token = geometry!.rootHandle.width;
  const rootWeights = dockAxisGeometry.layout.children.map((child) => child.weight);
  const nested = dockAxisGeometry.layout.children[1]!.node;
  if (nested.kind !== "column" || nested.children === undefined) throw new Error("dock geometry fixture must carry the nested column oracle");
  const nestedWeights = nested.children.map((child) => child.weight);
  expect(token).toBeGreaterThan(0);
  close(geometry!.viewport.width, dockAxisGeometry.viewport.width);
  close(geometry!.viewport.height, dockAxisGeometry.viewport.height);
  close(geometry!.nestedHandle.height, token);
  geometry!.canvasPadding.forEach((padding) => close(padding, token));
  for (const padding of geometry!.stackBodyPadding) {
    padding.inline.forEach((value) => close(value, token));
    padding.block.forEach((value, index) => {
      close(value, padding.clearance[index]!);
      close(value + padding.margin[index]!, 0);
    });
  }
  close(geometry!.rootGroup.width + token * 2, geometry!.viewport.width);
  close(geometry!.rootGroup.height + token * 2, geometry!.viewport.height);
  close(geometry!.rootPanels[0]!.width + token + geometry!.rootPanels[1]!.width, geometry!.rootGroup.width);
  close(geometry!.nestedPanels[0]!.height + token + geometry!.nestedPanels[1]!.height, geometry!.nestedGroup.height);
  expect(geometry!.rootPanels[0]!.width / (geometry!.rootGroup.width - token)).toBeCloseTo(rootWeights[0]! / rootWeights.reduce((sum, weight) => sum + weight, 0), 3);
  expect(geometry!.nestedPanels[0]!.height / (geometry!.nestedGroup.height - token)).toBeCloseTo(nestedWeights[0]! / nestedWeights.reduce((sum, weight) => sum + weight, 0), 3);
  close(geometry!.rootPanels[1]!.x - (geometry!.rootPanels[0]!.x + geometry!.rootPanels[0]!.width), token);
  close(geometry!.nestedPanels[1]!.y - (geometry!.nestedPanels[0]!.y + geometry!.nestedPanels[0]!.height), token);
});

test("Panel chrome measures its trailing drag handle inside the physical tab width", async ({ page }) => {
  await expectStoryMounts(page, "🖱️ui⚛️react-providers--panel-dock-story");
  const geometry = await page.evaluate(() => {
    const button = document.querySelector<HTMLElement>('[data-slot="panel-tab-button"]');
    const label = button?.firstElementChild;
    const handle = button?.querySelector<HTMLElement>('[data-slot="drag-handle"]');
    const leadingIcon = label?.querySelector("svg");
    if (!button || !(label instanceof HTMLElement) || !handle || !(leadingIcon instanceof SVGElement)) return null;
    const style = getComputedStyle(button);
    const width = (element: Element) => element.getBoundingClientRect().width;
    return {
      button: width(button),
      label: width(label),
      handle: width(handle),
      leadingIcon: width(leadingIcon),
      gap: Number.parseFloat(style.columnGap),
      padding: Number.parseFloat(style.paddingLeft) + Number.parseFloat(style.paddingRight),
      border: Number.parseFloat(style.borderLeftWidth) + Number.parseFloat(style.borderRightWidth),
      handleRight: handle.getBoundingClientRect().right,
      contentRight: button.getBoundingClientRect().right - Number.parseFloat(style.paddingRight) - Number.parseFloat(style.borderRightWidth),
    };
  });
  expect(geometry).not.toBeNull();
  expect(geometry!.handle).toBeGreaterThan(0);
  expect(geometry!.handle).toBeCloseTo(dockAxisGeometry.panelChrome.dragHandlePixels, 2);
  expect(geometry!.leadingIcon).toBeCloseTo(dockAxisGeometry.panelChrome.leadingIconPixels, 2);
  expect(geometry!.button).toBeCloseTo(geometry!.label + geometry!.gap + geometry!.handle + geometry!.padding + geometry!.border, 1);
  expect(geometry!.handleRight).toBeCloseTo(geometry!.contentRight, 1);
});

test("celebrated panel tab icons paint conic ink through --icon-mask, not a rectangular fill", async ({ page }) => {
  await expectStoryMounts(page, "🖱️ui⚛️react-paneltabbar--panel-variant");
  const paint = await page.evaluate(() => {
    const tab = document.querySelector('[data-slot="panel-tab-button"]') as HTMLElement | null;
    if (!tab) return null;
    tab.setAttribute("data-celebrated", "true");
    const icon = tab.querySelector("[data-icon]") as HTMLElement | null;
    const label = tab.querySelector('[data-slot="inline-label"]') as HTMLElement | null;
    if (!icon || !label) return null;
    const iconStyle = getComputedStyle(icon);
    const labelStyle = getComputedStyle(label);
    const beforeStyle = getComputedStyle(icon, "::before");
    return {
      maskImage: iconStyle.maskImage,
      backgroundImage: iconStyle.backgroundImage,
      beforeBackgroundImage: beforeStyle.backgroundImage,
      labelBackgroundImage: labelStyle.backgroundImage,
      labelBackgroundClip: labelStyle.backgroundClip,
      labelWebkitTextFillColor: labelStyle.webkitTextFillColor,
    };
  });
  expect(paint).not.toBeNull();
  expect(paint!.maskImage.startsWith('url("data:image/svg+xml')).toBe(true);
  expect(paint!.backgroundImage).toContain("conic-gradient");
  expect(paint!.beforeBackgroundImage === "none" || paint!.beforeBackgroundImage === "").toBe(true);
  expect(paint!.labelBackgroundImage).toContain("conic-gradient");
  expect(paint!.labelBackgroundClip).toBe("text");
  expect(paint!.labelWebkitTextFillColor === "rgba(0, 0, 0, 0)" || paint!.labelWebkitTextFillColor === "transparent").toBe(true);
});

// #region 🪟️SilhouetteAccessibilityFallbacks
type SilhouetteFallbackPaint = {
  readonly chipBackdrop: string;
  readonly chipBackground: string;
  readonly gapBackdrop: string;
  readonly gapBackground: string;
};

async function readSilhouetteFallbackPaint(page: Page): Promise<SilhouetteFallbackPaint | null> {
  return page.evaluate(() => {
    const chip = document.querySelector("[data-window-silhouette-chip]");
    const gap = document.querySelector("[data-window-silhouette-gap]");
    if (!(chip instanceof HTMLElement) || !(gap instanceof HTMLElement)) return null;
    const chipStyle = getComputedStyle(chip);
    const gapStyle = getComputedStyle(gap);
    return {
      chipBackdrop: chipStyle.backdropFilter || chipStyle.getPropertyValue("-webkit-backdrop-filter"),
      chipBackground: chipStyle.backgroundColor,
      gapBackdrop: gapStyle.backdropFilter || gapStyle.getPropertyValue("-webkit-backdrop-filter"),
      gapBackground: gapStyle.backgroundColor,
    };
  });
}

test("window silhouette keeps its gap transparent in reduced-transparency mode", async ({ page, context }) => {
  const cdp = await context.newCDPSession(page);
  await cdp.send("Emulation.setEmulatedMedia", { features: [{ name: "prefers-reduced-transparency", value: "reduce" }] });
  await expectStoryMounts(page, "🖱️ui⚛️react-mode--content-through-glass");
  const paint = await readSilhouetteFallbackPaint(page);
  expect(paint).not.toBeNull();
  expect(paint!.chipBackdrop).toBe("none");
  expect(paint!.chipBackground).not.toBe("rgba(0, 0, 0, 0)");
  expect(paint!.gapBackdrop).toBe("none");
  expect(paint!.gapBackground).toBe("rgba(0, 0, 0, 0)");
});

test("window silhouette uses system paint without filling its gap in forced-colors mode", async ({ page }) => {
  await page.emulateMedia({ forcedColors: "active" });
  await expectStoryMounts(page, "🖱️ui⚛️react-mode--content-through-glass");
  const paint = await readSilhouetteFallbackPaint(page);
  expect(paint).not.toBeNull();
  expect(paint!.chipBackdrop).toBe("none");
  expect(paint!.chipBackground).not.toBe("rgba(0, 0, 0, 0)");
  expect(paint!.gapBackdrop).toBe("none");
  expect(paint!.gapBackground).toBe("rgba(0, 0, 0, 0)");
});
// #endregion 🪟️SilhouetteAccessibilityFallbacks
