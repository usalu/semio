/** ⚙️ Collaboration e2e constants, relay and recorder: the repository root, the port range and boot budgets, the admin token and user credentials, the step names, the link cut schedule, the severable link relay, the frame helpers, the locales and the step recorder with its verdict. */
import { connect, createServer, type AddressInfo, type Socket } from "node:net";
import { findWorkspaceRoot } from "@semio-tech/process/routing";

//#region 🔖️CatalogSmokeVerify
export const repoRoot = findWorkspaceRoot();

//#endregion 🔖️CatalogSmokeVerify
//#region 🔖️CollabE2e
/** 🤝️ Two-user hub+shell end-to-end collaboration proof — ticket
 * `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS`, lane 3-C. Boots the real hub plus two
 * independent `s` react dev servers (one per user) and drives them as two separate Playwright browser
 * contexts through the ticket's whole collaboration story: space creation replication, sharing, artifact
 * creation replication, live co-editing, presence, check-in, admin visibility, and hub-restart
 * persistence. Every scenario step is reported individually (`STEP n: PASS/FAIL`) and the run continues
 * past a failing step where it safely can so later independent steps still produce evidence. */
export const COLLAB_E2E_PORT_MIN = 7400;

export const COLLAB_E2E_PORT_MAX = 7498;

export const COLLAB_E2E_HUB_BOOT_BUDGET_MS = Number(process.env.COLLAB_E2E_HUB_BOOT_BUDGET_MS ?? 300_000);

export const COLLAB_E2E_PREBUILD_BUDGET_MS = Number(process.env.COLLAB_E2E_PREBUILD_BUDGET_MS ?? 1_800_000);

export const COLLAB_E2E_DEV_BOOT_BUDGET_MS = Number(process.env.COLLAB_E2E_DEV_BOOT_BUDGET_MS ?? 300_000);

const COLLAB_E2E_ADMIN_TOKEN = "e2e-admin";

export const COLLAB_E2E_USER1_EMAIL = "user1@semio.dev";

export const COLLAB_E2E_USER2_EMAIL = "user2@semio.dev";

/** 🔑️ The two humans this scenario runs as. Provisioned zero-touch through the hub binary's own
 * operator verb `os-hub credential set` (ticket slice AU3 §3.3 — the only way a principal ever gets
 * its first credential, and deliberately not reachable over the network), then signed in from each
 * browser context through the shell's own hub workspace. There is no back door: the two shells
 * authenticate exactly the way a human does. Passwords are 8..=256 bytes per the hub's own bound. */
export const COLLAB_E2E_USER1_PASSWORD = "collab e2e first human phrase";

export const COLLAB_E2E_USER2_PASSWORD = "collab e2e second human phrase";

export const COLLAB_E2E_USER1_DISPLAY = "Collab User One";

export const COLLAB_E2E_USER2_DISPLAY = "Collab User Two";

export const COLLAB_E2E_STEP_NAMES = [
  "user1 creates a public studio space from Home; the row appears in user1's Home",
  "user1 shares the space with user2 as author; user2's Home lists it and user2 opens /spaces/{id}",
  "user1 creates a writer artifact; the row appears in both tables and opens an editor for user1",
  "user2 opens the same artifact; user1 types and user2 sees the text",
  "#s-presence-peers shows 2 peers, in distinct hub-assigned session colours, in both shells",
  "user1 checks in with a message; the check-in status reads checked in, the active checkpoint moves and the space table's updated column moves for both",
  "admin: /admin/api/connections lists both connections with their surfaces; /admin returns HTML",
  "user1's keystroke reaches user2's editor within ONE ServerFrame::Commands round trip",
  "hub restarts against the same OS_HUB_DATA; user2 reloads and the space + artifact are still there",
  "an edit typed while the hub is down survives the restart and reaches user2 via resume-token/frontier",
  "undo is per user: user1's undo reverts user1's own edit and leaves user2's edit standing in both shells",
  "a short connection loss does not freeze user2's shell; the edit typed offline lands once the link returns",
  "two simultaneous writers converge: both shells settle on the SAME text, ordered by the hub's own sequence",
  "writer/draw/puzzle3d surfaces show peer-cursor overlay markers that move when the peer pointer moves",
  "real link cuts (5 s / 15 s / 60 s) while user1 types: no shell freezes, nothing is rebuilt, every keystroke typed during a cut reaches the hub and user2",
] as const;

/** ⏱️ STEP 15's link cuts in milliseconds (`S_COLLAB_LINK_CUTS_MS`, comma-separated): a 5 s blip, a 15 s shortage and a 60 s one at
 * the link-shortage policy's bound. */
export const COLLAB_E2E_LINK_CUTS_MS = (process.env.S_COLLAB_LINK_CUTS_MS ?? "5000,15000,60000").split(",").map(Number);

/** 🔌️ A TCP relay between one shell and the hub that STEP 15 cuts for real: every relayed connection is destroyed and new ones are
 * refused until the cut ends (`setOffline` leaves an open WebSocket silently black-holed instead). Transparent otherwise, so every
 * other step runs through it unchanged. */
export type CollabLinkRelay = { readonly url: string; cut(ms: number): void; close(): Promise<void> };

export async function collabStartLinkRelay(hubBaseUrl: string): Promise<CollabLinkRelay> {
  const upstream = new URL(hubBaseUrl);
  const pairs = new Set<readonly [Socket, Socket]>();
  let cutUntil = 0;
  const server = createServer((client) => {
    if (Date.now() < cutUntil) {
      client.destroy();
      return;
    }
    const hub = connect(Number(upstream.port), upstream.hostname);
    const pair = [client, hub] as const;
    pairs.add(pair);
    const drop = (): void => {
      pairs.delete(pair);
      client.destroy();
      hub.destroy();
    };
    for (const socket of pair) {
      socket.on("error", drop);
      socket.on("close", drop);
    }
    client.pipe(hub);
    hub.pipe(client);
  });
  await new Promise<void>((resolve) => server.listen(0, "127.0.0.1", resolve));
  const cutAll = (): void => {
    for (const [client, hub] of pairs) {
      client.destroy();
      hub.destroy();
    }
    pairs.clear();
  };
  return {
    url: `http://127.0.0.1:${(server.address() as AddressInfo).port}`,
    cut: (ms) => {
      cutUntil = Date.now() + ms;
      cutAll();
    },
    close: () =>
      new Promise<void>((resolve) => {
        cutAll();
        server.close(() => resolve());
      }),
  };
}

/** 🎞️ One animation frame's latency on `page`, in ms: a frozen shell answers late or not at all. */
export async function collabFrameMs(page: import("playwright").Page): Promise<number> {
  return Math.round(await page.evaluate(() => new Promise<number>((resolve) => { const started = performance.now(); requestAnimationFrame(() => resolve(performance.now() - started)); })));
}

/** 🔁️ Whether `page` shows the rebuild notice ("fresh authoritative restore", en or de): the document was discarded and rebuilt. */
export async function collabShowsRebuild(page: import("playwright").Page): Promise<boolean> {
  return (await page.getByText(/fresh authoritative restore|autoritative Wiederherstellung/u).count()) > 0;
}

/** 🌍️ The language both humans' browser contexts run in (`--locale`), and the dialog option labels STEP 1/2 pick in it — the
 * Space app's own `LocalizedLabel`s (`🪐️space/…/🏠️home/…/✏️editor/🦀️.rs`); `checkedIn` is the shell's ready Check In status
 * (`CHECKIN_STATUS_LABELS.ready`, `🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts`). */
export type CollabLocale = "en" | "de";

export const COLLAB_E2E_LOCALES: Readonly<Record<CollabLocale, { readonly browser: string; readonly studio: string; readonly public: string; readonly author: string; readonly checkedIn: string; readonly open: string; readonly share: string }>> = {
  en: { browser: "en-US", studio: "Studio", public: "Public", author: "Author", checkedIn: "Checked in", open: "Open", share: "Share" },
  de: { browser: "de-DE", studio: "Studio", public: "Öffentlich", author: "Autor", checkedIn: "Eingecheckt", open: "Öffnen", share: "Teilen" },
};

/** 🧾️ One step's verdict: `true` PASS, `false` FAIL, `null` SKIP (the run does not own what the step needs). */
export type CollabStepOutcome = { readonly step: number; readonly name: string; readonly pass: boolean | null; readonly detail: string };

export type CollabRecord = (step: number, pass: boolean | null, detail: string) => void;

/** 🧾️ Appends every verdict to `results` and prints it as its `STEP n: PASS|FAIL|SKIP` line. */
export function collabRecorder(results: CollabStepOutcome[]): CollabRecord {
  return (step, pass, detail) => {
    results.push({ step, name: COLLAB_E2E_STEP_NAMES[step - 1]!, pass, detail });
    console.log(`STEP ${step}: ${collabVerdict(pass)}: ${COLLAB_E2E_STEP_NAMES[step - 1]} — ${detail}`);
  };
}

export function collabVerdict(pass: boolean | null): "PASS" | "FAIL" | "SKIP" {
  return pass === null ? "SKIP" : pass ? "PASS" : "FAIL";
}
//#endregion 🔖️CollabE2e
