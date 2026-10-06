import { cleanup, fireEvent, render as renderReact, screen, waitFor } from "@testing-library/react";
import { uiI18n, createShellI18nInstance, createShellScope, ShellScopeProvider, useHotkeys } from "@semio-tech/ui-react";
import { createBrowserStoragePort, isShellLocale } from "@semio-tech/framework";
import Ajv2020 from "ajv/dist/2020";
import { useEffect, useState, type ReactNode } from "react";
import { I18nextProvider } from "react-i18next";
import { afterEach, describe, expect, it, vi } from "vitest";
import fixture from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import { UIFind, UIFindProvider, UISearch, useUIFind } from "../../🟦️.tsx";
import { buildOsCommands } from "../../../🛠️ShellHelpers/🟦️.tsx";

type FixtureItem = (typeof fixture.items)[number];

const standaloneI18n = createShellI18nInstance("en");
const render = (children: ReactNode) => renderReact(<I18nextProvider i18n={standaloneI18n}>{children}</I18nextProvider>);

afterEach(async () => {
  cleanup();
  document.documentElement.lang = "en";
  document.documentElement.dataset.appearance = "light";
  await uiI18n.changeLanguage("en");
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

function ScopedSearchHarness({ id, locale, surface }: { readonly id: string; readonly locale: "en" | "de"; readonly surface: "search" | "find" }) {
  const [scope] = useState(() => createShellScope({ shellId: id, storage: createBrowserStoragePort(), initialLocale: locale }));
  return <ShellScopeProvider scope={scope}><div ref={(root) => { scope.rootRef.current = root; }} data-test-root={id} style={{ position: "relative" }}><ScopedSearchContent surface={surface} /><div ref={(layer) => { scope.portalLayerRef.current = layer; }} /></div></ShellScopeProvider>;
}

function ScopedSearchContent({ surface }: { readonly surface: "search" | "find" }) {
  const [open, setOpen] = useState(false);
  useHotkeys(surface === "search" ? "ctrl+p,meta+p" : "ctrl+f,meta+f", () => setOpen(true), { preventDefault: true });
  return <><button type="button">Owned shortcut focus</button><UIFindProvider><FindSeed />{surface === "search" ? <UISearch items={fixture.items.map((item) => ({ ...item, onSelect: () => {} }))} open={open} onOpenChange={setOpen} /> : <UIFind open={open} onOpenChange={setOpen} />}</UIFindProvider></>;
}

describe("ShellSearch React parity oracle", () => {
  it("keeps every neutral embedded shortcut, localized portal and restored focus in its owning root", async () => {
    for (const row of fixture.embedded.cases) {
      cleanup();
      render(<>{fixture.embedded.roots.map((root) => <ScopedSearchHarness key={root.id} id={root.id} locale={root.locale as "en" | "de"} surface={row.surface as "search" | "find"} />)}</>);
      const roots = fixture.embedded.roots.map((root) => document.querySelector<HTMLElement>(`[data-test-root="${root.id}"]`)!);
      const owner = roots[fixture.embedded.roots.findIndex((root) => root.id === row.root)]!;
      const trigger = owner.querySelector<HTMLButtonElement>("button")!;
      trigger.focus();
      fireEvent.keyDown(trigger, { key: row.key, ctrlKey: true });
      await waitFor(() => expect(roots.map((root) => root.querySelectorAll('[role="combobox"]').length)).toEqual(row.opened));
      const input = owner.querySelector<HTMLInputElement>('[role="combobox"]')!;
      expect(document.activeElement).toBe(input);
      expect(document.querySelectorAll(`[id="${input.id}"]`)).toHaveLength(1);
      const portal = input.closest<HTMLElement>('[data-slot="dialog-portal"]')!;
      expect(portal.dataset.dialogIsolation).toBe("scoped");
      const locale = fixture.embedded.roots.find((root) => root.id === row.root)!.locale as "en" | "de";
      expect(input.placeholder).toBe(fixture.locales[locale][row.surface === "search" ? "searchPlaceholder" : "findPlaceholder"]);
      const dialog = input.closest<HTMLElement>('[role="dialog"]')!;
      expect(dialog.style.position).toBe("absolute");
      expect(owner.querySelector(`#${dialog.getAttribute("aria-labelledby")}`)?.textContent).toBe(fixture.locales[locale][row.surface === "search" ? "searchTitle" : "findTitle"]);
      const peer = roots.find((root) => root !== owner)!;
      expect(peer.closest("[inert]")).toBeNull();
      expect(peer.getAttribute("aria-hidden")).toBeNull();
      fireEvent.keyDown(input, { key: "Escape" });
      await waitFor(() => expect(document.querySelector('[role="combobox"]')).toBeNull());
      expect(document.activeElement).toBe(trigger);
      console.log(`[DEBUG] scoped ${locale} ${row.surface} restored owner ${row.root}`);
    }
  });
  it("resolves canonical built-in command labels from the active React locale", async () => {
    const active = uiI18n.language ?? "";
    const previous = isShellLocale(active) ? active : "en";
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
