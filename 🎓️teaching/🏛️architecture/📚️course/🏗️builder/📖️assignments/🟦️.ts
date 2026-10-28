import { publishPrintArtifact } from "@semio-tech/print/modules/tectonic-template-compilation";
import { withPrintExecutionV1 } from "@semio-tech/print/artifacts/operation";
import { createIssuedResourceControlV1, type ScriptInvocation } from "@semio-tech/process/routing";
import course from "../../🔣️.json" with { type: "json" };
import plan from "../../../📐️parametric/📝️assignment/icons.json" with { type: "json" };

/** 📖️ Builds one localized course assignment through the retained shared Print pipeline. */
export async function buildCourseAssignmentV1(workspace: string, invocation: ScriptInvocation, language: "en" | "de"): Promise<void> {
  const assignment = course.assignments.find(row => row.id === "parametrization");
  if (!assignment) throw Error("Declared parametrization assignment required");
  const document = assignment.documents[language];
  const phaseControl = createIssuedResourceControlV1(invocation, "architecture-course:assignment-build", 0);
  await withPrintExecutionV1({ workspace, invocation, phaseControl, kind: "document" }, execution => publishPrintArtifact(execution, {
    version: 1, kind: "document", namespace: document.namespace, id: document.id, producer: document.producer,
    sourceRoot: "🎓️teaching/🏛️architecture/📐️parametric/📝️assignment", texPath: language + ".tex",
    sources: [language + ".tex", language + ".content.tex", "geometry.tex", "icons.json"], dark: false,
    resources: plan.resources.map(row => ({ source: row.source, name: row.output, format: "svg-png", width: row.width, sha256: row.sha256 })),
  }).then(() => undefined));
}
