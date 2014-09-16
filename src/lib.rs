pub mod id3 {
    use std::collections::HashMap;

    pub enum ID3Vertex {
        // Branch: Vertex at which the example set is split along an attribute.
        // attribute name, values -> branches
        Branch(String, HashMap<String,ID3Vertex>),
        // Leaf: Terminal vertex whose class has been decided
        // class name
        Leaf(String),
    }

    pub trait Record {
        fn get_attribute(&self, attr_name: &String) -> String;
    }

    // Checks the given attribute for equality among all records
    fn attr_all_eq(records: Vec<&Record>, attr_name: &String) -> bool {
        let first_attr = records[0].get_attribute(attr_name);
        let all_eq = records.iter().map(|x| x.get_attribute(attr_name)).all(|x| first_attr.cmp(&x) == Equal);

        all_eq
    }

    pub fn id3(examples: Vec<&Record>, label_attr_name: &String) -> Option<ID3Vertex> {
        let mut result = None;

        // Return labeled leaf if all labels in examples are equal
        if attr_all_eq(examples, label_attr_name) {
            result = Some(Leaf(label_attr_name.clone()));
        }

        result
    }
}