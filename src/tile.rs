use player;

pub enum TileType {
    Water,
    Land,
    City,
}
pub struct Tile<'a> {
    tiletype: TileType,
    owner: &'a player::Player,
}
impl<'a> Tile<'a> {
    fn getType(self) -> TileType {
        self.tiletype
    }
    fn getOwner(&self) -> &player::Player {
        self.owner
    }
    fn setOwner(&mut self,owner: &'a player::Player) {
        self.owner = owner;
    }
}
