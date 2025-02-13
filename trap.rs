use std::cmp::{max, min};
struct Trap;

impl Trap {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut trapt = vec![0; n];
        // Calculate the maximum water trapable from the left side and then the right side
        trapt[0] = height[0];
        for i in 1..n {
            trapt[i] = max(trapt[i-1], height[i]);
        }
        trapt[n-1] = height[n-1];
        for i in (0..n-1).rev() {
            trapt[i] = min(trapt[i], max(height[i], trapt[i+1]));
        }

        let mut water_trapt = 0;
        for i in 0..n {
            water_trapt += trapt[i] - height[i];
        }
        water_trapt
    }
}