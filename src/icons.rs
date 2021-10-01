pub struct Texture {
    pub id: u64,
    pub path: &'static str,
}

pub const ICON_SIZE: f32 = 40.0;

pub const ICO_ARMOR_POINT: Texture = Texture {
    id: 1,
    path: "icons/ico_armor_point.png",
};

pub const ICO_DAMAGE_POINT: Texture = Texture {
    id: 2,
    path: "icons/ico_damage_point.png",
};

pub const ICO_HEALTH_POINT: Texture = Texture {
    id: 3,
    path: "icons/ico_health_point.png",
};

pub const ICO_MANA_POINT: Texture = Texture {
    id: 4,
    path: "icons/ico_mana_point.png",
};

pub const ICO_MOVEMENT_POINT: Texture = Texture {
    id: 5,
    path: "icons/ico_movement_point.png",
};

pub const ICO_STAT_DEXTERITY: Texture = Texture {
    id: 6,
    path: "icons/ico_stat_dexterity.png",
};

pub const ICO_STAT_STRENGTH: Texture = Texture {
    id: 7,
    path: "icons/ico_stat_strength.png",
};

pub const ICO_TEAM_FRIENDS: Texture = Texture {
    id: 8,
    path: "icons/ico_team_friend.png",
};

pub const ICO_TEAM_ENEMY: Texture = Texture {
    id: 8,
    path: "icons/ico_team_enemy.png",
};

pub const TEXTURES: [Texture; 9] = [
    ICO_ARMOR_POINT,
    ICO_DAMAGE_POINT,
    ICO_HEALTH_POINT,
    ICO_MANA_POINT,
    ICO_MOVEMENT_POINT,
    ICO_STAT_DEXTERITY,
    ICO_STAT_STRENGTH,
    ICO_TEAM_FRIENDS,
    ICO_TEAM_ENEMY,
];
