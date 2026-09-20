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

//#region 🔖️Anchors
/** 🎯️ The live element ids this tour raises above the glass, owned by `🔐️HubSignIn`
 * (`os.hub.signIn.*`) and `🏘️SpaceBrowser` (`os.hub.spaces.*`, `os.hub.invite.*`). A step whose
 * anchor is not mounted degrades to a centered screen step in `UIIntroduction`; it never throws and
 * never blocks the tour, which is what keeps this safe at phone width where the spaces list and the
 * invite pane are not on screen at the same time. */
export const HUB_FIRST_RUN_ANCHORS_V1 = {
  signIn: "os.hub.signIn.email",
  signInSupport: ["os.hub.signIn.password", "os.hub.signIn.submit", "os.hub.signIn.hub"],
  space: "os.hub.spaces.createName",
  spaceSupport: ["os.hub.spaces.createSubmit", "os.hub.spaces.list", "os.hub.invite.redeemField"],
  invite: "os.hub.invite.create",
  inviteSupport: ["os.hub.invite.role", "os.hub.invite.expiry", "os.hub.invite.copy"],
} as const;
//#endregion 🔖️Anchors

//#region 🔖️Stages
/** 🪜️ The ordered stages of the hub flow. `done` is not a task — it is the terminal step that closes
 * the tour, so `HUB_FIRST_RUN_STAGES_V1.indexOf(stage)` is also the step index. */
export const HUB_FIRST_RUN_STAGES_V1 = ["welcome", "signIn", "space", "invite", "done"] as const;

export type HubFirstRunStageV1 = (typeof HUB_FIRST_RUN_STAGES_V1)[number];

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
//#endregion 🔖️Stages

//#region 🔖️Text
/** 🌐️ English first, German second, no default language — an unowned locale is refused rather than
 * silently downgraded, exactly as `hubSignInTextV1` refuses one (`AGENTS.md`: "multiple languages
 * with no default language"). Both locales carry the same keys; the component test asserts that. */
export const HUB_FIRST_RUN_TEXT_V1 = {
  en: {
    title: "Working together on a hub",
    welcomeTitle: "What a hub gives you",
    welcomeBody:
      "A hub is a server that holds shared spaces. Everyone signed into the same space sees the same documents, sees each other's cursors, and edits at the same time. Your work stays available locally even when the hub is unreachable.",
    signInTitle: "Sign in",
    signInBody: "Enter the email and password for your account on this hub. If you run the hub yourself, the first account is created with `os-hub credential set`. You can add more than one hub and switch between them.",
    signInTask: "Sign in to a hub",
    spaceTitle: "Create or join a space",
    spaceBody: "A space is one team's shared room. Name a new one to start your own, or paste an invite code someone sent you to join theirs.",
    spaceTaskCreate: "Create a space",
    spaceTaskJoin: "Or redeem an invite",
    inviteTitle: "Invite someone",
    inviteBody: "Create an invite, choose what the other person may do and how long the code stays valid, then send them the code. They redeem it in the same place you did.",
    inviteTask: "Create an invite",
    doneTitle: "That is the whole flow",
    doneBody: "Sign in, be in a space, invite the people you work with. Reopen this walkthrough any time from the command palette.",
  },
  de: {
    title: "Gemeinsam auf einem Hub arbeiten",
    welcomeTitle: "Wofür ein Hub da ist",
    welcomeBody:
      "Ein Hub ist ein Server mit gemeinsamen Räumen. Alle, die im selben Raum angemeldet sind, sehen dieselben Dokumente, sehen die Cursor der anderen und bearbeiten gleichzeitig. Deine Arbeit bleibt lokal verfügbar, auch wenn der Hub nicht erreichbar ist.",
    signInTitle: "Anmelden",
    signInBody: "Gib E-Mail und Passwort deines Kontos auf diesem Hub ein. Wenn du den Hub selbst betreibst, wird das erste Konto mit `os-hub credential set` angelegt. Du kannst mehrere Hubs hinterlegen und zwischen ihnen wechseln.",
    signInTask: "Bei einem Hub anmelden",
    spaceTitle: "Raum erstellen oder beitreten",
    spaceBody: "Ein Raum ist der gemeinsame Arbeitsraum eines Teams. Vergib einen Namen für einen eigenen Raum oder füge einen Einladungscode ein, den dir jemand geschickt hat.",
    spaceTaskCreate: "Einen Raum erstellen",
    spaceTaskJoin: "Oder eine Einladung einlösen",
    inviteTitle: "Jemanden einladen",
    inviteBody: "Erstelle eine Einladung, lege fest, was die andere Person darf und wie lange der Code gültig bleibt, und schicke ihr den Code. Sie löst ihn an derselben Stelle ein wie du.",
    inviteTask: "Eine Einladung erstellen",
    doneTitle: "Das ist der ganze Ablauf",
    doneBody: "Anmelden, in einem Raum sein, die Leute einladen, mit denen du arbeitest. Diese Einführung lässt sich jederzeit über die Befehlspalette erneut öffnen.",
  },
} as const;

export type HubFirstRunLocaleV1 = keyof typeof HUB_FIRST_RUN_TEXT_V1;

/** 🔡️ Refuses unowned locales instead of silently selecting a default language. */
export function hubFirstRunTextV1(locale: string): (typeof HUB_FIRST_RUN_TEXT_V1)[HubFirstRunLocaleV1] {
  if (locale !== "en" && locale !== "de") throw new Error("hub.first-run.locale-unsupported");
  return HUB_FIRST_RUN_TEXT_V1[locale];
}
//#endregion 🔖️Text

//#region 🔖️Definition
function step(
  id: HubFirstRunStageV1,
  title: string,
  body: string,
  introduce: string | null,
  show: readonly string[],
  placement: IntroductionStepDefinition["placement"],
  interactions: IntroductionStepDefinition["interactions"],
): IntroductionStepDefinition {
  return { id: `hub.firstRun.${id}`, title, body, introduce, show: [...show], placement, interactions, ordered: false, logos: [] };
}

/** 🎓️ The whole hub walkthrough, already localized, in stage order. The two screen steps
 * (`welcome`, `done`) carry no anchor so they render centered on any viewport; the three task steps
 * anchor to the live controls and list their work as checklist rows. Interactions are `panel` kinds
 * naming the surface the person must reach, so the step completes by *doing* rather than by pressing
 * Next — the mechanism `IntroductionInteractionKind` already defines. */
export function hubFirstRunIntroductionV1(locale: string): IntroductionDefinition {
  const text = hubFirstRunTextV1(locale);
  const anchors = HUB_FIRST_RUN_ANCHORS_V1;
  return {
    title: text.title,
    steps: [
      step("welcome", text.welcomeTitle, text.welcomeBody, null, [], "center", []),
      step("signIn", text.signInTitle, text.signInBody, anchors.signIn, anchors.signInSupport, "bottom", [
        { on: { kind: "panel", id: anchors.signIn }, label: text.signInTask, celebrate: "os.hub.signIn.signedInAs" },
      ]),
      step("space", text.spaceTitle, text.spaceBody, anchors.space, anchors.spaceSupport, "bottom", [
        { on: { kind: "panel", id: anchors.space }, label: text.spaceTaskCreate, celebrate: "os.hub.spaces.current" },
        { on: { kind: "panel", id: "os.hub.invite.redeemField" }, label: text.spaceTaskJoin, celebrate: "os.hub.invite.redeemed" },
      ]),
      step("invite", text.inviteTitle, text.inviteBody, anchors.invite, anchors.inviteSupport, "bottom", [
        { on: { kind: "panel", id: anchors.invite }, label: text.inviteTask, celebrate: "os.hub.invite.ready" },
      ]),
      step("done", text.doneTitle, text.doneBody, null, [], "center", []),
    ],
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
export const HUB_FIRST_RUN_INTRODUCTION_ID_V1 = "os.hub";
export const HUB_FIRST_RUN_STORAGE_KEY_V1 = `ui.introduction.seen.${HUB_FIRST_RUN_INTRODUCTION_ID_V1}`;
export const HUB_FIRST_RUN_SEEN_VALUE_V1 = "true";

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
