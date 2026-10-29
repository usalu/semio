import assert from "node:assert/strict";
import { writeFile } from "node:fs/promises";
import { isAbsolute, join } from "node:path";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { withCompiledPrintProbeV1, type CompiledPrintProbeV1 } from "../../../🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";
import { inspectPrintCompilerRefusalV1 } from "../../../🔨️modules/🖨️tectonic-template-compilation/📥️refusal/🟦️.ts";
import type { PrintExecutionV1 } from "../../../📦️artifacts/🎛️operation/🟦️.ts";
import { thesisPreamble, escapeThesisText, type Thesis } from "../../../🧾️template/🎓️thesis/🟦️.ts";
import fixture from "../🔣️.json";
import contrastFixture from "../../../🔨️modules/🎨print-design-token-paints/🧪️tests/🔣️.json";

const normalize = (value: string): string => value.normalize("NFC").replace(/[‘’]/g, "'").replace(/\s+/g, "");
const body = String.raw`\begin{document}
\setcounter{secnumdepth}{3}
\makeatletter
\convertcolorspec{named}{semio-chrome-text-normal}{HTML}{\SemioNormalHex}
\typeout{THESIS-COLOR-NORMAL=\SemioNormalHex}
\convertcolorspec{named}{\semio@heading@textcolor@chapter}{HTML}{\SemioOutlineHex}
\typeout{THESIS-COLOR-OUTLINE=\SemioOutlineHex}
\convertcolorspec{named}{\semio@heading@textcolor@section}{HTML}{\SemioFilledHex}
\typeout{THESIS-COLOR-FILLED=\SemioFilledHex}
\convertcolorspec{named}{\@citecolor}{HTML}{\SemioLinkHex}
\typeout{THESIS-COLOR-LINK=\SemioLinkHex}
\makeatother
\maketitle
\makefrontmatter
\typeout{THESIS-FRONT-PAGE=\arabic{page}}
\makefrontmatter
\typeout{THESIS-REPEATED-FRONT-PAGE=\arabic{page}}
\tableofcontents
\chapter{Front Research}Front research evidence.
\typeout{THESIS-FRONT-DEPTH=\arabic{secnumdepth}}
\typeout{THESIS-FRONT-CHAPTERS=\arabic{chapter}}
\begin{Abstract}Research abstract.\end{Abstract}
\begin{Kurzfassung}Deutsche Zusammenfassung.\end{Kurzfassung}
\begin{Dedication}Dedication content.\end{Dedication}
\begin{Abbreviations}\item[SEM] Research Method\end{Abbreviations}
\makedeclaration
\makemainmatter
\chapter{Research Body}Research evidence.
\typeout{THESIS-MAIN-DEPTH=\arabic{secnumdepth}}
\typeout{THESIS-MAIN-CHAPTERS=\arabic{chapter}}
\makebackmatter
\chapter{Research Appendix}Appendix evidence.
\typeout{THESIS-BACK-DEPTH=\arabic{secnumdepth}}
\typeout{THESIS-BACK-CHAPTERS=\arabic{chapter}}
\end{document}`;
const declaration = (language: "en" | "de", theme: "light" | "dark", degree: Thesis["degree"]): Thesis => ({
  language, theme, degree, ...(degree === "custom" ? { thesisType: "Independent Study" } : {}),
  title: "Thesis Fixture", author: "Ada Researcher", university: "Example University", awardedDegree: "Master of Science",
  programme: "Research Programme", studentId: "12345", supervisor: "Supervisor Example", reviewers: "Reviewer Example",
  submission: "2026-10-28", place: "Berlin", keywords: "Research Keywords", declarationText: "Authored declaration evidence.",
} as Thesis);

/** 🧾️ Retains bounded compiler evidence before interpreting any rendered fixture assertions. */
async function thesisProbeEvidence(probe: CompiledPrintProbeV1, name: string): Promise<{ log: string; contents: string; markers: readonly string[] }> {
  const log = new TextDecoder().decode(await probe.read("thesis.log", 1048576)), contents = new TextDecoder().decode(await probe.read("thesis.register-toc", 1048576)), markers = log.match(/THESIS-[A-Z-]+=[^\r\n]*/g) ?? [];
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (artifacts) {
    assert.ok(isAbsolute(artifacts), "Absolute rendering diagnostic directory");
    assert.match(name, /^[a-z-]+$/);
    const pdf = await probe.read("thesis.pdf", 67108864);
    for (const [extension, content] of [["log", log], ["register-toc", contents], ["pdf", pdf], ["facts.json", JSON.stringify({ kind: "private-thesis-probe-evidence", canonicalWorkspaceProduct: false, name, markers }, null, 2) + "\n"]] as const) {
      const target = join(artifacts, name + "." + extension);
      assert.ok(target.length <= fixture.privateWorkspace.maximumPathLength);
      checkResourceLeaseControl(probe.control);
      await writeFile(target, content, { signal: probe.control.signal });
    }
    checkResourceLeaseControl(probe.control);
  }
  return { log, contents, markers };
}

/** 🖨️ Exercises authored thesis declarations through the actual compiler and independent PDF.js reader. */
export async function verifyThesisRendering(execution: PrintExecutionV1): Promise<void> {
  for (const row of [...fixture.rendering.cases, fixture.rendering.doctoralSpecialization]) {
    const phd = row === fixture.rendering.doctoralSpecialization;
    const authored = thesisPreamble(declaration(row.language as "en" | "de", row.theme as "light" | "dark", row.degree as Thesis["degree"])) + body;
    const source = phd ? authored.replace("type=thesis", "type=phd").replace("\\maketitle", "\\makedissertationtitle") : authored;
    await withCompiledPrintProbeV1(execution, { version: 1, kind: "probe", name: "thesis.tex", source }, async probe => {
      const { log, contents, markers } = await thesisProbeEvidence(probe, "case-" + row.language + "-" + row.theme + "-" + row.degree + (phd ? "-phd" : ""));
      const pdf = await probe.inspect();
      assert.ok(pdf.pages.length >= 4, "Thesis must retain front matter and academic body");
      const text = pdf.pages.map(page => page.items.flatMap(item => "str" in item ? [item.str] : []).join(" ")).join(" ");
      assert.doesNotMatch(text, /[\uFFFD\uFFFF]/u);
      for (const expected of [row.heading, row.declarationHeading, row.dedicationHeading, row.abbreviationsHeading, "Thesis Fixture", "Ada Researcher", "Example University", "Master of Science", "Research Programme", "12345", "Supervisor Example", "Reviewer Example", "Research Keywords", "Authored declaration evidence.", "Research Body", "Research Appendix", "Deutsche Zusammenfassung."])
        assert.ok(normalize(text).includes(normalize(expected)), row.language + "/" + row.theme + "/" + row.degree + ": " + expected);
      const abstract = pdf.pages.find(page => normalize(page.items.flatMap(item => "str" in item ? [item.str] : []).join(" ")).includes("Researchabstract."));
      const research = pdf.pages.find(page => normalize(page.items.flatMap(item => "str" in item ? [item.str] : []).join(" ")).includes("Researchevidence."));
      assert.ok(abstract && research, "Front matter and research pages must exist");
      assert.ok(abstract.items.some(item => "str" in item && /^[ivxlcdm]+$/.test(item.str.trim()) && item.transform[5] < abstract.height / 4), "Front matter uses roman page numbers");
      assert.ok(research.items.some(item => "str" in item && /^1$/.test(item.str.trim()) && item.transform[5] < research.height / 4), "Main matter restarts Arabic numbering at one");
      const entries = [...contents.matchAll(/\\SemioTocAuxLine\s*\{([^{}]*)\}\s*\{([^{}]*)\}/g)].map(match => ({ number: match[1]!.trim(), title: normalize(match[2]!) }));
      for (const expected of [...fixture.rendering.contents, { title: row.declarationHeading, number: "" }, { title: row.dedicationHeading, number: "" }]) {
        const entry = entries.find(entry => entry.title === normalize(expected.title));
        assert.ok(entry, "Actual table of contents contains " + expected.title);
        assert.equal(entry.number, expected.number, expected.title + " contents number");
        assert.ok(normalize(text).split(normalize(expected.title)).length >= 3, "Printed contents and body contain " + expected.title);
      }
      const themePaints = contrastFixture.actualThemes.find(theme => theme.theme === row.theme);
      assert.ok(themePaints);
      for (const role of ["normal", "outline", "filled", "link"] as const) assert.ok(log.includes("THESIS-COLOR-" + role.toUpperCase() + "=" + themePaints[role]), row.theme + " actual semantic " + role + " paint");
      const frontPage = Number(log.match(/THESIS-FRONT-PAGE=(\d+)/)?.[1]);
      assert.ok(frontPage > 1 && frontPage % 2 === 1, "Front matter preserves title page numbering and starts recto");
      const repeatedFrontPage = Number(log.match(/THESIS-REPEATED-FRONT-PAGE=(\d+)/)?.[1]);
      assert.equal(repeatedFrontPage, frontPage, "Repeated front matter does not restart numbering");
      for (const [key, expected] of Object.entries(fixture.rendering.numbering)) {
        const marker = "THESIS-" + key.replace(/[A-Z]/g, letter => "-" + letter).toUpperCase() + "=" + expected;
        assert.ok(log.includes(marker), marker + "; actual typeouts=" + JSON.stringify(markers));
      }
      assert.doesNotMatch(log, /destination with the same identifier.*page\.i/);
      await probe.recheck();
    });
    console.log("[print] Thesis rendering " + row.language + "/" + row.theme + "/" + row.degree + (phd ? "/phd" : "") + " PASS");
  }
  await verifyMinimalTheses(execution);
  await verifyThesisCustomization(execution);
  const source = thesisPreamble(declaration("en", "light", "master")) + body;
  for (const row of fixture.rendering.invalid) {
    const mutated = "remove" in row ? source.replace(row.remove!, "") : source.replace(row.replace![0]!, row.replace![1]!);
    const invalid = "documentType" in row ? mutated.replace("type=thesis", "type=" + row.documentType) : mutated;
    assert.notEqual(invalid, source);
    const refusal = await inspectPrintCompilerRefusalV1(execution, { version: 1, kind: "probe", name: "thesis.tex", source: invalid });
    assert.ok(normalize(refusal.log).toLowerCase().includes(normalize(row.message).toLowerCase()), row.id + ": expected actual compiler diagnostic");
    console.log("[print] Thesis refusal " + row.id + " PASS");
  }
}


/** 🎛️ Compiles declaration precedence, custom section titles and localized acknowledgements independently. */
async function verifyThesisCustomization(execution: PrintExecutionV1): Promise<void> {
  for (const row of fixture.rendering.customization) {
    const { declarationText: omitted, ...authored } = declaration(row.language as "en" | "de", "light", "master");
    const source = thesisPreamble(authored) + [
      "\\begin{document}",
      "\\maketitle",
      "\\makedeclaration",
      "\\SemioSetup{declaration-text={" + escapeThesisText(row.declarationBody) + "},declaration-title={" + escapeThesisText(row.declarationTitle) + "},dedication-title={" + escapeThesisText(row.dedicationTitle) + "},abbreviations-title={" + escapeThesisText(row.abbreviationsTitle) + "}}",
      "\\makedeclaration",
      "\\makedeclaration[" + escapeThesisText(row.optionalBody) + "]",
      "\\makededication{Dedicated customization evidence.}",
      "\\begin{Abbreviations}\\item[SEM] Abbreviation customization evidence.\\end{Abbreviations}",
      "\\begin{Acknowledgements}Acknowledgement customization evidence.\\end{Acknowledgements}",
      "\\end{document}",
    ].join("\n");
    await withCompiledPrintProbeV1(execution, { version: 1, kind: "probe", name: "customization.tex", source }, async probe => {
      const pdf = await probe.inspect();
      const text = pdf.pages.map(page => page.items.flatMap(item => "str" in item ? [item.str] : []).join(" ")).join(" ");
      assert.doesNotMatch(text, /[\uFFFD\uFFFF]/u);
      for (const expected of [row.defaultDeclaration, row.defaultHeading, row.declarationTitle, row.dedicationTitle, row.abbreviationsTitle, row.acknowledgementsTitle, "Dedicated customization evidence.", "Abbreviation customization evidence.", "Acknowledgement customization evidence."])
        assert.ok(normalize(text).includes(normalize(expected)), row.language + " customization: " + expected);
      for (const expected of [row.declarationBody, row.optionalBody])
        assert.equal(normalize(text).split(normalize(expected)).length - 1, 1, row.language + " declaration body precedence: " + expected);
      await probe.recheck();
    });
    console.log("[print] Thesis customization " + row.language + " PASS");
  }
}


/** 🪶️ Compiles minimal academic covers and reads actual Babel scope transitions. */
async function verifyMinimalTheses(execution: PrintExecutionV1): Promise<void> {
  for (const row of fixture.rendering.minimal) {
    const source = thesisPreamble({ language: row.language, theme: row.theme, degree: row.degree, title: row.title, author: row.author } as Thesis) + String.raw`\begin{document}
\maketitle
\begin{Abstract}
\typeout{THESIS-ABSTRACT-LANGUAGE=\languagename}
Minimal English abstract.
\end{Abstract}
\typeout{THESIS-AFTER-ABSTRACT-LANGUAGE=\languagename}
\begin{Kurzfassung}
\typeout{THESIS-KURZFASSUNG-LANGUAGE=\languagename}
Minimale deutsche Zusammenfassung.
\end{Kurzfassung}
\typeout{THESIS-AFTER-KURZFASSUNG-LANGUAGE=\languagename}
\makemainmatter
\chapter{Minimal Research}Minimal research evidence.
\end{document}`;
    await withCompiledPrintProbeV1(execution, { version: 1, kind: "probe", name: "minimal.tex", source }, async probe => {
      const pdf = await probe.inspect();
      assert.ok(pdf.pages.length >= 4, "Minimal thesis retains cover, abstracts and main matter");
      const cover = pdf.pages[0]!.items.flatMap(item => "str" in item ? [item.str] : []).join(" ");
      assert.doesNotMatch(cover, /[\uFFFD\uFFFF]/u);
      for (const expected of [row.title, row.author, row.heading])
        assert.ok(normalize(cover).includes(normalize(expected)), row.language + " minimal cover: " + expected);
      for (const absent of row.absentLabels)
        assert.ok(!normalize(cover).includes(normalize(absent)), row.language + " minimal cover omits " + absent);
      assert.doesNotMatch(cover, /\b\d{4}\b/, "Minimal cover has no implicit date or identifier");
      const log = new TextDecoder().decode(await probe.read("minimal.log", 1048576));
      const scope = fixture.rendering.languageScopes.find(scope => scope.language === row.language)!;
      for (const key of ["abstract", "afterAbstract", "kurzfassung", "afterKurzfassung"] as const) {
        const marker = "THESIS-" + key.replace(/[A-Z]/g, letter => "-" + letter).toUpperCase() + "-LANGUAGE=" + scope[key];
        assert.ok(log.includes(marker), row.language + " actual Babel scope: " + marker);
      }
      await probe.recheck();
    });
    console.log("[print] Minimal thesis and Babel scope " + row.language + " PASS");
  }
}
