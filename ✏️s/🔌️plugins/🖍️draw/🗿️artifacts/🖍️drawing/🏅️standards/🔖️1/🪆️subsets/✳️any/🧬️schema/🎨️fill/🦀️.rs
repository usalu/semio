//! 🎨️ Typed fill editing for the inspector and canvas gradient handles.
use crate::{FillStyle, GradientStop};
use semio_framework_value::list::PagedList;
static EMPTY_STOPS: std::sync::LazyLock<PagedList<GradientStop, {usize::MAX}>> = std::sync::LazyLock::new(Default::default);
#[path="🎨️sampling/🦀️.rs"]
pub mod sampling;

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
pub enum FillEdit {
    Type { value: FillType },
    Coordinate { axis: FillAxis, value: f64 },
    Color { index: Option<usize>, value: [f64; 4] },
    Alpha { index: Option<usize>, value: f64 },
    Offset { index: usize, value: f64 },
    AddStop { offset: f64 },
    RemoveStop { index: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum FillType { None, Solid, LinearGradient, RadialGradient }

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum FillAxis { X1, Y1, X2, Y2, Cx, Cy, R }

pub fn stops(fill: &FillStyle) -> &PagedList<GradientStop, {usize::MAX}> {
    match fill { FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => stops, _ => &EMPTY_STOPS }
}

fn stops_mut(fill: &mut FillStyle) -> Result<&mut PagedList<GradientStop, {usize::MAX}>, &'static str> {
    match fill { FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => Ok(stops), _ => Err("Select a gradient fill") }
}

fn sort_stops(stops: &mut PagedList<GradientStop, {usize::MAX}>) {
    for index in 1..stops.len() {
        let mut current = index;
        while current > 0 && stops[current].offset.total_cmp(&stops[current - 1].offset).is_lt() { stops.swap(current, current - 1); current -= 1; }
    }
}

fn color_mut(fill: &mut FillStyle, index: Option<usize>) -> Result<&mut [f64;4], &'static str> {
    match (fill,index) {
        (FillStyle::Solid { color },None) => Ok(color),
        (fill,Some(index)) => stops_mut(fill)?.get_mut(index).map(|stop| &mut stop.color).ok_or("Missing gradient stop"),
        _ => Err("Select a gradient stop"),
    }
}

pub fn edit_fill(source: Option<&FillStyle>, edit: &FillEdit) -> Result<Option<FillStyle>, &'static str> {
    if let Some(fill) = source {
        if stops(fill).len() > 64 { return Err("This fill exceeds the interactive stop limit"); }
        if stops(fill).iter().any(|stop| !stop.offset.is_finite() || !(0.0..=1.0).contains(&stop.offset) || stop.color.iter().any(|value| !value.is_finite() || !(0.0..=1.0).contains(value))) { return Err("Invalid gradient stop"); }
    }
    if let FillEdit::Type { value } = edit {
        let color = match source { Some(FillStyle::Solid { color }) => *color, Some(fill) => stops(fill).first().map_or([0.0,0.0,0.0,1.0], |stop| stop.color), None => [0.0,0.0,0.0,1.0] };
        let stops = source.map(stops).filter(|stops| !stops.is_empty()).map_or_else(|| [GradientStop { offset:0.0,color }, GradientStop { offset:1.0,color:[1.0,1.0,1.0,color[3]] }].into_iter().collect(), Clone::clone);
        return Ok(match value {
            FillType::None => None,
            FillType::Solid => Some(FillStyle::Solid { color }),
            FillType::LinearGradient => Some(match source { Some(fill @ FillStyle::LinearGradient { .. }) => fill.clone(), _ => FillStyle::LinearGradient { x1:0.0,y1:0.0,x2:100.0,y2:0.0,stops } }),
            FillType::RadialGradient => Some(match source { Some(fill @ FillStyle::RadialGradient { .. }) => fill.clone(), _ => FillStyle::RadialGradient { cx:50.0,cy:50.0,r:50.0,stops } }),
        });
    }
    let mut fill = source.cloned().ok_or("Enable a fill first")?;
    match edit {
        FillEdit::Type { .. } => unreachable!(),
        FillEdit::Coordinate { axis,value } => {
            if !value.is_finite() || (*axis == FillAxis::R && *value <= 0.0) { return Err("Invalid gradient coordinate"); }
            let target = match (&mut fill,axis) {
                (FillStyle::LinearGradient { x1,.. },FillAxis::X1) => x1,
                (FillStyle::LinearGradient { y1,.. },FillAxis::Y1) => y1,
                (FillStyle::LinearGradient { x2,.. },FillAxis::X2) => x2,
                (FillStyle::LinearGradient { y2,.. },FillAxis::Y2) => y2,
                (FillStyle::RadialGradient { cx,.. },FillAxis::Cx) => cx,
                (FillStyle::RadialGradient { cy,.. },FillAxis::Cy) => cy,
                (FillStyle::RadialGradient { r,.. },FillAxis::R) => r,
                _ => return Err("Coordinate does not apply to this fill"),
            };
            *target = *value;
        }
        FillEdit::Color { index,value } => {
            if value.iter().any(|component|!component.is_finite()||!(0.0..=1.0).contains(component)){return Err("Invalid color components");}
            *color_mut(&mut fill,*index)? = *value;
        }
        FillEdit::Alpha { index,value } => {
            if !value.is_finite() || !(0.0..=1.0).contains(value) { return Err("Opacity must be between zero and one"); }
            color_mut(&mut fill,*index)?[3] = *value;
        }
        FillEdit::Offset { index,value } => {
            if !value.is_finite() || !(0.0..=1.0).contains(value) { return Err("Stop position must be between zero and one"); }
            let stops = stops_mut(&mut fill)?;
            stops.get_mut(*index).ok_or("Missing gradient stop")?.offset = *value;
            sort_stops(stops);
        }
        FillEdit::AddStop { offset } => {
            if !offset.is_finite() || !(0.0..=1.0).contains(offset) { return Err("Stop position must be between zero and one"); }
            let stops = stops_mut(&mut fill)?;
            if stops.len() >= 64 || stops.is_empty() { return Err("Cannot add another gradient stop"); }
            sort_stops(stops);
            let right = stops.iter().take_while(|stop| stop.offset <= *offset).count();
            let color = sampling::GradientRamp::new(stops)?.sample(*offset)?;
            stops.insert(right,GradientStop { offset:*offset,color });
        }
        FillEdit::RemoveStop { index } => {
            let stops = stops_mut(&mut fill)?;
            if stops.len() <= 2 || *index >= stops.len() { return Err("A gradient needs at least two stops"); }
            stops.remove(*index);
        }
    }
    Ok(Some(fill))
}

#[cfg(test)]
mod tests {
    #[test]
    fn fill_edit_fixtures() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let source: Option<crate::FillStyle> = serde_json::from_value(case["before"].clone()).unwrap();
            let edit = serde_json::from_value(case["edit"].clone()).unwrap();
            let result = super::edit_fill(source.as_ref(),&edit);
            if case["error"] == true { assert!(result.is_err(),"{}",case["name"]); }
            else { assert_eq!(result.unwrap(),serde_json::from_value(case["after"].clone()).unwrap(),"{}",case["name"]); }
        }
        eprintln!("[TRACE] typed fill edits matched the shared gradient fixtures");
    }
}
