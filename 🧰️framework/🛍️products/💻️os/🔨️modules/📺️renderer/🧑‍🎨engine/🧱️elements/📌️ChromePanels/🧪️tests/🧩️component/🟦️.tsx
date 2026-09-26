import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createFrameworkSettingsPanelTab, keybindingCaptureStepV1, type KeybindingCaptureKeyV1, type SettingsHostApi, themeAlphaInput, themeContrastBadgeText, themeContrastRatioFormatter, themeNumberInputRow, themeTextInputRow, type ConflictsHostApi } from "../../🟦️.tsx";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import userEvent from "@testing-library/user-event";
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

/** ⌨️ LAW over `🧫️fixtures/⌨️keybinding-capture/🔣️.json`: a capture waits through the modifiers a chord starts with,
 * records the completed chord in the dispatcher's spelling and ends only on Escape or a chord. Oracle: `user-event`
 * types the same chords on a real focused button, modifiers first, exactly as a hand does (ticket 26/09/23 U5). */
describe("keybinding capture", () => {
  const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/⌨️keybinding-capture/🔣️.json"), "utf8")) as {
    readonly rows: readonly { readonly id: string; readonly keys: readonly Partial<KeybindingCaptureKeyV1>[]; readonly expected: readonly string[] }[];
  };
  const outcome = (step: ReturnType<typeof keybindingCaptureStepV1>): string => (step.kind === "chord" ? `chord:${step.chord}` : step.kind);

  it("waits through every modifier and records the chord the fixture names", () => {
    expect(fixture.rows.length).toBeGreaterThanOrEqual(7);
    for (const row of fixture.rows) {
      const steps = row.keys.map((key) => outcome(keybindingCaptureStepV1({ key: "", ctrlKey: false, metaKey: false, altKey: false, shiftKey: false, ...key })));
      expect(steps, row.id).toEqual(row.expected);
    }
  });

  it("records ⌃⌥K typed by user-event on a focused capture button, modifiers first", async () => {
    const seen: string[] = [];
    const { getByRole } = render(<button type="button" onKeyDown={(event) => seen.push(outcome(keybindingCaptureStepV1(event.nativeEvent)))}>record</button>);
    getByRole("button", { name: "record" }).focus();
    await userEvent.keyboard("{Control>}{Alt>}k{/Alt}{/Control}");
    expect(seen).toEqual(fixture.rows.find((row) => row.id === "ctrl-alt-k")!.expected);
    seen.length = 0;
    await userEvent.keyboard("{Meta>}{Shift>}Z{/Shift}{/Meta}");
    expect(seen).toEqual(fixture.rows.find((row) => row.id === "meta-shift-z")!.expected);
  });
});

/** ⌨️ Settings ▸ Hotkeys names each bound control in the live language: an app action or OS command by the name the host
 * resolves from its manifest label, a shell control by its chrome bundle entry, and only an id nothing names by its
 * humanized segment (the rows used to be humanized ids for everything — "Undo" in a German shell; ticket 26/09/23 U5). */
describe("keybinding row names", () => {
  it("names app actions through the host, shell controls through the chrome bundles, and humanizes only the rest", () => {
    const host = {
      controlKeybindings: new Map([["undo", "mod+z"], ["ui.fullscreen.toggle", "mod+ctrl+f"], ["plugin.frobnicateWidget", "mod+alt+w"]]),
      controlKeybindingLabel: (controlId: string) => (controlId === "undo" ? "Rückgängig" : null),
      keybindingCaptureControlId: null,
      setKeybindingCaptureControlId: vi.fn(),
      setKeybindingOverride: vi.fn(),
      resetKeybindingOverride: vi.fn(),
      locks: {},
    } as unknown as SettingsHostApi;
    const panel = createFrameworkSettingsPanelTab(() => host);
    const leaf = panel.children.find((child): child is PanelTabLeaf => child.kind === "leaf" && child.id === "framework.settings.keybindings")!;
    const source = leaf.trees[0]!.tree;
    if (!("resolveTree" in source)) throw new Error("the keybindings tab must carry a lazily resolved tree");
    const rows = (source.resolveTree() as { sections: { items: { id: string; label: string }[] }[] }).sections[0]!.items;
    const labels = Object.fromEntries(rows.map((row) => [row.id.slice("framework.settings.keybindings.".length), String(row.label)]));
    expect(labels.undo).toBe("Rückgängig");
    expect(labels["ui.fullscreen.toggle"]).not.toBe("Toggle");
    expect(labels["ui.fullscreen.toggle"]?.length).toBeGreaterThan(0);
    expect(labels["plugin.frobnicateWidget"]).toBe("Frobnicate Widget");
  });
});
