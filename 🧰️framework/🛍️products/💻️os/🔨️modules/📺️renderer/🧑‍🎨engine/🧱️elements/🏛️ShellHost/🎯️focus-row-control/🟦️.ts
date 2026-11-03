/** ✏️ The `focusRowControl` verb of the React shell: move the keyboard focus into the first editable cell of a tree or grid row (`F2` on the focused row raises it) and refuse politely, in the interface language, a row that holds no cell to type into — the same rule, names and texts as the Rust shell (`plan_row_control_focus`, `ActionsCtx::framework_focus_row_control`).
 *
 * 🧫️ Pure planning over plain data plus a small DOM adapter; the language-agnostic fixture `🐚️Shell/🧪️tests/🧫️fixtures/🎯️focus-row-control/🔣️.json` drives both implementations. Typing in the cell is the shared text keyboard law (`🖱️ui/🧬️contract/🧫️fixtures/🧫️text-controls`): Escape drops the draft and shows the committed value, Enter commits.
 *
 * 🔗️ Rust twin: `🧰️framework/🔨️modules/🛂️manifest/🕹️actions/🗃️utility-tool-and-document-actions/🦀️.rs` (`plan_row_control_focus`) and `📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs`. */
import { FOCUS_ROW_CONTROL_ARG_ROW, FOCUS_ROW_CONTROL_EDITABLE_KINDS, FOCUS_ROW_CONTROL_KEY, type ShellLocale } from "@semio-tech/framework";
import { shellLabelTextV1 } from "../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";

/** ✏️ One control inside a row, reduced to what the rule reads. */
export type RowControlV1 = Readonly<{ id: string; kind: string; disabled?: boolean }>;

export const FOCUS_ROW_CONTROL_REFUSED_CODE_V1 = "shell.row-control-focus.refused";

/** ✏️ The control a focus lands on: the first enabled control, in row order, whose kind is typed into; `null` when the row holds none. */
export function planRowControlFocusV1(controls: readonly RowControlV1[]): string | null {
  return controls.find((control) => control.disabled !== true && (FOCUS_ROW_CONTROL_EDITABLE_KINDS as readonly string[]).includes(control.kind))?.id ?? null;
}

/** 🔊️ The polite refusal in `locale` (same text and code as the Rust shell). */
export function focusRowControlRefusalV1(locale: ShellLocale): Readonly<{ text: string; severity: "warning"; code: string }> {
  return { text: shellLabelTextV1("ui.rowControlFocus.refused")[locale], severity: "warning", code: FOCUS_ROW_CONTROL_REFUSED_CODE_V1 };
}

/** 🧾️ The row a `focusRowControl` action names; `null` while the row argument is missing. */
export function focusRowControlArgsV1(args: unknown): string | null {
  const row = (typeof args === "object" && args !== null ? (args as Readonly<Record<string, unknown>>) : {})[FOCUS_ROW_CONTROL_ARG_ROW];
  return typeof row === "string" && row.length > 0 ? row : null;
}

/** ⌨️ Whether a key event is the bare `F2` that asks to edit the focused row. */
export function isFocusRowControlKeyV1(event: Readonly<{ key: string; ctrlKey: boolean; metaKey: boolean; altKey: boolean; shiftKey: boolean }>): boolean {
  return event.key === FOCUS_ROW_CONTROL_KEY && !event.ctrlKey && !event.metaKey && !event.altKey && !event.shiftKey;
}

const ROW_V1 = '[role="treeitem"]';
const CONTROLS_V1 = 'input, textarea, select, button, [role="switch"], [role="checkbox"], [role="slider"], [role="spinbutton"]';
const INPUT_KINDS_V1: Readonly<Record<string, string>> = { text: "text", search: "text", email: "text", url: "text", tel: "text", password: "text", number: "number", date: "date", "datetime-local": "date", month: "date", week: "date", time: "date", color: "color", file: "file", checkbox: "toggle", radio: "toggle", range: "slider", button: "button", submit: "button", reset: "button" };

/** 🧩️ The kind name the rule reads for a DOM control (the kind names of the Rust retained document: `text`, `longText`, `number`, `date`, `color`, `file`, `toggle`, `select`, `slider`, `button`). */
export function rowControlKindOfElementV1(element: Element): string {
  const tag = element.tagName.toLowerCase();
  if (tag === "textarea") return "longText";
  if (tag === "select") return "select";
  if (tag === "button") return "button";
  if (tag === "input") return INPUT_KINDS_V1[(element.getAttribute("type") ?? "text").toLowerCase()] ?? "other";
  const role = element.getAttribute("role");
  return role === "switch" || role === "checkbox" ? "toggle" : role === "slider" || role === "spinbutton" ? "slider" : "other";
}

/** 🆔️ The authored key of the row whose DOM id is `domId`: the interpreter names a row `<surface>/<key>` (`uiNodeDomId`). */
export function rowKeyOfDomIdV1(domId: string): string {
  return domId.slice(domId.lastIndexOf("/") + 1);
}

/** 🔎️ The tree row element authored as `rowKey`. */
export function rowElementOfKeyV1(root: ParentNode, rowKey: string): HTMLElement | null {
  return [...root.querySelectorAll<HTMLElement>(ROW_V1)].find((row) => row.id !== "" && rowKeyOfDomIdV1(row.id) === rowKey) ?? null;
}

/** 🔎️ The controls the row holds itself, in document order; a nested row's controls are its own. */
export function rowControlElementsV1(row: Element): readonly HTMLElement[] {
  return [...row.querySelectorAll<HTMLElement>(CONTROLS_V1)].filter((control) => control.closest(ROW_V1) === row);
}

/** ⌨️ The authored key of the row that holds the keyboard focus, `null` when the focus is on anything but a row itself (a cell inside a row owns its own `F2`). */
export function focusedRowKeyV1(target: EventTarget | null): string | null {
  return target instanceof HTMLElement && target.matches(ROW_V1) && target.id !== "" ? rowKeyOfDomIdV1(target.id) : null;
}

/** ✏️ Focuses the first editable cell of the row authored as `rowKey` and selects its text, so typing replaces the cell; answers the focused element, or `null` when no window presents the row or it holds no editable cell. */
export function focusRowControlV1(root: ParentNode, rowKey: string): HTMLElement | null {
  const row = rowElementOfKeyV1(root, rowKey);
  if (row === null) return null;
  const controls = rowControlElementsV1(row);
  const planned = planRowControlFocusV1(controls.map((control, index) => ({ id: String(index), kind: rowControlKindOfElementV1(control), disabled: (control as HTMLInputElement).disabled === true || control.getAttribute("aria-disabled") === "true" })));
  const target = planned === null ? null : controls[Number(planned)] ?? null;
  if (target === null) return null;
  target.focus();
  if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) target.select();
  return target;
}
