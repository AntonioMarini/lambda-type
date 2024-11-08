
use bevy::{math::Vec2, prelude::Component};

/* #region Common */

#[derive(Component, Clone, Copy)]
pub struct Velocity {
    pub velocity: Vec2
}

impl Default for Velocity {
    fn default() -> Self {
        Velocity {
            velocity: Vec2::ZERO
        }
    }
}

#[derive(Component, Clone, Copy)]
#[allow(dead_code)]
pub struct Speed {
    pub base_speed: f32,
    pub max_speed: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Speed { base_speed: 100., max_speed: 100. }
    }
}

#[derive(Component)]
#[allow(dead_code)]
pub struct Acceleration {
    pub acceleration: f32
}

impl Default for Acceleration {
    fn default() -> Self {
        Acceleration {
            acceleration: 0.
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Direction {
    pub direction: Vec2
}

impl Default for Direction {
    fn default() -> Self {
        Direction {
            direction: Vec2::ZERO
        }
    }
}

#[derive(Component, Clone, Copy)]
#[allow(dead_code)]
pub struct Health{
    pub hp: i32,
    pub max_hp: i32
}

impl Default for Health {
    fn default() -> Self {
        Health { hp: 100, max_hp: 100}
    }
}

#[derive(Component, Clone, Copy)]
#[allow(dead_code)]
pub struct Boost {
    pub boost : i32,
    pub max_boost: i32
}

impl Default for Boost {
    fn default() -> Self {
        Boost {
            boost: 300,
            max_boost: 300
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Damage {
    #[allow(dead_code)]
    pub damage: f32
}

#[derive(Component, Clone, Copy)]
pub struct LifeTime {
    pub lifetime_millis: i128
}
/* #endregion */


/* #endregion */

#[derive(Component, Clone, Copy)]
#[allow(dead_code)]
pub enum Actors {
    Player,
    Enemy
}
