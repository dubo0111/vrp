use crate::construction::states::InsertionContext;
use crate::helpers::get_test_resource;
use crate::helpers::streams::input::create_c101_100_problem;
use crate::refinement::objectives::{Objective, PenalizeUnassigned};
use crate::streams::input::text::read_init_solution;
use crate::utils::DefaultRandom;
use std::io::BufReader;
use std::sync::Arc;

#[test]
pub fn can_read_init_solution() {
    let problem = Arc::new(create_c101_100_problem());
    let file = get_test_resource("data/solomon/C101.100.best.txt").unwrap();

    let result = read_init_solution(BufReader::new(file), problem.clone());

    assert!(result.is_ok());
    let solution = Arc::new(result.unwrap());
    assert_eq!(solution.routes.len(), 10);
    assert_eq!(
        PenalizeUnassigned::default()
            .estimate(&InsertionContext::new_from_solution(problem, (solution, None), Arc::new(DefaultRandom::new())))
            .total()
            .round(),
        828.936f64.round()
    );
}
