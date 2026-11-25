import { cleanup, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { BootstrapStatusNotice } from "../../../🪪️host-bootstrap/🟦️.tsx";
import { FloatingPane, NoticeActionRow, NoticeCell, NoticeStack } from "../../🟦️.tsx";

afterEach(cleanup);

describe("shell notice bands", () => {
  it("stacks notices as cells of one shared frame without a native border, box padding or gap", () => {
    const view = render(
      <NoticeStack data-semio-notice-stack="bootstrap" className="top-workbench left-1/2 bg-menu">
        <NoticeCell><BootstrapStatusNotice status={{ kind: "artifact-bootstrap-progress", documentId: "doc-a", clientInstanceId: "canvas-a", receivedBytes: 1, totalBytes: 4, receivedChunks: 1, totalChunks: 4 }} locale="en" onCancel={() => undefined} /></NoticeCell>
        <NoticeCell><BootstrapStatusNotice status={{ kind: "artifact-bootstrap-progress", documentId: "doc-b", clientInstanceId: "canvas-b", receivedBytes: 2, totalBytes: 4, receivedChunks: 2, totalChunks: 4 }} locale="en" onCancel={() => undefined} /></NoticeCell>
      </NoticeStack>,
    );
    const stack = view.container.querySelector<HTMLElement>("[data-semio-notice-stack]")!;
    expect(stack.getAttribute("data-shared-border-frame")).toBe("");
    expect(stack.getAttribute("data-shared-border-cells")).toBe("");
    expect(stack.getAttribute("data-orientation")).toBe("vertical");
    expect(stack.className).not.toMatch(/\bborder\b|\brounded|\bgap-|\bpx-|\bpy-/u);
    expect(stack.className).toContain("absolute");
    expect(stack.className).toContain("bg-menu");
    expect(stack.className).not.toContain("bg-background");
    const cells = Array.from(stack.children);
    expect(cells).toHaveLength(2);
    for (const cell of cells) {
      expect(cell.hasAttribute("data-shared-border-cell")).toBe(true);
      expect(cell.querySelector("[data-semio-bootstrap-status]")).toBeTruthy();
    }
  });

  it("owns a floating pane frame on the fixed box and keeps the scroller inside so no second border appears", () => {
    const view = render(
      <FloatingPane data-semio-floating-pane="hub-workspace" className="max-h-[80vh]" scrollClassName="px-double">
        <p>content</p>
      </FloatingPane>,
    );
    const pane = view.container.querySelector<HTMLElement>("[data-semio-floating-pane]")!;
    expect(pane.getAttribute("data-shared-border-frame")).toBe("");
    expect(pane.className).not.toMatch(/\bborder\b|\brounded|\boverflow-auto\b/u);
    const scroller = pane.firstElementChild as HTMLElement;
    expect(scroller.className).toContain("overflow-auto");
    expect(scroller.className).not.toMatch(/\bborder\b|\brounded/u);
    expect(scroller.textContent).toBe("content");
  });

  it("folds an install row and stall rows into one stack: nested rows are cells, each action is a framework button cell, no native border", () => {
    const view = render(
      <NoticeStack data-semio-notice-stack="install" className="top-[var(--navbar-band-height)]">
        <NoticeActionRow data-semio-plugin-install="" text="Installing" action={{ label: "Cancel", onPress: () => undefined }} actionProps={{ "data-semio-plugin-install-cancel": "" }} />
        <NoticeActionRow data-semio-command-stall-id="s1" text="Slow" action={{ label: "Cancel", onPress: () => undefined }} />
      </NoticeStack>,
    );
    const stack = view.container.querySelector<HTMLElement>("[data-semio-notice-stack]")!;
    expect(stack.className).not.toMatch(/border|mt-|gap-/u);
    for (const row of Array.from(stack.children) as HTMLElement[]) {
      expect(row.getAttribute("data-slot")).toBe("shared-border-row");
      expect(row.className).not.toMatch(/border|gap-/u);
      expect(row.children).toHaveLength(2);
      expect(row.children[0]!.classList.contains("flex-1")).toBe(true);
      expect(row.children[1]!.getAttribute("data-slot")).toBe("button-group");
    }
    expect(stack.querySelector("[data-semio-plugin-install-cancel]")).toBeTruthy();
    const withoutAction = render(<NoticeStack orientation="horizontal"><NoticeActionRow text="x" action={null} /></NoticeStack>);
    expect(withoutAction.container.querySelector("[data-orientation=horizontal]")).toBeTruthy();
    expect(withoutAction.container.querySelector("button")).toBeNull();
  });
});
