extern crate csv;
extern crate serialize;

use std::path::Path;

#[allow(dead_code)]
#[deriving(Decodable)]
struct Record {
    s1: String,
    s2: String,
    s3: String,
    s4: String,
    s5: String,
    s6: String,
    s7: String,
    s8: String,
    s9: String,
    s10: String,
    s11: String,
    s12: String,
    s13: String,
    s14: String,
    s15: String,
    s16: String,
    s17: String,
}

fn main() {
   let fp = &Path::new("../data/house-votes-84.data");
   let mut rdr = csv::Decoder::from_file(fp);

   for record in rdr.iter_decode::<Record>() {
       let record = record.unwrap();
       println!("{}", record.s1);
   }
}
