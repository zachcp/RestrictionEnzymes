// This file is auto-generated - do not edit directly

fn complement(base: char) -> char {
    match base {
        'A' => 'T',
        'T' => 'A',
        'G' => 'C',
        'C' => 'G',
        'R' => 'Y', // A or G -> T or C
        'Y' => 'R', // C or T -> G or A
        'M' => 'K', // A or C -> T or G
        'K' => 'M', // G or T -> C or A
        'S' => 'S', // G or C -> G or C (self-complementary)
        'W' => 'W', // A or T -> T or A (self-complementary)
        'H' => 'D', // A, C, or T -> T, G, or A
        'B' => 'V', // C, G, or T -> G, C, or A
        'V' => 'B', // A, C, or G -> T, G, or C
        'D' => 'H', // A, G, or T -> T, C, or A
        'N' => 'N', // Any base -> Any base
        other => other,
    }
}

pub trait RestrictionEnzymeTrait {
    const NAME: &'static str;
    const SITE: &'static str;
    const FST3: Option<i32>;
    const FST5: Option<i32>;
    const OVHG: Option<i32>;
    const OVHG_SEQ: Option<&'static str>;
    const SIZE: i32;
    const INACT_TEMP: i32;
    const OPT_TEMP: i32;
    const ID: Option<i32>;
    const FREQ: Option<f32>;
    const COMPSITE: Option<&'static str>;
    const SUBSTRAT: &'static str;
    const URI: Option<&'static str>;
    const SCD3: Option<i32>;
    const SCD5: Option<i32>;
    const SUPPLIERS: Option<&'static [&'static str]>;

    fn reverse_complement_site() -> String {
        Self::SITE
            .chars()
            .rev()
            .map(|base| complement(base))
            .collect()
    }

    fn is_palindrome() -> bool {
        Self::SITE == Self::reverse_complement_site()
    }

    fn is_end_sticky() -> bool {
        Self::OVHG_SEQ.unwrap_or("") != ""
    }

    fn is_end_blunt() -> bool {
        Self::OVHG_SEQ.unwrap_or("") == ""
    }
}
