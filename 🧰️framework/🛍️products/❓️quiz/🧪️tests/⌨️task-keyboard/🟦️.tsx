/** ⌨️ Every task kind works with the keyboard alone — reachable by Tab, operated with select, Enter and Space — keeps
 * focus on the moved item, announces changes politely, and still accepts pointer drag and drop.
 */

import { useState, type ReactElement } from "react";
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { answerComplete, type Answer, type ClassificationAnswer, type MatchingAnswer, type SheetClassificationTask, type SheetMatchingTask, type SheetSortingTask, type SheetTask, type SortingAnswer } from "@semio-tech/quiz";
import { ClassificationTaskView, MatchingTaskView, SortingTaskView, quizText, type TaskViewProps } from "@semio-tech/quiz-react";

const text = (en: string, de: string) => ({ en, de });

const CLIMATES: SheetClassificationTask = {
  kind: "classification",
  id: "climates",
  title: text("Climates", "Klimazonen"),
  prompt: text("Assign each place its climate.", "Ordnen Sie jedem Ort sein Klima zu."),
  axes: [
    { id: "temperature", label: text("Temperature", "Temperatur"), unit: "°C", min: -30, max: 50 },
    { id: "rain", label: text("Rain", "Regen"), unit: "mm", min: 0, max: 3000 },
    { id: "sun", label: text("Sunshine", "Sonnenschein"), unit: "h", min: 0, max: 4000 },
  ],
  categories: [
    { id: "hot-dry", label: text("Hot and dry", "Heiß und trocken"), profile: { temperature: 35, rain: 100, sun: 3800 } },
    { id: "cold-wet", label: text("Cold and wet", "Kalt und nass"), profile: { temperature: -5, rain: 1500, sun: 1200 } },
  ],
  items: [
    { id: "desert", label: text("Desert", "Wüste") },
    { id: "fjord", label: text("Fjord", "Fjord") },
  ],
};

const MASSES: SheetSortingTask = {
  kind: "sorting",
  id: "masses",
  title: text("Masses", "Massen"),
  prompt: text("Sort by mass.", "Nach Masse sortieren."),
  quantity: { label: text("Mass", "Masse"), unit: "g", scale: "logarithmic", prefixed: true },
  items: [
    { id: "horse", label: text("Horse", "Pferd") },
    { id: "mouse", label: text("Mouse", "Maus") },
    { id: "cat", label: text("Cat", "Katze") },
  ],
};

const LAMPS: SheetMatchingTask = {
  kind: "matching",
  id: "lamps",
  title: text("Lamps", "Lampen"),
  prompt: text("Match each lamp its power.", "Ordnen Sie jeder Lampe ihre Leistung zu."),
  dimensions: [{ id: "power", quantity: { label: text("Power", "Leistung"), unit: "W", scale: "logarithmic", prefixed: true }, cards: [50, 8, 2000] }],
  items: [
    { id: "led", label: text("LED bulb", "LED-Lampe") },
    { id: "halogen", label: text("Halogen spot", "Halogenstrahler") },
    { id: "floodlight", label: text("Floodlight", "Flutlicht") },
  ],
};

function Harness<T extends SheetTask, A extends Answer>(props: { readonly View: (props: TaskViewProps<T, A>) => ReactElement; readonly task: T; readonly answers: A[] }): ReactElement {
  const [answer, setAnswer] = useState<A | undefined>(undefined);
  const View = props.View;
  return (
    <View
      task={props.task}
      answer={answer}
      onAnswer={(next) => {
        props.answers.push(next);
        setAnswer(next);
      }}
      text={quizText("en")}
      locale="en"
    />
  );
}

async function tabbableNames(user: ReturnType<typeof userEvent.setup>, stops: number): Promise<string[]> {
  const names: string[] = [];
  for (let stop = 0; stop < stops; stop += 1) {
    await user.tab();
    const focused = document.activeElement as HTMLElement | null;
    names.push(focused?.getAttribute("aria-label") ?? focused?.textContent ?? "");
  }
  return names;
}

function drag(grip: Element, target: Element): void {
  Object.defineProperty(document, "elementFromPoint", { configurable: true, value: vi.fn(() => target) });
  act(() => {
    fireEvent.pointerDown(grip, { button: 0, clientX: 10, clientY: 90 });
    fireEvent.pointerMove(window, { clientX: 10, clientY: 50 });
    fireEvent.pointerMove(window, { clientX: 10, clientY: 10 });
  });
  expect(target.closest("[data-quiz-drop]")?.classList.contains("quiz-drop-active")).toBe(true);
  act(() => {
    fireEvent.pointerUp(window, { clientX: 10, clientY: 10 });
  });
  expect(document.querySelector(".quiz-drag-ghost")).toBeNull();
  Reflect.deleteProperty(document, "elementFromPoint");
}

afterEach(() => {
  vi.restoreAllMocks();
});

describe("⌨️ classification by keyboard", () => {
  it("reaches every item's category select by Tab, assigns, keeps focus on the moved item and announces it", async () => {
    const answers: ClassificationAnswer[] = [];
    render(<Harness View={ClassificationTaskView} task={CLIMATES} answers={answers} />);
    const user = userEvent.setup();
    expect(await tabbableNames(user, 2)).toEqual(["Category for Desert", "Category for Fjord"]);
    await user.tab({ shift: true });
    const desert = screen.getByRole("combobox", { name: "Category for Desert" });
    expect(document.activeElement).toBe(desert);
    await user.selectOptions(desert, "hot-dry");
    const moved = screen.getByRole("combobox", { name: "Category for Desert" });
    expect(document.activeElement).toBe(moved);
    expect(within(screen.getByRole("region", { name: "Hot and dry" })).getByText("Desert")).toBeTruthy();
    expect(screen.getByRole("status").textContent).toBe("Desert assigned to Hot and dry");
    await user.selectOptions(screen.getByRole("combobox", { name: "Category for Fjord" }), "cold-wet");
    expect(answers.at(-1)).toEqual({ kind: "classification", assignments: { desert: "hot-dry", fjord: "cold-wet" } });
    expect(answerComplete(CLIMATES, answers.at(-1))).toBe(true);
    await user.selectOptions(screen.getByRole("combobox", { name: "Category for Fjord" }), "");
    expect(answers.at(-1)).toEqual({ kind: "classification", assignments: { desert: "hot-dry" } });
    expect(screen.getByRole("status").textContent).toBe("Fjord is no longer assigned");
    expect(answerComplete(CLIMATES, answers.at(-1))).toBe(false);
  });

  it("also assigns by dragging an item onto a bin and back onto the pool", () => {
    const answers: ClassificationAnswer[] = [];
    render(<Harness View={ClassificationTaskView} task={CLIMATES} answers={answers} />);
    const chip = screen.getByRole("combobox", { name: "Category for Fjord" }).closest("li")!;
    drag(chip.querySelector(".quiz-grip")!, screen.getByRole("region", { name: "Cold and wet" }).querySelector("h4")!);
    expect(answers.at(-1)).toEqual({ kind: "classification", assignments: { fjord: "cold-wet" } });
    const moved = screen.getByRole("combobox", { name: "Category for Fjord" }).closest("li")!;
    drag(moved.querySelector(".quiz-grip")!, screen.getByRole("region", { name: "Items to classify" }));
    expect(answers.at(-1)).toEqual({ kind: "classification", assignments: {} });
  });

  it("shows every profiled category as a named spider diagram", () => {
    render(<Harness View={ClassificationTaskView} task={CLIMATES} answers={[]} />);
    expect(screen.getByRole("img", { name: "Spider diagram of Hot and dry" })).toBeTruthy();
    expect(screen.getByRole("img", { name: "Spider diagram of Cold and wet" })).toBeTruthy();
  });
});

describe("⌨️ sorting by keyboard", () => {
  it("records the presented order on request and moves items with Enter and Space, focus following the item", async () => {
    const answers: SortingAnswer[] = [];
    render(<Harness View={SortingTaskView} task={MASSES} answers={answers} />);
    const user = userEvent.setup();
    expect(answerComplete(MASSES, undefined)).toBe(false);
    await user.tab();
    expect(document.activeElement?.textContent).toBe("Keep this order");
    await user.keyboard("{Enter}");
    expect(answers).toEqual([{ kind: "sorting", order: ["horse", "mouse", "cat"] }]);
    expect(answerComplete(MASSES, answers[0])).toBe(true);
    expect(await tabbableNames(user, 2)).toEqual(["Move Horse up", "Move Horse down"]);
    await user.keyboard("{Enter}");
    expect(answers.at(-1)?.order).toEqual(["mouse", "horse", "cat"]);
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Move Horse down" }));
    await user.keyboard(" ");
    expect(answers.at(-1)?.order).toEqual(["mouse", "cat", "horse"]);
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Move Horse down" }));
    expect(screen.getByRole("status").textContent).toBe("Horse is now at position 3 of 3");
    const count = answers.length;
    await user.keyboard("{Enter}");
    expect(answers).toHaveLength(count);
    expect(screen.getByRole("button", { name: "Move Horse down" }).getAttribute("aria-disabled")).toBe("true");
    expect(
      within(screen.getByRole("list", { name: "Order by Mass" }))
        .getAllByRole("listitem")
        .map((item) => item.textContent?.replace(/[⠿↑↓\d]/gu, "")),
    ).toEqual(["Mouse", "Cat", "Horse"]);
  });

  it("also reorders by pointer drag and drop onto another item", () => {
    const answers: SortingAnswer[] = [];
    render(<Harness View={SortingTaskView} task={MASSES} answers={answers} />);
    const items = within(screen.getByRole("list", { name: "Order by Mass" })).getAllByRole("listitem");
    drag(items[2]!.querySelector(".quiz-grip")!, items[0]!);
    expect(answers.at(-1)?.order).toEqual(["cat", "horse", "mouse"]);
  });
});

describe("⌨️ matching by keyboard", () => {
  it("assigns value cards per item, uses each card once, shows which are used and unassigns with Enter", async () => {
    const answers: MatchingAnswer[] = [];
    render(<Harness View={MatchingTaskView} task={LAMPS} answers={answers} />);
    const user = userEvent.setup();
    expect(await tabbableNames(user, 2)).toEqual(["Power of LED bulb", "Remove the value of LED bulb"]);
    const led = screen.getByRole("combobox", { name: "Power of LED bulb" });
    expect(
      within(led)
        .getAllByRole("option")
        .map((option) => option.textContent),
    ).toEqual(["Not assigned", "50\u00a0W", "8\u00a0W", "2\u00a0kW"]);
    await user.selectOptions(led, "1");
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { led: 1 } } });
    const cards = screen.getByRole("list", { name: "Value cards: Power" });
    expect(within(cards).getAllByRole("listitem")[1]?.textContent).toContain("used by LED bulb");
    const halogen = screen.getByRole("combobox", { name: "Power of Halogen spot" });
    expect(within(halogen).getAllByRole("option")[2]?.textContent).toBe("8\u00a0W – used by LED bulb");
    await user.selectOptions(halogen, "1");
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { halogen: 1 } } });
    expect(screen.getByRole("status").textContent).toBe("8\u00a0W assigned to Halogen spot");
    screen.getByRole("button", { name: "Remove the value of Halogen spot" }).focus();
    await user.keyboard("{Enter}");
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: {} } });
    await user.selectOptions(screen.getByRole("combobox", { name: "Power of LED bulb" }), "1");
    await user.selectOptions(screen.getByRole("combobox", { name: "Power of Halogen spot" }), "0");
    expect(answerComplete(LAMPS, answers.at(-1))).toBe(false);
    await user.selectOptions(screen.getByRole("combobox", { name: "Power of Floodlight" }), "2");
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { led: 1, halogen: 0, floodlight: 2 } } });
    expect(answerComplete(LAMPS, answers.at(-1))).toBe(true);
  });

  it("also assigns by dragging a value card onto an item's row", () => {
    const answers: MatchingAnswer[] = [];
    render(<Harness View={MatchingTaskView} task={LAMPS} answers={answers} />);
    const card = within(screen.getByRole("list", { name: "Value cards: Power" })).getAllByRole("listitem")[2]!;
    const row = screen.getByRole("rowheader", { name: "Floodlight" }).closest("tr")!;
    drag(card.querySelector(".quiz-grip")!, row.querySelector("th")!);
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { floodlight: 2 } } });
  });
});
