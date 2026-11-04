/** 🎯️ After a keyboard commit the field keeps focus: when the commit's re-render left focus on the body (the field was replaced or blurred), put it back on the field with `id` on the next frame. Focus that moved to another real control is never stolen. Returns a cancel for the pending restore. */
export function restoreFocusAfterCommitV1(root: Pick<Document, "getElementById" | "activeElement" | "body">, id: string, schedule: (callback: () => void) => () => void = (callback) => { const handle = requestAnimationFrame(callback); return () => cancelAnimationFrame(handle); }): () => void {
  return schedule(() => {
    const active = root.activeElement;
    if (active !== null && active !== root.body) return;
    const field = root.getElementById(id);
    if (field instanceof HTMLElement) field.focus();
  });
}

/** ⌨️ Whether blur should still commit: a draft already committed by Enter is not committed a second time when focus later leaves unchanged. */
export function blurCommitsV1(committedByEnter: string | null, current: string): boolean {
  return committedByEnter !== current;
}

const FOCUSABLE_V1 = 'input:not([disabled]),textarea:not([disabled]),select:not([disabled]),button:not([disabled]),[role="textbox"],[role="treeitem"],[tabindex]:not([tabindex="-1"])';

/** 🎯️ Moves focus into a freshly opened editor window: waits (bounded frames) for the window element `elementId` to mount, then focuses its first focusable control, else the window itself. Focus a person already moved to another control is never stolen. Returns a cancel. */
export function focusWhenMountedV1(root: Pick<Document, "getElementById" | "activeElement" | "body">, elementId: string, attempts = 30, schedule: (callback: () => void) => () => void = (callback) => { const handle = requestAnimationFrame(callback); return () => cancelAnimationFrame(handle); }): () => void {
  let cancel = (): void => undefined;
  let remaining = attempts;
  const attempt = (): void => {
    const active = root.activeElement;
    if (active !== null && active !== root.body && !(active as HTMLElement).closest?.("[data-semio-focus-handoff]")) return;
    const container = root.getElementById(elementId);
    if (container === null) {
      if (--remaining > 0) cancel = schedule(attempt);
      return;
    }
    const target = container.querySelector<HTMLElement>(FOCUSABLE_V1);
    if (target !== null) {
      target.focus();
      return;
    }
    container.setAttribute("tabindex", "-1");
    container.focus();
  };
  cancel = schedule(attempt);
  return () => cancel();
}
