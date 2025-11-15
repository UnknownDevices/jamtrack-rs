use crate::rect::Rect;
use crate::strack::STrack;
use std::fmt::Debug;

/* ------------------------------------------------------------------------------
 * Object struct
 * ------------------------------------------------------------------------------ */

#[derive(Debug, Clone, Default)]
pub struct Object {
    pub rect: Rect<f32>,
    pub score: f32,
    pub label: i32,
    pub group: i32,
    pub track_id: Option<usize>,
}

impl Object {
    pub fn new(
        rect: Rect<f32>,
        score: f32,
        label: i32,
        group: i32,
        track_id: Option<usize>,
    ) -> Self {
        Self {
            rect,
            score,
            label,
            group,
            track_id,
        }
    }

    #[inline(always)]
    pub fn get_rect(&self) -> &Rect<f32> {
        &self.rect
    }

    #[inline(always)]
    pub fn get_x(&self) -> f32 {
        self.rect.x()
    }

    #[inline(always)]
    pub fn get_y(&self) -> f32 {
        self.rect.y()
    }

    #[inline(always)]
    pub fn get_width(&self) -> f32 {
        self.rect.width()
    }

    #[inline(always)]
    pub fn get_height(&self) -> f32 {
        self.rect.height()
    }

    #[inline(always)]
    pub fn get_score(&self) -> f32 {
        self.score
    }

    #[inline(always)]
    pub fn get_track_id(&self) -> Option<usize> {
        self.track_id
    }
}

impl From<STrack> for Object {
    fn from(strack: STrack) -> Self {
        Object::new(
            strack.get_rect(),
            strack.get_score(),
            strack.label,
            strack.group,
            Some(strack.get_track_id()),
        )
    }
}

impl From<&STrack> for Object {
    fn from(strack: &STrack) -> Self {
        Object::new(
            strack.get_rect(),
            strack.get_score(),
            strack.label,
            strack.group,
            Some(strack.get_track_id()),
        )
    }
}
