#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Preferred,
    Fixed(u16),
    Part(u16),
    Shrink,
    Fill,
}

impl Length {
    /// Resolves the a list of lengths into a list of actual sizes.
    /// The resolvation works as follows:
    ///
    /// First, it allocates spaces for fixed lengths: [`Length::Preferred`] and [`Length::Fixed`].
    /// Second, if there exists [`Length::Fill`], it divides the remaining space equally among them.
    /// Third, if there exists [`Length::Part`], it divides the remaining space proportionally among them.
    /// Finally, it divides the remaining space equally among [`Length::Shrink`].
    ///
    /// # Arguments
    ///
    /// * `total` - The total size to resolve.
    /// * `lengths` - The list of lengths to resolve.
    /// * `preferences` - The list of preferred sizes. The [`Length::Preferred`] would be replaced by the corresponding value in this list.
    ///
    /// # Example
    ///
    /// ```
    /// use voidtui::geometry::length::Length;
    /// let result = Length::resolve(
    ///     100,
    ///     vec![Length::Preferred, Length::Part(1), Length::Fill, Length::Fill],
    ///     vec![20, 10, 30, 10]
    /// );
    /// // First, 20 is allocated for `Length::Preferred` through `preferences`.
    /// // Second, the all remaining spaces are equally allocated for `Length::Fill`.
    /// // So finally, there is no space for `Length::Part`.
    /// assert_eq!(result, vec![20, 0, 40, 40]);
    /// ```
    pub fn resolve(total: u16, lengths: Vec<Length>, preferences: Vec<u16>) -> Vec<u16> {
        let mut result = Vec::with_capacity(lengths.len());
        let mut remaining = total;
        let mut fill_count = 0;
        let mut part_sum = 0;
        let mut shrink_count = 0;

        // First pass: handle Fixed and count Preferred/Fill/Part/Shrink
        for (length, preference) in lengths.iter().zip(preferences.into_iter()) {
            match length {
                Length::Preferred => {
                    result.push(preference);
                    remaining -= preference;
                }
                Length::Fixed(size) => {
                    if *size <= remaining {
                        remaining -= size;
                        result.push(*size);
                    } else {
                        result.push(remaining);
                        remaining = 0;
                    }
                }
                Length::Fill => {
                    fill_count += 1;
                    result.push(0);
                }
                Length::Part(weight) => {
                    part_sum += weight;
                    result.push(0);
                }
                Length::Shrink => {
                    shrink_count += 1;
                    result.push(0);
                }
            }
        }

        // Second pass: handle Fill
        let mut last = None;
        if fill_count > 0 {
            let fill_length = remaining / fill_count;
            for (i, length) in lengths.iter().enumerate() {
                if matches!(length, Length::Fill) {
                    last = Some(i);
                    result[i] = fill_length;
                    remaining -= fill_length;
                }
            }
        }
        // Due to the integer division, there might be some remaining space.
        // So let the last Length::Fill to fill it.
        if let Some(last) = last {
            result[last] += remaining;
            remaining = 0;
        }

        // Third pass: handle Part
        if part_sum > 0 {
            for (i, length) in lengths.iter().enumerate() {
                if let Length::Part(weight) = length {
                    result[i] = (remaining * weight) / part_sum;
                }
            }
        }

        // Final pass: handle Shrink
        if shrink_count > 0 {
            for (i, length) in lengths.iter().enumerate() {
                if matches!(length, Length::Shrink) {
                    result[i] = remaining / shrink_count;
                }
            }
        }

        result
    }
}

impl From<u16> for Length {
    fn from(value: u16) -> Self {
        Length::Fixed(value)
    }
}
