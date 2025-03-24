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


# create the files
for idx, (re_name, re_content) in enumerate(Restriction.Restriction_Dictionary.rest_dict.items()):
        #if idx < 4:
            safename = camelcase(re_name)
            print(re_content.keys())
            del(re_content['charac']) # this is a duplicate set of fileds. removing it make things simple
            with open(f"data/restriction_enzymes/enzymedata/{safename}.json", "w") as fout:
                    re_content['name'] = safename
                    json.dump(re_content, fout)



# create the files
with open(f"data/restriction_enzymes/enum.txt", "w") as fout:
    fout.write("enum RestrictionEnzymeEnum { \n ")
    for idx, (re_name, re_content) in enumerate(Restriction.Restriction_Dictionary.rest_dict.items()):
            #if idx < 4:
                safename = camelcase(re_name)
                fout.write(f"   {safename},\n")
    fout.write("}\n")



# outout a csv
with open("data/restriction_enzymes/enzyme.txt", "w") as fout:
    for idx, (re_name, re_content) in enumerate(Restriction.Restriction_Dictionary.rest_dict.items()):
        safename = camelcase(re_name)
        print(re_content.keys())
        re_content['name'] = safename

        for key in ['compsite', 'dna', 'freq', 'fst3', 'fst5', 'id', 'inact_temp', 'opt_temp', 'ovhg', 'ovhgseq', 'results', 'scd3', 'scd5', 'site', 'size', 'substrat', 'suppl', 'uri', 'name']:
            fout.write(f"{re_content[key]},")
        fout.write(f"\n")

# create the load all  code
#
# // Create a function to load data for each person
# fn load_restrictionenzyme_data(enzyme: RestrictionEnzymeEnum) -> RestrictionEnzyme {
#     let filename = match enzyme {
#         RestrictionEnzymeEnum::AA => "data/people/alice.json",
#         Person::Bob => "data/people/bob.json",
#         // Add more cases as needed
#     };
# with open(f"data/restriction_enzymes/load_all_fn.txt", "w") as fout:
#     fout.write("fn load_restrictionenzyme_data(enzyme: RestrictionEnzymeEnum) -> RestrictionEnzyme {\n ")
#     fout.write("    let filedata = match enzyme {\n ")
#     for idx, (re_name, re_content) in enumerate(Restriction.Restriction_Dictionary.rest_dict.items()):
#             #if idx < 4:
#                 safename = camelcase(re_name)
#                 # fout.write(f"       RestrictionEnzymeEnum::{safename} => \"data/restriction_enzymes/enzymedata/{safename}.json\",\n")
#                 fout.write(f"       RestrictionEnzymeEnum::{safename} => DATA_DIR.get_file(\"{safename}.json\").unwrap().contents_utf8().unwrap(),\n")
#     fout.write("        };\n")
#     # fout.write("    let json_data = fs::read_to_string(filename).expect(\"Failed to read JSON file\");\n")
#     fout.write("    let redata = serde_json::from_str(&filedata).expect(\"Failed to deserialize JSON data\");\n")
#     fout.write("    redata\n")
#     fout.write("}\n")
