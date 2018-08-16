#[macro_use]
extern crate stdweb;

use stdweb::web:: {
    document,
    IParentNode,
    Element,
};

// flag for preprocess mode (will split text by whitespace and add spans)
// todo: turn into enum if needed
PREPROCESS = true;

fn main() {
    // retrieve body node
    let body : Element = document().query_selector("body").unwrap().unwrap();
    // if preprocess, split text by whitespace (or anchor tag) and add
    // spans
    
    js! {
        @{body}.style = "background-color: pink;" 
    };
}