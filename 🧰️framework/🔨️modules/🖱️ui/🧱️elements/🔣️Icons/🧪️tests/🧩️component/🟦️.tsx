/** 🗂️ Named chrome glyphs follow shared vectors and native SVG parsing without loading the rich icon codec. */
import { act, cleanup, render } from "@testing-library/react";
import { ICONS } from "@semio-tech/assets";
import { activeUiTheme, setActiveUiTheme } from "@semio-tech/ui-styling";
import { renderToStaticMarkup } from "react-dom/server";
import { afterEach, describe, expect, it } from "vitest";
import { CatalogIcon, Icon, decodeIcon, type IconName, type IconSizeToken } from "../../🟦️.tsx";
import { Icon as ChromeIcon } from "@semio-tech/ui-react/chrome";
import { uiDataLabel } from "../../../🎗️UiLabel/🟦️.tsx";
import vectors from "../🧫️fixtures/🗂️catalog-glyphs/🔣️.json";

const savedTheme = activeUiTheme();
afterEach(() => { cleanup(); setActiveUiTheme(savedTheme); });

describe("named catalog chrome icons", () => {
  it("exports the named renderer at the slim chrome boundary", () => {
    expect(ChromeIcon).toBe(CatalogIcon);
    expect(ChromeIcon).not.toBe(Icon);
  });
  for (const vector of vectors.catalog) it(`renders ${vector.icon} with the same SVG, sizing, accessible title and mask as the full renderer`, () => {
    const props = { icon: vector.icon as IconName, size: vector.size as IconSizeToken, title: uiDataLabel(vector.title) };
    const { container } = render(<CatalogIcon {...props} />);
    const reference = new DOMParser().parseFromString(renderToStaticMarkup(<Icon {...props} />), "text/html").body.firstElementChild!;
    const actual = container.firstElementChild!;
    expect(actual.querySelector("svg")!.outerHTML).toBe(reference.querySelector("svg")!.outerHTML);
    for (const name of ["class", "data-icon", "data-icon-kind", "aria-hidden", "title"]) expect(actual.getAttribute(name)).toBe(reference.getAttribute(name));
    expect((actual as HTMLElement).style.getPropertyValue("--icon-mask")).toBe((reference as HTMLElement).style.getPropertyValue("--icon-mask"));
    expect(actual.querySelector("svg")).not.toBeNull();
    expect(actual.getAttribute("data-icon")).toBe(vector.canonical);
    expect(actual.getAttribute("title")).toBe(vector.title);
    expect(actual.getAttribute("aria-hidden")).toBeNull();
    expect(actual.classList.contains(vector.class)).toBe(true);
    expect((actual as HTMLElement).style.getPropertyValue("--icon-mask")).toContain("data:image/svg+xml");
  });
  for (const vector of vectors.boxes) it(`renders a decorative ${vector.icon} glyph with numeric size and caller classes`, () => {
    const props = { icon: vector.icon as IconName, size: vector.size, className: vector.className };
    const { container } = render(<CatalogIcon {...props} />);
    const actual = container.firstElementChild as HTMLElement;
    const reference = new DOMParser().parseFromString(renderToStaticMarkup(<Icon {...props} />), "text/html").body.firstElementChild as HTMLElement;
    expect(actual.getAttribute("aria-hidden")).toBe(vector.hidden);
    expect(actual.getAttribute("title")).toBeNull();
    expect(actual.classList.contains(vector.className)).toBe(true);
    expect(actual.className).toBe(reference.className);
    expect(actual.style.width).not.toBe("");
    expect(actual.style.width).toBe(actual.style.height);
    expect(actual.style.width).toBe(reference.style.width);
    expect(actual.style.height).toBe(reference.style.height);
    expect(actual.querySelector("svg")!.outerHTML).toBe(reference.querySelector("svg")!.outerHTML);
  });
  for (const vector of vectors.themes) it(`updates the mounted ${vector.icon} glyph and mask when the theme ${vector.id} changes`, () => {
    const icon = vector.icon as IconName;
    const { container } = render(<CatalogIcon icon={icon} />);
    const actual = container.firstElementChild as HTMLElement;
    const previous = actual.style.getPropertyValue("--icon-mask");
    const icons = vector.svg === "" ? { ...savedTheme.icons, aliases: { ...savedTheme.icons?.aliases, [icon]: vector.alias } } : { ...savedTheme.icons, variants: { ...savedTheme.icons?.variants, [icon]: vector.svg } };
    act(() => setActiveUiTheme({ ...savedTheme, icons }));
    const markup = vector.svg || ICONS[vector.alias as IconName];
    const reference = new DOMParser().parseFromString(markup, "text/html").querySelector("svg")!;
    expect(actual.querySelector("svg")!.outerHTML).toBe(reference.outerHTML);
    expect(actual.style.getPropertyValue("--icon-mask")).not.toBe(previous);
    expect(actual.getAttribute("data-icon")).toBe(icon);
    expect(actual.getAttribute("aria-hidden")).toBe("true");
    const full = new DOMParser().parseFromString(renderToStaticMarkup(<Icon icon={icon} />), "text/html").body.firstElementChild as HTMLElement;
    expect(actual.querySelector("svg")!.outerHTML).toBe(full.querySelector("svg")!.outerHTML);
    expect(actual.style.getPropertyValue("--icon-mask")).toBe(full.style.getPropertyValue("--icon-mask"));
  });
  for (const vector of vectors.rich) it(`retains the explicit rich ${vector.kind} renderer`, () => {
    const { container } = render(<Icon icon={decodeIcon(vector.icon)!} />);
    expect(container.firstElementChild!.getAttribute("data-icon-kind")).toBe(vector.kind);
    expect(container.textContent).toBe(vector.text);
  });
});
