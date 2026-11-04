import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { TransientNotice } from "../../🧱️elements/🏛️ShellHost/🍞️transient-notice/🟦️.tsx";
import { DefaultAppToggle } from "../../🧱️elements/🏛️ShellHost/🌟️default-app-toggle/🟦️.tsx";
import { TreeStatusAnnouncement } from "../../🧱️elements/🗣️Interpreter/🧱️node-tree/📢️status-announcement/🟦️.tsx";

afterEach(cleanup);

describe("transient notices announce by severity and name their close button", () => {
  const notice = (kind: "info" | "warning" | "error") => ({ message: "Disk is full", kind, code: "x" });

  it("uses assertive alerts for errors and warnings and polite status for info", () => {
    for (const kind of ["error", "warning"] as const) {
      render(<TransientNotice notice={notice(kind)} closeLabel="Close" onClose={() => undefined} />);
      expect(screen.getByRole("alert").getAttribute("aria-live")).toBe("assertive");
      cleanup();
    }
    render(<TransientNotice notice={notice("info")} closeLabel="Close" onClose={() => undefined} />);
    expect(screen.getByRole("status").getAttribute("aria-live")).toBe("polite");
  });

  it("names the close button with its notice and closes once", () => {
    let closed = 0;
    render(<TransientNotice notice={notice("error")} closeLabel="Close" onClose={() => { closed += 1; }} />);
    fireEvent.click(screen.getByRole("button", { name: "Close: Disk is full" }));
    expect(closed).toBe(1);
  });
});

describe("open-with default toggle", () => {
  it("keeps one fixed name and exposes state only through aria-pressed", () => {
    let toggles = 0;
    const view = render(<DefaultAppToggle isDefault={false} label="Set as default" onToggle={() => { toggles += 1; }} />);
    const off = screen.getByRole("button", { name: "Set as default" });
    expect(off.getAttribute("aria-pressed")).toBe("false");
    fireEvent.click(off);
    expect(toggles).toBe(1);
    view.rerender(<DefaultAppToggle isDefault label="Set as default" onToggle={() => undefined} />);
    expect(screen.getByRole("button", { name: "Set as default" }).getAttribute("aria-pressed")).toBe("true");
  });
});

describe("loading surfaces have a live region", () => {
  it("announces the loading text politely and busy", () => {
    render(<TreeStatusAnnouncement text="Loading surface…" busy />);
    const region = screen.getByRole("status");
    expect(region.getAttribute("aria-live")).toBe("polite");
    expect(region.getAttribute("aria-busy")).toBe("true");
    expect(region.textContent).toBe("Loading surface…");
  });
});
