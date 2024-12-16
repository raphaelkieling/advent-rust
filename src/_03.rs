use std::fs;

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
enum CorruptedOpType {
    Do,
    Dont,
    Mul,
}

#[derive(Debug, PartialEq)]
struct CorruptedOp {
    kind: CorruptedOpType,
    raw: String,
    values: Vec<f64>,
}

impl CorruptedOp {
    fn get_result(&self) -> f64 {
        return self.values.iter().fold(1.0, |acc, x| acc * x);
    }
}

struct CorruptedReader {
    content: String,
}

impl CorruptedReader {
    fn new() -> Self {
        return CorruptedReader {
            content: String::new(),
        };
    }

    fn read(&mut self, path: String) {
        let content = fs::read_to_string(path).expect("File is required");
        self.content = content;
    }

    fn get_corrupted_calls(&self) -> Vec<CorruptedOp> {
        let re =
            Regex::new(r"((?<op>(mul|do|don't))\(((?<first>(\d+)),(?<last>(\d+)))?\))").unwrap();

        return re
            .captures_iter(&self.content)
            .map(|x| {
                let raw = x.get(0).unwrap().as_str().to_string();
                let op = x.name("op").unwrap().as_str();

                match op {
                    "mul" => {
                        let a = x.name("first").unwrap().as_str().parse::<f64>().unwrap();
                        let b = x.name("last").unwrap().as_str().parse::<f64>().unwrap();

                        return CorruptedOp {
                            kind: CorruptedOpType::Mul,
                            raw,
                            values: vec![a, b],
                        };
                    }
                    "do" => {
                        return CorruptedOp {
                            kind: CorruptedOpType::Do,
                            raw,
                            values: vec![],
                        };
                    }
                    "don't" => {
                        return CorruptedOp {
                            kind: CorruptedOpType::Dont,
                            raw,
                            values: vec![],
                        };
                    }
                    _ => {
                        return CorruptedOp {
                            kind: CorruptedOpType::Mul,
                            raw,
                            values: vec![],
                        };
                    }
                };
            })
            .collect();
    }

    fn get_result(&self, ops: &Vec<CorruptedOp>) -> f64 {
        let mut val = 0.0;
        let mut current_op = CorruptedOpType::Do;

        for i in ops.iter() {
            if i.kind == CorruptedOpType::Do || i.kind == CorruptedOpType::Dont {
                current_op = i.kind;
                continue;
            }

            if current_op == CorruptedOpType::Dont {
                continue;
            }

            val += i.get_result();
        }

        return val;
    }
}

pub fn execute(path: String) {
    let mut reader = CorruptedReader::new();
    reader.read(path);

    let calls = reader.get_corrupted_calls();
    let result = reader.get_result(&calls);
    println!("Result: {:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_corrupted_calls() {
        let mut reader = CorruptedReader::new();
        reader.read("src/tests/03_161.txt".to_string());
        let calls = reader.get_corrupted_calls();
        let result = reader.get_result(&calls);

        assert_eq!(calls.len(), 4);
        assert_eq!(calls[0].raw, "mul(2,4)");
        assert_eq!(calls[1].raw, "mul(5,5)");
        assert_eq!(calls[2].raw, "mul(11,8)");
        assert_eq!(calls[3].raw, "mul(8,5)");
        assert_eq!(result, 161.0);
    }

    #[test]
    fn get_corrupted_calls_using_do_dont() {
        let mut reader = CorruptedReader::new();
        reader.read("src/tests/03_do.txt".to_string());
        let calls = reader.get_corrupted_calls();
        let result = reader.get_result(&calls);

        assert_eq!(calls.len(), 6);
        assert_eq!(result, 48.0);
    }

    #[test]
    fn get_0_since_all_wrong() {
        let mut reader = CorruptedReader::new();
        reader.read("src/tests/03_0.txt".to_string());
        let calls = reader.get_corrupted_calls();
        let result = reader.get_result(&calls);

        assert_eq!(result, 0.0);
    }

    #[test]
    fn partial() {
        let mut reader = CorruptedReader::new();
        reader.read("src/tests/03_partial.txt".to_string());
        let calls = reader.get_corrupted_calls();
        println!("Calls: {:?}", calls);
        let result = reader.get_result(&calls);

        assert_eq!(result, 971324.0);
    }

    #[test]
    fn get_result_of_corruped_call() {
        let reader = CorruptedReader::new();
        let val = reader.get_result(&vec![
            CorruptedOp {
                kind: CorruptedOpType::Mul,
                raw: "mul(2,4)".to_string(),
                values: vec![3.0, 4.0],
            },
            CorruptedOp {
                kind: CorruptedOpType::Mul,
                raw: "mul(2,4)".to_string(),
                values: vec![3.0, 4.0],
            },
        ]);

        assert_eq!(val, 24.0);
    }
}
