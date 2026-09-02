// #region 🔌️Adapters
import * as React from "react";
import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut, rankCommandValue } from "../🟦️.tsx";
// #endregion 🔌️Adapters

// #region ⌨️CommandMatrix
afterEach(() => cleanup());

describe("Command", () => {
  it("folds Turkish-sensitive I forms without inheriting the host locale", () => {
    const localeSensitive = "Istanbul"
      .normalize("NFKD")
      .replace(/[\u0300-\u036f]/g, "")
      .toLocaleLowerCase("tr");
    expect(localeSensitive).toBe("ıstanbul");
    expect(localeSensitive).not.toBe("istanbul");
    expect(rankCommandValue("Istanbul", "istanbul")).toBe(10_000);
    expect(rankCommandValue("İSTANBUL", "istanbul")).toBe(10_000);
    expect(rankCommandValue("ıstanbul", "istanbul")).toBe(0);
  });

  it("filters with deterministic normalized ranking and preserves hidden items in place", async () => {
    const { getByRole, container } = render(
      <Command>
        <CommandInput defaultValue="cafe" aria-label="Find command" />
        <CommandList>
          <CommandEmpty>No commands</CommandEmpty>
          <CommandItem value="Decaf café">Decaf</CommandItem>
          <CommandItem value="Café" keywords={["coffee"]}>
            Exact
          </CommandItem>
          <CommandItem value="Tea">Tea</CommandItem>
        </CommandList>
      </Command>,
    );
    const input = getByRole("combobox", { name: "Find command" });
    await waitFor(() => expect(input.getAttribute("aria-activedescendant")).toBe(getByRole("option", { name: "Exact" }).id));
    expect((container.querySelector('[data-value="Tea"]') as HTMLElement).hidden).toBe(true);
    expect((container.querySelector('[data-value="Decaf café"]') as HTMLElement).hidden).toBe(false);
    expect(document.querySelector('[data-slot="command-empty"]')).toHaveProperty("hidden", true);
    expect(rankCommandValue("Café", "cafe")).toBeGreaterThan(rankCommandValue("Decaf café", "cafe"));
  });

  it("owns uncontrolled query and empty state while shouldFilter=false exposes host-ranked rows", async () => {
    const view = render(
      <Command>
        <CommandInput aria-label="Filter" />
        <CommandList>
          <CommandEmpty>Nothing found</CommandEmpty>
          <CommandItem value="alpha">Alpha</CommandItem>
        </CommandList>
      </Command>,
    );
    fireEvent.change(view.getByRole("combobox"), { target: { value: "zzz" } });
    await waitFor(() => expect((view.container.querySelector('[data-value="alpha"]') as HTMLElement).hidden).toBe(true));
    expect(view.getByRole("status", { hidden: true }).hidden).toBe(false);
    view.rerender(
      <Command shouldFilter={false}>
        <CommandInput aria-label="Filter" value="zzz" onValueChange={() => undefined} />
        <CommandList>
          <CommandEmpty>Nothing found</CommandEmpty>
          <CommandItem value="alpha">Alpha</CommandItem>
        </CommandList>
      </Command>,
    );
    await waitFor(() => expect(view.getByRole("option", { name: "Alpha" }).hidden).toBe(false));
  });

  it("keeps controlled query and selected value authoritative during parent lag", async () => {
    const queryChange = vi.fn();
    const valueChange = vi.fn();
    const select = vi.fn();
    const { getByRole, container } = render(
      <Command value="alpha" onValueChange={valueChange}>
        <CommandInput aria-label="Filter" value="a" onValueChange={queryChange} />
        <CommandList>
          <CommandItem value="alpha" onSelect={select}>
            Alpha
          </CommandItem>
          <CommandItem value="beta">Beta</CommandItem>
        </CommandList>
      </Command>,
    );
    const input = getByRole("combobox") as HTMLInputElement;
    fireEvent.change(input, { target: { value: "z" } });
    expect(queryChange).toHaveBeenCalledWith("z");
    expect(input.value).toBe("a");
    expect(getByRole("option", { name: "Alpha" }).hidden).toBe(false);
    fireEvent.keyDown(input, { key: "Enter" });
    expect(valueChange).toHaveBeenCalledWith("alpha");
    expect(select).toHaveBeenCalledTimes(1);
    expect(container.querySelector('[data-slot="command"]')?.getAttribute("data-value")).toBe("alpha");
  });

  it("navigates ranked enabled options with stable duplicate-label IDs, pages, and loop", async () => {
    const select = vi.fn();
    const { getByRole, getAllByRole } = render(
      <Command loop>
        <CommandInput aria-label="Navigate" />
        <CommandList>
          <CommandItem value="first" disabled>
            Duplicate
          </CommandItem>
          <CommandItem value="second" onSelect={select}>
            Duplicate
          </CommandItem>
          {Array.from({ length: 6 }, (_, index) => (
            <CommandItem key={index} value={`row-${index}`}>
              Row {index}
            </CommandItem>
          ))}
        </CommandList>
      </Command>,
    );
    const input = getByRole("combobox");
    const duplicates = getAllByRole("option", { name: "Duplicate" });
    expect(duplicates[0]!.id).not.toBe(duplicates[1]!.id);
    await waitFor(() => expect(input.getAttribute("aria-activedescendant")).toBe(duplicates[1]!.id));
    fireEvent.keyDown(input, { key: "End" });
    expect(getByRole("option", { name: "Row 5" }).getAttribute("aria-selected")).toBe("true");
    fireEvent.keyDown(input, { key: "ArrowDown" });
    expect(duplicates[1]!.getAttribute("aria-selected")).toBe("true");
    fireEvent.keyDown(input, { key: "PageDown" });
    expect(getByRole("option", { name: "Row 4" }).getAttribute("aria-selected")).toBe("true");
    fireEvent.keyDown(input, { key: "Home" });
    fireEvent.keyDown(input, { key: "Enter" });
    expect(select).toHaveBeenCalledTimes(1);
  });

  it("does not navigate or activate while an IME composition is active", async () => {
    const first = vi.fn();
    const second = vi.fn();
    const { getByRole } = render(
      <Command>
        <CommandInput aria-label="Compose" />
        <CommandList>
          <CommandItem value="first" onSelect={first}>
            First
          </CommandItem>
          <CommandItem value="second" onSelect={second}>
            Second
          </CommandItem>
        </CommandList>
      </Command>,
    );
    const input = getByRole("combobox");
    await waitFor(() => expect(input.getAttribute("aria-activedescendant")).toBe(getByRole("option", { name: "First" }).id));
    fireEvent.keyDown(input, { key: "ArrowDown", isComposing: true });
    fireEvent.keyDown(input, { key: "Enter", isComposing: true });
    expect(getByRole("option", { name: "First" }).getAttribute("aria-selected")).toBe("true");
    expect(first).not.toHaveBeenCalled();
    expect(second).not.toHaveBeenCalled();
  });

  it("activates pointer selections once and respects a consumer's prevented pointer-down", () => {
    const select = vi.fn();
    const ownedPointerSelect = vi.fn();
    const { getByRole } = render(
      <Command shouldFilter={false}>
        <CommandList>
          <CommandItem value="normal" onSelect={select}>
            Normal
          </CommandItem>
          <CommandItem
            value="host"
            onPointerDown={(event) => {
              event.preventDefault();
              ownedPointerSelect();
            }}
            onSelect={select}
          >
            Host owned
          </CommandItem>
        </CommandList>
      </Command>,
    );
    fireEvent.pointerMove(getByRole("option", { name: "Normal" }));
    fireEvent.click(getByRole("option", { name: "Normal" }));
    expect(select).toHaveBeenCalledTimes(1);
    fireEvent.pointerDown(getByRole("option", { name: "Host owned" }));
    fireEvent.click(getByRole("option", { name: "Host owned" }));
    expect(ownedPointerSelect).toHaveBeenCalledTimes(1);
    expect(select).toHaveBeenCalledTimes(1);
  });

  it("hides empty groups and authored-hidden options without unmounting them", async () => {
    const { getByRole, container } = render(
      <Command>
        <CommandInput aria-label="Groups" defaultValue="visible" />
        <CommandList>
          <CommandGroup heading="Hidden group">
            <CommandItem value="missing">Missing</CommandItem>
          </CommandGroup>
          <CommandGroup heading="Visible group">
            <CommandItem value="visible">Visible</CommandItem>
            <CommandItem value="visible secret" hidden>
              Secret
            </CommandItem>
          </CommandGroup>
        </CommandList>
      </Command>,
    );
    await waitFor(() => expect((container.querySelector('[data-slot="command-group"][hidden]') as HTMLElement).textContent).toContain("Hidden group"));
    expect(getByRole("group", { name: "Visible group" }).hidden).toBe(false);
    expect((container.querySelector('[data-value="visible secret"]') as HTMLElement).hidden).toBe(true);
  });

  it("composes shortcuts and focus lifecycle through the owned Dialog", async () => {
    const outside = document.createElement("button");
    outside.textContent = "Before palette";
    document.body.appendChild(outside);
    outside.focus();
    const changes = vi.fn();
    const { getByRole } = render(
      <CommandDialog defaultOpen onOpenChange={changes} title={"Commands" as never} description="Choose one" showCloseButton={false}>
        <CommandInput aria-label="Palette query" />
        <CommandList>
          <CommandItem value="save">
            Save <CommandShortcut>⌘S</CommandShortcut>
          </CommandItem>
        </CommandList>
      </CommandDialog>,
    );
    const input = getByRole("combobox", { name: "Palette query" });
    expect(document.activeElement).toBe(input);
    expect(getByRole("dialog", { name: "Commands" }).contains(input)).toBe(true);
    expect(document.querySelector('[data-slot="command-shortcut"]')?.getAttribute("aria-hidden")).toBe("true");
    fireEvent.keyDown(document, { key: "Escape" });
    await waitFor(() => expect(document.querySelector('[role="dialog"]')).toBeNull());
    expect(changes).toHaveBeenCalledWith(false);
    expect(document.activeElement).toBe(outside);
    outside.remove();
  });
});
// #endregion ⌨️CommandMatrix
