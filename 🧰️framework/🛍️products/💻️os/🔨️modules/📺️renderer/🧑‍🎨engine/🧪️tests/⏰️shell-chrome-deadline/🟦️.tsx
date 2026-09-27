import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { ChromeControlHint, DEFAULT_UI_DRIVER, UiDriverProvider } from "@semio-tech/ui-react";
import Ajv2020 from "ajv/dist/2020";
import React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { scheduleShellTransientNoticeDismissV1, SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS } from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import fixture from "../../🧱️elements/🐚️Shell/🧪️fixtures/⏰️chrome-deadline/🔣️.json" with { type: "json" };
import schema from "../../🧱️elements/🐚️Shell/🧪️fixtures/⏰️chrome-deadline/🧬️schema/🔣️.json" with { type: "json" };

afterEach(() => {
  cleanup();
  vi.useRealTimers();
});

function NoticeHarness(): React.ReactElement | null {
  const [visible, setVisible] = React.useState(true);
  React.useEffect(() => {
    const timer = scheduleShellTransientNoticeDismissV1(() => setVisible(false));
    return () => clearTimeout(timer);
  }, []);
  return visible ? <div role="status">{fixture.text.notice}</div> : null;
}

describe("Shell chrome deadlines", () => {
  it("validates the neutral deadline fixture", () => {
    const validate = new Ajv2020({ strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.timing.noticeDismissMs).toBe(SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS);
  });

  it("actual React chrome reveals the tooltip at dwell and removes the notice at dismissal", () => {
    vi.useFakeTimers();
    const tooltip = render(
      <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
        <ChromeControlHint text={fixture.text.tooltip} always>
          <button type="button">Trigger</button>
        </ChromeControlHint>
      </UiDriverProvider>,
    );
    const trigger = tooltip.container.querySelector<HTMLElement>('[data-slot="chrome-control-hint"]')!;
    fireEvent.focus(trigger);
    act(() => vi.advanceTimersByTime(fixture.timing.tooltipDwellMs - 1));
    expect(document.querySelector('[role="tooltip"]')).toBeNull();
    act(() => vi.advanceTimersByTime(1));
    expect(document.querySelector('[role="tooltip"]')?.textContent).toBe(fixture.text.tooltip);
    tooltip.unmount();

    const notice = render(<NoticeHarness />);
    act(() => vi.advanceTimersByTime(fixture.timing.noticeDismissMs - 1));
    expect(notice.container.querySelector('[role="status"]')?.textContent).toBe(fixture.text.notice);
    act(() => vi.advanceTimersByTime(1));
    expect(notice.container.querySelector('[role="status"]')).toBeNull();
  });
});
