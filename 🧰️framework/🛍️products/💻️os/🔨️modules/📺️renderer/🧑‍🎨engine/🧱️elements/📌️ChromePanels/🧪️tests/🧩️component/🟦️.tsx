import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createFrameworkSettingsPanelTab, themeAlphaInput, themeContrastBadgeText, themeContrastRatioFormatter, themeNumberInputRow, themeTextInputRow, type ConflictsHostApi } from "../../🟦️.tsx";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import type { PanelTabLeaf } from "@semio-tech/ui-react";
import type { Rgba8 } from "@semio-tech/ui-styling";
import type { ReactElement } from "react";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

describe("ChromePanels theme inputs", () => {
  it("renders and lazily commits canonical text values", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const commit = vi.fn();
    const row = themeTextInputRow("theme.spacing.compact", "compact", "0.25rem", commit);
    const { getByRole } = render(<>{row.control}</>);
    const input = getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("0.25rem");
    fireEvent.change(input, { target: { value: "0.5rem" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith("0.5rem");
    expect(consoleError).not.toHaveBeenCalled();
  });

  it("renders and lazily commits canonical scalar and list numbers", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const commit = vi.fn();
    const row = themeNumberInputRow("theme.metrics.padding", "padding", [1, 2.5], commit);
    const { getByRole } = render(<>{row.control}</>);
    const input = getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("1, 2.5");
    fireEvent.change(input, { target: { value: "3, 4.5" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith([3, 4.5]);
    expect(consoleError).not.toHaveBeenCalled();
  });

  it("renders and lazily commits clamped appearance alpha", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const commit = vi.fn();
    const { getByRole } = render(themeAlphaInput("theme.appearance.alpha", 0.25, commit));
    const input = getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("0.25");
    fireEvent.change(input, { target: { value: "1.2" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith(1);
    expect(consoleError).not.toHaveBeenCalled();
  });
});

describe("♿️ theme contrast live warning", () => {
  const grade = (value: string) => `grade:${value}`;
  const warning = (options: { readonly ratio: string; readonly minimum: string; readonly counterpart: string }) => `warn ${options.ratio} < ${options.minimum} with ${options.counterpart}`;
  const palette: Record<string, Rgba8> = { base: [255, 255, 255, 255], foreground: [0, 0, 0, 255], mutedForeground: [119, 119, 119, 255], panel: [255, 255, 255, 255], accent: [0, 0, 0, 255], accentForeground: [0, 0, 0, 255], borderNormal: [0, 0, 0, 255] };

  it("prints the measured ratio in the ACTIVE locale's number format — no language is assumed", () => {
    expect(themeContrastRatioFormatter("en")(4.5)).toBe("4.50");
    expect(themeContrastRatioFormatter("de")(4.5)).toBe("4,50");
    expect(themeContrastRatioFormatter("de")(21)).toBe("21,00");
  });

  it("a pair below WCAG AA carries an inline warning naming the other paint of the pair", () => {
    const verdict = themeContrastBadgeText(palette, "mutedForeground", grade, themeContrastRatioFormatter("de"), warning)!;
    expect(verdict.passesBodyText).toBe(false);
    expect(verdict.grade).toBe("aaLarge");
    expect(verdict.counterpart).toBe("base");
    expect(verdict.text).toBe("4,48:1 · grade:aaLarge");
    expect(verdict.warning).toBe("warn 4,48 < 4,50 with base");
  });

  it("measures text against its OWN surface, never text against text, and says nothing for a border", () => {
    const accent = themeContrastBadgeText(palette, "accentForeground", grade, themeContrastRatioFormatter("en"), warning)!;
    expect(accent.counterpart).toBe("accent");
    expect(accent.ratio).toBe(1);
    expect(themeContrastBadgeText(palette, "foreground", grade, themeContrastRatioFormatter("en"), warning)!.counterpart).not.toBe("accentForeground");
    expect(themeContrastBadgeText(palette, "borderNormal", grade, themeContrastRatioFormatter("en"), warning)).toBeNull();
  });
});

describe("Inline Tree resolution controls", () => {
  it("keeps both labeled resolution buttons available before selecting the conflict", () => {
    const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "..", "..");
    const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🎛️inline-tree-controls", "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🎛️inline-tree-controls", "🔣️.json"), "utf8"));
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const resolve = vi.fn();
    const host: ConflictsHostApi = {
      conflicts: [{ id: fixture.conflict.id, kind: { kind: "quarantined", envelopes: [] }, status: "open", messages: [], actors: [], timestamp: { actor: 1, physical_ms: 0, logical: 0 } }],
      locale: "en",
      selectedConflictId: null,
      kindLabel: () => "Quarantined",
      messageText: () => fixture.conflict.message,
      onSelect: vi.fn(),
      onResolve: resolve,
      currentDocumentText: "",
    };
    const panel = createFrameworkSettingsPanelTab(() => null, undefined, () => host);
    const leaf = panel.children.find((child): child is PanelTabLeaf => child.kind === "leaf" && child.id === "framework.settings.conflicts")!;
    const source = leaf.trees[0]!.tree;
    // 🌲️ The conflicts tab resolves its tree lazily (`staticTreePanelDefinition`'s twin), which is
    // the half of `TreePanelSource` this law is about: a resolved config would not re-read the host.
    if (!("resolveTree" in source)) throw new Error("the conflicts tab must carry a lazily resolved tree");
    const tree = source.resolveTree() as { sections: { items: { id: string; control: ReactElement; items?: unknown[] }[] }[] };
    const rows = tree.sections[0]!.items;
    expect(rows).toHaveLength(fixture.expected.rowCount);
    expect(rows[0]!.id).toBe(fixture.rowId);
    expect(rows[0]!.items).toBeUndefined();
    const { container } = render(<>{rows[0]!.control}</>);
    const buttons = Array.from(container.querySelectorAll("button"));
    expect(buttons).toHaveLength(fixture.controls.length);
    for (const [index, expected] of fixture.controls.entries()) {
      const button = buttons[index]!;
      expect(button.id).toBe(`${fixture.rowId}.${expected.suffix}`);
      expect(button.textContent).toContain(expected.label);
      fireEvent.click(button);
      expect(resolve).toHaveBeenLastCalledWith(fixture.conflict.id, expected.suffix);
    }
    expect(container.querySelector(".flex.items-center")?.children).toHaveLength(2);
  });
});
