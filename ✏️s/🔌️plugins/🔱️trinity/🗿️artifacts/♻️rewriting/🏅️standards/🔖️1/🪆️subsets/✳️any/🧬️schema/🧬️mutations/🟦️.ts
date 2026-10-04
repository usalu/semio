/** ♻️ Rewriting direct-mutation discriminated union. */
import type { EditBeforeFixture } from "./🖼️edit-before-fixture/🟦️.ts";
import type { EditLhs } from "./👈️edit-lhs/🟦️.ts";
import type { EditRhs } from "./👉️edit-rhs/🟦️.ts";
import type { ChangeParameterBinding } from "./🔧️change-parameter/🟦️.ts";
import type { RemoveParameterBinding } from "./🧹️remove-parameter-binding/🟦️.ts";
import type { ChangeRuleLayoutPoint } from "./📐️change-rule-layout/🟦️.ts";
import type { RemoveRuleLayoutPoint } from "./🗑️remove-rule-layout/🟦️.ts";
import type { DragRuleNodes } from "./🫳️drag-rule/🟦️.ts";
import type { SetRuleLayoutPoints } from "./📍️set-rule-layout/🟦️.ts";

export type RewriteRuleMutation =
  | ({ mutation: "editBeforeFixture" } & EditBeforeFixture)
  | ({ mutation: "editLhs" } & EditLhs)
  | ({ mutation: "editRhs" } & EditRhs)
  | ({ mutation: "changeParameterBinding" } & ChangeParameterBinding)
  | ({ mutation: "removeParameterBinding" } & RemoveParameterBinding)
  | ({ mutation: "changeRuleLayoutPoint" } & ChangeRuleLayoutPoint)
  | ({ mutation: "removeRuleLayoutPoint" } & RemoveRuleLayoutPoint)
  | ({ mutation: "dragRuleNodes" } & DragRuleNodes)
  | ({ mutation: "setRuleLayoutPoints" } & SetRuleLayoutPoints);
