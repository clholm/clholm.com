#[macro_use]
extern crate stdweb;

use stdweb::web:: {
    document,
    IParentNode,
    IElement,
    Element,
};

// only used until rust::TryInto is stabilized
use stdweb::unstable::TryInto;

// structure

// flag for preprocess mode (will split text by whitespace and add spans)
// todo: turn into enum if needed
static PREPROCESS: bool = true;

// returns attributes of a elt as a string option
fn get_attributes(elt: Element) -> Option<String> {
    // retrieve attribute names and find their values
    let attr_names = elt.get_attribute_names();
    let mut ret = String::new();
    for attr in attr_names {
        // if the attr_val exists, add to ret string
        if let Some(attr_val) = elt.get_attribute(&attr) {
            ret.push_str(&format!("{}=\"{}\" ", attr, attr_val));
        }
    }
    // return Some(ret) if ret exists
    if ret.len() != 0 {
        Some(ret)
    }
    else {
        None
    }
}

// splits text by whitespace (or anchor tags) and adds spans
fn perform_preprocess() {
    // find all paragraph tags
    let paragraphs = document().query_selector_all("p").unwrap();
    for paragraph in &paragraphs {
        // retreive text from paragraph
        // let paragraph_text = paragraph.text_content().unwrap();
        // for c in paragraph_text.chars() {

        // }
        // let paragraph: IElement = paragraph.try_from();
        let paragraph: Element = paragraph.try_into().unwrap();
        let attr = get_attributes(paragraph);
        js! {
            console.log(@{attr});
        };
    }
}

fn main() {
    // retrieve body node
    if PREPROCESS {
        perform_preprocess();
    }
    // if preprocess, split text by whitespace (or anchor tag) and add
    // spans

    // js! {
    //     @{body}.style = "background-color: pink;" 
    // };
}