use unit;

pub struct Player {
    units: Vec<unit::Unit>,
    coins: u64,
}
impl Player {
    fn getUnits(self) -> Vec<unit::Unit> {
        self.units
    }
    fn addUnit(&mut self,unit: unit::Unit) {
        self.units.push(unit);
    }
    fn removeUnit(&mut self,elem: usize) {
        self.units.swap_remove(elem);
    }
    fn getCoins(&self) -> u64 {
        self.coins
    }
    fn setCoins(&mut self,coins: u64) {
        self.coins = coins;
    }
    fn addCoins(&mut self,coins: u64) {
        self.coins += coins;
    }
}
