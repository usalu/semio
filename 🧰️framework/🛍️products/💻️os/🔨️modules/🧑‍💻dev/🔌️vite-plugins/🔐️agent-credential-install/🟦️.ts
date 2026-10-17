/** 🔐️ The agent credential install endpoint of the dev server: the credentials directory, the well-formedness check and the Vite plugin that installs a credential. */
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { protectOwnerOnly } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🔐️owner-only/🟦️.ts";
import { agentClientLauncher, readDeclaredTools } from "../../../../../🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🟦️.ts";
import { AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1, AGENT_CREDENTIAL_INSTALL_RECEIPT_SCHEMA_V1, AGENT_CREDENTIAL_INSTALL_SCHEMA_V1, AGENT_CREDENTIAL_SCHEMA_V1, agentCredentialInstallFileNameV1, isAgentDelegationTokenV1 } from "@semio-tech/framework/directory/delegations";
import type { RendezvousServerResponse } from "../🛰️agent-bridge-rendezvous/🟦️.ts";

//#region 🔌️AgentCredentialInstall
/** 🗝️ Where this development host installs agent credentials for MCP clients. A directory path,
 * never a credential, spelled with the `S_` prefix the gateway's process-entry seal admits; unset, the
 * per-user default `~/.semio/agent/credentials` beside the bridge rendezvous. */
export const AGENT_CREDENTIALS_DIR_ENV = "S_AGENT_CREDENTIALS_DIR";

function agentCredentialsDir(): string {
  const pinned = process.env[AGENT_CREDENTIALS_DIR_ENV];
  if (pinned) return pinned;
  const home = process.env.HOME ?? process.env.USERPROFILE ?? ".";
  return join(home, ".semio", "agent", "credentials");
}

type CredentialInstallRequest = { url?: string; method?: string; headers: Record<string, string | string[] | undefined>; on(event: "data", handler: (chunk: Buffer) => void): void; on(event: "end" | "error", handler: () => void): void };

/** 🔐️ The credential file bytes `semio-os-mcp --credential-file` decodes, checked key for key
 * before anything reaches the disk. */
function agentCredentialFileIsWellFormed(contents: string): boolean {
  try {
    const value = JSON.parse(contents) as Record<string, unknown>;
    return (
      Object.keys(value).sort().join(",") === "audience,hubOrigin,schema,spaceId,token" &&
      value.schema === AGENT_CREDENTIAL_SCHEMA_V1 &&
      typeof value.hubOrigin === "string" &&
      typeof value.spaceId === "string" &&
      (value.audience === "read" || value.audience === "edit") &&
      typeof value.token === "string" &&
      isAgentDelegationTokenV1(value.token)
    );
  } catch {
    return false;
  }
}

/** 🔌️ Turns an agent delegation into a working MCP client on a development host: the delegation
 * pane posts the one-time credential here, this server writes it owner-only (`0600` in a `0700`
 * directory) under the delegation's own file name, and answers with its absolute path and the launcher
 * that starts `semio-os-mcp` from this checkout (the `semio` server declared in the root `📋️project.json`, `bun <repo>/📜️script.ts dev mcp stdio os`, which
 * stages the binary itself). `DELETE <endpoint>/<delegationId>` removes it once the delegation is
 * withdrawn. Only same-origin requests are served, so no other site can plant or remove a credential.
 * Contract: `📇️directory/🧬️schema` `AgentCredentialInstallRequestV1` / `AgentCredentialInstallReceiptV1`. */
export function semioAgentCredentialInstallVitePlugin(options: { readonly repoRoot: string; readonly credentialsRoot?: string }) {
  const root = options.credentialsRoot ?? agentCredentialsDir();
  const launcherOf = (): { readonly command: string; readonly args: readonly string[] } => agentClientLauncher(readDeclaredTools(options.repoRoot), options.repoRoot, "semio", process.execPath);
  return {
    name: "semio-agent-credential-install",
    apply: "serve" as const,
    configureServer(server: { middlewares: { use: (handler: (req: CredentialInstallRequest, res: RendezvousServerResponse, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        const path = req.url?.split("?")[0] ?? "";
        if (path !== AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1 && !path.startsWith(`${AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1}/`)) return next();
        const answer = (status: number, body?: unknown): void => {
          res.statusCode = status;
          res.setHeader("cache-control", "no-store");
          if (body === undefined) return res.end();
          res.setHeader("content-type", "application/json");
          res.end(JSON.stringify(body));
        };
        const site = req.headers["sec-fetch-site"];
        const origin = req.headers.origin;
        const host = req.headers.host;
        if (site !== "same-origin" && !(typeof origin === "string" && typeof host === "string" && (origin === `http://${host}` || origin === `https://${host}`))) return answer(403, { error: "same-origin requests only" });
        if (req.method === "DELETE" && path.length > AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1.length + 1) {
          try {
            rmSync(join(root, agentCredentialInstallFileNameV1(decodeURIComponent(path.slice(AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1.length + 1)))), { force: true });
            return answer(204);
          } catch {
            return answer(400, { error: "invalid delegation id" });
          }
        }
        if (req.method !== "POST" || path !== AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1) return answer(405, { error: "POST a credential or DELETE /<delegationId>" });
        const chunks: Buffer[] = [];
        let bytes = 0;
        let overflow = false;
        req.on("data", (chunk) => {
          bytes += chunk.length;
          if (bytes > 32 * 1024) overflow = true;
          else chunks.push(chunk);
        });
        req.on("error", () => answer(400, { error: "unreadable request" }));
        req.on("end", () => {
          if (overflow) return answer(413, { error: "credential install request too large" });
          try {
            const request = JSON.parse(Buffer.concat(chunks).toString("utf8")) as Record<string, unknown>;
            if (Object.keys(request).sort().join(",") !== "contents,delegationId,schema" || request.schema !== AGENT_CREDENTIAL_INSTALL_SCHEMA_V1 || typeof request.delegationId !== "string" || typeof request.contents !== "string" || !agentCredentialFileIsWellFormed(request.contents)) {
              return answer(400, { error: "not an AgentCredentialInstallRequestV1" });
            }
            const launcher = launcherOf();
            const file = join(root, agentCredentialInstallFileNameV1(request.delegationId));
            mkdirSync(root, { recursive: true, mode: 0o700 });
            protectOwnerOnly(root, "directory");
            writeFileSync(file, request.contents, { mode: 0o600 });
            protectOwnerOnly(file, "file");
            return answer(201, { schema: AGENT_CREDENTIAL_INSTALL_RECEIPT_SCHEMA_V1, credentialPath: file, launcher });
          } catch {
            return answer(400, { error: "not an AgentCredentialInstallRequestV1" });
          }
        });
      });
    },
  };
}
//#endregion 🔌️AgentCredentialInstall
