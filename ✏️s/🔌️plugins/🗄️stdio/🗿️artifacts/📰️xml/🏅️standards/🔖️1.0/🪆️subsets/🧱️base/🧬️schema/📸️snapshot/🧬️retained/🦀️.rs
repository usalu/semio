//! 🧬️ Flat paged ownership for shared XML documents.

use super::{XmlAttr, XmlDeclaration, XmlDocument, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote};
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind, list::PagedList, paged::PagedUtf8};

pub const RETAINED_XML_MAX_TEXT_BYTES: usize = u32::MAX as usize;
pub const RETAINED_XML_MAX_NODES: usize = u32::MAX as usize;
pub const RETAINED_XML_MAX_ATTRIBUTES: usize = u32::MAX as usize;
pub const RETAINED_XML_MAX_BOUNDARIES: usize = u32::MAX as usize;
pub const RETAINED_XML_MAX_DTD_DECLARATIONS: usize = u32::MAX as usize;

pub type RetainedXmlText = PagedUtf8<RETAINED_XML_MAX_TEXT_BYTES>;
pub type RetainedXmlNodes = PagedList<RetainedXmlNode, RETAINED_XML_MAX_NODES>;
pub type RetainedXmlAttributes = PagedList<RetainedXmlAttribute, RETAINED_XML_MAX_ATTRIBUTES>;
pub type RetainedXmlBoundaries = PagedList<usize, RETAINED_XML_MAX_BOUNDARIES>;
pub type RetainedXmlDtdDeclarations = PagedList<RetainedXmlDtdDeclaration, RETAINED_XML_MAX_DTD_DECLARATIONS>;

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedXmlAttribute {
    pub name: RetainedXmlText,
    pub value: RetainedXmlText,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RetainedXmlNodeKind {
    Element { name: RetainedXmlText, first_attribute: usize, attribute_count: usize, first_child: Option<usize> },
    Text { text: RetainedXmlText },
    CData { text: RetainedXmlText },
    Comment { text: RetainedXmlText },
    ProcessingInstruction { target: RetainedXmlText, data: RetainedXmlText },
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedXmlNode {
    pub next_sibling: Option<usize>,
    pub value: RetainedXmlNodeKind,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum RetainedXmlExternalId {
    System { system_id: RetainedXmlText },
    Public { public_id: RetainedXmlText, system_id: RetainedXmlText },
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RetainedXmlDtdDeclaration {
    Entity { parameter: bool, name: RetainedXmlText, value: RetainedXmlText },
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedXmlDoctype {
    pub prolog_position: u64,
    pub name: RetainedXmlText,
    pub external_id: Option<RetainedXmlExternalId>,
    pub declarations: RetainedXmlDtdDeclarations,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedXmlDeclaration {
    pub version: RetainedXmlText,
    pub encoding: Option<RetainedXmlText>,
    pub standalone: Option<bool>,
    pub quote: XmlQuote,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedXmlDocument {
    pub nodes: RetainedXmlNodes,
    pub attributes: RetainedXmlAttributes,
    pub prolog: RetainedXmlBoundaries,
    pub epilog: RetainedXmlBoundaries,
    pub root: Option<usize>,
    pub doctype: Option<RetainedXmlDoctype>,
    pub declaration: Option<RetainedXmlDeclaration>,
}

fn text(value: &str) -> Result<RetainedXmlText, ValueError> {
    RetainedXmlText::try_from_str(value)
}

fn append<T, const N: usize>(owner: &mut PagedList<T, N>, value: T) -> Result<usize, ValueError> {
    let ordinal = owner.len();
    owner.try_push(value).map_err(ValueError::from)?;
    Ok(ordinal)
}

fn append_controlled<T, const N: usize>(owner: &mut PagedList<T, N>, value: T, control: &mut NativeDecodeControl<'_>) -> Result<usize, ValueError> {
    while !owner.has_reserved_slot() {
        let required = owner.next_allocation_bytes()?;
        control.charge(required)?;
        let progress = owner.reserve_one(required).map_err(|error| ValueError::from(error.refusal()))?;
        if !progress.progressed {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "retained XML controlled allocation did not progress"));
        }
    }
    let ordinal = owner.len();
    owner.push_reserved(value).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "retained XML controlled append rejected its admitted slot"))?;
    Ok(ordinal)
}

struct NodeFrame<'a> {
    source: &'a XmlNode,
    first_attribute: usize,
    attribute_count: usize,
    child_position: usize,
    first_child: Option<usize>,
    last_child: Option<usize>,
}

impl RetainedXmlDocument {
    pub fn try_from_document(source: &XmlDocument) -> Result<Self, ValueError> {
        let mut output = Self::default();
        for node in &source.prolog {
            let ordinal = output.append_tree(node)?;
            append(&mut output.prolog, ordinal)?;
        }
        output.root = source.root.as_ref().map(|node| output.append_tree(node)).transpose()?;
        for node in &source.epilog {
            let ordinal = output.append_tree(node)?;
            append(&mut output.epilog, ordinal)?;
        }
        output.doctype = source.doctype.as_ref().map(RetainedXmlDoctype::try_from_doctype).transpose()?;
        output.declaration = source.declaration.as_ref().map(RetainedXmlDeclaration::try_from_declaration).transpose()?;
        output.validate()?;
        Ok(output)
    }

    pub fn try_from_document_controlled(source: &XmlDocument, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_stage(|control| {
            let total = source.prolog.len().saturating_add(source.epilog.len()).saturating_add(usize::from(source.root.is_some()));
            control.begin_stage(total)?;
            let mut output = Self::default();
            for node in &source.prolog {
                let ordinal = output.append_tree_controlled(node, control)?;
                append_controlled(&mut output.prolog, ordinal, control)?;
                control.step()?;
            }
            output.root = source.root.as_ref().map(|node| output.append_tree_controlled(node, control)).transpose()?;
            if source.root.is_some() {
                control.step()?;
            }
            for node in &source.epilog {
                let ordinal = output.append_tree_controlled(node, control)?;
                append_controlled(&mut output.epilog, ordinal, control)?;
                control.step()?;
            }
            output.doctype = source.doctype.as_ref().map(|value| RetainedXmlDoctype::try_from_doctype_controlled(value, control)).transpose()?;
            output.declaration = source.declaration.as_ref().map(|value| RetainedXmlDeclaration::try_from_declaration_controlled(value, control)).transpose()?;
            let mut incoming = control.allocate_vec::<u8>(output.nodes.len())?;
            incoming.resize(output.nodes.len(), 0);
            let root_count = output.prolog.len().saturating_add(output.epilog.len()).saturating_add(usize::from(output.root.is_some()));
            let roots = control.allocate_vec::<usize>(root_count)?;
            output.validate_owned(incoming, roots)?;
            Ok(output)
        })
    }

    fn frame<'a>(&mut self, source: &'a XmlNode) -> Result<NodeFrame<'a>, ValueError> {
        let first_attribute = self.attributes.len();
        let attribute_count = match source {
            XmlNode::Element { attrs, .. } => {
                for attr in attrs {
                    append(&mut self.attributes, RetainedXmlAttribute { name: text(&attr.name)?, value: text(&attr.value)? })?;
                }
                attrs.len()
            }
            _ => 0,
        };
        Ok(NodeFrame { source, first_attribute, attribute_count, child_position: 0, first_child: None, last_child: None })
    }

    fn append_tree(&mut self, source: &XmlNode) -> Result<usize, ValueError> {
        let mut stack = vec![self.frame(source)?];
        loop {
            let child = match stack.last_mut().expect("XML tree frontier retains its root") {
                NodeFrame { source: XmlNode::Element { children, .. }, child_position, .. } if *child_position < children.len() => {
                    let child = &children[*child_position];
                    *child_position += 1;
                    Some(child)
                }
                _ => None,
            };
            if let Some(child) = child {
                let frame = self.frame(child)?;
                stack.push(frame);
                continue;
            }
            let frame = stack.pop().expect("XML tree frontier retains its completed node");
            let value = match frame.source {
                XmlNode::Element { name, .. } => RetainedXmlNodeKind::Element {
                    name: text(name)?,
                    first_attribute: frame.first_attribute,
                    attribute_count: frame.attribute_count,
                    first_child: frame.first_child,
                },
                XmlNode::Text { text: value } => RetainedXmlNodeKind::Text { text: text(value)? },
                XmlNode::CData { text: value } => RetainedXmlNodeKind::CData { text: text(value)? },
                XmlNode::Comment { text: value } => RetainedXmlNodeKind::Comment { text: text(value)? },
                XmlNode::ProcessingInstruction { target, data } => RetainedXmlNodeKind::ProcessingInstruction { target: text(target)?, data: text(data)? },
            };
            let ordinal = append(&mut self.nodes, RetainedXmlNode { next_sibling: None, value })?;
            let Some(parent) = stack.last_mut() else { return Ok(ordinal) };
            if let Some(previous) = parent.last_child {
                self.nodes.get_mut(previous).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained XML sibling frontier is missing"))?.next_sibling = Some(ordinal);
            } else {
                parent.first_child = Some(ordinal);
            }
            parent.last_child = Some(ordinal);
        }
    }

    fn frame_controlled<'a>(&mut self, source: &'a XmlNode, control: &mut NativeDecodeControl<'_>) -> Result<NodeFrame<'a>, ValueError> {
        let first_attribute = self.attributes.len();
        let attribute_count = match source {
            XmlNode::Element { attrs, .. } => {
                for attr in attrs {
                    append_controlled(
                        &mut self.attributes,
                        RetainedXmlAttribute {
                            name: RetainedXmlText::try_from_str_controlled(&attr.name, control)?,
                            value: RetainedXmlText::try_from_str_controlled(&attr.value, control)?,
                        },
                        control,
                    )?;
                }
                attrs.len()
            }
            _ => 0,
        };
        Ok(NodeFrame { source, first_attribute, attribute_count, child_position: 0, first_child: None, last_child: None })
    }

    fn append_tree_controlled(&mut self, source: &XmlNode, control: &mut NativeDecodeControl<'_>) -> Result<usize, ValueError> {
        let mut stack = PagedList::<NodeFrame<'_>, RETAINED_XML_MAX_NODES>::default();
        let frame = self.frame_controlled(source, control)?;
        append_controlled(&mut stack, frame, control)?;
        loop {
            let last = stack.len().checked_sub(1).expect("XML tree frontier retains its root");
            let child = match stack.get_mut(last).expect("XML tree frontier retains its last frame") {
                NodeFrame { source: XmlNode::Element { children, .. }, child_position, .. } if *child_position < children.len() => {
                    let child = &children[*child_position];
                    *child_position += 1;
                    Some(child)
                }
                _ => None,
            };
            if let Some(child) = child {
                let frame = self.frame_controlled(child, control)?;
                append_controlled(&mut stack, frame, control)?;
                continue;
            }
            let frame = stack.pop().expect("XML tree frontier retains its completed node");
            let value = match frame.source {
                XmlNode::Element { name, .. } => RetainedXmlNodeKind::Element {
                    name: RetainedXmlText::try_from_str_controlled(name, control)?,
                    first_attribute: frame.first_attribute,
                    attribute_count: frame.attribute_count,
                    first_child: frame.first_child,
                },
                XmlNode::Text { text } => RetainedXmlNodeKind::Text { text: RetainedXmlText::try_from_str_controlled(text, control)? },
                XmlNode::CData { text } => RetainedXmlNodeKind::CData { text: RetainedXmlText::try_from_str_controlled(text, control)? },
                XmlNode::Comment { text } => RetainedXmlNodeKind::Comment { text: RetainedXmlText::try_from_str_controlled(text, control)? },
                XmlNode::ProcessingInstruction { target, data } => RetainedXmlNodeKind::ProcessingInstruction {
                    target: RetainedXmlText::try_from_str_controlled(target, control)?,
                    data: RetainedXmlText::try_from_str_controlled(data, control)?,
                },
            };
            let ordinal = append_controlled(&mut self.nodes, RetainedXmlNode { next_sibling: None, value }, control)?;
            let Some(parent) = stack.len().checked_sub(1).and_then(|index| stack.get_mut(index)) else { return Ok(ordinal) };
            if let Some(previous) = parent.last_child {
                self.nodes.get_mut(previous).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained XML sibling frontier is missing"))?.next_sibling = Some(ordinal);
            } else {
                parent.first_child = Some(ordinal);
            }
            parent.last_child = Some(ordinal);
        }
    }

    fn validate_owned(&self, mut incoming: Vec<u8>, mut roots: Vec<usize>) -> Result<(), ValueError> {
        roots.extend(self.prolog.iter().copied());
        roots.extend(self.root);
        roots.extend(self.epilog.iter().copied());
        for root in roots {
            let slot = incoming.get_mut(root).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "retained XML root ordinal is out of range"))?;
            *slot = slot.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "retained XML root ownership overflow"))?;
        }
        for (ordinal, node) in self.nodes.iter().enumerate() {
            if node.next_sibling.is_some_and(|next| next <= ordinal || next >= self.nodes.len()) {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, "retained XML sibling ordinal is not forward and in range"));
            }
            if let Some(next) = node.next_sibling {
                incoming[next] = incoming[next].checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "retained XML sibling ownership overflow"))?;
            }
            if let RetainedXmlNodeKind::Element { first_attribute, attribute_count, first_child, .. } = &node.value {
                let end = first_attribute.checked_add(*attribute_count).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "retained XML attribute range overflow"))?;
                if end > self.attributes.len() {
                    return Err(ValueError::new(ValueRefusalKind::InvalidValue, "retained XML attribute range is out of bounds"));
                }
                if first_child.is_some_and(|child| child >= ordinal) {
                    return Err(ValueError::new(ValueRefusalKind::InvalidValue, "retained XML child ordinal is not earlier than its parent"));
                }
                if let Some(child) = first_child {
                    incoming[*child] = incoming[*child].checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "retained XML child ownership overflow"))?;
                }
            }
        }
        if incoming.iter().any(|count| *count != 1) {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "retained XML nodes must have exactly one document or parent owner"));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        let incoming = vec![0u8; self.nodes.len()];
        let roots = Vec::with_capacity(self.prolog.len().saturating_add(self.epilog.len()).saturating_add(usize::from(self.root.is_some())));
        self.validate_owned(incoming, roots)
    }

    pub fn materialization_owned_bytes(&self) -> Result<usize, ValueError> {
        fn add(total: &mut usize, bytes: usize) -> Result<(), ValueError> {
            *total = total.checked_add(bytes).filter(|value| *value <= isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML materialization ownership overflow"))?;
            Ok(())
        }
        fn text<const N: usize>(total: &mut usize, value: &PagedUtf8<N>) -> Result<(), ValueError> {
            add(total, value.len())
        }
        let mut total = 0usize;
        add(&mut total, self.nodes.len())?;
        add(&mut total, self.prolog.len().saturating_add(self.epilog.len()).saturating_add(usize::from(self.root.is_some())).checked_mul(std::mem::size_of::<usize>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML root validation overflow"))?)?;
        add(&mut total, self.nodes.len().checked_mul(std::mem::size_of::<Option<XmlNode>>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML node materialization overflow"))?)?;
        add(&mut total, self.prolog.len().checked_mul(std::mem::size_of::<XmlNode>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML prolog materialization overflow"))?)?;
        add(&mut total, self.epilog.len().checked_mul(std::mem::size_of::<XmlNode>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML epilog materialization overflow"))?)?;
        for node in self.nodes.iter() {
            match &node.value {
                RetainedXmlNodeKind::Element { name, attribute_count, first_child, .. } => {
                    text(&mut total, name)?;
                    add(&mut total, attribute_count.checked_mul(std::mem::size_of::<XmlAttr>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML attribute materialization overflow"))?)?;
                    let mut children = 0usize;
                    let mut child = *first_child;
                    while let Some(ordinal) = child {
                        children = children.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML child materialization overflow"))?;
                        child = self.nodes.get(ordinal).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "retained XML child ordinal is out of range"))?.next_sibling;
                    }
                    add(&mut total, children.checked_mul(std::mem::size_of::<usize>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML child frontier overflow"))?)?;
                    add(&mut total, children.checked_mul(std::mem::size_of::<XmlNode>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML child owner overflow"))?)?;
                }
                RetainedXmlNodeKind::Text { text: value } | RetainedXmlNodeKind::CData { text: value } | RetainedXmlNodeKind::Comment { text: value } => text(&mut total, value)?,
                RetainedXmlNodeKind::ProcessingInstruction { target, data } => {
                    text(&mut total, target)?;
                    text(&mut total, data)?;
                }
            }
        }
        for attribute in self.attributes.iter() {
            text(&mut total, &attribute.name)?;
            text(&mut total, &attribute.value)?;
        }
        if let Some(doctype) = &self.doctype {
            text(&mut total, &doctype.name)?;
            add(&mut total, doctype.declarations.len().checked_mul(std::mem::size_of::<XmlDtdDeclaration>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained XML DTD materialization overflow"))?)?;
            if let Some(external) = &doctype.external_id {
                match external {
                    RetainedXmlExternalId::System { system_id } => text(&mut total, system_id)?,
                    RetainedXmlExternalId::Public { public_id, system_id } => {
                        text(&mut total, public_id)?;
                        text(&mut total, system_id)?;
                    }
                }
            }
            for declaration in doctype.declarations.iter() {
                match declaration {
                    RetainedXmlDtdDeclaration::Entity { name, value, .. } => {
                        text(&mut total, name)?;
                        text(&mut total, value)?;
                    }
                }
            }
        }
        if let Some(declaration) = &self.declaration {
            text(&mut total, &declaration.version)?;
            if let Some(encoding) = &declaration.encoding {
                text(&mut total, encoding)?;
            }
        }
        Ok(total)
    }

    pub fn materialize_exact(&self) -> Result<XmlDocument, ValueError> {
        let mut callback = |_| true;
        self.materialize(&mut NativeEncodeControl::new(self.materialization_owned_bytes()?, &mut callback))
    }

    pub fn materialize(&self, control: &mut NativeEncodeControl<'_>) -> Result<XmlDocument, ValueError> {
        control.scoped_stage(|control| {
            let mut incoming = control.allocate_vec::<u8>(self.nodes.len())?;
            incoming.resize(self.nodes.len(), 0);
            let root_count = self.prolog.len().saturating_add(self.epilog.len()).saturating_add(usize::from(self.root.is_some()));
            let roots = control.allocate_vec::<usize>(root_count)?;
            self.validate_owned(incoming, roots)?;
            control.begin_stage(self.nodes.len().saturating_add(self.attributes.len()).saturating_add(self.prolog.len()).saturating_add(self.epilog.len()))?;
            let mut built = control.allocate_vec::<Option<XmlNode>>(self.nodes.len())?;
            built.resize_with(self.nodes.len(), || None);
            for (ordinal, node) in self.nodes.iter().enumerate() {
                let value = match &node.value {
                    RetainedXmlNodeKind::Element { name, first_attribute, attribute_count, first_child } => {
                        let mut attrs = control.allocate_vec::<XmlAttr>(*attribute_count)?;
                        for position in *first_attribute..first_attribute + *attribute_count {
                            let attr = self.attributes.get(position).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "validated retained XML attribute vanished"))?;
                            attrs.push(XmlAttr { name: attr.name.to_string_owner_controlled(control)?, value: attr.value.to_string_owner_controlled(control)? });
                            control.step()?;
                        }
                        let mut child_count = 0usize;
                        let mut child = *first_child;
                        while let Some(position) = child {
                            child_count = child_count.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "retained XML child count overflow"))?;
                            child = self.nodes.get(position).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "validated retained XML child vanished"))?.next_sibling;
                        }
                        let mut child_ordinals = control.allocate_vec::<usize>(child_count)?;
                        child = *first_child;
                        while let Some(position) = child {
                            child_ordinals.push(position);
                            child = self.nodes.get(position).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "validated retained XML child vanished"))?.next_sibling;
                        }
                        let mut children = control.allocate_vec::<XmlNode>(child_ordinals.len())?;
                        for position in child_ordinals {
                            children.push(built.get_mut(position).and_then(Option::take).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained XML child materialization order is invalid"))?);
                        }
                        XmlNode::Element { name: name.to_string_owner_controlled(control)?, attrs, children }
                    }
                    RetainedXmlNodeKind::Text { text } => XmlNode::Text { text: text.to_string_owner_controlled(control)? },
                    RetainedXmlNodeKind::CData { text } => XmlNode::CData { text: text.to_string_owner_controlled(control)? },
                    RetainedXmlNodeKind::Comment { text } => XmlNode::Comment { text: text.to_string_owner_controlled(control)? },
                    RetainedXmlNodeKind::ProcessingInstruction { target, data } => XmlNode::ProcessingInstruction {
                        target: target.to_string_owner_controlled(control)?,
                        data: data.to_string_owner_controlled(control)?,
                    },
                };
                built[ordinal] = Some(value);
                control.step()?;
            }
            let root = self.root.map(|ordinal| built[ordinal].take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained XML root was consumed"))).transpose()?;
            let mut prolog = control.allocate_vec(self.prolog.len())?;
            for ordinal in self.prolog.iter().copied() {
                prolog.push(built[ordinal].take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained XML prolog node was consumed"))?);
            }
            let mut epilog = control.allocate_vec(self.epilog.len())?;
            for ordinal in self.epilog.iter().copied() {
                epilog.push(built[ordinal].take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained XML epilog node was consumed"))?);
            }
            if built.iter().any(Option::is_some) {
                return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "retained XML materialization left an unowned node"));
            }
            Ok(XmlDocument {
                root,
                doctype: self.doctype.as_ref().map(|value| value.materialize(control)).transpose()?,
                declaration: self.declaration.as_ref().map(|value| value.materialize(control)).transpose()?,
                prolog,
                epilog,
            })
        })
    }
}

impl RetainedXmlExternalId {
    fn try_from_external_id(source: &XmlExternalId) -> Result<Self, ValueError> {
        Ok(match source {
            XmlExternalId::System { system_id } => Self::System { system_id: text(system_id)? },
            XmlExternalId::Public { public_id, system_id } => Self::Public { public_id: text(public_id)?, system_id: text(system_id)? },
        })
    }

    fn try_from_external_id_controlled(source: &XmlExternalId, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        Ok(match source {
            XmlExternalId::System { system_id } => Self::System { system_id: RetainedXmlText::try_from_str_controlled(system_id, control)? },
            XmlExternalId::Public { public_id, system_id } => Self::Public {
                public_id: RetainedXmlText::try_from_str_controlled(public_id, control)?,
                system_id: RetainedXmlText::try_from_str_controlled(system_id, control)?,
            },
        })
    }

    fn materialize(&self, control: &mut NativeEncodeControl<'_>) -> Result<XmlExternalId, ValueError> {
        Ok(match self {
            Self::System { system_id } => XmlExternalId::System { system_id: system_id.to_string_owner_controlled(control)? },
            Self::Public { public_id, system_id } => XmlExternalId::Public {
                public_id: public_id.to_string_owner_controlled(control)?,
                system_id: system_id.to_string_owner_controlled(control)?,
            },
        })
    }
}

impl RetainedXmlDtdDeclaration {
    fn try_from_declaration(source: &XmlDtdDeclaration) -> Result<Self, ValueError> {
        match source {
            XmlDtdDeclaration::Entity { parameter, name, value } => Ok(Self::Entity { parameter: *parameter, name: text(name)?, value: text(value)? }),
        }
    }

    fn try_from_declaration_controlled(source: &XmlDtdDeclaration, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        match source {
            XmlDtdDeclaration::Entity { parameter, name, value } => Ok(Self::Entity {
                parameter: *parameter,
                name: RetainedXmlText::try_from_str_controlled(name, control)?,
                value: RetainedXmlText::try_from_str_controlled(value, control)?,
            }),
        }
    }

    fn materialize(&self, control: &mut NativeEncodeControl<'_>) -> Result<XmlDtdDeclaration, ValueError> {
        match self {
            Self::Entity { parameter, name, value } => Ok(XmlDtdDeclaration::Entity {
                parameter: *parameter,
                name: name.to_string_owner_controlled(control)?,
                value: value.to_string_owner_controlled(control)?,
            }),
        }
    }
}

impl RetainedXmlDoctype {
    fn try_from_doctype(source: &XmlDoctype) -> Result<Self, ValueError> {
        Ok(Self {
            prolog_position: source.prolog_position,
            name: text(&source.name)?,
            external_id: source.external_id.as_ref().map(RetainedXmlExternalId::try_from_external_id).transpose()?,
            declarations: RetainedXmlDtdDeclarations::try_from_fallible_iter(source.declarations.iter().map(RetainedXmlDtdDeclaration::try_from_declaration))?,
        })
    }

    fn try_from_doctype_controlled(source: &XmlDoctype, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let mut declarations = RetainedXmlDtdDeclarations::default();
        for declaration in &source.declarations {
            let declaration = RetainedXmlDtdDeclaration::try_from_declaration_controlled(declaration, control)?;
            append_controlled(&mut declarations, declaration, control)?;
        }
        Ok(Self {
            prolog_position: source.prolog_position,
            name: RetainedXmlText::try_from_str_controlled(&source.name, control)?,
            external_id: source.external_id.as_ref().map(|value| RetainedXmlExternalId::try_from_external_id_controlled(value, control)).transpose()?,
            declarations,
        })
    }

    fn materialize(&self, control: &mut NativeEncodeControl<'_>) -> Result<XmlDoctype, ValueError> {
        let mut declarations = control.allocate_vec(self.declarations.len())?;
        for declaration in self.declarations.iter() {
            declarations.push(declaration.materialize(control)?);
            control.step()?;
        }
        Ok(XmlDoctype {
            prolog_position: self.prolog_position,
            name: self.name.to_string_owner_controlled(control)?,
            external_id: self.external_id.as_ref().map(|value| value.materialize(control)).transpose()?,
            declarations,
        })
    }
}

impl RetainedXmlDeclaration {
    fn try_from_declaration(source: &XmlDeclaration) -> Result<Self, ValueError> {
        Ok(Self {
            version: text(&source.version)?,
            encoding: source.encoding.as_deref().map(text).transpose()?,
            standalone: source.standalone,
            quote: source.quote,
        })
    }

    fn try_from_declaration_controlled(source: &XmlDeclaration, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        Ok(Self {
            version: RetainedXmlText::try_from_str_controlled(&source.version, control)?,
            encoding: source.encoding.as_deref().map(|value| RetainedXmlText::try_from_str_controlled(value, control)).transpose()?,
            standalone: source.standalone,
            quote: source.quote,
        })
    }

    fn materialize(&self, control: &mut NativeEncodeControl<'_>) -> Result<XmlDeclaration, ValueError> {
        Ok(XmlDeclaration {
            version: self.version.to_string_owner_controlled(control)?,
            encoding: self.encoding.as_ref().map(|value| value.to_string_owner_controlled(control)).transpose()?,
            standalone: self.standalone,
            quote: self.quote,
        })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
