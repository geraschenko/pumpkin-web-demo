//! Minimal demo of pumpkin-core running in WebAssembly.
//!
//! This uses a patched pumpkin-core that replaces `std::time` with `web-time` for
//! WASM compatibility. See: https://github.com/geraschenko/Pumpkin

use pumpkin_core::{
    constraints::{self, Constraint},
    results::{ProblemSolution, SatisfactionResult},
    termination::Indefinite,
    Solver,
};
#[allow(unused_imports)]
use Constraint as _; // trait needed for .post()
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn init() {
    console_error_panic_hook::set_once();
}

/// Solve the constraint: x + y = target
/// where x is in [min_x, max_x] and y is in [min_y, max_y].
///
/// Returns a JSON string with the solution or error message.
#[wasm_bindgen]
pub fn solve_sum(min_x: i32, max_x: i32, min_y: i32, max_y: i32, target: i32) -> String {
    let mut solver = Solver::default();

    let x = solver.new_bounded_integer(min_x, max_x);
    let y = solver.new_bounded_integer(min_y, max_y);

    let tag = solver.new_constraint_tag();

    // Constraint: x + y = target
    if solver
        .add_constraint(constraints::equals(vec![x, y], target, tag))
        .post()
        .is_err()
    {
        return r#"{"error": "No solution exists"}"#.to_string();
    }

    let mut brancher = solver.default_brancher();
    let mut termination = Indefinite;

    // Extract result before solver/brancher go out of scope
    let result = match solver.satisfy(&mut brancher, &mut termination) {
        SatisfactionResult::Satisfiable(solution) => {
            let x_val = solution.solution().get_integer_value(x);
            let y_val = solution.solution().get_integer_value(y);
            format!(r#"{{"x": {}, "y": {}}}"#, x_val, y_val)
        }
        SatisfactionResult::Unsatisfiable(_, _) => r#"{"error": "No solution exists"}"#.to_string(),
        SatisfactionResult::Unknown(_, _) => {
            r#"{"error": "Search terminated without finding a solution"}"#.to_string()
        }
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_sum_basic() {
        let result = solve_sum(1, 10, 1, 10, 12);
        assert!(result.contains("\"x\":"));
        assert!(result.contains("\"y\":"));
    }

    #[test]
    fn test_solve_sum_impossible() {
        // x in [1,2], y in [1,2], target=100 is impossible
        let result = solve_sum(1, 2, 1, 2, 100);
        assert!(result.contains("error"));
    }
}
