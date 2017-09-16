pub enum UnitType {
    Boat,
    Soldier,
    //ya there's gonna be a lot more
}
pub struct Unit {
    unitType: UnitType,
    health: u8,
}
