use guppies::glam::Mat4;
use salvage::usvg::PathBbox;

#[derive(Debug, Clone, Copy)]
pub enum XConstraint {
    Left(f32),
    Right(f32),
    LeftAndRight { left: f32, right: f32 },
    Center(f32), //rightward_from_center
    Scale,
}

impl Default for XConstraint {
    fn default() -> Self {
        Self::LeftAndRight {
            left: 0.,
            right: 0.,
        }
    }
}
impl XConstraint {
    fn to_pre_post_transform(self, display: Mat4, svg: Mat4, bbox: PathBbox) -> (Mat4, Mat4) {
        let fill_x = Mat4::from_scale(
            [
                display.to_scale_rotation_translation().0.x
                    / svg.to_scale_rotation_translation().0.x,
                1.,
                1.,
            ]
            .into(),
        );
        let left_align = Mat4::from_translation([bbox.x() as f32, 0.0, 0.0].into()).inverse();
        let right_align =
            Mat4::from_translation([(bbox.x() + bbox.width()) as f32, 0.0, 0.0].into()).inverse();
        let center_x =
            Mat4::from_translation([(bbox.x() + bbox.width() / 2.) as f32, 0.0, 0.0].into())
                .inverse();

        let pre_scale_translate_x;
        let pre_scale_scale_x;
        let post_scale_translate_x;
        match self {
            XConstraint::Left(left) => {
                pre_scale_translate_x = left_align * Mat4::from_translation([left, 0., 0.].into());
                post_scale_translate_x = Mat4::from_translation([-1.0, 0., 0.].into());
                pre_scale_scale_x = Mat4::IDENTITY;
            }
            XConstraint::Right(right) => {
                pre_scale_translate_x =
                    right_align * Mat4::from_translation([right, 0., 0.].into());
                post_scale_translate_x = Mat4::from_translation([1.0, 0., 0.].into());
                pre_scale_scale_x = Mat4::IDENTITY;
            }
            XConstraint::Center(rightward_from_center) => {
                pre_scale_translate_x =
                    center_x * Mat4::from_translation([rightward_from_center, 0., 0.].into());
                post_scale_translate_x = Mat4::IDENTITY;
                pre_scale_scale_x = Mat4::IDENTITY;
            }
            XConstraint::LeftAndRight { left: _, right: _ } => {
                todo!();
            }
            XConstraint::Scale => {
                pre_scale_translate_x = center_x;
                post_scale_translate_x = Mat4::IDENTITY;
                pre_scale_scale_x = fill_x;
            }
        };
        (
            pre_scale_scale_x * pre_scale_translate_x,
            post_scale_translate_x,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum YConstraint {
    Top(f32),
    Bottom(f32),
    TopAndBottom { top: f32, bottom: f32 },
    Center(f32), //downward_from_center
    Scale,
}

impl Default for YConstraint {
    fn default() -> Self {
        Self::TopAndBottom {
            top: 0.,
            bottom: 0.,
        }
    }
}

impl YConstraint {
    fn to_pre_post_transform(self, display: Mat4, svg: Mat4, bbox: PathBbox) -> (Mat4, Mat4) {
        let fill_y = Mat4::from_scale(
            [
                1.,
                display.to_scale_rotation_translation().0.y
                    / svg.to_scale_rotation_translation().0.y,
                1.,
            ]
            .into(),
        );
        let top_align = Mat4::from_translation([0.0, bbox.y() as f32, 0.0].into()).inverse();
        let bottom_align =
            Mat4::from_translation([0.0, (bbox.y() + bbox.height()) as f32, 0.0].into()).inverse();
        let center_y =
            Mat4::from_translation([0.0, (bbox.y() + bbox.height() / 2.) as f32, 0.0].into())
                .inverse();

        let pre_scale_translate_y;
        let pre_scale_scale_y;
        let post_scale_translate_y;
        match self {
            YConstraint::Top(top) => {
                pre_scale_translate_y = bottom_align * Mat4::from_translation([0., top, 0.].into());
                post_scale_translate_y = Mat4::from_translation([-1.0, 0., 0.].into());
                pre_scale_scale_y = Mat4::IDENTITY;
            }
            YConstraint::Bottom(bottom) => {
                pre_scale_translate_y =
                    top_align * Mat4::from_translation([0., -bottom, 0.].into());
                post_scale_translate_y = Mat4::from_translation([1.0, 0., 0.].into());
                pre_scale_scale_y = Mat4::IDENTITY;
            }
            YConstraint::Center(downward_from_center) => {
                pre_scale_translate_y =
                    center_y * Mat4::from_translation([0., downward_from_center, 0.].into());
                post_scale_translate_y = Mat4::IDENTITY;
                pre_scale_scale_y = Mat4::IDENTITY;
            }
            YConstraint::TopAndBottom { top: _, bottom: _ } => {
                todo!();
            }
            YConstraint::Scale => {
                pre_scale_translate_y = center_y;
                post_scale_translate_y = Mat4::IDENTITY;
                pre_scale_scale_y = fill_y;
            }
        };
        (
            pre_scale_scale_y * pre_scale_translate_y,
            post_scale_translate_y,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Constraint {
    pub x: XConstraint,
    pub y: YConstraint,
}

impl Constraint {
    pub fn to_mat4(self, display: Mat4, svg: Mat4, bbox: PathBbox) -> Mat4 {
        let Constraint {
            x: constraint_x,
            y: constraint_y,
        } = self;

        let (pre_x, post_x) = constraint_x.to_pre_post_transform(display, svg, bbox);
        let (pre_y, post_y) = constraint_y.to_pre_post_transform(display, svg, bbox);

        let pre_xy = pre_x * pre_y;
        let post_xy = post_x * post_y;
        let normalize_scale = Mat4::from_scale([2., 2., 1.].into())
            * (Mat4::from_scale([0.5, 0.5, 1.].into()) * display).inverse();

        return post_xy * normalize_scale * pre_xy;
    }
}