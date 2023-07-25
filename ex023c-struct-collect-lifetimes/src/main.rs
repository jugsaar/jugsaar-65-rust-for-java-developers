use std::collections::HashMap;

struct Data {
    value: String,
}

impl Data {
    fn new(value: String) -> Data {
        Data { value }
    }

    fn value(&self) -> &str {
        &self.value
    }
}

struct MapDemo<'a> {
    map: HashMap<String, &'a Data>,
}

impl<'a> MapDemo<'a> {
    fn new() -> MapDemo<'a> {
        MapDemo {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: &'a Data) {
        self.map.insert(key, value);
    }

    fn load(&self, key: &str) -> Option<&'a str> {
        self.map.get(key).map(|data| data.value())
    }

    fn remove(&mut self, key: &str) -> bool {
        self.map.remove(key).is_some()
    }
}

fn main() {
    let mut demo = MapDemo::new();

    let data1 = Data::new("1".to_string());
    demo.insert("A".to_string(), &data1);

    let data2 = Data::new("2".to_string());
    demo.insert("B".to_string(), &data2);

    if let Some(a_value) = demo.load("A") {
        println!("A Value: {}", a_value);
    }

    let removed = demo.remove("B");
    println!("B removed: {}", removed);

    if let Some(b_value) = demo.load("B") {
        println!("B Value: {}", b_value);
    } else {
        println!("B not found.");
    }
}
