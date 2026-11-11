/** 🪟 Check the measured content and readiness of a composited silhouette. */
export function presentationWindowSettled(stack: Element, contentClipPath: string): boolean {
  const normalized = document.createElement("div").style;
  normalized.clipPath = contentClipPath;
  const border = stack.querySelector("[data-window-silhouette-border]");
  const actual = (stack.querySelector("[data-window-silhouette-content]") as HTMLElement | null)?.style.clipPath;
  if (!border || border.hasAttribute("data-pending") || !actual) return false;
  const numbers = /-?\d+(?:\.\d+)?/g, a = actual.match(numbers) ?? [], b = normalized.clipPath.match(numbers) ?? [];
  return actual.replace(numbers, "#") === normalized.clipPath.replace(numbers, "#") && a.length === b.length && a.every((value, index) => Math.abs(Number(value) - Number(b[index])) < .01);
}

/** 📐 Request fresh owned silhouette geometry after an ancestor camera transform. */
export function requestCameraRemeasure(root: Document): void {
  for (const stack of root.querySelectorAll("[data-presentation-window]")) {
    const value = Number(stack.getAttribute("data-silhouette-remeasure") ?? 0);
    stack.setAttribute("data-silhouette-remeasure", String(value + 1));
  }
}

/** 🪡 Wait until every shared-border root has painted its current layout. */
export function presentationBordersSettled(document: Document): boolean {
  const roots = document.querySelectorAll("[data-shared-border-root]");
  return roots.length > 0 && [...roots].every(root => !root.hasAttribute("data-shared-border-pending"));
}
