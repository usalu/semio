/** ♻️ Rewrite direct-mutation discriminated union. */
import type { EditBeforeFixture } from "./🖼️edit-before-fixture/🟦️component.ts";
import type { EditLhs } from "./🔍️edit-lhs/🟦️component.ts";
import type { EditRhs } from "./🎯️edit-rhs/🟦️component.ts";
import type { ChangeParameterBinding } from "./🔧️change-parameter-binding/🟦️component.ts";
import type { RemoveParameterBinding } from "./🧹️remove-parameter-binding/🟦️component.ts";
import type { ChangeRuleLayoutPoint } from "./📐️change-rule-layout-point/🟦️component.ts";
import type { RemoveRuleLayoutPoint } from "./🗑️remove-rule-layout-point/🟦️component.ts";

export type RewriteRuleMutation =
  | ({ mutation: "editBeforeFixture" } & EditBeforeFixture)
  | ({ mutation: "editLhs" } & EditLhs)
  | ({ mutation: "editRhs" } & EditRhs)
  | ({ mutation: "changeParameterBinding" } & ChangeParameterBinding)
  | ({ mutation: "removeParameterBinding" } & RemoveParameterBinding)
  | ({ mutation: "changeRuleLayoutPoint" } & ChangeRuleLayoutPoint)
  | ({ mutation: "removeRuleLayoutPoint" } & RemoveRuleLayoutPoint);
