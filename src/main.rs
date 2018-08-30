#![feature(type_ascription)]

#[macro_use]
extern crate stdweb;
extern crate regex;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;

// use statements for nphysics
use na::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{BodyHandle, ColliderHandle, Material};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World;
// use statements for stdweb
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
            // if captured text isn't just whitespace
            // TODO: report error if website all whitespace
            if &cap[0].split_whitespace().count() > &0 {
                span_text.push_str(
                    &format!(
                        "<span class=\"phys-obj phys-id-{}\">{}</span>", obj_count, &cap[0]
                    )
                );
                *obj_count += 1;
            }
        }
        ProcessedText {
            raw_html: span_text,
        }
    }
}

// struct that holds all information for an individual text object
// in the physics world (rigid body handle, collider handle)
struct TextNode {
    pub body_handle: BodyHandle,
    pub collider_handle: ColliderHandle,
}

impl TextNode {
    // constructor
    pub fn new(in_body_handle: BodyHandle, in_collider_handle: ColliderHandle) -> TextNode {
        TextNode {
            body_handle: in_body_handle,
            collider_handle: in_collider_handle,
        }
    }
}

// struct Realm contains the World that drives the physics engine and the vector
// of TextNodes that populates that World
struct Realm {
    pub world: World<f64>,
    pub text_nodes: Vec<TextNode>,
    pub ground: ColliderHandle,
}

impl Realm {
    // constructs Realm by creating World struct and populating text_ndoes vec
    pub fn new(obj_count: u32) -> Realm {
        // margin that the collision engine will use
        const COLLIDER_MARGIN: f64 = 0.01;
        // create nphysics world
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, -9.81));
        // find height and width of body for the ground of the world
        let body_finder = document().query_selector_all("body").unwrap();
        let mut body_height = 0;
        let mut body_width = 0;
        for body in body_finder { // should only be one body
            let body: HtmlElement = body.try_into().unwrap();
            body_height = body.offset_height();
            body_width = body.offset_width();
        }
        // create ground object in physics world
        // create ground shape handle
        let ground_half_height: f64 = 0.5;
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            (body_width as f64) / 2.0 - COLLIDER_MARGIN,
            ground_half_height - COLLIDER_MARGIN,
        )));
        // ground is located at the bottom of the page, where 0 is top of page
        // and -body height is the bottom of the page (thus page is located entirely in
        // 4th quadrant)
        let ground_y_pos = -(body_height as f64) - ground_half_height * 2.0;
        let ground_pos = Isometry2::new(Vector2::y() * ground_y_pos, na::zero());
        // add ground collider to world 
        let ground_handle = world.add_collider(
            COLLIDER_MARGIN,
            ground_shape,
            BodyHandle::ground(),
            ground_pos,
            Material::default(),
        );
        // create vector to hold all TextNodes
        let mut text_node_vec: Vec<TextNode> = Vec::with_capacity(obj_count as usize);
        // find each span with text and create a TextNode from it
        for i in 0..obj_count {
            // retrieve object with query_selector_all, used query_selector_all
            // for type reasons, there will only be one node in the returned nodelist
            let obj_finder = document().query_selector_all(&format!(".phys-id-{}", i)).unwrap();
            let obj: HtmlElement = obj_finder.item(0).unwrap().try_into().unwrap();
            // retrieve object attributes
            let bounding_rect = obj.get_bounding_client_rect();
            let bottom = bounding_rect.get_bottom();
            let left = bounding_rect.get_left();
            let obj_height = obj.offset_height();
            let obj_width = obj.offset_width();
            // calculate object's half extents and position
            let obj_half_height = obj_height as f64 / 2.0;
            let obj_half_width = obj_width as f64 / 2.0;
            // object's y position is -bottom + obj_half_height
            let y_pos = -bottom as f64 + obj_half_height;
            // object's x position is left + obj_half_width
            let x_pos = left + obj_half_width;
            // create shape of object from half heights and widths and retrieve
            // properties from shape handle
            let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
                obj_half_width - COLLIDER_MARGIN,
                obj_half_height - COLLIDER_MARGIN,
            )));
            let inertia = shape.inertia(1.0);
            let center_of_mass = shape.center_of_mass();
            let pos = Isometry2::new(Vector2::new(x_pos, y_pos), 0.0);
            // add rigid body to world
            let body_handle = world.add_rigid_body(pos, inertia, center_of_mass);
            // add collider to world and attach to above rigid body
            let collider_handle = world.add_collider(
                COLLIDER_MARGIN,
                shape.clone(),
                body_handle,
                Isometry2::identity(),
                Material::default(),
            );
            // create a text node from this object and push it into the text_node_vec
            text_node_vec.push(TextNode::new(body_handle, collider_handle));
        }
        // build and return the Realm!
        Realm {
            world: world,
            text_nodes: text_node_vec,
            ground: ground_handle,
        }
    }
}

// returns <p> with <span>s inserted
fn formatted_paragraph_factory(
    attrs: Option<String>,
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

fn main() {
    // initialize object count
    let mut obj_count: u32 = 0;
    // if preprocess, split text by whitespace (or anchor tag) and add
    // spans
    if PREPROCESS {
        perform_preprocess(&mut obj_count);
    }
    // create Realm
    let realm = Realm::new(obj_count);
}