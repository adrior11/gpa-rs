use super::{Course, GradingSystem};

/// Utility struct to accumulate statistics about courses and its grading.
#[derive(Default)]
pub struct Stats {
    pub all: u16,
    pub graded: u16,
    pub points_only: u16,
    pub weighted: f32,
}

impl Stats {
    pub fn add(&mut self, c: &Course, sys: &GradingSystem) {
        self.all += c.credits;

        // NOTE: explicitly differentiate between false and true for completed
        match (c.grade, c.completed) {
            (Some(g), _) if sys.ignore_failed && !sys.is_pass(g) => { /* ignore */ }
            (Some(g), _) => {
                self.weighted += g * f32::from(c.credits);
                self.graded += c.credits;
            }
            (None, true) => self.points_only += c.credits,
            _ => { /* ignore */ }
        }
    }

    pub fn avg(&self) -> f32 {
        if self.graded > 0 {
            self.weighted / f32::from(self.graded)
        } else {
            0.0
        }
    }

    pub const fn total(&self) -> u16 {
        self.graded + self.points_only
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_accumulation_and_average() {
        let mut s = Stats::default();
        let sys = GradingSystem::default();
        let courses = [
            Course::new("Algorithms".to_string(), 6, 1, Some(2.3), true), // passed
            Course::new("Data Structures".to_string(), 8, 1, Some(5.0), true), //failed
            Course::new("Project".to_string(), 12, 2, None, true),        // passed (ungraded)
        ];

        courses.iter().for_each(|c| s.add(c, &sys));

        assert_eq!(s.graded, 6);
        assert_eq!(s.points_only, 12);
        assert_eq!(s.all, 26);
        assert_eq!(s.avg(), 2.3);
        assert_eq!(s.total(), 18);
    }

    #[test]
    fn stats_avg_with_no_courses() {
        let s = Stats::default();

        assert_eq!(s.graded, 0);
        assert_eq!(s.points_only, 0);
        assert_eq!(s.all, 0);
        assert_eq!(s.avg(), 0.0);
        assert_eq!(s.total(), 0);
    }

    #[test]
    fn stats_avg_with_no_graded() {
        let mut s = Stats::default();
        let sys = GradingSystem::default();
        let courses = [
            Course::new("Algorithms".to_string(), 6, 1, None, true), // ungraded but completed
            Course::new("Data Structures".to_string(), 8, 1, None, false), // not completed
        ];

        courses.iter().for_each(|c| s.add(c, &sys));

        assert_eq!(s.graded, 0);
        assert_eq!(s.points_only, 6);
        assert_eq!(s.all, 14);
        assert_eq!(s.avg(), 0.0);
        assert_eq!(s.total(), 6);
    }
}
