#![feature(type_ascription)]

#[macro_use]
extern crate stdweb;
extern crate regex;

use stdweb::web:: {
    document,
    Node,
    IParentNode,
    INode,
    IElement,
    Element,
    IHtmlElement,
    HtmlElement,
};
use stdweb::unstable::TryInto; // only used until rust::TryInto is stabilized
use regex::Regex;

// flag for preprocess mode (will split text by whitespace and add spans)
// todo: turn into enum if needed
static PREPROCESS: bool = true;

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
    // and attributes (if there are any)
    pub fn new(attrs_option: Option<String>, html: String) -> Paragraph {
        if let Some(attrs) = attrs_option {
            Paragraph {
                raw_html: format!("<p {}>\n\t{}\n</p>", attrs, html),
            }
        }
        else {
            Paragraph {
                raw_html: format!("<p>\n\t{}\n</p>", html),
            }
        }
    }
}

// struct for text that has <span>s inserted
struct ProcessedText {
    // raw html that includes inserted span tag
    // ex. 
    // <span>Hello</span>World<span>
    pub raw_html: String,
}

impl ProcessedText {
    // constructor, takes text inside <p> tag and inserts a <span>
    // between whitespace
    pub fn new(html: String, obj_count: &mut u32) -> ProcessedText {
        let mut span_text: String = String::new();
        // groups by zero or more non-whitespace characters followed by
        // one or more whitespace character
        let reg = Regex::new(r"[^\s]*\s*").unwrap();
        for cap in reg.captures_iter(&html) {
            span_text.push_str(
                &format!(
                    "<span class=\"phys-obj phys-id-{}\">{}</span>", obj_count, &cap[0]
                )
            );
            *obj_count += 1;
        }
        ProcessedText {
            raw_html: span_text,
        }
    }
}

// returns <p> with <span>s inserted
fn formatted_paragraph_factory(attrs: Option<String>, 
                               html: String, 
                               obj_count: &mut u32) -> Paragraph {
    Paragraph::new(attrs, ProcessedText::new(html, obj_count).raw_html)
}

// returns attributes of a elt as a string option
fn get_attributes(elt: &Element) -> Option<String> {
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
fn perform_preprocess(obj_count: &mut u32) {
    // find all paragraph tags
    let paragraphs = document().query_selector_all("p").unwrap();
    for paragraph in &paragraphs {
        // retreive text from paragraph
        let paragraph_text = paragraph.text_content().unwrap();
        // retrieve paragraph element
        let paragraph: Element = paragraph.try_into().unwrap();
        // process text and attributes in order to replace paragraph
        // object on page
        let replacement: Paragraph = formatted_paragraph_factory(
            get_attributes(&paragraph),
            paragraph_text,
            obj_count
        );
        // create node from raw html
        let repl_node: Node = Node::from_html(&replacement.raw_html).unwrap();
        // find parent and replace this node
        let parent: Node = paragraph.parent_node().unwrap();
        parent.replace_child(&repl_node, &paragraph).unwrap();
    }
}

// generates physics "world" by creating all necessary objects
// and initializing all necessary params
fn generate_world(obj_count: u32) {
    // find height of body
    let body_finder = document().query_selector_all("body").unwrap();
    let mut body_height = 0;
    for body in body_finder { // should only be one body
        let body: HtmlElement = body.try_into().unwrap();
        body_height = body.offset_height();
    }
    js! {                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      
        console.log("body height is " + @{body_height});
    };
    // find each phys object
    for i in 0..obj_count {
        // retrieve object with query_selector_all, used query_selector_all
        // for type reasons, there will only be one node in the returned nodelist
        let obj_finder = document().query_selector_all(&format!(".phys-id-{}", i)).unwrap();
        let obj: HtmlElement = obj_finder.item(0).unwrap().try_into().unwrap();
        // get obj height, width, and position and console log it
        let bounding_rect = obj.get_bounding_client_rect();
        let bottom = bounding_rect.get_bottom();
        let left = bounding_rect.get_left();
        let obj_height = obj.offset_height();
        let obj_width = obj.offset_width();
        js! {
            console.log("obj " + @{i} + " bottom: " + @{bottom});
            console.log("obj " + @{i} + " left: " + @{left});
            console.log("obj " + @{i} + " height: " + @{obj_height});
            console.log("obj " + @{i} + " width: " + @{obj_width});
        };
    }
}

fn main() {
    // initialize object count
    let mut obj_count: u32 = 0;
    // if preprocess, split text by whitespace (or anchor tag) and add
    // spans
    if PREPROCESS {
        perform_preprocess(&mut obj_count);
    }
    // generate physics world, find height of each text object
    generate_world(obj_count);
}