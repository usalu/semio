//! 🧭️ Member keys and paths of the child members: the owned document member preparation size, the member step and path with their parsing and display. See [`super`].

use super::*;

#[doc(hidden)]
pub const OWNED_DOCUMENT_MEMBER_PREPARATION_BYTES: usize = store::MEMBER_OPEN_IDENTITY_BYTES * 8;

/// 👣️ One step of a [`MemberPath`]: the slot a member is declared in and its child id there.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,semio_framework_value::RetireOwned)]
pub struct MemberStep {
    pub slot: String,
    pub child_id: String,
}

/// 🛤️ The owner path of one composed member — every step from the document down to it — as hosts, agents and history
/// rows address it (the `store` of a history row, a tool run's member, an inference dependency): `slot/childId` per
/// step, steps joined by `/`, `%25` for `%` and `%2F` for `/` inside a component. A member of the document whose id
/// holds neither reads exactly `<slot>/<childId>`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,semio_framework_value::RetireOwned)]
pub struct MemberPath(pub(super) Vec<MemberStep>);

impl MemberPath {
    pub fn take_original(&mut self)->Self{std::mem::replace(self,Self(Vec::new()))}

    /// 🌲️ The path of a member the document itself owns.
    pub fn root(slot: impl Into<String>, child_id: impl Into<String>) -> Self {
        Self(vec![MemberStep { slot: slot.into(), child_id: child_id.into() }])
    }

    /// 🪵️ The path of the member this one owns at `(slot, child_id)`.
    pub fn join(mut self, slot: impl Into<String>, child_id: impl Into<String>) -> Self {
        self.0.push(MemberStep { slot: slot.into(), child_id: child_id.into() });
        self
    }

    /// 🧗️ The text of the owner's path — every step but the member's own; empty for a member of the document.
    pub fn owner_text(&self) -> String {
        Self(self.0[..self.0.len().saturating_sub(1)].to_vec()).to_string()
    }

    /// 🪢️ The path of the member at `(slot, child_id)` under the owner whose path text is `owner` (empty: the document).
    pub fn under(owner: &str, slot: impl Into<String>, child_id: impl Into<String>) -> Option<Self> {
        let owner = if owner.is_empty() { Self(Vec::new()) } else { Self::parse(owner)? };
        Some(owner.join(slot, child_id))
    }

    /// 🥾️ Every step from the document down to the member.
    pub fn steps(&self) -> &[MemberStep] {
        &self.0
    }

    /// 📜️ Reads the canonical text; `None` for anything [`std::fmt::Display`] never writes.
    pub fn parse(text: &str) -> Option<Self> {
        let mut components = Vec::new();
        for component in text.split('/') {
            components.push(Self::unescape(component)?);
        }
        if components.len() % 2 != 0 {
            return None;
        }
        let mut steps = Vec::with_capacity(components.len() / 2);
        let mut components = components.into_iter();
        while let (Some(slot), Some(child_id)) = (components.next(), components.next()) {
            steps.push(MemberStep { slot, child_id });
        }
        Some(Self(steps))
    }

    fn unescape(component: &str) -> Option<String> {
        if component.is_empty() {
            return None;
        }
        let mut text = String::with_capacity(component.len());
        let mut rest = component;
        while let Some(position) = rest.find('%') {
            text.push_str(&rest[..position]);
            match rest.get(position..position + 3)? {
                "%25" => text.push('%'),
                "%2F" => text.push('/'),
                _ => return None,
            }
            rest = &rest[position + 3..];
        }
        text.push_str(rest);
        Some(text)
    }
}

impl std::fmt::Display for MemberPath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, component) in self.0.iter().flat_map(|step| [step.slot.as_str(), step.child_id.as_str()]).enumerate() {
            if index != 0 {
                formatter.write_str("/")?;
            }
            for character in component.chars() {
                match character {
                    '%' => formatter.write_str("%25")?,
                    '/' => formatter.write_str("%2F")?,
                    other => std::fmt::Write::write_char(formatter, other)?,
                }
            }
        }
        Ok(())
    }
}
