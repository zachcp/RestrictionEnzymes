pub trait RestrictionEnzymeTrait {
    const NAME: &'static str;
    const SITE: &'static str;
    const FST3: Option<i32>;
    const FST5: Option<i32>;
    const OVHG_SEQ: Option<&'static str>;
    const SIZE: i32;

    const INACT_TEMP: i32 = 65;
    const OPT_TEMP: i32 = 37;
    const SUBSTRAT: &'static str = "DNA";
}

#[crabtime::function]
fn generate_restriction_enzymes() -> String {
    // Define enzyme data - simplified for clarity
    let enzymes = [
        ("EcoRI", "GAATTC", 5, 1, "AATT", 6),
        ("BamHI", "GGATCC", 5, 1, "GATC", 6),
        ("HindIII", "AAGCTT", 5, 1, "AGCT", 6),
    ];

    // Format the code as a string
    enzymes
        .iter()
        .map(|(name, site, fst3, fst5, ovhg_seq, size)| {
            let code = format!(
                r#"
pub struct {name};

impl RestrictionEnzymeTrait for {name} {{
    const NAME: &'static str = "{name}";
    const SITE: &'static str = "{site}";
    const FST3: Option<i32> = Some({fst3});
    const FST5: Option<i32> = Some({fst5});
    const OVHG_SEQ: Option<&'static str> = Some("{ovhg_seq}");
    const SIZE: i32 = {size};
}}
"#
            );

            // Optionally print each implementation for debugging
            println!("Generated implementation for {name}:");
            println!("{code}");

            code
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// Use a single, standalone macro invocation
generate_restriction_enzymes!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecori_name() {
        assert_eq!(EcoRI::NAME, "EcoRI");
    }
}
