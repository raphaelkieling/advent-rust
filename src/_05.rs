use std::fs;


#[derive(Clone, Debug)]
struct Rule {
    tail: usize,
    head: usize,
}

struct ProtocolReader {
    ordering_rules: Vec<Rule>,
    updates: Vec<Vec<usize>>,
}

impl ProtocolReader {
    fn new() -> Self {
        return ProtocolReader {
            ordering_rules: vec![],
            updates: vec![],
        };
    }

    fn read(&mut self, path: String) {
        let content = fs::read_to_string(path).expect("File is required");

        for l in content.lines().into_iter() {
            let is_rule = l.contains("|");
            if is_rule {
                let values = l.split("|").collect::<Vec<&str>>();

                let tail = values
                    .get(0)
                    .expect("tail")
                    .parse::<usize>()
                    .expect("tail must be a usize");
                let head = values
                    .get(1)
                    .expect("head")
                    .parse::<usize>()
                    .expect("head must be a usize");

                self.ordering_rules.push(Rule { tail, head });
            }

            let is_update = l.contains(",");
            if is_update {
                let values = l.split(",").collect::<Vec<&str>>();
                let mut curr_updates = vec![];

                for i in values.iter() {
                    let parsed_val = i.parse::<usize>().expect("Must be number");
                    curr_updates.push(parsed_val);
                }

                self.updates.push(curr_updates);
            }
        }

        dbg!(self.updates.clone());
        dbg!(self.ordering_rules.clone());
    }

    fn resolve(&self) -> isize {
        // determine the correct lines
        // sum the middle of each correct lien
        // return the sum of them
        return 0;
    }
}

pub fn execute(path: String) {
    let mut reader = ProtocolReader::new();
    reader.read(path);
    reader.resolve();
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
