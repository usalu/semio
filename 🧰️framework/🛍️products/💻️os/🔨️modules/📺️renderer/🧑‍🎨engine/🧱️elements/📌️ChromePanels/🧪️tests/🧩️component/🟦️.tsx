import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { themeAlphaInput, themeNumberInputRow, themeTextInputRow } from "../../🟦️.tsx";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

describe("ChromePanels theme inputs", () => {
  it("renders and lazily commits canonical text values", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const commit = vi.fn();
    const row = themeTextInputRow("theme.spacing.compact", "compact", "0.25rem", commit);
    const { getByRole } = render(<>{row.control}</>);
    const input = getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("0.25rem");
    fireEvent.change(input, { target: { value: "0.5rem" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith("0.5rem");
    expect(consoleError).not.toHaveBeenCalled();
  });

  it("renders and lazily commits canonical scalar and list numbers", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const commit = vi.fn();
    const row = themeNumberInputRow("theme.metrics.padding", "padding", [1, 2.5], commit);
    const { getByRole } = render(<>{row.control}</>);
    const input = getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("1, 2.5");
    fireEvent.change(input, { target: { value: "3, 4.5" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith([3, 4.5]);
    expect(consoleError).not.toHaveBeenCalled();
  });

  it("renders and lazily commits clamped appearance alpha", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const commit = vi.fn();
    const { getByRole } = render(themeAlphaInput("theme.appearance.alpha", 0.25, commit));
    const input = getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("0.25");
    fireEvent.change(input, { target: { value: "1.2" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith(1);
    expect(consoleError).not.toHaveBeenCalled();
  });
});
