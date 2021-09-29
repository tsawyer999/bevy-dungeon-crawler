use crate::element::Element;
use bevy::math::Vec3;

pub fn get_elements() -> [Element; 2] {
    return [
        Element {
            name: "hating bat",
            description: "annoying",
            team: "enemy",
            strength: 4,
            health: 8,
            mana: 0,
            armor: 1,
            min_damage: 1,
            max_damage: 2,
            position: Vec3::new(-1.0, 0.0, 0.0),
        },
        Element {
            name: "loving bat",
            description: "annoying",
            team: "friends",
            strength: 4,
            health: 8,
            mana: 0,
            armor: 1,
            min_damage: 1,
            max_damage: 2,
            position: Vec3::new(1.0, 0.0, 0.0),
        },
    ];
}
