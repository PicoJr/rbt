#[cfg(test)]
mod tests {
    use crate::pattern::Pattern;

    #[test]
    fn pattern_applied() {
        let expected = 0x42;
        let unexpected = 0x41;
        let value = vec![expected];
        let pattern_0 = Pattern::new(value.clone(), vec![0xFF], 4, 0);
        assert_eq!(pattern_0.compute_pattern(0, unexpected), expected);
        let pattern_1 = Pattern::new(value.clone(), vec![0xFF], 4, 0);
        assert_eq!(pattern_1.compute_pattern(4, unexpected), expected);
        let pattern_2 = Pattern::new(value.clone(), vec![0xFF], 4, 2);
        assert_eq!(pattern_2.compute_pattern(6, unexpected), expected);
        assert_eq!(pattern_2.compute_pattern(10, unexpected), expected);
    }

    #[test]
    fn pattern_mask() {
        // 0_0_1_1_
        let pattern_value = 0b0001_1011;
        // _1_0_1_0
        let previous_value = 0b1110_0100;
        // 10101010 (mask) 0xAA
        // 01001110
        let value = vec![pattern_value];
        let pattern_0 = Pattern::new(value, vec![0xAA], 4, 0);
        // 0xAA 1/2 byte from each
        assert_eq!(pattern_0.compute_pattern(0, previous_value), 0b01001110);
    }

    #[test]
    fn pattern_not_applied() {
        let unexpected = 0x42;
        let expected = 0x41;
        let value = vec![unexpected];
        let pattern_0 = Pattern::new(value.clone(), vec![0x00], 4, 0);
        assert_eq!(pattern_0.compute_pattern(0, expected), expected);
        let pattern_1 = Pattern::new(value.clone(), vec![0x00], 4, 0);
        assert_eq!(pattern_1.compute_pattern(4, expected), expected);
        let pattern_2 = Pattern::new(value.clone(), vec![0x00], 4, 2);
        assert_eq!(pattern_2.compute_pattern(6, expected), expected);
        assert_eq!(pattern_2.compute_pattern(10, expected), expected);
    }
}

pub mod pattern {
    pub struct Pattern {
        value: Vec<u8>,
        mask: Vec<u8>,
        periodicity: usize,
        offset: usize,
    }

    fn remove_leading_zeros(v: &[u8]) -> Vec<u8> {
        let mut non_zero_found = false;
        let mut rv: Vec<u8> = vec![];
        for &e in v {
            if e != 0 {
                non_zero_found = true;
                rv.push(e);
            } else if e == 0 && non_zero_found {
                rv.push(e);
            }
        }
        rv
    }

    impl Pattern {
        pub fn new(value: Vec<u8>, mask: Vec<u8>, periodicity: usize, offset: usize) -> Self {
            Pattern {
                value,
                mask,
                periodicity,
                offset,
            }
        }
        pub fn compute_pattern(&self, position: usize, value: u8) -> u8 {
            if (position as i64 - self.offset as i64) >= 0 {
                // previous test guarantees that next statement will evaluate to >= 0
                let position_in_pattern = (position - self.offset) % self.periodicity;
                if position_in_pattern < self.value.len() {
                    (self.mask[position_in_pattern] & self.value[position_in_pattern])
                        | (!self.mask[position_in_pattern] & value)
                } else {
                    value
                }
            } else {
                value
            }
        }

        pub fn from_string(pattern_str: &str) -> Result<Pattern, &'static str> {
            let mut pattern_init = pattern_str.split(',');
            let int_value: usize;
            match pattern_init.next().unwrap().parse::<usize>() {
                Err(_) => return Err("error while parsing pattern"),
                Ok(value) => int_value = value,
            }
            let value: [u8; 8] = int_value.to_be_bytes();
            let value = remove_leading_zeros(&value.to_vec());
            let int_mask: usize;
            match pattern_init.next().unwrap().parse::<usize>() {
                Err(_) => return Err("error while parsing pattern"),
                Ok(value) => int_mask = value,
            }
            let mask: [u8; 8] = int_mask.to_be_bytes();
            let mask = remove_leading_zeros(&mask.to_vec());
            let periodicity: usize;
            match pattern_init.next().unwrap().parse::<usize>() {
                Err(_) => return Err("error while parsing pattern"),
                Ok(value) => periodicity = value,
            }
            let offset: usize;
            match pattern_init.next().unwrap().parse::<usize>() {
                Err(_) => return Err("error while parsing pattern"),
                Ok(value) => offset = value,
            }
            Ok(Pattern::new(value, mask, periodicity, offset))
        }
    }
}
