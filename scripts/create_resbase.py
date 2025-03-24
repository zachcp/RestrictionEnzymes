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


# # create the files
# for idx, (re_name, re_content) in enumerate(Restriction.Restriction_Dictionary.rest_dict.items()):
#         #if idx < 4:
#             safename = camelcase(re_name)
#             print(re_content.keys())
#             del(re_content['charac']) # this is a duplicate set of fileds. removing it make things simple
#             with open(f"data/restriction_enzymes/enzymedata/{safename}.json", "w") as fout:
#                     re_content['name'] = safename
#                     json.dump(re_content, fout)



# # create the files
# with open(f"data/restriction_enzymes/enum.txt", "w") as fout:
#     fout.write("enum RestrictionEnzymeEnum { \n ")
#     for idx, (re_name, re_content) in enumerate(Restriction.Restriction_Dictionary.rest_dict.items()):
#             #if idx < 4:
#                 safename = camelcase(re_name)
#                 fout.write(f"   {safename},\n")
#     fout.write("}\n")



# # outout a csv
# with open("data/restriction_enzymes/enzyme.txt", "w") as fout:
#     for idx, (re_name, re_content) in enumerate(Restriction.Restriction_Dictionary.rest_dict.items()):
#         safename = camelcase(re_name)
#         print(re_content.keys())
#         re_content['name'] = safename

#         for key in ['compsite', 'dna', 'freq', 'fst3', 'fst5', 'id', 'inact_temp', 'opt_temp', 'ovhg', 'ovhgseq', 'results', 'scd3', 'scd5', 'site', 'size', 'substrat', 'suppl', 'uri', 'name']:
#             fout.write(f"{re_content[key]},")
#         fout.write(f"\n")


# # Generate the trait definition
# with open("src/rebase_trait.rs", "w") as fout:
#     fout.write("""pub trait RestrictionEnzymeTrait {
#     const NAME: &'static str;
#     const SITE: &'static str;
#     const FST3: Option<i32>;
#     const FST5: Option<i32>;
#     const OVHG: Option<i32>;
#     const OVHG_SEQ: Option<&'static str>;
#     const SIZE: i32;

#     const INACT_TEMP: i32;
#     const OPT_TEMP: i32;
#     const ID: Option<i32>;

#     const FREQ: Option<f64>;
#     const COMPSITE: Option<&'static str>;
#     const SUBSTRAT: &'static str;
#     const URI: Option<&'static str>;

#     const SCD3: Option<i32>;
#     const SCD5: Option<i32>;
#     const SUPPLIERS: Option<&'static [&'static str]>;
# }
# """)


# # Generate the implementations for each enzyme
# with open("src/rebase2.rs", "w") as fout:
#     for idx, (re_name, re_content) in enumerate(Restriction.Restriction_Dictionary.rest_dict.items()):
#         safename = camelcase(re_name)

#         # Handle suppliers formatting
#         suppliers = re_content.get('suppl', None)
#         suppliers_val = "None"
#         if suppliers and suppliers != "None":
#             # Handle suppliers as a tuple
#             if isinstance(suppliers, tuple) and suppliers:
#                 supplier_items = [s for s in suppliers if s]
#                 if supplier_items:
#                     suppliers_formatted = '", "'.join(supplier_items)
#                     suppliers_val = f'Some(&["{suppliers_formatted}"])'

#         # Handle optional fields
#         ovhgseq = re_content.get('ovhgseq', '')
#         ovhgseq_val = "None" if not ovhgseq else f'Some("{ovhgseq}")'

#         compsite = re_content.get('compsite', None)
#         compsite_val = "None" if not compsite or compsite == "None" else f'Some("{compsite.replace("`", "\\`").replace("\"", "\\\"")}")'

#         uri = re_content.get('uri', None)
#         uri_val = "None" if not uri or uri == "None" else f'Some("{uri}")'

#         # Handle numeric option fields
#         scd3 = re_content.get('scd3', None)
#         scd3_val = "None" if scd3 is None or scd3 == "None" else f'Some({scd3})'

#         scd5 = re_content.get('scd5', None)
#         scd5_val = "None" if scd5 is None or scd5 == "None" else f'Some({scd5})'

#         id_val = "None" if re_content.get('id', None) is None or re_content.get('id', None) == "None" else f'Some({re_content["id"]})'

#         freq_val = "None" if re_content.get('freq', None) is None or re_content.get('freq', None) == "None" else f'Some({re_content["freq"]})'

#         # Create the implementation
#         fout.write(f"""
# pub struct {safename};

# impl RestrictionEnzymeTrait for {safename} {{
#     const NAME: &'static str = "{re_name}";
#     const SITE: &'static str = "{re_content['site']}";
#     const FST3: Option<i32> = Some({re_content['fst3']});
#     const FST5: Option<i32> = Some({re_content['fst5']});
#     const OVHG: Option<i32> = Some({re_content['ovhg']});
#     const OVHG_SEQ: Option<&'static str> = {ovhgseq_val};
#     const SIZE: i32 = {re_content['size']};
#     const ID: Option<i32> = {id_val};
#     const INACT_TEMP: i32 = {re_content['inact_temp']};
#     const OPT_TEMP: i32 = {re_content['opt_temp']};
#     const FREQ: Option<f64> = {freq_val};
#     const COMPSITE: Option<&'static str> = {compsite_val};
#     const SUBSTRAT: &'static str = "{re_content['substrat']}";
#     const URI: Option<&'static str> = {uri_val};
#     const SCD3: Option<i32> = {scd3_val};
#     const SCD5: Option<i32> = {scd5_val};
#     const SUPPLIERS: Option<&'static [&'static str]> = {suppliers_val};
# }}
# """)



# Generate the full module file with all enzymes included
with open("src/rebase3.rs", "w") as fout:
    fout.write("""// This file is auto-generated - do not edit directly

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

    const FREQ: Option<f64>;
    const COMPSITE: Option<&'static str>;
    const SUBSTRAT: &'static str;
    const URI: Option<&'static str>;

    const SCD3: Option<i32>;
    const SCD5: Option<i32>;
    const SUPPLIERS: Option<&'static [&'static str]>;
}

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
    const FST3: Option<i32> = Some({re_content['fst3']});
    const FST5: Option<i32> = Some({re_content['fst5']});
    const OVHG: Option<i32> = Some({re_content['ovhg']});
    const OVHG_SEQ: Option<&'static str> = {ovhgseq_val};
    const SIZE: i32 = {re_content['size']};
    const ID: Option<i32> = {id_val};
    const INACT_TEMP: i32 = {re_content['inact_temp']};
    const OPT_TEMP: i32 = {re_content['opt_temp']};
    const FREQ: Option<f64> = {freq_val};
    const COMPSITE: Option<&'static str> = {compsite_val};
    const SUBSTRAT: &'static str = "{re_content['substrat']}";
    const URI: Option<&'static str> = {uri_val};
    const SCD3: Option<i32> = {scd3_val};
    const SCD5: Option<i32> = {scd5_val};
    const SUPPLIERS: Option<&'static [&'static str]> = {suppliers_val};
}}
""")

    # Add helper methods to map enum to implementations
    fout.write("""
// Method to get enzyme by enum
pub fn get_enzyme(enzyme: RestrictionEnzymeEnum) -> &'static dyn RestrictEnzymeObj {
    match enzyme {
""")

    for re_name, _ in Restriction.Restriction_Dictionary.rest_dict.items():
        safename = camelcase(re_name)
        fout.write(f"        RestrictionEnzymeEnum::{safename} => &{safename},\n")

    fout.write("""    }
}

// Trait extension for runtime access to enzyme properties
pub trait RestrictEnzymeObj {
    fn name(&self) -> &'static str;
    fn site(&self) -> &'static str;
    fn fst3(&self) -> Option<i32>;
    fn fst5(&self) -> Option<i32>;
    fn ovhg(&self) -> Option<i32>;
    fn ovhgseq(&self) -> Option<&'static str>;
    fn size(&self) -> i32;
    // Add other methods as needed
}

// Implement RestrictEnzymeObj for all enzymes
impl<T: RestrictionEnzymeTrait> RestrictEnzymeObj for T {
    fn name(&self) -> &'static str { T::NAME }
    fn site(&self) -> &'static str { T::SITE }
    fn fst3(&self) -> Option<i32> { T::FST3 }
    fn fst5(&self) -> Option<i32> { T::FST5 }
    fn ovhg(&self) -> Option<i32> { T::OVHG }
    fn ovhgseq(&self) -> Option<&'static str> { T::OVHG_SEQ }
    fn size(&self) -> i32 { T::SIZE }
    // Implement other methods as needed
}
""")
