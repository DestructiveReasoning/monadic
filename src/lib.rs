//Wrapper trait to simulate higher-kinded types
pub trait TypeClass<U> {
    type C;
    type T;
}

#[macro_export]
macro_rules! derive_typeclass {
    ($t: ident) => {
        impl<T,U> TypeClass<U> for $t<T> {
            type C = T;
            type T = $t<U>;
        }
    }
}

pub trait Functor<U>: TypeClass<U> {
    fn fmap<F>(&self, f: F) -> Self::T where F: Fn(&Self::C) -> U;
}

pub trait Monad<U>: Functor<U> {
    fn ret(U) -> Self::T;
    fn bind<F>(&self, f: F) -> Self::T where F: FnMut(&Self::C) -> Self::T;
}

#[macro_export]
macro_rules! monadic {
    ($l:expr => $r:expr;) => {
        $l.bind($r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    derive_typeclass!(Vec);
    derive_typeclass!(Option);

    impl<U, T> Functor<U> for Vec<T> {
        fn fmap<F>(&self, f: F) -> Vec<U> where F: Fn(&T) -> U {
            let mut newvec = Vec::new();
            for c in self {
                newvec.push(f(c));
            }
            newvec
        }
    }

    impl<U,T> Functor<U> for Option<T> {
        fn fmap<F>(&self, f: F) -> Option<U> where F: Fn(&T) -> U {
            match *self {
                None => None,
                Some(ref x) => Some(f(&x)),
            }
        }
    }

    impl<U,T> Monad<U> for Option<T> {
        fn ret(t: U) -> Option<U> {
            Some(t)
        }
        fn bind<F>(&self, mut f: F) -> Option<U> where F: FnMut(&T) -> Option<U> {
            match *self {
                None => None,
                Some(ref x) => f(&x),
            }
        }
    }

    #[test]
    fn test_functor() {
        let v = vec![1,2,3,4,5];
        let newv = v.fmap(|x| 2*x);
        assert_eq!(newv, [2,4,6,8,10]);

        let o: Option<u16> = Some(3);
        let bado: Option<u16> = None;
        let newo = o.fmap(|x| 2 * x);
        let newbado = bado.fmap(|x| 2 * x);
        assert_eq!(newo, Some(6));
        assert_eq!(newbado, None);
    }

    #[test]
    fn test_monad() {
        let o1: Option<u16> = Some(4);
        let newo1 = o1.bind(|x| if x % 2 == 0 { Some(x/2) } else { None });
        let newnewo1 = monadic!{o1 => |x| if x % 2 == 0 { Some(x/2) } else { None };};
        assert_eq!(newo1, Some(2));
        assert_eq!(newnewo1, Some(2));
    }
}
