import AjvDraft07 from "ajv";
/** 📝️ Neutral assignment vectors are adjudicated by the independent JSON Schema implementation. */
import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import { readFileSync } from "node:fs";
import schema from "../../🧾️template/📝️assignment/🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
import { assignmentPreamble, escapeAssignmentText, isAssignment } from "../../🧾️template/📝️assignment/🟦️.ts";

const oracle = new AjvDraft07({ strict: true }).compile(schema);
for (const row of fixture.cases) test(row.name, () => {
  expect(isAssignment(row.value)).toBe(row.valid);
  expect(isAssignment(row.value)).toBe(Boolean(oracle(row.value)));
});
test("metadata preserves plain text through LaTeX escaping", () => {
  expect(escapeAssignmentText(fixture.escaping.input)).toBe(fixture.escaping.expected);
  const preamble = assignmentPreamble({ language: "en", theme: "light", title: fixture.escaping.input, course: "A&B" });
  expect(preamble).toContain("\\title{" + fixture.escaping.expected + "}");
  expect(preamble).toContain("\\documentclass[theme=light,language=en]{assignment}");
  expect(preamble).toContain("\\SemioAssignmentSetup{course={A\\&B}}");
});
test("declarations are checked before emitting TeX", () => {
  expect(() => assignmentPreamble({ title: "Missing language" } as never)).toThrow("Invalid assignment");
});
test("assignment tables retain configurable blank parameter columns and arbitrary rows", () => {
  const latex = readFileSync(new URL("../../🖋️latex/semio-assignment.sty", import.meta.url), "utf8");
  expect(latex).toContain("value-columns .initial:n = 5");
  expect(latex).toContain("\\NewDocumentEnvironment { AssignmentParameterTable }");
  expect(latex).toContain("\\NewDocumentCommand \\AssignmentParameterRow");
  expect(latex).toContain("\\prg_replicate:nn { \\l_semio_assignment_columns_int } { & }");
  expect(latex).not.toMatch(/\\int_compare.*8/);
});

test("canonical black icons retain their light appearance and gain readable dark-theme contrast", () => {
  const latex = readFileSync(new URL("../../🖋️latex/semio-assignment.sty", import.meta.url), "utf8");
  expect(fixture.contrast.light.backing).toBeNull();
  expect(latex).toContain("\\str_if_eq:VnTF \\l_semio_theme_tl { dark }");
  expect(latex).toContain("\\colorbox { " + fixture.contrast.dark.backing + " } { \\includegraphics[#1]{#2} }");
  expect(latex).toContain("\\setlength { \\fboxsep } { ." + String(fixture.contrast.dark.paddingMillimetres).split(".")[1] + "mm }");
  expect(fixture.contrast.imageHeightMillimetres + 2 * fixture.contrast.dark.paddingMillimetres).toBeLessThan(fixture.contrast.rowHeightMillimetres);
});

for (const row of fixture.pdfMetadata) test("PDF metadata reflects explicit " + row.language + " assignment declaration", () => {
  const latex = readFileSync(new URL("../../🖋️latex/semio-assignment.sty", import.meta.url), "utf8");
  const declaration = { language: row.language, theme: "light", title: "A & B", author: "Course Author" };
  expect(isAssignment(declaration)).toBe(Boolean(oracle(declaration)));
  expect(oracle(declaration)).toBe(true);
  expect(latex).toContain("\\AtBeginDocument {");
  expect(latex).toContain("pdftitle = { \\@title }, pdfauthor = { \\@author }");
  expect(latex).toContain("{ " + row.language + " } { \\hypersetup { pdflang = " + row.pdfLanguage + " } }");
});

test("transposed matrices put type icons in the header and retain eight writable parameter rows", () => {
  const latex = readFileSync(new URL("../../🖋️latex/semio-assignment.sty", import.meta.url), "utf8");
  const matrix = fixture.matrix;
  const validate = new Ajv({ strict: true }).compile({
    type: "object",
    properties: {
      typeColumns: { type: "integer", const: 8 },
      parameterRows: { type: "integer", const: 8 },
      totalColumns: { type: "integer", const: 9 },
      totalRows: { type: "integer", const: 9 },
      headerHeightMillimetres: { type: "number", const: 16 },
      rowHeightMillimetres: { type: "number", const: 6 },
      labelWidthMillimetres: { type: "number", const: 27 },
      columnPaddingMillimetres: { type: "number", const: 2 },
      imageFitsCell: { type: "boolean", const: true },
    },
    required: Object.keys(matrix),
    additionalProperties: false,
  });
  expect(validate(matrix)).toBe(true);
  expect(matrix.typeColumns + 1).toBe(matrix.totalColumns);
  expect(matrix.parameterRows + 1).toBe(matrix.totalRows);
  expect(latex).toContain("\\NewDocumentEnvironment { AssignmentParameterMatrix }");
  expect(latex).toContain("type-columns .initial:n = " + matrix.typeColumns);
  expect(latex).toContain("parameter-rows .initial:n = " + matrix.parameterRows);
  expect(latex).toContain("row-height .initial:n = " + matrix.rowHeightMillimetres + "mm");
  expect(latex).toContain("label-width .initial:n = " + matrix.labelWidthMillimetres + "mm");
  expect(latex).toContain("header-height .initial:n = " + matrix.headerHeightMillimetres + "mm");
  expect(latex).toContain("padding .initial:n = " + matrix.columnPaddingMillimetres + "mm");
  expect(latex).toContain("\\setlength { \\tabcolsep } { \\l_semio_assignment_matrix_padding_dim }");
  expect(latex).toContain("\\textbf{#2} & #3");
  expect(latex).toContain("\\vbox to \\l_semio_assignment_matrix_header_dim");
  expect(latex).toContain("\\renewcommand { \\arraystretch } { 0 }");
  expect(latex).toContain("\\semio_assignment_matrix_writing_cell:");
  expect(latex).toContain("\\NewDocumentCommand \\AssignmentMatrixImage");
  expect(matrix.headerHeightMillimetres + matrix.parameterRows * matrix.rowHeightMillimetres).toBe(64);
  expect(latex).toContain("\\NewExpandableDocumentCommand \\AssignmentParameterMatrixBlankRows { m }");
  expect(latex).toContain("\\AssignmentParameterMatrixBlankRows{\\l_semio_assignment_parameter_rows_int}");
  expect(latex).toContain("\\prg_replicate:nn {#1} { \\semio_assignment_matrix_blank_row: }");
  expect(latex).toContain("\\prg_replicate:nn { \\l_semio_assignment_type_columns_int } { & \\semio_assignment_matrix_writing_cell: }");
});


test("unnamed assignment matrices retain connected frames and automatic right chips", () => {
  const contract = fixture.silhouette;
  const validate = new Ajv({ strict: true }).compile({
    type: "object",
    properties: {
      component: { const: "Table" },
      connectedOutline: { const: true },
      titleInsideTable: { const: false },
      rightAutomaticChip: { const: true },
    },
    required: ["component", "connectedOutline", "titleInsideTable", "rightAutomaticChip"],
    additionalProperties: false,
  });
  expect(validate(contract)).toBe(true);
  const latex = readFileSync(new URL("../../🖋️latex/semio-assignment.sty", import.meta.url), "utf8");
  expect(latex).toContain("\\tl_set:Nn \\l_semio_assignment_matrix_title_tl {#1}");
  expect(latex).toContain("\\exp_args:NV \\semio_table_long_open:n \\l_semio_assignment_matrix_title_tl");
  expect(latex).toContain("\\use:c { semio@table@long@continuation@chrome }");
  expect(latex).toContain("\\semio_table_long_close:");
});

test("blank table names preserve an explicit active matrix context", () => {
  const latex = readFileSync(new URL("../../🖋️latex/semio-assignment.sty", import.meta.url), "utf8");
  expect(latex).toContain("\\bool_set_true:N \\l_semio_assignment_matrix_active_bool");
  expect(latex.split("\\bool_if:NT \\l_semio_assignment_matrix_active_bool").length - 1).toBe(2);
  expect(latex).not.toContain("\\tl_if_empty:NF \\l_semio_assignment_matrix_title_tl");
});
