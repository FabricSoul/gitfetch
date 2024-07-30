use chrono::NaiveDate;

pub struct ContributionData {
    pub total: u32,
    pub longest_streak: u32,
    pub current_streak: u32,
    pub max_contributions: u32,
    pub daily_contributions: Vec<(NaiveDate, u32)>,
}

pub fn calculate_contribution_ranges(
    daily_contributions: &[(NaiveDate, u32)],
    percentiles: &[usize; 4],
) -> Vec<u32> {
    let mut contributions: Vec<u32> = daily_contributions
        .iter()
        .map(|&(_, count)| count)
        .filter(|&count| count > 0) // Exclude days with zero contributions
        .collect();

    if contributions.is_empty() {
        return vec![0, 1, 2, 3, 4]; // Default range if no contributions
    }

    contributions.sort_unstable();

    // Calculate Q1, Q3, and IQR for outlier detection
    let q1_index = contributions.len() / 4;
    let q3_index = 3 * contributions.len() / 4;
    let q1 = contributions[q1_index] as f64;
    let q3 = contributions[q3_index] as f64;
    let iqr = q3 - q1;

    // Define outlier threshold (1.5 times IQR)
    let outlier_threshold = (q3 + 1.5 * iqr).round() as u32;

    // Filter out outliers
    let filtered_contributions: Vec<u32> = contributions
        .into_iter()
        .filter(|&count| count <= outlier_threshold)
        .collect();

    if filtered_contributions.is_empty() {
        return vec![0, 1, 2, 3, 4]; // Fallback if all values are considered outliers
    }

    let max_contribution = *filtered_contributions.last().unwrap();

    // Calculate percentiles
    let mut ranges = vec![0]; // Always start with 0 for no contributions

    for &p in &percentiles[1..] {
        let index = p * filtered_contributions.len() / 100;
        let value = filtered_contributions[index.min(filtered_contributions.len() - 1)];
        if value > *ranges.last().unwrap() {
            ranges.push(value);
        }
    }

    // Add the maximum contribution (excluding outliers) as the last range
    if max_contribution > *ranges.last().unwrap() {
        ranges.push(max_contribution);
    }

    // Ensure we have 5 distinct values
    while ranges.len() < 5 {
        let last = *ranges.last().unwrap();
        let new_value = if last < max_contribution {
            last + 1
        } else {
            last
        };
        ranges.push(new_value);
    }

    ranges
}
