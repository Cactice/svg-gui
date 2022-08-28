use guppies::glam::Mat4;
use regex::RegexSet;
use salvage::{
    callback::{IndicesPriority, InitCallback, PassDown},
    geometry::Geometry,
    usvg::{self, Node, NodeExt, NodeKind},
};
#[derive(Clone, Debug, Default)]
pub struct RegexPattern {
    pub regex_pattern: String,
    pub index: usize,
}
#[derive(Clone, Debug)]
pub struct RegexPatterns {
    pub inner: Vec<RegexPattern>,
}

impl Default for RegexPatterns {
    fn default() -> Self {
        let mut regex_patterns = Self { inner: vec![] };
        regex_patterns
    }
}
impl RegexPatterns {
    pub fn add(&mut self, regex_pattern: &str) -> RegexPattern {
        let regex_pattern = RegexPattern {
            regex_pattern: regex_pattern.to_string(),
            index: self.inner.len(),
        };
        self.inner.push(regex_pattern.clone());
        regex_pattern
    }
}

pub fn default_directives() -> InitCallback<'static> {
    let mut transform_count = 1;
    let mut regex_patterns = RegexPatterns::default();
    let _clickable_regex_pattern = regex_patterns.add(r"#clickable(?:$| |#)");
    let dynamic_regex_pattern = regex_patterns.add(r"#dynamic(?:$| |#)");
    let dynamic_text_regex_pattern = regex_patterns.add(r"#dynamicText(?:$| |#)");
    let defaults = RegexSet::new(regex_patterns.inner.iter().map(|r| &r.regex_pattern)).unwrap();
    let callback = InitCallback::new(move |(node, pass_down)| {
        let PassDown {
            transform_id: parent_transform_id,
            indices_priority: parent_priority,
        } = pass_down;
        let node_ref = node.borrow();
        let id = NodeKind::id(&node_ref);
        let default_matches = defaults.matches(&id);
        let transform_id = if default_matches.matched(dynamic_regex_pattern.index) {
            transform_count += 1;
            transform_count
        } else {
            *parent_transform_id
        };
        let indices_priority = if !default_matches.matched(dynamic_text_regex_pattern.index) {
            IndicesPriority::Variable
        } else {
            IndicesPriority::Fixed
        };
        let indices_priority = *parent_priority.max(&indices_priority);
        let geometry = {
            if let usvg::NodeKind::Path(ref p) = *node.borrow() {
                Some(Geometry::new(p, transform_id, indices_priority))
            } else {
                None
            }
        };
        (
            geometry,
            PassDown {
                indices_priority,
                transform_id,
            },
        )
    });
    callback
}

pub fn get_center(node: &Node) -> Mat4 {
    let bbox = node.calculate_bbox().unwrap();
    let center = Mat4::from_translation(
        [
            (bbox.x() + bbox.width() / 2.) as f32,
            (bbox.y() + bbox.height() / 2.) as f32,
            0.,
        ]
        .into(),
    );
    center
}