/** 🧱️ The `docker-stack-check` verb: the stack exactly as a host gets it, proven on a throw-away project with Caddy's internal CA, from update to restore and erasure.
 * @see ../🟦️.ts — the constants, tools and drift check shared by the operator verbs */

import { X509Certificate, randomBytes } from "node:crypto";
import { mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { connect } from "node:tls";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import course from "../../🔣️.json" with { type: "json" };
import { COURSE_DEPLOY_CLEANUP_MS, COURSE_IMAGE_DATABASE, COURSE_IMAGE_REPOSITORY, COURSE_STACK_CERTIFICATES, COURSE_STACK_FILES, type DockerAnswer, docker, dockerOk, dockerServerVersion, flag, interruption, pause, say } from "../🟦️.ts";
import { type Door, checkBodyLimit, checkCrossOrigin, checkPresence, databaseEvents, extractCourseStack, identifyLearner, learnerKnown, probe } from "../🩺️docker-image-check/🟦️.ts";

//#region 🧱️Stack
/** 🔖️ The SHA-256 fingerprint of the certificate the TLS listener on `localhost:<port>` presents for the name
 * `localhost`, whoever signed it. */
function servedCertificate(port: number, signal: AbortSignal): Promise<string> {
  return new Promise((accept, reject) => {
    if (signal.aborted) return reject(new Error("cancelled"));
    const socket = connect({ host: "localhost", port, servername: "localhost", rejectUnauthorized: false }, () => {
      const fingerprint = socket.getPeerCertificate().fingerprint256;
      socket.destroy();
      accept(fingerprint);
    });
    socket.setTimeout(10_000, () => socket.destroy(new Error(`no TLS handshake on localhost:${port} within 10 s`)));
    socket.once("error", reject);
    signal.addEventListener("abort", () => socket.destroy(new Error("cancelled")), { once: true });
  });
}

/** 🏭️ A certificate for `localhost` as an operator would supply one — one PEM bundle, the chain first and the private key
 * after it — minted by the internal CA of a throw-away container of `caddyImage`: another CA than the stack's own, so
 * never the certificate the stack obtains for itself, and no tool outside Docker. */
async function mintLocalCertificate(caddyImage: string, signal: AbortSignal): Promise<string> {
  const issued = "/data/caddy/certificates/local/localhost/localhost";
  const minted = await docker(["run", "--rm", "--interactive", "--entrypoint", "/bin/sh", caddyImage, "-c", `cat > /tmp/Caddyfile && caddy start --config /tmp/Caddyfile --adapter caddyfile > /dev/null 2>&1 && for attempt in $(seq 1 150); do [ -s ${issued}.crt ] && [ -s ${issued}.key ] && [ -s ${issued}.json ] && exec cat ${issued}.crt ${issued}.key; sleep 0.2; done; exit 1`], { signal, input: "{\n\tskip_install_trust\n}\n\nlocalhost {\n\trespond ok\n}\n" });
  if (minted.code !== 0 || !/-----BEGIN CERTIFICATE-----[\s\S]+PRIVATE KEY-----/u.test(minted.stdout)) throw new Error(`a throw-away ${caddyImage} minted no certificate for localhost (exit ${minted.code}): ${minted.stderr.slice(-600)}`);
  return `${minted.stdout}\n`;
}

/** ✅️ `docker-stack-check [--tag <tag>] [--http-port <n>] [--https-port <n>] [--keep]` — the stack as a host gets and
 * runs it: the two stack files copied out of the image into an empty directory, an `.env` of overrides beside them (the
 * proctor host `localhost` — Caddy's internal CA, no public certificate —, the tag, the ports and a request body limit
 * for both services), `docker compose config`, `caddy fmt` and `caddy validate` in the Caddy image, `up --wait` on a
 * throw-away project, the hardening and limits Docker applied to both containers, the API over HTTPS through Caddy
 * (plain HTTP redirects, a forged `X-Forwarded-Proto` changes nothing, the response headers, the body limit of `.env`), the
 * cross-origin contract and shared presence (WebSocket) through Caddy, a backup taken while serving and read back, an
 * update (the proctor container replaced) during which no request fails and after which the data is still there, a
 * restore refused while serving and accepted once stopped, a learner erased on request, a certificate of the operator
 * ({@link mintLocalCertificate}) put into `certificates` and served instead of Caddy's own once the caddy service is
 * recreated (SHA-256 fingerprints of what the listener presents before and after, against the supplied file), then
 * `down --volumes` unless `--keep`. */
export async function checkCourseStack(segments: readonly string[], invocation: ScriptInvocation): Promise<void> {
  const httpPort = flag(segments, "--http-port") ?? "18080";
  const httpsPort = flag(segments, "--https-port") ?? "18443";
  const tag = flag(segments, "--tag") ?? "latest";
  const keep = segments.includes("--keep");
  const { signal, release } = interruption(invocation);
  const project = `architecture-course-check-${Date.now().toString(36)}`;
  const bodyLimit = 32_768;
  const overrides = { PROCTOR_HOST: "localhost", PROCTOR_TAG: tag, COURSE_HTTP_PORT: httpPort, COURSE_HTTPS_PORT: httpsPort, PROCTOR_LIMIT_BODY_BYTES: String(bodyLimit) };
  const env = Object.fromEntries([...Object.keys(overrides), "PROCTOR_ALLOWED_ORIGINS"].map((name) => [name, undefined]));
  const door: Door = { base: `https://localhost:${httpsPort}`, headers: {}, insecure: true };
  let directory = "";
  let started = false;
  const composed = (args: readonly string[], input?: Uint8Array): Promise<DockerAnswer> => docker(["compose", "--project-name", project, "--file", "compose.yaml", ...args], { signal, env, cwd: directory, input });
  const compose = async (args: readonly string[]): Promise<string> => {
    const answer = await composed(args);
    if (answer.code !== 0) throw new Error(`docker compose ${args.slice(0, 3).join(" ")} exited ${answer.code}: ${(answer.stderr || answer.stdout).split("\n").slice(-6).join(" | ").slice(0, 900)}`);
    return answer.stdout;
  };
  try {
    say(`docker ${await dockerServerVersion(signal)}; project ${project}`);
    directory = await extractCourseStack(`${COURSE_IMAGE_REPOSITORY}:${tag}`, signal);
    const footprint = readdirSync(directory).sort();
    if (!Bun.deepEquals(footprint, [...COURSE_STACK_FILES].sort())) throw new Error(`the image carries ${footprint.join(", ")} as its stack, not exactly ${COURSE_STACK_FILES.join(" and ")}`);
    say(`host footprint: ${footprint.join(" and ")} from ${COURSE_IMAGE_REPOSITORY}:${tag} in ${directory}`);
    mkdirSync(join(directory, COURSE_STACK_CERTIFICATES));
    writeFileSync(join(directory, ".env"), Object.entries(overrides).map(([name, value]) => `${name}=${value}\n`).join(""));
    say(`.env overrides: ${Object.entries(overrides).map(([name, value]) => `${name}=${value}`).join(" ")}`);
    const config = await compose(["config"]);
    if (!config.includes("proctor") || !config.includes("caddy")) throw new Error("docker compose config lacks the proctor or the caddy service");
    say("docker compose config resolves");
    const caddyImage = /image:\s*(caddy:\S+)/u.exec(config)?.[1];
    if (!caddyImage) throw new Error("docker compose config names no caddy image");
    const caddyfile = readFileSync(join(directory, "Caddyfile"), "utf8");
    const validated = await docker(["run", "--rm", "--interactive", "--env", "PROCTOR_HOST=localhost", "--entrypoint", "/bin/sh", caddyImage, "-c", `cat > /tmp/Caddyfile && mkdir /${COURSE_STACK_CERTIFICATES} && caddy fmt /tmp/Caddyfile > /dev/null && caddy validate --config /tmp/Caddyfile --adapter caddyfile`], { signal, input: caddyfile });
    if (validated.code !== 0 || /"level":"(?:warn|error)"/u.test(validated.stderr)) throw new Error(`caddy fmt and caddy validate do not accept the Caddyfile without a warning: ${(validated.stderr || validated.stdout).slice(-900)}`);
    say(`caddy fmt and caddy validate (${caddyImage}): formatted, ${validated.stderr.split("\n").filter((line) => /Valid configuration/u.test(line)).join(" ") || "valid"}, no warning`);
    started = true;
    const up = Date.now();
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180"]);
    say(`up --wait: proctor and caddy healthy after ${Date.now() - up} ms`);
    for (const service of ["proctor", "caddy"]) {
      const host = JSON.parse(await dockerOk(["inspect", "--format", "{{json .HostConfig}}", `${project}-${service}-1`], { signal })) as { ReadonlyRootfs: boolean; CapDrop: string[] | null; SecurityOpt: string[] | null; Memory: number; PidsLimit: number | null; LogConfig: { Type: string; Config: Record<string, string> }; RestartPolicy: { Name: string }; PortBindings: Record<string, unknown> | null };
      const hardened = host.ReadonlyRootfs && host.CapDrop?.includes("ALL") && host.SecurityOpt?.some((option) => option.startsWith("no-new-privileges")) && host.Memory > 0 && (host.PidsLimit ?? 0) > 0 && host.LogConfig.Type === "json-file" && host.LogConfig.Config["max-size"] && host.LogConfig.Config["max-file"] && host.RestartPolicy.Name === "unless-stopped";
      if (!hardened) throw new Error(`the ${service} container is not hardened, limited and log-rotated as compose.yaml says: ${JSON.stringify(host).slice(0, 900)}`);
      if (service === "proctor" && Object.keys(host.PortBindings ?? {}).length > 0) throw new Error(`the proctor publishes ${JSON.stringify(host.PortBindings)} on the host`);
      say(`${service}: read-only root, capabilities dropped, no new privileges, ${host.Memory / 1024 ** 2} MiB, ${host.PidsLimit} pids, logs ${host.LogConfig.Config["max-file"]} × ${host.LogConfig.Config["max-size"]}, restart ${host.RestartPolicy.Name}`);
    }
    const instance = await probe(`${door.base}/instance`, { insecure: true }, signal);
    if (instance.status !== 200 || !instance.body.includes("teaching-proctor")) throw new Error(`GET ${door.base}/instance through Caddy answered ${instance.status}: ${instance.body.slice(0, 300)}`);
    const sent = Object.fromEntries(["strict-transport-security", "x-content-type-options", "x-frame-options", "content-security-policy", "referrer-policy", "server", "via"].map((name) => [name, instance.headers.get(name)]));
    if (!sent["strict-transport-security"] || sent["x-content-type-options"] !== "nosniff" || sent["x-frame-options"] !== "DENY" || !sent["content-security-policy"]?.includes("frame-ancestors 'none'") || !sent["referrer-policy"] || sent.server || sent.via) throw new Error(`GET /instance through Caddy carries the headers ${JSON.stringify(sent)}`);
    say(`GET ${door.base}/instance through Caddy: 200, ${Object.entries(sent).filter(([, value]) => value).map(([name, value]) => `${name}: ${value}`).join("; ")}`);
    const redirect = await probe(`http://localhost:${httpPort}/instance`, { headers: { "x-forwarded-proto": "https" } }, signal);
    if (redirect.status < 300 || redirect.status >= 400) throw new Error(`plain HTTP (claiming X-Forwarded-Proto: https) answered ${redirect.status} instead of redirecting to HTTPS`);
    say(`plain HTTP claiming X-Forwarded-Proto: https: ${redirect.status} → ${redirect.headers.get("location")}`);
    const forged = await probe(`${door.base}/instance`, { headers: { "x-forwarded-proto": "http" }, insecure: true }, signal);
    if (forged.status !== 200) throw new Error(`HTTPS with a client's X-Forwarded-Proto: http answered ${forged.status}; Caddy must replace the header`);
    say("HTTPS with a client's X-Forwarded-Proto: http: 200, Caddy replaced the header");
    await checkCrossOrigin(door, signal);
    await checkBodyLimit(door, bodyLimit, signal);
    (await checkPresence(door, signal)).close();
    const before = await identifyLearner(door, signal);
    const backup = await composed(["exec", "--no-TTY", "proctor", "proctor", "backup", "-"]);
    if (backup.code !== 0) throw new Error(`proctor backup - exited ${backup.code}: ${backup.stderr.slice(-600)}`);
    writeFileSync(join(directory, COURSE_IMAGE_DATABASE), backup.bytes);
    const events = await databaseEvents(join(directory, COURSE_IMAGE_DATABASE));
    if (events < 1) throw new Error(`the backup holds ${events} events after learner ${before} registered`);
    say(`backup while serving: ${backup.bytes.length} bytes, ${events} events (read with bun:sqlite)`);
    const after = await identifyLearner(door, signal);
    const answers: number[] = [];
    let updating = true;
    const watching = (async (): Promise<void> => {
      while (updating) {
        answers.push(await probe(`${door.base}/instance`, { insecure: true, timeoutMs: 60_000 }, signal).then((answer) => answer.status, () => 0));
        await pause(250, signal).catch(() => undefined);
      }
    })();
    const updated = Date.now();
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180", "--force-recreate", "proctor"]).finally(() => (updating = false));
    await watching;
    if (answers.some((status) => status !== 200)) throw new Error(`while the proctor container was replaced, GET /instance through Caddy answered ${answers.join(" ")}`);
    if (!(await learnerKnown(door, before, signal)) || !(await learnerKnown(door, after, signal))) throw new Error("a learner registered before the update is gone after it");
    say(`update (proctor container replaced in ${Date.now() - updated} ms): ${answers.length} requests through Caddy, none failed; both learners still known`);
    const refused = await composed(["exec", "--no-TTY", "proctor", "proctor", "restore", "-"], backup.bytes);
    if (refused.code === 0) throw new Error("proctor restore replaced the database of a serving proctor");
    say(`restore while serving: refused (${refused.stderr.split("\n").at(-1)?.slice(0, 160)})`);
    await compose(["stop", "proctor"]);
    const restored = await composed(["run", "--rm", "--no-TTY", "--no-deps", "proctor", "restore", "-"], backup.bytes);
    if (restored.code !== 0) throw new Error(`proctor restore - exited ${restored.code}: ${restored.stderr.slice(-600)}`);
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180"]);
    if (!(await learnerKnown(door, before, signal)) || (await learnerKnown(door, after, signal))) throw new Error("the restored proctor does not answer the moment of the backup");
    say("restore once stopped: the proctor answers the moment of the backup (the earlier learner known, the later one not)");
    const handle = `Deploy Check ${randomBytes(3).toString("hex")}`;
    const named = await identifyLearner(door, signal, handle);
    if (!(await learnerKnown(door, named, signal))) throw new Error(`the learner registered as ${handle} is unknown`);
    await compose(["stop", "proctor"]);
    const erasure = ["run", "--rm", "--no-TTY", "--no-deps", "proctor", "erase", "--handle", handle];
    const [planned, erased] = [await composed([...erasure, "--dry-run"]), await composed(erasure)];
    if (planned.code !== 0 || erased.code !== 0) throw new Error(`proctor erase exited ${planned.code} (dry run) and ${erased.code}: ${(planned.stderr + erased.stderr).slice(-600)}`);
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180"]);
    if ((await learnerKnown(door, named, signal)) || !(await learnerKnown(door, before, signal))) throw new Error(`after erasing ${handle} that learner is still known, or another one is gone`);
    say(`erase --handle ${JSON.stringify(handle)}: ${planned.stdout.split("\n").at(-1)}; then ${erased.stdout.split("\n").at(-1)?.slice(0, 60)}; that learner is unknown, the other one still known`);
    const own = await servedCertificate(Number(httpsPort), signal);
    const bundle = await mintLocalCertificate(caddyImage, signal);
    const supplied = new X509Certificate(bundle).fingerprint256;
    if (own === supplied) throw new Error(`the minted certificate ${supplied} is the one Caddy already serves`);
    writeFileSync(join(directory, COURSE_STACK_CERTIFICATES, "localhost.pem"), bundle, { mode: 0o644 });
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180", "--force-recreate", "caddy"]);
    const served = await servedCertificate(Number(httpsPort), signal);
    if (served !== supplied) throw new Error(`with ${COURSE_STACK_CERTIFICATES}/localhost.pem (${supplied}) supplied and the caddy service recreated, Caddy serves ${served} (its own was ${own})`);
    const logged = await composed(["logs", "--no-log-prefix", "caddy"]);
    const unmanaged = /skipping automatic certificate management[^\n]*"domain":"localhost"/u.test(`${logged.stdout}\n${logged.stderr}`);
    const through = await probe(`${door.base}/instance`, { insecure: true }, signal);
    if (!unmanaged || through.status !== 200) throw new Error(`with the supplied certificate Caddy ${unmanaged ? "manages none of its own" : "still manages its own"} and GET /instance answers ${through.status}`);
    say(`supplied certificate: Caddy served its own ${own}; with ${COURSE_STACK_CERTIFICATES}/localhost.pem of another CA and the caddy service recreated it serves ${served}, the supplied one, manages none for localhost, and GET /instance answers 200`);
    say("stack passed");
  } catch (error) {
    if (started) console.error(`[deploy] stack logs:\n${(await docker(["compose", "--project-name", project, "--file", "compose.yaml", "logs", "--tail", "30"], { signal: AbortSignal.timeout(5_000), env, cwd: directory }).catch(() => ({ stdout: "", stderr: "" }))).stdout.slice(-3000)}`);
    throw error;
  } finally {
    release();
    if (started && !keep) await docker(["compose", "--project-name", project, "--file", "compose.yaml", "down", "--volumes", "--timeout", "30"], { signal: AbortSignal.timeout(COURSE_DEPLOY_CLEANUP_MS), env, cwd: directory }).catch(() => undefined);
    if (directory && !keep) rmSync(directory, { recursive: true, force: true });
  }
}
//#endregion 🧱️Stack
