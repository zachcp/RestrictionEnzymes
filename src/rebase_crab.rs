pub trait RestrictionEnzymeTrait {
    const NAME: &'static str;
    const SITE: &'static str;
    const FST3: Option<i32>;
    const FST5: Option<i32>;
    const OVHG: Option<i32>;
    const OVHG_SEQ: Option<&'static str>;
    const SIZE: i32;
    const FREQ: Option<f64>;
    const COMPSITE: Option<&'static str>;
    const SCD3: Option<i32>;
    const SCD5: Option<i32>;
    const ID: Option<i32>;
    const INACT_TEMP: i32 = 65; // Default value, will be overridden if needed
    const OPT_TEMP: i32 = 37; // Default value, will be overridden if needed
    const SUBSTRAT: &'static str = "DNA"; // Default value, will be overridden if needed
    const SUPPLIERS: Option<&'static [&'static str]>;
    const URI: Option<&'static str>;
}

mod generated {
    use super::RestrictionEnzymeTrait;
    use crabtime;

    // Define a generator that parses the CSV data
    #[crabtime::function]
    fn generate_enzymes() -> String {
        // Parse the CSV data to generate enzyme definitions
        let csv_data = r#"#  ['compsite', 'dna', 'freq', 'fst3', 'fst5', 'id', 'inact_temp', 'opt_temp', 'ovhg', 'ovhgseq', 'results', 'scd3', 'scd5', 'site', 'size', 'substrat', 'suppl', 'uri', 'name']
(?=(?P<AanI>TTATAA)),None,4096.0,-3,3,15358,65,37,0,,None,None,None,TTATAA,6,DNA,('B',),https://identifiers.org/rebase:15358,Aani,
(?=(?P<AarI>CACCTGC))|(?=(?P<AarI_as>GCAGGTG)),None,16384.0,8,11,2892,65,37,-4,NNNN,None,None,None,CACCTGC,7,DNA,('B',),https://identifiers.org/rebase:2892,Aari,
(?=(?P<AasI>GAC......GTC)),None,4096.0,-7,7,5465,65,37,2,NN,None,None,None,GACNNNNNNGTC,12,DNA,('B',),https://identifiers.org/rebase:5465,Aasi,"#;

        let mut result = String::new();

        for line in csv_data.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 19 {
                continue; // Skip lines with insufficient fields
            }

            let compsite = fields[0];
            let freq = fields[2];
            let fst3 = fields[3];
            let fst5 = fields[4];
            let id = fields[5];
            let inact_temp = fields[6];
            let opt_temp = fields[7];
            let ovhg = fields[8];
            let ovhgseq = fields[9];
            let scd3 = fields[11];
            let scd5 = fields[12];
            let site = fields[13];
            let size = fields[14];
            let substrat = fields[15];
            let suppliers = fields[16];
            let uri = fields[17];
            let name = fields[18].trim_end_matches(',');

            // Format the supplier string
            let suppliers_formatted =
                if suppliers == "None" || suppliers == "('',)" || suppliers.is_empty() {
                    "None".to_string()
                } else {
                    // Extract supplier codes from format like ('B',) or ('B','C')
                    let supplier_list = suppliers
                        .trim_start_matches("('")
                        .trim_end_matches("',)")
                        .replace("','", "\", \"");
                    format!("Some(&[\"{}\"])", supplier_list)
                };

            // Format optional fields
            let compsite_formatted = if compsite == "None" || compsite.is_empty() {
                "None".to_string()
            } else {
                format!("Some(\"{}\")", compsite)
            };

            let ovhgseq_formatted = if ovhgseq.is_empty() || ovhgseq == "None" {
                "None".to_string()
            } else {
                format!("Some(\"{}\")", ovhgseq)
            };

            let scd3_formatted = if scd3 == "None" {
                "None".to_string()
            } else {
                format!("Some({})", scd3)
            };

            let scd5_formatted = if scd5 == "None" {
                "None".to_string()
            } else {
                format!("Some({})", scd5)
            };

            let id_formatted = if id == "None" {
                "None".to_string()
            } else {
                format!("Some({})", id)
            };

            let uri_formatted = if uri == "None" || uri.is_empty() {
                "None".to_string()
            } else {
                format!("Some(\"{}\")", uri)
            };

            let freq_formatted = if freq == "None" {
                "None".to_string()
            } else {
                format!("Some({})", freq)
            };

            // Create the struct implementation
            result.push_str(&format!(
                r#"
pub struct {0};

impl RestrictionEnzymeTrait for {0} {{
    const NAME: &'static str = "{0}";
    const SITE: &'static str = "{1}";
    const FST3: Option<i32> = Some({2});
    const FST5: Option<i32> = Some({3});
    const OVHG: Option<i32> = Some({4});
    const OVHG_SEQ: Option<&'static str> = {5};
    const SIZE: i32 = {6};
    const FREQ: Option<f64> = {7};
    const COMPSITE: Option<&'static str> = {8};
    const SCD3: Option<i32> = {9};
    const SCD5: Option<i32> = {10};
    const ID: Option<i32> = {11};
    const INACT_TEMP: i32 = {12};
    const OPT_TEMP: i32 = {13};
    const SUBSTRAT: &'static str = "{14}";
    const SUPPLIERS: Option<&'static [&'static str]> = {15};
    const URI: Option<&'static str> = {16};
}}
"#,
                name,
                site,
                fst3,
                fst5,
                ovhg,
                ovhgseq_formatted,
                size,
                freq_formatted,
                compsite_formatted,
                scd3_formatted,
                scd5_formatted,
                id_formatted,
                inact_temp,
                opt_temp,
                substrat,
                suppliers_formatted,
                uri_formatted
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
        // assert_eq!(generated::EcoRI::NAME, "EcoRI");
        assert_eq!(generated::Aani::NAME, "Aani");
    }
}
