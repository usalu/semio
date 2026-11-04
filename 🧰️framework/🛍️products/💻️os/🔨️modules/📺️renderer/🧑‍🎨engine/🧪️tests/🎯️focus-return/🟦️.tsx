import { cleanup, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { blurCommitsV1, focusWhenMountedV1, restoreFocusAfterCommitV1 } from "../../🧱️elements/🎯️focus-return/🟦️.ts";

afterEach(cleanup);
const now = (callback: () => void) => { callback(); return () => undefined; };

describe("focus return after an Enter commit", () => {
  it("puts focus back on the field when the commit left it on the body", () => {
    const view = render(<input id="field-1" aria-label="name" />);
    expect(document.activeElement).toBe(document.body);
    restoreFocusAfterCommitV1(document, "field-1", now);
    expect(document.activeElement).toBe(view.container.querySelector("#field-1"));
  });

  it("never steals focus from another control", () => {
    const view = render(<><input id="field-1" aria-label="a" /><button id="other">b</button></>);
    (view.container.querySelector("#other") as HTMLElement).focus();
    restoreFocusAfterCommitV1(document, "field-1", now);
    expect(document.activeElement).toBe(view.container.querySelector("#other"));
  });

  it("does not commit twice: blur after an Enter commit of the same text is skipped, edited text commits", () => {
    expect(blurCommitsV1("abc", "abc")).toBe(false);
    expect(blurCommitsV1("abc", "abcd")).toBe(true);
    expect(blurCommitsV1(null, "abc")).toBe(true);
  });
});

describe("focus into a freshly opened editor window", () => {
  const frames = () => {
    const queue: (() => void)[] = [];
    return { schedule: (callback: () => void) => { queue.push(callback); return () => undefined; }, flush: () => { const next = queue.shift(); next?.(); return next !== undefined; } };
  };

  it("waits for the window to mount, then focuses its first control", () => {
    const f = frames();
    const view = render(<div />);
    focusWhenMountedV1(document, "framework.window.main", 5, f.schedule);
    f.flush();
    view.rerender(<div id="framework.window.main"><button>x</button><input aria-label="edit" /></div>);
    f.flush();
    expect(document.activeElement?.tagName).toBe("BUTTON");
  });

  it("falls back to the window itself and never steals focus from another control", () => {
    const f = frames();
    const view = render(<><div id="framework.window.main" /><button id="other">o</button></>);
    focusWhenMountedV1(document, "framework.window.main", 5, f.schedule);
    f.flush();
    expect(document.activeElement).toBe(view.container.querySelector("#framework\\.window\\.main"));
    (view.container.querySelector("#other") as HTMLElement).focus();
    focusWhenMountedV1(document, "framework.window.main", 5, f.schedule);
    f.flush();
    expect(document.activeElement).toBe(view.container.querySelector("#other"));
  });

  it("gives up after the bounded attempts when the window never mounts", () => {
    const f = frames();
    focusWhenMountedV1(document, "missing", 3, f.schedule);
    let ran = 0;
    while (f.flush()) ran += 1;
    expect(ran).toBe(3);
  });
});
