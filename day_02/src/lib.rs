pub fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

enum SafetyDetectionState {
    Init,
    Increasing,
    Decreasing,
}

pub fn safe_report(report: &[i32]) -> bool {
    let mut detection_state = SafetyDetectionState::Init;

    let mut report_iter = report.into_iter().peekable();

    while let Some(level) = report_iter.next() {
        let Some(next_level) = report_iter.peek() else {
            continue;
        };

        let level_difference = level - *next_level;

        match detection_state {
            SafetyDetectionState::Init => match level_difference {
                -3..=-1 => detection_state = SafetyDetectionState::Increasing,
                0 => return false,
                1..=3 => detection_state = SafetyDetectionState::Decreasing,
                4.. => return false,
                _ => return false,
            },
            SafetyDetectionState::Increasing => {
                if level_difference < -3 || level_difference > -1 {
                    return false;
                }
            }
            SafetyDetectionState::Decreasing => {
                if level_difference > 3 || level_difference < 1 {
                    return false;
                }
            }
        }
    }

    return true;
}

pub fn safe_report_with_problem_dampener(report: &[i32]) -> bool {
    if safe_report(report) {
        return true;
    }

    for (i, _) in report.into_iter().enumerate() {
        let mut possibly_safe = Vec::with_capacity(report.len());
        possibly_safe.extend_from_slice(report);
        possibly_safe.remove(i);

        if safe_report(&possibly_safe) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let safe = safe_report(&[7, 6, 4, 2, 1]);
        assert_eq!(safe, true);
    }

    #[test]
    fn example_2() {
        let safe = safe_report(&[1, 2, 7, 8, 9]);
        assert_eq!(safe, false);
    }

    #[test]
    fn example_3() {
        let safe = safe_report(&[9, 7, 6, 2, 1]);
        assert_eq!(safe, false);
    }

    #[test]
    fn example_4() {
        let safe = safe_report(&[1, 3, 2, 4, 5]);
        assert_eq!(safe, false);
    }

    #[test]
    fn example_5() {
        let safe = safe_report(&[8, 6, 4, 4, 1]);
        assert_eq!(safe, false);
    }

    #[test]
    fn example_6() {
        let safe = safe_report(&[1, 3, 6, 7, 9]);
        assert_eq!(safe, true);
    }

    #[test]
    fn test_possibly_safe_one_bad_level() {
        let safe = safe_report_with_problem_dampener(&[1, 3, 2, 4, 5]);
        assert_eq!(safe, true);
    }

    #[test]
    fn test_possibly_safe_cant_fix() {
        let safe = safe_report_with_problem_dampener(&[9, 7, 6, 2, 1]);
        assert_eq!(safe, false);
    }
}
