#[derive(Default)]
struct GameState {
    data: super::super::script::lua::CardData,
    current_round: usize,
    // total_players: usize,
    winning_player: Option<String>,
}

// #[derive(Default,Copy, Clone)]
// struct CardData {
//     cards :i32,
// }



// impl UserData for CardData {
//     fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
//         fields.add_field_method_get("current_round", |_, this| Ok(this.cards));
//         fields.add_field_method_set("current_round", |_, this, val| {
//             this.cards = val;
//             Ok(())
//         });
//     }
// }