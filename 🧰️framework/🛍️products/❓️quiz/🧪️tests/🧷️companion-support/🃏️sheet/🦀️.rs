//! 🃏️ The shared quiz builders every companion law builds its sheets and runs from.

use quiz_schema::{Axis, Category, ClassificationItem, ClassificationTask, Dimension, MatchingItem, MatchingTask, Quantity, Quiz, Scale, SortingItem, SortingTask, Task, Text};
use std::collections::BTreeMap;

pub fn text(value: &str) -> Text {
    Text { en: value.to_string(), de: format!("{value} (de)") }
}

pub fn quantity(unit: &str, scale: Scale) -> Quantity {
    Quantity { label: text(unit), short: None, unit: unit.to_string(), scale, prefixed: false, additive: false }
}

pub fn classification(id: &str, draw: Option<usize>) -> ClassificationTask {
    let axis = |id: &str| Axis { id: id.to_string(), label: text(id), short: None, unit: "u".to_string(), min: 0.0, max: 10.0 };
    let category = |id: &str, profile: [f64; 3]| Category { id: id.to_string(), label: text(id), short: None, icon: None, description: None, profile: Some(BTreeMap::from([("heat".to_string(), profile[0]), ("cool".to_string(), profile[1]), ("cost".to_string(), profile[2])])) };
    let item = |id: &str, category: &str| ClassificationItem { id: id.to_string(), label: text(id), short: None, icon: None, category: category.to_string(), explanation: Some(text(&format!("{id} is {category}"))) };
    ClassificationTask {
        id: id.to_string(),
        title: text(id),
        prompt: text("classify"),
        icon: None,
        axes: Some(vec![axis("heat"), axis("cool"), axis("cost")]),
        categories: vec![category("passive", [1.0, 2.0, 9.0]), category("low", [3.0, 3.0, 6.0]), category("old", [10.0, 6.0, 1.0])],
        items: vec![item("a", "passive"), item("b", "low"), item("c", "old"), item("d", "low")],
        draw,
    }
}

pub fn sorting(id: &str, values: &[f64], scale: Scale, draw: Option<usize>) -> SortingTask {
    SortingTask {
        id: id.to_string(),
        title: text(id),
        prompt: text("sort"),
        icon: None,
        quantity: quantity("W", scale),
        items: values.iter().enumerate().map(|(index, &value)| SortingItem { id: format!("s{index}"), label: text(&format!("s{index}")), short: None, icon: None, value, familiar: None, explanation: None }).collect(),
        draw,
    }
}

pub fn matching(id: &str, rows: &[(f64, f64)], draw: Option<usize>) -> MatchingTask {
    MatchingTask {
        id: id.to_string(),
        title: text(id),
        prompt: text("match"),
        icon: None,
        dimensions: vec![Dimension { id: "load".to_string(), quantity: quantity("W/m²", Scale::Linear), icon: None }, Dimension { id: "demand".to_string(), quantity: quantity("kWh/(m²a)", Scale::Logarithmic), icon: None }],
        items: rows.iter().enumerate().map(|(index, &(load, demand))| MatchingItem { id: format!("m{index}"), label: text(&format!("m{index}")), short: None, icon: None, values: BTreeMap::from([("load".to_string(), load), ("demand".to_string(), demand)]), familiar: None, explanation: None }).collect(),
        draw,
    }
}

pub fn quiz() -> Quiz {
    Quiz {
        json_schema: None,
        schema: "semio.quiz/v1".to_string(),
        id: "energy".to_string(),
        emoji: "⚡".to_string(),
        title: text("Energy"),
        short: None,
        description: text("About energy"),
        tasks: vec![
            Task::Classification(classification("standards", None)),
            Task::Sorting(sorting("power", &[1.0, 60.0, 2000.0, 1.0e6, 1.0e9], Scale::Logarithmic, Some(4))),
            Task::Matching(matching("buildings", &[(10.0, 15.0), (40.0, 90.0), (120.0, 250.0)], None)),
        ],
    }
}
