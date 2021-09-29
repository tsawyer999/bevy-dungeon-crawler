use bevy::math::Vec3;

#[derive(Copy, Clone)]
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
    pub position: Vec3,
}

lazy_static! {
    pub static ref EMPTY_ELEMENT: Element = {
        let element = Element {
            name: "",
            description: "",
            team: "",
            strength: 0,
            health: 0,
            mana: 0,
            armor: 0,
            min_damage: 0,
            max_damage: 0,
            position: Vec3::new(0.0, 0.0, 0.0)
        };

        return element;
    };
}
