/** 🎨️ Chromium verifies semantic chrome paints against the shared theme palettes. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { compile } from "@tailwindcss/node";
import { interactiveActiveFillClass, interactiveOnClass, interactiveTabActiveClass } from "../../../../../../../🔨️modules/🖱️ui/🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { chromium } from "playwright";
import { applyUiThemeToRoot, clearUiThemeFromRoot, resolveThemeAppearancePalettes, semioTheme } from "@semio-tech/ui-styling";
import { describe, expect, it } from "vitest";

const engine = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const fixture = JSON.parse(readFileSync(resolve(engine, "🧫️fixtures/🎨️chrome-palette/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(engine, "🧬️schema/🎨️chrome-palette/🔣️.json"), "utf8"));
const cssPath = resolve(engine, "../../../../🖱️ui/🎨️styling/🖌️ui/🎨️.css");
const css = readFileSync(cssPath, "utf8");

describe("🎨️ Chrome palette projection", () => {
  it("validates the neutral semantic CSS contract", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("uses the semantic active paints for tree, toggle and tab controls", async () => {
    const classes = [interactiveActiveFillClass, interactiveOnClass, interactiveTabActiveClass];
    const compiler = await compile(css, { base: dirname(cssPath), onDependency: () => {} });
    const compiled = compiler.build(classes.flatMap((value) => value.split(" ")));
    const root = document.createElement("section");
    const theme = semioTheme();
    root.className = "semio-scope dark";
    classes.forEach((value, index) => {
      const button = document.createElement("button");
      button.className = value;
      button.dataset.state = index === 2 ? "active" : "on";
      button.style.cssText = "display:block;width:100px;height:32px";
      button.textContent = "control-" + index;
      root.append(button);
    });
    applyUiThemeToRoot(root, theme);
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      await page.setContent(root.outerHTML);
      await page.addStyleTag({ content: compiled });
      const palette = resolveThemeAppearancePalettes(theme, "dark").chrome;
      const expected = (key: string) => "rgb(" + palette[key].slice(0, 3).join(", ") + ")";
      for (let index = 0; index < classes.length; index++) {
        const button = page.getByRole("button", { name: "control-" + index, exact: true });
        expect(await button.evaluate((element) => getComputedStyle(element).color)).toBe(expected("activeForeground"));
        expect(await button.evaluate((element) => getComputedStyle(element).backgroundColor)).toBe(expected("activeBase"));
        await button.hover();
        expect(await button.evaluate((element) => getComputedStyle(element).color)).toBe(expected("activeForeground"));
        expect(await button.evaluate((element) => getComputedStyle(element).backgroundColor)).toBe(expected("activeHover"));
        await page.mouse.move(600, 500);
      }
    } finally {
      await browser.close();
      clearUiThemeFromRoot(root);
    }
  }, 30_000);


  it("keeps nested selected tree labels and icons on the active foreground", async () => {
    const compiler = await compile(css, { base: dirname(cssPath), onDependency: () => {} });
    const compiled = compiler.build(interactiveActiveFillClass.split(" "));
    const globals = readFileSync(resolve(cssPath, "../../../🌐️globals/🎨️.css"), "utf8");
    const root = document.createElement("section");
    root.className = "semio-scope dark";
    root.innerHTML = '<div data-selected="true" data-tree-hover-path="row" data-tree-selection-path="row"><div data-slot="tree-row-layout"><div data-slot="tree-row-content" class="' + interactiveActiveFillClass + '"><button data-slot="tree-label">Curvilinear</button><span data-slot="tree-icon">Icon</span></div><span data-slot="tree-gutter-slot">Guide</span></div><div data-slot="tree-item-content"><div data-slot="tree-row-layout"><div data-slot="tree-row-content"><button data-slot="tree-label">Unselected descendant</button></div></div></div></div>';
    const theme = semioTheme();
    applyUiThemeToRoot(root, theme);
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      await page.setContent(root.outerHTML);
      await page.addStyleTag({ content: compiled + globals });
      const palette = resolveThemeAppearancePalettes(theme, "dark").chrome;
      const expected = "rgb(" + palette.activeForeground.slice(0, 3).join(", ") + ")";
      for (const slot of ["tree-label", "tree-icon"]) {
        expect(await page.locator('[data-selected="true"] > [data-slot="tree-row-layout"] [data-slot="' + slot + '"]').evaluate(el => getComputedStyle(el).color)).toBe(expected);
      }
      expect(await page.getByRole("button", { name: "Unselected descendant" }).evaluate(el => getComputedStyle(el).color)).not.toBe(expected);
      expect(await page.locator('[data-slot="tree-gutter-slot"]').evaluate(el => getComputedStyle(el).color)).not.toBe(expected);
    } finally {
      await browser.close();
      clearUiThemeFromRoot(root);
    }
  }, 30_000);

  it("paints authored light and dark palettes independently and switches appearance without reapplying", async () => {
    const themes = [semioTheme(), { ...structuredClone(semioTheme()), id: "custom.chrome" }];
    for (const appearance of ["light", "dark"] as const) Object.assign(themes[1].appearances[appearance].chrome, fixture.custom[appearance]);
    const roots = themes.map((theme, index) => {
      const root = document.createElement("section");
      root.id = "theme-" + index;
      root.className = "semio-scope";
      root.innerHTML = '<footer data-slot="footer"><button data-slot="toggle-group-item" data-state="on" style="width:100px;height:32px">selected-' + index + '</button></footer>';
      applyUiThemeToRoot(root, theme);
      return root;
    });
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      await page.setContent(roots.map((root) => root.outerHTML).join(""));
      await page.addStyleTag({ content: css });
      for (const appearances of [["light", "dark"], ["dark", "light"], ["light", "light"]] as const) {
        await page.mouse.move(600, 500);
        const observed = await page.evaluate(({ appearances, tokens }) => {
          return [...document.querySelectorAll<HTMLElement>("[data-ui-theme]")].map((root, index) => {
            root.classList.toggle("dark", appearances[index] === "dark");
            return Object.fromEntries(Object.entries(tokens).map(([key, variable]) => {
              const probe = document.createElement("span");
              probe.style.color = "var(" + variable + ")";
              root.append(probe);
              const color = getComputedStyle(probe).color;
              probe.remove();
              const channels = color.match(/[\d.]+/g)!.map(Number);
              return [key, [channels[0], channels[1], channels[2], Math.round((channels[3] ?? 1) * 255)]];
            }));
          });
        }, { appearances, tokens: fixture.tokens as Record<string, string> });
        themes.forEach((theme, index) => {
          const palette = resolveThemeAppearancePalettes(theme, appearances[index]).chrome;
          for (const key of Object.keys(fixture.tokens)) expect(observed[index][key], theme.id + ":" + appearances[index] + ":" + key).toEqual(palette[key]);
        });
        for (const [index, theme] of themes.entries()) {
          const palette = resolveThemeAppearancePalettes(theme, appearances[index]).chrome;
          const button = page.getByRole("button", { name: "selected-" + index, exact: true });
          const expected = (key: string) => "rgb(" + palette[key].slice(0, 3).join(", ") + ")";
          expect(await button.evaluate((element) => getComputedStyle(element).color)).toBe(expected("activeForeground"));
          expect(await button.evaluate((element) => getComputedStyle(element).backgroundColor)).toBe(expected("activeBase"));
          await button.hover();
          expect(await button.evaluate((element) => getComputedStyle(element).backgroundColor)).toBe(expected("activeHover"));
          expect(await button.evaluate((element) => getComputedStyle(element).color)).toBe(expected("activeForeground"));
          await page.mouse.move(600, 500);
        }
      }
    } finally {
      await browser.close();
      for (const root of roots) {
        clearUiThemeFromRoot(root);
        expect(root.getAttribute("style")).toBe("");
        expect(root.dataset.uiTheme).toBeUndefined();
      }
    }
  }, 30_000);
});
