pub fn read_u16(data: &[u8]) -> Option<(u16, &[u8])> {
    let word = u16::from_be_bytes([data.get(0).map(|b| *b)?, data.get(1).map(|b| *b)?]);

    //This is safe as get_unchecked only invokes undefined behavior if get would return none (idx > len)
    Some((word, unsafe { data.get_unchecked(2..) }))
}

pub unsafe fn read_u16_unchecked(data: &[u8]) -> (u16, &[u8]) {
    let word = u16::from_be_bytes([*data.get_unchecked(0), *data.get_unchecked(1)]);

    //This is safe as get_unchecked only invokes undefined behavior if get would return none (idx > len)
    (word, data.get_unchecked(2..))
}

#[cfg(test)]
mod read_u16_test {
    use super::read_u16;

    #[test]
    fn read8() {
        let (w8, tail) = read_u16(&[0, 8]).unwrap();
        assert_eq!(w8, 8);
        assert_eq!(tail, []);
    }

    #[test]
    fn read0() {
        let (w8, tail) = read_u16(&[0, 0, 100]).unwrap();
        assert_eq!(w8, 0);
        assert_eq!(tail, &[100]);
    }

    #[test]
    fn read256() {
        let (w8, tail) = read_u16(&[1, 0, 100, 200, 2, 3, 4, 5]).unwrap();
        assert_eq!(w8, 256);
        assert_eq!(tail, &[100, 200, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic(expected="called `Option::unwrap()` on a `None` value")]
    fn read_error1() {
        let (_w8, _tail) = read_u16(&[1]).unwrap();
    }

    #[test]
    #[should_panic(expected="called `Option::unwrap()` on a `None` value")]
    fn read_error0() {
        let (_w8, _tail) = read_u16(&[]).unwrap();
    }
}
