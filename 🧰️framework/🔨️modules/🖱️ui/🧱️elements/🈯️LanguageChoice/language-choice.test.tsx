import * as React from "react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { LanguageChoice } from "./🟦️.tsx";

afterEach(cleanup);

interface Case { readonly name: string; readonly options: readonly (readonly [string, string])[]; readonly value: string | null; readonly checked: readonly boolean[]; readonly tabStops: number }
const { cases } = JSON.parse(readFileSync(resolve(import.meta.dirname, "language-choice.cases.json"), "utf8")) as { cases: Case[] };

describe("LanguageChoice", () => {
  for (const c of cases) {
    it(`renders one radio per language in its own words: ${c.name}`, () => {
      render(<LanguageChoice label="Language" options={c.options.map(([value, endonym]) => ({ value, endonym }))} value={c.value ?? undefined} onChoose={() => undefined} />);
      const group = screen.getByRole("radiogroup", { name: "Language" });
      expect(group.hasAttribute("data-shared-border-frame")).toBe(true);
      const radios = screen.getAllByRole("radio");
      expect(radios.map((radio) => radio.textContent)).toEqual(c.options.map(([, endonym]) => endonym));
      expect(radios.map((radio) => radio.getAttribute("lang"))).toEqual(c.options.map(([value]) => value));
      expect(radios.map((radio) => radio.getAttribute("aria-checked") === "true")).toEqual([...c.checked]);
      expect(radios.filter((radio) => radio.tabIndex === 0)).toHaveLength(c.tabStops);
      expect(group.querySelectorAll("[data-shared-border-cell]")).toHaveLength(c.options.length);
    });
  }

  it("preselects nothing and reports the chosen language, never a press-off", () => {
    const onChoose = vi.fn();
    render(<LanguageChoice label="Language" options={[{ value: "en", endonym: "English" }, { value: "de", endonym: "Deutsch" }]} onChoose={onChoose} />);
    expect(screen.getAllByRole("radio").some((radio) => radio.getAttribute("aria-checked") === "true")).toBe(false);
    fireEvent.click(screen.getByRole("radio", { name: "Deutsch" }));
    expect(onChoose).toHaveBeenCalledExactlyOnceWith("de");
  });

  it("does not report the already chosen language again", () => {
    const onChoose = vi.fn();
    render(<LanguageChoice label="Language" options={[{ value: "en", endonym: "English" }, { value: "de", endonym: "Deutsch" }]} value="en" onChoose={onChoose} />);
    fireEvent.click(screen.getByRole("radio", { name: "English" }));
    expect(onChoose).not.toHaveBeenCalled();
  });
});
