import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { uiI18n } from "@semio-tech/ui-react";
import Ajv2020 from "ajv/dist/2020";
import { useEffect, useState } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fixture from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { UIFind, UIFindProvider, UISearch, useUIFind } from "../../🟦️.tsx";
import { buildOsCommands } from "../../../🛠️ShellHelpers/🟦️.tsx";

type FixtureItem = (typeof fixture.items)[number];

afterEach(() => {
  cleanup();
  document.documentElement.lang = "en";
  document.documentElement.dataset.appearance = "light";
});

function SearchHarness({ onSelect }: { readonly onSelect: (id: string) => void }) {
  const [open, setOpen] = useState(true);
  const items = fixture.items.map((item: FixtureItem) => ({ ...item, onSelect: () => onSelect(item.id) }));
  return <UISearch items={items} open={open} onOpenChange={setOpen} placeholder={fixture.locales.en.searchPlaceholder} emptyMessage={fixture.locales.en.empty} />;
}

function ReopenableSearchHarness({ onSelect }: { readonly onSelect: (id: string) => void }) {
  const [open, setOpen] = useState(true);
  const items = fixture.items.map((item: FixtureItem) => ({ ...item, onSelect: () => onSelect(item.id) }));
  return <><button type="button" onClick={() => setOpen(true)}>Reopen</button><UISearch items={items} open={open} onOpenChange={setOpen} placeholder={fixture.locales.en.searchPlaceholder} emptyMessage={fixture.locales.en.empty} /></>;
}

function FindSeed() {
  const find = useUIFind();
  useEffect(() => {
    find.setFindItems(fixture.items);
  }, [find.setFindItems]);
  return null;
}

describe("ShellSearch React parity oracle", () => {
  it("resolves canonical built-in command labels from the active React locale", async () => {
    const previous = uiI18n.language || "en";
    try {
      for (const row of fixture.producer.localizedCommands) {
        for (const locale of ["en", "de"] as const) {
          await uiI18n.changeLanguage(locale);
          const command = buildOsCommands([], [], false).find((candidate) => candidate.id === row.id);
          expect(command?.label, `${row.id}:${locale}`).toBe(row.labels[locale]);
        }
      }
    } finally {
      await uiI18n.changeLanguage(previous);
    }
  });

  it("validates the language-neutral fixture and pins the authored token geometry", () => {
    document.documentElement.lang = "en";
    document.documentElement.dataset.appearance = "light";
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    render(<SearchHarness onSelect={vi.fn()} />);
    const dialog = screen.getByRole("dialog", { name: fixture.locales.en.searchTitle });
    const input = document.getElementById(fixture.controls.searchInputId) as HTMLInputElement;
    const list = screen.getByRole("listbox");
    expect(input.getAttribute("role")).toBe(fixture.accessibility.inputRole);
    expect(input.getAttribute("aria-controls")).toBe(list.id);
    expect(input.getAttribute("aria-expanded")).toBe("true");
    expect(screen.getByRole(fixture.accessibility.closeRole, { name: fixture.locales.en.close })).toBeTruthy();
    expect(input.placeholder).toBe(fixture.locales.en.searchPlaceholder);
    expect(dialog.className).toContain("top-[50%]");
    expect(dialog.className).toContain("left-[50%]");
    expect(dialog.className).toContain("sm:max-w-lg");
    expect(dialog.className).toContain("p-0");
    expect(input.className).toContain("h-medium");
    expect(input.parentElement?.className).toContain("h-medium");
    expect(input.parentElement?.className).toContain("gap-single");
    expect(input.parentElement?.className).toContain("px-tiny");
    const commandClass = dialog.querySelector<HTMLElement>("[data-slot=command]")?.className ?? "";
    expect(commandClass).toContain("data-[slot=command-input-wrapper]:h-large");
    expect(commandClass).toContain("[data-slot=command-item]]:py-tiny");
    expect(list.className).toContain("max-h-layout-command");
    expect(screen.getByRole("group", { name: "Panels" }).className).toContain("p-single");
    expect(screen.getByRole("option", { name: fixture.producer.stagedCommand.label }).className).toContain("p-single");
    const publishedIds = screen.getAllByRole("option").map((row) => row.getAttribute("data-command-item-id"));
    expect(publishedIds).toEqual([...fixture.producer.baseOrder, ...fixture.producer.hostSuffixOrder]);
    expect(publishedIds).not.toEqual(expect.arrayContaining(fixture.producer.forbiddenIds));
    expect(fixture.geometry.desktopMaxWidthPx).toBe(512);
    expect(fixture.geometry.inputHeightUiSpacing * fixture.geometry.uiSpacingPx).toBeCloseTo(28.8);
    expect(fixture.geometry.inputPaddingXUiSpacing).toBe(3);
    expect(fixture.geometry.inputGapUiSpacing).toBe(1);
    expect(fixture.geometry.inputIconUiSpacing).toBe(5);
    expect(fixture.geometry.listMaxHeightUiSpacing * fixture.geometry.uiSpacingPx).toBe(300);
  });

  it("publishes an editable combobox relationship and stable active descendant item identity", async () => {
    render(<SearchHarness onSelect={vi.fn()} />);
    const input = document.getElementById(fixture.controls.searchInputId) as HTMLInputElement;
    const list = screen.getByRole("listbox");
    expect(input.getAttribute("aria-controls")).toBe(list.id);
    fireEvent.keyDown(input, { key: "ArrowDown" });
    const activeId = input.getAttribute("aria-activedescendant");
    const active = activeId ? document.getElementById(activeId) : null;
    expect(active?.getAttribute("role")).toBe(fixture.accessibility.itemRole);
    expect(active?.getAttribute("data-command-item-id")).toBe(fixture.producer.baseOrder[1]);
  });

  it("activates only through a completed row click and preserves query across dismissal", async () => {
    const onSelect = vi.fn();
    render(<ReopenableSearchHarness onSelect={onSelect} />);
    const input = document.getElementById(fixture.controls.searchInputId) as HTMLInputElement;
    fireEvent.change(input, { target: { value: fixture.queries[1].query } });
    const row = screen.getByRole("option", { name: fixture.producer.stagedCommand.label });
    fireEvent.pointerDown(row, { button: 0 });
    fireEvent.pointerUp(screen.getByRole("dialog"), { button: 0 });
    expect(onSelect).not.toHaveBeenCalled();
    expect(screen.getByRole("dialog")).toBeTruthy();
    fireEvent.keyDown(input, { key: "Escape" });
    expect(screen.queryByRole("dialog")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Reopen" }));
    expect((document.getElementById(fixture.controls.searchInputId) as HTMLInputElement).value).toBe(fixture.queries[1].query);
    fireEvent.click(screen.getByRole("option", { name: fixture.producer.stagedCommand.label }));
    expect(onSelect).toHaveBeenCalledOnce();
  });

  it("matches JavaScript NFKD plus mark removal for every neutral Unicode case", () => {
    for (const row of fixture.normalization.cases) {
      expect(row.value.normalize(fixture.normalization.form).replace(/\p{M}/gu, "").toLowerCase().trim().slice(0, fixture.normalization.limit)).toBe(row.expected);
    }
  });

  it("focuses, filters, renders groups and empty state, then activates and closes", async () => {
    const onSelect = vi.fn();
    render(<SearchHarness onSelect={onSelect} />);
    const input = document.getElementById(fixture.controls.searchInputId) as HTMLInputElement;
    await waitFor(() => expect(document.activeElement).toBe(input));
    expect(screen.getAllByRole("option").map((row) => row.getAttribute("data-command-item-id"))).toEqual(fixture.queries[0].expectedIds);
    fireEvent.change(input, { target: { value: fixture.queries[1].query } });
    expect(screen.getAllByRole("option").map((row) => row.getAttribute("data-command-item-id"))).toEqual(fixture.queries[1].expectedIds);
    fireEvent.pointerMove(screen.getByRole("option", { name: fixture.producer.stagedCommand.label }));
    fireEvent.keyDown(input, { key: "Enter" });
    expect(onSelect).toHaveBeenCalledWith(fixture.producer.stagedCommand.id);
    expect(screen.queryByRole("dialog", { name: fixture.locales.en.searchTitle })).toBeNull();
  });

  it("publishes the no-result state and the authored Find input", async () => {
    render(<SearchHarness onSelect={vi.fn()} />);
    const search = document.getElementById(fixture.controls.searchInputId) as HTMLInputElement;
    fireEvent.change(search, { target: { value: fixture.queries[2].query } });
    expect(screen.getByRole("status").textContent).toBe(fixture.locales.en.empty);
    cleanup();
    const FindHarness = () => {
      const [open, setOpen] = useState(true);
      return (
        <UIFindProvider>
          <FindSeed />
          <UIFind open={open} onOpenChange={setOpen} placeholder={fixture.locales.en.findPlaceholder} emptyMessage={fixture.locales.en.empty} />
        </UIFindProvider>
      );
    };
    render(<FindHarness />);
    await waitFor(() => expect(document.getElementById(fixture.controls.findInputId)).toBeTruthy());
    expect((document.getElementById(fixture.controls.findInputId) as HTMLInputElement).placeholder).toBe(fixture.locales.en.findPlaceholder);
    expect(screen.getByRole("dialog", { name: fixture.locales.en.findTitle })).toBeTruthy();
  });
});
