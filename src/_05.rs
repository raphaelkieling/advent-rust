use std::fs;

struct Rule {
    tail: usize,
    head: usize,
}

struct ProtocolReader {
    ordering_rules: Vec<Rule>,
    updates: Vec<usize>,

    raw_updates: Vec<String>,
    raw_ordering_rules: Vec<String>,
}

impl ProtocolReader {
    fn new() -> Self {
        return ProtocolReader {
            ordering_rules: vec![],
            updates: vec![],

            raw_updates: vec![],
            raw_ordering_rules: vec![],
        };
    }

    fn read(&mut self, path: String) {
        let content = fs::read_to_string(path).expect("File is required");

        for l in content.lines().into_iter() {
            let is_rule = l.contains("|");
            if is_rule {
                self.raw_ordering_rules.push(l.to_string());
            }

            let is_sequence = l.contains(",");
            if is_sequence {
                self.raw_updates.push(l.to_string());
            }
        }
    }

    fn resolve(&mut self) {
        for u in self.raw_updates.iter() {
            let val = u.split(",").collect();

            //val.parse::<usize>().expect("Must be number");
            self.updates.push(val);
        }

        dbg!(&self.updates);
    }
}

pub fn execute(path: String) {
    let mut reader = ProtocolReader::new();
    reader.read(path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_143_middle_updates() {
        let path = "src/tests/05_143.txt".to_string();
        let mut reader = ProtocolReader::new();
        reader.read(path);
        reader.resolve();

        assert_eq!(0, 143);
    }
}
