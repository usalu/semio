/** 🧱️ A task shows its first item at once and the rest in chunks: opening a sheet never renders every item in one commit, and an answer never starts the chunks again.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🧱️gradual-items/🟦️.ts
 * @see https://react.dev/reference/react/startTransition
 */

import { Profiler, useState, type ReactElement } from "react";
import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import type { Answer, SheetClassificationTask, SheetMatchingTask, SheetSortingTask, SheetTask } from "@semio-tech/course";
import { ClassificationTaskView, ITEM_CHUNK, ITEMS_FIRST, MatchingTaskView, SortingTaskView, courseText, useGradualCount, type TaskViewProps } from "@semio-tech/course-react";

const text = (en: string, de: string = en) => ({ en, de });
const ITEMS = Array.from({ length: 30 }, (_, index) => ({ id: `item-${index}`, label: text(`Item ${index}`) }));

const CLASSIFICATION: SheetClassificationTask = {
  kind: "classification",
  id: "many-categories",
  title: text("Many"),
  prompt: text("Assign."),
  categories: [{ id: "a", label: text("A") }, { id: "b", label: text("B") }],
  items: ITEMS,
};

const SORTING: SheetSortingTask = {
  kind: "sorting",
  id: "many-masses",
  title: text("Many"),
  prompt: text("Sort."),
  quantity: { label: text("Mass"), unit: "g", scale: "logarithmic", prefixed: true, additive: true },
  items: ITEMS,
};

const MATCHING: SheetMatchingTask = {
  kind: "matching",
  id: "many-lamps",
  title: text("Many"),
  prompt: text("Match."),
  dimensions: [{ id: "power", quantity: { label: text("Power"), unit: "W", scale: "logarithmic", prefixed: true, additive: true }, cards: ITEMS.map((_, index) => index + 1) }],
  items: ITEMS,
};

function Probe(props: { readonly total: number; readonly id: string; readonly seen: number[] }): ReactElement {
  const shown = useGradualCount(props.total, props.id);
  props.seen.push(shown);
  return <p>{shown}</p>;
}

function Harness<T extends SheetTask, A extends Answer>(props: { readonly View: (props: TaskViewProps<T, A>) => ReactElement; readonly task: T; readonly given: A; readonly commits: number[]; readonly busy: boolean[]; readonly count: () => number }): ReactElement {
  const [answer, setAnswer] = useState<A | undefined>(undefined);
  const View = props.View;
  return (
    <>
      <button type="button" onClick={() => setAnswer(props.given)}>answer</button>
      <Profiler id="view" onRender={() => { props.commits.push(props.count()); props.busy.push(document.querySelector('[aria-busy="true"]') !== null); }}>
        <View task={props.task} answer={answer} onAnswer={setAnswer} text={courseText("en")} locale="en" />
      </Profiler>
    </>
  );
}

const ROWS: Readonly<Record<SheetTask["kind"], string>> = { classification: "[data-course-item]", sorting: "[data-course-item]", matching: "li.course-slot" };

afterEach(() => {
  document.body.replaceChildren();
});

describe("useGradualCount", () => {
  it("starts with the first items, grows by at most one chunk per commit and ends with all of them", () => {
    const seen: number[] = [];
    render(<Probe total={30} id="one" seen={seen} />);
    expect(seen[0]).toBe(ITEMS_FIRST);
    expect(seen.at(-1)).toBe(30);
    const distinct = [...new Set(seen)];
    for (let index = 1; index < distinct.length; index++) {
      expect(distinct[index]! - distinct[index - 1]!).toBeGreaterThan(0);
      expect(distinct[index]! - distinct[index - 1]!).toBeLessThanOrEqual(ITEM_CHUNK);
    }
    expect(distinct.length).toBeGreaterThanOrEqual(Math.ceil((30 - ITEMS_FIRST) / ITEM_CHUNK));
  });

  it("shows a short list whole in one commit, and never more than there are", () => {
    const seen: number[] = [];
    const { rerender } = render(<Probe total={ITEMS_FIRST} id="one" seen={seen} />);
    expect(new Set(seen)).toEqual(new Set([ITEMS_FIRST]));
    const empty: number[] = [];
    rerender(<Probe total={0} id="one" seen={empty} />);
    expect(empty.at(-1)).toBe(0);
  });

  it("starts again for another list, and keeps what it showed for the same list when the total shrinks", () => {
    const seen: number[] = [];
    const { rerender } = render(<Probe total={30} id="one" seen={seen} />);
    expect(seen.at(-1)).toBe(30);
    const other: number[] = [];
    rerender(<Probe total={30} id="two" seen={other} />);
    expect(other[0]).toBe(ITEMS_FIRST);
    expect(other.at(-1)).toBe(30);
    const fewer: number[] = [];
    rerender(<Probe total={12} id="two" seen={fewer} />);
    expect(new Set(fewer)).toEqual(new Set([12]));
  });
});

describe("a task of many items", () => {
  const views = [
    ["classification", ClassificationTaskView, CLASSIFICATION, { kind: "classification", assignments: { "item-3": "a" } }],
    ["sorting", SortingTaskView, SORTING, { kind: "sorting", order: ITEMS.map((item) => item.id).reverse(), guesses: {} }],
    ["matching", MatchingTaskView, MATCHING, { kind: "matching", assignments: { power: { "item-3": 0 } } }],
  ] as const;
  for (const [kind, View, task, given] of views) {
    it(`renders ${kind} items first one, then chunk by chunk, and an answer renders none of it again`, () => {
      const commits: number[] = [];
      const busy: boolean[] = [];
      const rows = (): number => document.querySelectorAll(ROWS[kind]).length;
      render(<Harness View={View as never} task={task as never} given={given as never} commits={commits} busy={busy} count={rows} />);
      expect(commits[0]).toBe(ITEMS_FIRST);
      expect(rows()).toBe(30);
      expect(busy.slice(0, commits.indexOf(30))).not.toContain(false);
      expect(busy[0]).toBe(true);
      expect(busy.at(-1)).toBe(false);
      expect(document.querySelector('[aria-busy="true"]')).toBeNull();
      const distinct = [...new Set(commits)];
      for (let index = 1; index < distinct.length; index++) expect(distinct[index]! - distinct[index - 1]!).toBeLessThanOrEqual(ITEM_CHUNK);
      expect(distinct.length).toBeGreaterThanOrEqual(Math.ceil((30 - ITEMS_FIRST) / ITEM_CHUNK));
      const before = commits.length;
      act(() => {
        fireEvent.click(screen.getByRole("button", { name: "answer" }));
      });
      expect(rows()).toBe(30);
      expect(commits.length - before).toBe(1);
      expect(commits.at(-1)).toBe(30);
    });
  }
});
