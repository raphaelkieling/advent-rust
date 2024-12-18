use std::fs;

struct XMASReader {
    data: Vec<Vec<String>>
}

impl XMASReader {
     fn new() -> Self {
        return XMASReader {
            data: vec![]
        };
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

struct XMASExplorer {
    reader: XMASReader,
    all_possibilities: Vec<String>
}

impl XMASExplorer {
    fn new(reader: XMASReader) -> Self {
        return XMASExplorer { reader, all_possibilities: vec![] };
    }

    fn explore(&mut self) -> &Self {
        let size = 4;
        let data = self.reader.data.clone();

        for (yx, yv) in self.reader.data.iter().enumerate() {
            for (xx, xv) in yv.iter().enumerate() {
                let mut possibilities : Vec<String> = vec![];
                let yxi = yx as isize;

                let can_down = (yx + size) <= self.reader.data.len();
                let can_right = (xx + size) <= yv.len();
                let can_up = (yxi - size as isize) > 0;

                // Check right
                if can_right {
                    let mut is_right:String = String::new();
                    is_right.push_str(xv);
                    is_right.push_str(&yv[xx + 1]);
                    is_right.push_str(&yv[xx + 2]);
                    is_right.push_str(&yv[xx + 3]);

                    possibilities.push(is_right.clone());
                }

                // Check down
                if can_down {
                    let mut is_down:String = String::new();
                    is_down.push_str(&data[yx][xx]);
                    is_down.push_str(&data[yx + 1][xx]);
                    is_down.push_str(&data[yx + 2][xx]);
                    is_down.push_str(&data[yx + 3][xx]);

                    possibilities.push(is_down.clone());
                }


                // Diagonal RIGHT + DOWN
                if can_right && can_down {
                    let mut is_diagonal_right_down:String = String::new();
                    is_diagonal_right_down.push_str(&data[yx][xx]);
                    is_diagonal_right_down.push_str(&data[yx + 1][xx + 1]);
                    is_diagonal_right_down.push_str(&data[yx + 2][xx + 2]);
                    is_diagonal_right_down.push_str(&data[yx + 3][xx + 3]);

                    possibilities.push(is_diagonal_right_down.clone());
                }

                // Diagonal RIGHT + UP
                if can_right && can_up {
                    let mut is_diagonal_right_up:String = String::new();
                    is_diagonal_right_up.push_str(&data[yx][xx]);
                    is_diagonal_right_up.push_str(&data[yx - 1][xx + 1]);
                    is_diagonal_right_up.push_str(&data[yx - 2][xx + 2]);
                    is_diagonal_right_up.push_str(&data[yx - 3][xx + 3]);

                    possibilities.push(is_diagonal_right_up.clone());
                }
                

                self.all_possibilities.extend(possibilities);
            }
        }

        println!("A: {:?}", self.all_possibilities);

        return self;
    }

    fn count_valid(&self) -> isize {
        let mut count_valids = 0;
        let valids = vec![
            "XMAS",
            "SAMX"
        ];

        for i in self.all_possibilities.iter() {
           if valids.iter().any(|&x| x == i) {
                count_valids += 1;
           }
        }

        return count_valids;
    }
}

pub fn execute(path: String) {
    let mut reader = XMASReader::new();
    reader.read(path);

    let mut explorer =  XMASExplorer::new(reader);
    let val = explorer.explore().count_valid();

    println!("Quantity: {:?}", val);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_quantity() {
        let mut reader = XMASReader::new();
        reader.read("src/tests/04_18.txt".to_string());

        let mut explorer =  XMASExplorer::new(reader);
        let val = explorer.explore().count_valid();

        assert_eq!(val, 18);
    }
}