use rand::Rng;
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score: i32,
    current_score: i32,
    enemy_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_index: 0,
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

const MOVEMENT_SPEED: f32 = 250.0;

fn main() {
    let mut game = Game::new();

    game.audio_manager
        .play_music("music/Mysterious Magic.ogg", 0.3);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = UP;
    player.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let reset_label = game.add_text("reset_label", "Press R to reset");
    reset_label.translation = Vec2::new(520.0, 280.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

// This fires every frame
fn game_logic(engine: &mut Engine, state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        println!("Collision: {:?}", event);

        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            state.current_score += 1;

            println!("Current Score: {}", state.current_score);

            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", state.current_score);

            if state.current_score > state.high_score {
                state.high_score = state.current_score;

                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", state.high_score);
            }

            for label in event.pair.into_iter() {
                if !label.starts_with("player") {
                    engine.sprites.remove(&label);
                }
            }

            engine.audio_manager.play_sfx("sfx/impact1.ogg", 0.5);
        }
    }

    if state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("enemy_{}", state.enemy_index);
        let enemy = engine.add_sprite(label.clone(), SpritePreset::RacingCarRed);

        enemy.translation = Vec2::new(
            rand::thread_rng().gen_range(-550.0..550.0),
            rand::thread_rng().gen_range(-325.0..325.0),
        );
        enemy.collision = true;

        state.enemy_index += 1;
    }

    handle_keyboard_events(engine, state);

    // Spawn enemies on mouse click
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(location) = engine.mouse_state.location() {
            let label = format!("enemy_{}", state.enemy_index);
            let enemy = engine.add_sprite(label.clone(), SpritePreset::RacingCarRed);

            enemy.translation = location;
            enemy.collision = true;

            state.enemy_index += 1;

            engine.audio_manager.play_sfx("sfx/click.ogg", 0.5);
        }
    }
}

fn handle_keyboard_events(engine: &mut Engine, state: &mut GameState) {
    let player = engine.sprites.get_mut("player").unwrap();

    // WASD movement
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::W, KeyCode::Up])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::S, KeyCode::Down])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::A, KeyCode::Left])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::D, KeyCode::Right])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    // Press R to reset
    if engine.keyboard_state.pressed(KeyCode::R) {
        let score = engine.texts.get_mut("score").unwrap();
        score.value = format!("Score: {}", state.current_score);

        // Collect labels of enemies to be removed
        let labels_to_remove: Vec<_> = engine
            .sprites
            .iter()
            .filter(|(label, _)| label.starts_with("enemy"))
            .map(|(label, _)| label.clone())
            .collect();

        // Remove all enemies
        for label in labels_to_remove {
            engine.sprites.remove(&label);
        }

        state.current_score = 0;
        state.enemy_index = 0;
    }
}
