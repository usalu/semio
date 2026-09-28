const focusable = 'button:not([disabled]),input:not([disabled]),textarea:not([disabled]),select:not([disabled]),a[href],[tabindex]:not([tabindex="-1"]),[contenteditable]:not([contenteditable="false"])';

type FocusOwner = { target: HTMLElement; id: string; path: readonly string[] };

/** 🎯️ Retains local focus through row replacement without overriding a deliberate focus move. */
export function retainTreeFocus(root: HTMLElement): () => void {
  const document = root.ownerDocument;
  let owner: FocusOwner | undefined;
  const remember = (event: FocusEvent) => {
    if (!(event.target instanceof HTMLElement)) return;
    const row = event.target.closest<HTMLElement>('[data-tree-focus-path]');
    let path: string[] = [];
    try {
      const value: unknown = JSON.parse(row?.dataset.treeFocusPath ?? "[]");
      if (Array.isArray(value)) path = value.slice(-64).filter((id): id is string => typeof id === "string");
    } catch { path = []; }
    owner = {target:event.target, id:event.target.id, path};
  };
  const release = (event: FocusEvent) => {
    if (event.target !== owner?.target) return;
    const previous = owner;
    if (event.relatedTarget) { owner = undefined; return; }
    queueMicrotask(() => {
      if (owner === previous && previous?.target.isConnected) owner = undefined;
    });
  };
  const leave = (event: Event) => {
    if (event.target instanceof Node && !root.contains(event.target)) owner = undefined;
  };
  const candidate = (element: HTMLElement | null): HTMLElement | undefined => {
    if (!element || !root.contains(element) || element.closest('[hidden],[inert],[aria-hidden="true"]')) return;
    if (element.matches(focusable)) return element;
    return Array.from(element.querySelectorAll<HTMLElement>(focusable)).find(control => !control.closest('[hidden],[inert],[aria-hidden="true"]'));
  };
  const observer = new MutationObserver(() => {
    if (!owner || owner.target.isConnected || !root.isConnected) return;
    if (document.activeElement && document.activeElement !== document.body) { owner = undefined; return; }
    let target = owner.id ? candidate(document.getElementById(owner.id)) : undefined;
    for (let index = owner.path.length - 1; !target && index >= 0; index--) target = candidate(document.getElementById(owner.path[index]));
    owner = undefined;
    (target ?? root).focus({preventScroll:true});
  });
  root.addEventListener("focusin", remember);
  root.addEventListener("focusout", release);
  document.addEventListener("focusin", leave);
  document.addEventListener("pointerdown", leave, true);
  observer.observe(root, {childList:true, subtree:true});
  return () => {
    observer.disconnect();
    root.removeEventListener("focusin", remember);
    root.removeEventListener("focusout", release);
    document.removeEventListener("focusin", leave);
    document.removeEventListener("pointerdown", leave, true);
    owner = undefined;
  };
}
