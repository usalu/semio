/** 🧩️ Puzzle 2d editor surface — namespaced re-export of every window's typed twin. Namespaced (not a
 * blanket `export *`) since each window independently declares a same-named `ViewModel`-shaped
 * interface — a blanket re-export would collide. */
export * as selectionWindow from "./🎭️modes/✏️edit/🪟️windows/🎯️selection/🟦️component";
export * as overviewWindow from "./🎭️modes/✏️edit/🪟️windows/👁️overview/🟦️component";
export * as detailWindow from "./🎭️modes/✏️edit/🪟️windows/🔍️detail/🟦️component";
