//! 🔀️ Guarded field edits; composition preserves every intermediate precondition.
use crate::ChartSnapshot;
use semio_framework_value::{DslValue,FromValue,ToValue};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff, DiffAlgebra, DiffRegions, TouchedPaths};

#[derive(Clone, Debug, PartialEq)]
pub struct ChartEdit {
    pub path: Vec<String>,
    pub before: Option<DslValue>,
    pub after: Option<DslValue>,
}

impl ToValue for ChartEdit {
    fn to_value(&self) -> DslValue {
        let mut fields = vec![("path".into(),self.path.to_value())];
        if let Some(value)=&self.before { fields.push(("before".into(),value.clone())); }
        if let Some(value)=&self.after { fields.push(("after".into(),value.clone())); }
        DslValue::object(fields)
    }
}
impl FromValue for ChartEdit {
    fn from_value(value:DslValue)->Result<Self,semio_framework_value::ValueError>{
        let DslValue::Object(fields)=value else{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"chart edit must be an object"));};
        let mut path=None;let mut before=None;let mut after=None;
        for(key,value)in fields{match key.as_str(){"path"=>path=Some(Vec::<String>::from_value(value)?),"before"=>before=Some(value),"after"=>after=Some(value),_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown chart edit field {key}"))),}}
        Ok(Self{path:path.ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"chart edit path is required"))?,before,after})
    }
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
pub struct ChartDiff {
    pub edits: Vec<ChartEdit>,
}

pub fn chart_values_equal(left:&DslValue,right:&DslValue)->bool{
    match(left,right){
        (DslValue::Object(a),DslValue::Object(b))=>a.len()==b.len()&&a.iter().all(|(key,value)|b.iter().find(|(other,_)|other==key).is_some_and(|(_,other)|chart_values_equal(value,other))),
        (DslValue::Array(a),DslValue::Array(b))=>a.len()==b.len()&&a.iter().zip(b).all(|(a,b)|chart_values_equal(a,b)),
        (DslValue::Number(a),DslValue::Number(b))=>a.as_f64()==b.as_f64(),
        _=>left==right,
    }
}
fn presence_equal(left:Option<&DslValue>,right:Option<&DslValue>)->bool{match(left,right){(Some(a),Some(b))=>chart_values_equal(a,b),(None,None)=>true,_=>false}}

pub fn read_path<'a>(root: &'a DslValue, path: &[String]) -> Option<&'a DslValue> {
    let mut value = root;
    for segment in path {
        value = match value {
            DslValue::Object(entries) => entries.iter().find(|(key, _)| key == segment).map(|(_, value)| value)?,
            DslValue::Array(items) => items.get(array_index(segment)?)?,
            _ => return None,
        };
    }
    Some(value)
}

fn array_index(segment: &str) -> Option<usize> {
    let index = segment.parse::<usize>().ok()?;
    (index.to_string() == segment).then_some(index)
}

pub fn valid_path(path: &[String]) -> bool {
    !path.is_empty() && path.len() <= 64 && path.iter().all(|part| !part.is_empty() && !["__proto__", "constructor", "prototype"].contains(&part.as_str()))
        && ["width", "height", "margin", "theme", "language", "tables", "scales", "coordinate", "layers", "guides", "title", "annotations", "presets"].contains(&path[0].as_str())
}

fn write_path(root: &mut DslValue, edit: &ChartEdit) -> MutationApplyResult<()> {
    if !valid_path(&edit.path) {
        return Err(MutationApplyError::new("print.chart.path", "invalid chart address").at(edit.path.clone()));
    }
    let mut parent = root;
    for segment in &edit.path[..edit.path.len() - 1] {
        parent = match parent {
            DslValue::Object(entries) => entries.iter_mut().find(|(key, _)| key == segment).map(|(_, value)| value),
            DslValue::Array(items) => array_index(segment).and_then(|index| items.get_mut(index)),
            _ => None,
        }.ok_or_else(|| MutationApplyError::new("print.chart.parent", "chart address parent is missing").at(edit.path.clone()))?;
    }
    let segment = &edit.path[edit.path.len() - 1];
    match parent {
        DslValue::Object(entries) => {
            let index = entries.iter().position(|(key, _)| key == segment);
            if !presence_equal(index.map(|index| &entries[index].1),edit.before.as_ref()) {
                return Err(MutationApplyError::new("print.chart.precondition", "chart field changed since diff was authored").at(edit.path.clone()));
            }
            match (index, &edit.after) {
                (Some(index), Some(value)) => entries[index].1 = value.clone(),
                (Some(index), None) => { entries.remove(index); },
                (None, Some(value)) => entries.push((segment.clone(), value.clone())),
                (None, None) => {},
            }
        },
        DslValue::Array(items) => {
            let index = array_index(segment).ok_or_else(|| MutationApplyError::new("print.chart.index", "array index must be canonical").at(edit.path.clone()))?;
            if index > items.len() || !presence_equal(items.get(index),edit.before.as_ref()) {
                return Err(MutationApplyError::new("print.chart.precondition", "array index or prior value is invalid").at(edit.path.clone()));
            }
            match &edit.after {
                Some(value) if index == items.len() => items.push(value.clone()),
                Some(value) => items[index] = value.clone(),
                None if index < items.len() => { items.remove(index); },
                None => {},
            }
        },
        _ => return Err(MutationApplyError::new("print.chart.parent", "chart address parent is scalar").at(edit.path.clone())),
    }
    Ok(())
}

impl MutationDiff<ChartSnapshot> for ChartDiff {
    fn apply(&self, base: &ChartSnapshot) -> MutationApplyResult<ChartSnapshot> {
        let mut next = base.clone();
        for edit in &self.edits { write_path(&mut next.chart, edit)?; }
        if !self.edits.is_empty() { crate::inferences::validate_chart(&next).map_err(|message| MutationApplyError::new("print.chart.schema", message))?; }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) { self.edits.extend(other.edits); }
}

impl DiffAlgebra<ChartSnapshot> for ChartDiff {
    fn inverse(&self, base: &ChartSnapshot) -> Self {
        match self.apply(base) { Ok(next) => Self::between(&next, base), Err(_) => Self::default() }
    }
    fn between(base: &ChartSnapshot, other: &ChartSnapshot) -> Self {
        let mut names = std::collections::BTreeSet::new();
        for value in [&base.chart, &other.chart] { if let Some(entries) = value.as_object() { names.extend(entries.iter().map(|(name, _)| name.clone())); } }
        Self { edits: names.into_iter().filter_map(|name| {
            let before = base.chart.get(&name).cloned();
            let after = other.chart.get(&name).cloned();
            (!presence_equal(before.as_ref(),after.as_ref())).then_some(ChartEdit { path: vec![name], before, after })
        }).collect() }
    }
    fn is_empty(&self) -> bool { self.edits.is_empty() }
}

impl DiffRegions for ChartDiff {
    fn touches(&self) -> TouchedPaths { TouchedPaths::new(self.edits.iter().map(|edit| format!("chart/{}", edit.path.join("/")))) }
}
