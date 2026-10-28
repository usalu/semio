/** ⌨️ Preview accessibility, native semantics and authoritative keyboard answer controls. */
import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { courseText } from "@semio-tech/course-react";
import { TaskView } from "@semio-tech/course-react/modules/run/.tsx";
import { quizStylesheet } from "@semio-tech/course-react/stylesheet";
import type { Mockup, SheetClassificationTask } from "@semio-tech/course";
import { MockupView } from "../../🎯️targets/⚛️react/🔨️modules/🖥️mockup/🟦️.tsx";
import vectors from "../../🧫️fixtures/🖥️mockups/🔣️.json";

describe("workspace previews", () => {
  it("distinguishes native parameter, component, slider, panel and plugin widgets", () => {
    for (const vector of vectors.filter((vector) => vector.id.startsWith("widget-"))) {
      const { container, unmount } = render(<MockupView value={vector.mockup as Mockup} locale="en" />);
      expect(container.querySelector(`[data-widget="${vector.id.slice(7)}"]`)).not.toBeNull();
      if (vector.id === "widget-slider") {
        expect(screen.getByRole("img", { name: "-2 ≤ 3 ≤ 8" })).toBeDefined();
        expect(container.querySelector<HTMLElement>(".course-slider-thumb")!.style.left).toBe("50%");
      }
      if (vector.id === "widget-panel") expect(container.querySelector(".course-panel-values")!.textContent).toBe("1224");
      if (vector.id === "widget-plugin") expect(container.querySelector(".course-plugin-label")!.textContent).toContain("Solver");
      unmount();
    }
  });
  it("shows a selected coordinate, formula, reference table and workbook tabs in both languages", () => {
    const value = vectors[0]!.mockup as Mockup;
    for (const locale of ["en", "de"] as const) {
      const view = render(<MockupView value={value} locale={locale} />);
      const region = screen.getByRole("region", { name: value.caption[locale] });
      expect(region.tabIndex).toBe(0);
      expect(within(region).getByRole("table", { name: "Areas" })).toBeDefined();
      expect(within(region).getByText("=B2*2")).toBeDefined();
      expect(within(region).getByText("C2")).toBeDefined();
      expect(within(region).getByText("Costs")).toBeDefined();
      expect(view.container.querySelectorAll("[data-selected]")).toHaveLength(1);
      expect(view.container.querySelector("code[data-highlight]")).not.toBeNull();
      expect(view.container.querySelectorAll("button,input,select")).toHaveLength(0);
      view.unmount();
    }
  });

  it("provides exact textual port relations and indexed tree data without requiring SVG vision", () => {
    const value = vectors[1]!.mockup as Mockup;
    const { container } = render(<MockupView value={value} locale="de" />);
    expect(screen.getByText(/von Numbers · N → nach Operation · D/)).toBeDefined();
    expect(screen.getByRole("region", { name: "Datenbaum" })).toBeDefined();
    expect(screen.getByText("{0;1}")).toBeDefined();
    expect(container.querySelector("path[data-highlight]")).not.toBeNull();
    expect(container.querySelector("svg")!.getAttribute("aria-hidden")).toBe("true");
    expect(container.querySelector(".course-graph-relations li[data-highlight] .sr-only")?.textContent).toContain("Hervorgehoben");
  });

  it("keeps previews outside controls while keyboard answers and timed locking stay authoritative", () => {
    const label = { en: "Reference", de: "Referenz" };
    const task: SheetClassificationTask = { kind: "classification", id: "reference", title: label, prompt: label, categories: [{ id: "a", label: { en: "A", de: "A" } }, { id: "b", label: { en: "B", de: "B" } }], items: [{ id: "one", label, mockup: vectors[0]!.mockup as Mockup }] };
    const onAnswer = vi.fn();
    const props = { task, answer: undefined, onAnswer, text: courseText("en"), locale: "en" as const };
    const view = render(<TaskView {...props} />);
    const select = screen.getByRole("combobox");
    select.focus();
    fireEvent.change(select, { target: { value: "b" } });
    expect(onAnswer).toHaveBeenCalledWith({ kind: "classification", assignments: { one: "b" } });
    expect(view.container.querySelector("figure")?.closest("button,label,select")).toBeNull();
    onAnswer.mockClear();
    view.rerender(<TaskView {...props} locked />);
    fireEvent.change(screen.getByRole("combobox"), { target: { value: "a" } });
    expect(onAnswer).not.toHaveBeenCalled();
    expect(screen.getByRole("region", { name: "Reference cells" }).tabIndex).toBe(0);
    expect(quizStylesheet).toContain("overflow: auto");
    expect(quizStylesheet).toContain(".course-mockup { grid-column: 1 / -1; min-width: 0; max-width: 100%");
  });
});
