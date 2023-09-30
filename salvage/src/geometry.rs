use crate::prepare_triangles_from_path::prepare_triangles_from_path;
use guppies::primitives::{Indices, Triangles};
use usvg::{Path, Tree};

#[derive(Clone, Debug, Default)]
pub struct Geometry {
    pub triangles: Triangles,
    pub id: String,
}
impl From<Tree> for Geometry {
    fn from(tree: Tree) -> Self {
        let geometry = tree
            .root()
            .descendants()
            .into_iter()
            .filter_map(|node| {
                if let usvg::NodeKind::Path(ref p) = *node.borrow() {
                    Some(Geometry::new(p, 1))
                } else {
                    None
                }
            })
            .fold(Geometry::default(), |mut acc, curr| {
                acc.extend(&curr);
                acc
            });
        geometry
    }
}
impl Geometry {
    pub fn extend(&mut self, other: &Self) {
        let v_len = self.triangles.vertices.len() as u32;
        let other_indices_with_offset: Indices =
            other.triangles.indices.iter().map(|i| i + v_len).collect();
        self.triangles
            .vertices
            .extend(other.triangles.vertices.iter());
        self.triangles.indices.extend(other_indices_with_offset);
    }
    pub fn new(p: &Path, transform_id: u32) -> Self {
        let triangles = prepare_triangles_from_path(p, transform_id);
        Self {
            triangles,
            id: p.id.to_owned(),
        }
    }
}
