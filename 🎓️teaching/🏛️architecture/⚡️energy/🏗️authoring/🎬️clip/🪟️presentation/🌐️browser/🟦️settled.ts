/** 🪟 Check the readiness of a composited window frame: its border painted and its content plane measured. */
export function presentationWindowSettled(stack: Element): boolean {
  const border = stack.querySelector("[data-window-frame-border]");
  const content = stack.querySelector("[data-window-frame-content]");
  return Boolean(border && !border.hasAttribute("data-pending") && content?.getAttribute("data-frame-state") === "ready");
}

/** 📐 Request fresh owned frame geometry after an ancestor camera transform. */
export function requestCameraRemeasure(root: Document): void {
  for (const stack of root.querySelectorAll("[data-presentation-window]")) {
    const value = Number(stack.getAttribute("data-frame-remeasure") ?? 0);
    stack.setAttribute("data-frame-remeasure", String(value + 1));
  }
}

/** 🪡 Wait until every shared-border root has painted its current layout. */
export function presentationBordersSettled(document: Document): boolean {
  const roots = document.querySelectorAll("[data-shared-border-root]");
  return roots.length > 0 && [...roots].every(root => !root.hasAttribute("data-shared-border-pending"));
}
