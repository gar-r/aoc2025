use crate::{input::Problem, shape::Shape};

pub fn can_fit(p: &Problem, shapes: &Vec<Shape>) -> Result<bool, String> {
    // check if it is impossible to fit the shapes due to space constraint
    let (width, height) = (p.area.0 as usize, p.area.1 as usize);
    let available = width * height;
    let mut needed = 0;
    for (i, shape) in shapes.iter().enumerate() {
        needed += shape.area() * p.gifts[i] as usize;
    }
    if available < needed {
        return Ok(false);
    }

    // check if it is trivially possible to fit all shapes
    let mut needed = 0;
    for g in &p.gifts {
        needed += (*g as usize) * 9; // assume every gift is placed in uniform 3x3 boxes
    }
    if needed <= available {
        return Ok(true);
    }

    Err(String::from("unable to solve"))
}
