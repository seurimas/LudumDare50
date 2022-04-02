use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterizedSpriteAnimation {
    base_frames: Vec<usize>,
    pub parameters: usize,
    pub parameter_offset: usize,
    pub single_frame_duration: f32,
}

impl ParameterizedSpriteAnimation {
    pub fn new(parameter_offset: usize, parameters: usize, single_frame_duration: f32) -> Self {
        ParameterizedSpriteAnimation {
            base_frames: vec![],
            parameters,
            parameter_offset,
            single_frame_duration,
        }
    }

    pub fn len(&self) -> usize {
        self.base_frames.len()
    }

    pub fn add_frame(&mut self, sprite_idx: usize) {
        self.base_frames.push(sprite_idx);
    }

    pub fn get_frame_with_parameter(&self, frame_idx: usize, parameter: usize) -> Option<usize> {
        if parameter >= self.parameters {
            None
        } else if frame_idx < self.base_frames.len() {
            Some(self.base_frames[frame_idx] + parameter * self.parameter_offset)
        } else {
            None
        }
    }

    pub fn set_frame(&mut self, frame_idx: usize, sprite_idx: usize) {
        self.base_frames[frame_idx] = sprite_idx;
    }

    pub fn remove_frame(&mut self, frame_idx: usize) {
        self.base_frames.remove(frame_idx);
    }

    pub fn sample_with_parameter(
        &self,
        parameter: usize,
        mut time_index: f32,
        looping: bool,
    ) -> Option<usize> {
        let animation_duration = self.base_frames.len() as f32 * self.single_frame_duration;
        if time_index >= animation_duration {
            if !looping {
                return None;
            } else {
                time_index = time_index % animation_duration;
            }
        }
        let frame_idx = (time_index / self.single_frame_duration) as usize;
        self.get_frame_with_parameter(frame_idx, parameter)
    }
}
