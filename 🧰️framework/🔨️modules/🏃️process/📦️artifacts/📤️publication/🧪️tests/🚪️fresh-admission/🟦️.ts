import {expect, mock, test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdtempSync, rmSync} from "node:fs";
import {join} from "node:path";
import * as leaseModule from "../../../../🔒️leases/🟦️.ts";
import {withFreshArtifactPublicationWritersV1} from "../../🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {declareBuildTestEnvelopeV1, withBuildTestInvocationV1} from "../../../../🧪️testing/📥️invocation/🟦️.ts";
import cases from "./🔣️.json" with {type:"json"};
import policy from "../../🧬️schema/🚪️fresh-admission.json" with {type:"json"};

declareBuildTestEnvelopeV1({owner:"fresh-artifact-directory-admission",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});

test("fresh publication tolerates directory contention and keeps the target try-lock", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Explicit test artifact directory required");
  const root = mkdtempSync(join(output, "fw-"));
  const leases = {...leaseModule};
  const independent = new Database(":memory:");
  const oracle = independent.query("SELECT CASE WHEN ?1 = 'directory' THEN min(?2 + 1, ?3) ELSE 1 END AS attempts, ?1 = 'directory' AND ?2 < ?3 AS succeeds");
  try {
    await withBuildTestInvocationV1(async invocation => {
      const control = createIssuedResourceControlV1(invocation, "artifact:retry-directory-admission", 0);
      for (const [index, candidate] of cases.entries()) {
        const counts = {capture:0, destination:0};
        const target = join(root, String(index));
        const expected = oracle.get(candidate.failure, candidate.failures, policy.const.directoryAttempts);
        expect(expected).toEqual({
          attempts:candidate.resource === "capture" ? candidate.captureAttempts : candidate.destinationAttempts,
          succeeds:Number(candidate.published),
        });
        mock.module("../../../../🔒️leases/🟦️.ts", () => ({
          ...leases,
          acquireResourceLease: async (options: leaseModule.LeaseOptions) => {
            const resource = options.resource.startsWith("artifact-capture:") ? "capture" : "destination";
            if (++counts[resource] <= candidate.failures && resource === candidate.resource) {
              const error = new leases.ResourceLeaseTimeoutErrorV1(options, 1);
              if (candidate.failure === "directory") {
                Object.defineProperty(error, "resourceLeaseAdmission", {
                  value: {operation:"open", resource:options.resource, mode:"shared"},
                });
              }
              throw error;
            }
            return leases.acquireResourceLease(options);
          },
        }));
        const publish = withFreshArtifactPublicationWritersV1(target, root, control, async () => "ready");
        if (candidate.published) await expect(publish).resolves.toBe("ready");
        else await expect(publish).rejects.toThrow(leases.ResourceLeaseTimeoutErrorV1);
        expect(counts).toEqual({capture:candidate.captureAttempts, destination:candidate.destinationAttempts});
      }
    });
  } finally {
    mock.module("../../../../🔒️leases/🟦️.ts", () => leases);
    independent.close();
    rmSync(root, {recursive:true,force:true});
  }
});
