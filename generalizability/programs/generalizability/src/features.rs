use crate::SupportedLanguage;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

// Language specific features that one or more of the supported languages
// in the list isn't compatible with. Sourced from
// https://en.wikipedia.org/wiki/Comparison_of_regular_expression_engines
#[derive(PartialEq, Eq, Hash)]
pub enum Feature {
    Flags,
    Backreferences,
    Conditionals,
    AtomicGroups,
    NamedCapture,
}

impl Feature {
    pub fn as_str(&self) -> &'static str {
        match self {
            Feature::Flags => "flags/directives",
            Feature::Backreferences => "backreferences",
            Feature::Conditionals => "conditionals",
            Feature::AtomicGroups => "atomic groups",
            Feature::NamedCapture => "named captures",
        }
    }
}

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Helper macro to specify the HashSet<_> type for into, so a hashset can be initialized inline
/// from an arbitrary type
///
/// # Examples
/// ```rust
/// use std::collections::HashSet;
/// use crate::generalizability::set_from;
/// let x = set_from!([1u8,2u8,3u8], u8);
/// assert!(x.contains(&2u8));
/// ```
///
#[macro_export]
macro_rules! set_from {
    ($n: expr, $t: ty) => {{
        let x: ::std::collections::HashSet<$t> = $n.into();
        x
    }};
}

lazy_static! {
    /// A mapping of languages supported by the program to sets of the features that they support -
    /// this enables the component, for any encountered regex, to look up one of its features
    /// against the languages that support it
    pub static ref LANGUAGE_FEATURE_SUPPORT: HashMap<SupportedLanguage, HashSet<Feature>> = HashMap::from([
        (SupportedLanguage::Python, set_from!([
            Feature::Flags,
            Feature::Conditionals,
            Feature::AtomicGroups,
            Feature::NamedCapture,
            Feature::Backreferences,
        ], Feature)),
        (SupportedLanguage::Java, set_from!([
            Feature::Flags,
            Feature::AtomicGroups,
            Feature::NamedCapture,
            Feature::Backreferences
        ], Feature)),
        (SupportedLanguage::JavaScript, set_from!([
            Feature::Backreferences
        ], Feature)),
        // Golang's RegExp has the same syntax as RE2
        (SupportedLanguage::Golang, set_from!([
            Feature::Flags,
            Feature::NamedCapture,
        ], Feature)),
        (SupportedLanguage::PHP, set_from!([
            Feature::Flags,
            Feature::Conditionals,
            Feature::AtomicGroups,
            Feature::NamedCapture,
            Feature::Backreferences,
        ], Feature)),
        (SupportedLanguage::Ruby, set_from!([
            Feature::Flags,
            Feature::Conditionals,
            Feature::AtomicGroups,
            Feature::NamedCapture,
            Feature::Backreferences,
        ], Feature))
    ]);
}
