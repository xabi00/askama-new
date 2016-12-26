#![feature(proc_macro)]

#[macro_use]
extern crate askama_codegen;
extern crate askama;

use askama::Template;

#[derive(Template)]
#[template(path = "simple.html")]
struct TestTemplate {
    strvar: String,
    num: i64,
    i18n: String,
}

#[test]
fn it_works() {
    let s = TestTemplate {
        strvar: "foo".to_string(),
        num: 42,
        i18n: "Iñtërnâtiônàlizætiøn".to_string(),
    };
    assert_eq!(s.render(), "hello world, foo\n\
                            with number: 42\n\
                            Iñtërnâtiônàlizætiøn is important\n\
                            in vars too: Iñtërnâtiônàlizætiøn\n");
}
