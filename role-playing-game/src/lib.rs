pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // The revive method should check to ensure that the Player is indeed dead (their health has reached 0),
        // and if they are, the method should return a new Player instance with 100 health. If the Player's level
        // is 10 or above, they should also be revived with 100 mana. If the Player's level is below 10, their
        // mana should be None. The revive method should preserve the Player's level.
        if self.health > 0 {
            None
        } else {
            Some(Player {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // The cast_spell method takes a mutable reference to the Player as well as a mana_cost
        // parameter indicating how much mana the spell costs. It returns the amount of damage that
        // the cast spell performs, which will always be two times the mana cost of the spell if the
        // spell is successfully cast.
        return match self.mana {
            // 1- If the player does not have access to a mana pool, attempting to cast the spell must
            // decrease their health by the mana cost of the spell. The damage returned must be 0.
            None => {
                if self.health >= mana_cost {
                    self.health -= mana_cost;
                } else { self.health = 0; }
                0
            }
            // 2- If the player has a mana pool but insufficient mana, the method should not affect the
            // pool, but instead return 0
            Some(mana) => {
                if mana < mana_cost {
                    0
                }
                // 3- Otherwise, the mana_cost should be deducted from the Player's mana pool and the
                // appropriate amount of damage should be returned.
                else {
                    self.mana = Some(mana - mana_cost);
                    mana_cost * 2
                }
            }
        }
    }
}
