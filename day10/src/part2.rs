use good_lp::{Expression, ProblemVariables, Solution, SolverModel, variable};

use crate::Machine;

// using ILP solver for part 2
pub fn find_min_button_presses(m: &Machine) -> u32 {
    // create the variables for the solver
    let mut xs = Vec::new();
    let mut vars = ProblemVariables::new();
    for _ in &m.buttons {
        xs.push(vars.add(variable().integer().min(0)));
    }

    // minimize the button presses (sum of values on the xs vector)
    let mut problem = vars
        .minimise(xs.iter().sum::<Expression>())
        .using(good_lp::highs);

    // expressions: joltage counter values come from the button presses
    let mut exps = vec![Expression::with_capacity(m.buttons.len()); m.joltages.len()];
    for (i, b) in m.buttons.iter().enumerate() {
        for w in &b.wiring {
            exps[*w] += xs[i];
        }
    }

    // constraint: desired joltages must match the result joltages
    for (i, e) in exps.into_iter().enumerate() {
        let j = m.joltages[i];
        problem.add_constraint(e.eq(j as f64));
    }

    let solution = problem.solve().unwrap();
    xs.iter().map(|&v| solution.value(v)).sum::<f64>() as u32
}
