/** 🧩️ Bounded staged body positions borrow admitted records without copying their graph. */
import type { UiNodeRecord } from "../../../../../../../../🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts";
import { STAGED_FORM_BODY_LIMITS } from "@semio-tech/framework/manifest/staging";

export type StagedBodySlot = "form" | "status" | "field" | "control";

export function stagedBodyNode(record: UiNodeRecord, slot: StagedBodySlot): boolean {
  const children = record.children;
  if (!Array.isArray(children) || !Array.isArray(record.bindings)) return false;
  const component = record.component;
  if (slot === "form") return component.type === "container" && component.role === "form" && record.bindings.length === 0 && children.length >= 1 && children.length <= STAGED_FORM_BODY_LIMITS.controls + 1 && children.every(id => Number.isSafeInteger(id) && id >= 0 && id !== record.id) && new Set(children).size === children.length;
  if (slot === "field") return component.type === "container" && component.role === "field" && record.bindings.length === 0 && children.length === 1 && Number.isSafeInteger(children[0]) && children[0]! >= 0 && children[0] !== record.id;
  if (slot === "status") return component.type === "extension" && component.extension === "framework.stagedFormStatus" && children.length === 0 && record.bindings.length === 0;
  if (children.length !== 0) return false;
  if (component.type === "text") return record.bindings.length === 0;
  if (record.bindings.length !== 1) return false;
  return component.type === "input" && ["text", "longText"].includes(component.kind) && component.commit !== "blur" || component.type === "select" && Array.isArray(component.items) && component.items.length <= 32 || component.type === "button";
}
