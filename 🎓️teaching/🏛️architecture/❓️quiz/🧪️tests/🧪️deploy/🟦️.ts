/** 🧪️ The split deployment agrees with its one authored source of hosts (`🚀️deploy/🔣️.json`): the Dockerfile bakes the
 * production gate for the site origin, `compose.yaml` (parsed by the third-party `yaml` library) runs the proctor unexposed
 * behind Caddy with the same defaults, the Caddyfile serves the proctor host, and `publish` accepts exactly a CDN artifact
 * that bakes the proctor origin and names no loopback address.
 * @see ../../🚀️deploy/🔣️.json — the hosts, image and port
 * @see ../../🚀️deploy/🟦️.ts — the operator verbs under test */
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";
import { afterEach, describe, expect, it } from "vitest";
import deployment from "../../🚀️deploy/🔣️.json" with { type: "json" };
import { QUIZ_PROCTOR_ORIGIN, QUIZ_SITE_HEADERS, QUIZ_SITE_ORIGIN, releaseProctorOrigin, siteArtifactProblems } from "../../🚀️deploy/🟦️.ts";

const deploy = resolve(dirname(fileURLToPath(import.meta.url)), "../../🚀️deploy");
const read = (name: string): string => readFileSync(join(deploy, name), "utf8");
type Service = { image?: string; pull_policy?: string; build?: { dockerfile: string }; expose?: string[]; ports?: string[]; environment?: Record<string, string>; volumes?: string[]; depends_on?: Record<string, { condition: string }>; restart?: string };
const compose = parse(read("compose.yaml")) as { services: Record<string, Service>; volumes: Record<string, unknown> };
const scratch: string[] = [];

/** 📂️ A throw-away site artifact with `files` (relative path → content). */
function artifact(files: Record<string, string>): string {
  const root = mkdtempSync(join(tmpdir(), "quiz-site-"));
  scratch.push(root);
  for (const [path, content] of Object.entries(files)) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  }
  return root;
}

const released = (origin: string): Record<string, string> => ({ "index.html": "<!doctype html>", "404.html": "<!doctype html>", ".nojekyll": "", CNAME: `${deployment.site.host}\n`, "assets/app.js": `const proctor = ${JSON.stringify(origin)};` });

afterEach(() => {
  for (const root of scratch.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe("architecture quiz deployment", () => {
  it("serves the proctor on a subdomain of the site", () => {
    expect(deployment.proctor.host).toBe(`proctor.${deployment.site.host}`);
    expect(QUIZ_SITE_ORIGIN).toBe(`https://${deployment.site.host}`);
    expect(QUIZ_PROCTOR_ORIGIN).toBe(`https://${deployment.proctor.host}`);
    expect(releaseProctorOrigin({})).toBe(QUIZ_PROCTOR_ORIGIN);
    expect(releaseProctorOrigin({ PROCTOR_URL: "https://staging.example" })).toBe("https://staging.example");
  });

  it("bakes the production gate for the site origin into a proctor-only image", () => {
    const dockerfile = read("Dockerfile");
    for (const line of ["PROCTOR_MODE=production", `PROCTOR_ALLOWED_ORIGINS=${QUIZ_SITE_ORIGIN}`, "PROCTOR_TRUSTED_FORWARDING=proxy", "PROCTOR_BIND=0.0.0.0", `PROCTOR_PORT=${deployment.proctor.port}`, `EXPOSE ${deployment.proctor.port}`, "HEALTHCHECK", "/usr/bin/tini", "USER quiz", 'VOLUME ["/srv/quiz/data"]']) expect(dockerfile).toContain(line);
    for (const absent of ["PROCTOR_SITE", "bun install", "bun.sh", "/srv/quiz/site"]) expect(dockerfile).not.toContain(absent);
  });

  it("runs the proctor unexposed behind Caddy with the same defaults", () => {
    const { proctor, caddy } = compose.services;
    expect(proctor!.image).toBe(`${deployment.proctor.image}:\${PROCTOR_TAG:-latest}`);
    expect(proctor!.pull_policy).toBe("missing");
    expect(proctor!.build!.dockerfile).toBe("🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile");
    expect(proctor!.expose).toEqual([String(deployment.proctor.port)]);
    expect(proctor!.ports).toBeUndefined();
    expect(proctor!.environment!.PROCTOR_ALLOWED_ORIGINS).toBe(`\${PROCTOR_ALLOWED_ORIGINS:-${QUIZ_SITE_ORIGIN}}`);
    expect(caddy!.environment!.PROCTOR_HOST).toBe(`\${PROCTOR_HOST:-${deployment.proctor.host}}`);
    expect(caddy!.depends_on).toEqual({ proctor: { condition: "service_healthy" } });
    expect(caddy!.ports).toEqual(["${QUIZ_HTTP_PORT:-80}:80", "${QUIZ_HTTPS_PORT:-443}:443", "${QUIZ_HTTPS_PORT:-443}:443/udp"]);
    for (const service of [proctor!, caddy!]) expect(service.restart).toBe("unless-stopped");
    expect(Object.keys(compose.volumes).sort()).toEqual(["caddy-config", "caddy-data", "proctor-data"]);
  });

  it("proxies the proctor host to the proctor service", () => {
    const caddyfile = read("Caddyfile");
    expect(caddyfile).toContain(`{$PROCTOR_HOST:${deployment.proctor.host}} {`);
    expect(caddyfile).toContain(`reverse_proxy proctor:${deployment.proctor.port}`);
    expect(caddyfile.split("\n").filter((line) => !line.startsWith("#")).join("\n")).not.toMatch(/Access-Control/iu);
  });

  it("publishes exactly a CDN artifact that bakes the proctor origin", () => {
    expect(siteArtifactProblems(artifact(released(QUIZ_PROCTOR_ORIGIN)), QUIZ_PROCTOR_ORIGIN)).toEqual([]);
    expect(siteArtifactProblems(artifact(released("https://localhost:18443")), "https://localhost:18443")).toEqual([]);
    expect(siteArtifactProblems(artifact({ ...released(QUIZ_PROCTOR_ORIGIN), CNAME: "quizze.example\n" }), QUIZ_PROCTOR_ORIGIN)).toEqual([`CNAME names "quizze.example", not ${deployment.site.host}`]);
    expect(siteArtifactProblems(artifact({ ...released(QUIZ_PROCTOR_ORIGIN), "assets/dev.js": "fetch('http://127.0.0.1:8791')" }), QUIZ_PROCTOR_ORIGIN)).toEqual(["the site names 127.0.0.1 outside the baked proctor origin"]);
    expect(siteArtifactProblems(artifact({ "index.html": "", CNAME: `${deployment.site.host}\n` }), QUIZ_PROCTOR_ORIGIN)).toEqual(["404.html is missing", ".nojekyll is missing", `no script bakes the proctor origin ${QUIZ_PROCTOR_ORIGIN}`]);
  });

  it("marks hashed assets immutable and documents revalidated without overlapping rules", () => {
    const rules = QUIZ_SITE_HEADERS.trimEnd().split("\n").filter((line) => !line.startsWith(" "));
    expect(rules).toEqual(["/assets/*", "/", "/index.html", "/404.html"]);
    expect(QUIZ_SITE_HEADERS).toContain("/assets/*\n  Cache-Control: public, max-age=31536000, immutable\n");
    expect(QUIZ_SITE_HEADERS.match(/Cache-Control: no-cache/gu)).toHaveLength(3);
  });
});
