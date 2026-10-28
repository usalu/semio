import { expect, test, spyOn } from "bun:test";
import { Database } from "bun:sqlite";
import diagnostics from "../../🧬️schema/📣️diagnostics.json";
import { cargoProvenanceConsoleSinkV1, nativeBuildDiagnosticLinesV1 } from "../🟦️.ts";

test("every native build diagnostic exists in English and German with the same placeholders and renders as the SQLite oracle says, in the chosen language or in both", () => {
  const database = new Database(":memory:"), labels = diagnostics.const.labels, holes = (text: string) => [...text.matchAll(/\{([A-Za-z]+)\}/gu)].map(match => match[1]).sort().join(",");
  try {
    expect(Object.keys(labels.en)).toEqual(Object.keys(labels.de));
    for (const key of Object.keys(labels.en) as (keyof typeof labels.en)[]) {
      expect(holes(labels.en[key]), key).toBe(holes(labels.de[key]));
      expect(labels.en[key], key).not.toBe(labels.de[key]);
      const values = Object.fromEntries([...labels.en[key].matchAll(/\{([A-Za-z]+)\}/gu)].map(match => [match[1]!, "v" + match[1]]));
      for (const [environment, languages] of [[{ SEMIO_LANGUAGE: "de" }, ["de"]], [{ LANG: "en_US.UTF-8" }, ["en"]], [{}, ["en", "de"]]] as [Record<string, string>, string[]][]) {
        const lines = nativeBuildDiagnosticLinesV1(key, values, environment);
        expect(lines, key).toHaveLength(languages.length);
        languages.forEach((language, index) => {
          const oracle = database.query("select replace(replace(replace(replace(replace(replace(json_extract(?1,'$.'||?2||'.'||?3),'{count}',?4),'{path}',?5),'{directory}',?6),'{tokens}',?7),'{cores}',?8),'{budget}',?9) line").get(JSON.stringify(labels), language, key, values.count ?? "", values.path ?? "", values.directory ?? "", values.tokens ?? "", values.cores ?? "", values.budget ?? "") as { line: string };
          const expected = oracle.line.replaceAll("{work}", "vwork").replaceAll("{bytes}", "vbytes").replaceAll("{totalBytes}", "vtotalBytes");
          expect(lines[index], `${key} ${language}`).toBe(expected);
        });
      }
    }
    for (const key of Object.keys(labels.en) as (keyof typeof labels.en)[]) { expect(labels.de[key], key).not.toMatch(/\b(?:the|of|not|shown|staged)\b/u); expect(labels.de[key], key).not.toMatch(/[a-zA-Z](?:ae|oe|ue)[a-zA-Z]/u); }
    const printed: string[] = [], spy = spyOn(console, "log").mockImplementation(((line: unknown) => { printed.push(String(line)); }) as typeof console.log);
    try { cargoProvenanceConsoleSinkV1("provenance", { SEMIO_LANGUAGE: "de" })({ bytes: 5, totalBytes: 10, work: 3 }); cargoProvenanceConsoleSinkV1("recipe", { SEMIO_LANGUAGE: "en" })({ bytes: 0, totalBytes: 0, work: 1 }); } finally { spy.mockRestore(); }
    expect(printed).toEqual(["[cargo-provenance] Build-Dateien werden gelesen: 3 Dateivorgang/-vorgänge, 5 von 10 Bytes", "[cargo-target-recipe] reading the target recipe inputs: 1 file operation(s), 0 of 0 bytes"]);
    expect(printed.join("\n")).not.toContain("{");
  } finally { database.close(); }
});
