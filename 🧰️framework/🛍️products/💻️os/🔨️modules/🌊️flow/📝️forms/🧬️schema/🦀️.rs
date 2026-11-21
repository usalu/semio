//! 📝️ Original Flow widget field loans supplied to an explicitly selected form producer.
use semio_framework_value::DslValue;

/// 🔤️ Registered Flow variable schemas with an editable generation answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowVariableAnswerSchema {
    Text,
    Number,
    Boolean,
    Point,
    Vector,
}

impl FlowVariableAnswerSchema {
    pub const AXES: [&'static str; 3] = ["x", "y", "z"];

    pub fn from_schema_id(id: &str) -> Option<Self> {
        match id {
            "text" => Some(Self::Text),
            "number" => Some(Self::Number),
            "boolean" => Some(Self::Boolean),
            "point" => Some(Self::Point),
            "vector" => Some(Self::Vector),
            _ => None,
        }
    }

    pub fn default_answer(self) -> DslValue {
        match self {
            Self::Text => DslValue::String(String::new()),
            Self::Number => DslValue::float(0.0),
            Self::Boolean => DslValue::Bool(false),
            Self::Point | Self::Vector => DslValue::Array(vec![DslValue::float(0.0); Self::AXES.len()]),
        }
    }

    pub fn accepts(self, value: &DslValue) -> bool {
        let finite_number = |value: &DslValue| value.as_f64().is_some_and(f64::is_finite);
        match self {
            Self::Text => value.as_str().is_some(),
            Self::Number => finite_number(value),
            Self::Boolean => value.as_bool().is_some(),
            Self::Point | Self::Vector => value.as_array().is_some_and(|axes| axes.len() == Self::AXES.len() && axes.iter().all(finite_number)),
        }
    }
}

/// 🏭️ Owns the actual child field and form constructions independently of Flow's snapshot source.
pub trait FlowFormProducer {
 type Field;
 type Form;
 fn slider(&self,id:&str,label:&str,value:f64,min:f64,max:f64,step:f64)->Self::Field;
 fn note(&self,id:&str,text:&str)->Self::Field;
 fn image(&self,id:&str,source:&str)->Self::Field;
 fn variable(&self,id:&str,name:&str,schema:&str)->Option<Self::Field>;
 fn finish(&self,fields:Vec<Self::Field>)->Self::Form;
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
