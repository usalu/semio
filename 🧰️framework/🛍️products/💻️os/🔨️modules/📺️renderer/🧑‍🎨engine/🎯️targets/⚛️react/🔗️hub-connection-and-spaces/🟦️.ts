/** 🔗️ Hub connection and spaces exports of the renderer entry: the hub connection lane with its sign-in, space browser and delegation panes, the hub first run walkthrough and the spaces chrome labels. */
import { useHubConnection } from "@semio-tech/framework-os/modules/renderer/engine/elements/HubConnection";
import { hubUiLabel } from "@semio-tech/framework-os/modules/renderer/engine/elements/HubConnection/ui-labels";
import { type HubConnectionPortV1, type HubConnectionValueV1, type HubInviteCapabilityV1, type HubRedemptionStateV1 } from "@semio-tech/framework-os/modules/renderer/engine/elements/HubConnection/connection-contract";
import { createHubConnectionFetchPortV1, type HubFetchResponseV1 } from "@semio-tech/framework-os/modules/renderer/engine/elements/HubConnection/fetch-port";
import { HubSignInPane, hubSignInFormOfferedV1, hubSignInSubmittableV1, type HubSignInPaneProps } from "@semio-tech/framework-os/modules/renderer/engine/elements/HubSignIn";
import { SpaceBrowser, spaceBrowserPresencePeersV1, type SpaceBrowserProps } from "@semio-tech/framework-os/modules/renderer/engine/elements/SpaceBrowser";
import { AgentDelegations, agentDelegationInstantV1, type AgentDelegationsProps } from "@semio-tech/framework-os/modules/renderer/engine/elements/AgentDelegations";
import { HubFirstRun, hubFirstRunStageAtIndex, type HubFirstRunProps } from "@semio-tech/framework-os/modules/renderer/engine/elements/HubFirstRun";

//#region 🔖️HubConnection
/** 🔗️ ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END slice AU2 — the browser-facing hub
 * relationship: the headless `useHubConnection` lane plus the sign-in and end-user spaces surfaces
 * it drives. */
/** 🤖️ ticket 26/09/18 slice M6b — the delegation pane that turns a signed-in human into the issuer
 * of an AI agent's own scoped, revocable principal in the space that is open. */
export { AgentDelegations, HubSignInPane, SpaceBrowser, agentDelegationInstantV1, createHubConnectionFetchPortV1, hubSignInFormOfferedV1, hubSignInSubmittableV1, hubUiLabel, spaceBrowserPresencePeersV1, useHubConnection };
export type { AgentDelegationsProps, HubConnectionPortV1, HubConnectionValueV1, HubFetchResponseV1, HubInviteCapabilityV1, HubRedemptionStateV1, HubSignInPaneProps, SpaceBrowserProps };
//#endregion 🔖️HubConnection

//#region 🔖️HubFirstRun
/** 🎓️ ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END slice D2 (G12 #12) — the hub first-run
 * walkthrough (sign in → create or join a space → invite), rendered through the shell's existing
 * `UIIntroduction` surface rather than a second onboarding chrome. */
export { HubFirstRun, hubFirstRunStageAtIndex };
export type { HubFirstRunProps };
//#endregion 🔖️HubFirstRun

//#region 🔖️SpacesI18n
/** 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS — `ui.home.*`/`ui.space.*`/
 * `ui.presence.*`/`ui.checkin.*`/`ui.identity.*` chrome vocabulary consumed by `ShellHost`'s identity
 * bootstrap, directory-lane, and routing regions. This is new domain-neutral-looking chrome text, so
 * it would normally register into `uiChromeTranslationBundles` — but that dictionary lives in
 * `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`, outside this
 * ticket's lease. Mirrors `ShellHelpers`' own `SurfaceRoleLabels` workaround for the identical
 * situation: a local `{en, de}` pair table resolved directly off a locale, English declared first,
 * no default language (CLAUDE.md). Appended only — never reorders or edits any key above this
 * region. */
export type SpacesUiLabelKey =
  | "ui.home.title"
  | "ui.home.createSpace"
  | "ui.home.emptyState"
  | "ui.space.title"
  | "ui.space.studio"
  | "ui.space.createArtifact"
  | "ui.space.share"
  | "ui.presence.online"
  | "ui.presence.offline"
  | "ui.presence.peers"
  | "ui.checkin.action"
  | "ui.checkin.pending"
  | "ui.checkin.auto"
  | "ui.identity.signIn"
  | "ui.identity.signOut"
  | "ui.identity.offline"
  | "ui.identity.connecting"
  | "ui.sync.status.persisted"
  | "ui.sync.status.pending"
  | "ui.sync.status.remote"
  | "ui.sync.status.remote.connected"
  | "ui.sync.status.remote.connecting"
  | "ui.sync.status.remote.backoff"
  | "ui.sync.status.remote.detached"
  | "ui.sync.browse";

const SPACES_UI_LABELS: Readonly<Record<SpacesUiLabelKey, { readonly en: string; readonly de: string }>> = {
  "ui.home.title": { en: "Spaces", de: "Spaces" },
  "ui.home.createSpace": { en: "Create Space", de: "Space erstellen" },
  "ui.home.emptyState": { en: "No spaces yet", de: "Noch keine Spaces" },
  "ui.space.title": { en: "Space", de: "Space" },
  "ui.space.studio": { en: "Studio", de: "Studio" },
  "ui.space.createArtifact": { en: "Create Artifact", de: "Artefakt erstellen" },
  "ui.space.share": { en: "Share", de: "Teilen" },
  "ui.presence.online": { en: "Online", de: "Online" },
  "ui.presence.offline": { en: "Offline", de: "Offline" },
  "ui.presence.peers": { en: "Peers", de: "Teilnehmer" },
  "ui.checkin.action": { en: "Check In", de: "Einchecken" },
  "ui.checkin.pending": { en: "Uncommitted changes", de: "Nicht übernommene Änderungen" },
  "ui.checkin.auto": { en: "Auto check-in", de: "Automatisches Einchecken" },
  "ui.identity.signIn": { en: "Sign In", de: "Anmelden" },
  "ui.identity.signOut": { en: "Sign Out", de: "Abmelden" },
  "ui.identity.offline": { en: "Working offline", de: "Offline arbeiten" },
  "ui.identity.connecting": { en: "Connecting…", de: "Verbindung wird hergestellt…" },
  "ui.sync.status.persisted": { en: "Persisted", de: "Gespeichert" },
  "ui.sync.status.pending": { en: "Pending", de: "Ausstehend" },
  "ui.sync.status.remote": { en: "Remote", de: "Remote" },
  "ui.sync.status.remote.connected": { en: "connected", de: "verbunden" },
  "ui.sync.status.remote.connecting": { en: "connecting", de: "verbindet" },
  "ui.sync.status.remote.backoff": { en: "backoff", de: "erneuter Versuch" },
  "ui.sync.status.remote.detached": { en: "detached", de: "getrennt" },
  "ui.sync.browse": { en: "Browse…", de: "Durchsuchen…" },
};

/** 📇️ Resolves one of {@link SpacesUiLabelKey}'s frozen bilingual pairs — `locale` accepts any
 * string (a live `UiLocale`/`ShellLocale` value) and falls back to English for anything but `"de"`,
 * matching `ShellHelpers.frozenLabelText`'s own fallback rule. */
export function spacesUiLabel(key: SpacesUiLabelKey, locale: string = "en"): string {
  const pair = SPACES_UI_LABELS[key];
  return locale === "de" ? pair.de : pair.en;
}
//#endregion 🔖️SpacesI18n
