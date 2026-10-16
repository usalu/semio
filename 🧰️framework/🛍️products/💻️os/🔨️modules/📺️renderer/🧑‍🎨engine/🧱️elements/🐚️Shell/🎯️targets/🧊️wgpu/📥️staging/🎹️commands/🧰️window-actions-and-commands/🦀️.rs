//! 🎹️ `shell-commands` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🎬️WindowActionsAndSearchPanes
/// 🆔️ React's id for a window-search input whose engagement names none, or names an internal chrome
/// control (`id={… ? UI_WINDOW_SEARCH.action : input.id}`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10868`).
const WINDOW_SEARCH_ACTION_CONTROL_ID: &str = "ui.windowSearch.action";

/// 🆔️ React's id for the search pane's possibles chevron (`Action id={UI_WINDOW_SEARCH.suggestions}`,
/// `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10938`).
const WINDOW_SEARCH_SUGGESTIONS_CONTROL_ID: &str = "ui.windowSearch.suggestions";

/// 🔎️ React's `searchPossibleRankScore` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10269`) — the window search
/// line's OWN ranker, higher-is-better, `-1` for no match. Deliberately NOT `rank_fuzzy_items`: that
/// is the ⌘️K palette's ranker (`//#region 🔎️FuzzyRankingParity`), and React's window search never
/// calls it, so borrowing it here would order the two shells' suggestion lists differently.
fn search_possible_rank_score(query: &str, possible: &ui_wgpu::wgpu::WindowEngagementPossible) -> i32 {
    let needle = normalize_engagement_action_text(query).to_lowercase();
    if needle.is_empty() {
        return -1;
    }
    let label = normalize_engagement_action_text(&possible.label).to_lowercase();
    let detail = possible.detail.clone().unwrap_or_default().to_lowercase();
    let id = possible.id.to_lowercase();
    if label.starts_with(&needle) {
        return 3000 - label.chars().count() as i32;
    }
    if detail.starts_with(&needle) {
        return 2000 - detail.chars().count() as i32;
    }
    if id.starts_with(&needle) {
        return 1000 - id.chars().count() as i32;
    }
    if format!("{label} {detail} {id}").contains(&needle) {
        return 500;
    }
    -1
}

/// ⌨️ React's `normalizeEngagementActionText`: separators dropped, tokens PascalCased, a decimal point
/// INSIDE a number preserved (`3.5` stays `3.5`). Display and matching only — the typed line itself
/// reaches the program verbatim.
pub fn normalize_engagement_action_text(text: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    let mut word = String::new();
    let chars: Vec<char> = text.chars().collect();
    for (index, ch) in chars.iter().copied().enumerate() {
        let decimal_inside_number = ch == '.' && index > 0 && chars[index - 1].is_ascii_digit() && chars.get(index + 1).is_some_and(char::is_ascii_digit);
        if ch.is_alphanumeric() || decimal_inside_number {
            word.push(ch);
            continue;
        }
        if !word.is_empty() {
            words.push(core::mem::take(&mut word));
        }
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
        .into_iter()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// 🔎️ React's `filterSearchPossibles`: an empty query keeps the host's own order, otherwise the rows
/// that score at all, best first.
pub fn filter_search_possibles<'a>(query: &str, possibles: &'a [ui_wgpu::wgpu::WindowEngagementPossible]) -> Vec<&'a ui_wgpu::wgpu::WindowEngagementPossible> {
    if normalize_engagement_action_text(query).is_empty() {
        return possibles.iter().collect();
    }
    let mut scored: Vec<(i32, usize, &ui_wgpu::wgpu::WindowEngagementPossible)> = possibles.iter().enumerate().map(|(index, possible)| (search_possible_rank_score(query, possible), index, possible)).filter(|(score, _, _)| *score >= 0).collect();
    scored.sort_by(|left, right| right.0.cmp(&left.0).then(left.1.cmp(&right.1)));
    scored.into_iter().map(|(_, _, possible)| possible).collect()
}

impl CommandState {
    /// 🔎️ ONE window's Search pane body — React's top-middle `Pane` content: the typed action line,
    /// the possibles chevron, and (while the chevron is open) the ranked suggestion rows
    /// (`Search`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10758`). `None` when the engagement declares no
    /// input, which is React's own `if (!hasInput) return null`.
    ///
    /// ✍️ React's line binds FOUR moments at once and this one binds the same four (the retained
    /// producers landed with this packet, `🖱️ui/🎯️targets/🧊️/⚡️events`'s `search_line_action`):
    ///
    /// * `onChange` on every keystroke — `commit` is deliberately UNSET so the node fires
    ///   `Trigger::Change` per keystroke, which is what feeds the guest's autocomplete. The line
    ///   used to commit on blur and dispatch `on_submit` there, i.e. the program saw the verb and
    ///   never the typing (`📓️w13b-actions-search-pane-bodies.md` §6 gap 4).
    /// * `onSubmit` on Enter, `onAbort` on Escape — the guest's own descriptors, unaltered.
    /// * `onRepeatLast` on Space over an EMPTY line, bound only OUTSIDE a live session because
    ///   that is React's own branch: `applySearchSpaceAction` routes an empty line to
    ///   `onRepeatLast` when idle and to `onSubmit` during a session
    ///   (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10512`), and the retained line falls through to the
    ///   submit it did bind when no repeat is offered.
    pub fn build_window_search_ui(&self, window_id: &str, prefs: &PrefsState, windows: &WindowState) -> Option<UiNode> {
        let engagement = windows.window_engagements.get(window_id)?;
        let input = engagement.input.as_ref()?;
        let is_de = prefs.locale_id == "de";
        let draft = windows.engagement_inputs.get(window_id).cloned().or_else(|| input.value.clone()).unwrap_or_default();
        let placeholder = input.placeholder.clone().unwrap_or_else(|| shell_chrome_string(if engagement.session_active.unwrap_or(false) { "windowSearch.actionActive" } else { "windowSearch.action" }, is_de).to_string());
        let control_id = input.id.clone().filter(|id| !id.is_empty() && id != "search-input").unwrap_or_else(|| WINDOW_SEARCH_ACTION_CONTROL_ID.to_string());
        let session_active = engagement.session_active.unwrap_or(false);
        let mut children: Vec<UiNode> = vec![UiNode::Input(UiInputNode {
            id: control_id,
            input_kind: "text".into(),
            value: draft.clone(),
            placeholder: Some(Label::data(placeholder)),
            accessibility_label: None,
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            precision: None,
            snaps: Vec::new(),
            on_change: input.on_change.clone().unwrap_or_else(|| ActionDescriptor { controller_id: "framework".into(), action: "setEngagementInput".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "window": window_id.to_string() }) }),
            on_submit: input.on_submit.clone(),
            on_abort: input.on_abort.clone(),
            on_repeat_last: (!session_active).then(|| input.on_repeat_last.clone()).flatten(),
            presence: UiPresence { state: if input.disabled.unwrap_or(false) { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
            menu: None,
            ..Default::default()
        })];
        let possibles: &[ui_wgpu::wgpu::WindowEngagementPossible] = engagement.possible_engagements.as_deref().unwrap_or(&[]);
        if !possibles.is_empty() {
            let open = self.search_possibles_open.get(window_id).copied().unwrap_or(false);
            children.push(UiNode::Toggle(UiToggleNode {
                appearance: ui_contract::ToggleAppearance::Button,
                id: WINDOW_SEARCH_SUGGESTIONS_CONTROL_ID.into(),
                icon_id: if open { IconName::ChevronDown } else { IconName::ChevronRight },
                text: Some(Label::data(shell_chrome_string("windowSearch.suggestions", is_de))),
                on_change: ActionDescriptor { controller_id: "framework".into(), action: "toggleSearchPossibles".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "window": window_id.to_string() }) },
                presence: UiPresence { selected: open, ..UiPresence::default() },
                menu: None,
            }));
            if open {
                let matches = filter_search_possibles(&draft, possibles);
                if matches.is_empty() {
                    children.push(UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(shell_chrome_string("windowSearch.noMatches", is_de)), emphasize: Some(false), data_attributes: None, menu: None }));
                }
                for possible in matches {
                    let Some(action) = possible.action.clone() else { continue };
                    children.push(UiNode::Button(UiButtonNode {
                        id: Some(possible.id.clone()),
                        icon_id: IconName::CircleDot,
                        label: Label::data(match possible.detail.as_deref() {
                            Some(detail) if !detail.is_empty() => format!("{} — {detail}", normalize_engagement_action_text(&possible.label)),
                            _ => normalize_engagement_action_text(&possible.label),
                        }),
                        action,
                        style: None,
                        presence: UiPresence::default(),
                        menu: None,
                    }));
                }
            }
        }
        Some(UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("search".into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None }))
    }
}

//#endregion 🎬️WindowActionsAndSearchPanes
impl CommandState {
    pub fn stage_arg(&mut self, window_id: &str, action_id: &str, arg_id: &str, value: Value) {
        self.staged_action_args.entry(staged_key(window_id, action_id)).or_default().insert(arg_id.to_string(), value);
    }

    pub fn staged_map_for(&self, window_id: &str, action_id: &str) -> serde_json::Map<String, Value> {
        self.staged_action_args.get(&staged_key(window_id, action_id)).cloned().unwrap_or_default()
    }

    pub fn reset_staged_args(&mut self, window_id: &str, action_id: &str) {
        self.staged_action_args.remove(&staged_key(window_id, action_id));
    }

    /// 📝️ The effective value of one argument — staged if present, else the declared default: React's
    /// `effectiveActionArgs` per row (`🛠️ShellHelpers/🟦️.tsx`), which is what the Actions pane's staged
    /// form paints and what Execute merges.
    pub fn effective_arg_value(&self, window_id: &str, action_id: &str, arg: &semio_framework::ActionArgDef) -> Option<Value> {
        self.staged_action_args.get(&staged_key(window_id, action_id)).and_then(|map| map.get(&arg.id).cloned()).or_else(|| arg.default.as_ref().map(dsl_value_as_json))
    }
}

//#region 🔎️FuzzyRankingParity
/// 🔎️ The Rust twin of `🔨️modules/🖱️ui/🔨️modules/🔎️fuzzy-ranking/🟦️.ts` — the SAME ranking both shell
/// search surfaces use in React (`🔎️ShellSearch/🟦️.tsx`'s `UISearch`/`UIFind` both call `rankFuzzyItems`
/// with `label` ×2, `description` ×1, `category` ×0.5, threshold `0.4`, limit `20`).
///
/// It replaces this target's former hand-rolled `fuzzy_match_score` (a plain subsequence scorer with no
/// threshold, no per-field weights, no token splitting and no typo tolerance): identical queries ranked
/// differently in the two renderers, and a query matching nothing still returned every item in id order
/// rather than an empty palette. Ported branch for branch — prefix, substring, gapped-subsequence and
/// bounded Damerau–Levenshtein — so the constants below are the TypeScript constants, not new ones.
///
const FUZZY_TEXT_LIMIT: usize = 96;

pub const FUZZY_SEARCH_THRESHOLD: f64 = 0.4;

pub const FUZZY_SEARCH_LIMIT: usize = 20;

/// 🔎️ `normalizeFuzzyText` — lowercase, marks stripped, trimmed, bounded.
pub fn normalize_fuzzy_text(value: &str) -> String {
    ui_wgpu::wgpu::normalize_nfkd_text(value, false, FUZZY_TEXT_LIMIT)
}

/// 🔎️ `boundedDamerauLevenshtein` — edit distance with transpositions, abandoned past `maximum`.
fn bounded_damerau_levenshtein(left: &[char], right: &[char], maximum: usize) -> Option<usize> {
    if left.len().abs_diff(right.len()) > maximum {
        return None;
    }
    let width = right.len() + 1;
    let mut previous_previous = vec![0usize; width];
    let mut previous: Vec<usize> = (0..width).collect();
    let mut current = vec![0usize; width];
    for row in 1..=left.len() {
        current[0] = row;
        let mut row_minimum = row;
        let from = row.saturating_sub(maximum).max(1);
        let to = (row + maximum).min(right.len());
        for slot in current.iter_mut().take(from).skip(1) {
            *slot = maximum + 1;
        }
        for column in from..=to {
            let substitution = previous[column - 1] + usize::from(left[row - 1] != right[column - 1]);
            let mut distance = (previous[column] + 1).min(current[column - 1] + 1).min(substitution);
            if row > 1 && column > 1 && left[row - 1] == right[column - 2] && left[row - 2] == right[column - 1] {
                distance = distance.min(previous_previous[column - 2] + 1);
            }
            current[column] = distance;
            row_minimum = row_minimum.min(distance);
        }
        for slot in current.iter_mut().take(width).skip(to + 1) {
            *slot = maximum + 1;
        }
        if row_minimum > maximum {
            return None;
        }
        core::mem::swap(&mut previous_previous, &mut previous);
        core::mem::swap(&mut previous, &mut current);
    }
    (previous[right.len()] <= maximum).then_some(previous[right.len()])
}

/// 🔎️ `fuzzyTokenScore` — LOWER is better, `None` means "no match at all". Both already normalized.
pub fn fuzzy_token_score(query: &str, value: &str, threshold: f64) -> Option<f64> {
    if value.is_empty() {
        return None;
    }
    if value == query {
        return Some(0.0);
    }
    let query_chars: Vec<char> = query.chars().collect();
    let value_chars: Vec<char> = value.chars().collect();
    let query_len = query_chars.len() as f64;
    let value_len = value_chars.len() as f64;
    let extra_ratio = (value_len - query_len).max(0.0) / value_len.max(1.0);
    if value.starts_with(query) {
        return Some(0.04 + extra_ratio * 0.12);
    }
    if let Some(byte_index) = value.find(query) {
        let substring_index = value[..byte_index].chars().count() as f64;
        return Some(0.1 + (substring_index / value_len) * 0.12 + extra_ratio * 0.12);
    }
    let mut query_index = 0usize;
    let mut first_index: Option<usize> = None;
    let mut last_index = 0usize;
    for (value_index, candidate) in value_chars.iter().enumerate() {
        if query_index >= query_chars.len() {
            break;
        }
        if *candidate != query_chars[query_index] {
            continue;
        }
        first_index.get_or_insert(value_index);
        last_index = value_index;
        query_index += 1;
    }
    if query_index == query_chars.len() {
        let first_index = first_index.unwrap_or(0);
        let span = (last_index - first_index + 1) as f64;
        let gaps = (span - query_len).max(0.0);
        let score = 0.2 + (gaps / span.max(1.0)) * 0.18 + extra_ratio * 0.08 + (first_index as f64 / value_len) * 0.06;
        if score <= threshold {
            return Some(score);
        }
    }
    let maximum_distance = ((query_len * threshold) as usize).max(1);
    let mut best_distance: Option<usize> = None;
    for candidate in value.split(|character: char| !character.is_alphanumeric()) {
        if candidate.is_empty() {
            continue;
        }
        let candidate_chars: Vec<char> = candidate.chars().collect();
        if let Some(distance) = bounded_damerau_levenshtein(&query_chars, &candidate_chars, maximum_distance) {
            if best_distance.is_none_or(|current| distance < current) {
                best_distance = Some(distance);
            }
        }
    }
    let score = 0.18 + (best_distance? as f64 / query_len.max(1.0)) * 0.45;
    (score <= threshold).then_some(score)
}

/// 🔎️ `rankFuzzyItems` — weighted multi-field, multi-token ranking; ties break on declaration order.
/// An empty query keeps the declared order (React's `items.slice(0, limit)`), so the palette's cold list
/// is never reshuffled by the ranker.
pub fn rank_fuzzy_items<Item>(items: Vec<Item>, query: &str, fields: &[(fn(&Item) -> Option<&str>, f64)], threshold: f64, limit: usize) -> Vec<Item> {
    if limit == 0 {
        return Vec::new();
    }
    let normalized_query = normalize_fuzzy_text(query);
    if normalized_query.is_empty() {
        let mut items = items;
        items.truncate(limit);
        return items;
    }
    let tokens: Vec<String> = normalized_query.split_whitespace().map(str::to_string).collect();
    let maximum_weight = fields.iter().fold(1.0f64, |best, (_, weight)| best.max(weight.max(0.0)));
    let mut ranked: Vec<(f64, usize, Item)> = Vec::new();
    for (ref_index, item) in items.into_iter().enumerate() {
        let values: Vec<(String, f64)> = fields.iter().map(|(read, weight)| (normalize_fuzzy_text(read(&item).unwrap_or_default()), weight.max(0.0))).collect();
        let mut score = 0.0;
        let mut matched = true;
        for token in &tokens {
            let mut token_score: Option<f64> = None;
            for (value, weight) in &values {
                let Some(field_score) = fuzzy_token_score(token, value, threshold) else {
                    continue;
                };
                let weighted = field_score + ((maximum_weight - weight) / maximum_weight) * 0.08;
                if token_score.is_none_or(|best| weighted < best) {
                    token_score = Some(weighted);
                }
            }
            match token_score {
                Some(value) if value <= threshold => score += value,
                _ => {
                    matched = false;
                    break;
                }
            }
        }
        if matched {
            ranked.push((score / tokens.len() as f64, ref_index, item));
        }
    }
    ranked.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap_or(core::cmp::Ordering::Equal).then(left.1.cmp(&right.1)));
    ranked.into_iter().take(limit).map(|(_, _, item)| item).collect()
}

//#endregion 🔎️FuzzyRankingParity
//#endregion ActionPanelAndUtilities

/// 🧮️ Resolves required presence and current catalog membership before native staged execution.
pub fn resolved_execute_args(defs: &[semio_framework::ActionArgDef], staged: &serde_json::Map<String, Value>) -> Option<serde_json::Map<String, Value>> {
    let staged_dsl = semio_framework_value::DslValue::from(Value::Object(staged.clone()));
    let effective = semio_framework::effective_action_args(defs, &staged_dsl, None);
    if semio_framework::unresolved_action_args(defs, &effective).is_empty() {
        Value::from(&effective).as_object().cloned()
    } else {
        None
    }
}

// #endregion
// #region staging
pub fn staged_key(window_id: &str, action_id: &str) -> String {
    format!("{window_id}:{action_id}")
}
