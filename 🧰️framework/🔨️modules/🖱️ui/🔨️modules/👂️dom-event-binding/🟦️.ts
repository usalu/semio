/** 👂️ One owner for a set of DOM listeners: `listen` registers a listener and remembers how to remove it, `dispose`
 * removes every one in reverse order — so a controller installs many listeners and tears them down in one call.
 *
 * @see https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener
 */

/** 🎯️ Anything that takes and removes DOM event listeners (elements, documents, windows, media query lists). */
export type DOMListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

/** 🎧️ A fresh listener owner; see the module docs. */
export function createDOMEventBinding() {
  const cleanups: Array<() => void> = [];
  return {
    listen<E extends Event>(target: DOMListenerTarget | null | undefined, type: string, listener: (event: E) => void, options?: boolean | AddEventListenerOptions) {
      if (!target) return;
      const wrapped = listener as EventListener;
      target.addEventListener(type, wrapped, options);
      cleanups.push(() => target.removeEventListener(type, wrapped, options));
    },
    dispose() {
      while (cleanups.length > 0) cleanups.pop()?.();
    },
  };
}
