import { spawn } from "node:child_process";
import { mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { isDevPortInUse } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { protectOwnerOnly } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔐️owner-only/🟦️.ts";
import { DEV_LOCAL_HUB_DEFAULT_URL, claimDevHubLeaseV1, devHubLeasePathsV1, devHubLeaseRootV1, devHubLocaleV1, devHubStatusTextV1, devLocalHubDataDir, ensureCurrentTrustedCatalogV1, parseHubPort, releaseDevHubLeaseV1, type DevHubStatusV1, type DevHubCatalogPublisherV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";
import { LOCAL_ADMIN_CAPABILITY_FILE, startLocalSessionBroker } from "../🔐️credential-issuance/🟦️.ts";
import { finishLocalHub, hubDevBinaryPath, LOCAL_HUB_ADMINISTRATOR_PROFILE, LOCAL_HUB_ADMINISTRATOR_SUBJECT, LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES, LOCAL_HUB_DEVELOPMENT_PROFILES, startLocalHub, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS, waitForReadiness } from "../🏃️execution/🟦️.ts";

function terminateProcessTree(pid: number | undefined): void {
  if (pid === undefined) return;
  try {
    if (process.platform === "win32") spawn("taskkill", ["/pid", String(pid), "/T", "/F"], { stdio: "ignore", shell: false });
    else process.kill(-pid, "SIGTERM");
  } catch {}
}

/** 📤️ The hub's own product verb (`os-hub:trusted-catalog-bootstrap`), as a child the signal terminates; the publisher
 * writes a new immutable generation and moves `current.json` only after a candidate hub loaded it, so a cancelled run
 * leaves the previous generation in place.
 * @see ../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts TrustedCatalogBootstrapScript */
export function devHubCatalogBootstrapPublisherV1(repoRoot: string): DevHubCatalogPublisherV1 {
  return (dataDir, packages, signal, onLine) =>
    new Promise<void>((resolvePublished, rejectPublished) => {
      const child = spawn("bun", ["nx", "run", "os-hub:trusted-catalog-bootstrap", "--packages", packages, "--outputStyle=stream"], { cwd: repoRoot, env: { ...process.env, OS_HUB_DATA: dataDir }, stdio: ["ignore", "pipe", "pipe"], shell: false, detached: process.platform !== "win32" });
      const abort = (): void => terminateProcessTree(child.pid);
      signal.addEventListener("abort", abort, { once: true });
      let pending = "";
      const lines = (chunk: Buffer): void => {
        pending += chunk.toString("utf8");
        const parts = pending.split("\n");
        pending = parts.pop() ?? "";
        for (const line of parts) if (line.trim().length > 0) onLine(line);
      };
      child.stdout.on("data", lines);
      child.stderr.on("data", lines);
      child.once("error", rejectPublished);
      child.once("exit", (code, exitSignal) => {
        signal.removeEventListener("abort", abort);
        if (signal.aborted) rejectPublished(new DOMException("catalog publication cancelled", "AbortError"));
        else if (code === 0) resolvePublished();
        else rejectPublished(new Error(`os-hub:trusted-catalog-bootstrap exited ${code ?? exitSignal}`));
      });
    });
}

/** 🗄️ `local-hub [hubUrl] [dataDir]`: the owner process — claims the hub port and the data root, makes the catalog one the
 * current hub loads (republishing a stale one, cancelled by SIGINT/SIGTERM), stages the hub binary, boots the hub with the
 * development profiles, runs the session broker, and holds until signalled. An owner that finds either claim held by a
 * live owner, or the port bound by anyone, exits at once. */
export class DevLocalHubScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const [hubArg, dataArg] = segments;
    const hubUrl = (hubArg ?? DEV_LOCAL_HUB_DEFAULT_URL).replace(/\/+$/u, "");
    const dataDir = dataArg ? resolve(dataArg) : devLocalHubDataDir(this.repoRoot);
    const locale = devHubLocaleV1();
    const report = (status: DevHubStatusV1): void => console.log(devHubStatusTextV1(status, locale));
    const port = parseHubPort(hubUrl);
    const paths = devHubLeasePathsV1(devHubLeaseRootV1(this.repoRoot), port, dataDir);
    mkdirSync(dataDir, { recursive: true });
    protectOwnerOnly(dataDir, "directory");
    const claim = claimDevHubLeaseV1(paths, { pid: process.pid, port, dataDir, hubUrl, acquiredAt: Date.now() });
    if (claim.kind === "held") {
      report({ kind: "owned", hubUrl, pid: claim.lease.pid });
      return;
    }
    const cancel = new AbortController();
    const stopPublishing = (): void => cancel.abort();
    process.once("SIGINT", stopPublishing);
    process.once("SIGTERM", stopPublishing);
    try {
      if (isDevPortInUse("127.0.0.1", port)) {
        report({ kind: "port-taken", hubUrl });
        return;
      }
      const catalog = await ensureCurrentTrustedCatalogV1(dataDir, devHubCatalogBootstrapPublisherV1(this.repoRoot), { packages: LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES, signal: cancel.signal, report });
      process.off("SIGINT", stopPublishing);
      process.off("SIGTERM", stopPublishing);
      if (catalog === "cancelled") return;
      const hubPkg = join(this.repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
      const binaryPath = hubDevBinaryPath(hubPkg);
      process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
      const adminToken = process.env.OS_HUB_ADMIN_TOKEN ?? "dev-local-hub-admin";
      const profiles = [...LOCAL_HUB_DEVELOPMENT_PROFILES, LOCAL_HUB_ADMINISTRATOR_PROFILE];
      const run = await startLocalHub(this.repoRoot, hubPkg, profiles, { port, dataDir, binaryPath, adminToken, adminSubjects: [LOCAL_HUB_ADMINISTRATOR_SUBJECT], capture: false });
      let broker: ReturnType<typeof startLocalSessionBroker> | null = null;
      const stop = (): void => {
        broker?.stop();
        void finishLocalHub(run);
      };
      process.once("SIGINT", stop);
      process.once("SIGTERM", stop);
      try {
        await waitForReadiness(run, false, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
        broker = startLocalSessionBroker(run, dataDir, profiles, 2, LOCAL_HUB_ADMINISTRATOR_PROFILE.profileId);
        console.log(`[dev-local-hub] ready at ${hubUrl}; session broker for ${broker.record.profiles.join(",")}; admin capability in ${join(dataDir, LOCAL_ADMIN_CAPABILITY_FILE)}`);
        await new Promise<void>((resolveExit) => (run.child.exitCode !== null ? resolveExit() : run.child.once("exit", () => resolveExit())));
      } finally {
        process.off("SIGINT", stop);
        process.off("SIGTERM", stop);
        broker?.stop();
        await finishLocalHub(run);
      }
    } finally {
      process.off("SIGINT", stopPublishing);
      process.off("SIGTERM", stopPublishing);
      releaseDevHubLeaseV1(paths, process.pid);
    }
  }
}
