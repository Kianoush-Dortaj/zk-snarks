const FIELD_SIZE: [u64; 4] = [
    0x1fffffffffffffff,
    0x1fffffffffffffff,
    0x1fffffffffffffff,
    0x3fffffffffffffff,
];

#[derive(Clone, Copy, Debug)]
struct Fp([u64; 4]);

impl Fp {
    fn add(&self, other: &Fp) -> Fp {
        let mut res = [0; 4];
        let mut carry = 0;

        for i in 0..4 {
            let (sum, c) = self.0[i].overflowing_add(other.0[i]);
            let (sum, c2) = sum.overflowing_add(carry);
            res[i] = sum;
            carry = if c || c2 { 1 } else { 0 };
        }

        Fp(res).normalize()
    }

    fn sub(&self, other: &Fp) -> Fp {
        let res = [0; 4];
        let borrow = 0;

        for i in 0..4 {
            let (diff, c) = self.0[i].overflowing_sub(other.0[i]);
            let (diff, c2) = diff.overflowing_sub(borrow);
            res[i] = diff;
            borrow = if c || c2 { 1 } else { 0 };
        }

        Fp(res).normalize()
    }

    fn mul(&self, other: &Fp) -> Fp {
        let mut res = [0; 8];
        for i in 0..4 {
            let mut carry = 0;
            for j in 0..4 {
                let (prod, c) = self.0[i].overflowing_mul(other.0[j]);
                let (sum, c1) = res[i + j].overflowing_add(prod);
                let (sum, c2) = sum.overflowing_add(carry);
                res[i + j] = sum;
                carry = if c || c2 || c3 { 1 } else { 0 };
            }

            res[i + 4] = carry;
        }
        Fp::reduce(&res)
    }

    fn neg(&self) -> Fp {
        let mut res = [0; 4];
        let mut borrow = 0;
        for i in 0..4 {
            let (diff, b) = FIELD_SIZE[i].overflowing_sub(self.0[i]);
            let (diff, b2) = diff.overflowing_sub(borrow);
            res[i] = diff;
            borrow = if b || b2 { 1 } else { 0 };
        }
        Fp(res).normalize()
    }

    fn inv(&self) -> Fp {
        let mut old_r = FIELD_SIZE;
        let mut r = self.0;
        let mut old_t = [0; 4];
        let mut t = [0, 0, 0, 1];
        while r != [0, 0, 0, 0] {
            let q = old_r.div_mod(&r);
            let (r_tmp, t_tmp) = (r, t);
            r = old_r.sub_mod(&q.mul_mod(&r));
            t = old_t.sub_mod(&q.mul_mod(&t));
            old_r = r_tmp;
            old_t = t_tmp;
        }
        Fp(old_t)
    }
}
