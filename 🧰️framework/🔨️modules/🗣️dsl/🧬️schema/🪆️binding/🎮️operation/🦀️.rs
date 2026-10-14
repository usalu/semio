//! 🎮️ Canonical operation text parses and prints through the actual Record grammar.

    use crate::{print, DslVariants, JoinMode, Limits, ParseOptions, SourceMode, TextError};

    pub fn parse_op<T: DslVariants>(line: &str) -> Result<T, TextError> {
        let variants = T::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword.as_str() || line.starts_with(&format!("{keyword} ")) {
                let record = crate::parse_exact(line, &(spec_fn.ordinary)(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;
                return T::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }

    pub fn print_op<T: DslVariants>(op: &T) -> String {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        print(&record, &(spec_fn.ordinary)(), JoinMode::Inline)
    }
