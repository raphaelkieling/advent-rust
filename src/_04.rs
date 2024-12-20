use std::fs;

#[derive(Clone)]
struct XMASReader {
    data: Vec<Vec<String>>,
}

impl XMASReader {
    fn new() -> Self {
        return XMASReader { data: vec![] };
    }

    fn read(&mut self, path: String) {
        let content = fs::read_to_string(path).expect("File is required");
        for l in content.lines().into_iter() {
            let mut vex: Vec<String> = Vec::new();
            for i in l.chars().into_iter() {
                vex.push(i.to_string());
            }
            self.data.push(vex);
        }
    }
}

struct XMASv2Explorer {
    reader: XMASReader,
}

impl XMASv2Explorer {
    fn new(reader: XMASReader) -> Self {
        return XMASv2Explorer { reader };
    }

    fn explore(&self) -> usize {
        let mut count_valids = 0;
        let valids = vec!["MAS", "SAM"];

        let size = 2;
        let data = self.reader.data.clone();

        for (yx, yv) in self.reader.data.iter().enumerate() {
            for (xx, xv) in yv.iter().enumerate() {
                let mut possibilities: Vec<String> = vec![];
                let can_down = (yx + size) < self.reader.data.len();
                let can_right = (xx + size) < yv.len();

                // Diagonal RIGHT + DOWN
                if can_right && can_down {
                    let mut is_diagonal_right_down: String = String::new();
                    is_diagonal_right_down.push_str(&xv);
                    is_diagonal_right_down.push_str(&data[yx + 1][xx + 1]);
                    is_diagonal_right_down.push_str(&data[yx + 2][xx + 2]);

                    possibilities.push(is_diagonal_right_down.clone());
                }

                // Diagonal LEFT + DOWN
                if can_right && can_down {
                    let mut is_diagonal_left_down: String = String::new();
                    is_diagonal_left_down.push_str(&data[yx][xx + 2]);
                    is_diagonal_left_down.push_str(&data[yx + 1][xx + 1]);
                    is_diagonal_left_down.push_str(&data[yx + 2][xx]);

                    possibilities.push(is_diagonal_left_down.clone());
                }

                let mut count_curr_valid = 0;
                for p in possibilities.clone() {
                    if valids.iter().any(|&x| x == p) {
                        count_curr_valid += 1;
                    }
                }

                if count_curr_valid >= 2 {
                    count_valids += 1;
                }
            }
        }

        return count_valids;
    }
}

struct XMASExplorer {
    reader: XMASReader,
}

impl XMASExplorer {
    fn new(reader: XMASReader) -> Self {
        return XMASExplorer { reader };
    }

    fn explore(&self) -> usize {
        let mut count_valids = 0;
        let valids = vec!["XMAS", "SAMX"];

        let size = 3;
        let data = self.reader.data.clone();

        for (yx, yv) in self.reader.data.iter().enumerate() {
            for (xx, xv) in yv.iter().enumerate() {
                let mut possibilities: Vec<String> = vec![];
                let xxi = xx as isize;

                let can_down = (yx + size) < self.reader.data.len();
                let can_right = (xx + size) < yv.len();
                let can_left = (xxi - size as isize) >= 0;

                // Check right
                if can_right {
                    let mut is_right: String = String::new();
                    is_right.push_str(xv);
                    is_right.push_str(&yv[xx + 1]);
                    is_right.push_str(&yv[xx + 2]);
                    is_right.push_str(&yv[xx + 3]);

                    possibilities.push(is_right.clone());
                }

                // Check down
                if can_down {
                    let mut is_down: String = String::new();
                    is_down.push_str(&data[yx][xx]);
                    is_down.push_str(&data[yx + 1][xx]);
                    is_down.push_str(&data[yx + 2][xx]);
                    is_down.push_str(&data[yx + 3][xx]);

                    possibilities.push(is_down.clone());
                }

                // Diagonal RIGHT + DOWN
                if can_right && can_down {
                    let mut is_diagonal_right_down: String = String::new();
                    is_diagonal_right_down.push_str(&data[yx][xx]);
                    is_diagonal_right_down.push_str(&data[yx + 1][xx + 1]);
                    is_diagonal_right_down.push_str(&data[yx + 2][xx + 2]);
                    is_diagonal_right_down.push_str(&data[yx + 3][xx + 3]);

                    possibilities.push(is_diagonal_right_down.clone());
                }

                // Diagonal LEFT + DOWN
                if can_left && can_down {
                    let mut is_diagonal_left_down: String = String::new();
                    is_diagonal_left_down.push_str(&data[yx][xx]);
                    is_diagonal_left_down.push_str(&data[yx + 1][xx - 1]);
                    is_diagonal_left_down.push_str(&data[yx + 2][xx - 2]);
                    is_diagonal_left_down.push_str(&data[yx + 3][xx - 3]);

                    possibilities.push(is_diagonal_left_down.clone());
                }

                for i in possibilities.clone().iter() {
                    if valids.iter().any(|&x| x == i) {
                        count_valids += 1;
                    }
                }
            }
        }

        return count_valids;
    }
}

pub fn execute(path: String) {
    let mut reader = XMASReader::new();
    reader.read(path);

    let explorer = XMASExplorer::new(reader.clone());
    let val = explorer.explore();

    println!("Quantity: {:?}", val);

    let explorer_v2 = XMASv2Explorer::new(reader);
    let val_v2 = explorer_v2.explore();

    println!("Quantity V2: {:?}", val_v2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_quantity() {
        let mut reader = XMASReader::new();
        reader.read("src/tests/04_18.txt".to_string());

        // Part 1
        let explorer = XMASExplorer::new(reader);
        let val = explorer.explore();

        assert_eq!(val, 18);
    }

    #[test]
    fn get_quantity_v2() {
        let mut reader = XMASReader::new();
        reader.read("src/tests/04_x-mas.txt".to_string());

        // Part 2
        let explorer = XMASv2Explorer::new(reader);
        let val = explorer.explore();

        assert_eq!(val, 9);
    }
}
