//! 📏 Exact logical-edge arbitration, matching the shared border-presentation segment schema.

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BorderState {
    Normal,
    Focus,
    Active,
    Waiting,
    Loading,
    Introduced,
    Celebrated,
    Hover,
}

impl BorderState {
    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "normal" => Self::Normal,
            "focus" => Self::Focus,
            "active" => Self::Active,
            "waiting" => Self::Waiting,
            "loading" => Self::Loading,
            "introduced" => Self::Introduced,
            "celebrated" => Self::Celebrated,
            "hover" => Self::Hover,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BorderSegment {
    pub id: String,
    pub owner: String,
    pub from: BorderPoint,
    pub to: BorderPoint,
    pub state: BorderState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedBorderSegment {
    pub segment: BorderSegment,
    pub contributors: Vec<String>,
}

/// 🧮 Splits collinear intervals at every endpoint; higher state then lexical owner/id wins.
pub fn resolve_border_segments(input: &[BorderSegment]) -> Result<Vec<ResolvedBorderSegment>, &'static str> {
    let mut ids = std::collections::HashSet::new();
    let mut normalized = Vec::with_capacity(input.len());
    for segment in input {
        if !ids.insert(&segment.id) {
            return Err("duplicate border id");
        }
        let (a, b) = (segment.from, segment.to);
        if ![a.x, a.y, b.x, b.y].iter().all(|value| value.is_finite()) {
            return Err("nonfinite border coordinate");
        }
        if a.x != b.x && a.y != b.y {
            return Err("diagonal border");
        }
        if a == b {
            continue;
        }
        let horizontal = a.y == b.y;
        let (line, start, end) = if horizontal { (a.y, a.x.min(b.x), a.x.max(b.x)) } else { (a.x, a.y.min(b.y), a.y.max(b.y)) };
        normalized.push((!horizontal, line, start, end, segment));
    }
    normalized.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)).then(a.2.total_cmp(&b.2)));
    let mut output = Vec::new();
    let mut offset = 0;
    while offset < normalized.len() {
        let (vertical, line, _, _, _) = normalized[offset];
        let end = offset + normalized[offset..].iter().take_while(|item| item.0 == vertical && item.1 == line).count();
        let group = &normalized[offset..end];
        let mut endpoints = group.iter().flat_map(|item| [item.2, item.3]).collect::<Vec<_>>();
        endpoints.sort_by(f64::total_cmp);
        endpoints.dedup();
        for pair in endpoints.windows(2) {
            let mut covering = group.iter().filter(|item| item.2 <= pair[0] && item.3 >= pair[1]).map(|item| item.4).collect::<Vec<_>>();
            covering.sort_by(|a, b| b.state.cmp(&a.state).then(a.owner.cmp(&b.owner)).then(a.id.cmp(&b.id)));
            let Some(winner) = covering.first() else { continue };
            let mut segment = (*winner).clone();
            segment.from = if vertical { BorderPoint { x: line, y: pair[0] } } else { BorderPoint { x: pair[0], y: line } };
            segment.to = if vertical { BorderPoint { x: line, y: pair[1] } } else { BorderPoint { x: pair[1], y: line } };
            let mut contributors = covering.iter().map(|segment| segment.id.clone()).collect::<Vec<_>>();
            contributors.sort();
            output.push(ResolvedBorderSegment { segment, contributors });
        }
        offset = end;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edge(id: &str, owner: &str, a: (f64, f64), b: (f64, f64), state: BorderState) -> BorderSegment {
        BorderSegment { id: id.into(), owner: owner.into(), from: BorderPoint { x: a.0, y: a.1 }, to: BorderPoint { x: b.0, y: b.1 }, state }
    }

    #[test]
    fn partial_hover_keeps_active_outline_and_is_order_independent() {
        let mut input = vec![edge("window", "w", (0.0, 0.0), (10.0, 0.0), BorderState::Active), edge("button", "b", (7.0, 0.0), (3.0, 0.0), BorderState::Hover), edge("side", "w", (10.0, 0.0), (10.0, 5.0), BorderState::Active)];
        let result = resolve_border_segments(&input).unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result.iter().map(|edge| edge.segment.id.as_str()).collect::<Vec<_>>(), ["window", "button", "window", "side"]);
        input.reverse();
        assert_eq!(result, resolve_border_segments(&input).unwrap());
    }

    #[test]
    fn invalid_coordinates_and_duplicate_ids_are_rejected() {
        let a = edge("a", "w", (0.0, 0.0), (1.0, 0.0), BorderState::Normal);
        assert!(resolve_border_segments(&[a.clone(), a]).is_err());
        assert!(resolve_border_segments(&[edge("a", "w", (0.0, 0.0), (1.0, 1.0), BorderState::Normal)]).is_err());
        assert!(resolve_border_segments(&[edge("a", "w", (f64::NAN, 0.0), (1.0, 0.0), BorderState::Normal)]).is_err());
        assert!(resolve_border_segments(&[edge("a", "w", (1.0, 1.0), (1.0, 1.0), BorderState::Normal)]).unwrap().is_empty());
    }
}
