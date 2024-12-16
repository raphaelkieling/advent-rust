use std::{collections::HashMap, fs};

struct HistorianReader {
    a: Vec<f32>,
    b: Vec<f32>,
    b_occurence: HashMap<String, i32>,
    content: String,
}

impl HistorianReader {
    fn new() -> HistorianReader {
        HistorianReader {
            a: Vec::new(),
            b: Vec::new(),
            b_occurence: HashMap::new(),
            content: String::new(),
        }
    }

    fn read(&mut self, file: String) -> &mut Self {
        // Read the file
        self.content = fs::read_to_string(file).expect("The file is required");
        let lines = self.content.lines();

        // Parse the file
        for line in lines.into_iter() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let a = parts[0].parse::<f32>().expect("A valid number");
            let b = parts[1].parse::<f32>().expect("A valid number");
            self.a.push(a);
            self.b.push(b);

            // Add occurrence of map
            let key = parts[1].to_string();
            match self.b_occurence.get(&key) {
                Some(count) => self.b_occurence.insert(key, count + 1),
                None => self.b_occurence.insert(key, 1),
            };
        }

        // Store the data
        return self;
    }

    fn sort(&mut self) -> &Self {
        self.a.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.b.sort_by(|a, b| a.partial_cmp(b).unwrap());
        return self;
    }
}

struct Historian {
    reader: HistorianReader,
}

#[derive(Debug)]
struct HistorianResult {
    sum: f32,
    score: f32,
}

impl Historian {
    fn new(reader: HistorianReader) -> Historian {
        Historian { reader }
    }

    fn get_sum(&self) -> f32 {
        let mut sum = 0.0;
        for (i, _) in self.reader.a.iter().enumerate() {
            sum += (self.reader.a[i] - self.reader.b[i]).abs();
        }

        return sum;
    }

    fn get_score(&self) -> f32 {
        let mut sum = 0.0;

        for v in self.reader.a.iter() {
            let val = *self.reader.b_occurence.get(&v.to_string()).unwrap_or(&0) as f32;
            sum += v * val;
        }

        return sum;
    }

    fn compute(&self) -> HistorianResult {
        return HistorianResult {
            sum: self.get_sum(),
            score: self.get_score(),
        };
    }
}

pub fn execute(path: String) {
    // Get the file
    let mut reader = HistorianReader::new();
    reader.read(path).sort();

    // Compute
    let historian = Historian::new(reader);
    let result = historian.compute();

    // Show
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_is_2() {
        let reader = HistorianReader {
            a: Vec::from([0.0, 1.0]),
            b: Vec::from([0.0, 3.0]),
            b_occurence: HashMap::new(),
            content: String::new(),
        };
        let h = Historian::new(reader);
        let result = h.compute();

        assert_eq!(result.sum, 2.0);
    }

    #[test]
    fn score_is_10() {
        let reader = HistorianReader {
            a: Vec::from([0.0, 5.0]),
            b: Vec::from([0.0, 5.0, 5.0]),
            b_occurence: HashMap::from([("5".to_string(), 2)]),
            content: String::new(),
        };
        let h = Historian::new(reader);
        let result = h.compute();

        assert_eq!(result.score, 10.0);
    }
}
