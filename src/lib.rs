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
            let states: Vec<String> = contents.trim().split("====\n")
                .map(|s| s.to_string())
                .collect();
            states
        }
    }
}
