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

    fn check_line(&self, line: UpdateLine, get_possibilities: bool) -> CheckResult {
        let mut counter: HashMap<String, bool> = HashMap::new();
        let mut possibilities: Vec<UpdateLine> = vec![];
        let mut is_valid = true;

        for (_ix, nmr) in line.data.iter().enumerate() {
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
                            if get_possibilities {
                                println!("Trying: {} do not find {}", nmr, rule.tail);
                                let mut new_possibility: Vec<usize> = line.data.clone();

                                // Add the possibility
                                // 1. Remove the head
                                if let Some(index) =
                                    new_possibility.iter().position(|&x| x == rule.head)
                                {
                                    new_possibility.remove(index);
                                }

                                // 2. Add the head after the tail.
                                if let Some(index) =
                                    new_possibility.iter().position(|&x| x == rule.tail)
                                {
                                    new_possibility.insert(index + 1, rule.head);
                                }

                                // 3. Add the new possitibility
                                let new_update_line = UpdateLine {
                                    data: new_possibility.clone(),
                                };
                                possibilities.push(new_update_line.clone());

                                // 4. Get new possibilities
                                let result_possibility = self.check_line(new_update_line, true);
                                if result_possibility.is_valid {
                                    if result_possibility.possibilities.len() > 0 {
                                        possibilities.extend(result_possibility.possibilities);
                                    }
                                }

                                println!("For this, now is: {:?}", new_possibility);
                            }

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
            possibilities,
        };
    }

    fn resolve(&self) -> isize {
        let mut valid_lines: Vec<UpdateLine> = vec![];

        // determine the correct lines
        for line in self.updates.iter() {
            println!("-------");
            println!("For: {:?}", line);
            let result = self.check_line(line.clone(), true);
            if result.possibilities.len() > 0 {
                for possibility in result.possibilities.iter() {
                    println!(">>>>>>>>>>>>>");
                    println!("For: {:?}", possibility);

                    let result_pos = self.check_line(possibility.clone(), false);
                    if result_pos.is_valid {
                        println!("The final valid is: {:?}", possibility);
                        valid_lines.push(possibility.clone());
                    }
                }
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
    fn get_123_middle_updates() {
        let path = "src/tests/05_143.txt".to_string();
        let mut reader = ProtocolReader::new();
        reader.read(path);
        let val = reader.resolve();

        assert_eq!(val, 123);
    }
}
