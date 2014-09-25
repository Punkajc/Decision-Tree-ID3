extern crate csv;
extern crate serialize;
extern crate decision_tree;

use std::path::Path;
use decision_tree::id3;

struct VotingRecord {
    fields: Vec<String>,
}

static class_name: &'static str = "class_name";
static handicapped: &'static str = "handicapped";
static water_project: &'static str = "water_project";
static budget_resolution: &'static str = "budget_resolution";
static physician_freeze: &'static str = "physician_freeze";
static el_salvador_aid: &'static str = "el_salvador_aid";
static religious_schools: &'static str = "religious_schools";
static anti_sattelite_ban: &'static str = "anti_sattelite_ban";
static aid_to_contras: &'static str = "aid_to_contras";
static mx_missile: &'static str = "mx_missile";
static immigration: &'static str = "immigration";
static synfuels_cutback: &'static str = "synfuels_cutback";
static education_spending: &'static str = "education_spending";
static right_to_sue: &'static str = "right_to_sue";
static crime: &'static str = "crime";
static duty_free_exports: &'static str = "duty_free_exports";
static export_south_africa: &'static str = "export_south_africa";

impl VotingRecord {
    fn new(fields: Vec<String>) -> VotingRecord {
        VotingRecord {
            fields: fields,
        }
    }
}

impl id3::Record for VotingRecord {
    fn get_attribute(&self, attr_name: &str) -> &str {
        let val = match attr_name {
            class_name => &self.fields[0],
            handicapped => &self.fields[1],
            water_project => &self.fields[2],
            budget_resolution => &self.fields[3],
            physician_freeze => &self.fields[4],
            el_salvador_aid => &self.fields[5],
            religious_schools => &self.fields[6],
            anti_sattelite_ban => &self.fields[7],
            aid_to_contras => &self.fields[8],
            mx_missile => &self.fields[9],
            immigration => &self.fields[10],
            synfuels_cutback => &self.fields[11],
            education_spending => &self.fields[12],
            right_to_sue => &self.fields[13],
            crime => &self.fields[14],
            duty_free_exports => &self.fields[15],
            export_south_africa => &self.fields[16],
            _ => &self.fields[17], // dirty, error throwing hack
        };
        val.as_slice()
    }

    fn get_attribute_names(&self) -> Vec<&str> {
        vec![handicapped,water_project,budget_resolution,physician_freeze,
             el_salvador_aid,religious_schools,anti_sattelite_ban,
             aid_to_contras,mx_missile,immigration,synfuels_cutback,
             education_spending,right_to_sue,crime,duty_free_exports,
             export_south_africa]
    }
}

impl<'a> id3::Record for &'a VotingRecord {
    fn get_attribute(&self, attr_name: &str) -> &str {
        self.get_attribute(attr_name)
    }

    fn get_attribute_names(&self) -> Vec<&str> {
        self.get_attribute_names()
    }
}

#[cfg(not(test))]
fn main() {
    let fp = &Path::new("./data/voting-records/house-votes-84.data");
    let mut rdr = csv::Reader::from_file(fp);
    let rows = csv::collect(rdr.records()).unwrap();
    let records = rows.map_in_place(|x| VotingRecord::new(x));
    let attr_names = vec![handicapped,water_project,budget_resolution,
                          physician_freeze,el_salvador_aid,religious_schools,
                          anti_sattelite_ban,aid_to_contras,mx_missile,
                          immigration,synfuels_cutback,education_spending,
                          right_to_sue,crime,duty_free_exports,
                          export_south_africa];


    let root_vertex = id3::id3(records.iter().collect(), class_name, attr_names,0f64);

    println!("{}", root_vertex);
}
