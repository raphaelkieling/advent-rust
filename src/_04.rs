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
    reader: XMASReader
}

impl XMASExplorer {
    fn new(reader: XMASReader) -> Self {
        return XMASExplorer { reader };
    }

    fn explore(&self) -> i64 {
        let mut valids: i64 = 0; 
        let size = 4;
        let data = self.reader.data.clone();
        let mut all_possibilites: Vec<String> = Vec::new();

        for (yx, yv) in self.reader.data.iter().enumerate() {
            for (xx, xv) in yv.iter().enumerate() {
                let mut possibilities : Vec<String> = vec![];
                let xxi = xx as isize;
                let yxi = yx as isize;

                // Check right
                if (xx + size) < yv.len() {
                    let mut is_right:String = String::new();
                    is_right.push_str(xv);
                    is_right.push_str(&yv[xx + 1]);
                    is_right.push_str(&yv[xx + 2]);
                    is_right.push_str(&yv[xx + 3]);

                    possibilities.push(is_right.clone());
                    possibilities.push(is_right.clone().chars().rev().collect());
                }
                // Check left
                if (xxi - size as isize) > 0  {
                    let mut is_left:String = String::new();
                    is_left.push_str(xv);
                    is_left.push_str(&yv[xx - 1]);
                    is_left.push_str(&yv[xx - 2]);
                    is_left.push_str(&yv[xx - 3]);

                    possibilities.push(is_left.clone());
                    possibilities.push(is_left.clone().chars().rev().collect());
                    
                }

                // Check up
                if (yx + size) < self.reader.data.len() {
                    let mut is_up:String = String::new();
                    is_up.push_str(&data[yx][xx]);
                    is_up.push_str(&data[yx + 1][xx]);
                    is_up.push_str(&data[yx + 2][xx]);
                    is_up.push_str(&data[yx + 3][xx]);

                    possibilities.push(is_up.clone());
                    possibilities.push(is_up.clone().chars().rev().collect());
                }

                // Check down
                if (yxi - size as isize) > 0 {
                    let mut is_down:String = String::new();
                    is_down.push_str(&data[yx][xx]);
                    is_down.push_str(&data[yx - 1][xx]);
                    is_down.push_str(&data[yx - 2][xx]);
                    is_down.push_str(&data[yx - 3][xx]);

                    possibilities.push(is_down.clone());
                    possibilities.push(is_down.clone().chars().rev().collect());
                }

                all_possibilites.extend(possibilities);
            }
        }

        println!("A: {:?}", all_possibilites);

        return valids;
    }
}

pub fn execute(path: String) {
    let mut reader = XMASReader::new();
    reader.read(path);

    let explorer =  XMASExplorer::new(reader);
    let val = explorer.explore();

    println!("Quantity: {:?}", val);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_quantity() {
        let mut reader = XMASReader::new();
        reader.read("src/tests/04_18.txt".to_string());

        let explorer =  XMASExplorer::new(reader);
        let val = explorer.explore();

        assert_eq!(val, 18);
    }
}