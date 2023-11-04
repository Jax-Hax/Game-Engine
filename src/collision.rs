use glam::{Vec2, Vec3};
use winit::dpi::PhysicalPosition;

use bevy_ecs::prelude::*;

use crate::{resources::WindowEvents, prelude::Instance};
#[derive(Component)]
pub struct Box2D {
    x_max: f32,
    x_min: f32,
    y_max: f32,
    y_min: f32,
    enabled: bool
}
impl Box2D {
    pub fn new(p1: Vec2, p2: Vec2) -> Self{
        Box2D {
            x_max: if p1.x > p2.x {p1.x} else {p2.x},
            x_min: if p1.x < p2.x {p1.x} else {p2.x},
            y_max: if p1.y > p2.y {p1.y} else {p2.y},
            y_min: if p1.y < p2.y {p1.y} else {p2.y},
            enabled: true
        }
    }
    pub fn check_collision(&self, instance: &Instance, window_events: &WindowEvents) -> bool {
        let x = window_events.screen_mouse_pos.x + instance.position.x;
        let y = (window_events.screen_mouse_pos.y + instance.position.y) / window_events.aspect_ratio;
        if self.enabled{
            if x < self.x_max && x > self.x_min && y < self.y_max && y > self.y_min {
                return true;
            }
        }
        return false;
    }
}
#[derive(Component)]
pub struct Circle {
    center_x: f32,
    center_y: f32,
    radius: f32,
    enabled: bool,
}
impl Circle {
    pub fn new(center: Vec2, radius: f32, enabled: bool) -> Self{
        Circle {
            center_x: center.x,
            center_y: center.y,
            radius,
            enabled
        }
    }
    pub fn check_collision(&self, pos: &PhysicalPosition<f32>) -> bool {
        let x = pos.x;
        let y = pos.y;
        //find distance betweenn two points
        let dist_x = (x - self.center_x).powi(2);
        let dist_y = (y - self.center_y).powi(2);
        let dist = (dist_x + dist_y).sqrt();
        if self.enabled && dist < self.radius {
            return true;
        }
        return false;
    }
}
pub fn oriented_bounding_box_with_ray(
    ray_origin: Vec3,        // Ray origin, in world space
	ray_direction: Vec3,     // Ray direction (NOT target position!), in world space. Must be normalize()'d.
	aabb_min: Vec3,          // Minimum X,Y,Z coords of the mesh when not transformed at all.
	aabb_max: Vec3,          // Maximum X,Y,Z coords. Often aabb_min*-1 if your mesh is centered, but it's not always the case.
	model_matrix) -> f32 /*intersection distance */