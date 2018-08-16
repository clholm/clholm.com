#[macro_use]
extern crate stdweb;

use stdweb::web:: {
    document,
    IParentNode,
    Element,
};

// flag for preprocess mode (will split text by whitespace and add spans)
// todo: turn into enum if needed

fn main() {
    // retrieve body node
    let body : Element = document().query_selector("body").unwrap().unwrap();
    js! {
        @{body}.style = "background-color: pink;" 
    };
}