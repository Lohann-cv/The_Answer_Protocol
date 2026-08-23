use crate::world::map::WorldId;

pub struct PlayerId(pub u32);

pub enum PlayerClass {
    Warrior,
    Mage,
    Archer,
    Summoner,
    Healer,
    Tank,
    Thief,
}

pub struct Stats {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub stamina: u32,
    pub luck: u32,
}

impl Stats {
    pub fn new(class: PlayerClass) -> Self {
        match class {
            PlayerClass::Warrior => Stats {
                strength: 10,
                agility: 5,
                intelligence: 3,
                stamina: 8,
                luck: 2,
            },
            PlayerClass::Mage => Stats {
                strength: 3,
                agility: 4,
                intelligence: 10,
                stamina: 5,
                luck: 6,
            },
            PlayerClass::Archer => Stats {
                strength: 5,
                agility: 10,
                intelligence: 4,
                stamina: 7,
                luck: 8,
            },
            PlayerClass::Summoner => Stats {
                strength: 4,
                agility: 6,
                intelligence: 9,
                stamina: 6,
                luck: 7,
            },
            PlayerClass::Healer => Stats {
                strength: 6,
                agility: 7,
                intelligence: 8,
                stamina: 9,
                luck: 5,
            },
            PlayerClass::Tank => Stats {
                strength: 9,
                agility: 3,
                intelligence: 4,
                stamina: 10,
                luck: 4,
            },
            PlayerClass::Thief => Stats {
                strength: 7,
                agility: 9,
                intelligence: 6,
                stamina: 8,
                luck: 10,
            },
        }
    }
}

pub struct Player {
    pub id: PlayerId,
    pub level: u32,
    pub experience: u32,
    pub health: u32,
    pub max_health: u32,
    pub position: WorldId,
    pub class: PlayerClass,
    pub stats: Stats,
}

impl Player {
    pub fn new(id: PlayerId, class: PlayerClass) -> Self {
        let stats = Stats::new(class);
        let max_health = stats.stamina * 10;
        Player {
            id,
            level: 1,
            experience: 0,
            health: max_health,
            max_health,
            position: WorldId(0),
            class,
            stats,
        }
    }
}