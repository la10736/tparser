use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

pub struct Sp<A, B>(A, B);

impl<A, B> FromStr for Sp<A, B>
    where
        A: FromStr,
        B: FromStr,
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.splitn(2, ' ');
        Ok(Self(
            components.next()
                .ok_or("errore")
                .and_then(|c| c.parse().map_err(|_| "errore"))?,
            components.next()
                .ok_or("errore")
                .and_then(|c| c.parse().map_err(|_| "errore"))?
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn third() {
        let Sp(a, Sp(b, c)) = "1 2.4 c".parse().unwrap();

        assert_eq!(1i64, a);
        assert_eq!(2.4f32, b);
        assert_eq!('c', c);
    }

    #[test]
    fn forth() {
        let Sp(a, Sp(b, Sp(c, d))) = "1 2.4 c 1".parse().unwrap();

        assert_eq!(1i64, a);
        assert_eq!(2.4f32, b);
        assert_eq!('c', c);
        assert_eq!(1i32, d);
    }

    macro_rules! sp {
        ($x:ident) => { $x };
        ($x:ident, $( $y:ident ),* ) => { Sp($x, sp!($ ($y),*) ) };
    }

    #[test]
    fn macro_1() {
        let sp!(a, b) = "1 2".parse().unwrap();

        assert_eq!(1u32, a);
        assert_eq!(2u64, b);
    }
    #[test]
    fn macro_2() {
        let sp!(a, b, c) = "1 2 4".parse().unwrap();

        assert_eq!(1u32, a);
        assert_eq!(2u64, b);
        assert_eq!(4u8, c);
    }

    #[test]
    fn macro_9() {
        let sp!(a) = "1".parse().unwrap();

        assert_eq!(1u32, a);
    }

    #[test]
    fn first() {
        let Sp(a, b) = "1 2.4".parse().unwrap();

        assert_eq!(1u32, a);
        assert_eq!(2.4f64, b);
    }

    #[test]
    fn second() {
        let Sp(a, b) = "1 2.4".parse().unwrap();

        assert_eq!(1i64, a);
        assert_eq!(2.4f32, b);
    }
}