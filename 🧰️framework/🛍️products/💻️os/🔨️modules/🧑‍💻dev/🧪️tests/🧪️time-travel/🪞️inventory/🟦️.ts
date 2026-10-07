/** 🪞️ Inventories the domain-neutral inputs and Actions rail projected by retained accessibility. */
export type UniversalMirrorNodeV1 = {
  readonly key: string;
  readonly role: string;
  readonly label: string;
  readonly value?: string | null;
  readonly valueNow?: string | null;
  readonly valueMin?: string | null;
  readonly valueMax?: string | null;
  readonly checked?: string | null;
  readonly selected?: string | null;
  readonly step?: string | null;
  readonly min?: string | null;
  readonly max?: string | null;
  readonly disabled?: boolean;
  readonly readonly?: boolean;
};

export type UniversalControlV1 = { readonly index: number; readonly key?: string; readonly pointer: string; readonly role: string; readonly name: string; readonly value: string | null; readonly min: string | null; readonly max: string | null; readonly step: string | null; readonly options: number | null; readonly disabled: boolean };
export type UniversalVerbV1 = { readonly id: string; readonly verb: string; readonly category: string; readonly label: string; readonly disabled: boolean };

export function universalMirrorControlsV1(nodes: readonly UniversalMirrorNodeV1[]): UniversalControlV1[] {
  const controls: UniversalControlV1[] = [];
  const roles = new Set(["textbox", "spinbutton", "slider", "radiogroup", "listbox", "combobox", "switch", "checkbox"]);
  const prefix = "framework.history.editor.input.";
  for (const node of nodes) {
    const at = node.key.lastIndexOf(prefix);
    if (at < 0) continue;
    const suffix = node.key.slice(at + prefix.length);
    if (/::|\.(?:useSelection|chip\.\d+)(?:\.row)?$/u.test(suffix)) continue;
    const pointer = suffix.replace(/\.row$/u, "");
    if (controls.some(control => control.pointer === pointer)) continue;
    const chips = nodes.filter(other => other.key.startsWith(`${node.key.replace(/\.row$/u, "")}.chip.`) && other.key.endsWith(".row"));
    const reference = suffix.endsWith(".row") && (chips.length > 0 || nodes.some(other => other.key === node.key.replace(/\.row$/u, ".useSelection")));
    if (!reference && !roles.has(node.role)) continue;
    const choices = nodes.filter(other => (other.key.startsWith(`${node.key}::`) || other.key.startsWith(`${node.key}.`)) && (other.role === "radio" || other.role === "option"));
    const role = reference ? "reference list" : node.role === "checkbox" ? "switch" : node.role;
    controls.push({ index: controls.length, key: node.key, pointer, role, name: node.label.replace(/\s+/gu, " ").trim().slice(0, 80), value: reference ? String(chips.length) : role === "switch" ? node.checked ?? null : role === "radiogroup" || role === "listbox" ? choices.find(choice => choice.selected === "true" || choice.checked === "true")?.label ?? node.value ?? null : node.value ?? node.valueNow ?? null, min: node.min ?? node.valueMin ?? null, max: node.max ?? node.valueMax ?? null, step: node.step ?? null, options: role === "radiogroup" || role === "listbox" ? choices.length : null, disabled: node.disabled === true || node.readonly === true });
  }
  return controls;
}
export function universalMirrorVerbsV1(nodes: readonly UniversalMirrorNodeV1[]): UniversalVerbV1[] {
  const verbs: UniversalVerbV1[] = [];
  let category = "";
  for (const node of nodes) {
    const match = /(?:^|[\/␟\u001f])action\.(category\.)?([A-Za-z0-9_-]+)$/u.exec(node.key);
    if (!match) continue;
    if (match[1]) { category = match[2]!; continue; }
    verbs.push({ id: node.key, verb: match[2]!, category, label: node.label.replace(/\s+/gu, " ").trim().slice(0, 60), disabled: node.disabled === true });
  }
  return verbs;
}
