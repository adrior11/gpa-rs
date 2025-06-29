use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradingSystem {
    pub credit_goal: u16,
    pub ignore_failed: bool,
    pub lower_is_better: bool,
    pub pass_mark: f32,
}

impl Default for GradingSystem {
    fn default() -> Self {
        Self {
            credit_goal: 180,
            pass_mark: 4.0,
            ignore_failed: true,
            lower_is_better: true,
        }
    }
}

impl GradingSystem {
    pub fn is_pass(&self, g: f32) -> bool {
        if self.lower_is_better {
            g <= self.pass_mark
        } else {
            g >= self.pass_mark
        }
    }

    pub fn better(&self, a: f32, b: f32) -> bool {
        if self.lower_is_better {
            a <= b
        } else {
            a >= b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grading_system_pass_and_better() {
        let mut sys = GradingSystem::default();
        // lower is better
        assert!(sys.is_pass(3.7));
        assert!(!sys.is_pass(4.5));
        assert!(sys.better(1.3, 2.0));
        assert!(!sys.better(3.0, 2.0));

        // flip the polarity
        sys.lower_is_better = false;
        sys.pass_mark = 50.0;
        assert!(sys.is_pass(65.0));
        assert!(!sys.is_pass(40.0));
        assert!(sys.better(80.0, 60.0));
        assert!(!sys.better(20.0, 40.0));
    }
}
