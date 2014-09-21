#[experimental]
#[doc = "ID3 decision tree implementation"]
pub mod id3 {
    use std::collections::TreeMap;
    use std::f64;

    #[doc = "A vertex in a decision tree."]
    pub enum ID3Vertex {
        /// Vertex at which the example set is split along an attribute.
        Branch(String, TreeMap<String,ID3Vertex>),
        /// Terminal vertex whose class has been decided.
        Leaf(String),
    }

    #[doc = "Trait for labeled data."]
    pub trait Record {
        fn get_attribute(&self, attr_name: &str) -> &str;
        fn get_attribute_names(&self) -> Vec<&str>;
    }

    #[unstable]
    pub fn id3(examples: Vec<&Record>, label_attr_name: &str) -> ID3Vertex {
        // Return labeled leaf if all labels in examples are equal
        if attr_all_eq(&examples, label_attr_name) {
            return Leaf(label_attr_name.into_string());
        }

        // Choose attribute to split on
        // Assumes split_attribute() is Some
        let split_attr_name = split_attribute(&examples, label_attr_name).unwrap();
        let mut branch_map = TreeMap::new();

        let attr_domain: Vec<&str> = examples.iter().map(|x| (x.get_attribute(split_attr_name.as_slice()))).collect();
        for attr_value in attr_domain.iter() {
            let example_subset: Vec<&Record> = examples.iter().filter(|x| x.get_attribute(split_attr_name.as_slice()) == *attr_value).map(|x| *x).collect();
            let child_vertex = id3(example_subset, label_attr_name);
            branch_map.insert(attr_value.to_string(), child_vertex);
        }
        
        Branch(split_attr_name.to_string(), branch_map)
    }

    // Checks the given attribute for equality among all records
    fn attr_all_eq(records: &Vec<&Record>, attr_name: &str) -> bool {
        let first_attr = records[0].get_attribute(attr_name);
        let all_eq = records.iter().map(|x| x.get_attribute(attr_name)).all(|x| first_attr.cmp(&x) == Equal);

        all_eq
    }

    // assumes records.iter().next() is Some
    fn split_attribute(records: &Vec<&Record>, label_attr_name: &str) -> Option<String> {
        let mut min_entropy = f64::MAX_VALUE;
        let mut best_attr = None;

        let attr_names = records.iter().next().unwrap().get_attribute_names();
        for attr_name in attr_names.iter() {
            // Assuming discrete attributes
            if attr_name == &label_attr_name { continue; }
            let attr_label_pairs: Vec<(&str, &str)> = records.iter().map(|x| (x.get_attribute(*attr_name),x.get_attribute(label_attr_name))).collect();
            let entropy = split_entropy(attr_label_pairs);
            if entropy < min_entropy {
                best_attr = Some(attr_name.to_string());
                min_entropy = entropy;
            }
        }
        
        best_attr
    }

    fn split_entropy(attributes: Vec<(&str,&str)>) -> f64 {
        let mut total_entropy = 0f64;
        // N_m
        let total_attr_count = attributes.iter().count() as f64;

        // Loop through attribute values
        let mut attr_values = attributes.iter().map(|&x| x.val0());
        for attr_value in attr_values {
            let mut attr_entropy = 0f64;
            // N_mj
            let attr_count = attributes.iter().filter(|x| x.val0() == attr_value).count() as f64;

            // Loop through label values
            let mut label_values = attributes.iter().filter(|x| x.val0() == attr_value).map(|x| x.val1());
            for label_value in label_values {
                // N^i_mj
                let label_count = attributes.iter().filter(|x| x.val0() == attr_value).filter(|x| x.val1() == label_value).count() as f64;
                // p^i_mj
                let label_prop = label_count / attr_count;
                attr_entropy = attr_entropy + (label_prop * label_prop.log2())
            }
            total_entropy = total_entropy + ((attr_count / total_attr_count) * attr_entropy);
        }

        (-1f64) * total_entropy
    }
}
