/** 🎓️ The hub first-run walkthrough contract — the three things a person has to do before the
 * collaboration outcome exists for them (sign in → create or join a space → invite someone), framed
 * as an {@link IntroductionDefinition} so the shell's existing `UIIntroduction` glass/veil/checklist
 * machinery renders it rather than a second onboarding surface. Pure: no React, no DOM, no storage
 * handle of its own — the seen-flag enters through the same injected {@link HubConnectionStorageV1}
 * port `../🔐️sign-in/🟦️.ts` uses. The definition is state-independent (one tour, both locales) and
 * only the *entry step* is derived from live hub state, so the tour teaches the whole flow while
 * still opening on whatever the person has not done yet. */

import type { IntroductionDefinition, IntroductionStepDefinition } from "@semio-tech/framework";
import type { HubConnectionStorageV1 } from "../🔐️sign-in/🟦️.ts";
import contract from "./🔣️.json" with { type: "json" };

//#region 🔖️Anchors
/** 🧬️ The language-neutral contract (`🔣️.json`, shape `🧬️.schema.json`): stages, anchors per target, step
 * structure, localized text, the seen-flag key and the two workspace controls. The Rust twin
 * (`🎓️HubFirstRun/🎯️targets/🧊️wgpu/🦀️.rs`) reads the very same file, and both are held to the shared
 * vectors in `🧪️tests/🧫️fixtures/🔣️.json`. */
interface HubFirstRunAnchorSetV1 {
  readonly signIn: string;
  readonly signInSupport: readonly string[];
  readonly space: string;
  readonly spaceSupport: readonly string[];
  readonly invite: string;
  readonly inviteSupport: readonly string[];
  readonly redeemField: string;
  readonly celebrate: Readonly<Record<"signIn" | "space" | "redeem" | "invite", string | null>>;
}
interface HubFirstRunStepDataV1 {
  readonly stage: HubFirstRunStageV1;
  readonly title: keyof HubFirstRunTextRowV1;
  readonly body: keyof HubFirstRunTextRowV1;
  readonly placement: IntroductionStepDefinition["placement"];
  readonly introduce: "signIn" | "space" | "invite" | null;
  readonly show: "signInSupport" | "spaceSupport" | "inviteSupport" | null;
  readonly interactions: readonly { readonly on: "signIn" | "space" | "invite" | "redeemField"; readonly label: keyof HubFirstRunTextRowV1; readonly celebrate: "signIn" | "space" | "redeem" | "invite" }[];
}
interface HubFirstRunControlDataV1 {
  readonly id: string;
  readonly icon: string;
  readonly variant: "ghost" | "outline";
  readonly label: Readonly<Record<"en" | "de", string>>;
}
interface HubFirstRunContractV1 {
  readonly introductionId: string;
  readonly storageKeyPrefix: string;
  readonly seenValue: string;
  readonly stepIdPrefix: string;
  readonly stages: readonly HubFirstRunStageV1[];
  readonly anchors: Readonly<Record<"react" | "wgpu", HubFirstRunAnchorSetV1>>;
  readonly steps: readonly HubFirstRunStepDataV1[];
  readonly completion: Readonly<Record<HubFirstRunStageV1, "never" | "signedIn" | "spaceOpen" | "inviteReady">>;
  readonly text: Readonly<Record<"en" | "de", HubFirstRunTextRowV1>>;
  readonly controls: Readonly<{ replay: HubFirstRunControlDataV1; close: HubFirstRunControlDataV1; order: readonly ("replay" | "close")[] }>;
}
const DATA = contract as unknown as HubFirstRunContractV1;

/** 🎯️ The live element ids this tour raises above the glass, owned by `🔐️HubSignIn`
 * (`os.hub.signIn.*`) and `🏘️SpaceBrowser` (`os.hub.spaces.*`, `os.hub.invite.*`). A step whose
 * anchor is not mounted degrades to a centered screen step in `UIIntroduction`; it never throws and
 * never blocks the tour, which is what keeps this safe at phone width where the spaces list and the
 * invite pane are not on screen at the same time. */
export const HUB_FIRST_RUN_ANCHORS_V1 = {
  signIn: DATA.anchors.react.signIn,
  signInSupport: DATA.anchors.react.signInSupport,
  space: DATA.anchors.react.space,
  spaceSupport: DATA.anchors.react.spaceSupport,
  invite: DATA.anchors.react.invite,
  inviteSupport: DATA.anchors.react.inviteSupport,
} as const;

/** 🎛️ The two workspace controls in render order — `os.hub.firstRun.replay` then `os.hub.signIn.cancel` —
 * with their icon, variant and en/de label, so `HubWorkspace` and the wgpu workspace compose the same
 * row from one definition. */
export const HUB_FIRST_RUN_CONTROLS_V1 = DATA.controls;
//#endregion 🔖️Anchors


//#region 🔖️Stages
/** 🪜️ The ordered stages of the hub flow. `done` is not a task — it is the terminal step that closes
 * the tour, so `HUB_FIRST_RUN_STAGES_V1.indexOf(stage)` is also the step index. */
export type HubFirstRunStageV1 = "welcome" | "signIn" | "space" | "invite" | "done";

export const HUB_FIRST_RUN_STAGES_V1: readonly HubFirstRunStageV1[] = DATA.stages;

/** 🧭️ What the shell knows about the person's hub right now. Every field is something
 * `useHubConnection` already holds; this module reads it, never fetches it. */
export interface HubFirstRunStateV1 {
  readonly signedIn: boolean;
  readonly spaceCount: number;
  readonly openSpaceId: string | null;
  readonly canInvite: boolean;
}

/** 🎯️ The first stage the person has *not* completed — where the tour opens. Not signed in → the
 * sign-in step. Signed in with no space → the space step. In a space but not allowed to invite (a
 * viewer/guest role) → `done` rather than teaching a control that would refuse them. */
export function hubFirstRunStageV1(state: HubFirstRunStateV1): HubFirstRunStageV1 {
  if (!state.signedIn) return "signIn";
  if (state.spaceCount === 0 || state.openSpaceId === null) return "space";
  if (state.canInvite) return "invite";
  return "done";
}

/** 🔢️ The step index {@link hubFirstRunIntroductionV1}'s definition should open at for `state`. */
export function hubFirstRunStepIndexV1(state: HubFirstRunStateV1): number {
  return HUB_FIRST_RUN_STAGES_V1.indexOf(hubFirstRunStageV1(state));
}

/** 🧭️ Projects the live hub workspace onto {@link HubFirstRunStateV1}. Kept here rather than inline
 * in the mount so the projection is covered by laws instead of being an untested expression inside a
 * component: `canInvite` is `spaceRowInvitableV1`'s rule (`access === "author"`) applied to the open
 * space when there is one and to any row otherwise, so someone who owns a space they have not opened
 * still reaches the invite step once they open it. */
export function hubFirstRunStateFromV1(
  sessionPhase: string,
  rows: readonly { readonly id: string; readonly access: string }[],
  activeSpaceId: string | null,
): HubFirstRunStateV1 {
  const open = activeSpaceId === null ? null : (rows.find((row) => row.id === activeSpaceId) ?? null);
  return {
    signedIn: sessionPhase === "signed-in",
    spaceCount: rows.length,
    openSpaceId: open === null ? null : activeSpaceId,
    canInvite: open === null ? rows.some((row) => row.access === "author") : open.access === "author",
  };
}
/** ✅️ The one completion rule both renderers share (`🔣️.json` `completion`, vectors `completionVectors`):
 * whether the person has just done what `stage` teaches — signed in, a space open, an invite ready.
 * The welcome and done screens are never completed by doing; they advance by button only. */
export function hubFirstRunStageDoneV1(stage: HubFirstRunStageV1, state: HubFirstRunStateV1, inviteReady: boolean): boolean {
  switch (DATA.completion[stage]) {
    case "signedIn":
      return state.signedIn;
    case "spaceOpen":
      return state.signedIn && state.openSpaceId !== null;
    case "inviteReady":
      return inviteReady;
    default:
      return false;
  }
}
//#endregion 🔖️Stages

//#region 🔖️Text
/** 🌐️ English first, German second, no default language — an unowned locale is refused rather than
 * silently downgraded, exactly as `hubSignInTextV1` refuses one (`AGENTS.md`: "multiple languages
 * with no default language"). Both locales carry the same keys; the component test asserts that. */
export const HUB_FIRST_RUN_TEXT_V1: Readonly<Record<"en" | "de", HubFirstRunTextRowV1>> = DATA.text;

export type HubFirstRunTextRowV1 = Readonly<Record<"title" | "welcomeTitle" | "welcomeBody" | "signInTitle" | "signInBody" | "signInTask" | "spaceTitle" | "spaceBody" | "spaceTaskCreate" | "spaceTaskJoin" | "inviteTitle" | "inviteBody" | "inviteTask" | "doneTitle" | "doneBody", string>>;

export type HubFirstRunLocaleV1 = "en" | "de";

/** 🔡️ Refuses unowned locales instead of silently selecting a default language. */
export function hubFirstRunTextV1(locale: string): HubFirstRunTextRowV1 {
  if (locale !== "en" && locale !== "de") throw new Error("hub.first-run.locale-unsupported");
  return HUB_FIRST_RUN_TEXT_V1[locale];
}
//#endregion 🔖️Text

//#region 🔖️Definition
/** 🎓️ The whole hub walkthrough, already localized, in stage order. The two screen steps
 * (`welcome`, `done`) carry no anchor so they render centered on any viewport; the three task steps
 * anchor to the live controls and list their work as checklist rows. Interactions are `panel` kinds
 * naming the surface the person must reach, so the step completes by *doing* rather than by pressing
 * Next — the mechanism `IntroductionInteractionKind` already defines. */
export function hubFirstRunIntroductionV1(locale: string): IntroductionDefinition {
  const text = hubFirstRunTextV1(locale);
  const anchors = DATA.anchors.react;
  return {
    title: text.title,
    steps: DATA.steps.map((row): IntroductionStepDefinition => ({
      id: `${DATA.stepIdPrefix}${row.stage}`,
      title: text[row.title],
      body: text[row.body],
      introduce: row.introduce === null ? null : anchors[row.introduce],
      show: row.show === null ? [] : [...anchors[row.show]],
      placement: row.placement,
      interactions: row.interactions.map((interaction) => ({ on: { kind: "panel", id: anchors[interaction.on] }, label: text[interaction.label], celebrate: anchors.celebrate[interaction.celebrate] ?? undefined })),
      ordered: false,
      logos: [],
    })),
  };
}
//#endregion 🔖️Definition

//#region 🔖️Seen
/** 🔖️ Local-only, per-profile: "this person has already been walked through the hub flow". It is a
 * convenience flag, never authority — a cleared browser profile replays the tour, which is harmless,
 * and nothing about sign-in or membership is inferred from it.
 *
 * Deliberately **the same key space every other introduction uses**
 * (`UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX` + an app id, value `"true"`, read by
 * `readStoredIntroductionSeen`/`writeStoredIntroductionSeen` in `🖱️ui/🎯️targets/⚛️react`) rather
 * than a private key of its own: the hub walkthrough is an introduction like any other, so clearing
 * introductions or reading "has this person been introduced to X" must find it. This module restates
 * the key instead of importing those helpers only because it is pure TS with no React dependency —
 * a law cross-checks the two against each other so the restatement cannot drift. */
export const HUB_FIRST_RUN_INTRODUCTION_ID_V1 = DATA.introductionId;
export const HUB_FIRST_RUN_STORAGE_KEY_V1 = `${DATA.storageKeyPrefix}${DATA.introductionId}`;
export const HUB_FIRST_RUN_SEEN_VALUE_V1 = DATA.seenValue;

/** 🔎️ Whether the tour has already run for this profile. A throwing or absent store reads `false`
 * (show the tour) rather than propagating — first-run must never be the thing that breaks the shell. */
export function hubFirstRunSeenV1(storage: HubConnectionStorageV1 | null): boolean {
  if (!storage) return false;
  try {
    return storage.getItem(HUB_FIRST_RUN_STORAGE_KEY_V1) === HUB_FIRST_RUN_SEEN_VALUE_V1;
  } catch {
    return false;
  }
}

/** ✍️ Records that the tour ran. A throwing store is swallowed: the cost is replaying the tour once
 * more, which is strictly better than failing the shell on a quota error. */
export function markHubFirstRunSeenV1(storage: HubConnectionStorageV1 | null): void {
  if (!storage) return;
  try {
    storage.setItem(HUB_FIRST_RUN_STORAGE_KEY_V1, HUB_FIRST_RUN_SEEN_VALUE_V1);
  } catch {
    return;
  }
}

/** 🚦️ Whether to auto-start the tour: never when it has already run, and never before the hub
 * surface is actually open, so it cannot ambush someone who is working locally and has not asked for
 * a hub. An explicit command-palette replay bypasses this and passes `force`. */
export function hubFirstRunShouldStartV1(storage: HubConnectionStorageV1 | null, hubSurfaceOpen: boolean): boolean {
  return hubSurfaceOpen && !hubFirstRunSeenV1(storage);
}
//#endregion 🔖️Seen
