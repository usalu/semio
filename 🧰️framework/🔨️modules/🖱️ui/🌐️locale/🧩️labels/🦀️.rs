//! 🎗️ Defines complete multilingual app label sets at their actual neutral type owner.
    #[macro_export]
    macro_rules! app_labels {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident {
            $( $field:ident: native_en $nen:expr, native_de $nde:expr, reuse_en $ren:expr, reuse_de $rde:expr );+ $(;)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $Name {
            $( $vis $field: $crate::LabelText ),+ ,
            locale: $crate::Locale,
        }

        impl $Name {
            $vis const NATIVE_EN: Self = Self { $( $field: $crate::LabelText::__from_app_labels($nen) ),+ , locale: $crate::Locale::En };
            $vis const NATIVE_DE: Self = Self { $( $field: $crate::LabelText::__from_app_labels($nde) ),+ , locale: $crate::Locale::De };
            $vis const REUSE_EN: Self = Self { $( $field: $crate::LabelText::__from_app_labels($ren) ),+ , locale: $crate::Locale::En };
            $vis const REUSE_DE: Self = Self { $( $field: $crate::LabelText::__from_app_labels($rde) ),+ , locale: $crate::Locale::De };

            $vis const FIELD_NAMES: &'static [&'static str] = &[ $( stringify!($field) ),+ ];

            $vis fn for_each_label(&self, mut visit: impl FnMut(&'static str, &$crate::LabelText)) {
                $( visit(stringify!($field), &self.$field); )+
            }

            $vis fn locale(&self) -> $crate::Locale {
                self.locale
            }

            $vis fn is_de(&self) -> bool {
                matches!(self.locale, $crate::Locale::De)
            }
        }

        impl $crate::AppLabels for $Name {
            fn labels(locale: $crate::Locale, terminology: $crate::Terminology) -> &'static Self {
                match (terminology, locale) {
                    ($crate::Terminology::Native, $crate::Locale::En) => &Self::NATIVE_EN,
                    ($crate::Terminology::Native, $crate::Locale::De) => &Self::NATIVE_DE,
                    ($crate::Terminology::Reuse, $crate::Locale::En) => &Self::REUSE_EN,
                    ($crate::Terminology::Reuse, $crate::Locale::De) => &Self::REUSE_DE,
                }
            }
        }
    };
}

