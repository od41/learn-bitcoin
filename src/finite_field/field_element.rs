pub struct FieldElement {
    num: u128, 
    prime: u128
}

impl FieldElement {
    pub fn new(num: u128, prime: u128) -> FieldElement {
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

    pub fn is_not_equal(&self, other: &FieldElement) -> bool {
        self.num != other.num || self.prime != other.prime
    }

    pub fn add(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't add numbers in different fields");
        }

        FieldElement {
            num: (self.num + other.num) % self.prime,
            prime: self.prime
        }
    }

    pub fn sub(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't subtract numbers in different fields");
        }

        let num;
        if self.num < other.num {
            let temp_num = (other.num - self.num) % self.prime;
            num = self.prime - temp_num;
        } else {
            num = (self.num - other.num) % self.prime;
        }

        FieldElement {
            num,
            prime: self.prime
        }
    }

    pub fn mul(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't multiply numbers in different fields");
        }

        FieldElement {
            num: (self.num * other.num) % self.prime,
            prime: self.prime
        }
    }

    pub fn exp(&self, power: u32) -> FieldElement {
        let num = self.num.pow(power) % self.prime;
        FieldElement {
            num,
            prime: self.prime
        }
    }

    pub fn div(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't divide numbers in different fields");
        }

        let exp = other.exp((30) as u32);
        println!("exp in div {}", exp.num);

        // self.mul(&exp) 
        exp
    }
}