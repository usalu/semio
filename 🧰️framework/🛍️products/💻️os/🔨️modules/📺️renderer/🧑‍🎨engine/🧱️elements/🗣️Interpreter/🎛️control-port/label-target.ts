/** 🎯️ A field labels the actual labelable element used by the shared ordinary control view. */
import type { UiNodeRecord } from "../../../../../../../../🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts";
import type { UiControlContext } from "./🟦️.ts";
import { controlDomId } from "../🪪️stable-dom-ids/🟦️.ts";

export function selectOptionDomId(id: string, ordinal: number): string { return `${id}.option.${ordinal}`; }

export function controlLabelTarget(context: UiControlContext, record: UiNodeRecord): string | null {
  const component = record.component;
  const id = controlDomId(context, record);
  if (component.type === "input" || component.type === "button") return id;
  if (component.type !== "select") return null;
  if (component.appearance !== "segmented") return id;
  if (component.items.length === 0) return null;
  const value = context.controls.value(record) ?? component.value;
  const ordinal = component.items.findIndex(item => item.value === value);
  return selectOptionDomId(id, ordinal < 0 ? 0 : ordinal);
}
