// #region 🧲️Header
/** @emoji 🎓️ The hub first-run walkthrough pane — the presentational half of
 * `📇️directory/🎓️first-run/🟦️.ts`. It owns no transport, no storage handle and no hub state: the
 * caller passes the live {@link HubFirstRunStateV1} and receives `onDismiss`, exactly as
 * `🔐️HubSignIn`/`🏘️SpaceBrowser` receive theirs, so the same pane renders in a story, in a test and
 * in `ShellHost` without a stub. Rendering is delegated to `UIIntroduction`, the shell's existing
 * introduction surface — this element adds no second onboarding chrome. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useMemo, useState, type ReactElement } from "react";
import { UIIntroduction } from "@semio-tech/ui-react";
import {
  HUB_FIRST_RUN_STAGES_V1,
  hubFirstRunIntroductionV1,
  hubFirstRunStepIndexV1,
  type HubFirstRunStageV1,
  type HubFirstRunStateV1,
} from "../../../../📇️directory/🎓️first-run/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️Types
export interface HubFirstRunProps {
  /** 🌐️ `en`/`de` only — an unowned locale is a thrown refusal from the contract, never a silent
   * English fallback. */
  readonly locale: string;
  /** 🧭️ Live hub state; decides which step the tour opens on. */
  readonly state: HubFirstRunStateV1;
  /** 🚪️ `completed` is `true` when the person walked to the last step, `false` when they skipped. */
  readonly onDismiss: (completed: boolean) => void;
  /** 🔢️ Overrides the derived entry step — used by stories and by a replay that should restart at
   * the beginning regardless of state. */
  readonly initialStepIndex?: number;
}
//#endregion 🔖️Types

//#region 🔖️Pane
/** 🎓️ Renders the walkthrough. The step index is local component state seeded from the hub state, so
 * a background change (a peer's invite landing, a session expiring) never yanks the person to a
 * different step mid-read. */
export function HubFirstRun({ locale, state, onDismiss, initialStepIndex }: HubFirstRunProps): ReactElement {
  const introduction = useMemo(() => hubFirstRunIntroductionV1(locale), [locale]);
  const [stepIndex, setStepIndex] = useState(() => initialStepIndex ?? hubFirstRunStepIndexV1(state));
  return <UIIntroduction introduction={introduction} stepIndex={stepIndex} onStepIndexChange={setStepIndex} onDismiss={onDismiss} />;
}

/** 🪜️ The stage a rendered step index belongs to — the inverse of `hubFirstRunStepIndexV1`, for a
 * caller that wants to log or gate on where the person is. Out-of-range indices answer `done` rather
 * than throwing, so a definition that grows a step cannot crash a stale caller. */
export function hubFirstRunStageAtIndex(index: number): HubFirstRunStageV1 {
  return HUB_FIRST_RUN_STAGES_V1[index] ?? "done";
}
//#endregion 🔖️Pane
