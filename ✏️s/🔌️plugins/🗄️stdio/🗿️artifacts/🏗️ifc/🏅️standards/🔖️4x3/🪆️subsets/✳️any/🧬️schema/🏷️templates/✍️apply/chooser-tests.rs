//! 🔽️ The real addressed template action exposes the entire release without an implicit choice.
use super::*;
use semio_framework_plugin::{ActionArgControl, ArgSchema};
use semio_framework_ui_locale::{Locale, Terminology};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn released_template_action_choices_are_complete_explicit_and_localized() {
    let contract: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("chooser.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let action = action();
    let argument = action.args.iter().find(|argument| argument.id == contract["argument"].as_str().unwrap()).unwrap();
    assert_eq!(argument.required, contract["required"].as_bool().unwrap());
    assert_eq!(argument.default.is_some(), contract["automaticSelection"].as_bool().unwrap());
    let ActionArgControl::Select { options } = argument.control() else { panic!("the released template argument must be a select control") };
    assert_eq!(options.len() as u64, contract["choices"].as_u64().unwrap());
    let catalog = crate::templates::ifc4x3_template_catalog();
    assert_eq!(options.len(), catalog.templates.len());
    assert_eq!(options.iter().map(|option| &option.value).collect::<BTreeSet<_>>().len(), options.len());
    for (locale, key) in [(Locale::En, "en"), (Locale::De, "de")] {
        assert_eq!(argument.label.resolve(Terminology::default(), locale), contract["labels"][key].as_str().unwrap());
        for (option, template) in options.iter().zip(catalog.templates) {
            assert_eq!(option.value, template.name);
            assert_eq!(option.label.resolve(Terminology::default(), locale), template.name);
        }
    }
    let ArgSchema::String { options: schema_options, option_source, .. } = &argument.schema else { panic!("a template identifier is a string") };
    assert!(option_source.is_none());
    assert_eq!(*schema_options, options);
    let mut populations = BTreeMap::new();
    for template in catalog.templates {
        *populations.entry(template.kind).or_insert(0u64) += 1;
    }
    for (kind, count) in populations {
        assert_eq!(count, contract["kinds"][kind].as_u64().unwrap());
    }
}
