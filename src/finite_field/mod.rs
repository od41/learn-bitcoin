pub struct FieldElement {
    num: u64, 
    prime: u64
}

impl FieldElement {
    pub fn new(num: u64, prime: u64) -> FieldElement {
        if num >= prime {
            panic!("Num not in field range 0 to {}", prime);
        }
        FieldElement {num, prime}
    }

    pub fn to_string(self) -> String {
        format!("FieldElement_{}_{}", self.num, self.prime)
    }

    pub fn is_equal(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}