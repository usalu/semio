import { INTERACTIVITY_AUDIT_PUZZLE_FILL_ENVELOPE_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_STATE_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_TRANSPORT_FILE, INTERACTIVITY_AUDIT_PUZZLE5D_FILL_PRECOMPUTE_FILE, INTERACTIVITY_AUDIT_PUZZLE5D_FILL_WINDOW_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_FILE, INTERACTIVITY_AUDIT_PUZZLE3D_TERMINOLOGY_FILE, INTERACTIVITY_AUDIT_PUZZLE5D_TERMINOLOGY_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_PREVIEW_FIXTURE_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_TEST_FILE, policyReadFileSafe, interactivityPuzzleFillPreviewJsonFailures } from "../../../../../📜️script.ts";

/** 🧪️ Executes interactivity puzzle fill preview json policy assertions. */
export function interactivityPuzzleFillPreviewJsonSelfTests(repoRoot: string): void {
  const sources = [
    INTERACTIVITY_AUDIT_PUZZLE_FILL_ENVELOPE_FILE,
    INTERACTIVITY_AUDIT_PUZZLE_FILL_STATE_FILE,
    INTERACTIVITY_AUDIT_PUZZLE_FILL_TRANSPORT_FILE,
    INTERACTIVITY_AUDIT_PUZZLE5D_FILL_PRECOMPUTE_FILE,
    INTERACTIVITY_AUDIT_PUZZLE5D_FILL_WINDOW_FILE,
    INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_FILE,
    INTERACTIVITY_AUDIT_PUZZLE3D_TERMINOLOGY_FILE,
    INTERACTIVITY_AUDIT_PUZZLE5D_TERMINOLOGY_FILE,
    INTERACTIVITY_AUDIT_PUZZLE_FILL_PREVIEW_FIXTURE_FILE,
    INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_TEST_FILE,
  ].map((file) => policyReadFileSafe(repoRoot, file));
  const mutations: [string, number, string, string][] = [
    ["output-cap", 1, "FILL_PREVIEW_JSON_MAX_BYTES: usize = 4 * 1024", "FILL_PREVIEW_JSON_MAX_BYTES: usize = 8 * 1024"],
    ["whole-preview", 1, "pub(crate) struct FillPreviewJsonCursor", "fn whole_preview() { let _ = serde_json::to_vec(&self.preview); }\npub(crate) struct FillPreviewJsonCursor"],
    ["lost-fuel", 1, "fuel.checked_sub(1)", "fuel.checked_sub(0)"],
    ["lost-deadline", 0, "now_us >= deadline", "false"],
    ["deadline-equality", 0, "now_us >= deadline", "now_us > deadline"],
    ["missing-start-clock", 0, "let mut previous_us = default_now_us()?", "let mut previous_us = default_now_us().unwrap_or(0)"],
    ["missing-current-clock", 0, "let now_us = default_now_us()?", "let now_us = default_now_us().unwrap_or(previous_us)"],
    ["backward-clock", 0, "if now_us < previous_us", "if false"],
    ["backward-read-authority", 0, "previous_us = now_us", "previous_us = previous_us"],
    ["deadline-overflow", 0, "let deadline = previous_us.checked_add(2_000)?", "let deadline = previous_us.saturating_add(2_000)"],
    ["lost-puzzle3d-consumer", 2, "session.fill_preview_json_page(&color, labels.fill_progress.as_str())", "None"],
    ["lost-puzzle5d-adapter", 3, "self.inner.fill_preview_json_page(color, status_label)", "None"],
    ["lost-puzzle5d-consumer", 4, "labels.fill_progress.as_str()", "\"Fill progress\""],
    ["short-page", 5, "diagnostic.candidatePage.length !== 8", "diagnostic.candidatePage.length > 8"],
    ["array-ghost", 5, "!Array.isArray(candidateGhost)", "true"],
    ["root-key-census", 5, "censusAllowedOwnKeys(parsed, WORLD_FILL_ROOT_KEYS) < 0", "false"],
    ["diagnostic-key-census", 5, "censusAllowedOwnKeys(diagnostic, WORLD_FILL_DIAGNOSTIC_KEYS) !== WORLD_FILL_DIAGNOSTIC_KEYS.size", "false"],
    ["ghost-key-census", 5, "censusAllowedOwnKeys(candidateGhost, WORLD_FILL_GHOST_KEYS) === WORLD_FILL_GHOST_KEYS.size", "true"],
    ["root-source-index-shape", 5, 'Object.prototype.hasOwnProperty.call(parsed, "sourceVortexIndex") && !nonnegativeInteger(parsed.sourceVortexIndex)', "false"],
    ["root-color-byte-cap", 5, '!boundedUtf8(parsed.color, WORLD_FILL_COLOR_MAX_BYTES)', "false"],
    ["root-opacity", 5, "parsed.opacity !== 0.35", "false"],
    ["root-ghost-authority", 5, "diagnostic.candidateGhost.targetVortexFullId !== parsed.targetVortexFullId", "false"],
    ["hidden-label", 5, "<span>{diagnostic.statusLabel}</span>", "<span />"],
    ["puzzle3d-locale-default", 6, "None", "Some(Locale::En)"],
    ["puzzle5d-locale-default", 7, "None", "Some(Locale::En)"],
    ["fixture-cap", 8, '"maximumBytes": 4096', '"maximumBytes": 4097'],
    ["missing-oracle-law", 1, "retained_preview_json_matches_language_neutral_fixture_and_test_only_serde_oracle", "preview_json_smoke"],
    ["missing-extra-root-law", 9, 'page("Fill progress", {}, { extra: true })', 'page("Fill progress")'],
    ["missing-extra-diagnostic-law", 9, 'page("Fill progress", { extra: true })', 'page("Fill progress")'],
    ["missing-known-root-shape-laws", 9, "for (const malformedRoot of [", "for (const ignoredMalformedRoot of ["],
    ["missing-root-ghost-authority-laws", 9, "for (const mismatchedRoot of [", "for (const ignoredMismatchedRoot of ["],
  ];
  for (const [name, index, from, to] of mutations) {
    const mutated = [...sources];
    mutated[index] = mutated[index]!.replace(from, to);
    if (mutated[index] === sources[index]) throw new Error(`[verify interactivity] Puzzle fill preview self-test mutation ${name} no longer reaches production source.`);
    if (interactivityPuzzleFillPreviewJsonFailures(...(mutated as [string, string, string, string, string, string, string, string, string, string])).length === 0) throw new Error(`[verify interactivity] Puzzle fill preview self-test ${name} was falsely accepted.`);
  }
  const failures = interactivityPuzzleFillPreviewJsonFailures(...(sources as [string, string, string, string, string, string, string, string, string, string]));
  if (failures.length !== 0) throw new Error(`[verify interactivity] Puzzle fill preview baseline was falsely rejected: ${failures.join("; ")}`);
}
