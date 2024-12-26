use std::{collections::HashMap, fs};

#[derive(Clone, Debug)]
struct Rule {
    tail: usize,
    head: usize,
}

#[derive(Clone, Debug)]
struct UpdateLine {
    data: Vec<usize>,
}

struct CheckResult {
    is_valid: bool,
    possibilities: Vec<UpdateLine>,
}

struct ProtocolReader {
    ordering_rules: Vec<Rule>,
    updates: Vec<UpdateLine>,
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

                self.updates.push(UpdateLine { data: curr_updates });
            }
        }
    }

    fn check_line(&self, line: UpdateLine, counter: &mut HashMap<String, bool>) -> CheckResult {
        let mut is_valid = true;
        for nmr in &line.data {
            // Check if the word has a rule version
            // Find ALL the rules with this head and try
            for rule in self.ordering_rules.iter() {
                if *nmr == rule.head {
                    // Check tail
                    if counter.contains_key(&rule.tail.to_string()) {
                        is_valid = true;
                    } else {
                        // To become invalid, the tail must be in the array
                        let has_but_wrong = line
                            .data
                            .clone()
                            .iter()
                            .filter(|x| {
                                return *x == &rule.tail;
                            })
                            .count();

                        if has_but_wrong > 0 {
                            println!("Trying: {} do not find {}", nmr, rule.tail);
                            is_valid = false;
                        }
                    }
                }
            }
            // Save the word
            counter.insert(nmr.to_string(), true);

            if !is_valid {
                break;
            }
        }

        return CheckResult {
            is_valid,
            possibilities: vec![],
        };
    }

    fn resolve(&self) -> isize {
        let mut valid_lines: Vec<UpdateLine> = vec![];

        // determine the correct lines
        for line in self.updates.iter() {
            println!("-------");
            println!("For: {:?}", line);
            let mut word_counter: HashMap<String, bool> = HashMap::new();
            let result = self.check_line(line.clone(), &mut word_counter);
            if result.is_valid {
                valid_lines.push(line.clone());
            }
        }

        // sum the middle of each correct lien
        let mut middle_sum = 0 as isize;
        valid_lines.iter().for_each(|x| {
            let mid = x.data.len() / 2;
            let val = *x.data.get(mid).expect("Get sum") as isize;
            middle_sum += val;
        });

        // return the sum of them
        return middle_sum;
    }
}

pub fn execute(path: String) {
    let mut reader = ProtocolReader::new();
    reader.read(path);
    let val = reader.resolve();

    println!("Quantity: {}", val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_143_middle_updates() {
        let path = "src/tests/05_143.txt".to_string();
        let mut reader = ProtocolReader::new();
        reader.read(path);
        let val = reader.resolve();

        assert_eq!(val, 143);
    }
}
