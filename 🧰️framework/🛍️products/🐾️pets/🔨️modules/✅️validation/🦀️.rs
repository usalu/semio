//! ✅️ Validation of menageries, species and ensembles without any schema library — the constraints serde cannot express and the document rules of the design (§3) — and the assembly of a menagerie from its ensemble.
//!
//! A finding carries a JSON Pointer (RFC 6901) into the validated document and the kebab-case code of the TypeScript
//! twin. Findings are returned deduplicated and sorted by path, then code, in code point order (UTF-8 byte order).
//! The twin judges untyped JSON; here a document is decoded into its typed twin first, so the structural codes
//! `required`, `property-unknown`, a wrong JSON type, an enumeration mismatch and a tuple of the wrong length
//! surface as serde errors instead. What is left of the structure is checked here: `type-invalid` (a number that is
//! not finite), `value-invalid` (the `schema` identifier), `slug-invalid`, `length-invalid` and `items-too-few`. A
//! value that fails its structure is not judged any further, and an id that is not a slug does not count as declared.
//! Then the document rules follow: `duplicate-id`, `unknown-reference`, `bone-order`, `key-order`, `loop-seam`,
//! `ease-range`, `out-of-range`, `self-bond`, `duplicate-bond`, `missing-gait-clip`, `float-hover`, `empty-cast`,
//! `duplicate-scene`.
//!
//! @see <https://www.rfc-editor.org/rfc/rfc6901> — JSON Pointer
//! @see ../../🧬️schema/🔣️.json — the structure mirrored here
//! @see ../../README.md — the table of codes, the pointer each one is reported at and the rule behind it
//! @see ../✅️validation/🟦️.ts — the TypeScript twin

use crate::schema::{Activity, Bond, Bone, Cast, Channel, Clip, Ensemble, Face, Gait, Key, Locomotion, Menagerie, Part, Repertoire, Shape, Species, Text, Track, ACTIVITIES, ENSEMBLE_SCHEMA, MENAGERIE_SCHEMA};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Display;

//#region 🔖️Findings
/// 🚥️ What is wrong at a path — the shared vocabulary of both cores.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IssueCode {
    TypeInvalid,
    Required,
    PropertyUnknown,
    ValueInvalid,
    SlugInvalid,
    LengthInvalid,
    ItemsTooFew,
    DuplicateId,
    UnknownReference,
    BoneOrder,
    KeyOrder,
    LoopSeam,
    EaseRange,
    OutOfRange,
    SelfBond,
    DuplicateBond,
    MissingGaitClip,
    FloatHover,
    EmptyCast,
    DuplicateScene,
}

impl IssueCode {
    /// 💬️ The wire string, e.g. `duplicate-id`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TypeInvalid => "type-invalid",
            Self::Required => "required",
            Self::PropertyUnknown => "property-unknown",
            Self::ValueInvalid => "value-invalid",
            Self::SlugInvalid => "slug-invalid",
            Self::LengthInvalid => "length-invalid",
            Self::ItemsTooFew => "items-too-few",
            Self::DuplicateId => "duplicate-id",
            Self::UnknownReference => "unknown-reference",
            Self::BoneOrder => "bone-order",
            Self::KeyOrder => "key-order",
            Self::LoopSeam => "loop-seam",
            Self::EaseRange => "ease-range",
            Self::OutOfRange => "out-of-range",
            Self::SelfBond => "self-bond",
            Self::DuplicateBond => "duplicate-bond",
            Self::MissingGaitClip => "missing-gait-clip",
            Self::FloatHover => "float-hover",
            Self::EmptyCast => "empty-cast",
            Self::DuplicateScene => "duplicate-scene",
        }
    }
}

/// 🩺️ One finding: where (JSON pointer) and what (kebab-case code).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Issue {
    pub path: String,
    pub code: IssueCode,
}

const SLUG_MAX: usize = 64;
const SEAM_SLACK: f64 = 1e-9;

/// 🧷️ A JSON pointer one step below `base`, escaping `~` and `/`.
fn at(base: &str, key: impl Display) -> String {
    format!("{base}/{}", key.to_string().replace('~', "~0").replace('/', "~1"))
}

/// 🏷️ Whether `value` is a slug of 1…64 characters: `^[a-z0-9]+(?:-[a-z0-9]+)*$`.
fn is_slug(value: &str) -> bool {
    value.len() <= SLUG_MAX && value.split('-').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

/// 🎨️ Whether `value` is a colour: `^#[0-9a-f]{6}$`.
fn is_color(value: &str) -> bool {
    value.len() == 7 && value.starts_with('#') && value.bytes().skip(1).all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// 🧹️ The findings gathered while a document is walked.
#[derive(Default)]
struct Findings {
    issues: Vec<Issue>,
}

impl Findings {
    /// 📣️ Records a finding.
    fn report(&mut self, path: &str, code: IssueCode) {
        self.issues.push(Issue { path: path.to_string(), code });
    }

    /// 🧮️ The findings deduplicated and sorted by path, then code, in code point order.
    fn finish(mut self) -> Vec<Issue> {
        self.issues.sort_by(|left, right| left.path.cmp(&right.path).then_with(|| left.code.as_str().cmp(right.code.as_str())));
        self.issues.dedup();
        self.issues
    }

    /// 🔡️ A string of at least `min` code points.
    fn string(&mut self, value: &str, path: &str, min: usize) -> bool {
        if value.chars().count() < min {
            self.report(path, IssueCode::LengthInvalid);
            return false;
        }
        true
    }

    /// 🔖️ A slug; hands it on when valid.
    fn slug<'a>(&mut self, value: &'a str, path: &str) -> Option<&'a str> {
        if !is_slug(value) {
            self.report(path, IssueCode::SlugInvalid);
            return None;
        }
        Some(value)
    }

    /// 🔢️ A finite number; hands it on when valid.
    fn number(&mut self, value: f64, path: &str) -> Option<f64> {
        if !value.is_finite() {
            self.report(path, IssueCode::TypeInvalid);
            return None;
        }
        Some(value)
    }

    /// 📏️ A number above zero (`or_zero`: zero or above); hands it on when valid.
    fn positive(&mut self, value: f64, path: &str, or_zero: bool) -> Option<f64> {
        let valid = self.number(value, path)?;
        let outside = if or_zero { valid < 0.0 } else { valid <= 0.0 };
        if outside {
            self.report(path, IssueCode::OutOfRange);
            return None;
        }
        Some(valid)
    }

    /// 🎚️ A number in `[low, high]`.
    fn within(&mut self, value: f64, path: &str, low: f64, high: f64) {
        if let Some(valid) = self.number(value, path) {
            if valid < low || valid > high {
                self.report(path, IssueCode::OutOfRange);
            }
        }
    }

    /// 🌍️ A text in every language.
    fn text(&mut self, value: &Text, path: &str) {
        self.string(&value.en, &at(path, "en"), 1);
        self.string(&value.de, &at(path, "de"), 1);
    }

    /// 👯️ Reports `duplicate-id` at every entry that repeats an earlier one.
    fn unique(&mut self, ids: &[Option<&str>], path: &str, key: Option<&str>) {
        let mut seen = BTreeSet::new();
        for (index, id) in ids.iter().enumerate() {
            let Some(id) = id else { continue };
            if !seen.insert(*id) {
                let entry = at(path, index);
                let repeated = match key {
                    Some(key) => at(&entry, key),
                    None => entry,
                };
                self.report(&repeated, IssueCode::DuplicateId);
            }
        }
    }

    /// 🔗️ A slug that must be one of `known`; reports `unknown-reference` otherwise.
    fn reference<'a>(&mut self, value: &'a str, path: &str, known: Option<&BTreeSet<&str>>) -> Option<&'a str> {
        let valid = self.slug(value, path)?;
        if known.is_some_and(|known| !known.contains(valid)) {
            self.report(path, IssueCode::UnknownReference);
        }
        Some(valid)
    }
}
//#endregion 🔖️Findings

//#region 🔖️Species
/// 🦴️ The bones of a rig: parents first, exactly the first without a parent; hands the bone ids on.
fn bones<'a>(value: &'a [Bone], path: &str, findings: &mut Findings) -> BTreeSet<&'a str> {
    let ids: Vec<Option<&str>> = value
        .iter()
        .enumerate()
        .map(|(index, bone)| {
            let entry = at(path, index);
            findings.number(bone.x, &at(&entry, "x"));
            findings.number(bone.y, &at(&entry, "y"));
            if let Some(rotation) = bone.rotation {
                findings.number(rotation, &at(&entry, "rotation"));
            }
            if let Some(parent) = &bone.parent {
                findings.slug(parent, &at(&entry, "parent"));
            }
            findings.slug(&bone.id, &at(&entry, "id"))
        })
        .collect();
    if ids.is_empty() {
        findings.report(path, IssueCode::ItemsTooFew);
    }
    findings.unique(&ids, path, Some("id"));
    for (index, bone) in value.iter().enumerate() {
        let Some(parent) = &bone.parent else {
            if index > 0 {
                findings.report(&at(path, index), IssueCode::BoneOrder);
            }
            continue;
        };
        if !is_slug(parent) {
            continue;
        }
        let first = ids.iter().position(|id| *id == Some(parent.as_str()));
        if first.is_none() {
            findings.report(&at(&at(path, index), "parent"), IssueCode::UnknownReference);
        }
        if index == 0 || first.is_some_and(|first| first >= index) {
            findings.report(&at(&at(path, index), "parent"), IssueCode::BoneOrder);
        }
    }
    ids.into_iter().flatten().collect()
}

/// 🔷️ The geometry of a part: the variant `kind` names, with positive extents.
fn shape(value: &Shape, path: &str, findings: &mut Findings) {
    match value {
        Shape::Path(outline) => {
            findings.string(&outline.d, &at(path, "d"), 1);
        }
        Shape::Ellipse(ellipse) => {
            findings.number(ellipse.cx, &at(path, "cx"));
            findings.number(ellipse.cy, &at(path, "cy"));
            findings.positive(ellipse.rx, &at(path, "rx"), false);
            findings.positive(ellipse.ry, &at(path, "ry"), false);
        }
        Shape::Rect(rect) => {
            findings.number(rect.x, &at(path, "x"));
            findings.number(rect.y, &at(path, "y"));
            findings.positive(rect.width, &at(path, "width"), false);
            findings.positive(rect.height, &at(path, "height"), false);
            if let Some(radius) = rect.radius {
                findings.positive(radius, &at(path, "radius"), true);
            }
        }
        Shape::Line(line) => {
            findings.number(line.x1, &at(path, "x1"));
            findings.number(line.y1, &at(path, "y1"));
            findings.number(line.x2, &at(path, "x2"));
            findings.number(line.y2, &at(path, "y2"));
        }
    }
}

/// 🧩️ The parts of a species, each on a bone of the rig; hands the part ids on.
fn parts<'a>(value: &'a [Part], path: &str, findings: &mut Findings, bone_ids: &BTreeSet<&str>) -> BTreeSet<&'a str> {
    let ids: Vec<Option<&str>> = value
        .iter()
        .enumerate()
        .map(|(index, part)| {
            let entry = at(path, index);
            findings.reference(&part.bone, &at(&entry, "bone"), Some(bone_ids));
            shape(&part.shape, &at(&entry, "shape"), findings);
            if let Some(width) = part.stroke_width {
                findings.positive(width, &at(&entry, "strokeWidth"), false);
            }
            findings.slug(&part.id, &at(&entry, "id"))
        })
        .collect();
    findings.unique(&ids, path, Some("id"));
    ids.into_iter().flatten().collect()
}

/// 🙂️ The face: eyes whose pupil is smaller than their white, an optional mouth, and the part it is drawn above.
fn face(value: &Face, path: &str, findings: &mut Findings, bone_ids: &BTreeSet<&str>, part_ids: &BTreeSet<&str>) {
    let eyes = at(path, "eyes");
    let ids: Vec<Option<&str>> = value
        .eyes
        .iter()
        .enumerate()
        .map(|(index, eye)| {
            let entry = at(&eyes, index);
            findings.reference(&eye.bone, &at(&entry, "bone"), Some(bone_ids));
            findings.number(eye.x, &at(&entry, "x"));
            findings.number(eye.y, &at(&entry, "y"));
            let radius = findings.positive(eye.radius, &at(&entry, "radius"), false);
            let pupil = findings.positive(eye.pupil, &at(&entry, "pupil"), false);
            if let (Some(radius), Some(pupil)) = (radius, pupil) {
                if pupil >= radius {
                    findings.report(&at(&entry, "pupil"), IssueCode::OutOfRange);
                }
            }
            findings.slug(&eye.id, &at(&entry, "id"))
        })
        .collect();
    findings.unique(&ids, &eyes, Some("id"));
    if let Some(mouth) = &value.mouth {
        let entry = at(path, "mouth");
        findings.reference(&mouth.bone, &at(&entry, "bone"), Some(bone_ids));
        findings.number(mouth.x, &at(&entry, "x"));
        findings.number(mouth.y, &at(&entry, "y"));
        findings.positive(mouth.width, &at(&entry, "width"), false);
    }
    if let Some(above) = &value.above {
        findings.reference(above, &at(path, "above"), Some(part_ids));
    }
}

/// 🔑️ One key: its phase, its value and an optional easing whose abscissas lie in [0, 1]; hands phase and value on.
fn keyframe(value: &Key, path: &str, findings: &mut Findings) -> (Option<f64>, Option<f64>) {
    if let Some(ease) = &value.ease {
        let ease_path = at(path, "ease");
        let entries: Vec<Option<f64>> = ease.iter().enumerate().map(|(index, field)| findings.number(*field, &at(&ease_path, index))).collect();
        for index in [0, 2] {
            if entries[index].is_some_and(|entry| !(0.0..=1.0).contains(&entry)) {
                findings.report(&at(&ease_path, index), IssueCode::EaseRange);
            }
        }
    }
    (findings.number(value.at, &at(path, "at")), findings.number(value.value, &at(path, "value")))
}

/// 🛤️ One track: at least two keys in strictly ascending phase from 0 to 1 and, in a looping clip, a seamless end — the same value at both ends, or on the rotation channel a difference of a whole number of turns, judged within 1e-9 of a turn (authored decimals such as −359.8 and −719.8 are a turn apart, and the quotient of their difference is not an exact integer).
fn track(value: &Track, path: &str, findings: &mut Findings, bone_ids: &BTreeSet<&str>, looping: bool) {
    findings.reference(&value.bone, &at(path, "bone"), Some(bone_ids));
    let keys_path = at(path, "keys");
    let keys: Vec<(Option<f64>, Option<f64>)> = value.keys.iter().enumerate().map(|(index, entry)| keyframe(entry, &at(&keys_path, index), findings)).collect();
    for (index, (phase, _)) in keys.iter().enumerate() {
        if phase.is_some_and(|phase| !(0.0..=1.0).contains(&phase)) {
            findings.report(&at(&at(&keys_path, index), "at"), IssueCode::KeyOrder);
        }
    }
    if keys.len() < 2 {
        findings.report(&keys_path, IssueCode::KeyOrder);
        return;
    }
    let last = keys.len() - 1;
    for (index, (phase, _)) in keys.iter().enumerate() {
        let Some(phase) = *phase else { continue };
        let before = if index > 0 { keys[index - 1].0 } else { None };
        if (index == 0 && phase != 0.0) || (index == last && phase != 1.0) || before.is_some_and(|before| phase <= before) {
            findings.report(&at(&at(&keys_path, index), "at"), IssueCode::KeyOrder);
        }
    }
    let (Some(from), Some(to)) = (keys[0].1, keys[last].1) else { return };
    if !looping || from == to {
        return;
    }
    let revolutions = (to - from) / 360.0;
    let apart = revolutions - (revolutions + 0.5).floor();
    if value.channel != Channel::Rotation || apart > SEAM_SLACK || apart < 0.0 - SEAM_SLACK {
        findings.report(&at(&at(&keys_path, last), "value"), IssueCode::LoopSeam);
    }
}

/// 🎞️ The clips of a species; hands the clip ids on.
fn clips<'a>(value: &'a [Clip], path: &str, findings: &mut Findings, bone_ids: &BTreeSet<&str>) -> BTreeSet<&'a str> {
    let ids: Vec<Option<&str>> = value
        .iter()
        .enumerate()
        .map(|(index, clip)| {
            let entry = at(path, index);
            findings.positive(clip.seconds, &at(&entry, "seconds"), false);
            let tracks = at(&entry, "tracks");
            for (member, entry) in clip.tracks.iter().enumerate() {
                track(entry, &at(&tracks, member), findings, bone_ids, clip.looping);
            }
            findings.slug(&clip.id, &at(&entry, "id"))
        })
        .collect();
    findings.unique(&ids, path, Some("id"));
    ids.into_iter().flatten().collect()
}

/// 🗃️ The repertoire: clip ids per activity; hands the activities that have at least one clip on.
fn repertoire(value: &Repertoire, path: &str, findings: &mut Findings, clip_ids: &BTreeSet<&str>) -> Vec<Activity> {
    let mut played = Vec::new();
    for activity in ACTIVITIES {
        let Some(listed) = value.clips(activity) else { continue };
        let entries = at(path, activity.as_str());
        for (index, clip) in listed.iter().enumerate() {
            findings.reference(clip, &at(&entries, index), Some(clip_ids));
        }
        if !listed.is_empty() {
            played.push(activity);
        }
    }
    played
}

/// 🏃️ The locomotion: a positive speed, a hover exactly when floating, and a clip for the gait of walkers and hoppers.
fn locomotion(value: &Locomotion, path: &str, findings: &mut Findings, played: &[Activity]) {
    findings.positive(value.speed, &at(path, "speed"), false);
    if let Some(hover) = value.hover {
        findings.positive(hover, &at(path, "hover"), false);
    }
    if (value.gait == Gait::Float) != value.hover.is_some() {
        findings.report(&at(path, "hover"), IssueCode::FloatHover);
    }
    let needed = match value.gait {
        Gait::Walk => Some(Activity::Walk),
        Gait::Hop => Some(Activity::Hop),
        Gait::Float => None,
    };
    if needed.is_some_and(|needed| !played.contains(&needed)) {
        findings.report(&at(path, "gait"), IssueCode::MissingGaitClip);
    }
}

/// 🧬️ A species; hands its id on when that is a slug.
fn species<'a>(value: &'a Species, path: &str, findings: &mut Findings) -> Option<&'a str> {
    findings.text(&value.name, &at(path, "name"));
    findings.text(&value.thing, &at(path, "thing"));
    let grounds = at(path, "grounds");
    for (index, ground) in value.grounds.iter().enumerate() {
        findings.string(ground, &at(&grounds, index), 1);
    }
    let size = at(path, "size");
    findings.positive(value.size.width, &at(&size, "width"), false);
    findings.positive(value.size.height, &at(&size, "height"), false);
    let palette = at(path, "palette");
    for (name, color) in [("body", &value.palette.body), ("accent", &value.palette.accent), ("detail", &value.palette.detail)] {
        if !is_color(color) {
            findings.report(&at(&palette, name), IssueCode::OutOfRange);
        }
    }
    let bone_ids = bones(&value.bones, &at(path, "bones"), findings);
    let part_ids = parts(&value.parts, &at(path, "parts"), findings, &bone_ids);
    face(&value.face, &at(path, "face"), findings, &bone_ids, &part_ids);
    let clip_ids = clips(&value.clips, &at(path, "clips"), findings, &bone_ids);
    let played = repertoire(&value.repertoire, &at(path, "repertoire"), findings, &clip_ids);
    locomotion(&value.locomotion, &at(path, "locomotion"), findings, &played);
    let temperament = at(path, "temperament");
    for (name, trait_value) in [("energy", value.temperament.energy), ("sociability", value.temperament.sociability), ("curiosity", value.temperament.curiosity)] {
        findings.within(trait_value, &at(&temperament, name), 0.0, 1.0);
    }
    findings.slug(&value.id, &at(path, "id"))
}
//#endregion 🔖️Species

//#region 🔖️Documents
/// 🤝️ The bonds: two different species each, every unordered pair at most once, an affinity in [−1, 1].
fn bonds(value: &[Bond], path: &str, findings: &mut Findings, species_ids: Option<&BTreeSet<&str>>) {
    let mut pairs = BTreeSet::new();
    for (index, bond) in value.iter().enumerate() {
        let entry = at(path, index);
        findings.within(bond.affinity, &at(&entry, "affinity"), -1.0, 1.0);
        let between = at(&entry, "between");
        let left = findings.reference(&bond.between[0], &at(&between, 0), species_ids);
        let right = findings.reference(&bond.between[1], &at(&between, 1), species_ids);
        let (Some(left), Some(right)) = (left, right) else { continue };
        if left == right {
            findings.report(&between, IssueCode::SelfBond);
            continue;
        }
        if !pairs.insert(if left < right { (left, right) } else { (right, left) }) {
            findings.report(&between, IssueCode::DuplicateBond);
        }
    }
}

/// 🎟️ The casts: unique scenes, each with at least one core species.
fn casts(value: &[Cast], path: &str, findings: &mut Findings, species_ids: Option<&BTreeSet<&str>>) {
    let mut seen = BTreeSet::new();
    for (index, cast) in value.iter().enumerate() {
        let entry = at(path, index);
        for (list, members) in [("core", &cast.core), ("rotation", &cast.rotation)] {
            let entries = at(&entry, list);
            for (member, id) in members.iter().enumerate() {
                findings.reference(id, &at(&entries, member), species_ids);
            }
            if list == "core" && members.is_empty() {
                findings.report(&entries, IssueCode::EmptyCast);
            }
        }
        let Some(scene) = findings.slug(&cast.scene, &at(&entry, "scene")) else { continue };
        if !seen.insert(scene) {
            findings.report(&at(&entry, "scene"), IssueCode::DuplicateScene);
        }
    }
}

/// 📇️ The head every document shares: its schema identifier, id and title.
fn head(schema: &str, id: &str, title: &Text, findings: &mut Findings, expected: &str) {
    if schema != expected {
        findings.report("/schema", IssueCode::ValueInvalid);
    }
    findings.slug(id, "/id");
    findings.text(title, "/title");
}

/// 🧪️ The findings of one species document; none when it is valid.
pub fn species_issues(document: &Species) -> Vec<Issue> {
    let mut findings = Findings::default();
    species(document, "", &mut findings);
    findings.finish()
}

/// 🎪️ The findings of a menagerie document: its species, their unique ids, and bonds and casts that name them.
pub fn menagerie_issues(document: &Menagerie) -> Vec<Issue> {
    let mut findings = Findings::default();
    head(&document.schema, &document.id, &document.title, &mut findings, MENAGERIE_SCHEMA);
    let ids: Vec<Option<&str>> = document.species.iter().enumerate().map(|(index, entry)| species(entry, &at("/species", index), &mut findings)).collect();
    findings.unique(&ids, "/species", Some("id"));
    let known: BTreeSet<&str> = ids.into_iter().flatten().collect();
    bonds(&document.bonds, "/bonds", &mut findings, Some(&known));
    casts(&document.casts, "/casts", &mut findings, Some(&known));
    findings.finish()
}

/// 🗂️ The findings of an ensemble document: unique species paths, bonds and casts (whose species ids resolve only in the assembled menagerie).
pub fn ensemble_issues(document: &Ensemble) -> Vec<Issue> {
    let mut findings = Findings::default();
    head(&document.schema, &document.id, &document.title, &mut findings, ENSEMBLE_SCHEMA);
    let paths: Vec<Option<&str>> = document.species.iter().enumerate().map(|(index, path)| findings.string(path, &at("/species", index), 1).then_some(path.as_str())).collect();
    findings.unique(&paths, "/species", None);
    bonds(&document.bonds, "/bonds", &mut findings, None);
    casts(&document.casts, "/casts", &mut findings, None);
    findings.finish()
}

/// 🧺️ The menagerie an ensemble describes, given its species documents in the order of the ensemble's paths; a species' `$schema` hint stays behind.
pub fn assemble_menagerie(ensemble: &Ensemble, species: &[Species]) -> Menagerie {
    let members = species.iter().map(|member| Species { json_schema: None, ..member.clone() }).collect();
    Menagerie { json_schema: None, schema: MENAGERIE_SCHEMA.to_string(), id: ensemble.id.clone(), title: ensemble.title.clone(), species: members, bonds: ensemble.bonds.clone(), casts: ensemble.casts.clone() }
}
//#endregion 🔖️Documents

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
