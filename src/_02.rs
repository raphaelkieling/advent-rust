use std::fs;

#[derive(Debug)]
struct Level {
    content: Vec<f32>,
}

#[derive(Eq, PartialEq, Debug)]
enum LevelConsistence {
    Decreasing,
    Increasing,
    NotDefined,
}

impl Level {
    fn check_save(&self, content: &Vec<f32>) -> bool {
        let mut def = LevelConsistence::NotDefined;
        let mut last_value = content[0].clone();

        for (i, v) in content.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let diff = (v - last_value).abs();

            if !(diff >= 1.0 && diff <= 3.0) {
                return false;
            }

            if *v == last_value {
                return false;
            }

            if *v > last_value {
                if def == LevelConsistence::Decreasing {
                    return false;
                }

                def = LevelConsistence::Increasing;
            }

            if *v < last_value {
                if def == LevelConsistence::Increasing {
                    return false;
                }

                def = LevelConsistence::Decreasing;
            }

            last_value = *v;
        }

        return true;
    }

    fn is_safe(&self) -> bool {
        // Check if valid
        let is_valid = self.check_save(&self.content);
        if is_valid {
            return true;
        }

        // Try to skip each one of the values
        for skip_index in 0..self.content.len() {
            let content_without_skip = &self
                .content
                .iter()
                .enumerate()
                .filter(|(i, _)| return *i != skip_index)
                .map(|(_, v)| return *v)
                .collect();

            if self.check_save(content_without_skip) {
                return true;
            }
        }

        return false;
    }
}

#[derive(Debug)]
struct Report {
    levels: Vec<Level>,
}

impl Report {
    fn new() -> Self {
        return Report { levels: Vec::new() };
    }

    fn get_count(&self) -> i32 {
        return self.levels.iter().filter(|x| return x.is_safe()).count() as i32;
    }

    fn read(&mut self, path: String) -> &Self {
        let content = fs::read_to_string(path).expect("File is required");
        let lines = content.lines();

        for l in lines.into_iter() {
            let values = l
                .split(" ")
                .map(|x| return x.parse::<f32>().unwrap())
                .collect();

            self.levels.push(Level { content: values });
        }

        return self;
    }
}

pub fn execute(path: String) {
    let mut report = Report::new();
    report.read(path);

    println!("Count safe: {:?}", report.get_count());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn return_2() {
        let path = "src/tests/02_2.txt".to_string();
        let mut report = Report::new();
        report.read(path);

        assert_eq!(report.get_count(), 1);
    }

    #[test]
    fn return_4() {
        let path = "src/tests/02_4.txt".to_string();
        let mut report = Report::new();
        report.read(path);

        assert_eq!(report.get_count(), 4);
    }

    #[test]
    fn resolve_first_skip() {
        let report2 = Report {
            levels: Vec::from([Level {
                content: Vec::from([6.0, 2.0, 1.0]),
            }]),
        };

        assert_eq!(report2.get_count(), 1);
    }
}
