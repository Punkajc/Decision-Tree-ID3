extern crate csv;
extern crate serialize;
extern crate decision_tree;

use std::path::Path;
use std::rand::{task_rng, Rng};
use decision_tree::{id3, tree};

#[deriving(Clone)]
struct VotingRecord {
    fields: Vec<String>,
}

#[deriving(Clone)]
struct MonkRecord {
    fields: Vec<String>,
}

static SPACE: u8 = ' ' as u8;
static class_name: &'static str = "class_name";
// Voting Records attribute names
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
// Monks Problems attributes names
static a1: &'static str = "a1";
static a2: &'static str = "a2";
static a3: &'static str = "a3";
static a4: &'static str = "a4";
static a5: &'static str = "a5";
static a6: &'static str = "a6";

impl VotingRecord {
    fn new(fields: Vec<String>) -> VotingRecord {
        VotingRecord {
            fields: fields,
        }
    }
}

impl MonkRecord {
    fn new(fields: Vec<String>) -> MonkRecord {
        MonkRecord {
            fields: fields,
        }
    }
}

impl tree::Record for VotingRecord {
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
            _ => &self.fields[89], // dirty, error throwing hack
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

impl tree::Record for MonkRecord {
    fn get_attribute(&self, attr_name: &str) -> &str {
        let val = match attr_name {
            class_name => &self.fields[1],
            a1 => &self.fields[2],
            a2 => &self.fields[3],
            a3 => &self.fields[4],
            a4 => &self.fields[5],
            a5 => &self.fields[6],
            a6 => &self.fields[7],
            _ => {
                println!("{}", attr_name); // print in case of error
                &self.fields[99] // dirty, error throwing hack
            },
        };
        val.as_slice()
    }

   fn get_attribute_names(&self) -> Vec<&str> {
       vec![a1, a2, a3, a4, a5, a6]
   }
}

impl<'a> tree::Record for &'a VotingRecord {
    fn get_attribute(&self, attr_name: &str) -> &str {
        self.get_attribute(attr_name)
    }

    fn get_attribute_names(&self) -> Vec<&str> {
        self.get_attribute_names()
    }
}

impl<'a> tree::Record for &'a MonkRecord {
    fn get_attribute(&self, attr_name: &str) -> &str {
        self.get_attribute(attr_name)
    }

    fn get_attribute_names(&self) -> Vec<&str> {
        self.get_attribute_names()
    }
}

#[cfg(not(test))]
fn main() {
    let fp_voting = &Path::new("./data/voting-records/house-votes-84.data");
    let fp_monk = &Path::new("./data/monks-problems/monks-1.train");

    let mut rdr_voting = csv::Reader::from_file(fp_voting);
    let mut rdr_monk = csv::Reader::from_file(fp_monk).delimiter(SPACE);

    let rows_voting = csv::collect(rdr_voting.records()).unwrap();
    let rows_monk = csv::collect(rdr_monk.records()).unwrap();

    let mut records_voting = rows_voting.map_in_place(|x| VotingRecord::new(x));
    let mut records_monk = rows_monk.map_in_place(|x| MonkRecord::new(x));

    let attr_names_voting = vec![handicapped,water_project,budget_resolution,
                          physician_freeze,el_salvador_aid,religious_schools,
                          anti_sattelite_ban,aid_to_contras,mx_missile,
                          immigration,synfuels_cutback,education_spending,
                          right_to_sue,crime,duty_free_exports,
                          export_south_africa];
    let attr_names_monk = vec![a1, a2, a3, a4, a5, a6];
    
    let mut rng = task_rng();
    rng.shuffle(records_voting.as_mut_slice());
    let (test_slice_voting, train_slice_voting) = records_voting.split_at_mut(30);
    rng.shuffle(records_monk.as_mut_slice());
    let (test_slice_monk, train_slice_monk) = records_monk.split_at_mut(30);

    let root_vertex_voting = id3::id3(train_slice_voting.to_vec().iter().collect(), class_name, attr_names_voting, 0f64);
    let root_vertex_monk = id3::id3(train_slice_monk.to_vec().iter().collect(), class_name, attr_names_monk, 0f64);

    let test_values_voting: Vec<bool> = test_slice_voting.iter().map(|x| tree::test(&root_vertex_voting, x, class_name)).collect();
    let test_values_monk: Vec<bool> = test_slice_monk.iter().map(|x| tree::test(&root_vertex_monk, x, class_name)).collect();

    let total_count_voting = test_values_voting.len();
    let total_count_monk = test_values_monk.len();
    let true_count_voting = test_values_voting.iter().filter(|x| **x).count();
    let true_count_monk = test_values_monk.iter().filter(|x| **x).count();
    let false_count_voting = total_count_voting - true_count_voting;
    let false_count_monk = total_count_monk - true_count_monk;

    println!("Voting Records");
    println!("{}", root_vertex_voting);
    println!("");
    println!("correct: {}/30", true_count_voting);
    println!("incorrect: {}/30", false_count_voting);

    println!("\n");

    println!("Monks Problems I");
    println!("{}", root_vertex_monk);
    println!("");
    println!("correct: {}/30", true_count_monk);
    println!("incorrect: {}/30", false_count_monk);

}
