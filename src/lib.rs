#![crate_name = "decision_tree"]
#![experimental]
#![license = "MIT"]

pub mod tree {
    use std::collections::TreeMap;

    #[doc = "A vertex in a decision tree."]
    #[deriving(Show)]
    pub enum DecisionVertex {
        /// Vertex at which the example set is split along an attribute.
        Branch(String, TreeMap<String,DecisionVertex>),
        /// Terminal vertex whose class has been decided.
        Leaf(String),
    }
    
    #[doc = "Trait for labeled data."]
    pub trait Record {
        fn get_attribute(&self, attr_name: &str) -> &str;
        fn get_attribute_names(&self) -> Vec<&str>;
    }    
}

#[experimental]
#[doc = "ID3 decision tree implementation"]
pub mod id3 {
    use std::collections::HashSet;
    use std::collections::TreeMap;
    use std::str::eq_slice;
    use std::f64;
    use tree::{Record, DecisionVertex, Leaf, Branch};

    #[unstable]
    #[doc = "
    ID3 Decision Tree Algorithm

    Recursively generates a decision tree from a dataset. [Wikipedia Link](http://en.wikipedia.org/wiki/ID3_algorithm)

    # Arguments

    * 'dataset' - The dataset to label or recursively split.
    * 'label_attribute_name' - The name of the classification attribute.

    # Safety Note

    This is, in its present form, an incomplete and relatively untested implementation. It may fail unexpectedly. Do not use in a production setting.
    "]
    pub fn id3<T: Record>(dataset: Vec<&T>, label_attribute_name: &str, attributes: Vec<&str>, entropy_threshold: f64) -> DecisionVertex {

        // Return labeled leaf if all labels in dataset are equal
        if attr_all_eq(&dataset, label_attribute_name) {
            return Leaf(dataset.iter().next().unwrap().get_attribute(label_attribute_name).to_string());
        }

        // IN PROGRESS
        // Return labeled leaf if entropy of dataset is below threshold
        //if entropy(&dataset, label_attribute_name) < entropy_theshold {
            
        //}

        // Choose attribute to split on
        // Assumes split_attribute() is Some
        let split_attr_name = split_attribute(&dataset, label_attribute_name, &attributes).unwrap();

        let mut branch_map = TreeMap::new();
        // Add a new branch for each possible attribute value
        let attr_domain: HashSet<&str> = dataset.iter().map(|x| x.get_attribute(split_attr_name.as_slice())).collect();
        for attr_value in attr_domain.iter() {
            let example_subset: Vec<&T> = dataset.iter().filter(|x| eq_slice(x.get_attribute(split_attr_name.as_slice()), *attr_value)).map(|x| *x).collect();
            let attrs_left = attributes.iter().filter(|x| !eq_slice(**x, split_attr_name.as_slice())).map(|x| *x).collect();
            let child_vertex = id3(example_subset, label_attribute_name, attrs_left, entropy_threshold);
            branch_map.insert(attr_value.to_string(), child_vertex);
        }

        Branch(split_attr_name.to_string(), branch_map)
    }

    // Checks the given attribute for equality among all records
    fn attr_all_eq<T: Record>(records: &Vec<&T>, attr_name: &str) -> bool {
        let first_attr = records[0].get_attribute(attr_name);
        let all_eq = records.iter().map(|x| x.get_attribute(attr_name)).all(|x| eq_slice(first_attr,x));

        all_eq
    }

    // assumes records.iter().next() is Some
    fn split_attribute<T: Record>(records: &Vec<&T>, label_attr_name: &str, attributes: &Vec<&str>) -> Option<String> {
        let mut min_entropy = f64::MAX_VALUE;
        let mut best_attr = None;

        for attr_name in attributes.iter() {
            // Assuming discrete attributes
            if eq_slice(*attr_name, label_attr_name) { continue; }
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
        let attr_values: HashSet<&str> = attributes.iter().map(|&x| x.val0()).collect();
        for attr_value in attr_values.iter() {
            let mut attr_entropy = 0f64;
            // N_mj
            let attr_count = attributes.iter().filter(|x| eq_slice(x.val0(), *attr_value)).count() as f64;

            // Loop through label values
            let label_values: HashSet<&str> = attributes.iter().filter(|x| x.val0() == *attr_value).map(|x| x.val1()).collect();
            //DEBUG PRINT
            //println!("{}", label_values)
            for label_value in label_values.iter() {
                // N^i_mj
                let label_count = attributes.iter().filter(|x| eq_slice(x.val0(), *attr_value)).filter(|x| x.val1() == *label_value).count() as f64;
                // p^i_mj
                let label_prop = label_count / attr_count;
                attr_entropy = attr_entropy + (label_prop * label_prop.log2())
            }

            total_entropy = total_entropy + ((attr_count / total_attr_count) * attr_entropy);
        }

        (-1f64) * total_entropy
    }

    fn entropy<T: Record>(dataset: &Vec<T>, label_attribute_name: &str) -> f64{
        let mut total_entropy = 0f64;
        let total_count = dataset.len() as f64;
        let labels: Vec<&str> = dataset.iter().map(|x| x.get_attribute(label_attribute_name)).collect();

        let label_values: HashSet<&&str> = labels.iter().collect();
        for label_value in label_values.iter() {
            let count = labels.iter().filter(|x| eq_slice(**label_value, **x)).count() as f64;
            let proportion = count / total_count;
            total_entropy = total_entropy + (proportion * proportion.log2());
        }

        -total_entropy
    }

    #[test]
    fn split_entropy_empty_set() {
        let dataset = vec![];
        let result = split_entropy(dataset);

        assert_eq!(0f64, result);
    }

    #[test]
    fn split_entropy_set_size_one() {
        let dataset = vec![("DrKwint", "y")];
        let result = split_entropy(dataset);

        assert_eq!(0f64, result);
    }

    #[test]
    fn split_entropy_slides_humidity() {
        let dataset = vec![
            ("high", "n"),
            ("high", "n"),
            ("high", "y"),
            ("high", "y"),
            ("normal", "y"),
            ("normal", "n"),
            ("normal", "y"),
            ("high", "n"),
            ("normal", "y"),
            ("normal", "y"),
            ("normal", "y"),
            ("high", "y"),
            ("normal", "y"),
            ("high", "n")];
        let result = split_entropy(dataset);

        assert!(0.788f64 < result && result < 0.789f64);
    }
}
