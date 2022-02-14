pub mod features;
pub mod root;

pub use features::*;
use paste::paste;
pub use root::*;

/// Helper macro that will generate an annotation object, and call one of the
/// set_ functions on it, as well as setting the note.
macro_rules! annotate {
    ($annotation: expr,
    $func: ident,
    $source: expr,
    $val: expr) => {
        paste! {
            $annotation.[<set_$func>]($source);
            $annotation.set_note($val.to_string());
        }
    };
    ($func: ident, $source: expr, $val: expr) => {{
        let mut a = Annotation::new();
        annotate!(a, $func, $source, $val);
        a
    }};
}

/// Find the languages which don't support the given feature, and generate an annotation
/// explaining it
///
/// # Examples
/// ```rust
/// use generalizability::{Feature, unsupported_annotate};
/// use generalizability::root::*;
///
/// // Sample non-generalizable token - a global flag
/// let mut test_token = Token::new();
/// test_token.set_token(String::from("/g"));
/// test_token.set_field_type(TokenType::Flag);
/// test_token.set_flag(FlagType::Global);
///
/// let annotation = unsupported_annotate(Feature::Flags, test_token);
/// assert_eq!(annotation.note, "JavaScript may not support flags/directives");
/// ```
pub fn unsupported_annotate(feature: Feature, token: Token) -> Annotation {
    let mut feat_unsupported_languages: Vec<&SupportedLanguage> = Vec::new();
    for (lang, lang_features) in LANGUAGE_FEATURE_SUPPORT.iter() {
        if !lang_features.contains(&feature) {
            feat_unsupported_languages.push(lang)
        }
    }

    let lang_str: String;
    match feat_unsupported_languages.len() {
        l if l > 1 => {
            // Pull the languages together into a string of the format "x, y, and z"
            lang_str = format!(
                "{}, and {:?}",
                feat_unsupported_languages[0..l - 1]
                    .iter_mut()
                    .map(|f| format!("{:?}", *f))
                    .collect::<Vec<String>>()
                    .join(", "),
                feat_unsupported_languages[l - 1]
            );
        }
        1 => {
            // If there's just one language, leave it alone in the string
            lang_str = format!("{:?}", feat_unsupported_languages[0]);
        }
        _ => {
            // This should be unreachable
            panic!(
                "Feature {} doesn't have any unsupported languages - \
        should be removed from features.rs",
                feature
            );
        }
    };

    let res_str = format!("{} may not support {}", lang_str, feature);

    annotate!(token, token, res_str)
}

pub fn check_expression(expr: Expression) -> Vec<Annotation> {
    let mut annotations: Vec<Annotation> = Vec::new();

    // Figure out which features are used
    for token in expr.tokens {
        // The Token_oneof_sub_type is the PB generated union type for pb 'oneof'. Just use
        // unwrap() here, because the only way it would be None is if there wasn't a sub_type,
        // but it's not an optional field
        match token.clone().sub_type.unwrap() {
            Token_oneof_sub_type::flag(_) => {
                annotations.push(unsupported_annotate(Feature::Flags, token))
            }
            Token_oneof_sub_type::lookaround(_) => {
                annotations.push(unsupported_annotate(Feature::Conditionals, token))
            }
            Token_oneof_sub_type::groupref(t) => match t {
                GroupReferenceType::GroupName => {
                    annotations.push(unsupported_annotate(Feature::NamedCapture, token))
                }
                GroupReferenceType::NumericReference => {
                    annotations.push(unsupported_annotate(Feature::Backreferences, token))
                }
                _ => {}
            },
            Token_oneof_sub_type::characterclass(t) => {
                if t == CharacterClassType::AtomicGroup {
                    annotations.push(unsupported_annotate(Feature::AtomicGroups, token))
                }
            }
            _ => {}
        }
    }

    annotations
}

pub fn generalizability(expr: Expression) -> Result<Output, &'static str> {
    let annotations = check_expression(expr);
    let mut output = Output::new();
    // Use this inline instead of cloning annotations
    let mut annotations_count = 0;
    for annotation in annotations {
        output.mut_annotations().push(annotation);
        annotations_count += 1;
    }
    match annotations_count {
        0 => output.set_status(String::from("")),
        _ => output.set_status(format!(
            "Found {} generalizability issues",
            annotations_count
        )),
    };
    Ok(output)
}
