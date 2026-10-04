/** ⌨️ Every task kind works with the keyboard alone — reachable by Tab, operated with select, Enter and Space — keeps
 * focus on the moved item, announces changes politely, and still accepts pointer drag and drop. Browsing a select with
 * the arrow keys never changes what another item holds: a closed select commits every option the arrow keys pass on
 * Windows and Linux, so a card another item uses is a disabled option (`@testing-library/user-event` refuses to select
 * one, as the browsers do) and only a pointer drag takes a card over.
 *
 * @see https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element — `change` fires per keyboard step
 */

import { useState, type ReactElement } from "react";
import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { answerComplete, type Answer, type ClassificationAnswer, type MatchingAnswer, type SheetClassificationTask, type SheetMatchingTask, type SheetSortingTask, type SheetTask, type SortingAnswer } from "@semio-tech/quiz";
import { ClassificationTaskView, MatchingTaskView, SortingTaskView, ordered, quizText, type TaskViewProps } from "@semio-tech/quiz-react";

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
  quantity: { label: text("Mass", "Masse"), unit: "g", scale: "logarithmic", prefixed: true, additive: true },
  items: [
    { id: "horse", label: text("Horse", "Pferd") },
    { id: "mouse", label: text("Mouse", "Maus") },
    { id: "cat", label: text("Cat", "Katze") },
  ],
};

/** 🪜️ The masses with their keys shown: the ascending ladder of their values (mouse 20 g, cat 4 kg, horse 500 kg). */
const LADDER: SheetSortingTask = { ...MASSES, keys: [20, 4000, 500_000] };

const LAMPS: SheetMatchingTask = {
  kind: "matching",
  id: "lamps",
  title: text("Lamps", "Lampen"),
  prompt: text("Match each lamp its power.", "Ordnen Sie jeder Lampe ihre Leistung zu."),
  dimensions: [{ id: "power", quantity: { label: text("Power", "Leistung"), unit: "W", scale: "logarithmic", prefixed: true, additive: true }, cards: [50, 8, 2000] }],
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

/** ⬇️ What ArrowDown does on a closed select in Chrome, Edge and Firefox on Windows and Linux: the next option that is
 * not disabled becomes the value and `change` fires at once. */
function arrowDown(select: HTMLSelectElement): void {
  const next = [...select.options].slice(select.selectedIndex + 1).find((option) => !option.disabled);
  if (next === undefined) return;
  act(() => {
    fireEvent.change(select, { target: { value: next.value } });
  });
}

/** 📢️ What the polite region of a task's moves last said (its second region speaks hints only). */
function spoken(): HTMLElement {
  return screen.getAllByRole("status")[0]!;
}

function explicitLists(): boolean[] {
  return [...document.querySelectorAll("ul, ol")].map((list) => list.getAttribute("role") === "list");
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
    await waitFor(() => expect(spoken().textContent).toBe("Desert assigned to Hot and dry"));
    await user.selectOptions(screen.getByRole("combobox", { name: "Category for Fjord" }), "cold-wet");
    expect(answers.at(-1)).toEqual({ kind: "classification", assignments: { desert: "hot-dry", fjord: "cold-wet" } });
    expect(answerComplete(CLIMATES, answers.at(-1))).toBe(true);
    await user.selectOptions(screen.getByRole("combobox", { name: "Category for Fjord" }), "");
    expect(answers.at(-1)).toEqual({ kind: "classification", assignments: { desert: "hot-dry" } });
    await waitFor(() => expect(spoken().textContent).toBe("Fjord is no longer assigned"));
    expect(answerComplete(CLIMATES, answers.at(-1))).toBe(false);
    expect(explicitLists()).not.toContain(false);
  });

  it("announces only where an item ends up when the arrow keys pass several categories", () => {
    vi.useFakeTimers();
    try {
      const answers: ClassificationAnswer[] = [];
      render(<Harness View={ClassificationTaskView} task={CLIMATES} answers={answers} />);
      const said = (): string => spoken().textContent ?? "";
      arrowDown(screen.getByRole("combobox", { name: "Category for Desert" }) as HTMLSelectElement);
      act(() => {
        vi.advanceTimersByTime(399);
      });
      expect(said()).toBe("");
      arrowDown(screen.getByRole("combobox", { name: "Category for Desert" }) as HTMLSelectElement);
      expect(answers.map((answer) => answer.assignments.desert)).toEqual(["hot-dry", "cold-wet"]);
      act(() => {
        vi.advanceTimersByTime(399);
      });
      expect(said()).toBe("");
      act(() => {
        vi.advanceTimersByTime(1);
      });
      expect(said()).toBe("Desert assigned to Cold and wet");
    } finally {
      vi.useRealTimers();
    }
  });

  it("also assigns by dragging an item onto a bin and back onto the pool", () => {
    const answers: ClassificationAnswer[] = [];
    render(<Harness View={ClassificationTaskView} task={CLIMATES} answers={answers} />);
    const chip = screen.getByRole("combobox", { name: "Category for Fjord" }).closest("li")!;
    drag(chip.querySelector("[data-quiz-grip]")!, screen.getByRole("region", { name: "Cold and wet" }).querySelector("h4")!);
    expect(answers.at(-1)).toEqual({ kind: "classification", assignments: { fjord: "cold-wet" } });
    const moved = screen.getByRole("combobox", { name: "Category for Fjord" }).closest("li")!;
    drag(moved.querySelector("[data-quiz-grip]")!, screen.getByRole("region", { name: "Items to classify" }));
    expect(answers.at(-1)).toEqual({ kind: "classification", assignments: {} });
  });

  it("drags a ghost that shows the category the item holds, and Escape abandons the drag", () => {
    const answers: ClassificationAnswer[] = [];
    render(<Harness View={ClassificationTaskView} task={CLIMATES} answers={answers} />);
    act(() => {
      fireEvent.change(screen.getByRole("combobox", { name: "Category for Fjord" }), { target: { value: "cold-wet" } });
    });
    const chip = screen.getByRole("combobox", { name: "Category for Fjord" }).closest("li")!;
    act(() => {
      fireEvent.pointerDown(chip.querySelector("[data-quiz-grip]")!, { button: 0, clientX: 10, clientY: 90 });
      fireEvent.pointerMove(window, { clientX: 10, clientY: 50 });
    });
    const ghost = document.querySelector<HTMLElement>(".quiz-drag-ghost")!;
    expect(ghost.getAttribute("aria-hidden")).toBe("true");
    expect(ghost.querySelector("select")?.value).toBe("cold-wet");
    act(() => {
      fireEvent.keyDown(window, { key: "Escape" });
    });
    expect(document.querySelector(".quiz-drag-ghost")).toBeNull();
    expect(answers).toEqual([{ kind: "classification", assignments: { fjord: "cold-wet" } }]);
  });

  it("shows every profiled category as a named spider diagram", () => {
    render(<Harness View={ClassificationTaskView} task={CLIMATES} answers={[]} />);
    const diagram = screen.getByRole("img", { name: "Spider diagram of Hot and dry" });
    expect(document.getElementById(diagram.getAttribute("aria-describedby") ?? "")?.textContent?.replaceAll(" ", " ")).toBe("Temperature: 35 °C; Rain: 100 mm; Sunshine: 3,800 h. The values follow as a table.");
    expect(screen.getByRole("img", { name: "Spider diagram of Cold and wet" })).toBeTruthy();
  });
});

describe("⌨️ sorting by keyboard", () => {
  it("records the presented order on request and moves items with Enter and Space, focus following the item, each place showing its key", async () => {
    const answers: SortingAnswer[] = [];
    render(<Harness View={SortingTaskView} task={LADDER} answers={answers} />);
    const user = userEvent.setup();
    expect(answerComplete(LADDER, undefined)).toBe(false);
    expect([...document.querySelectorAll(".quiz-sort-key")].map((key) => key.textContent)).toEqual(["20 g", "4 kg", "500 kg"]);
    expect(screen.queryByRole("textbox")).toBeNull();
    await user.tab();
    expect(document.activeElement?.textContent).toBe("Keep this order");
    await user.keyboard("{Enter}");
    expect(answers).toEqual([{ kind: "sorting", order: ["horse", "mouse", "cat"] }]);
    expect(answerComplete(LADDER, answers[0])).toBe(true);
    expect(screen.queryByRole("button", { name: "Keep this order" })).toBeNull();
    expect(document.activeElement).toBe(screen.getByRole("list", { name: "Order by Mass" }));
    expect(spoken().textContent).toBe("Order kept");
    expect(await tabbableNames(user, 2)).toEqual(["Move Horse up", "Move Horse down"]);
    await user.tab({ shift: true });
    await user.keyboard("{Enter}");
    expect(answers).toHaveLength(1);
    expect(spoken().textContent).toBe("Horse is already first");
    const told = spoken().firstElementChild;
    await user.keyboard("{Enter}");
    expect(spoken().textContent).toBe("Horse is already first");
    expect(spoken().firstElementChild).not.toBe(told);
    await user.tab();
    await user.keyboard("{Enter}");
    expect(answers.at(-1)?.order).toEqual(["mouse", "horse", "cat"]);
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Move Horse down" }));
    await user.keyboard(" ");
    expect(answers.at(-1)?.order).toEqual(["mouse", "cat", "horse"]);
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Move Horse down" }));
    expect(spoken().textContent).toBe("Horse is now in place 3 of 3: 500 kg");
    const count = answers.length;
    await user.keyboard("{Enter}");
    expect(answers).toHaveLength(count);
    expect(spoken().textContent).toBe("Horse is already last");
    expect(screen.getByRole("button", { name: "Move Horse down" }).getAttribute("aria-disabled")).toBe("true");
    expect(explicitLists()).not.toContain(false);
    expect(
      within(screen.getByRole("list", { name: "Order by Mass" }))
        .getAllByRole("listitem")
        .map((item) => `${item.querySelector(".quiz-sort-label")?.textContent} ${item.querySelector(".quiz-sort-key")?.textContent}`),
    ).toEqual(["Mouse 20 g", "Cat 4 kg", "Horse 500 kg"]);
  });

  it("also reorders by pointer drag and drop onto another item", () => {
    const answers: SortingAnswer[] = [];
    render(<Harness View={SortingTaskView} task={LADDER} answers={answers} />);
    const items = within(screen.getByRole("list", { name: "Order by Mass" })).getAllByRole("listitem");
    drag(items[2]!.querySelector("[data-quiz-grip]")!, items[0]!);
    expect(answers.at(-1)?.order).toEqual(["cat", "horse", "mouse"]);
  });

  it("orders the items by their typed guesses where the keys are hidden, shows the unit and what a prefix reads as, and moves nothing by hand", async () => {
    const answers: SortingAnswer[] = [];
    render(<Harness View={SortingTaskView} task={MASSES} answers={answers} />);
    const user = userEvent.setup();
    const horse = screen.getByRole("textbox", { name: "Guess for Horse" });
    const mouse = screen.getByRole("textbox", { name: "Guess for Mouse" });
    const cat = screen.getByRole("textbox", { name: "Guess for Cat" });
    expect(horse.getAttribute("placeholder")).toBe("e.g. 2 kg");
    expect(screen.queryByRole("button")).toBeNull();
    expect(document.querySelectorAll("[data-quiz-grip], [data-quiz-drop], .quiz-sort-key")).toHaveLength(0);
    expect(screen.getByText("Guessed: 0 of 3")).toBeTruthy();
    expect(await tabbableNames(user, 3)).toEqual(["Guess for Horse", "Guess for Mouse", "Guess for Cat"]);
    await user.click(horse);
    await user.type(horse, "500 kg");
    expect(document.querySelector(`[id="${horse.id}--preview"]`)?.textContent).toBe("= 500 kg");
    expect(answers).toHaveLength(0);
    await user.keyboard("{Enter}");
    expect(answers.at(-1)).toEqual({ kind: "sorting", order: ["horse", "mouse", "cat"], guesses: { horse: 500_000 } });
    expect((horse as HTMLInputElement).value).toBe("500 kg");
    expect(spoken().textContent).toBe("Horse: guess 500 kg, now at position 1 of 3");
    expect(answerComplete(MASSES, answers.at(-1))).toBe(false);
    expect(screen.getByText("Guessed: 1 of 3")).toBeTruthy();
    await user.type(mouse, "20");
    expect(document.querySelector(`[id="${mouse.id}--preview"]`)?.textContent).toBe("= 20 g");
    await user.keyboard("{Enter}");
    expect(answers.at(-1)).toEqual({ kind: "sorting", order: ["mouse", "horse", "cat"], guesses: { horse: 500_000, mouse: 20 } });
    expect(spoken().textContent).toBe("Mouse: guess 20 g, now at position 1 of 3");
    expect(document.activeElement).toBe(screen.getByRole("textbox", { name: "Guess for Mouse" }));
    await user.type(cat, "4 kg");
    await user.tab();
    expect(answers.at(-1)).toEqual({ kind: "sorting", order: ["mouse", "cat", "horse"], guesses: { horse: 500_000, mouse: 20, cat: 4000 } });
    expect(answerComplete(MASSES, answers.at(-1))).toBe(true);
    expect(
      within(screen.getByRole("list", { name: "Order by Mass" }))
        .getAllByRole("listitem")
        .map((item) => item.getAttribute("data-quiz-item")),
    ).toEqual(["mouse", "cat", "horse"]);
    const count = answers.length;
    await user.clear(cat);
    await user.type(cat, "heavy{Enter}");
    expect(answers).toHaveLength(count);
    expect(cat.getAttribute("aria-invalid")).toBe("true");
    expect(screen.getByRole("alert").textContent).toBe("Enter a positive number, optionally with an SI prefix and the unit, such as 2 kg.");
    await user.clear(cat);
    await user.keyboard("{Enter}");
    expect(answers.at(-1)).toEqual({ kind: "sorting", order: ["mouse", "cat", "horse"], guesses: { horse: 500_000, mouse: 20 } });
    expect(spoken().textContent).toBe("Guess for Cat removed");
    expect(screen.queryByRole("alert")).toBeNull();
    expect(answerComplete(MASSES, answers.at(-1))).toBe(false);
    expect(screen.getByText("Guessed: 2 of 3")).toBeTruthy();
  });

  it("keeps the rows where they are while Tab goes from guess to guess, and reorders them on Enter or when focus leaves the list", async () => {
    const answers: SortingAnswer[] = [];
    render(
      <>
        <Harness View={SortingTaskView} task={MASSES} answers={answers} />
        <button type="button">After</button>
      </>,
    );
    const user = userEvent.setup();
    const rows = (): readonly (string | null)[] =>
      within(screen.getByRole("list", { name: "Order by Mass" }))
        .getAllByRole("listitem")
        .map((item) => item.getAttribute("data-quiz-item"));
    await user.click(screen.getByRole("textbox", { name: "Guess for Horse" }));
    await user.keyboard("500 kg");
    await user.tab();
    await user.keyboard("20");
    await user.tab();
    expect(answers.at(-1)).toEqual({ kind: "sorting", order: ["mouse", "horse", "cat"], guesses: { horse: 500_000, mouse: 20 } });
    expect(rows()).toEqual(["horse", "mouse", "cat"]);
    expect(document.activeElement).toBe(screen.getByRole("textbox", { name: "Guess for Cat" }));
    await user.keyboard("4 kg{Enter}");
    expect(answers.at(-1)?.order).toEqual(["mouse", "cat", "horse"]);
    expect(rows()).toEqual(["mouse", "cat", "horse"]);
    expect(document.activeElement).toBe(screen.getByRole("textbox", { name: "Guess for Cat" }));
    await user.clear(screen.getByRole("textbox", { name: "Guess for Mouse" }));
    await user.keyboard("1 Mg");
    await user.tab();
    expect(answers.at(-1)?.order).toEqual(["cat", "horse", "mouse"]);
    expect(rows()).toEqual(["mouse", "cat", "horse"]);
    expect(document.activeElement).toBe(screen.getByRole("textbox", { name: "Guess for Cat" }));
    await user.click(screen.getByRole("button", { name: "After" }));
    expect(rows()).toEqual(["cat", "horse", "mouse"]);
  });

  it("keeps an exact guess when its rounded text is left unchanged, and restores it on Escape", async () => {
    const answers: SortingAnswer[] = [];
    render(<Harness View={SortingTaskView} task={MASSES} answers={answers} />);
    const user = userEvent.setup();
    const horse = screen.getByRole("textbox", { name: "Guess for Horse" }) as HTMLInputElement;
    await user.type(horse, "1234567 g{Enter}");
    expect(answers.at(-1)?.guesses).toEqual({ horse: 1_234_567 });
    expect(horse.value).toBe("1.235 Mg");
    const count = answers.length;
    await user.click(horse);
    await user.tab();
    expect(answers).toHaveLength(count);
    expect(answers.at(-1)?.guesses).toEqual({ horse: 1_234_567 });
    await user.type(horse, "9");
    await user.keyboard("{Escape}");
    expect(horse.value).toBe("1.235 Mg");
    expect(answers).toHaveLength(count);
  });
});

describe("↕️ ordering by guesses", () => {
  it("sorts the guessed items among the places they occupy and leaves the others where they are", () => {
    expect(ordered(["a", "b", "c", "d", "e"], { b: 5, d: 1 })).toEqual(["a", "d", "c", "b", "e"]);
    expect(ordered(["a", "b", "c"], { a: 3, b: 2, c: 1 })).toEqual(["c", "b", "a"]);
  });

  it("keeps ties in their order, returns the same order when it already fits, and never moves unguessed items", () => {
    const order = ["a", "b", "c", "d"];
    expect(ordered(order, { a: 2, c: 2, d: 3 })).toBe(order);
    expect(ordered(["c", "a", "b"], { a: 1, b: 1 })).toEqual(["c", "a", "b"]);
    expect(ordered(["b", "a"], { a: 1, b: 1 })).toEqual(["b", "a"]);
    expect(ordered(order, {})).toBe(order);
    expect(ordered(order, { b: 1 })).toBe(order);
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
    const halogen = screen.getByRole("combobox", { name: "Power of Halogen spot" }) as HTMLSelectElement;
    expect(halogen.options[2]?.disabled).toBe(true);
    expect(within(halogen).getAllByRole("option")[2]?.textContent).toBe("8\u00a0W – used by LED bulb");
    await user.selectOptions(halogen, "1");
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { led: 1 } } });
    await waitFor(() => expect(spoken().textContent).toContain("assigned to LED bulb"));
    screen.getByRole("button", { name: "Remove the value of LED bulb" }).focus();
    await user.keyboard("{Enter}");
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: {} } });
    await waitFor(() => expect(spoken().textContent).toBe("LED bulb has no value"));
    await user.selectOptions(halogen, "1");
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { halogen: 1 } } });
    await waitFor(() => expect(spoken().textContent).toBe("8\u00a0W assigned to Halogen spot"));
    screen.getByRole("button", { name: "Remove the value of Halogen spot" }).focus();
    await user.keyboard("{Enter}");
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: {} } });
    await user.selectOptions(screen.getByRole("combobox", { name: "Power of LED bulb" }), "1");
    await user.selectOptions(screen.getByRole("combobox", { name: "Power of Halogen spot" }), "0");
    expect(answerComplete(LAMPS, answers.at(-1))).toBe(false);
    await user.selectOptions(screen.getByRole("combobox", { name: "Power of Floodlight" }), "2");
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { led: 1, halogen: 0, floodlight: 2 } } });
    expect(answerComplete(LAMPS, answers.at(-1))).toBe(true);
    expect(explicitLists()).not.toContain(false);
  });

  it("never takes a card from another item while the arrow keys browse a select", () => {
    const answers: MatchingAnswer[] = [];
    render(<Harness View={MatchingTaskView} task={LAMPS} answers={answers} />);
    const select = (item: string): HTMLSelectElement => screen.getByRole("combobox", { name: `Power of ${item}` }) as HTMLSelectElement;
    arrowDown(select("LED bulb"));
    arrowDown(select("Halogen spot"));
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { led: 0, halogen: 1 } } });
    arrowDown(select("Floodlight"));
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { led: 0, halogen: 1, floodlight: 2 } } });
    const count = answers.length;
    arrowDown(select("Floodlight"));
    arrowDown(select("LED bulb"));
    arrowDown(select("LED bulb"));
    expect(answers).toHaveLength(count);
    expect([...select("LED bulb").options].map((option) => option.disabled)).toEqual([false, false, true, true]);
  });

  it("names the list of rows by its quantity, and in every row the item, its select by quantity and item and its button by what it does", () => {
    render(<Harness View={MatchingTaskView} task={LAMPS} answers={[]} />);
    const rows = within(screen.getByRole("list", { name: "Power" })).getAllByRole("listitem");
    expect(rows.map((row) => row.querySelector(".quiz-slot-label")?.textContent)).toEqual(["LED bulb", "Halogen spot", "Floodlight"]);
    expect(rows.map((row) => within(row).getByRole("combobox").getAttribute("aria-label"))).toEqual(["Power of LED bulb", "Power of Halogen spot", "Power of Floodlight"]);
    expect(rows.map((row) => within(row).getByRole("button").getAttribute("aria-label"))).toEqual(["Remove the value of LED bulb", "Remove the value of Halogen spot", "Remove the value of Floodlight"]);
    expect(rows.map((row) => [...row.children].map((part) => part.tagName))).toEqual(Array.from({ length: 3 }, () => ["SPAN", "SELECT", "BUTTON"]));
    expect(screen.queryByRole("table")).toBeNull();
  });

  it("also assigns by dragging a value card onto an item's row, taking the card over from the item that used it", () => {
    const answers: MatchingAnswer[] = [];
    render(<Harness View={MatchingTaskView} task={LAMPS} answers={answers} />);
    const card = (): HTMLElement => within(screen.getByRole("list", { name: "Value cards: Power" })).getAllByRole("listitem")[2]!;
    const row = (item: string): HTMLElement => screen.getByRole("combobox", { name: `Power of ${item}` }).closest("li")!;
    drag(card().querySelector("[data-quiz-grip]")!, row("Floodlight").querySelector(".quiz-slot-label")!);
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { floodlight: 2 } } });
    drag(card().querySelector("[data-quiz-grip]")!, row("LED bulb").querySelector(".quiz-slot-label")!);
    expect(answers.at(-1)).toEqual({ kind: "matching", assignments: { power: { led: 2 } } });
  });

  it("carries a ghost that lies in a copy of its list, so the copy is laid out for the same room as its source", () => {
    render(<Harness View={MatchingTaskView} task={LAMPS} answers={[]} />);
    const cards = screen.getByRole("list", { name: "Value cards: Power" });
    act(() => {
      fireEvent.pointerDown(within(cards).getAllByRole("listitem")[0]!.querySelector("[data-quiz-grip]")!, { button: 0, clientX: 10, clientY: 90 });
      fireEvent.pointerMove(window, { clientX: 10, clientY: 50 });
    });
    const ghost = document.querySelector<HTMLElement>(".quiz-drag-ghost")!;
    expect(ghost.tagName).toBe("UL");
    expect(ghost.classList.contains("quiz-cards")).toBe(true);
    expect([ghost.getAttribute("aria-hidden"), ghost.getAttribute("aria-label"), ghost.childElementCount]).toEqual(["true", null, 1]);
    expect(ghost.firstElementChild?.classList.contains("quiz-value")).toBe(true);
    act(() => {
      fireEvent.keyDown(window, { key: "Escape" });
    });
    expect(document.querySelector(".quiz-drag-ghost")).toBeNull();
  });

  it("takes a typed guess per item and quantity where the keys are hidden, with no cards to assign", async () => {
    const answers: MatchingAnswer[] = [];
    const hidden: SheetMatchingTask = { ...LAMPS, dimensions: LAMPS.dimensions.map(({ id, quantity }) => ({ id, quantity })) };
    render(<Harness View={MatchingTaskView} task={hidden} answers={answers} />);
    const user = userEvent.setup();
    expect(screen.queryByRole("list", { name: "Value cards: Power" })).toBeNull();
    expect(screen.queryByRole("combobox")).toBeNull();
    expect(screen.getByText("Guessed: 0 of 3")).toBeTruthy();
    expect(await tabbableNames(user, 3)).toEqual(["Guess for Power of LED bulb", "Guess for Power of Halogen spot", "Guess for Power of Floodlight"]);
    const led = screen.getByRole("textbox", { name: "Guess for Power of LED bulb" });
    await user.click(led);
    await user.type(led, "8{Enter}");
    expect(answers.at(-1)).toEqual({ kind: "matching", guesses: { power: { led: 8 } } });
    expect(answerComplete(hidden, answers.at(-1))).toBe(false);
    await waitFor(() => expect(spoken().textContent).toBe("LED bulb: guess 8 W"));
    await user.type(screen.getByRole("textbox", { name: "Guess for Power of Halogen spot" }), "50 W{Enter}");
    await user.type(screen.getByRole("textbox", { name: "Guess for Power of Floodlight" }), "2 kW{Enter}");
    expect(answers.at(-1)).toEqual({ kind: "matching", guesses: { power: { led: 8, halogen: 50, floodlight: 2000 } } });
    expect(answerComplete(hidden, answers.at(-1))).toBe(true);
    expect(screen.getByText("Guessed: 3 of 3")).toBeTruthy();
    await user.clear(led);
    await user.keyboard("{Enter}");
    expect(answers.at(-1)).toEqual({ kind: "matching", guesses: { power: { halogen: 50, floodlight: 2000 } } });
    await waitFor(() => expect(spoken().textContent).toBe("Guess for LED bulb removed"));
    expect(explicitLists()).not.toContain(false);
  });
});
