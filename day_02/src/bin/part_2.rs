use day_02::{parse_reports, safe_report_with_problem_dampener};

fn main() {
    let reports = parse_reports(include_str!("input.txt"));

    let num_safe_reports = reports
        .into_iter()
        .map(|report| safe_report_with_problem_dampener(&report))
        .filter(|is_safe| *is_safe)
        .count();

    println!("{}", num_safe_reports);
}
