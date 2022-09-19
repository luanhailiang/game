use super::super::script::lua::LuaState;
fn print_message_system() {
    println!("This game is fun!");
}

fn new_round_system(mut game_state: ResMut<GameState>) {
    // game_state.current_round += 1;
    // println!(
    //     "Begin round {} ",
    //     game_state.current_round
    // );
    // game_state.data;
    game_state.data.cards+=1;
    println!("{:?}",game_state.data.cards);
    LuaState::get_instance().lock().unwrap().exec("test",&game_state.data).unwrap();
    println!("{:?}",game_state.data.cards);
}

fn score_check_system(
    mut game_state: ResMut<GameState>,
    query: Query<(&Player, &Score)>,
) {
    for (player, score) in &query {
        if score.value == 100 {
            game_state.winning_player = Some(player.name.clone());
        }
    }
}

fn game_over_system(
    game_state: Res<GameState>,
) {
    if let Some(ref player) = game_state.winning_player {
        println!("{} won the game!", player);
    } else if game_state.current_round == 100 {
        println!("Ran out of rounds. Nobody wins!");
    }
}
