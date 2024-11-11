use bevy::{math::Vec2, prelude::Component};

/* #region Common */
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
#[allow(dead_code)]
pub struct Damage {
    pub damage: f32
}

impl Default for Damage {
    fn default() -> Self {
        Damage { damage: 50. }
    }
}

#[derive(Component, Clone, Copy)]
pub struct LifeTime {
    pub lifetime_millis: i128
}

impl Default for LifeTime {
    fn default() -> Self {
        LifeTime { lifetime_millis: 1000 }
    }
}
/* #endregion */


/* #endregion */

#[derive(Component, Clone, Copy, PartialEq, Default)]
#[allow(dead_code)]
pub enum Actors {
    #[default]
    None,
    Player,
    Enemy
}

#[derive(Component, Copy, Clone)]
pub struct Speed {
    pub value: f32
}

impl Default for Speed {
    fn default() -> Self {
        Speed {value: 100.}
    }
}

#[derive(Component, Copy, Clone)]
pub struct Orientation {
    pub value: Vec2
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation {
            value: Vec2::ZERO
        }
    }
}


