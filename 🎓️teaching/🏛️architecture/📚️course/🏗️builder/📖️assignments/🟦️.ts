import { publishPrintArtifact } from "@semio-tech/print/modules/tectonic-template-compilation";
import { withPrintExecutionV1 } from "@semio-tech/print/artifacts/operation";
import { createIssuedResourceControlV1, type ScriptInvocation } from "@semio-tech/process/routing";
import course from "../../🔣️.json" with { type: "json" };
import plan from "../../../📐️parametric/📝️assignment/icons.json" with { type: "json" };

export type CourseAssignmentLanguage = "en" | "de";
type AssignmentSource = Readonly<{ sourceRoot: string; sources: readonly string[]; resources: readonly Readonly<{ source: string; name: string; format: "svg-png"; width: number; sha256: string }>[] }>;

/** 🗂️ Authored TeX leaves and their language-independent sources for every declared course assignment. */
export const COURSE_ASSIGNMENT_SOURCES: Readonly<Record<string, AssignmentSource>> = {
  parametrization: {
    sourceRoot: "🎓️teaching/🏛️architecture/📐️parametric/📝️assignment", sources: ["geometry.tex", "icons.json"],
    resources: plan.resources.map(row => ({ source: row.source, name: row.output, format: "svg-png", width: row.width, sha256: row.sha256 })),
  },
  typology: {
    sourceRoot: "🎓️teaching/🏛️architecture/🏙️typology/📝️assignment", sources: ["cover.jpg", "logo-luh.png", "logo-luh-dark.png", "schema-m-b-1-q.png", "schema-m-w-2-h.png", "schema-e-b-2-qh.png"], resources: [],
  },
};

/** 📖️ Builds one localized course assignment through the retained shared Print pipeline. */
export async function buildCourseAssignmentV1(workspace: string, invocation: ScriptInvocation, id: string, language: CourseAssignmentLanguage): Promise<void> {
  const assignment = course.assignments.find(row => row.id === id), source = COURSE_ASSIGNMENT_SOURCES[id];
  if (!assignment || !source) throw Error("Declared course assignment required: " + id);
  const document = assignment.documents[language];
  const phaseControl = createIssuedResourceControlV1(invocation, "architecture-course:assignment-build", 0);
  await withPrintExecutionV1({ workspace, invocation, phaseControl, kind: "document" }, execution => publishPrintArtifact(execution, {
    version: 1, kind: "document", namespace: document.namespace, id: document.id, producer: document.producer,
    sourceRoot: source.sourceRoot, texPath: language + ".tex", sources: [language + ".tex", language + ".content.tex", ...source.sources], dark: false, resources: source.resources,
  }).then(() => undefined));
}
