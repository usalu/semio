//! 👁️ Provider layer: the declaration-aware snapshot details provider.

use semio_s_artifact_stdio_ifc_declarations::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Locale;
use semio_framework_value::{DslValue, FromValue, RetainedWorkOutcome, ToValue};
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_contract::editing::{DslSnapshotDetailsProvider, SnapshotDetailItemCapabilities, SnapshotDetailPathSegment as P, SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailVariant, SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{ifc_arguments_identical, ifc_complex_identical, IfcComplexType, IfcEntity, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{edit_rules, IfcMutation};
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
pub use semio_s_artifact_stdio_ifc_details_edit::*;

const REFERENCE_WINDOW: usize = 256;
const DEPTH_LIMIT: usize = 64;


pub struct IfcDetailsProvider<'a, C: IfcDetailsProfile> {
    snapshot: &'a IfcSnapshot,
    profile: std::marker::PhantomData<fn() -> C>,
    raw: DslSnapshotDetailsProvider<'a, IfcSnapshot>,
    derived: std::cell::RefCell<std::collections::BTreeMap<usize, std::collections::BTreeSet<(&'static str, &'static str)>>>,
    contextual_enumerations:Option<(&'a IfcDetailsRenderWork,u64)>,
}

pub fn derived_keys<C: IfcDetailsProfile>(name: &str, complex: &[IfcComplexType]) -> std::collections::BTreeSet<(&'static str, &'static str)> {
    std::iter::once(name)
        .chain(complex.iter().map(|part| part.name.as_str()))
        .filter_map(|name| C::catalog().entities.get(&name.to_ascii_uppercase()))
        .flat_map(|declaration| declaration.all_attributes.iter())
        .filter(|attribute| attribute.derived)
        .map(|attribute| (attribute.declared_by.as_str(), attribute.name.as_str()))
        .collect()
}


fn named(name: &str) -> IfcTypeExpression {
    IfcTypeExpression { kind: "named".into(), name: Some(name.into()), ..Default::default() }
}

fn type_request(name: &str) -> DslValue {
    DslValue::Object(vec![("$ifcType".into(), DslValue::String(name.into()))])
}


fn resolved<C: IfcDetailsProfile>(expression: &IfcTypeExpression, depth: usize) -> IfcTypeExpression {
    if depth >= DEPTH_LIMIT {
        return expression.clone();
    }
    let Some(name) = expression.name.as_ref() else { return expression.clone() };
    let Some(declaration) = C::catalog().types.get(&name.to_ascii_uppercase()) else { return expression.clone() };
    match declaration.type_expr.as_ref() {
        Some(next) if declaration.kind == "defined" => resolved::<C>(next, depth + 1),
        _ => expression.clone(),
    }
}

impl<'a, C: IfcDetailsProfile> IfcDetailsProvider<'a, C> {
    pub fn attributes(name: &str) -> Option<&'static [IfcAttributeDeclaration]> {
        C::catalog().attribute_declarations(name)
    }
    pub fn new(snapshot: &'a IfcSnapshot) -> Self {
        Self { snapshot, profile: std::marker::PhantomData, raw: DslSnapshotDetailsProvider::new(snapshot), derived: Default::default(),contextual_enumerations:None }
    }
    pub fn with_contextual_enumerations(mut self,work:&'a IfcDetailsRenderWork,revision:u64)->Result<Self,semio_framework_value::ValueError>{work.validate(self.snapshot,revision)?;self.contextual_enumerations=Some((work,revision));Ok(self)}
    fn enumeration_choices(&self,path:&[P])->Option<(IfcEnumerationView<'a>,bool)>{let(work,revision)=self.contextual_enumerations?;let(entry,prefix)=work.at(self.snapshot,revision,path)?;let list=if path.len()==prefix{true}else if path.len()==prefix+2&&key(&path[prefix],"value")&&matches!(path[prefix+1],P::Index(_)){false}else{return None;};let view=entry.view(self.snapshot,revision).ok()?;view.allowed?;Some((view,list))}
    fn enumeration_variant(&self,path:&[P],locale:Locale,index:usize)->Option<SnapshotDetailVariant>{let(view,list)=self.enumeration_choices(path)?;if list&&index==0{return Some(SnapshotDetailVariant{label:match locale{Locale::En=>"Unset ($)",Locale::De=>"Nicht gesetzt ($)"}.into(),value:IfcValue::Unset.to_value()});}let index=index.checked_sub(usize::from(list))?;let preview=contextual_render::choice_label(view.allowed?.get(index)?);Some(SnapshotDetailVariant{label:match(locale,list){(Locale::En,false)=>format!("Choice {}: {preview}",index+1),(Locale::De,false)=>format!("Wert {}: {preview}",index+1),(Locale::En,true)=>format!("Use only {}: {preview}",index+1),(Locale::De,true)=>format!("Nur {} verwenden: {preview}",index+1)},value:contextual_render::choice_request(view.enumeration?,index,list)})}

    fn entity_at(&self, path: &[P]) -> Option<&'a IfcEntity> {
        let [P::Key(root), P::Index(index)] = path else { return None };
        (root == "entities").then(|| self.snapshot.entities.get(*index)).flatten()
    }

    fn complex_at(&self, path: &[P]) -> Option<&'a [semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType]> {
        let (P::Key(field), parent) = path.split_last()? else { return None };
        (field == "complex").then(|| self.entity_at(parent).map(|entity| entity.complex.as_slice())).flatten()
    }

    fn constituent_at(&self, path: &[P]) -> Option<&'a IfcComplexType> {
        let [P::Key(root), P::Index(entity), P::Key(field), P::Index(index)] = path else { return None };
        (root == "entities" && field == "complex").then(|| self.snapshot.entities.get(*entity)?.complex.get(*index)).flatten()
    }

    pub fn is_reference_payload(&self, path: &[P]) -> bool {
        if let Some((P::Key(field), parent)) = path.split_last() {
            if field == "appendReference" {
                return self.reference_collection(parent).is_some();
            }
            if field == "reference" {
                return self.reference_slot(parent);
            }
        }
        matches!(self.context(path), Some((attribute, _, IfcValue::Reference(_), true)) if !attribute.derived)
    }

    pub fn accepts_reference(&self, path: &[P], id: Option<u64>) -> bool {
        self.accepts_reference_controlled(path, id, &mut || Ok(())).unwrap_or(false)
    }
    pub fn accepts_reference_controlled(&self, path: &[P], id: Option<u64>, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<bool, String> {
        let context_path = if matches!(path.last(),Some(P::Key(field))if field=="reference") { &path[..path.len() - 1] } else { path };
        let Some((attribute, expression, current, _)) = self.context(context_path) else { return Ok(false) };
        let Some(id) = id else { return Ok(expression.optional || (attribute.optional && (context_path.len() == 4 || (context_path.len() == 6 && key(&context_path[2], "complex"))))) };
        if attribute.derived {
            return Ok(false);
        }
        let slot = if matches!(context_path.last(), Some(P::Key(field)) if field == "value") { &context_path[..context_path.len() - 1] } else { context_path };
        if let Some((P::Index(selected), parent)) = slot.split_last() {
            if let Some((P::Key(field), collection)) = parent.split_last() {
                if field == "value" {
                    if let Some((_, expression, IfcValue::Aggregate(items), false)) = self.context(collection) {
                        let domain = resolved::<C>(&expression, 0);
                        if domain.unique || domain.aggregate.as_deref() == Some("SET") {
                            for (index, value) in items.iter().enumerate() {
                                tick()?;
                                if index != *selected && value == &IfcValue::Reference(id) && current != &IfcValue::Reference(id) {
                                    return Ok(false);
                                }
                            }
                        }
                    }
                }
            }
        }
        for entity in &self.snapshot.entities {
            tick()?;
            if entity.id == id {
                for name in std::iter::once(entity.name.as_str()).chain(entity.complex.iter().map(|part| part.name.as_str())) {
                    tick()?;
                    if Self::accepts_entity(&expression, name, 0) {
                        return Ok(true);
                    }
                }
                return Ok(false);
            }
        }
        Ok(false)
    }

    fn reference_collection(&self, path: &[P]) -> Option<(IfcTypeExpression, &'a [IfcValue])> {
        let (attribute, expression, IfcValue::Aggregate(items), false) = self.context(path)? else { return None };
        let expression = resolved::<C>(&expression, 0);
        if attribute.derived || expression.aggregate.as_deref() == Some("ARRAY") || expression.upper.is_some_and(|upper| items.len() >= upper.max(0) as usize) || !Self::has_entity_type(expression.item.as_deref()?, 0) {
            return None;
        }
        Some((expression, items))
    }

    pub fn accepts_reference_append(&self, path: &[P], id: u64) -> bool {
        self.accepts_reference_append_controlled(path, id, &mut || Ok(())).unwrap_or(false)
    }
    pub fn accepts_reference_append_controlled(&self, path: &[P], id: u64, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<bool, String> {
        let Some((P::Key(field), parent)) = path.split_last() else { return Ok(false) };
        if field != "appendReference" {
            return Ok(false);
        }
        let Some((expression, items)) = self.reference_collection(parent) else { return Ok(false) };
        if expression.unique || expression.aggregate.as_deref() == Some("SET") {
            for item in items {
                tick()?;
                if item == &IfcValue::Reference(id) {
                    return Ok(false);
                }
            }
        }
        for entity in &self.snapshot.entities {
            tick()?;
            if entity.id == id {
                for name in std::iter::once(entity.name.as_str()).chain(entity.complex.iter().map(|part| part.name.as_str())) {
                    tick()?;
                    if Self::accepts_entity(expression.item.as_deref().ok_or("reference collection item is absent")?, name, 0) {
                        return Ok(true);
                    }
                }
                return Ok(false);
            }
        }
        Ok(false)
    }

    pub fn reference_slot(&self, path: &[P]) -> bool {
        self.context(path).is_some_and(|(attribute, expression, _, payload)| !attribute.derived && !payload && Self::has_entity_type(&expression, 0))
    }

    fn has_entity_type(expression: &IfcTypeExpression, depth: usize) -> bool {
        if depth >= DEPTH_LIMIT {
            return false;
        }
        let expression = resolved::<C>(expression, depth);
        let Some(name) = expression.name.as_ref() else { return false };
        if C::catalog().entities.contains_key(&name.to_ascii_uppercase()) {
            return true;
        }
        C::catalog().types.get(&name.to_ascii_uppercase()).is_some_and(|declaration| declaration.kind == "select" && declaration.members.iter().any(|member| Self::has_entity_type(&named(member), depth + 1)))
    }

    fn argument_templates(&self, attributes: &[IfcAttributeDeclaration], previous: Option<&[IfcAttributeDeclaration]>, values: &[IfcValue]) -> Vec<IfcValue> {
        attributes
            .iter()
            .map(|attribute| {
                if attribute.derived {
                    return IfcValue::Derived;
                }
                if let Some(value) = previous
                    .and_then(|previous| previous.iter().position(|old| old.name == attribute.name && old.type_expr == attribute.type_expr))
                    .and_then(|index| values.get(index))
                    .filter(|value| !matches!(value, IfcValue::Derived) && (!matches!(value, IfcValue::Unset) || attribute.optional))
                {
                    return value.clone();
                }
                if attribute.optional {
                    return IfcValue::Unset;
                }
                if attribute.name == "GlobalId" {
                    return IfcValue::String(String::new());
                }
                self.template(&attribute.type_expr, 0).unwrap_or(IfcValue::Unset)
            })
            .collect()
    }

    pub fn entity_template(&self, name: &str, current: Option<&IfcEntity>) -> Option<IfcEntity> {
        let partial = current.is_some_and(|entity| !entity.complex.is_empty());
        let attributes = if partial { C::catalog().entities.get(&name.to_ascii_uppercase())?.attributes.as_slice() } else { Self::attributes(name)? };
        let previous = current.and_then(|entity| if partial { C::catalog().entities.get(&entity.name.to_ascii_uppercase()).map(|declaration| declaration.attributes.as_slice()) } else { Self::attributes(&entity.name) });
        let args = self.argument_templates(&attributes, previous.as_deref(), current.map_or(&[], |entity| entity.args.as_slice()));
        let id = current.map_or(0, |entity| entity.id);
        let mut entity = IfcEntity { id, name: name.to_ascii_uppercase(), args, complex: current.map_or_else(Vec::new, |entity| entity.complex.clone()) };
        self.normalize_entity(&mut entity, partial);
        Some(entity)
    }

    pub fn normalized_argument(&self, attribute: &IfcAttributeDeclaration, value: &IfcValue, derived: &std::collections::BTreeSet<(&str, &str)>) -> Option<IfcValue> {
        let effective = attribute.derived || derived.contains(&(attribute.declared_by.as_str(), attribute.name.as_str()));
        if effective {
            return (!matches!(value, IfcValue::Derived)).then_some(IfcValue::Derived);
        }
        matches!(value, IfcValue::Derived).then(|| if attribute.optional { IfcValue::Unset } else { self.template(&attribute.type_expr, 0).unwrap_or(IfcValue::Unset) })
    }

    pub fn normalize_entity(&self, entity: &mut IfcEntity, partial: bool) {
        let derived = derived_keys::<C>(&entity.name, &entity.complex);
        let attributes = if partial || !entity.complex.is_empty() { C::catalog().entities.get(&entity.name.to_ascii_uppercase()).map(|declaration| declaration.attributes.as_slice()) } else { Self::attributes(&entity.name) };
        if let Some(attributes) = attributes {
            for (attribute, value) in attributes.iter().zip(&mut entity.args) {
                if let Some(next) = self.normalized_argument(attribute, value, &derived) {
                    *value = next;
                }
            }
        }
        for part in &mut entity.complex {
            if let Some(declaration) = C::catalog().entities.get(&part.name.to_ascii_uppercase()) {
                for (attribute, value) in declaration.attributes.iter().zip(&mut part.args) {
                    if let Some(next) = self.normalized_argument(attribute, value, &derived) {
                        *value = next;
                    }
                }
            }
        }
    }

    pub fn constituent_template(&self, name: &str, current: Option<&IfcComplexType>) -> Option<IfcComplexType> {
        let declaration = C::catalog().entities.get(&name.to_ascii_uppercase())?;
        let previous = current.and_then(|part| C::catalog().entities.get(&part.name.to_ascii_uppercase())).map(|declaration| declaration.attributes.as_slice());
        Some(IfcComplexType { name: declaration.name.clone(), args: self.argument_templates(&declaration.attributes, previous, current.map_or(&[], |part| part.args.as_slice())) })
    }

    fn context(&self, path: &[P]) -> Option<(IfcAttributeDeclaration, IfcTypeExpression, &'a IfcValue, bool)> {
        if path.len() < 4 || path.len() > DEPTH_LIMIT || !key(&path[0], "entities") {
            return None;
        }
        let P::Index(entity_index) = path[1] else { return None };
        let entity = self.snapshot.entities.get(entity_index)?;
        let (name, args, start) = if key(&path[2], "args") {
            (&entity.name, &entity.args, 3)
        } else if path.len() >= 6 && key(&path[2], "complex") && key(&path[4], "args") {
            let P::Index(index) = path[3] else { return None };
            let constituent = entity.complex.get(index)?;
            (&constituent.name, &constituent.args, 5)
        } else {
            return None;
        };
        let P::Index(index) = path[start] else { return None };
        let attributes = if start == 5 || !entity.complex.is_empty() { C::catalog().entities.get(&name.to_ascii_uppercase())?.attributes.as_slice() } else { Self::attributes(name)? };
        let mut attribute = attributes.get(index)?.clone();
        let mut cache = self.derived.borrow_mut();
        let derived = cache.entry(entity_index).or_insert_with(|| derived_keys::<C>(&entity.name, &entity.complex));
        if derived.contains(&(attribute.declared_by.as_str(), attribute.name.as_str())) {
            attribute.derived = true;
        }
        let mut expression = attribute.type_expr.clone();
        let mut value = args.get(index)?;
        let mut cursor = start + 1;
        let mut payload = false;
        while cursor < path.len() {
            if !key(&path[cursor], "value") {
                return None;
            }
            cursor += 1;
            payload = true;
            if cursor == path.len() {
                break;
            }
            match value {
                IfcValue::Aggregate(items) => {
                    let P::Index(index) = path[cursor] else { return None };
                    value = items.get(index)?;
                    let aggregate = resolved::<C>(&expression, 0);
                    expression = *aggregate.item?;
                    expression.optional |= aggregate.optional;
                    cursor += 1;
                    payload = false;
                }
                IfcValue::TypedValue(typed) if key(&path[cursor], "items") => {
                    cursor += 1;
                    if cursor == path.len() {
                        return Some((attribute, named(&typed.name), value, true));
                    }
                    let P::Index(index) = path[cursor] else { return None };
                    value = typed.items.get(index)?;
                    expression = named(&typed.name);
                    cursor += 1;
                    payload = false;
                }
                _ => return None,
            }
        }
        Some((attribute, expression, value, payload))
    }

    fn accepts_entity(expression: &IfcTypeExpression, name: &str, depth: usize) -> bool {
        if depth >= DEPTH_LIMIT {
            return false;
        }
        let Some(expected) = expression.name.as_ref() else { return false };
        if C::catalog().is_subtype(name, expected) {
            return true;
        }
        let Some(declaration) = C::catalog().types.get(&expected.to_ascii_uppercase()) else { return false };
        declaration.kind == "select" && declaration.members.iter().any(|member| Self::accepts_entity(&named(member), name, depth + 1))
    }

    pub fn template(&self, expression: &IfcTypeExpression, depth: usize) -> Option<IfcValue> {
        if depth >= DEPTH_LIMIT {
            return None;
        }
        let expression = resolved::<C>(expression, depth);
        if expression.kind == "aggregate" {
            let count = if expression.aggregate.as_deref() == Some("ARRAY") { expression.upper?.checked_sub(expression.lower?)?.checked_add(1)?.max(0) as usize } else { expression.lower.unwrap_or(0).max(0) as usize };
            if count > REFERENCE_WINDOW {
                return None;
            }
            if expression.unique && count > 1 {
                return None;
            }
            let value = self.template(expression.item.as_deref()?, depth + 1)?;
            return Some(IfcValue::Aggregate(vec![value; count]));
        }
        let name = expression.name.as_ref()?.to_ascii_uppercase();
        if C::catalog().entities.contains_key(&name) {
            return self.snapshot.entities.iter().take(REFERENCE_WINDOW).find(|entity| C::catalog().is_subtype(&entity.name, &name)).map(|entity| IfcValue::Reference(entity.id));
        }
        if let Some(declaration) = C::catalog().types.get(&name) {
            return match declaration.kind.as_str() {
                "enum" => declaration.members.first().map(|member| IfcValue::Enum(member.to_ascii_uppercase())),
                "select" => declaration.members.iter().find_map(|member| self.select_template(member, depth + 1)),
                _ => None,
            };
        }
        match name.as_str() {
            "STRING" => Some(IfcValue::String(String::new())),
            "BINARY" => Some(IfcValue::Binary("0".into())),
            "REAL" | "NUMBER" => Some(IfcValue::Real(0.0)),
            "INTEGER" => Some(IfcValue::Integer(0)),
            "BOOLEAN" | "LOGICAL" => Some(IfcValue::Enum("F".into())),
            _ => None,
        }
    }

    fn select_template(&self, name: &str, depth: usize) -> Option<IfcValue> {
        let value = self.template(&named(name), depth)?;
        if matches!(value, IfcValue::Reference(_) | IfcValue::TypedValue(_)) {
            return Some(value);
        }
        Some(IfcValue::TypedValue(IfcTypedValue { name: name.to_ascii_uppercase(), items: vec![value] }))
    }

    fn select_members(name: &str, depth: usize, members: &mut std::collections::BTreeSet<String>) {
        if depth >= DEPTH_LIMIT {
            return;
        }
        if let Some(declaration) = C::catalog().types.get(&name.to_ascii_uppercase()).filter(|declaration| declaration.kind == "select") {
            for member in &declaration.members {
                Self::select_members(member, depth + 1, members);
            }
        } else {
            members.insert(name.to_ascii_uppercase());
        }
    }

    fn unused_template(&self, expression: &IfcTypeExpression, existing: &[IfcValue]) -> Option<IfcValue> {
        let candidate = self.template(expression, 0)?;
        if !existing.contains(&candidate) {
            return Some(candidate);
        }
        if let Some(entity) = self.snapshot.entities.iter().take(REFERENCE_WINDOW).find(|entity| Self::accepts_entity(expression, &entity.name, 0) && !existing.contains(&IfcValue::Reference(entity.id))) {
            return Some(IfcValue::Reference(entity.id));
        }
        let domain = resolved::<C>(expression, 0);
        if let Some(declaration) = domain.name.as_ref().and_then(|name| C::catalog().types.get(&name.to_ascii_uppercase())) {
            if declaration.kind == "enum" {
                return declaration.members.iter().map(|member| IfcValue::Enum(member.to_ascii_uppercase())).find(|value| !existing.contains(value));
            }
            if declaration.kind == "select" {
                let mut members = std::collections::BTreeSet::new();
                Self::select_members(&declaration.name, 0, &mut members);
                return members.iter().filter_map(|member| self.select_template(member, 0)).find(|value| !existing.contains(value));
            }
        }
        (0..REFERENCE_WINDOW)
            .filter_map(|index| {
                let value = match &candidate {
                    IfcValue::String(_) => {
                        let text = if domain.fixed { format!("{index:0width$}", width = domain.width.unwrap_or(0).clamp(0, REFERENCE_WINDOW as i64) as usize) } else { index.to_string() };
                        if domain.width.is_some_and(|width| text.chars().count() > width.max(0) as usize) {
                            return None;
                        }
                        IfcValue::String(text)
                    }
                    IfcValue::Integer(_) => IfcValue::Integer(index as i64),
                    IfcValue::Real(_) => IfcValue::Real(index as f64),
                    IfcValue::Binary(_) => IfcValue::Binary(format!("0{index:02X}")),
                    _ => return None,
                };
                (!existing.contains(&value)).then_some(value)
            })
            .next()
    }

    fn alternatives(&self, path: &[P], locale: Locale) -> Vec<SnapshotDetailVariant> {
        let Some((attribute, expression, value, payload)) = self.context(path) else { return Vec::new() };
        if payload || attribute.derived {
            return Vec::new();
        }
        let mut variants = Vec::new();
        if expression.optional || (attribute.optional && (path.len() == 4 || (path.len() == 6 && key(&path[2], "complex")))) {
            variants.push(SnapshotDetailVariant {
                label: match locale {
                    Locale::En => "Unset ($)",
                    Locale::De => "Nicht gesetzt ($)",
                }
                .into(),
                value: IfcValue::Unset.to_value(),
            });
        }
        if let Some(template) = self.template(&expression, 0) {
            if matches!(value, IfcValue::Unset) {
                variants.push(SnapshotDetailVariant {
                    label: match locale {
                        Locale::En => "Create value",
                        Locale::De => "Wert erstellen",
                    }
                    .into(),
                    value: template.to_value(),
                });
            }
        }
        let expression = resolved::<C>(&expression, 0);
        if let Some(declaration) = expression.name.as_ref().and_then(|name| C::catalog().types.get(&name.to_ascii_uppercase())).filter(|declaration| declaration.kind == "select") {
            let mut members = std::collections::BTreeSet::new();
            Self::select_members(&declaration.name, 0, &mut members);
            for member in members {
                if let Some(value) = self.select_template(&member, 0) {
                    variants.push(SnapshotDetailVariant { label: member, value: value.to_value() });
                }
            }
        }
        for entity in self.snapshot.entities.iter().take(REFERENCE_WINDOW).filter(|entity| Self::accepts_entity(&expression, &entity.name, 0)) {
            let title = entity.args.get(2).and_then(IfcValue::as_str).unwrap_or("");
            variants.push(SnapshotDetailVariant { label: format!("#{} {} {title}", entity.id, entity.name), value: IfcValue::Reference(entity.id).to_value() });
        }
        variants
    }
}

impl<C: IfcDetailsProfile> SnapshotDetailsProvider for IfcDetailsProvider<'_, C> {
    fn value(&self, path: &[P]) -> Option<SnapshotDetailValue> {
        if self.complex_at(path).is_some() {
            return Some(SnapshotDetailValue::Array);
        }
        if let Some((P::Key(field), parent)) = path.split_last() {
            if field == "appendReference" && self.reference_collection(parent).is_some() {
                return Some(SnapshotDetailValue::String(String::new()));
            }
            if field == "reference" && self.reference_slot(parent) {
                let (_, _, value, _) = self.context(parent)?;
                return Some(SnapshotDetailValue::String(value.as_reference().map_or_else(String::new, |id| id.to_string())));
            }
        }
        if let Some((_, _, IfcValue::Reference(id), true)) = self.context(path) {
            return Some(SnapshotDetailValue::String(id.to_string()));
        }
        self.raw.value(path)
    }
    fn child_count(&self, path: &[P]) -> usize {
        if self.entity_at(path).is_some() {
            return 4;
        }
        if let Some(complex) = self.complex_at(path) {
            return complex.len();
        }
        if let Some((_, _, value, false)) = self.context(path) {
            return usize::from(!matches!(value, IfcValue::Unset | IfcValue::Derived | IfcValue::Reference(_))) + usize::from(self.reference_slot(path)) + usize::from(self.reference_collection(path).is_some());
        }
        self.raw.child_count(path)
    }
    fn object_key(&self, path: &[P], index: usize) -> Option<String> {
        if self.entity_at(path).is_some() {
            return ["id", "name", "args", "complex"].get(index).map(|field| (*field).into());
        }
        if let Some((_, _, value, false)) = self.context(path) {
            let payload = !matches!(value, IfcValue::Unset | IfcValue::Derived | IfcValue::Reference(_));
            if index == 0 && payload {
                return Some("value".into());
            }
            if index == usize::from(payload) {
                if self.reference_slot(path) {
                    return Some("reference".into());
                }
                if self.reference_collection(path).is_some() {
                    return Some("appendReference".into());
                }
            }
            return None;
        }
        self.raw.object_key(path, index)
    }
    fn has_source(&self) -> bool {
        true
    }
    fn source(&self) -> Option<String> {
        self.raw.source()
    }
    fn allows_untyped_creation(&self, _path: &[P]) -> bool {
        false
    }
    fn enum_values(&self, path: &[P]) -> Vec<DslValue> {
        let Some((_, expression, value, true)) = self.context(path) else { return Vec::new() };
        if !matches!(value, IfcValue::Enum(_)) {
            return Vec::new();
        }
        let expression = resolved::<C>(&expression, 0);
        let Some(name) = expression.name else { return Vec::new() };
        if matches!(name.as_str(), "BOOLEAN" | "LOGICAL") {
            return if name == "BOOLEAN" { vec!["T", "F"] } else { vec!["T", "F", "U"] }.into_iter().map(|value| DslValue::String(value.into())).collect();
        }
        C::catalog().types.get(&name.to_ascii_uppercase()).filter(|declaration| declaration.kind == "enum").map(|declaration| declaration.members.iter().map(|member| DslValue::String(member.to_ascii_uppercase())).collect()).unwrap_or_default()
    }
    fn presentation(&self, path: &[P], locale: Locale) -> Option<SnapshotDetailPresentation> {
        if let Some((work,revision))=self.contextual_enumerations{if let Some((entry,prefix))=work.at(self.snapshot,revision,path){if path.len()==prefix{if let Some(diagnostic)=entry.view(self.snapshot,revision).ok()?.diagnostic{return Some(SnapshotDetailPresentation{label:"EnumerationValues".into(),description:Some(match(locale,diagnostic){(Locale::En,IfcEnumerationDiagnostic::Unset)=>"Enumeration reference is unset. Set its exact entity ID to obtain source choices.",(Locale::De,IfcEnumerationDiagnostic::Unset)=>"Aufzählungsreferenz ist nicht gesetzt. Exakte Element-ID für Quellwerte festlegen.",(Locale::En,IfcEnumerationDiagnostic::Missing)=>"Enumeration reference target is missing. Repair the reference to obtain source choices.",(Locale::De,IfcEnumerationDiagnostic::Missing)=>"Ziel der Aufzählungsreferenz fehlt. Referenz für Quellwerte korrigieren.",(Locale::En,IfcEnumerationDiagnostic::WrongKind)=>"Referenced source is not a complete enumeration. Repair its type or values.",(Locale::De,IfcEnumerationDiagnostic::WrongKind)=>"Referenzierte Quelle ist keine vollständige Aufzählung. Typ oder Werte korrigieren.",(Locale::En,IfcEnumerationDiagnostic::Ambiguous)=>"Enumeration source identities or constituents are ambiguous. Repair the source.",(Locale::De,IfcEnumerationDiagnostic::Ambiguous)=>"Quellidentitäten oder Bestandteile sind mehrdeutig. Quelle korrigieren.",(Locale::En,IfcEnumerationDiagnostic::Empty)=>"Referenced enumeration has no allowed values. Repair its values.",(Locale::De,IfcEnumerationDiagnostic::Empty)=>"Referenzierte Aufzählung hat keine zulässigen Werte. Werte korrigieren."}.into())});}}}}
        if let Some((attribute, expression, value, payload)) = self.context(path) {
            if !payload {
                let state = match (locale, value) {
                    (Locale::En, IfcValue::Unset) => " — unset",
                    (Locale::De, IfcValue::Unset) => " — nicht gesetzt",
                    (Locale::En, IfcValue::Derived) => " — derived",
                    (Locale::De, IfcValue::Derived) => " — abgeleitet",
                    _ => "",
                };
                let type_name = expression.name.as_deref().or(expression.aggregate.as_deref()).unwrap_or("");
                return Some(SnapshotDetailPresentation {
                    label: format!("{}{state}", attribute.name),
                    description: Some(match locale {
                        Locale::En => format!("{} · {type_name}. Suggestions inspect the first {REFERENCE_WINDOW} entities. The reference field accepts every exact entity ID.", attribute.declared_by),
                        Locale::De => format!("{} · {type_name}. Vorschläge prüfen die ersten {REFERENCE_WINDOW} Entitäten. Das Referenzfeld nimmt jede exakte Entitäts-ID an.", attribute.declared_by),
                    }),
                });
            }
        }
        if path.len() == 2 && key(&path[0], "entities") {
            let P::Index(index) = path[1] else { return None };
            let entity = self.snapshot.entities.get(index)?;
            return C::entity_presentation(entity, locale).or_else(|| Some(SnapshotDetailPresentation { label: format!("#{} {}", entity.id, entity.name), description: None }));
        }
        let P::Key(last) = path.last()? else { return None };
        let label = match (locale, last.as_str()) {
            (Locale::En, "appendReference") => "Add reference by entity ID",
            (Locale::De, "appendReference") => "Referenz per Entitäts-ID hinzufügen",
            (Locale::En, "reference") => "Reference entity ID",
            (Locale::De, "reference") => "Referenzierte Entitäts-ID",
            (Locale::En, "entities") => "Entities",
            (Locale::De, "entities") => "Entitäten",
            (Locale::En, "header") => "File header",
            (Locale::De, "header") => "Dateikopf",
            (Locale::En, "args") => "Attributes",
            (Locale::De, "args") => "Attribute",
            (Locale::En, "complex") => "Complex constituents",
            (Locale::De, "complex") => "Komplexe Bestandteile",
            (Locale::En, "name") => "IFC type",
            (Locale::De, "name") => "IFC-Typ",
            (Locale::En, "value") => "Value",
            (Locale::De, "value") => "Wert",
            _ => return self.raw.presentation(path, locale),
        };
        Some(SnapshotDetailPresentation { label: label.into(), description: None })
    }
    fn item_capabilities(&self, path: &[P]) -> SnapshotDetailItemCapabilities {
        let mut capabilities = SnapshotDetailItemCapabilities { edit: false, rename: false, remove: false, reorder: false };
        if self.is_reference_payload(path) {
            capabilities.edit = true;
            return capabilities;
        }
        if let Some((attribute, _, value, payload)) = self.context(path) {
            capabilities.edit = payload && !attribute.derived && !matches!(value, IfcValue::Aggregate(_) | IfcValue::TypedValue(_));
            if matches!(path.last(), Some(P::Index(_))) && payload == false && path.len() > 4 {
                let parent = &path[..path.len() - 1];
                if let Some((_, parent_type, IfcValue::Aggregate(items), true)) = self.context(parent) {
                    let aggregate = resolved::<C>(&parent_type, 0);
                    capabilities.remove = aggregate.aggregate.as_deref() != Some("ARRAY") && items.len() > aggregate.lower.unwrap_or(0).max(0) as usize;
                    capabilities.reorder = matches!(aggregate.aggregate.as_deref(), Some("LIST") | Some("ARRAY"));
                }
            }
            return capabilities;
        }
        if path.len() >= 2 && key(&path[0], "header") {
            capabilities.edit = !matches!(path.last(), Some(P::Key(value)) if value == "kind");
        }
        if path.len() > 3 && key(&path[0], "header") && matches!(path.last(), Some(P::Index(_))) && path.iter().any(|segment| key(segment, "value")) {
            capabilities.remove = self.raw.child_count(&path[..path.len() - 1]) > 1;
            capabilities.reorder = true;
        }
        if path.len() == 2 && key(&path[0], "entities") {
            capabilities.remove = true;
        }
        if path.len() >= 4 && key(&path[0], "entities") && key(&path[2], "complex") {
            if path.len() == 4 {
                capabilities.remove = true;
                capabilities.reorder = true;
            }
            if path.len() == 5 && key(&path[4], "name") {
                capabilities.edit = true;
            }
        }
        capabilities
    }
    fn allows_collection_insert(&self, path: &[P]) -> bool {
        if path.len() == 1 && key(&path[0], "entities") {
            return self.creation_template(path, true).is_some();
        }
        if path.len() == 3 && key(&path[0], "entities") && key(&path[2], "complex") {
            return true;
        }
        if path.len() > 2 && key(&path[0], "header") && path.last().is_some_and(|segment| key(segment, "value")) && matches!(self.raw.value(path), Some(SnapshotDetailValue::Array)) {
            return true;
        }
        let Some((attribute, expression, IfcValue::Aggregate(items), true)) = self.context(path) else { return false };
        let expression = resolved::<C>(&expression, 0);
        !attribute.derived && expression.aggregate.as_deref() != Some("ARRAY") && expression.upper.is_none_or(|upper| items.len() < upper.max(0) as usize) && self.creation_template(path, true).is_some()
    }
    fn creation_template(&self, path: &[P], collection_item: bool) -> Option<DslValue> {
        if let Some((work,revision))=self.contextual_enumerations{if let Some((entry,prefix))=work.at(self.snapshot,revision,path){if collection_item&&path.len()==prefix+1&&key(&path[prefix],"value"){let view=entry.view(self.snapshot,revision).ok()?;view.allowed?.first()?;return Some(contextual_render::choice_request(view.enumeration?,0,false));}if !collection_item&&path.len()==prefix{let view=entry.view(self.snapshot,revision).ok()?;view.allowed?.first()?;return Some(contextual_render::choice_request(view.enumeration?,0,true));}}}
        if collection_item && path.len() == 1 && key(&path[0], "entities") {
            return self.entity_template("IFCCARTESIANPOINT", None).map(|entity| entity.to_value());
        }
        if collection_item && path.len() == 3 && key(&path[0], "entities") && key(&path[2], "complex") {
            return Some(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: "IFCREPRESENTATIONITEM".into(), args: Vec::new() }.to_value());
        }
        if collection_item && path.len() > 2 && key(&path[0], "header") && path.last().is_some_and(|segment| key(segment, "value")) {
            return Some(IfcValue::String(String::new()).to_value());
        }
        let (attribute, expression, value, payload) = self.context(path)?;
        if attribute.derived {
            return None;
        }
        if collection_item {
            let expression = resolved::<C>(&expression, 0);
            if expression.unique || expression.aggregate.as_deref() == Some("SET") {
                return self.unused_template(expression.item.as_deref()?, value.as_aggregate()?).map(|value| value.to_value());
            }
            self.template(expression.item.as_deref()?, 0).map(|candidate| candidate.to_value())
        } else if !payload {
            self.template(&expression, 0).map(|value| value.to_value())
        } else {
            None
        }
    }
    fn variants(&self, path: &[P], locale: Locale) -> Vec<SnapshotDetailVariant> {
        if self.enumeration_choices(path).is_some(){return(0..self.variant_count(path,locale)).filter_map(|index|self.enumeration_variant(path,locale,index)).collect();}
        C::detail_variants(self.snapshot, path, locale).unwrap_or_else(|| self.alternatives(path, locale))
    }
    fn variant_count(&self, path: &[P], locale: Locale) -> usize {
        if let Some((view,list))=self.enumeration_choices(path){return view.allowed.map_or(0,|values|values.len())+usize::from(list);}
        if let Some(variants) = C::detail_variants(self.snapshot, path, locale) {
            return variants.len();
        }
        if let Some(current) = self.entity_at(path) {
            return C::catalog().entities.values().filter(|entity| !current.complex.is_empty() || !entity.abstract_entity).count();
        }
        if self.constituent_at(path).is_some() {
            return C::catalog().entities.len();
        }
        self.alternatives(path, locale).len()
    }
    fn variant(&self, path: &[P], locale: Locale, index: usize) -> Option<SnapshotDetailVariant> {
        if self.enumeration_choices(path).is_some(){return self.enumeration_variant(path,locale,index);}
        if let Some(variants) = C::detail_variants(self.snapshot, path, locale) {
            return variants.into_iter().nth(index);
        }
        if let Some(current) = self.entity_at(path) {
            let declaration = C::catalog().entities.values().filter(|entity| !current.complex.is_empty() || !entity.abstract_entity).nth(index)?;
            return Some(SnapshotDetailVariant { label: declaration.name.clone(), value: type_request(&declaration.name) });
        }
        if let Some(current) = self.constituent_at(path) {
            let declaration = C::catalog().entities.values().nth(index)?;
            let _ = current;
            return Some(SnapshotDetailVariant { label: declaration.name.clone(), value: type_request(&declaration.name) });
        }
        self.alternatives(path, locale).into_iter().nth(index)
    }
}

#[cfg(test)]
#[path="../contextual_render_tests.rs"]
mod contextual_render_tests;
