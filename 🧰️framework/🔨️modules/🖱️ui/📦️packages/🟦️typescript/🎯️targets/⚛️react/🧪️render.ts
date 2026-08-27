//#region 🔖️Contracts
export interface UiTestQueryOptions {
  readonly name?: string | RegExp;
  readonly exact?: boolean;
}

export interface UiTestRenderResult {
  readonly container: HTMLElement;
  getByRole(role: string, options?: UiTestQueryOptions): HTMLElement;
  getByText(text: string | RegExp, options?: UiTestQueryOptions): HTMLElement;
  rerender(node: unknown): void;
  unmount(): void;
}

export interface UiTestScreen {
  getAllByRole(role: string, options?: UiTestQueryOptions): HTMLElement[];
  getByRole(role: string, options?: UiTestQueryOptions): HTMLElement;
  getByText(text: string | RegExp, options?: UiTestQueryOptions): HTMLElement;
  queryByText(text: string | RegExp, options?: UiTestQueryOptions): HTMLElement | null;
}

export interface UiTestWithin {
  getAllByText(text: string | RegExp, options?: UiTestQueryOptions): HTMLElement[];
  getByText(text: string | RegExp, options?: UiTestQueryOptions): HTMLElement;
}

export type UiTestEventInit = Readonly<Record<string, unknown>>;
//#endregion 🔖️Contracts

//#region 🔌️TestingLibraryAdapter
import { act as testingAct, cleanup as testingCleanup, fireEvent as testingFireEvent, render as testingRender, screen as testingScreen, waitFor as testingWaitFor, within as testingWithin } from "@testing-library/react";

/** 🧪️ Renders a UI fixture behind the repository-owned DOM-test boundary. */
export function render(node: unknown): UiTestRenderResult {
  const result = testingRender(node as Parameters<typeof testingRender>[0]);
  return {
    container: result.container,
    getByRole: (role, options) => result.getByRole(role, options as Parameters<typeof result.getByRole>[1]),
    getByText: (value, options) => result.getByText(value, options as Parameters<typeof result.getByText>[1]),
    rerender: (next) => result.rerender(next as Parameters<typeof result.rerender>[0]),
    unmount: result.unmount,
  };
}

/** 🧹 Unmounts every UI fixture registered through the active DOM-test environment. */
export function cleanup(): void {
  testingCleanup();
}

/** 🖱️ Dispatches owned semantic DOM events without exposing the underlying test adapter. */
export const fireEvent = {
  change(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.change(target, init);
  },
  click(target: Element): boolean {
    return testingFireEvent.click(target);
  },
  dragOver(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.dragOver(target, init);
  },
  dragStart(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.dragStart(target, init);
  },
  drop(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.drop(target, init);
  },
  keyDown(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.keyDown(target, init);
  },
  keyUp(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.keyUp(target, init);
  },
  pointerDown(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.pointerDown(target, init);
  },
  pointerMove(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.pointerMove(target, init);
  },
  pointerUp(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.pointerUp(target, init);
  },
  pointerCancel(target: Element, init?: UiTestEventInit): boolean {
    return testingFireEvent.pointerCancel(target, init);
  },
};

/** 🔎️ Provides owned document-level semantic queries. */
export const screen: UiTestScreen = {
  getAllByRole: (role, options) => [...testingScreen.getAllByRole(role, options as Parameters<typeof testingScreen.getAllByRole>[1])],
  getByRole: (role, options) => testingScreen.getByRole(role, options as Parameters<typeof testingScreen.getByRole>[1]),
  getByText: (value, options) => testingScreen.getByText(value, options as Parameters<typeof testingScreen.getByText>[1]),
  queryByText: (value, options) => testingScreen.queryByText(value, options as Parameters<typeof testingScreen.queryByText>[1]),
};

/** 🎯️ Scopes owned semantic queries to one fixture subtree. */
export function within(container: HTMLElement): UiTestWithin {
  const queries = testingWithin(container);
  return {
    getAllByText: (value, options) => [...queries.getAllByText(value, options as Parameters<typeof queries.getAllByText>[1])],
    getByText: (value, options) => queries.getByText(value, options as Parameters<typeof queries.getByText>[1]),
  };
}

/** ⏳ Repeats an assertion until it succeeds or the owned UI-test deadline expires. */
export async function waitFor(assertion: () => void | Promise<void>, timeoutMs = 1_000): Promise<void> {
  await testingWaitFor(assertion, { timeout: timeoutMs });
}

/** ⚛️ Flushes one synchronous UI update transaction. */
export function act(update: () => void): void {
  testingAct(update);
}
//#endregion 🔌️TestingLibraryAdapter
