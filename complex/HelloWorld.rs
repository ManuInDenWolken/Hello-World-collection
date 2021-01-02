#![feature(fmt_internals)]
#![feature(print_internals)]

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(u8)]
enum Lower {
    e = 0x65,
    l = 0x6c,
    o = 0x6f,
    r = 0x72,
    d = 0x64,
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum Upper {
    H = 0x48,
    W = 0x57,
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum Punct {
    Space = 0x20,
    Bang = 0x21,
}

trait Char {
    fn as_num(&self) -> u8;
}

impl Char for Upper {
    fn as_num(&self) -> u8 {
        *self as u8
    }
}

impl Char for Lower {
    fn as_num(&self) -> u8 {
        *self as u8
    }
}

impl Char for Punct {
    fn as_num(&self) -> u8 {
        *self as u8
    }
}

fn main() {
    let b: Vec<&dyn Char> = vec![
        &Upper::H,
        &Lower::e,
        &Lower::l,
        &Lower::l,
        &Lower::o,
        &Punct::Space,
        &Upper::W,
        &Lower::o,
        &Lower::r,
        &Lower::l,
        &Lower::d,
        &Punct::Bang,
    ];
    let utf8 = b.iter().map(|x| x.as_num()).collect::<Vec<_>>();
    let string = ::std::str::from_utf8(&utf8).unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &[""],
            #[allow(clippy::match_single_binding)]
            &match (&string,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
}
