use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum HitboxType {
    Hit,
    Block,
    User(u8),
}

#[derive(Component, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Hitbox {
    pub hitbox_type: HitboxType,
    pub min: Vec2,
    pub max: Vec2,
}

impl Hitbox {
    pub fn get_center(&self) -> Vec2 {
        Vec2::new(self.min.x + self.max.x, -self.min.y - self.max.y) / 2.0
    }

    pub fn get_half_x(&self) -> f32 {
        (self.max.x - self.min.x) / 2.0
    }

    pub fn get_half_y(&self) -> f32 {
        (self.max.y - self.min.y) / 2.0
    }

    pub fn flip_x(&mut self) {
        let min = Vec2::new(-self.max.x, self.min.y);
        let max = Vec2::new(-self.min.x, self.max.y);
        println!("{:?} {:?} -> {:?} {:?}", self.min, self.max, min, max);
        self.min = min;
        self.max = max;
    }
}

#[derive(Component, Default, Debug, Clone, Deserialize, Serialize)]
pub struct SpriteSheetHitboxes {
    hitboxes: Vec<Vec<Hitbox>>,
}

impl SpriteSheetHitboxes {
    pub fn from_texture_atlas(texture_atlas: &TextureAtlas) -> Self {
        let mut hitboxes = Vec::new();
        hitboxes.resize_with(texture_atlas.textures.len(), || vec![]);
        SpriteSheetHitboxes { hitboxes }
    }
    pub fn is_for(&self, texture_atlas: &TextureAtlas) -> bool {
        texture_atlas.textures.len() == self.hitboxes.len()
    }
    pub fn add_hitbox(&mut self, idx: usize, hitbox: Hitbox) {
        if let Some(hitboxes) = self.hitboxes.get_mut(idx) {
            hitboxes.push(hitbox);
        }
    }
    pub fn remove_hitbox(&mut self, idx: usize, hitbox_idx: usize) {
        if let Some(hitboxes) = self.hitboxes.get_mut(idx) {
            hitboxes.remove(hitbox_idx);
        }
    }
    pub fn get_hitboxes(&self, idx: usize) -> Option<&Vec<Hitbox>> {
        self.hitboxes.get(idx)
    }
}
