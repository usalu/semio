/** ⏱️ Text editor caret cadence: the caret blink interval and the cadence installer that blinks, holds and resets the caret. */
//#region 🔖️TextEditorHost
//#region EditingHelpers
export const TEXT_EDITOR_CARET_BLINK_MS = 500;

export type TextEditorCaretCadenceV1 = Readonly<{
  reset: () => void;
  suspend: () => void;
  dispose: () => void;
  visible: () => boolean;
  armed: () => boolean;
}>;

/** ⌨️ Gives the canvas editor the focus-owned caret cadence native HTML inputs receive from
 * the browser. The hidden textarea remains the focus/IME authority; the canvas session receives
 * only visible-state transitions and paints the caret itself. */
export function installTextEditorCaretCadenceV1(target: HTMLTextAreaElement, present: (visible: boolean) => void): TextEditorCaretCadenceV1 {
  let timer: ReturnType<typeof setTimeout> | undefined;
  let current = false;
  let focused = target.ownerDocument.activeElement === target;
  const publish = (visible: boolean) => {
    if (current === visible) return;
    current = visible;
    present(visible);
  };
  const clear = () => {
    if (timer !== undefined) clearTimeout(timer);
    timer = undefined;
  };
  const arm = () => {
    clear();
    if (!focused) return;
    timer = setTimeout(() => {
      timer = undefined;
      if (!focused) return;
      publish(!current);
      arm();
    }, TEXT_EDITOR_CARET_BLINK_MS);
  };
  const reset = () => {
    if (!focused) return;
    publish(true);
    arm();
  };
  const focus = () => {
    focused = true;
    reset();
  };
  const suspend = () => {
    focused = false;
    clear();
    publish(false);
  };
  const resetEvents = ["beforeinput", "input", "keydown", "pointerdown", "compositionstart", "compositionupdate", "compositionend"] as const;
  target.addEventListener("focus", focus);
  target.addEventListener("blur", suspend);
  for (const event of resetEvents) target.addEventListener(event, reset);
  target.ownerDocument.defaultView?.addEventListener("pagehide", suspend);
  if (focused) reset();
  else present(false);
  return {
    reset,
    suspend,
    visible: () => current,
    armed: () => timer !== undefined,
    dispose: () => {
      suspend();
      target.removeEventListener("focus", focus);
      target.removeEventListener("blur", suspend);
      for (const event of resetEvents) target.removeEventListener(event, reset);
      target.ownerDocument.defaultView?.removeEventListener("pagehide", suspend);
    },
  };
}
//#endregion EditingHelpers
//#endregion 🔖️TextEditorHost
