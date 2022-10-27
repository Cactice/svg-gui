use salvage::{callback::PassDown, svg_set::SvgSet, usvg::Node};

use crate::svg_init::get_default_init_callback;

pub fn use_svg<C: FnMut(Node, PassDown)>(xml: &str, mut callback: C) -> SvgSet {
    let mut default_callback = get_default_init_callback();
    SvgSet::new(
        xml,
        PassDown {
            transform_id: 1,
            ..Default::default()
        },
        |node, passdown| {
            callback(node.clone(), passdown);
            default_callback(node.clone(), passdown)
        },
    )
}