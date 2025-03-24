pub trait RestrictionEnzymeTrait {
    const NAME: &'static str;
    const SITE: &'static str;
    const FST3: Option<i32>;
    const FST5: Option<i32>;
    const OVHG_SEQ: Option<&'static str>;
    const SIZE: i32;

    const INACT_TEMP: i32 = 65; // Default value, will be overridden if needed
    const OPT_TEMP: i32 = 37; // Default value, will be overridden if needed
    const SUBSTRAT: &'static str = "DNA"; // Default value, will be overridden if needed
}

mod generated {
    use super::RestrictionEnzymeTrait;
    use crabtime;

    // Define a simple generator that only outputs a few enzymes
    #[crabtime::function]
    fn generate_enzymes() -> String {
        // Just generate three fixed enzymes to keep it simple
        let enzymes = [
            ("EcoRI", "GAATTC", 5, 1, "AATT", 6),
            ("BamHI", "GGATCC", 5, 1, "GATC", 6),
            ("HindIII", "AAGCTT", 5, 1, "AGCT", 6),
        ];

        // Generate minimal code to avoid potential syntax issues
        let mut result = String::new();

        for (name, site, fst3, fst5, ovhg_seq, size) in enzymes {
            // Use format! which is more reliable than string concatenation
            result.push_str(&format!(
                r#"
pub struct {0};

impl RestrictionEnzymeTrait for {0} {{
    const NAME: &'static str = "{0}";
    const SITE: &'static str = "{1}";
    const FST3: Option<i32> = Some({2});
    const FST5: Option<i32> = Some({3});
    const OVHG_SEQ: Option<&'static str> = Some("{4}");
    const SIZE: i32 = {5};
}}
"#,
                name, site, fst3, fst5, ovhg_seq, size
            ));
        }

        result
    }

    // Call the generator
    generate_enzymes!();
}

// Re-export the generated structures
pub use generated::*;

#[cfg(test)]
mod tests {
    use super::{RestrictionEnzymeTrait, generated};

    #[test]
    fn test_ecori_name() {
        assert_eq!(generated::EcoRI::NAME, "EcoRI");
    }
}
