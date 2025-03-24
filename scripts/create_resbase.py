import json
from Bio import Restriction


def camelcase(name):
    if name == "":
          raise ValueError("Name should not be empty!")
    if "_" in name:
        name1, name2 = name.split("_")
        return f"{name1.capitalize()}{name2.capitalize()}"
    else:
        return(name.capitalize())



# Generate the full module file with all enzymes included
with open("src/enzyme_trait.rs", "w") as fout:

    fout.write("""
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
""")


# Generate the full module file with all enzymes included
with open("src/enzymes.rs", "w") as fout:

    fout.write("""// This file is auto-generated - do not edit directly

use super::enzyme_trait::RestrictionEnzymeTrait;

// Enum containing all restriction enzymes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestrictionEnzymeEnum {
""")

    # Add all enum variants
    for re_name, _ in Restriction.Restriction_Dictionary.rest_dict.items():
        safename = camelcase(re_name)



        fout.write(f"    {safename},\n")

    fout.write("}\n\n")

    # Add implementations for all enzymes
    for re_name, re_content in Restriction.Restriction_Dictionary.rest_dict.items():
        safename = camelcase(re_name)
        # Handle optional integer fields - ensure we output valid Rust Option constants
        fst3 = re_content.get('fst3', None)
        fst3_val = "None" if fst3 is None or str(fst3) == "None" else f'Some({fst3})'

        fst5 = re_content.get('fst5', None)
        fst5_val = "None" if fst5 is None or str(fst5) == "None" else f'Some({fst5})'

        ovhg = re_content.get('ovhg', None)
        ovhg_val = "None" if ovhg is None or str(ovhg) == "None" else f'Some({ovhg})'

        # Handle suppliers formatting - fixed to handle tuple
        suppliers = re_content.get('suppl', None)
        suppliers_val = "None"
        if suppliers and suppliers != "None":
            # Handle suppliers as a tuple
            if isinstance(suppliers, tuple) and suppliers:
                supplier_items = [s for s in suppliers if s]
                if supplier_items:
                    suppliers_formatted = '", "'.join(supplier_items)
                    suppliers_val = f'Some(&["{suppliers_formatted}"])'

        # Handle optional fields
        ovhgseq = re_content.get('ovhgseq', '')
        ovhgseq_val = "None" if not ovhgseq else f'Some("{ovhgseq}")'

        compsite = re_content.get('compsite', None)
        compsite_val = "None" if not compsite or compsite == "None" else f'Some("{compsite.replace("`", "\\`").replace("\"", "\\\"")}")'

        uri = re_content.get('uri', None)
        uri_val = "None" if not uri or uri == "None" else f'Some("{uri}")'

        # Handle numeric option fields
        scd3 = re_content.get('scd3', None)
        scd3_val = "None" if scd3 is None or scd3 == "None" else f'Some({scd3})'

        scd5 = re_content.get('scd5', None)
        scd5_val = "None" if scd5 is None or scd5 == "None" else f'Some({scd5})'

        id_val = "None" if re_content.get('id', None) is None or re_content.get('id', None) == "None" else f'Some({re_content["id"]})'

        freq_val = "None" if re_content.get('freq', None) is None or re_content.get('freq', None) == "None" else f'Some({re_content["freq"]})'

        # Create the implementation
        fout.write(f"""
pub struct {safename};

impl RestrictionEnzymeTrait for {safename} {{
    const NAME: &'static str = "{re_name}";
    const SITE: &'static str = "{re_content['site']}";
    const FST3: Option<i32> = {fst3_val};
    const FST5: Option<i32> = {fst5_val};
    const OVHG: Option<i32> = {ovhg_val};
    const OVHG_SEQ: Option<&'static str> = {ovhgseq_val};
    const SIZE: i32 = {re_content['size']};
    const ID: Option<i32> = {id_val};
    const INACT_TEMP: i32 = {re_content['inact_temp']};
    const OPT_TEMP: i32 = {re_content['opt_temp']};
    const FREQ: Option<f32> = {freq_val};
    const COMPSITE: Option<&'static str> = {compsite_val};
    const SUBSTRAT: &'static str = "{re_content['substrat']}";
    const URI: Option<&'static str> = {uri_val};
    const SCD3: Option<i32> = {scd3_val};
    const SCD5: Option<i32> = {scd5_val};
    const SUPPLIERS: Option<&'static [&'static str]> = {suppliers_val};
}}
""")
