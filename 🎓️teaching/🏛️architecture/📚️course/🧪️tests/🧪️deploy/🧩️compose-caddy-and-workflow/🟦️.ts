/** 🧩️ The compose stack, Caddy proxy and release workflow tests. */
import { expect, it } from "vitest";
import deployment from "../../../🚀️deploy/🔣️.json" with { type: "json" };
import readiness from "../../../🧫️fixtures/🚀️readiness/🔣️.json" with { type: "json" };
import { COURSE_PAGES_DIRECTORY, COURSE_SITE_ORIGIN, COURSE_STACK_CERTIFICATES, caddySizeBytes, stackRequestBodyLimit } from "../../../🚀️deploy/🟦️.ts";
import { compose, read, site, workflow } from "../🧰️deploy-fixtures/🟦️.ts";
import type { Job, Service } from "../🧰️deploy-fixtures/🟦️.ts";

/** 🔐 Registers the hardened compose stack, the Caddy proxy and the release workflow tests. */
export function registerComposeCaddyAndWorkflowTests(): void {
  it("runs the proctor unexposed, hardened and limited behind a pinned Caddy", () => {
    const { proctor, caddy } = compose.services as { proctor: Service; caddy: Service };
    expect(compose.name).toBe("architecture-course");
    expect(proctor.image).toBe(`${deployment.proctor.image}:\${PROCTOR_TAG:-latest}`);
    expect(proctor.pull_policy).toBe("missing");
    expect(proctor.build).toBeUndefined();
    expect(proctor.expose).toEqual([String(deployment.proctor.port)]);
    expect(proctor.ports).toBeUndefined();
    expect(proctor.environment!.PROCTOR_ALLOWED_ORIGINS).toBe(`\${PROCTOR_ALLOWED_ORIGINS:-${COURSE_SITE_ORIGIN}}`);
    expect(proctor.volumes).toEqual(["proctor-data:/srv/course/data"]);
    expect(proctor.stop_grace_period).toBe("30s");
    expect(proctor.cap_add).toBeUndefined();
    expect(caddy.image).toMatch(/^caddy:\d+\.\d+\.\d+@sha256:[0-9a-f]{64}$/u);
    expect(caddy.environment!.PROCTOR_HOST).toBe(`\${PROCTOR_HOST:-${deployment.proctor.host}}`);
    expect(caddy.depends_on).toEqual({ proctor: { condition: "service_healthy" } });
    expect(caddy.ports).toEqual(["${COURSE_HTTP_PORT:-80}:80", "${COURSE_HTTPS_PORT:-443}:443", "${COURSE_HTTPS_PORT:-443}:443/udp"]);
    expect(caddy.volumes).toEqual(["./Caddyfile:/etc/caddy/Caddyfile:ro", `./${COURSE_STACK_CERTIFICATES}:/${COURSE_STACK_CERTIFICATES}:ro`, "caddy-data:/data", "caddy-config:/config"]);
    expect(caddy.cap_add).toEqual(["NET_BIND_SERVICE"]);
    expect(caddy.healthcheck!.test[0]).toBe("CMD");
    for (const service of [proctor, caddy]) {
      expect(service.restart).toBe("unless-stopped");
      expect(service.env_file).toEqual([{ path: ".env", required: false }]);
      expect(service.read_only).toBe(true);
      expect(service.cap_drop).toEqual(["ALL"]);
      expect(service.security_opt).toEqual(["no-new-privileges:true"]);
      expect(service.logging).toEqual({ driver: "json-file", options: { "max-size": "10m", "max-file": "5" } });
      expect(Number(service.deploy!.resources.limits.cpus)).toBeGreaterThan(0);
      expect(service.deploy!.resources.limits.memory).toMatch(/^\d+[MG]$/u);
      expect(service.deploy!.resources.limits.pids).toBeGreaterThan(0);
    }
    expect(Object.keys(compose.volumes).sort()).toEqual(["caddy-config", "caddy-data", "proctor-data"]);
  });
  it("proxies the proctor host to the proctor service with a body cap, timeouts and response headers", () => {
    const caddyfile = read("Caddyfile");
    const directives = caddyfile.split("\n").filter((line) => !line.trimStart().startsWith("#")).join("\n");
    expect(directives).toContain(`{$PROCTOR_HOST:${deployment.proctor.host}} {`);
    expect(directives).toContain(`reverse_proxy proctor:${deployment.proctor.port}`);
    expect(directives).toContain(`\ttls {\n\t\tload /${COURSE_STACK_CERTIFICATES}\n\t}\n`);
    expect(directives).not.toMatch(/Access-Control/iu);
    for (const directive of ["skip_install_trust", "read_header ", "read_body ", "idle ", "lb_try_duration ", "dial_timeout ", "response_header_timeout ", "encode zstd gzip", 'Strict-Transport-Security "max-age=31536000"', 'X-Content-Type-Options "nosniff"', 'X-Frame-Options "DENY"', "frame-ancestors 'none'", "-Server", "-Via"]) expect(directives).toContain(directive);
    expect(directives).toContain("max_size {$PROCTOR_LIMIT_BODY_BYTES:16384}");
    expect(directives).toContain('\t@oversized `{http.request.header.Content-Length} != "" && int({http.request.header.Content-Length}) > {$PROCTOR_LIMIT_BODY_BYTES:16384}`\n\trespond @oversized 413\n');
    expect(stackRequestBodyLimit(caddyfile)).toBe(16_384);
    expect(stackRequestBodyLimit("example.org {\n\trequest_body {\n\t\tmax_size 8KB\n\t}\n}\n")).toBe(8000);
    expect([caddySizeBytes("512"), caddySizeBytes("8KB"), caddySizeBytes("8KiB"), caddySizeBytes("2MiB"), caddySizeBytes("1mb")]).toEqual([512, 8000, 8192, 2_097_152, 1_000_000]);
    expect(() => caddySizeBytes("eight")).toThrow();
    expect(() => stackRequestBodyLimit("example.org {\n\treverse_proxy proctor:8791\n}\n")).toThrow("caps no request body");
  });
  it("provisions the native release frontend before every managed release command", () => {
    const contract = readiness.release.nativeIngress;
    const names = Object.keys(contract.ordinaryCommands) as (keyof typeof contract.ordinaryCommands)[];
    for (const name of names) {
      const steps = workflow.jobs[name]!.steps as (Job["steps"][number] & { id?: string; shell?: string })[];
      const setup = steps.findIndex(step => step.id === contract.stepId);
      const launcher = steps.findIndex(step => step.uses === "./.github/actions/repo-launcher");
      expect(setup, `${name} has no native frontend provisioning`).toBeGreaterThanOrEqual(0);
      expect(launcher, `${name} has no repo launcher provisioning`).toBeGreaterThanOrEqual(0);
      expect(setup).toBeGreaterThan(launcher);
      expect(steps[setup]!.shell).toBe(contract.shell);
      expect(steps[setup]!.run?.trim().split("\n").map(line => line.trimEnd())).toEqual(contract.recipe);
      expect(steps.slice(0, setup).filter(step => step.run)).toEqual([]);
      const ordinary = steps.filter((step, index) => step.run && index !== setup).map(step => step.run);
      expect(ordinary).toEqual(contract.ordinaryCommands[name].map(argv => `${contract.envelope} repo ${argv.join(" ")}`));
    }
  });
  it("releases from the actual repository default branch only, with pinned actions and the least permissions", () => {
    expect(Object.keys(workflow.on)).toEqual(["workflow_dispatch"]);
    expect(workflow.permissions).toEqual({ contents: "read" });
    expect(workflow.concurrency).toEqual({ group: "architecture-course", "cancel-in-progress": false });
    const { configuration, check, site: siteJob, pages, proctor } = workflow.jobs as { configuration: Job; check: Job; site: Job; pages: Job; proctor: Job };
    expect(Object.keys(workflow.jobs)).toEqual([readiness.release.configurationJob, readiness.release.job, "site", "pages", "proctor"]);
    expect(configuration.if).toBe(readiness.release.condition);
    expect(configuration.outputs).toEqual({runner:`\${{ vars.${readiness.release.runnerVariable} }}`});
    expect(configuration.steps.map(step=>step.run)).toEqual(['test -n "$ARCHITECTURE_COURSE_RUNNER"']);
    expect(check.needs).toBe(readiness.release.configurationJob);
    expect(check.outputs).toEqual({runner:readiness.release.runner});
    expect(check["runs-on"]).toBe(readiness.release.runner);
    expect(check.if).toBe(readiness.release.condition);
    expect(check.steps.filter((step) => step.run).map((step) => step.run)).toEqual(readiness.release.commands);
    expect(check.permissions).toBeUndefined();
    expect(check["continue-on-error"]).not.toBe(true);
    for (const step of check.steps) expect(step["continue-on-error"]).not.toBe(true);
    for (const consumer of readiness.release.consumers) expect(workflow.jobs[consumer]?.needs).toBe(readiness.release.job);
    expect(siteJob.if).toBe(readiness.release.siteCondition);
    expect(proctor.if).toBe(readiness.release.proctorCondition);
    expect(siteJob.permissions).toBeUndefined();
    expect([pages.needs, pages.permissions]).toEqual(["site", { pages: "write", "id-token": "write" }]);
    expect(proctor.permissions).toEqual({ contents: "read", packages: "write" });
    expect(configuration["runs-on"]).toBe("ubuntu-24.04");expect(pages["runs-on"]).toBe("ubuntu-24.04");
    for(const job of[siteJob,proctor])expect(job["runs-on"]).toBe(readiness.release.consumerRunner);
    for (const job of [configuration, check, siteJob, pages, proctor]) {
      expect(job["timeout-minutes"]).toBeGreaterThan(0);
      for (const step of job.steps) if (step.uses) expect(step.uses).toMatch(/^(?:\.\/\.github\/actions\/repo-launcher|[\w-]+\/[\w-]+@[0-9a-f]{40})$/u);
      for (const step of job.steps) if (step.uses?.startsWith("actions/checkout@")) expect(step.with).toEqual({ "persist-credentials": false });
    }
    expect(siteJob.steps.filter((step) => step.run).map((step) => step.run)).toEqual(readiness.release.siteCommands);
    expect(siteJob.steps.at(-1)!.with).toEqual({ path: `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/release/${COURSE_PAGES_DIRECTORY}` });
    const release = proctor.steps.filter(step => step.run || step.uses?.startsWith("docker/login-action@")).map(step => step.run ?? step.uses!.split("@")[0]);
    expect(release).toEqual(readiness.release.proctorSteps);
  });
}
