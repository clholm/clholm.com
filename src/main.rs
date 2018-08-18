#[macro_use]
extern crate stdweb;

use stdweb::web:: {
    document,
    IParentNode,
    INode,
    IElement,
    Element,
};

// only used until rust::TryInto is stabilized
use stdweb::unstable::TryInto;

// STRUCTS
// paragraph struct
struct Paragraph {
    // raw html that includes attributes of <p> tag
    // ex. 
    //  <p >
    //      <span>hello</span> <span>world</world>
    //  </p>
    pub raw_html: String,
}

impl Paragraph {
    // constructor, takes raw html that belongs inside <p> tag
    // and attributes
    pub fn new(attributes: String, html: String) -> Paragraph {
        Paragraph {
            raw_html: format!("<p {}>\n\t{}\n</p>", attributes, html),
        }
    }
}

// span struct
struct Span {
    // raw html that includes inserted span tag
    // ex. 
    // <span>Hello</span>World<span>
    pub raw_html: String,
}

impl Span {
    // constructor, takes text inside <p> tag and inserts a <span>
    // between whitespace
    // pub fn new(text: String) -> Span {
    //     for c in paragraph_text.chars() {
            
    //     }
    //     Span {
    //         raw_html: format!("<p {}>\n\t{}\n</p>", attributes, html),
    //     }
    // }
}


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
        let paragraph_text = paragraph.text_content().unwrap();
        let paragraph: Element = paragraph.try_into().unwrap();
        // let attr = get_attributes(paragraph);
        if let Some(attr) = get_attributes(paragraph) {
            let paragraph_html: Paragraph = Paragraph::new(attr, paragraph_text);
            js! {
                console.log(@{paragraph_html.raw_html});
            };
        }
        else {
            let paragraph_html: Paragraph = Paragraph::new("".to_string(), paragraph_text);
            js! {
                console.log(@{paragraph_html.raw_html});
            };
        }
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