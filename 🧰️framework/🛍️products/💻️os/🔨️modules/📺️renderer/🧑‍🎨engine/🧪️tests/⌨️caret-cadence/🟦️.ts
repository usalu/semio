
import { afterEach, describe, expect, it, vi } from "vitest";
import { installTextEditorCaretCadenceV1, TEXT_EDITOR_CARET_BLINK_MS } from "../../🧱️elements/✏️TextEditor/🟦️.tsx";
import fixture from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/⌨️caret-cadence/🧫️fixtures/🔣️.json" with { type: "json" };

afterEach(() => {
  vi.useRealTimers();
  document.body.replaceChildren();
});

describe("accepted caret cadence", () => {
  it("validates the language-neutral cadence fixture", () => {
    expect(fixture.halfPeriodMs).toBe(TEXT_EDITOR_CARET_BLINK_MS);
  });

  it("the actual React text-editor input sink starts solid, blinks, resets, and cancels with focus", () => {
    vi.useFakeTimers();
    for (const row of fixture.cases) {
      const sink = document.createElement("textarea");
      document.body.append(sink);
      let visible = false;
      const cadence = installTextEditorCaretCadenceV1(sink, (next) => {
        visible = next;
      });
      for (const step of row.steps) {
        vi.advanceTimersByTime(step.advanceMs);
        switch (step.event) {
          case "focus":
            sink.blur();
            sink.focus();
            break;
          case "edit":
            sink.dispatchEvent(new InputEvent("input", { bubbles: true, data: "x", inputType: "insertText" }));
            break;
          case "blur":
            sink.blur();
            break;
          case "hide":
            window.dispatchEvent(new PageTransitionEvent("pagehide"));
            break;
          case "tick":
            break;
        }
        expect(visible, `${row.id}:${step.event}:${step.advanceMs}:visible`).toBe(step.visible);
        expect(cadence.armed(), `${row.id}:${step.event}:${step.advanceMs}:armed`).toBe(step.armed);
      }
      cadence.dispose();
      sink.remove();
    }
  });
});
