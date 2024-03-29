
#[derive(Debug, Default, Clone)]
pub struct SizeHint {
    lower: u64,
    upper: Option<u64>,
}

impl SizeHint {
    #[inline]
    pub fn new() -> SizeHint {
        SizeHint::default()
    }

    #[inline]
    pub fn with_exact(value: u64) -> SizeHint {
        SizeHint {
            lower: value,
            upper: Some(value),
        }
    }

    #[inline]
    pub fn lower(&self) -> u64 {
        self.lower
    }

    #[inline]
    pub fn set_lower(&mut self, value: u64) {
        assert!(
            value <= self.upper.unwrap_or(u64::MAX),
            "`value` is greater than `upper`"
        );
        self.lower = value;
    }

    #[inline]
    pub fn upper(&self) -> Option<u64> {
        self.upper
    }

    #[inline]
    pub fn set_upper(&mut self, value: u64) {
        assert!(value >= self.lower, "`value` is less than `lower`");
        self.upper = Some(value);
    }

    #[inline]
    pub fn exact(&self) -> Option<u64> {
        if Some(self.lower) == self.upper {
            self.upper
        } else {
            None
        }
    }

    #[inline]
    pub fn set_exact(&mut self, value: u64) {
        self.lower = value;
        self.upper = Some(value);
    }
}
