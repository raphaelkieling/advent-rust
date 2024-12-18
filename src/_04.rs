use std::fs;

struct XMASReader {
    content: String,
    data: Vec<Vec<String>>
}

impl XMASReader {
     fn new() -> Self {
        return XMASReader {
            content: String::new(),
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
        return 0;
    }
}

pub fn execute(path: String) {
    let mut reader = XMASReader::new();
    reader.read(path);

    println!("Data: {:?}", reader.data);

    let explorer =  XMASExplorer::new(reader);
    explorer.explore();
}