#[macro_export]
macro_rules! states {
    ($shape:expr) => {
        {
            use std::fs::File;
            use std::io::prelude::*;
            let filename = format!("assets/states/{}.states", $shape);
            let mut f = File::open(filename).expect("missing states file");
            let mut contents = String::new();
            f.read_to_string(&mut contents).expect("cannot read states file");
            let shapes = contents.trim().split("====\n").map(|s| s.to_string());
            let mut states: Vec<Vec<Vec<bool>>> = vec![];
            for (shape_num, shape) in shapes.enumerate() {
                states.push(vec![]);
                for line in shape.lines() {
                    let bools: Vec<bool> = line.chars()
                        .map(|c| {
                            match c.to_digit(10) {
                                Some(0) => false,
                                Some(1) => true,
                                _ => panic!("bad states files"),
                            }
                        }).collect();
                    states[shape_num].push(bools);
                }
            }
            states
        }
    }
}
