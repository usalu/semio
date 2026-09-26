// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔗️HubConnection/component.tsx
/** @emoji 🔗️ `🔗️HubConnection` — the headless hook that owns a human's relationship with one hub:
 * which hub is chosen, signing in against `POST /auth/sessions`, holding the minted capability in
 * memory only, listing/creating spaces and invitations through the directory command path, and
 * redeeming an invitation. Presentation lives in `🔐️HubSignIn` and `🏘️SpaceBrowser`; every network
 * call enters through the injected {@link HubConnectionPortV1}, so a test drives the real contract
 * against a fake transport. Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AU2; hub-side
 * contract in that ticket's `📓️au1-hub-auth-sessions-and-rate-limit.md` §1.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useEffect, useMemo, useReducer, useRef, useState } from "react";
import { registerUiTranslationBundles } from "@semio-tech/ui-react";
import type { DirectoryCommand, DirectoryCommandReceiptV1, DirectorySpaceKind, DirectorySpaceListEntryV1, DirectorySpaceRole, DirectorySpaceVisibility } from "@semio-tech/framework-os";
import {
  hubSessionInitialStateV1,
  localBootstrapHubConnectionV1,
  readHubConnectionBookV1,
  reduceHubSessionV1,
  removeHubConnectionV1,
  runHubSessionAuthorityV1,
  parseHubSessionMintResultV1,
  runHubSignInV1,
  runHubSignOutV1,
  selectHubConnectionV1,
  selectedHubConnectionV1,
  upsertHubConnectionV1,
  writeHubConnectionBookV1,
  hubConnectionIdForOriginV1,
  parseHubOriginV1,
  type HubConnectionBookV1,
  type HubConnectionStorageV1,
  type HubConnectionV1,
  type HubSessionAuthorityV1,
  type HubSessionStateV1,
  type HubSignInClientClassV1,
  type HubSignInCredentialV1,
  type HubSignInErrorCodeV1,
  type HubSignInTransportV1,
} from "../../../../📇️directory/🔐️sign-in/🟦️.ts";
import {
  archiveSpaceCommandV1,
  createInviteCommandV1,
  createSpaceCommandV1,
  directoryCommandAnsweredV1,
  filterSpaceRowsV1,
  inviteRedemptionErrorFromStatusV1,
  parseInviteTokenV1,
  spaceMemberPresenceV1,
  spaceRowsV1,
  type InviteRedemptionErrorCodeV1,
  type SpaceBrowserPhaseV1,
  type SpaceMemberPresenceV1,
  type SpaceRowV1,
} from "../../../../📇️directory/🏘️spaces/🟦️.ts";
import {
  agentCredentialCommandV1,
  agentCredentialFileV1,
  agentDelegationErrorFromResponseV1,
  agentDelegationListPathV1,
  agentDelegationRevokePathV1,
  agentDelegationRowsV1,
  createAgentDelegationBodyV1,
  parseAgentDelegationListV1,
  parseAgentDelegationReceiptV1,
  agentCredentialInstallRequestV1,
  agentMcpClientConfigJsonV1,
  agentMcpClientConfigV1,
  AgentCredentialInstallUnavailableV1,
  AGENT_DELEGATION_PATH_V1,
  type AgentCredentialInstallReceiptV1,
  type AgentCredentialInstallRequestV1,
  type AgentAudienceV1,
  type AgentDelegationErrorCodeV1,
  type AgentDelegationPhaseV1,
  type AgentDelegationReceiptV1,
  type AgentDelegationRowV1,
  type AgentDelegationSummaryV1,
} from "../../../../📇️directory/🤖️delegations/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🌐️Labels
/** 🌐️ `os.hub.*` — every string the three hub elements render, English first and German second.
 * {@link registerUiTranslationBundles} makes both locales a compile-time requirement: a key added to
 * one and not the other does not type-check. */
export const hubUiLabel = registerUiTranslationBundles({
  en: {
    translation: {
      os: {
        hub: {
          signIn: {
            title: { label: { normal: "Sign in to a hub", beginner: "Join a shared hub" } },
            description: { label: { normal: "Choose a hub and sign in to collaborate. Everything you make stays usable on this device even when no hub is reachable.", beginner: "Pick a hub and sign in to work with other people. Your work stays on this device too." } },
            hub: { label: { normal: "Hub", beginner: "Hub" } },
            addHub: { label: { normal: "Add another hub", beginner: "Add a hub" } },
            hubAddress: { label: { normal: "Hub address", beginner: "Hub address" } },
            addHubSubmit: { label: { normal: "Add hub", beginner: "Add" } },
            forgetHub: { label: { normal: "Forget this hub", beginner: "Remove hub" } },
            email: { label: { normal: "Email", beginner: "Email" } },
            password: { label: { normal: "Password", beginner: "Password" } },
            submit: { label: { normal: "Sign in", beginner: "Sign in" } },
            cancel: { label: { normal: "Cancel sign-in", beginner: "Cancel" } },
            signOut: { label: { normal: "Sign out", beginner: "Sign out" } },
            busy: { label: { normal: "Signing in…", beginner: "Signing in…" } },
            signedInAs: { label: { normal: "Signed in as {{user}}", beginner: "You are {{user}}" } },
            signedOut: { label: { normal: "Not signed in to a hub.", beginner: "You are working on your own." } },
            expired: { label: { normal: "Your session expired. Sign in again to continue collaborating.", beginner: "Please sign in again." } },
            offline: { label: { normal: "No connection to this hub. You can keep working locally.", beginner: "No connection. You can keep working." } },
            localOnly: { label: { normal: "Working on this device only", beginner: "Working on this device only" } },
            errorRegion: { label: { normal: "Sign-in problem", beginner: "Problem" } },
          },
          firstRun: {
            replay: { label: { normal: "How this works", beginner: "Show me how this works" } },
          },
          agent: {
            title: { label: { normal: "AI agents", beginner: "AI helpers" } },
            description: { label: { normal: "Give an AI agent its own access to this space. It acts under its own name, and you can withdraw it at any time.", beginner: "Let an AI helper work in this space. It has its own name and you can stop it at any time." } },
            signedOut: { label: { normal: "Sign in to the hub to give an agent access.", beginner: "Sign in first." } },
            noSpace: { label: { normal: "Open a space to manage the agents that work in it.", beginner: "Open a space first." } },
            notAuthor: { label: { normal: "Only an author of this space can give an agent access to it.", beginner: "Only people who may edit can do this." } },
            createTitle: { label: { normal: "Give an agent access", beginner: "Add an AI helper" } },
            name: { label: { normal: "Agent name", beginner: "Name" } },
            audience: { label: { normal: "The agent may", beginner: "It may" } },
            audienceRead: { label: { normal: "Read only", beginner: "Only look" } },
            audienceEdit: { label: { normal: "Read and edit", beginner: "Look and change" } },
            expiry: { label: { normal: "Access expires after", beginner: "Stops after" } },
            expiryDay: { label: { normal: "1 day", beginner: "1 day" } },
            expiryWeek: { label: { normal: "1 week", beginner: "1 week" } },
            expiryMonth: { label: { normal: "30 days", beginner: "30 days" } },
            create: { label: { normal: "Create delegation", beginner: "Add helper" } },
            ready: { label: { normal: "The credential file is ready. It is shown once — download it now; it cannot be shown again.", beginner: "Download the file now. You cannot get it again." } },
            download: { label: { normal: "Download credential file", beginner: "Download the file" } },
            downloaded: { label: { normal: "Credential file saved.", beginner: "Saved." } },
            downloadFailed: { label: { normal: "Could not save the file. Copy the text below into a file yourself.", beginner: "Could not save. Copy the text below." } },
            dismiss: { label: { normal: "Discard credential", beginner: "Discard" } },
            permission: { label: { normal: "Keep the file readable by you alone — run chmod 600 on it. The agent refuses a file anyone else can read.", beginner: "Only you may read the file. Run chmod 600 on it." } },
            command: { label: { normal: "Start the agent with", beginner: "Start it with" } },
            principal: { label: { normal: "Acts as {{principal}}", beginner: "Called {{principal}}" } },
            mcpTitle: { label: { normal: "Connect an AI client", beginner: "Use it in your AI app" } },
            mcpInstall: { label: { normal: "Set up MCP client", beginner: "Set up" } },
            mcpInstalling: { label: { normal: "Installing the credential for MCP clients…", beginner: "Setting up…" } },
            mcpReady: { label: { normal: "Add this entry to your MCP client's configuration (for example .mcp.json or claude_desktop_config.json). It names a credential file only you can read — the secret itself is not in it.", beginner: "Paste this into your AI app's settings." } },
            mcpCopy: { label: { normal: "Copy MCP configuration", beginner: "Copy" } },
            mcpCopied: { label: { normal: "MCP configuration copied.", beginner: "Copied." } },
            mcpUnavailable: { label: { normal: "This app cannot install the credential for you. Download the file and start the agent with the command below.", beginner: "Download the file and use the command below." } },
            mcpFailed: { label: { normal: "The credential could not be installed. Nothing was written; download the file instead.", beginner: "That did not work. Download the file instead." } },
            mcpRevocable: { label: { normal: "Withdrawing this agent below stops the client at once and removes the installed credential.", beginner: "Stopping the helper stops the app too." } },
            list: { label: { normal: "Agents with access", beginner: "Your AI helpers" } },
            empty: { label: { normal: "No agent has access to this space.", beginner: "No helpers yet." } },
            loading: { label: { normal: "Loading agent delegations…", beginner: "Loading…" } },
            submitting: { label: { normal: "Waiting for the hub…", beginner: "Working…" } },
            failed: { label: { normal: "The hub did not accept that. Nothing was changed.", beginner: "That did not work. Nothing changed." } },
            refresh: { label: { normal: "Refresh agent delegations", beginner: "Refresh" } },
            created: { label: { normal: "Created {{when}}", beginner: "Added {{when}}" } },
            expires: { label: { normal: "Expires {{when}}", beginner: "Stops {{when}}" } },
            lastUsed: { label: { normal: "Last used {{when}}", beginner: "Last used {{when}}" } },
            lastUsedNever: { label: { normal: "Never used", beginner: "Never used" } },
            stateLive: { label: { normal: "Active", beginner: "Working" } },
            stateExpired: { label: { normal: "Expired", beginner: "Stopped" } },
            stateRevoked: { label: { normal: "Withdrawn", beginner: "Stopped by you" } },
            revoke: { label: { normal: "Withdraw {{name}}", beginner: "Stop {{name}}" } },
            revokeConfirmTitle: { label: { normal: "Withdraw access from {{name}}?", beginner: "Stop {{name}}?" } },
            revokeConfirmBody: { label: { normal: "Its open sessions end at once and its credential file stops working. This cannot be undone.", beginner: "It stops right away. You cannot undo this." } },
            revokeConfirm: { label: { normal: "Withdraw access", beginner: "Stop it" } },
            revokeCancel: { label: { normal: "Keep access", beginner: "Keep it" } },
            errorForbidden: { label: { normal: "You may not manage agents in this space.", beginner: "You are not allowed to do this." } },
            errorRateLimited: { label: { normal: "Too many attempts. Try again in a moment.", beginner: "Too fast. Wait a moment." } },
            errorUnreachable: { label: { normal: "The hub could not be reached. Try again in a moment.", beginner: "No connection. Try again." } },
            errorRefused: { label: { normal: "The hub refused that.", beginner: "The hub said no." } },
          },
          spaces: {
            title: { label: { normal: "Spaces",beginner: "Your spaces" } },
            list: { label: { normal: "Your spaces", beginner: "Your spaces" } },
            search: { label: { normal: "Find a space", beginner: "Find a space" } },
            open: { label: { normal: "Open {{name}}", beginner: "Open {{name}}" } },
            current: { label: { normal: "Currently open", beginner: "Open now" } },
            empty: { label: { normal: "No spaces yet. Create one, or redeem an invitation someone sent you.", beginner: "No spaces yet. Make one or use an invitation." } },
            noMatches: { label: { normal: "No space matches that search.", beginner: "Nothing found." } },
            loading: { label: { normal: "Loading your spaces…", beginner: "Loading…" } },
            stale: { label: { normal: "Showing the spaces from your last connection.", beginner: "Showing what we last saw." } },
            refresh: { label: { normal: "Refresh spaces", beginner: "Refresh" } },
            roleAuthor: { label: { normal: "Author", beginner: "Can edit" } },
            roleSpectator: { label: { normal: "Spectator", beginner: "Can view" } },
            accessPublic: { label: { normal: "Public", beginner: "Open to everyone" } },
            memberCount: { label: { normal: "{{count}} members", beginner: "{{count}} people" } },
            documentCount: { label: { normal: "{{count}} documents", beginner: "{{count}} files" } },
            onlineCount: { label: { normal: "{{count}} here now", beginner: "{{count}} here now" } },
            createTitle: { label: { normal: "Create a space", beginner: "Make a space" } },
            createName: { label: { normal: "Space name", beginner: "Name" } },
            createKind: { label: { normal: "Space kind", beginner: "Kind" } },
            createVisibility: { label: { normal: "Visibility", beginner: "Who can see it" } },
            kindAtelier: { label: { normal: "Atelier", beginner: "Atelier" } },
            kindStudio: { label: { normal: "Studio", beginner: "Studio" } },
            visibilityPrivate: { label: { normal: "Private", beginner: "Only members" } },
            visibilityPublic: { label: { normal: "Public", beginner: "Everyone" } },
            createSubmit: { label: { normal: "Create space", beginner: "Create" } },
            archive: { label: { normal: "Archive {{name}}", beginner: "Archive {{name}}" } },
            members: { label: { normal: "Members", beginner: "People" } },
            online: { label: { normal: "Here now", beginner: "Here now" } },
            away: { label: { normal: "Away", beginner: "Away" } },
            owner: { label: { normal: "Owner", beginner: "Owner" } },
            submitting: { label: { normal: "Waiting for the hub…", beginner: "Working…" } },
            failed: { label: { normal: "The hub did not accept that. Nothing was changed.", beginner: "That did not work. Nothing changed." } },
          },
          invite: {
            title: { label: { normal: "Invitations", beginner: "Invitations" } },
            createTitle: { label: { normal: "Invite someone", beginner: "Invite someone" } },
            role: { label: { normal: "They may", beginner: "They may" } },
            expiry: { label: { normal: "Invitation expires after", beginner: "Expires after" } },
            expiryHour: { label: { normal: "1 hour", beginner: "1 hour" } },
            expiryDay: { label: { normal: "1 day", beginner: "1 day" } },
            expiryWeek: { label: { normal: "1 week", beginner: "1 week" } },
            create: { label: { normal: "Create invitation", beginner: "Create invitation" } },
            ready: { label: { normal: "The invitation link is ready. It is shown once — copy it now.", beginner: "Copy the link now. It is shown once." } },
            copy: { label: { normal: "Copy invitation link", beginner: "Copy link" } },
            copied: { label: { normal: "Invitation link copied.", beginner: "Copied." } },
            copyFailed: { label: { normal: "Could not reach the clipboard. Select the link and copy it.", beginner: "Could not copy. Select the link." } },
            dismiss: { label: { normal: "Discard invitation link", beginner: "Discard" } },
            redeemTitle: { label: { normal: "Redeem an invitation", beginner: "Use an invitation" } },
            redeemField: { label: { normal: "Invitation link or code", beginner: "Invitation link or code" } },
            redeem: { label: { normal: "Join space", beginner: "Join" } },
            redeeming: { label: { normal: "Joining…", beginner: "Joining…" } },
            redeemed: { label: { normal: "You joined the space.", beginner: "You joined." } },
            errorInvalid: { label: { normal: "That invitation is not valid.", beginner: "That invitation does not work." } },
            errorExpired: { label: { normal: "That invitation has expired. Ask for a new one.", beginner: "Too late. Ask for a new one." } },
            errorAlreadyMember: { label: { normal: "You are already a member of that space.", beginner: "You are already in it." } },
            errorUnauthorized: { label: { normal: "Sign in to the hub before redeeming an invitation.", beginner: "Sign in first." } },
            errorUnreachable: { label: { normal: "The hub could not be reached. Try again in a moment.", beginner: "No connection. Try again." } },
            errorRefused: { label: { normal: "The hub refused that invitation.", beginner: "The hub said no." } },
            errorCancelled: { label: { normal: "Redeeming was cancelled.", beginner: "Cancelled." } },
          },
        },
      },
    },
  },
  de: {
    translation: {
      os: {
        hub: {
          signIn: {
            title: { label: { normal: "Bei einem Hub anmelden", beginner: "Einem geteilten Hub beitreten" } },
            description: { label: { normal: "Wähle einen Hub und melde dich an, um zusammenzuarbeiten. Alles, was du erstellst, bleibt auf diesem Gerät nutzbar, auch wenn kein Hub erreichbar ist.", beginner: "Wähle einen Hub und melde dich an, um mit anderen zu arbeiten. Deine Arbeit bleibt auch auf diesem Gerät." } },
            hub: { label: { normal: "Hub", beginner: "Hub" } },
            addHub: { label: { normal: "Weiteren Hub hinzufügen", beginner: "Hub hinzufügen" } },
            hubAddress: { label: { normal: "Hub-Adresse", beginner: "Hub-Adresse" } },
            addHubSubmit: { label: { normal: "Hub hinzufügen", beginner: "Hinzufügen" } },
            forgetHub: { label: { normal: "Diesen Hub vergessen", beginner: "Hub entfernen" } },
            email: { label: { normal: "E-Mail", beginner: "E-Mail" } },
            password: { label: { normal: "Passwort", beginner: "Passwort" } },
            submit: { label: { normal: "Anmelden", beginner: "Anmelden" } },
            cancel: { label: { normal: "Anmeldung abbrechen", beginner: "Abbrechen" } },
            signOut: { label: { normal: "Abmelden", beginner: "Abmelden" } },
            busy: { label: { normal: "Anmeldung läuft…", beginner: "Anmeldung läuft…" } },
            signedInAs: { label: { normal: "Angemeldet als {{user}}", beginner: "Du bist {{user}}" } },
            signedOut: { label: { normal: "Nicht bei einem Hub angemeldet.", beginner: "Du arbeitest für dich allein." } },
            expired: { label: { normal: "Deine Sitzung ist abgelaufen. Melde dich erneut an, um weiter zusammenzuarbeiten.", beginner: "Bitte melde dich erneut an." } },
            offline: { label: { normal: "Keine Verbindung zu diesem Hub. Du kannst lokal weiterarbeiten.", beginner: "Keine Verbindung. Du kannst weiterarbeiten." } },
            localOnly: { label: { normal: "Nur auf diesem Gerät", beginner: "Nur auf diesem Gerät" } },
            errorRegion: { label: { normal: "Problem bei der Anmeldung", beginner: "Problem" } },
          },
          firstRun: {
            replay: { label: { normal: "So funktioniert das", beginner: "Zeig mir, wie das geht" } },
          },
          agent: {
            title: { label: { normal: "KI-Agenten", beginner: "KI-Helfer" } },
            description: { label: { normal: "Gib einem KI-Agenten einen eigenen Zugang zu diesem Space. Er handelt unter eigenem Namen, und du kannst den Zugang jederzeit zurückziehen.", beginner: "Lass einen KI-Helfer in diesem Space arbeiten. Er hat einen eigenen Namen, und du kannst ihn jederzeit stoppen." } },
            signedOut: { label: { normal: "Melde dich beim Hub an, um einem Agenten Zugang zu geben.", beginner: "Melde dich zuerst an." } },
            noSpace: { label: { normal: "Öffne einen Space, um seine Agenten zu verwalten.", beginner: "Öffne zuerst einen Space." } },
            notAuthor: { label: { normal: "Nur ein Autor dieses Spaces kann einem Agenten Zugang geben.", beginner: "Das dürfen nur Personen, die bearbeiten können." } },
            createTitle: { label: { normal: "Einem Agenten Zugang geben", beginner: "KI-Helfer hinzufügen" } },
            name: { label: { normal: "Name des Agenten", beginner: "Name" } },
            audience: { label: { normal: "Der Agent darf", beginner: "Er darf" } },
            audienceRead: { label: { normal: "Nur lesen", beginner: "Nur schauen" } },
            audienceEdit: { label: { normal: "Lesen und bearbeiten", beginner: "Schauen und ändern" } },
            expiry: { label: { normal: "Zugang läuft ab nach", beginner: "Endet nach" } },
            expiryDay: { label: { normal: "1 Tag", beginner: "1 Tag" } },
            expiryWeek: { label: { normal: "1 Woche", beginner: "1 Woche" } },
            expiryMonth: { label: { normal: "30 Tage", beginner: "30 Tage" } },
            create: { label: { normal: "Zugang erstellen", beginner: "Helfer hinzufügen" } },
            ready: { label: { normal: "Die Zugangsdatei ist bereit. Sie wird nur einmal angezeigt — lade sie jetzt herunter; ein zweites Mal ist nicht möglich.", beginner: "Lade die Datei jetzt herunter. Ein zweites Mal geht nicht." } },
            download: { label: { normal: "Zugangsdatei herunterladen", beginner: "Datei herunterladen" } },
            downloaded: { label: { normal: "Zugangsdatei gespeichert.", beginner: "Gespeichert." } },
            downloadFailed: { label: { normal: "Die Datei konnte nicht gespeichert werden. Kopiere den Text unten selbst in eine Datei.", beginner: "Speichern nicht möglich. Kopiere den Text unten." } },
            dismiss: { label: { normal: "Zugangsdatei verwerfen", beginner: "Verwerfen" } },
            permission: { label: { normal: "Nur du darfst die Datei lesen können — führe chmod 600 darauf aus. Der Agent verweigert eine Datei, die andere lesen können.", beginner: "Nur du darfst die Datei lesen. Führe chmod 600 darauf aus." } },
            command: { label: { normal: "Starte den Agenten mit", beginner: "Starte ihn mit" } },
            principal: { label: { normal: "Handelt als {{principal}}", beginner: "Heißt {{principal}}" } },
            mcpTitle: { label: { normal: "KI-Client verbinden", beginner: "In deiner KI-App nutzen" } },
            mcpInstall: { label: { normal: "MCP-Client einrichten", beginner: "Einrichten" } },
            mcpInstalling: { label: { normal: "Zugangsdaten für MCP-Clients werden installiert…", beginner: "Wird eingerichtet…" } },
            mcpReady: { label: { normal: "Füge diesen Eintrag in die Konfiguration deines MCP-Clients ein (zum Beispiel .mcp.json oder claude_desktop_config.json). Er nennt eine Zugangsdatei, die nur du lesen kannst — das Geheimnis selbst steht nicht darin.", beginner: "Füge das in die Einstellungen deiner KI-App ein." } },
            mcpCopy: { label: { normal: "MCP-Konfiguration kopieren", beginner: "Kopieren" } },
            mcpCopied: { label: { normal: "MCP-Konfiguration kopiert.", beginner: "Kopiert." } },
            mcpUnavailable: { label: { normal: "Diese App kann die Zugangsdaten nicht für dich installieren. Lade die Datei herunter und starte den Agenten mit dem Befehl unten.", beginner: "Lade die Datei herunter und nutze den Befehl unten." } },
            mcpFailed: { label: { normal: "Die Zugangsdaten konnten nicht installiert werden. Es wurde nichts geschrieben; lade stattdessen die Datei herunter.", beginner: "Das hat nicht geklappt. Lade stattdessen die Datei herunter." } },
            mcpRevocable: { label: { normal: "Ziehst du diesen Agenten unten zurück, stoppt der Client sofort und die installierten Zugangsdaten werden entfernt.", beginner: "Stoppst du den Helfer, stoppt auch die App." } },
            list: { label: { normal: "Agenten mit Zugang", beginner: "Deine KI-Helfer" } },
            empty: { label: { normal: "Kein Agent hat Zugang zu diesem Space.", beginner: "Noch keine Helfer." } },
            loading: { label: { normal: "Agenten-Zugänge werden geladen…", beginner: "Wird geladen…" } },
            submitting: { label: { normal: "Warten auf den Hub…", beginner: "Wird ausgeführt…" } },
            failed: { label: { normal: "Der Hub hat das nicht angenommen. Es wurde nichts geändert.", beginner: "Das hat nicht geklappt. Nichts wurde geändert." } },
            refresh: { label: { normal: "Agenten-Zugänge aktualisieren", beginner: "Aktualisieren" } },
            created: { label: { normal: "Erstellt {{when}}", beginner: "Hinzugefügt {{when}}" } },
            expires: { label: { normal: "Läuft ab {{when}}", beginner: "Endet {{when}}" } },
            lastUsed: { label: { normal: "Zuletzt genutzt {{when}}", beginner: "Zuletzt genutzt {{when}}" } },
            lastUsedNever: { label: { normal: "Nie genutzt", beginner: "Nie genutzt" } },
            stateLive: { label: { normal: "Aktiv", beginner: "Arbeitet" } },
            stateExpired: { label: { normal: "Abgelaufen", beginner: "Beendet" } },
            stateRevoked: { label: { normal: "Zurückgezogen", beginner: "Von dir gestoppt" } },
            revoke: { label: { normal: "{{name}} zurückziehen", beginner: "{{name}} stoppen" } },
            revokeConfirmTitle: { label: { normal: "Zugang von {{name}} zurückziehen?", beginner: "{{name}} stoppen?" } },
            revokeConfirmBody: { label: { normal: "Die offenen Sitzungen enden sofort und die Zugangsdatei funktioniert nicht mehr. Das lässt sich nicht rückgängig machen.", beginner: "Er stoppt sofort. Du kannst das nicht rückgängig machen." } },
            revokeConfirm: { label: { normal: "Zugang zurückziehen", beginner: "Stoppen" } },
            revokeCancel: { label: { normal: "Zugang behalten", beginner: "Behalten" } },
            errorForbidden: { label: { normal: "Du darfst die Agenten dieses Spaces nicht verwalten.", beginner: "Das darfst du nicht." } },
            errorRateLimited: { label: { normal: "Zu viele Versuche. Versuche es gleich erneut.", beginner: "Zu schnell. Warte einen Moment." } },
            errorUnreachable: { label: { normal: "Der Hub war nicht erreichbar. Versuche es gleich erneut.", beginner: "Keine Verbindung. Versuche es erneut." } },
            errorRefused: { label: { normal: "Der Hub hat das abgelehnt.", beginner: "Der Hub hat abgelehnt." } },
          },
          spaces: {
            title: { label: { normal: "Spaces", beginner: "Deine Spaces" } },
            list: { label: { normal: "Deine Spaces", beginner: "Deine Spaces" } },
            search: { label: { normal: "Space suchen", beginner: "Space suchen" } },
            open: { label: { normal: "{{name}} öffnen", beginner: "{{name}} öffnen" } },
            current: { label: { normal: "Gerade geöffnet", beginner: "Jetzt offen" } },
            empty: { label: { normal: "Noch keine Spaces. Erstelle einen oder löse eine Einladung ein.", beginner: "Noch keine Spaces. Erstelle einen oder nutze eine Einladung." } },
            noMatches: { label: { normal: "Kein Space passt zu dieser Suche.", beginner: "Nichts gefunden." } },
            loading: { label: { normal: "Deine Spaces werden geladen…", beginner: "Wird geladen…" } },
            stale: { label: { normal: "Angezeigt werden die Spaces der letzten Verbindung.", beginner: "Angezeigt wird der letzte Stand." } },
            refresh: { label: { normal: "Spaces aktualisieren", beginner: "Aktualisieren" } },
            roleAuthor: { label: { normal: "Autor", beginner: "Darf bearbeiten" } },
            roleSpectator: { label: { normal: "Zuschauer", beginner: "Darf ansehen" } },
            accessPublic: { label: { normal: "Öffentlich", beginner: "Für alle offen" } },
            memberCount: { label: { normal: "{{count}} Mitglieder", beginner: "{{count}} Personen" } },
            documentCount: { label: { normal: "{{count}} Dokumente", beginner: "{{count}} Dateien" } },
            onlineCount: { label: { normal: "{{count}} gerade hier", beginner: "{{count}} gerade hier" } },
            createTitle: { label: { normal: "Space erstellen", beginner: "Space anlegen" } },
            createName: { label: { normal: "Space-Name", beginner: "Name" } },
            createKind: { label: { normal: "Space-Art", beginner: "Art" } },
            createVisibility: { label: { normal: "Sichtbarkeit", beginner: "Wer ihn sehen kann" } },
            kindAtelier: { label: { normal: "Atelier", beginner: "Atelier" } },
            kindStudio: { label: { normal: "Studio", beginner: "Studio" } },
            visibilityPrivate: { label: { normal: "Privat", beginner: "Nur Mitglieder" } },
            visibilityPublic: { label: { normal: "Öffentlich", beginner: "Alle" } },
            createSubmit: { label: { normal: "Space erstellen", beginner: "Erstellen" } },
            archive: { label: { normal: "{{name}} archivieren", beginner: "{{name}} archivieren" } },
            members: { label: { normal: "Mitglieder", beginner: "Personen" } },
            online: { label: { normal: "Gerade hier", beginner: "Gerade hier" } },
            away: { label: { normal: "Abwesend", beginner: "Abwesend" } },
            owner: { label: { normal: "Eigentümer", beginner: "Eigentümer" } },
            submitting: { label: { normal: "Warten auf den Hub…", beginner: "Wird ausgeführt…" } },
            failed: { label: { normal: "Der Hub hat das nicht angenommen. Es wurde nichts geändert.", beginner: "Das hat nicht geklappt. Nichts wurde geändert." } },
          },
          invite: {
            title: { label: { normal: "Einladungen", beginner: "Einladungen" } },
            createTitle: { label: { normal: "Jemanden einladen", beginner: "Jemanden einladen" } },
            role: { label: { normal: "Die Person darf", beginner: "Die Person darf" } },
            expiry: { label: { normal: "Einladung läuft ab nach", beginner: "Läuft ab nach" } },
            expiryHour: { label: { normal: "1 Stunde", beginner: "1 Stunde" } },
            expiryDay: { label: { normal: "1 Tag", beginner: "1 Tag" } },
            expiryWeek: { label: { normal: "1 Woche", beginner: "1 Woche" } },
            create: { label: { normal: "Einladung erstellen", beginner: "Einladung erstellen" } },
            ready: { label: { normal: "Der Einladungslink ist bereit. Er wird nur einmal angezeigt — kopiere ihn jetzt.", beginner: "Kopiere den Link jetzt. Er wird nur einmal angezeigt." } },
            copy: { label: { normal: "Einladungslink kopieren", beginner: "Link kopieren" } },
            copied: { label: { normal: "Einladungslink kopiert.", beginner: "Kopiert." } },
            copyFailed: { label: { normal: "Die Zwischenablage war nicht erreichbar. Markiere den Link und kopiere ihn.", beginner: "Kopieren nicht möglich. Markiere den Link." } },
            dismiss: { label: { normal: "Einladungslink verwerfen", beginner: "Verwerfen" } },
            redeemTitle: { label: { normal: "Einladung einlösen", beginner: "Einladung nutzen" } },
            redeemField: { label: { normal: "Einladungslink oder Code", beginner: "Einladungslink oder Code" } },
            redeem: { label: { normal: "Space beitreten", beginner: "Beitreten" } },
            redeeming: { label: { normal: "Beitritt läuft…", beginner: "Beitritt läuft…" } },
            redeemed: { label: { normal: "Du bist dem Space beigetreten.", beginner: "Du bist beigetreten." } },
            errorInvalid: { label: { normal: "Diese Einladung ist nicht gültig.", beginner: "Diese Einladung funktioniert nicht." } },
            errorExpired: { label: { normal: "Diese Einladung ist abgelaufen. Bitte um eine neue.", beginner: "Zu spät. Bitte um eine neue." } },
            errorAlreadyMember: { label: { normal: "Du bist bereits Mitglied dieses Spaces.", beginner: "Du bist schon dabei." } },
            errorUnauthorized: { label: { normal: "Melde dich beim Hub an, bevor du eine Einladung einlöst.", beginner: "Melde dich zuerst an." } },
            errorUnreachable: { label: { normal: "Der Hub war nicht erreichbar. Versuche es gleich erneut.", beginner: "Keine Verbindung. Versuche es erneut." } },
            errorRefused: { label: { normal: "Der Hub hat diese Einladung abgelehnt.", beginner: "Der Hub hat abgelehnt." } },
            errorCancelled: { label: { normal: "Das Einlösen wurde abgebrochen.", beginner: "Abgebrochen." } },
          },
        },
      },
    },
  },
});
//#endregion 🌐️Labels

/** ⏳️ The longest a single re-authentication timer is allowed to sleep. A session TTL may be a year
 * (`MAX_SESSION_TTL_SECS`), which overflows `setTimeout`'s 32-bit delay and fires immediately; the
 * timer re-arms from the remaining deadline on every wake instead. */
export const HUB_SESSION_AUTHORITY_RECHECK_MAX_MS = 30 * 60 * 1000;

//#region 🔖️Port
/** 🔌️ Everything `useHubConnection` needs from the outside world. `signIn` is the auth transport;
 * the three directory calls are the space/command/invite lanes. A fake implementing exactly this
 * drives every test, and the shell's adapter is {@link createHubConnectionFetchPortV1}. */
export interface HubConnectionPortV1 {
  readonly signIn: HubSignInTransportV1;
  readonly storage: HubConnectionStorageV1 | null;
  readonly bootstrapOrigin: string;
  readonly deviceInstanceId: string;
  readonly clientClass: HubSignInClientClassV1;
  /** 🎫️ The session a previous page load of this browsing context left behind, if any. Its presence
   * is what turns a reload into a re-bootstrap rather than a sign-in form; the hub still has the
   * last word, because the hook confirms it with one `GET /auth/sessions/me` before believing it. */
  readonly restoredCapability?: Readonly<{ userId: string }> | null;
  listSpaces(origin: string, signal: AbortSignal): Promise<readonly DirectorySpaceListEntryV1[]>;
  /** 👥️ `GET /directory/spaces/{id}` — the hub's own administration page, whose `members` window is
   * the only authoritative roster. A non-member's hub answers `404`, which reaches the caller as an
   * empty roster rather than an error. */
  readSpaceMembers(origin: string, spaceId: string, signal: AbortSignal): Promise<readonly HubSpaceMemberRowV1[]>;
  submitCommand(origin: string, command: DirectoryCommand, signal: AbortSignal): Promise<DirectoryCommandReceiptV1>;
  redeemInvite(origin: string, token: string, signal: AbortSignal): Promise<Readonly<{ status: number }>>;
  writeClipboard?(text: string): Promise<void>;
  /** 🤖️ `GET /auth/agent-delegations?space=<id>` — this human's agent delegations in one space,
   * revoked ones included so the pane can show what was withdrawn. Never a token, never a selector. */
  listAgentDelegations(origin: string, spaceId: string, signal: AbortSignal): Promise<readonly AgentDelegationSummaryV1[]>;
  /** 🤖️ `POST /auth/agent-delegations` — the one call that ever yields a delegation capability, and
   * it yields it exactly once. The receipt is handed straight to the pane and never stored. */
  createAgentDelegation(origin: string, body: string, signal: AbortSignal): Promise<AgentDelegationReceiptV1>;
  /** 🤖️ `DELETE /auth/agent-delegations/{id}` — withdrawal, which cascades to every live agent
   * session minted from it. */
  revokeAgentDelegation(origin: string, delegationId: string, signal: AbortSignal): Promise<void>;
  /** 📄️ Hands the human a file to save. Separate from the clipboard port because a credential must
   * end up in a file the agent can read, not in a paste buffer. */
  saveFile?(file: Readonly<{ fileName: string; contents: string; mediaType: string }>): Promise<void>;
  /** 🔌️ Installs one delegation's credential file owner-only where `semio-os-mcp` reads it and
   * answers with its absolute path and this host's MCP launcher. Absent on a host that cannot write
   * files for the human (a plain browser); the pane then offers the download and the command. */
  installAgentCredential?(request: AgentCredentialInstallRequestV1, signal: AbortSignal): Promise<AgentCredentialInstallReceiptV1>;
  /** 🗑️ Removes an installed credential once its delegation is withdrawn. */
  uninstallAgentCredential?(delegationId: string): Promise<void>;
}

/** 🤖️ One refusal carrying the hub's own code, so the pane names a cause instead of a status. */
export class AgentDelegationRefusalV1 extends Error {
  readonly code: AgentDelegationErrorCodeV1;

  constructor(code: AgentDelegationErrorCodeV1) {
    super(`hub.agent-delegation.${code}`);
    this.name = "AgentDelegationRefusalV1";
    this.code = code;
  }
}

/** 🎁️ The one-time delegation capability, held here and nowhere else — the invitation lane's twin.
 * `receipt.token` never enters the connection book, a URL or a log line: it is handed to
 * `🤖️AgentDelegations` only while the human is looking at it, and `dismissDelegation` drops it. */
export interface HubAgentCredentialV1 {
  readonly receipt: AgentDelegationReceiptV1;
  readonly file: Readonly<{ fileName: string; contents: string; mediaType: string }>;
  readonly command: string;
  readonly save: "idle" | "saved" | "failed";
  /** 🔌️ The MCP client configuration for this delegation, once the host installed its credential. */
  readonly mcpClient: HubAgentMcpClientV1;
}

/** 🔌️ Where setting up an MCP client for one delegation stands. `unavailable` is a host that cannot
 * install files (the pane offers the download and command instead); `config` is the exact text a
 * human pastes into their client's configuration. */
export interface HubAgentMcpClientV1 {
  readonly phase: "idle" | "installing" | "ready" | "copied" | "unavailable" | "failed";
  readonly config: string | null;
}

/** 👥️ One roster row as the hub's space-administration page serves it
 * (`DirectorySpaceAdministrationMemberRowV1`). Declared here so the port has no dependency on the
 * whole administration page type. */
export interface HubSpaceMemberRowV1 {
  readonly userId: string;
  readonly displayName: string;
  readonly email: string;
  readonly role: DirectorySpaceRole;
  readonly owner: boolean;
}

/** 🎟️ The one-shot invitation capability, held here and nowhere else. `token` never enters the
 * connection book, a URL, a log line or React's devtools-visible props of any persisted element —
 * it is handed to `🏘️SpaceBrowser` only while the human is looking at it. */
export interface HubInviteCapabilityV1 {
  readonly spaceId: string;
  readonly link: string;
  readonly copy: "idle" | "copied" | "failed";
}

export interface HubRedemptionStateV1 {
  readonly phase: "idle" | "redeeming" | "redeemed" | "failed";
  readonly error: InviteRedemptionErrorCodeV1 | null;
}

/** 🏘️ The whole hub surface state one shell mounts. */
export interface HubConnectionValueV1 {
  readonly book: HubConnectionBookV1;
  readonly connection: HubConnectionV1;
  readonly session: HubSessionStateV1;
  /** 🪪️ The last `GET /auth/sessions/me` answer, or `null` before the first read. */
  readonly authority: HubSessionAuthorityV1 | null;
  readonly members: readonly SpaceMemberPresenceV1[];
  readonly rows: readonly SpaceRowV1[];
  readonly spacesPhase: SpaceBrowserPhaseV1;
  readonly search: string;
  readonly invite: HubInviteCapabilityV1 | null;
  readonly redemption: HubRedemptionStateV1;
  /** 🤖️ The agent delegations of the space currently being watched, newest live first. */
  readonly delegations: readonly AgentDelegationRowV1[];
  readonly delegationPhase: AgentDelegationPhaseV1;
  readonly delegationError: AgentDelegationErrorCodeV1 | null;
  /** 🎁️ The delegation capability minted a moment ago, readable exactly once. */
  readonly agentCredential: HubAgentCredentialV1 | null;
  selectConnection(id: string): void;
  addRemoteHub(typed: string, label: string): HubSignInErrorCodeV1 | "invalid-origin" | null;
  forgetHub(id: string): void;
  signIn(credential: Readonly<{ email: string; password: string }>): void;
  cancelSignIn(): void;
  signOut(): void;
  refreshSpaces(): void;
  /** 👥️ Re-reads the roster for one space; `null` clears it. */
  watchSpaceMembers(spaceId: string | null): void;
  setSearch(query: string): void;
  createSpace(name: string, kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility): void;
  archiveSpace(spaceId: string): void;
  createInvite(spaceId: string, role: DirectorySpaceRole, ttlSecs: number): void;
  copyInvite(): void;
  dismissInvite(): void;
  redeemInvite(typed: string): void;
  refreshDelegations(): void;
  createDelegation(agentLabel: string, audience: AgentAudienceV1, ttlSecs: number): void;
  downloadAgentCredential(): void;
  dismissAgentCredential(): void;
  installAgentMcpClient(): void;
  copyAgentMcpClientConfig(): void;
  revokeDelegation(delegationId: string): void;
}

export interface HubConnectionOperationOwnerV1 {
  readonly generation: number;
  readonly connectionId: string;
  readonly origin: string;
}

/** 🪪️ Accepts a completion only while both its connection identity and selection generation remain current. */
export function hubConnectionOperationOwnerCurrentV1(owner: HubConnectionOperationOwnerV1, generation: number, connectionId: string): boolean {
  return owner.generation === generation && owner.connectionId === connectionId;
}
//#endregion 🔖️Port

//#region 🔖️Hook
function useLiveRef<T>(value: T): { current: T } {
  const ref = useRef(value);
  ref.current = value;
  return ref;
}

/** 🔗️ Mounts one hub relationship. Every async lane carries its own `AbortController` and
 * connection generation so a switch, cancel press or unmount rejects even abort-insensitive completions.
 * The session capability lives inside the port, never in this hook's state, so React never serializes
 * it into a snapshot. */
export function useHubConnection(port: HubConnectionPortV1, onlineUserIds: readonly string[] = []): HubConnectionValueV1 {
  const [book, setBook] = useState<HubConnectionBookV1>(() => readHubConnectionBookV1(port.storage, port.bootstrapOrigin));
  const connection = selectedHubConnectionV1(book);
  const [session, dispatchSession] = useReducer(reduceHubSessionV1, connection.id, hubSessionInitialStateV1);
  const [rows, setRows] = useState<readonly SpaceRowV1[]>([]);
  const [spacesPhase, setSpacesPhase] = useState<SpaceBrowserPhaseV1>("loading");
  const [search, setSearch] = useState("");
  const [invite, setInvite] = useState<HubInviteCapabilityV1 | null>(null);
  const [redemption, setRedemption] = useState<HubRedemptionStateV1>({ phase: "idle", error: null });
  const [authority, setAuthority] = useState<HubSessionAuthorityV1 | null>(null);
  const [memberRows, setMemberRows] = useState<readonly HubSpaceMemberRowV1[]>([]);
  const [watchedSpaceId, setWatchedSpaceId] = useState<string | null>(null);
  const [delegationSummaries, setDelegationSummaries] = useState<readonly AgentDelegationSummaryV1[]>([]);
  const [delegationPhase, setDelegationPhase] = useState<AgentDelegationPhaseV1>("idle");
  const [delegationError, setDelegationError] = useState<AgentDelegationErrorCodeV1 | null>(null);
  const [agentCredential, setAgentCredential] = useState<HubAgentCredentialV1 | null>(null);
  const delegationAbort = useRef<AbortController | null>(null);
  const rebootstrapped = useRef(false);
  const signInAbort = useRef<AbortController | null>(null);
  const authorityAbort = useRef<AbortController | null>(null);
  const membersAbort = useRef<AbortController | null>(null);
  const spacesAbort = useRef<AbortController | null>(null);
  const commandAbort = useRef<AbortController | null>(null);
  const signOutAbort = useRef<AbortController | null>(null);
  const redemptionAbort = useRef<AbortController | null>(null);
  const operationGeneration = useRef(1);
  const operationConnectionId = useRef(connection.id);
  const portRef = useLiveRef(port);
  const connectionRef = useLiveRef(connection);
  const watchedSpaceIdRef = useLiveRef(watchedSpaceId);

  if (operationConnectionId.current !== connection.id) {
    operationGeneration.current++;
    operationConnectionId.current = connection.id;
  }

  const captureOperationOwner = useCallback((): HubConnectionOperationOwnerV1 => ({ generation: operationGeneration.current, connectionId: connectionRef.current.id, origin: connectionRef.current.origin }), [connectionRef]);
  const operationOwnerCurrent = useCallback((owner: HubConnectionOperationOwnerV1): boolean => hubConnectionOperationOwnerCurrentV1(owner, operationGeneration.current, operationConnectionId.current), []);

  const retireOperationsForConnection = useCallback((connectionId: string) => {
    if (operationConnectionId.current === connectionId) return;
    operationGeneration.current++;
    operationConnectionId.current = connectionId;
    signInAbort.current?.abort();
    signOutAbort.current?.abort();
    commandAbort.current?.abort();
    authorityAbort.current?.abort();
    membersAbort.current?.abort();
    spacesAbort.current?.abort();
    redemptionAbort.current?.abort();
    delegationAbort.current?.abort();
    signInAbort.current = null;
    signOutAbort.current = null;
    commandAbort.current = null;
    authorityAbort.current = null;
    membersAbort.current = null;
    spacesAbort.current = null;
    redemptionAbort.current = null;
    delegationAbort.current = null;
  }, []);

  const persist = useCallback((next: HubConnectionBookV1) => {
    writeHubConnectionBookV1(portRef.current.storage, next);
    setBook(next);
  }, [portRef]);

  const loadSpaces = useCallback((owner = captureOperationOwner()) => {
    if (!operationOwnerCurrent(owner)) return;
    spacesAbort.current?.abort();
    const abort = new AbortController();
    spacesAbort.current = abort;
    setSpacesPhase((current) => (current === "ready" || current === "stale" ? current : "loading"));
    void portRef.current
      .listSpaces(owner.origin, abort.signal)
      .then((entries) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setRows(spaceRowsV1(entries));
        setSpacesPhase("ready");
        dispatchSession({ kind: "connectivity", offline: false });
      })
      .catch(() => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setSpacesPhase((current) => (current === "loading" ? "failed" : "stale"));
        dispatchSession({ kind: "connectivity", offline: true });
      });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  /** 🪪️ Reads `GET /auth/sessions/me`. This is the only source of `expiresAtMs` — AU1 §1.1 keeps the
   * expiry out of the mint response precisely so the two can never disagree — and a `401` here is
   * the hub's own statement that the capability is gone, which is what drives re-authentication. */
  const readAuthority = useCallback((owner = captureOperationOwner()) => {
    if (!operationOwnerCurrent(owner)) return;
    authorityAbort.current?.abort();
    const abort = new AbortController();
    authorityAbort.current = abort;
    void runHubSessionAuthorityV1(portRef.current.signIn, owner.origin, abort.signal).then((outcome) => {
      if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
      if (outcome.kind === "authority") {
        setAuthority(outcome.authority);
        dispatchSession({ kind: "minted", userId: outcome.authority.userId, expiresAtMs: outcome.authority.expiresAtMs });
        dispatchSession({ kind: "connectivity", offline: false });
        return;
      }
      if (outcome.kind === "expired") {
        setAuthority(null);
        dispatchSession({ kind: "expired" });
        return;
      }
      if (outcome.code === "unreachable") dispatchSession({ kind: "connectivity", offline: true });
    });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  const loadMembers = useCallback((spaceId: string | null, owner = captureOperationOwner()) => {
    if (!operationOwnerCurrent(owner)) return;
    membersAbort.current?.abort();
    if (spaceId === null) {
      setMemberRows([]);
      return;
    }
    const abort = new AbortController();
    membersAbort.current = abort;
    void portRef.current
      .readSpaceMembers(owner.origin, spaceId, abort.signal)
      .then((rows) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setMemberRows(rows);
      })
      .catch(() => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setMemberRows([]);
      });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  /** 🤖️ Reads `GET /auth/agent-delegations?space=<id>`. `null` clears the lane without a round trip:
   * a delegation belongs to exactly one space, so there is nothing to show until one is open. */
  const loadDelegations = useCallback((spaceId: string | null, owner = captureOperationOwner()) => {
    if (!operationOwnerCurrent(owner)) return;
    delegationAbort.current?.abort();
    if (spaceId === null) {
      setDelegationSummaries([]);
      setDelegationPhase("idle");
      setDelegationError(null);
      return;
    }
    const abort = new AbortController();
    delegationAbort.current = abort;
    setDelegationPhase((current) => (current === "ready" ? current : "loading"));
    void portRef.current
      .listAgentDelegations(owner.origin, spaceId, abort.signal)
      .then((summaries) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setDelegationSummaries(summaries);
        setDelegationPhase("ready");
        setDelegationError(null);
      })
      .catch((error: unknown) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setDelegationPhase("failed");
        setDelegationError(error instanceof AgentDelegationRefusalV1 ? error.code : "unreachable");
      });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  const watchSpaceMembers = useCallback((spaceId: string | null) => {
    setWatchedSpaceId(spaceId);
    loadMembers(spaceId);
    setAgentCredential(null);
    loadDelegations(spaceId);
  }, [loadDelegations, loadMembers]);

  const runCommand = useCallback(
    (command: DirectoryCommand, onReceipt: (receipt: DirectoryCommandReceiptV1) => void) => {
      const owner = captureOperationOwner();
      commandAbort.current?.abort();
      const abort = new AbortController();
      commandAbort.current = abort;
      setSpacesPhase("submitting");
      void portRef.current
        .submitCommand(owner.origin, command, abort.signal)
        .then((receipt) => {
          if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
          onReceipt(receipt);
          loadSpaces(owner);
          loadMembers(watchedSpaceIdRef.current, owner);
        })
        .catch(() => {
          if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
          setSpacesPhase("failed");
        });
    },
    [captureOperationOwner, loadMembers, loadSpaces, operationOwnerCurrent, portRef, watchedSpaceIdRef],
  );

  useEffect(() => {
    loadSpaces();
    return () => spacesAbort.current?.abort();
  }, [connection.id, loadSpaces]);

  /** ♻️ Re-bootstrap after a reload: the port restored a capability this browsing context minted
   * earlier, so the session is presumed live only until the hub's own `me` answer confirms or
   * refuses it. Mount-only — a later sign-out must not resurrect the session it just ended. */
  useEffect(() => {
    const restored = portRef.current.restoredCapability;
    if (!restored || rebootstrapped.current) return;
    rebootstrapped.current = true;
    dispatchSession({ kind: "minted", userId: restored.userId, expiresAtMs: null });
    readAuthority();
  }, [portRef, readAuthority, rebootstrapped]);

  useEffect(() => {
    const online = (): void => dispatchSession({ kind: "connectivity", offline: false });
    const offline = (): void => dispatchSession({ kind: "connectivity", offline: true });
    const target = typeof globalThis.addEventListener === "function" ? globalThis : null;
    target?.addEventListener("online", online);
    target?.addEventListener("offline", offline);
    return () => {
      target?.removeEventListener("online", online);
      target?.removeEventListener("offline", offline);
    };
  }, []);

  useEffect(() => () => {
    operationGeneration.current++;
    signInAbort.current?.abort();
    signOutAbort.current?.abort();
    commandAbort.current?.abort();
    authorityAbort.current?.abort();
    membersAbort.current?.abort();
    spacesAbort.current?.abort();
    redemptionAbort.current?.abort();
    delegationAbort.current?.abort();
  }, []);

  /** ♻️ Re-authentication without a round trip per render: once the locally known deadline has
   * passed, one `me` read confirms it with the hub (which may have extended or revoked it) instead
   * of the shell deciding from its own clock. */
  useEffect(() => {
    if (session.phase !== "signed-in" || session.expiresAtMs === null) return undefined;
    const remaining = session.expiresAtMs - Date.now();
    if (remaining <= 0) {
      readAuthority();
      return undefined;
    }
    const timer = setTimeout(readAuthority, Math.min(remaining, HUB_SESSION_AUTHORITY_RECHECK_MAX_MS));
    return () => clearTimeout(timer);
  }, [readAuthority, session.expiresAtMs, session.phase]);

  const selectConnection = useCallback((id: string) => {
    const next = selectHubConnectionV1(book, id);
    if (next.selectedId === operationConnectionId.current) return;
    retireOperationsForConnection(next.selectedId);
    setAuthority(null);
    setMemberRows([]);
    setInvite(null);
    setRedemption({ phase: "idle", error: null });
    setRows([]);
    setSpacesPhase("loading");
    setAgentCredential(null);
    setDelegationSummaries([]);
    setDelegationPhase("idle");
    setDelegationError(null);
    dispatchSession({ kind: "select-connection", connectionId: next.selectedId });
    persist(next);
  }, [book, persist, retireOperationsForConnection]);

  const addRemoteHub = useCallback((typed: string, label: string): HubSignInErrorCodeV1 | "invalid-origin" | null => {
    let origin: string;
    try {
      origin = parseHubOriginV1(typed);
    } catch {
      return "invalid-origin";
    }
    const trimmed = label.trim();
    const next = upsertHubConnectionV1(book, { id: hubConnectionIdForOriginV1(origin), label: trimmed.length > 0 ? trimmed.slice(0, 128) : origin, origin, kind: "remote", lastUserId: null });
    const changed = next.selectedId !== operationConnectionId.current;
    retireOperationsForConnection(next.selectedId);
    if (changed) {
      setAuthority(null);
      setMemberRows([]);
      setInvite(null);
      setRedemption({ phase: "idle", error: null });
      setRows([]);
      setSpacesPhase("loading");
      setAgentCredential(null);
      setDelegationSummaries([]);
      setDelegationPhase("idle");
      setDelegationError(null);
    }
    dispatchSession({ kind: "select-connection", connectionId: next.selectedId });
    persist(next);
    return null;
  }, [book, persist, retireOperationsForConnection]);

  const forgetHub = useCallback((id: string) => {
    const next = removeHubConnectionV1(book, id);
    const changed = next.selectedId !== operationConnectionId.current;
    retireOperationsForConnection(next.selectedId);
    if (changed) {
      setAuthority(null);
      setMemberRows([]);
      setInvite(null);
      setRedemption({ phase: "idle", error: null });
      setRows([]);
      setSpacesPhase("loading");
      setAgentCredential(null);
      setDelegationSummaries([]);
      setDelegationPhase("idle");
      setDelegationError(null);
    }
    dispatchSession({ kind: "select-connection", connectionId: next.selectedId });
    persist(next);
  }, [book, persist, retireOperationsForConnection]);

  const signIn = useCallback((input: Readonly<{ email: string; password: string }>) => {
    const owner = captureOperationOwner();
    signInAbort.current?.abort();
    const abort = new AbortController();
    signInAbort.current = abort;
    dispatchSession({ kind: "submit" });
    const active = portRef.current;
    const credential: HubSignInCredentialV1 = { email: input.email, password: input.password, deviceInstanceId: active.deviceInstanceId, clientClass: active.clientClass };
    void runHubSignInV1(active.signIn, owner.origin, credential, abort.signal).then((outcome) => {
      if (!operationOwnerCurrent(owner)) return;
      if (outcome.kind === "failed") {
        dispatchSession({ kind: "failed", code: outcome.code, retryAfterSeconds: outcome.retryAfterSeconds });
        if (outcome.code === "unreachable") dispatchSession({ kind: "connectivity", offline: true });
        return;
      }
      dispatchSession({ kind: "minted", userId: outcome.result.userId, expiresAtMs: null });
      dispatchSession({ kind: "connectivity", offline: false });
      const target = book.connections.find((entry) => entry.id === owner.connectionId);
      if (target) persist(upsertHubConnectionV1(book, { ...target, lastUserId: outcome.result.userId }));
      readAuthority(owner);
      loadSpaces(owner);
      loadMembers(watchedSpaceIdRef.current, owner);
    });
  }, [book, captureOperationOwner, loadMembers, loadSpaces, operationOwnerCurrent, persist, portRef, readAuthority, watchedSpaceIdRef]);

  const cancelSignIn = useCallback(() => {
    signInAbort.current?.abort();
    signInAbort.current = null;
    dispatchSession({ kind: "failed", code: "cancelled", retryAfterSeconds: null });
  }, []);

  const signOut = useCallback(() => {
    const owner = captureOperationOwner();
    signOutAbort.current?.abort();
    authorityAbort.current?.abort();
    membersAbort.current?.abort();
    setAuthority(null);
    setMemberRows([]);
    dispatchSession({ kind: "sign-out" });
    const abort = new AbortController();
    signOutAbort.current = abort;
    void runHubSignOutV1(portRef.current.signIn, owner.origin, abort.signal).then(() => {
      if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
      dispatchSession({ kind: "signed-out" });
      setRows([]);
      setInvite(null);
      setRedemption({ phase: "idle", error: null });
      setSpacesPhase("loading");
      setAgentCredential(null);
      setDelegationSummaries([]);
      setDelegationPhase("idle");
      setDelegationError(null);
    });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  const createSpace = useCallback((name: string, kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility) => {
    let command: DirectoryCommand;
    try {
      command = createSpaceCommandV1(name, kind, visibility);
    } catch {
      setSpacesPhase("failed");
      return;
    }
    runCommand(command, () => setSearch(""));
  }, [runCommand]);

  const archiveSpace = useCallback((spaceId: string) => {
    try {
      runCommand(archiveSpaceCommandV1(spaceId), () => undefined);
    } catch {
      setSpacesPhase("failed");
    }
  }, [runCommand]);

  const createInvite = useCallback((spaceId: string, role: DirectorySpaceRole, ttlSecs: number) => {
    let command: DirectoryCommand;
    try {
      command = createInviteCommandV1(spaceId, role, ttlSecs);
    } catch {
      setSpacesPhase("failed");
      return;
    }
    const origin = connectionRef.current.origin;
    runCommand(command, (receipt) => {
      if (receipt.result.kind !== "invite") {
        setSpacesPhase("failed");
        return;
      }
      setInvite({ spaceId, link: `${origin}/#semio-invite=${receipt.result.inviteToken}`, copy: "idle" });
    });
  }, [connectionRef, runCommand]);

  const copyInvite = useCallback(() => {
    const owner = captureOperationOwner();
    const current = invite;
    const write = portRef.current.writeClipboard;
    if (current === null || write === undefined) {
      setInvite((value) => (value === null ? value : { ...value, copy: "failed" }));
      return;
    }
    void write(current.link)
      .then(() => { if (operationOwnerCurrent(owner)) setInvite((value) => (value === null ? value : { ...value, copy: "copied" })); })
      .catch(() => { if (operationOwnerCurrent(owner)) setInvite((value) => (value === null ? value : { ...value, copy: "failed" })); });
  }, [captureOperationOwner, invite, operationOwnerCurrent, portRef]);

  const dismissInvite = useCallback(() => setInvite(null), []);

  const redeemInvite = useCallback((typed: string) => {
    const owner = captureOperationOwner();
    let token: string;
    try {
      token = parseInviteTokenV1(typed);
    } catch {
      setRedemption({ phase: "failed", error: "invalid-invite" });
      return;
    }
    redemptionAbort.current?.abort();
    setRedemption({ phase: "redeeming", error: null });
    const abort = new AbortController();
    redemptionAbort.current = abort;
    void portRef.current
      .redeemInvite(owner.origin, token, abort.signal)
      .then((response) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        if (response.status >= 200 && response.status < 300) {
          setRedemption({ phase: "redeemed", error: null });
          loadSpaces(owner);
          return;
        }
        setRedemption({ phase: "failed", error: inviteRedemptionErrorFromStatusV1(response.status) });
      })
      .catch(() => { if (operationOwnerCurrent(owner)) setRedemption({ phase: "failed", error: abort.signal.aborted ? "cancelled" : "unreachable" }); });
  }, [captureOperationOwner, loadSpaces, operationOwnerCurrent, portRef]);

  const refreshSpaces = useCallback(() => loadSpaces(), [loadSpaces]);

  const refreshDelegations = useCallback(() => loadDelegations(watchedSpaceIdRef.current), [loadDelegations, watchedSpaceIdRef]);

  /** 🤖️ Mints one delegation. The receipt is turned into a credential file HERE, once, and the raw
   * token is never written anywhere else: the pane only ever sees the rendered file. */
  const createDelegation = useCallback((agentLabel: string, audience: AgentAudienceV1, ttlSecs: number) => {
    const owner = captureOperationOwner();
    const spaceId = watchedSpaceIdRef.current;
    if (spaceId === null) {
      setDelegationError("malformed-request");
      setDelegationPhase("failed");
      return;
    }
    let body: string;
    try {
      body = createAgentDelegationBodyV1(spaceId, agentLabel, audience, ttlSecs);
    } catch {
      setDelegationError("malformed-request");
      setDelegationPhase("failed");
      return;
    }
    delegationAbort.current?.abort();
    const abort = new AbortController();
    delegationAbort.current = abort;
    setDelegationPhase("submitting");
    setDelegationError(null);
    void portRef.current
      .createAgentDelegation(owner.origin, body, abort.signal)
      .then((receipt) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setAgentCredential({ receipt, file: agentCredentialFileV1(receipt, owner.origin), command: agentCredentialCommandV1(receipt, owner.origin), save: "idle", mcpClient: { phase: portRef.current.installAgentCredential === undefined ? "unavailable" : "idle", config: null } });
        loadDelegations(spaceId, owner);
      })
      .catch((error: unknown) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setDelegationPhase("failed");
        setDelegationError(error instanceof AgentDelegationRefusalV1 ? error.code : "unreachable");
      });
  }, [captureOperationOwner, loadDelegations, operationOwnerCurrent, portRef, watchedSpaceIdRef]);

  const downloadAgentCredential = useCallback(() => {
    const owner = captureOperationOwner();
    const current = agentCredential;
    const save = portRef.current.saveFile;
    if (current === null || save === undefined) {
      setAgentCredential((value) => (value === null ? value : { ...value, save: "failed" }));
      return;
    }
    void save(current.file)
      .then(() => { if (operationOwnerCurrent(owner)) setAgentCredential((value) => (value === null ? value : { ...value, save: "saved" })); })
      .catch(() => { if (operationOwnerCurrent(owner)) setAgentCredential((value) => (value === null ? value : { ...value, save: "failed" })); });
  }, [agentCredential, captureOperationOwner, operationOwnerCurrent, portRef]);

  const dismissAgentCredential = useCallback(() => setAgentCredential(null), []);

  const installAgentMcpClient = useCallback(() => {
    const owner = captureOperationOwner();
    const current = agentCredential;
    const install = portRef.current.installAgentCredential;
    if (current === null) return;
    const mcpClient = (next: HubAgentMcpClientV1) => setAgentCredential((value) => (value === null || value.receipt.delegationId !== current.receipt.delegationId ? value : { ...value, mcpClient: next }));
    if (install === undefined) {
      mcpClient({ phase: "unavailable", config: null });
      return;
    }
    mcpClient({ phase: "installing", config: null });
    void install(agentCredentialInstallRequestV1(current.receipt, owner.origin), new AbortController().signal)
      .then((installed) => { if (operationOwnerCurrent(owner)) mcpClient({ phase: "ready", config: agentMcpClientConfigJsonV1(agentMcpClientConfigV1(current.receipt, owner.origin, installed)) }); })
      .catch((error: unknown) => { if (operationOwnerCurrent(owner)) mcpClient({ phase: error instanceof AgentCredentialInstallUnavailableV1 ? "unavailable" : "failed", config: null }); });
  }, [agentCredential, captureOperationOwner, operationOwnerCurrent, portRef]);

  const copyAgentMcpClientConfig = useCallback(() => {
    const config = agentCredential?.mcpClient.config ?? null;
    const write = portRef.current.writeClipboard;
    if (config === null || write === undefined) return;
    const delegationId = agentCredential?.receipt.delegationId;
    void write(config).then(() => setAgentCredential((value) => (value === null || value.receipt.delegationId !== delegationId ? value : { ...value, mcpClient: { phase: "copied", config } })));
  }, [agentCredential, portRef]);

  const revokeDelegation = useCallback((delegationId: string) => {
    const owner = captureOperationOwner();
    const spaceId = watchedSpaceIdRef.current;
    try {
      agentDelegationRevokePathV1(delegationId);
    } catch {
      setDelegationError("malformed-request");
      setDelegationPhase("failed");
      return;
    }
    delegationAbort.current?.abort();
    const abort = new AbortController();
    delegationAbort.current = abort;
    setDelegationPhase("submitting");
    setDelegationError(null);
    void portRef.current
      .revokeAgentDelegation(owner.origin, delegationId, abort.signal)
      .then(() => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setAgentCredential((value) => (value === null || value.receipt.delegationId !== delegationId ? value : null));
        void portRef.current.uninstallAgentCredential?.(delegationId).catch(() => undefined);
        loadDelegations(spaceId, owner);
      })
      .catch((error: unknown) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setDelegationPhase("failed");
        setDelegationError(error instanceof AgentDelegationRefusalV1 ? error.code : "unreachable");
      });
  }, [captureOperationOwner, loadDelegations, operationOwnerCurrent, portRef, watchedSpaceIdRef]);

  const visibleRows = useMemo(() => filterSpaceRowsV1(rows, search), [rows, search]);
  const members = useMemo(() => spaceMemberPresenceV1(memberRows, onlineUserIds), [memberRows, onlineUserIds]);
  const delegations = useMemo(() => agentDelegationRowsV1(delegationSummaries, Date.now()), [delegationSummaries]);

  return {
    book,
    connection,
    session,
    authority,
    members,
    rows: visibleRows,
    spacesPhase,
    search,
    invite,
    redemption,
    delegations,
    delegationPhase,
    delegationError,
    agentCredential,
    selectConnection,
    addRemoteHub,
    forgetHub,
    signIn,
    cancelSignIn,
    signOut,
    refreshSpaces,
    watchSpaceMembers,
    setSearch,
    createSpace,
    archiveSpace,
    createInvite,
    copyInvite,
    dismissInvite,
    redeemInvite,
    refreshDelegations,
    createDelegation,
    downloadAgentCredential,
    dismissAgentCredential,
    installAgentMcpClient,
    copyAgentMcpClientConfig,
    revokeDelegation,
  };
}
//#endregion 🔖️Hook

//#region 🔖️FetchPort
/** 👥️ Reads the `members` window out of the hub's own space-administration page. `members` is a
 * `DirectorySpaceAdministrationWindowV1` — `{ rows, nextCursor? }`, not a bare array — and a
 * `"public"`-access page (the projection a non-member gets) carries no `members` key at all, which
 * is an empty roster rather than an error. Every row is bounds-checked here rather than trusted, so
 * a proxy's HTML or an over-long display name can never be rendered as a roster. */
export function parseHubSpaceMemberRowsV1(source: string): readonly HubSpaceMemberRowV1[] {
  let value: unknown;
  try {
    value = JSON.parse(source);
  } catch {
    return [];
  }
  if (value === null || typeof value !== "object") return [];
  const window = (value as { members?: unknown }).members;
  const members = window !== null && typeof window === "object" ? (window as { rows?: unknown }).rows : undefined;
  if (!Array.isArray(members)) return [];
  const rows: HubSpaceMemberRowV1[] = [];
  for (const entry of members.slice(0, HUB_SPACE_MEMBER_ROWS_MAX)) {
    if (entry === null || typeof entry !== "object") continue;
    const { userId, displayName, email, role, owner } = entry as Record<string, unknown>;
    if (typeof userId !== "string" || userId.length === 0 || userId.length > 256 || /\p{Cc}/u.test(userId)) continue;
    if (typeof email !== "string" || email.length > 320 || /\p{Cc}/u.test(email)) continue;
    if (typeof displayName !== "string" || displayName.length > 128 || /\p{Cc}/u.test(displayName)) continue;
    if (role !== "author" && role !== "spectator") continue;
    rows.push({ userId, displayName, email, role, owner: owner === true });
  }
  return rows;
}

/** 📏️ `SPACE_ADMINISTRATION_PAGE_MAX` on the hub side; a longer array is a wire fault, not a page. */
export const HUB_SPACE_MEMBER_ROWS_MAX = 256;

/** 📨️ Minimal structural response the fetch adapter reads — declared here so this element never
 * requires the ambient `Response` type. */
export interface HubFetchResponseV1 {
  readonly status: number;
  readonly headers: { get(name: string): string | null };
  text(): Promise<string>;
}

/** 🌐️ Builds the real port over an injected request function. The minted capability is captured in
 * this closure and sent as `Authorization: Bearer …` (AU1 §1.1); it is never returned, stored or
 * placed in a URL, so no caller — including React — can read it back out. */
export function createHubConnectionFetchPortV1(options: {
  readonly request: (url: string, init: Readonly<{ method: string; headers: Record<string, string>; body?: string }>, signal: AbortSignal) => Promise<HubFetchResponseV1>;
  readonly storage: HubConnectionStorageV1 | null;
  readonly bootstrapOrigin: string;
  readonly deviceInstanceId: string;
  readonly clientClass: HubSignInClientClassV1;
  readonly parseSpaces: (body: string) => readonly DirectorySpaceListEntryV1[];
  readonly sealCommand: (command: DirectoryCommand) => Readonly<{ body: string; parseReceipt: (body: string) => Promise<DirectoryCommandReceiptV1> }>;
  readonly writeClipboard?: (text: string) => Promise<void>;
  /** 📄️ Hands the human a file to save — the one-time agent credential. Injected rather than reached
   * for, so a headless test drives the same lane a browser download does. */
  readonly saveFile?: (file: Readonly<{ fileName: string; contents: string; mediaType: string }>) => Promise<void>;
  /** 🔌️ Installs a delegation's credential for MCP clients; see {@link HubConnectionPortV1.installAgentCredential}. */
  readonly installAgentCredential?: (request: AgentCredentialInstallRequestV1, signal: AbortSignal) => Promise<AgentCredentialInstallReceiptV1>;
  readonly uninstallAgentCredential?: (delegationId: string) => Promise<void>;
  /** 🎫️ The capability a previous page load minted and this context remembered, restored so a
   * reload continues the same hub session instead of asking for the password again. */
  readonly restoredCapability?: Readonly<{ token: string; userId: string }> | null;
  /** 📣️ Announces every change of the capability this port holds — a mint, a sign-out, or the hub
   * refusing it. The shell is what remembers it and what hands it to the credential-owning worker;
   * this port stays the only thing that ever puts it on a wire. */
  readonly onCapability?: (capability: Readonly<{ token: string; userId: string }> | null) => void;
}): HubConnectionPortV1 {
  let capability: string | null = options.restoredCapability?.token ?? null;
  const announce = (next: Readonly<{ token: string; userId: string }> | null): void => {
    capability = next?.token ?? null;
    options.onCapability?.(next);
  };
  const authorized = (json: boolean): Record<string, string> => ({
    ...(json ? { "content-type": "application/json" } : {}),
    ...(capability === null ? {} : { authorization: `Bearer ${capability}` }),
  });
  const answer = async (response: HubFetchResponseV1): Promise<{ status: number; body: string; retryAfterHeader: string | null }> => ({
    status: response.status,
    body: await response.text(),
    retryAfterHeader: response.headers.get("retry-after"),
  });
  return {
    bootstrapOrigin: options.bootstrapOrigin,
    storage: options.storage,
    deviceInstanceId: options.deviceInstanceId,
    clientClass: options.clientClass,
    restoredCapability: options.restoredCapability ? { userId: options.restoredCapability.userId } : null,
    ...(options.writeClipboard === undefined ? {} : { writeClipboard: options.writeClipboard }),
    signIn: {
      mint: async (origin, body, signal) => {
        const result = await answer(await options.request(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body }, signal));
        if (result.status === 200) {
          try {
            const minted = parseHubSessionMintResultV1(result.body);
            announce({ token: minted.token, userId: minted.userId });
          } catch {
            announce(null);
          }
        }
        return result;
      },
      read: async (origin, signal) => {
        const result = await answer(await options.request(`${origin}/auth/sessions/me`, { method: "GET", headers: authorized(false) }, signal));
        if (result.status === 401) announce(null);
        return result;
      },
      end: async (origin, signal) => {
        const result = await answer(await options.request(`${origin}/auth/sessions/me`, { method: "DELETE", headers: authorized(false) }, signal));
        announce(null);
        return result;
      },
    },
    readSpaceMembers: async (origin, spaceId, signal) => {
      const response = await options.request(`${origin}/directory/spaces/${encodeURIComponent(spaceId)}`, { method: "GET", headers: authorized(false) }, signal);
      if (response.status !== 200) return [];
      return parseHubSpaceMemberRowsV1(await response.text());
    },
    listSpaces: async (origin, signal) => {
      const response = await options.request(`${origin}/directory/spaces`, { method: "GET", headers: authorized(false) }, signal);
      if (response.status !== 200) throw new Error("hub.spaces.unavailable");
      return options.parseSpaces(await response.text());
    },
    submitCommand: async (origin, command, signal) => {
      const sealed = options.sealCommand(command);
      const response = await options.request(`${origin}/directory/commands`, { method: "POST", headers: authorized(true), body: sealed.body }, signal);
      if (!directoryCommandAnsweredV1(response.status)) throw new Error("hub.command.refused");
      return sealed.parseReceipt(await response.text());
    },
    redeemInvite: async (origin, token, signal) => {
      const response = await options.request(`${origin}/directory/invites/${encodeURIComponent(token)}/redeem`, { method: "POST", headers: authorized(false) }, signal);
      return { status: response.status };
    },
    listAgentDelegations: async (origin, spaceId, signal) => {
      const response = await options.request(`${origin}${agentDelegationListPathV1(spaceId)}`, { method: "GET", headers: authorized(false) }, signal);
      const text = await response.text();
      if (response.status !== 200) throw new AgentDelegationRefusalV1(agentDelegationErrorFromResponseV1(response.status, text));
      return parseAgentDelegationListV1(text);
    },
    createAgentDelegation: async (origin, body, signal) => {
      const response = await options.request(`${origin}${AGENT_DELEGATION_PATH_V1}`, { method: "POST", headers: authorized(true), body }, signal);
      const text = await response.text();
      if (response.status !== 201) throw new AgentDelegationRefusalV1(agentDelegationErrorFromResponseV1(response.status, text));
      return parseAgentDelegationReceiptV1(text);
    },
    revokeAgentDelegation: async (origin, delegationId, signal) => {
      const response = await options.request(`${origin}${agentDelegationRevokePathV1(delegationId)}`, { method: "DELETE", headers: authorized(false) }, signal);
      if (response.status !== 204) throw new AgentDelegationRefusalV1(agentDelegationErrorFromResponseV1(response.status, await response.text()));
    },
    ...(options.saveFile === undefined ? {} : { saveFile: options.saveFile }),
    ...(options.installAgentCredential === undefined ? {} : { installAgentCredential: options.installAgentCredential }),
    ...(options.uninstallAgentCredential === undefined ? {} : { uninstallAgentCredential: options.uninstallAgentCredential }),
  };
}

/** 🏠️ The bootstrap connection this device always offers, exported so a shell and a story build the
 * same initial book. */
export { localBootstrapHubConnectionV1 };
//#endregion 🔖️FetchPort
