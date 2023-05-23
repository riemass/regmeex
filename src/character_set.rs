use std::fmt;

pub struct CharacterSet {
    set: [u64; 4],
}

impl CharacterSet {

    pub fn new() -> Self {
        Self { set: [0,0,0,0] }
    }

    pub fn set(&mut self, c: u8) {
        let big_mask = 0b11000000;        
        let small_mask = 0b00111111;        

        let index = (c & big_mask) >> 6;
        let shift = c & small_mask;

        self.set[index as usize] |= 1 << shift
    }

    pub fn get(&self, c: u8) -> bool {
        let big_mask = 0b11000000;        
        let small_mask = 0b00111111;        

        let index = (c & big_mask) >> 6;
        let shift = c & small_mask;

        (self.set[index as usize] & 1 << shift)  != 0
    }

    pub fn chars(&self) -> Vec<u8> {
        let mut chs = Vec::new();
        for i in 1..255 {
            if self.get(i as u8) {
                chs.push(i as u8);
            }
        }
        chs
    }

}

impl fmt::Debug for CharacterSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharacterSet")
         .field("chars", &self.chars())
         .finish()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let cs = CharacterSet::new();
        for val in 0..255 {
            assert!(!cs.get(val as u8));
        }

    }

    #[test]
    fn test_2() {
        let mut cs = CharacterSet::new();
        cs.set('x' as u8);
        assert!(cs.get('x' as u8));
    }

    #[test]
    fn test_3() {
        let mut cs = CharacterSet::new();
        for val in 0..255 {
            assert!(!cs.get(val as u8));
            cs.set(val as u8);
            assert!(cs.get(val as u8));
        }
    }
}
