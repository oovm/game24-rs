use crate::trees::{Maybe32, V2O4, V3O4, V4O4};

pub fn solvable4(input: &[i32], target: i32) -> bool {
    match input.len() {
        0 => unimplemented!(),
        1 => input[0] == target,
        2 => {
            for f in V2O4.iter() {
                if f(Maybe32::from(input[0]), Maybe32::from(input[1])) == target {
                    return true;
                }
            }
            return false;
        }
        3 => {
            for f in V3O4.iter() {
                if f(Maybe32::from(input[0]), Maybe32::from(input[1]), Maybe32::from(input[2])) == target {
                    return true;
                }
            }
            return false;
        }
        4 => {
            for f in V4O4.iter() {
                if f(Maybe32::from(input[0]), Maybe32::from(input[1]), Maybe32::from(input[2]), Maybe32::from(input[3]))
                    == target
                {
                    return true;
                }
            }
            return false;
        }
        _ => unimplemented!(),
    }
}
