/** 🛰️ The agent bridge rendezvous of the dev server: the rendezvous directory, the live offer lookup and the Vite plugin that answers offers. */
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { AGENT_BRIDGE_OFFER_ENDPOINT, AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION, agentBridgeOfferAnswerV1, parseAgentBridgeOfferRecordV2, parseAgentBridgeOfferScopeV1, selectAgentBridgeOfferV1, type AgentBridgeOfferRecordV2, type AgentBridgeOfferScopeV1 } from "../../../📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts";

//#region 🛰️AgentBridgeRendezvous
/** 🏷️ The carrier that points this dev session and one `semio-os-mcp` gateway at a rendezvous
 * of their own instead of the per-user default — the exact twin of the gateway's own
 * `🛰️rendezvous::RENDEZVOUS_DIR_ENV`. It is a directory path, never a credential (the admission proof
 * stays in the owner-only offer file the supervisor reads), and it is spelled with the `S_` prefix
 * the gateway's process-entry seal admits. Without it, `liveAgentBridgeOfferFor` hands a shell
 * whichever local gateway published last, so two local agents running at once cross-wire (a hub gateway's offer
 * only ever reaches its own human's shell). */
export const AGENT_BRIDGE_RENDEZVOUS_DIR_ENV = "S_AGENT_BRIDGE_DIR";

function agentBridgeRendezvousDir(): string {
  const pinned = process.env[AGENT_BRIDGE_RENDEZVOUS_DIR_ENV];
  if (pinned) return pinned;
  const home = process.env.HOME ?? process.env.USERPROFILE ?? ".";
  return join(home, ".semio", "agent", "bridge");
}

/** 📨️ The live gateway offer a shell of `scope` may dial (`selectAgentBridgeOfferV1`: a hub gateway's offer only
 * for its human's shell open on its hub and space, a local gateway's offer for any shell). `null` means no MCP gateway is
 * offering this shell a bridge, which is an ordinary state (nobody launched one), never an error. */
export function liveAgentBridgeOfferFor(scope: AgentBridgeOfferScopeV1, root: string = agentBridgeRendezvousDir()): AgentBridgeOfferRecordV2 | null {
  const directory = join(root, "offers");
  if (!existsSync(directory)) return null;
  const live = readdirSync(directory)
    .filter((name) => name.endsWith(".json"))
    .flatMap((name) => {
      try {
        const offer = parseAgentBridgeOfferRecordV2(JSON.parse(readFileSync(join(directory, name), "utf8")));
        if (offer === null) return [];
        process.kill(offer.pid, 0);
        return [offer];
      } catch {
        return [];
      }
    });
  return selectAgentBridgeOfferV1(live, scope);
}

type RendezvousServerRequest = { url?: string; method?: string };
export type RendezvousServerResponse = { statusCode: number; setHeader: (name: string, value: string) => void; end: (body?: string) => void };

/** 🛰️ Publishes THIS dev session as a live os session the stdio MCP gateway can discover, and
 * serves the browser shell the one offer its request's scope may dial ({@link liveAgentBridgeOfferFor}) on
 * {@link AGENT_BRIDGE_OFFER_ENDPOINT} — always `200`, the offer or the typed "not offered". The admission proof never travels through an environment
 * variable or a build-time define: the dev server reads the owner-only offer file and hands it over
 * loopback, on request, exactly like the local supervisor it is.
 *
 * Both halves are removed when the dev server closes, so a gateway that starts later never believes a
 * dead session. */
export function semioAgentBridgeRendezvousVitePlugin(options: { readonly shellKind: "react" | "wgpu-web"; readonly rendezvousRoot?: string; readonly sessionId?: string }) {
  const root = options.rendezvousRoot ?? agentBridgeRendezvousDir();
  const sessionsDir = join(root, "sessions");
  const recordPath = join(sessionsDir, `${process.pid}.json`);
  const removeRecord = (): void => {
    try {
      if (existsSync(recordPath)) rmSync(recordPath, { force: true });
    } catch {
      // best effort — a stale record is swept by the gateway's own liveness probe
    }
  };
  return {
    name: "semio-agent-bridge-rendezvous",
    apply: "serve" as const,
    configureServer(server: { middlewares: { use: (handler: (req: RendezvousServerRequest, res: RendezvousServerResponse, next: () => void) => void) => void }; httpServer?: { once: (event: string, handler: () => void) => void } | null }) {
      mkdirSync(sessionsDir, { recursive: true });
      writeFileSync(
        recordPath,
        JSON.stringify({ schemaVersion: AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION, sessionId: options.sessionId ?? `dev-${process.pid}`, pid: process.pid, shellKind: options.shellKind, startedAtMs: Date.now() }, null, 2),
        { mode: 0o600 },
      );
      process.once("exit", removeRecord);
      server.httpServer?.once("close", removeRecord);
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith(AGENT_BRIDGE_OFFER_ENDPOINT)) return next();
        const offer = liveAgentBridgeOfferFor(parseAgentBridgeOfferScopeV1(req.url), root);
        res.statusCode = 200;
        res.setHeader("content-type", "application/json");
        res.setHeader("cache-control", "no-store");
        res.end(JSON.stringify(agentBridgeOfferAnswerV1(offer)));
      });
    },
    closeBundle: removeRecord,
  };
}
//#endregion 🛰️AgentBridgeRendezvous
