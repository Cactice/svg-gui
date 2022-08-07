use crate::{
    callback::{IndicesPriority, InitCallback},
    fill::iterate_fill,
    stroke::iterate_stroke,
};
use guppies::{
    glam::{Vec2, Vec4},
    primitives::{Index, Indices, Rect, Vertex, Vertices},
};
use lyon::lyon_tessellation::VertexBuffers;
use roxmltree::{Document, NodeId};
use std::{collections::HashMap, iter, ops::Range, sync::Arc};
use usvg::{fontdb::Source, NodeKind, Options, Path, PathBbox, Tree};
pub const FALLBACK_COLOR: Vec4 = Vec4::ONE;

pub fn prepare_vertex_buffer(p: &Path, transform_id: u32) -> VertexBuffers<Vertex, Index> {
    let mut vertex_buffer = VertexBuffers::<Vertex, Index>::new();
    if let Some(ref stroke) = p.stroke {
        let color = match stroke.paint {
            usvg::Paint::Color(c) => Vec4::new(
                c.red as f32 / u8::MAX as f32,
                c.green as f32 / u8::MAX as f32,
                c.blue as f32 / u8::MAX as f32,
                stroke.opacity.value() as f32,
            ),
            _ => FALLBACK_COLOR,
        };
        iterate_stroke(stroke, p, &mut vertex_buffer, color, transform_id);
    }
    if let Some(ref fill) = p.fill {
        let color = match fill.paint {
            usvg::Paint::Color(c) => Vec4::new(
                c.red as f32 / u8::MAX as f32,
                c.green as f32 / u8::MAX as f32,
                c.blue as f32 / u8::MAX as f32,
                fill.opacity.value() as f32,
            ),
            _ => FALLBACK_COLOR,
        };

        iterate_fill(p, &color, &mut vertex_buffer, transform_id);
    };
    vertex_buffer
}