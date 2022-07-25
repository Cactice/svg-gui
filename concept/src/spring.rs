use glam::Mat4;
use natura::{AngularFrequency, DampingRatio, DeltaTime, Spring};
use std::default::Default;
use std::iter::zip;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{sleep, spawn};
use std::time::Duration;
use windowing::tesselation::callback::Callback;

pub type StaticCallback = Callback<'static, (), ()>;
pub struct SpringMat4NonAtomic {
    spring: Spring,
    target: Mat4,
    pub current: Mat4,
    velocity: Mat4,
    pub is_animating: bool,
}
#[derive(Default, Clone)]
pub struct SpringMat4(pub Arc<Mutex<SpringMat4NonAtomic>>);
impl Default for SpringMat4NonAtomic {
    fn default() -> Self {
        Self {
            spring: Spring::new(
                DeltaTime(natura::fps(60)),
                AngularFrequency(10.0),
                DampingRatio(1.),
            ),
            is_animating: false,
            current: Default::default(),
            target: Default::default(),
            velocity: Default::default(),
        }
    }
}

impl SpringMat4 {
    pub fn get_inner(&self) -> MutexGuard<'_, SpringMat4NonAtomic> {
        self.0.lock().expect("SpringMat4 mutex unwrap failed")
    }
    pub fn spring_to(&mut self, target: Mat4, on_complete: Option<StaticCallback>) {
        {
            self.get_inner().target = target
        }
        self.update(on_complete.unwrap_or(StaticCallback::new(|_| ())));
    }
    pub fn update(&mut self, mut on_complete: StaticCallback) {
        let animating_complete = {
            let mut mutable = self.get_inner();
            let mut current_position_vec = vec![];
            let mut vel_vec = vec![];
            zip(
                zip(
                    mutable.current.to_cols_array(),
                    mutable.velocity.to_cols_array(),
                ),
                mutable.target.to_cols_array(),
            )
            .for_each(|((current_position, vel), target)| {
                let (new_current_position, new_vel) =
                    mutable
                        .spring
                        .update(current_position as f64, vel as f64, target as f64);
                current_position_vec.push(new_current_position as f32);
                vel_vec.push(new_vel as f32);
            });
            mutable.current = Mat4::from_cols_array(&current_position_vec.try_into().unwrap());
            mutable.velocity = Mat4::from_cols_array(&vel_vec.try_into().unwrap());

            mutable.current.abs_diff_eq(mutable.target, 1.0)
                && mutable.velocity.abs_diff_eq(Mat4::ZERO, 100.0)
        };
        if !animating_complete {
            let arc = self.0.clone();
            spawn(move || {
                sleep(Duration::from_millis(16));
                Self(arc).update(on_complete);
            });
        } else {
            on_complete.process_events(&());
            self.0.lock().unwrap().is_animating = false;
        }
    }
}

pub struct MutCount<T> {
    pub unwrapped: T, //TODO: remove pub
    pub mut_count: u8,
}
impl<T: std::default::Default> Default for MutCount<T> {
    fn default() -> Self {
        Self {
            unwrapped: Default::default(),
            mut_count: 1,
        }
    }
}

impl<T> MutCount<T> {
    pub fn reset_mut_count(&mut self) {
        self.mut_count = 0
    }
}
impl<T> From<T> for MutCount<T> {
    fn from(unwrapped: T) -> Self {
        Self {
            unwrapped,
            mut_count: 0,
        }
    }
}
impl<T> Deref for MutCount<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.unwrapped
    }
}
impl<T> DerefMut for MutCount<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mut_count += 1;
        &mut self.unwrapped
    }
}
