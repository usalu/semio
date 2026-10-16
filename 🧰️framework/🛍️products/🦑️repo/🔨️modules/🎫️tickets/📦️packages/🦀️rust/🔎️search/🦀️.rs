//! 🔎️ The ticket query.

use super::*;

//#region 🔎️Search

/// 🔍️ What narrows a ticket listing.
///
/// Regex filtering is deliberately absent: it needs a regular-expression engine, which this crate
/// does not own and may not import. A caller that needs it pre-filters and passes the surviving
/// slugs in `slugs`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketQuery {
    #[serde(default)]
    pub filter: String,
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub match_case: bool,
    #[serde(default)]
    pub match_whole_word: bool,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub include_years: Vec<i64>,
    #[serde(default)]
    pub exclude_years: Vec<i64>,
    #[serde(default)]
    pub slugs: Vec<String>,
}

impl TicketQuery {
    /// ✔️ Whether a ticket survives the query.
    pub fn matches(&self, ticket: &Ticket) -> bool {
        if let Some(status) = &self.status {
            if ticket.status.as_str() != status {
                return false;
            }
        }
        if !self.include_years.is_empty() && !self.include_years.contains(&ticket.year) {
            return false;
        }
        if self.exclude_years.contains(&ticket.year) {
            return false;
        }
        if !self.slugs.is_empty() && !self.slugs.contains(&ticket.slug) {
            return false;
        }
        let identity = TicketId::new(ticket.year, ticket.month, ticket.day, ticket.slug.clone()).id();
        if !self.filter.is_empty() && ![identity.as_str(), ticket.slug.as_str(), ticket.title.as_str()].iter().any(|candidate| self.matches_filter(candidate)) {
            return false;
        }
        if !self.query.is_empty() {
            let haystack = format!("{identity} {} {} {} {}", ticket.slug, ticket.title, ticket.description, ticket.status.as_str());
            if !haystack.to_lowercase().contains(&self.query.to_lowercase()) {
                return false;
            }
        }
        true
    }

    fn matches_filter(&self, name: &str) -> bool {
        let (target, pattern) = if self.match_case { (name.to_string(), self.filter.clone()) } else { (name.to_lowercase(), self.filter.to_lowercase()) };
        if self.match_whole_word {
            return target.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_')).any(|word| word == pattern);
        }
        target.contains(&pattern)
    }
}

//#endregion 🔎️Search
