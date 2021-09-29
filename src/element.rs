use bevy::math::Vec3;

pub struct Element {
    pub name: &'static str,
    pub description: &'static str,
    pub team: &'static str,
    pub strength: i32,
    pub health: i32,
    pub mana: i32,
    pub armor: i32,
    pub min_damage: i32,
    pub max_damage: i32,
    pub position: Vec3
}
