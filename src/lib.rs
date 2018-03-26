#[macro_use]
extern crate lazy_static;
extern crate publicsuffix;

use publicsuffix::List;

lazy_static! {
    pub static ref STATIC_SUFFIX_LIST : List = List::from_str( include_str!("../data/public_suffix_list.dat")).unwrap();
}

#[test]
fn it_works() {
    let domain = &STATIC_SUFFIX_LIST.parse_domain("www.example.com").unwrap();
    assert_eq!(domain.root(), Some("example.com"));
    assert_eq!(domain.suffix(), Some("com"));
}
