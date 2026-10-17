//! 🔢️ Exact values of the Part-21 codec: the decimal real, the value tree and the instance record. See [`super`].

use super::*;

//#region 🔖️Value
/// 🔢️ Exact logical STEP real: decimal coefficient/scale plus an optional base-10 exponent.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields )]
pub struct Part21Decimal {
    pub negative: bool,
    pub coefficient: String,
    pub scale: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub exponent: Option<i32>,
}

impl Part21Decimal {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn parse(text: &str) -> Result<Self, String> {
        let (negative, unsigned) = match text.as_bytes().first() {
            Some(b'-') => (true, &text[1..]),
            Some(b'+') => (false, &text[1..]),
            _ => (false, text),
        };
        let (mantissa, exponent) = match unsigned.find(['E', 'e']) {
            Some(index) => (&unsigned[..index], Some(unsigned[index + 1..].parse::<i32>().map_err(|e| e.to_string())?)),
            None => (unsigned, None),
        };
        let (integer, fraction) = mantissa.split_once('.').ok_or_else(|| format!("STEP real requires decimal point: {text:?}"))?;
        if integer.is_empty() || !integer.bytes().all(|b| b.is_ascii_digit()) || !fraction.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!("invalid STEP real: {text:?}"));
        }
        Ok(Self { negative, coefficient: format!("{integer}{fraction}"), scale: fraction.len() as u32, exponent })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_f64(value: f64) -> Self {
        let text = format!("{value}");
        let normalized = if text.contains('.') || text.contains('e') || text.contains('E') { text } else { format!("{text}.") };
        Self::parse(&normalized).expect("finite f64 has valid STEP decimal form")
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_f64(&self) -> Option<f64> {
        self.to_string().parse().ok()
    }
}

impl From<f64> for Part21Decimal {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
    }
}

impl fmt::Display for Part21Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            f.write_char('-')?;
        }
        let scale = self.scale as usize;
        if scale >= self.coefficient.len() {
            f.write_str("0.")?;
            for _ in 0..scale.saturating_sub(self.coefficient.len()) {
                f.write_char('0')?;
            }
            f.write_str(&self.coefficient)?;
        } else {
            let split = self.coefficient.len() - scale;
            f.write_str(&self.coefficient[..split])?;
            f.write_char('.')?;
            f.write_str(&self.coefficient[split..])?;
        }
        if let Some(exponent) = self.exponent {
            write!(f, "E{exponent}")?;
        }
        Ok(())
    }
}

/// 🔤️ A single typed value in Part-21 argument-list syntax.
#[derive(Clone, Debug, PartialEq,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned)]
pub enum Part21Value {
    Ref(u64),
    Str(String),
    Enum(String),
    Int(i64),
    Real(Part21Decimal),
    List(Vec<Part21Value>),
    /// 🏷️ A "defined type" wrapper appearing as an argument, e.g. `IFCLENGTHMEASURE(3000.)`.
    Typed {
        name: String,
        items: Vec<Part21Value>,
    },
    Unset,
    Derived,
}

impl Part21Value {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_ref_id(&self) -> Option<u64> {
        if let Part21Value::Ref(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_str(&self) -> Option<&str> {
        if let Part21Value::Str(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_enum(&self) -> Option<&str> {
        if let Part21Value::Enum(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_real(&self) -> Option<f64> {
        match self {
            Part21Value::Real(r) => r.to_f64(),
            Part21Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_list(&self) -> Option<&[Part21Value]> {
        if let Part21Value::List(items) = self {
            Some(items.as_slice())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_typed(&self) -> Option<(&str, &[Part21Value])> {
        if let Part21Value::Typed { name, items } = self {
            Some((name.as_str(), items.as_slice()))
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_unset(&self) -> bool {
        matches!(self, Part21Value::Unset)
    }
}
//#endregion 🔖️Value

//#region 🔖️Instance
/// 🧩️ One `#N = TYPE(args...)` line, or `#N = (TYPE1(...) TYPE2(...))` for a complex instance —
/// every `(type_name, args)` pair is kept, nothing about a multi-type instance is dropped.
#[derive(Clone, Debug, PartialEq, Default, semio_framework_value::RetainedClone)]
pub struct Part21Instance {
    pub id: u64,
    pub entities: Vec<(String, Vec<Part21Value>)>,
}

impl Part21Instance {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn entity(&self, type_name: &str) -> Option<&Vec<Part21Value>> {
        self.entities.iter().find(|(name, _)| name.eq_ignore_ascii_case(type_name)).map(|(_, args)| args)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn primary(&self) -> Option<(&str, &Vec<Part21Value>)> {
        self.entities.first().map(|(name, args)| (name.as_str(), args))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_type(&self, type_name: &str) -> bool {
        self.entities.iter().any(|(name, _)| name.eq_ignore_ascii_case(type_name))
    }
}
//#endregion 🔖️Instance
