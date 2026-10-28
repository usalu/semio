//! 🎮️ Canonical operation text parses and prints through the actual Record grammar.

    use crate::{print, DslVariants, JoinMode, Limits, ParseOptions, SourceMode, TextError};

    pub fn parse_op<T: DslVariants>(line: &str) -> Result<T, TextError> {
        let variants = T::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword.as_str() || line.starts_with(&format!("{keyword} ")) {
                let spec=(spec_fn.ordinary)();let body=if spec.keyword.is_none(){line.strip_prefix(keyword).unwrap().trim_start()}else{line};
                let record=crate::parse_exact(body,&spec,&ParseOptions{limits:Limits::default(),mode:SourceMode::Inline})?;
                return T::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }

    pub fn print_op<T: DslVariants>(op: &T) -> String {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        let spec=(spec_fn.ordinary)();let body=print(&record,&spec,JoinMode::Inline);if spec.keyword.is_none(){if body.is_empty(){keyword}else{format!("{keyword} {body}")}}else{body}
    }
