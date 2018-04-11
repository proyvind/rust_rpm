#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/*
 * generate bogus 'pub type poptOption = *mut poptOption;',
 * breaking build

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

 * let's include a manually modified copy with the bogus line
 * removed
*/
include!("bindings.rs");

#[link(name = "rpm")]
#[link(name = "rpmbuild")]
#[link(name = "rpmio")]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


