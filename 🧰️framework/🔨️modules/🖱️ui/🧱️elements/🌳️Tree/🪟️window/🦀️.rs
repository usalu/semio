//! 🪟️ The tree window request rule — the Rust twin of `🌳️Tree/🟦️.tsx` region `🪟️TreeWindow`
//! (`treeWindowVisibleRowsForViewport`, `treeWindowRequestsForViewport`, `capTreeWindowRequests`) and of the React observer's
//! body/served rules (`treeWindowBodyRequestsV1`, `treeWindowServedRequestsV1`, `treeWindowReportSignatureV1`,
//! `📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`). One rule, two hosts: both read the same neutral vectors
//! (`🧫️fixtures/🪟️window-requests/🔣️.json`, `🧬️contract/🧫️fixtures/🪟️tree-window-served.json`), so React and wgpu request the same
//! windows for the same viewport. Pure: no layout, no DOM, no retained tree — a host measures, this answers.
//! @see 🎫️ 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING · 📓️design-virtualised-tree.md §6.1, §7

/// 🪟️ Rows requested beyond each edge of the viewport (`TREE_WINDOW_OVERSCAN_ROWS`, `🌳️Tree/🟦️.tsx`).
pub const TREE_WINDOW_OVERSCAN_ROWS: u32 = 8;

/// 🪟️ Hard ceiling on one window request (`TREE_WINDOW_ROWS_MAX`, `🌳️Tree/🟦️.tsx`).
pub const TREE_WINDOW_ROWS_MAX: u32 = 128;

/// 🪟️ Rows an off-screen container that already materialised something is asked for (`TREE_WINDOW_OFFSCREEN_ROWS`).
pub const TREE_WINDOW_OFFSCREEN_ROWS: u32 = 0;

/// 🌱️ Rows an off-screen container that materialised nothing yet is asked for (`TREE_WINDOW_OFFSCREEN_SEED_ROWS`).
pub const TREE_WINDOW_OFFSCREEN_SEED_ROWS: u32 = 1;

/// 📐️ One MATERIALISED row of a windowed container: its entry index and its top, in the container's own space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TreeWindowRowMeasure {
    pub index: u32,
    pub top: f64,
}

/// 📐️ One windowed container as a host measured it (`TreeWindowContainerMeasure`): its window path, its window, its full
/// virtual extent in viewport space (content origin at 0), the pitch of one closed row, and its materialised rows' tops.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeWindowContainerMeasure {
    pub key: String,
    pub total: u32,
    pub offset: u32,
    pub length: u32,
    pub top: f64,
    pub height: f64,
    pub row_px: f64,
    pub rows: Vec<TreeWindowRowMeasure>,
}

/// 🪟️ One container's next window: materialise `rows` entries starting at `offset`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeWindowRequest {
    pub key: String,
    pub offset: u32,
    pub rows: u32,
}

/// 📐️ What one on-screen container contributes to the viewport rule (`TreeWindowVisibleRows`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TreeWindowVisibleRows {
    pub total: u32,
    pub visible_rows: u32,
    pub first_visible_row: u32,
    pub distance_px: f64,
}

/// ⚓️ Which viewport edge a short window is pinned to (`TreeWindowServedMemoryV1.anchor`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeWindowAnchor {
    Start,
    End,
}

/// 🧠️ What the observer keeps of one windowed container between two measures (`TreeWindowServedMemoryV1`).
#[derive(Clone, Debug, PartialEq)]
pub struct TreeWindowServedMemory {
    pub asked: TreeWindowRequest,
    pub capacity: Option<u32>,
    pub anchor: TreeWindowAnchor,
    pub first_visible: Option<u32>,
    pub total: u32,
}

/// 🗂️ Insertion-ordered key → value table with last-write-wins, the JS `Map` the TypeScript rule builds.
fn put<T>(table: &mut Vec<(String, T)>, key: &str, value: T) {
    match table.iter_mut().find(|(existing, _)| existing == key) {
        Some(slot) => slot.1 = value,
        None => table.push((key.to_owned(), value)),
    }
}

fn get<'a, T>(table: &'a [(String, T)], key: &str) -> Option<&'a T> {
    table.iter().find(|(existing, _)| existing == key).map(|(_, value)| value)
}

/// 📐️ The child row of `container` drawn at pixel `y` (`treeWindowRowIndexAt`): the materialised band reads the rows' real
/// tops, the two spacer bands the uniform pitch.
fn row_index_at(container: &TreeWindowContainerMeasure, y: f64, total: u32, offset: u32, length: u32) -> u32 {
    let clamp = |index: f64| index.max(0.0).min(f64::from(total) - 1.0) as u32;
    let local = y - container.top;
    let pitch = container.row_px;
    let Some(first) = container.rows.first() else { return clamp((local / pitch).floor()) };
    if y < first.top {
        return offset.min(clamp((local / pitch).floor()));
    }
    let trailing = f64::from(total - offset - length) * pitch;
    let rows_end = container.top + container.height.max(0.0) - trailing;
    if y >= rows_end {
        return clamp(f64::from(offset + length) + ((y - rows_end) / pitch).floor());
    }
    let (mut low, mut high) = (0usize, container.rows.len() - 1);
    while low < high {
        let middle = (low + high).div_ceil(2);
        if container.rows[middle].top <= y {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    clamp(f64::from(container.rows[low].index))
}

/// 📐️ Every container measured against the viewport once (`treeWindowVisibleRowsForViewport`); a container off the
/// viewport is absent.
pub fn visible_rows_for_viewport(containers: &[TreeWindowContainerMeasure], viewport_top: f64, viewport_height: f64) -> Vec<(String, TreeWindowVisibleRows)> {
    let mut visible = Vec::new();
    let height = viewport_height.max(0.0);
    let viewport_bottom = viewport_top + height;
    let viewport_centre = viewport_top + height / 2.0;
    for container in containers {
        let total = container.total;
        if total == 0 {
            continue;
        }
        let extent = container.height.max(0.0);
        if (container.top + extent).min(viewport_bottom) - container.top.max(viewport_top) <= 0.0 {
            continue;
        }
        let offset = container.offset.min(total);
        let length = container.length.min(total - offset);
        let first_visible_row = row_index_at(container, container.top.max(viewport_top), total, offset, length);
        let last_visible_row = row_index_at(container, (container.top + extent).min(viewport_bottom) - 1.0, total, offset, length);
        let visible_rows = (i64::from(last_visible_row) - i64::from(first_visible_row) + 1).max(1) as u32;
        put(&mut visible, &container.key, TreeWindowVisibleRows { total, visible_rows, first_visible_row, distance_px: (container.top + extent / 2.0 - viewport_centre).abs() });
    }
    visible
}

/// 🪟️ One container's window at an EXACT row count (`treeWindowRequestAtRows`).
fn request_at_rows(key: &str, metrics: &TreeWindowVisibleRows, rows: u32, overscan: u32) -> TreeWindowRequest {
    let bounded = rows.min(metrics.total).max(1);
    TreeWindowRequest { key: key.to_owned(), offset: metrics.first_visible_row.saturating_sub(overscan).min(metrics.total - bounded), rows: bounded }
}

/// 🪟️ The windows the viewport wants for every container it intersects (`treeWindowRequestsForViewport`).
pub fn requests_for_viewport(containers: &[TreeWindowContainerMeasure], viewport_top: f64, viewport_height: f64, overscan: u32) -> Vec<TreeWindowRequest> {
    let visible = visible_rows_for_viewport(containers, viewport_top, viewport_height);
    containers.iter().filter_map(|container| get(&visible, &container.key).map(|metrics| request_at_rows(&container.key, metrics, (metrics.visible_rows + 2 * overscan).min(TREE_WINDOW_ROWS_MAX), overscan))).collect()
}

/// 🪟️ Fits a whole body's request inside `budget` in the guest's cost model — every container `1 + rows` —
/// overscan first, then rows furthest from the viewport centre (`capTreeWindowRequests`).
pub fn cap_requests(requests: Vec<TreeWindowRequest>, visible: &[(String, TreeWindowVisibleRows)], budget: usize) -> Vec<TreeWindowRequest> {
    let cost = |rows: &mut dyn Iterator<Item = u32>| -> usize { rows.fold(0usize, |carry, rows| carry + 1 + rows as usize) };
    if cost(&mut requests.iter().map(|request| request.rows)) <= budget {
        return requests;
    }
    struct Entry {
        index: usize,
        key: String,
        ceiling: u32,
        rows: u32,
        overscan: u32,
        metrics: TreeWindowVisibleRows,
    }
    let mut entries: Vec<Entry> = requests
        .iter()
        .enumerate()
        .map(|(index, request)| Entry {
            index,
            key: request.key.clone(),
            ceiling: request.rows,
            rows: request.rows,
            overscan: TREE_WINDOW_OVERSCAN_ROWS,
            metrics: get(visible, &request.key).copied().unwrap_or(TreeWindowVisibleRows { total: request.offset + request.rows, visible_rows: request.rows, first_visible_row: request.offset, distance_px: f64::INFINITY }),
        })
        .collect();
    for overscan in (0..TREE_WINDOW_OVERSCAN_ROWS).rev() {
        for entry in &mut entries {
            let shrunk = (entry.metrics.visible_rows + 2 * overscan).max(1);
            entry.rows = entry.ceiling.min(shrunk);
            entry.overscan = if shrunk < entry.ceiling { overscan } else { TREE_WINDOW_OVERSCAN_ROWS };
        }
        if cost(&mut entries.iter().map(|entry| entry.rows)) <= budget {
            break;
        }
    }
    let mut excess = cost(&mut entries.iter().map(|entry| entry.rows)) as i64 - budget as i64;
    let mut furthest_first: Vec<usize> = (0..entries.len()).collect();
    furthest_first.sort_by(|left, right| {
        let (left, right) = (&entries[*left], &entries[*right]);
        right.metrics.distance_px.partial_cmp(&left.metrics.distance_px).unwrap_or(std::cmp::Ordering::Equal).then(right.index.cmp(&left.index))
    });
    for &at in &furthest_first {
        if excess <= 0 {
            break;
        }
        let given = excess.min(i64::from(entries[at].rows) - 1);
        if given <= 0 {
            continue;
        }
        entries[at].rows -= given as u32;
        entries[at].overscan = 0;
        excess -= given;
    }
    for &at in &furthest_first {
        if excess <= 0 {
            break;
        }
        if entries[at].rows == 0 {
            continue;
        }
        entries[at].rows = 0;
        excess -= 1;
    }
    entries
        .iter()
        .map(|entry| {
            if entry.rows > 0 {
                request_at_rows(&entry.key, &entry.metrics, entry.rows, entry.overscan)
            } else {
                TreeWindowRequest { key: entry.key.clone(), offset: entry.metrics.first_visible_row.min(entry.metrics.total.saturating_sub(1)), rows: 0 }
            }
        })
        .collect()
}

/// 🪟️ Every windowed container of one body priced together (`treeWindowBodyRequestsV1`): on-screen ones with the rows they
/// need, off-screen ones as spacer windows at the offset they hold (one seed row if they hold nothing), then capped.
pub fn body_requests(containers: &[TreeWindowContainerMeasure], viewport_height: f64, budget: usize) -> Vec<TreeWindowRequest> {
    let visible = visible_rows_for_viewport(containers, 0.0, viewport_height);
    let mut wanted = Vec::new();
    for request in requests_for_viewport(containers, 0.0, viewport_height, TREE_WINDOW_OVERSCAN_ROWS) {
        let key = request.key.clone();
        put(&mut wanted, &key, request);
    }
    let requests = containers
        .iter()
        .map(|container| match get(&wanted, &container.key) {
            Some(known) => known.clone(),
            None => {
                let total = container.total;
                let rows = if container.length > 0 { TREE_WINDOW_OFFSCREEN_ROWS } else { TREE_WINDOW_OFFSCREEN_SEED_ROWS.min(total) };
                TreeWindowRequest { key: container.key.clone(), offset: container.offset.min(total.saturating_sub(rows)), rows }
            }
        })
        .collect();
    cap_requests(requests, &visible, budget)
}

/// 🧠️ [`body_requests`] spent where the guest can actually answer it (`treeWindowServedRequestsV1`): a window answered
/// SHORT teaches the container's capacity, and from then on the window is placed for that capacity — centred on the visible
/// rows when it covers them, else pinned to the edge the reader scrolls towards — so every row stays reachable.
pub fn served_requests(containers: &[TreeWindowContainerMeasure], viewport_height: f64, memory: &[(String, TreeWindowServedMemory)], budget: usize) -> (Vec<TreeWindowRequest>, Vec<(String, TreeWindowServedMemory)>) {
    let visible = visible_rows_for_viewport(containers, 0.0, viewport_height);
    let mut next = Vec::new();
    let requests = body_requests(containers, viewport_height, budget)
        .into_iter()
        .map(|request| {
            let container = containers.iter().rev().find(|container| container.key == request.key);
            let total = container.map_or(0, |container| container.total);
            let offset = container.map_or(0, |container| container.offset);
            let length = container.map_or(0, |container| container.length);
            let known = get(memory, &request.key);
            let priced = known.filter(|known| known.total == total);
            let short = priced.is_some_and(|priced| length > 0 && offset == priced.asked.offset && length < priced.asked.rows.min(total.saturating_sub(offset)));
            let capacity = if short { Some(length) } else { priced.and_then(|priced| priced.capacity).map(|capacity| capacity.max(length)) };
            let metrics = get(&visible, &request.key).copied();
            let first_visible = metrics.map(|metrics| metrics.first_visible_row);
            let last_visible = metrics.map(|metrics| metrics.first_visible_row + metrics.visible_rows - 1);
            let previous = known.and_then(|known| known.first_visible);
            let anchor = match (first_visible, last_visible, previous) {
                (Some(first), Some(last), None) => {
                    if first > 0 && last + 1 >= total {
                        TreeWindowAnchor::End
                    } else {
                        TreeWindowAnchor::Start
                    }
                }
                (Some(first), Some(_), Some(previous)) if first > previous => TreeWindowAnchor::End,
                (Some(first), Some(_), Some(previous)) if first < previous => TreeWindowAnchor::Start,
                (Some(_), Some(_), Some(_)) => known.map_or(TreeWindowAnchor::Start, |known| known.anchor),
                _ => known.map_or(TreeWindowAnchor::Start, |known| known.anchor),
            };
            let asked = match (capacity, metrics, last_visible) {
                (Some(capacity), Some(metrics), Some(last_visible)) if request.rows > capacity => {
                    let start = if capacity >= metrics.visible_rows {
                        i64::from(metrics.first_visible_row) - i64::from((capacity - metrics.visible_rows) / 2)
                    } else if anchor == TreeWindowAnchor::End {
                        i64::from(last_visible) - i64::from(capacity) + 1
                    } else {
                        i64::from(metrics.first_visible_row)
                    };
                    let placed = start.max(0).min(i64::from(total.saturating_sub(capacity))) as u32;
                    TreeWindowRequest { key: request.key.clone(), offset: placed, rows: request.rows.min(total - placed).max(1) }
                }
                _ => request,
            };
            put(&mut next, &asked.key.clone(), TreeWindowServedMemory { asked: asked.clone(), capacity, anchor, first_visible: first_visible.or(previous), total });
            asked
        })
        .collect();
    (requests, next)
}

/// 🪟️ The one line a report is compared on (`treeWindowReportSignatureV1`): the host refreshes only when the ANSWER changed.
pub fn report_signature(requests: &[TreeWindowRequest], viewport_rows: u32) -> String {
    let body: Vec<String> = requests.iter().map(|request| format!("{}:{}:{}", request.key, request.offset, request.rows)).collect();
    format!("{viewport_rows}|{}", body.join(","))
}

#[cfg(test)]
#[path = "../🧪️tests/🪟️window/🦀️.rs"]
mod tests;
