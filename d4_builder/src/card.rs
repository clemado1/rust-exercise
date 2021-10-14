use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Ability {
    Charge,
    Taunt,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Trigger {
    BattleCry,
    Death,
    EnemyDeath,
    Damage,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub name: String,
    pub strength: i32,
    pub health: i32,
    pub cost: i32,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            name: "".to_string(),
            strength: 1,
            health: 1,
            cost: 1,
            abilities: Vec::new(),
            triggers: BTreeMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct CardBuilder {
    pub name: String,
    pub strength: Option<i32>,
    pub health: Option<i32>,
    pub cost: Option<i32>,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl CardBuilder {
    pub fn new (name: String) -> Self {
        CardBuilder {
            name,
            ..Default::default()
        }
    }

    pub fn strength(mut self, s: i32) -> Self {
        self.strength = Some(s);
        self
    }

    pub fn health(mut self, s: i32) -> Self {
        self.health = Some(s);
        self
    }

    pub fn cost(mut self, s: i32) -> Self {
        self.cost = Some(s);
        self
    }

    pub fn trigger(mut self, t: Trigger, s: String) -> Self {
        self.triggers.insert(t, s);
        self
    }

    pub fn build(self) -> Card {
        Card {
            name: self.name,
            strength: self.strength.unwrap_or(1),
            health: self.health.unwrap_or(1),
            cost: self.cost.unwrap_or(1),
            abilities: self.abilities,
            triggers: self.triggers,
        }
    }
}

impl Card {
    pub fn build(name: String) -> CardBuilder {
        CardBuilder::new(name)
    }
}

#[cfg(test)]
mod test_builder {
    use super::*;

    #[test]
    pub fn test_card_builder() {
        let c1 = Card::build("General Blight".to_string()).
                strength(4)
                .trigger(Trigger::BattleCry, "Deal 2 Damage".to_string())
                .build();

        let mut triggers = BTreeMap::new();
        triggers.insert(Trigger::BattleCry, "Deal 2 Damage".to_string());
        let c2 = Card {
            name: "General Blight".to_string(),
            strength: 4,
            health: 1,
            cost: 1,
            abilities: Vec::new(),
            triggers,
        };

        assert_eq!(c1, c2);
    }

    #[test]
    pub fn test_card_default() {
        let c1 = Card {
            name: "General Blight".to_string(),
            ..Default::default()
        };

        let c2 = Card {
            name: "General Blight".to_string(),
            strength: 1,
            health: 1,
            cost: 1,
            abilities: Vec::new(),
            triggers: BTreeMap::new(),
        };

        assert_eq!(c1, c2);
    }
}