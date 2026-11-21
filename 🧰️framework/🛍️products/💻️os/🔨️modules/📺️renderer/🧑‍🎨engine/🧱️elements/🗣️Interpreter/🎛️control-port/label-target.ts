/** 🎯️ A field labels the actual labelable element used by the shared ordinary control view. */
import type { UiNodeRecord } from "../../../../../../../../🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts";
import type { UiControlContext } from "./🟦️.ts";
import { controlDomId } from "../🪪️stable-dom-ids/🟦️.ts";
import { fieldAssociation } from "../../../../../../../../🔨️modules/🖱️ui/🧱️elements/📝️Field/🔗️association/🟦️.ts";

export function selectOptionDomId(id: string, ordinal: number): string { return `${id}.option.${ordinal}`; }

export function controlLabelTarget(context: UiControlContext, record: UiNodeRecord): string | null {
  if (record.activity === "loading" || record.activity === "waiting") return null;
  const component = record.component;
  const id = controlDomId(context, record);
  if (component.type === "input" || component.type === "button" || component.type === "toggle" || component.type === "numberStepper") return id;
  if (component.type !== "select") return null;
  if (component.appearance !== "segmented") return id;
  if (component.items.length === 0) return null;
  const value = context.controls.value(record) ?? component.value;
  const ordinal = component.items.findIndex(item => item.value === value);
  return selectOptionDomId(id, ordinal < 0 ? 0 : ordinal);
}

/** 🔗️ Uses the published child and its actual address for field labels and validation. */
export function fieldControlAssociation(context: UiControlContext, field: Extract<UiNodeRecord["component"], { type: "container" }>, fieldId: string, control: UiNodeRecord) {
  const target = controlLabelTarget(context, control);
  const association = fieldAssociation(fieldId, target, Boolean(field.description), Boolean(field.error));
  const fieldContext = {
    ...context,
    field: {
      label: control.accessibility.label ? undefined : association.label,
      describedBy: association.describedBy,
      error: association.error,
      required: field.required ?? undefined,
      invalid: Boolean(field.error),
    },
  };
  return { target, association, context: fieldContext };
}
