/** 💬️ The hints of an easy run as the client shows and speaks them, in English and in German, with the guesses of the German client. */

import { type ReactElement } from "react";
import { cleanup, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { computeAccessibleDescription } from "dom-accessibility-api";
import { afterEach, describe, expect, it, vi } from "vitest";
import { type Answer, type Hint, type MatchingAnswer, type SheetClassificationTask, type SheetMatchingTask, type SortingAnswer } from "@semio-tech/quiz";
import { ClassificationTaskView, MatchingTaskView, SortingTaskView, classificationHintText, formatScore, hintTerm, quizText } from "@semio-tech/quiz-react";
import { TaskView } from "@semio-tech/quiz-react/modules/run/.tsx";
import { CLIMATES, en, HINT_QUESTIONS, hinted, HOUSES, LAMPS, MASSES, NBSP, PLACES, POWERS, TEMPERATURES, text } from "../🪞️challenge-views-kit/🟦️.ts";

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

describe("💡️ hints of an easy run", () => {
  for (const { locale, task, hint, question } of HINT_QUESTIONS) {
    it(`asks “${question}” beside the item it doubts, with a question mark in a circle, describing that item's controls`, () => {
      const answer: Answer =
        task.kind === "sorting"
          ? { kind: "sorting", order: task.items.map((item) => item.id) }
          : task.kind === "matching"
            ? { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item, index) => [item.id, index]))])) }
            : { kind: "classification", assignments: { desert: "wet", fjord: "wet", steppe: "dry" } };
      render(<TaskView task={task} answer={answer} hints={[hint]} onAnswer={() => undefined} text={quizText(locale)} locale={locale} />);
      const notes = document.querySelectorAll<HTMLElement>(".quiz-hint");
      expect(notes).toHaveLength(1);
      const note = notes[0]!;
      expect(note.getAttribute("data-hint")).toBe(hint.kind);
      expect(note.querySelector(".quiz-hint-question")?.textContent).toBe(question);
      expect([note.querySelector(".quiz-hint-symbol")?.textContent, note.querySelector(".quiz-hint-symbol")?.getAttribute("aria-hidden")]).toEqual(["?", "true"]);
      expect(note.closest("li")?.getAttribute("data-presence-anchor")).toBe(`item:${hint.item}`);
      const label = (task.items.find((item) => item.id === hint.item)?.label ?? text("", ""))[locale];
      const controls = task.kind === "sorting" ? within(note.closest("li")!).getAllByRole("button") : within(note.closest("li")!).getAllByRole("combobox");
      expect(controls.length).toBeGreaterThan(0);
      for (const control of controls) expect(computeAccessibleDescription(control), label).toBe(question);
      expect(document.body.textContent).not.toMatch(/≪|≫|far too|viel zu|wrong category|falschen Kategorie|as much as|ganze|\{\{|quiz\.[a-z]+\.[a-zA-Z]+/u);
      expect(question).not.toMatch(/\d{7}|\d[,.]\d{3}[,.]\d{3}|-\d/u);
    });
  }

  it("leaves every other item's controls undescribed", () => {
    render(<SortingTaskView task={MASSES} answer={{ kind: "sorting", order: ["horse", "mouse", "cat"] }} hints={[{ kind: "compare", item: "horse", other: "mouse", factor: 200, verdict: "under" }]} onAnswer={() => undefined} text={en} locale="en" />);
    for (const name of ["Move Mouse up", "Move Cat down"]) expect(screen.getByRole("button", { name }).getAttribute("aria-describedby")).toBeNull();
  });

  it("names things in its question by their short forms while the rows, bins and controls keep their labels", () => {
    const { unmount } = render(<SortingTaskView task={POWERS} answer={{ kind: "sorting", order: ["tealight", "kettle", "sun"] }} hints={[{ kind: "compare", item: "sun", other: "tealight", factor: 1.1e26, verdict: "under" }]} onAnswer={() => undefined} text={en} locale="en" />);
    expect(screen.getByRole("button", { name: "Move Total radiant power of the Sun up" })).toBeTruthy();
    expect(document.querySelector(".quiz-hint-question")?.textContent).toBe(`Are you sure 1.1${NBSP}×${NBSP}10²⁶ × “Tea light” together only add up to the power of 1 × “Sun”?`);
    unmount();
    render(<ClassificationTaskView task={CLIMATES} answer={{ kind: "classification", assignments: { desert: "wet" } }} hints={[{ kind: "category", item: "desert", category: "wet" }]} onAnswer={() => undefined} text={en} locale="en" />);
    expect(screen.getByRole("heading", { name: "Wet climate zone" })).toBeTruthy();
    expect(computeAccessibleDescription(screen.getByRole("combobox", { name: "Category for Hot desert of the Sahara" }))).toBe("Are you sure “Desert” belongs to Wet — Rain on most days?");
  });

  it("names the quantity on a matching of several columns, so one pair asks a different question in each", () => {
    const answer: MatchingAnswer = { kind: "matching", assignments: { load: { old: 0, passive: 1 }, energy: { old: 0, passive: 1 }, indoor: { old: 0, passive: 1 } } };
    const hints: readonly Hint[] = [
      { kind: "compare", item: "old", other: "passive", dimension: "load", factor: 0.083, verdict: "reversed" },
      { kind: "compare", item: "old", other: "passive", dimension: "energy", factor: 0.0375, verdict: "reversed" },
    ];
    const { rerender } = render(<MatchingTaskView task={HOUSES} answer={answer} hints={[]} onAnswer={() => undefined} text={en} locale="en" />);
    rerender(<MatchingTaskView task={HOUSES} answer={answer} hints={hints} onAnswer={() => undefined} text={en} locale="en" />);
    expect(hinted()).toBe("New hints: 2");
    expect(["Specific heating load of Unrenovated 1960s single-family house", "Annual heating energy of Unrenovated 1960s single-family house"].map((name) => computeAccessibleDescription(screen.getByRole("combobox", { name })))).toEqual([
      "Are you sure “Passive house” is higher in heating load than “Old house”?",
      "Are you sure “Passive house” is higher in heating energy than “Old house”?",
    ]);
  });

  it("writes a quantity's or an axis's name lower-case mid-sentence in English unless its second letter is a capital, and as it is in German", () => {
    const names = [
      [text("Heating demand", "Heizwärmebedarf"), "heating demand", "Heizwärmebedarf"],
      [text("U-value", "U-Wert"), "U-value", "U-Wert"],
      [text("CO₂ emissions", "CO₂-Emissionen"), "CO₂ emissions", "CO₂-Emissionen"],
      [text("PV yield", "PV-Ertrag"), "PV yield", "PV-Ertrag"],
      [text("Ärger", "Ärger"), "ärger", "Ärger"],
      [text("Power", "Leistung"), "power", "Leistung"],
      [text("n", "n"), "n", "n"],
    ] as const;
    for (const [label, english, german] of names) expect([hintTerm({ label }, "en"), hintTerm({ label }, "de")], label.en).toEqual([english, german]);
    expect(hintTerm({ label: text("Specific cooling load", "Spezifische Kühllast"), short: text("Cooling load", "Kühllast") }, "en")).toBe("cooling load");
  });

  it("asks the profile questions the second hint audit quotes with the axis lower-case in English", () => {
    const profiles: SheetClassificationTask = {
      kind: "classification",
      id: "profiles",
      title: text("Profiles", "Profile"),
      prompt: text("Assign each standard its profile.", "Ordne jedem Standard sein Profil zu."),
      axes: [{ id: "heating", label: text("Heating demand", "Heizwärmebedarf"), unit: "kWh/(m²·a)", min: 0, max: 400 }],
      categories: [
        { id: "a", label: text("Profile A", "Profil A"), profile: { heating: 15 } },
        { id: "b", label: text("Profile B", "Profil B"), profile: { heating: 303 } },
      ],
      items: [
        { id: "enev", label: text("New build to EnEV 2014", "Neubau nach EnEV 2014") },
        { id: "passive", label: text("Passive house", "Passivhaus") },
      ],
    };
    const asked = (hint: Hint): readonly string[] => (["en", "de"] as const).map((locale) => classificationHintText(profiles, hint, quizText(locale), locale));
    expect(asked({ kind: "profile", item: "enev", category: "b", axis: "heating" })).toEqual([
      `Are you sure “New build to EnEV 2014” fits Profile B, with heating demand at about 303${NBSP}kWh/(m²·a)?`,
      `Bist du sicher, dass „Neubau nach EnEV 2014“ zu Profil B passt, mit Heizwärmebedarf bei rund 303${NBSP}kWh/(m²·a)?`,
    ]);
    expect(asked({ kind: "profile", item: "enev", category: "b", axis: "heating", other: "passive", above: true })).toEqual([
      "Are you sure “New build to EnEV 2014” is higher in heating demand than “Passive house”?",
      "Bist du sicher, dass „Neubau nach EnEV 2014“ in puncto Heizwärmebedarf höher ist als „Passivhaus“?",
    ]);
  });

  it("reads every key with its place, never as the value of the item standing there", () => {
    render(<SortingTaskView task={MASSES} answer={{ kind: "sorting", order: ["horse", "mouse", "cat"] }} onAnswer={() => undefined} text={en} locale="en" />);
    const rows = within(screen.getByRole("list", { name: "Order by Mass" })).getAllByRole("listitem");
    expect(rows.map((row) => row.querySelector(".quiz-sort-place .sr-only")?.textContent)).toEqual([`Place 1: 20${NBSP}g`, `Place 2: 4${NBSP}kg`, `Place 3: 500${NBSP}kg`]);
    for (const row of rows) {
      expect(row.querySelector(".quiz-sort-key")?.getAttribute("aria-hidden")).toBe("true");
      expect(row.querySelector(".quiz-sort-place")?.getAttribute("aria-hidden")).toBeNull();
    }
  });

  it("speaks a hint once when it appears, not those a task already has, and again when it comes back", () => {
    const answer: SortingAnswer = { kind: "sorting", order: ["horse", "mouse", "cat"] };
    const low: readonly Hint[] = [{ kind: "compare", item: "horse", other: "mouse", factor: 200, verdict: "under" }];
    const view = (hints: readonly Hint[]): ReactElement => <SortingTaskView task={MASSES} answer={answer} hints={hints} onAnswer={() => undefined} text={en} locale="en" />;
    const shown = render(view(low));
    expect(hinted()).toBe("");
    shown.unmount();
    const { rerender } = render(view([]));
    expect(hinted()).toBe("");
    rerender(view(low));
    const spoken = "Are you sure 200 × “Mouse” together only add up to the mass of 1 × “Horse”?";
    expect(hinted()).toBe(spoken);
    const first = screen.getAllByRole("status")[1]!.firstElementChild;
    rerender(view([...low]));
    expect(screen.getAllByRole("status")[1]!.firstElementChild).toBe(first);
    rerender(view([]));
    rerender(view(low));
    expect(hinted()).toBe(spoken);
    expect(screen.getAllByRole("status")[1]!.firstElementChild).not.toBe(first);
    expect(screen.getAllByRole("status")[0]!.textContent).toBe("");
  });

  it("speaks several hints that appear at once only by their count, leaving their questions beside their items", () => {
    const answer: SortingAnswer = { kind: "sorting", order: ["horse", "cat", "mouse"] };
    const view = (hints: readonly Hint[]): ReactElement => <SortingTaskView task={MASSES} answer={answer} hints={hints} onAnswer={() => undefined} text={en} locale="en" />;
    const { rerender } = render(view([]));
    const three: readonly Hint[] = [
      { kind: "compare", item: "horse", other: "cat", factor: 0.008, verdict: "over" },
      { kind: "compare", item: "cat", other: "mouse", factor: 25000, verdict: "over" },
      { kind: "compare", item: "mouse", other: "cat", factor: 125, verdict: "over" },
    ];
    rerender(view(three));
    expect(hinted()).toBe("New hints: 3");
    expect(document.querySelectorAll(".quiz-hint")).toHaveLength(3);
    rerender(view(three.slice(0, 1)));
    rerender(view(three.slice(0, 2)));
    expect(hinted()).toBe("Are you sure it takes 25,000 × “Mouse” to add up to the mass of 1 × “Cat”?");
  });

  it("asks again when a hint's question changes — another reference or another claimed factor — and not when it stays", () => {
    const answer: MatchingAnswer = { kind: "matching", assignments: { power: { led: 2, halogen: 1, floodlight: 0 } } };
    const view = (hints: readonly Hint[]): ReactElement => <MatchingTaskView task={LAMPS} answer={answer} hints={hints} onAnswer={() => undefined} text={en} locale="en" />;
    const { rerender } = render(view([]));
    rerender(view([{ kind: "compare", item: "led", other: "halogen", dimension: "power", factor: 250, verdict: "over" }]));
    expect(hinted()).toBe("Are you sure it takes 250 × “Halogen spot” to add up to the power of 1 × “LED bulb”?");
    const first = screen.getAllByRole("status")[1]!.firstElementChild;
    rerender(view([{ kind: "compare", item: "led", other: "halogen", dimension: "power", factor: 250, verdict: "over" }]));
    expect(screen.getAllByRole("status")[1]!.firstElementChild).toBe(first);
    rerender(view([{ kind: "compare", item: "led", other: "floodlight", dimension: "power", factor: 40, verdict: "over" }]));
    expect(hinted()).toBe("Are you sure it takes 40 × “Floodlight” to add up to the power of 1 × “LED bulb”?");
    expect(screen.getByRole("combobox", { name: "Power of Floodlight" }).getAttribute("aria-describedby")).toBeNull();
  });

  it("draws the profiles of a classification whose keys are hidden as shapes, with no descriptions and no numbers", () => {
    render(<ClassificationTaskView task={PLACES} answer={undefined} onAnswer={() => undefined} text={en} locale="en" />);
    const image = screen.getByRole("img", { name: "Spider diagram of Dry" });
    const percent = (share: number): string => formatScore(share, "en");
    expect(document.getElementById(image.getAttribute("aria-describedby") ?? "")?.textContent).toBe(`Rain: ${percent(0.05)}; Sun: ${percent(0.9)}; Heat: ${percent(0.8)}. The values follow as a table.`);
    expect(screen.queryByText("Minimum")).toBeNull();
  });
});

describe("🇩🇪 hints and guesses in German", () => {
  const de = quizText("de");
  const raw = /\{\{|quiz\.[a-z]+\.[a-zA-Z]+/u;

  it("speaks a classification question in German once it appears, and several new ones by their count, each beside its item", () => {
    const answer = { kind: "classification", assignments: { desert: "wet", fjord: "wet", steppe: "wet" } } as const;
    const view = (hints: readonly Hint[]): ReactElement => <ClassificationTaskView task={CLIMATES} answer={answer} hints={hints} onAnswer={() => undefined} text={de} locale="de" />;
    const { rerender } = render(view([]));
    rerender(view([{ kind: "profile", item: "desert", category: "wet", axis: "sun" }]));
    expect(hinted()).toBe(`Bist du sicher, dass „Wüste“ zu Nass passt, mit Sonnenschein bei rund 1.200${NBSP}h?`);
    rerender(view([{ kind: "profile", item: "desert", category: "wet", axis: "sun", other: "fjord", above: false }]));
    expect(hinted()).toBe("Bist du sicher, dass „Wüste“ in puncto Sonnenschein niedriger ist als „Fjord“?");
    rerender(view([{ kind: "profile", item: "desert", category: "wet", axis: "sun", other: "fjord", above: false }, { kind: "group", item: "fjord", other: "steppe", together: true }, { kind: "category", item: "steppe", category: "wet" }]));
    expect(hinted()).toBe("Neue Hinweise: 2");
    expect([...document.querySelectorAll(".quiz-hint")].map((note) => [note.closest("li")?.getAttribute("data-quiz-item"), note.querySelector(".quiz-hint-question")?.textContent])).toEqual([
      ["desert", "Bist du sicher, dass „Wüste“ in puncto Sonnenschein niedriger ist als „Fjord“?"],
      ["fjord", "Bist du sicher, dass „Fjord“ und „Steppe“ in dieselbe Kategorie gehören?"],
      ["steppe", "Bist du sicher, dass „Steppe“ zu Nass gehört — Regen an den meisten Tagen?"],
    ]);
    expect(document.body.textContent).not.toMatch(raw);
  });

  it("speaks a reversed pair of a matching of several columns in German by its order alone, naming the quantity, with the question mark beside it", () => {
    const answer: MatchingAnswer = { kind: "matching", assignments: { load: { old: 0, passive: 1 }, energy: { old: 1, passive: 0 }, indoor: { old: 0, passive: 1 } } };
    const view = (hints: readonly Hint[]): ReactElement => <MatchingTaskView task={HOUSES} answer={answer} hints={hints} onAnswer={() => undefined} text={de} locale="de" />;
    const { rerender } = render(view([]));
    rerender(view([{ kind: "compare", item: "passive", other: "old", dimension: "load", factor: 12, verdict: "reversed" }]));
    const question = "Bist du sicher, dass „Passivhaus“ in puncto Heizlast höher ist als „Altbau“?";
    expect(hinted()).toBe(question);
    const note = document.querySelector(".quiz-hint")!;
    expect([note.getAttribute("data-hint"), note.querySelector(".quiz-hint-symbol")?.textContent, note.querySelector(".quiz-hint-question")?.textContent]).toEqual(["compare", "?", question]);
    expect(computeAccessibleDescription(screen.getByRole("combobox", { name: "Spezifische Heizlast von Passivhaus" }))).toBe(question);
    expect(document.body.textContent).not.toMatch(raw);
  });

  it("names every guess field in German, with its example in the unit, and refuses an unreadable guess with a German message", async () => {
    const user = userEvent.setup();
    const { keys: _masses, ...masses } = MASSES;
    const { keys: _temperatures, ...temperatures } = TEMPERATURES;
    const lamps: SheetMatchingTask = { ...LAMPS, dimensions: LAMPS.dimensions.map(({ cards: _cards, ...dimension }) => dimension) };
    const cases = [
      { view: <SortingTaskView task={masses} answer={undefined} onAnswer={() => undefined} text={de} locale="de" />, field: "Schätzwert für Pferd", example: `2${NBSP}kg`, invalid: `Gib eine positive Zahl ein, optional mit SI-Vorsatz und Einheit, zum Beispiel 2${NBSP}kg.`, guide: "Gib für jedes Element deinen Schätzwert ein, zum Beispiel 2 kg. Die Elemente ordnen sich nach deinen Schätzwerten." },
      { view: <SortingTaskView task={temperatures} answer={undefined} onAnswer={() => undefined} text={de} locale="de" />, field: "Schätzwert für Eis", example: `2${NBSP}K`, invalid: `Gib eine Zahl ein, optional mit SI-Vorsatz und Einheit, zum Beispiel 2${NBSP}K.`, guide: "Gib für jedes Element deinen Schätzwert ein, zum Beispiel 2 K. Die Elemente ordnen sich nach deinen Schätzwerten." },
      { view: <MatchingTaskView task={lamps} answer={undefined} onAnswer={() => undefined} text={de} locale="de" />, field: "Schätzwert für Leistung von LED-Lampe", example: `2${NBSP}kW`, invalid: `Gib eine positive Zahl ein, optional mit SI-Vorsatz und Einheit, zum Beispiel 2${NBSP}kW.`, guide: "Gib für jedes Element und jede Größe deinen Schätzwert ein; jedes Feld zeigt ein Beispiel in seiner Einheit." },
    ];
    for (const { view, field, example, invalid, guide } of cases) {
      const shown = render(view);
      expect(screen.getByText(guide)).toBeTruthy();
      expect(screen.getByText("Geschätzt: 0 von 3")).toBeTruthy();
      const input = screen.getByRole("textbox", { name: field });
      expect(input.getAttribute("placeholder"), field).toBe(`z. B. ${example}`);
      await user.type(input, "viel{Enter}");
      expect(input.getAttribute("aria-invalid"), field).toBe("true");
      expect(screen.getByRole("alert").textContent, field).toBe(invalid);
      expect(computeAccessibleDescription(input), field).toBe(invalid);
      await user.clear(input);
      await user.type(input, "-3");
      expect(input.getAttribute("aria-invalid"), field).toBe("false");
      expect(document.body.textContent, field).not.toMatch(raw);
      shown.unmount();
    }
  });
});
