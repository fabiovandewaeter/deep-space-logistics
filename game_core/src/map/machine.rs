// game_core/src/map/machine.rs
use hecs::World;

use crate::item::{Inventory, recipe::Recipe};

#[derive(Debug)]
pub struct Machine {
    pub recipe: Option<Recipe>,
    pub progress: u32,
    pub is_running: bool,
    pub input_inventory: Inventory,
    pub output_inventory: Inventory,
}
#[derive(Debug)]
pub struct Factory;
pub struct Assembler;
pub struct ChemicalPlant;

pub fn sys_process_machine(world: &mut World) {
    for machine in world.query::<&mut Machine>().iter() {
        let Some(recipe) = &machine.recipe else {
            continue;
        };

        // if craft just started, consume inputs
        if machine.progress == 0 {
            // checks if there is enough items in input inventory
            if !recipe
                .inputs
                .iter()
                .all(|i| machine.input_inventory.has(&i.item_type, i.quantity))
            {
                machine.is_running = false;
                continue;
            }

            // removes from input
            recipe.inputs.iter().for_each(|i| {
                machine.input_inventory.remove(&i.item_type, i.quantity);
            });
        }
        machine.progress += 1;

        if machine.progress >= recipe.duration {
            // adds to output
            recipe.outputs.iter().for_each(|i| {
                machine
                    .output_inventory
                    .add(i.item_type.clone(), i.quantity)
            });
            machine.progress = 0;
        }
    }
}

#[cfg(feature = "native")]
#[cfg(test)]
mod tests {
    use crate::{
        game::Game,
        item::Inventory,
        load_game_data,
        map::machine::{Factory, Machine},
    };

    #[test]
    fn test_factory_consume_inputs_and_produce_output() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let game_data = game.game_data();
        let recipe = game_data.get_recipe("iron_gear");
        let input_item = game_data.get_item_type("iron_plate");
        let output_item = game_data.get_item_type("iron_gear");
        let input_quantity = recipe.inputs[0].quantity;
        let output_quantity = recipe.outputs[0].quantity;
        let duration = recipe.duration;

        let machine_entity = game.world_mut().spawn((
            Machine {
                recipe: Some(recipe),
                progress: 0,
                is_running: true,
                input_inventory: Inventory::default(),
                output_inventory: Inventory::default(),
            },
            Factory,
        ));

        {
            let mut machine = game.get::<&mut Machine>(machine_entity).unwrap();
            machine
                .input_inventory
                .add(input_item.clone(), input_quantity);
            assert!(machine.input_inventory.has(&input_item, input_quantity));
            assert!(machine.output_inventory.items.is_empty());
        }

        for _ in 0..duration {
            game.step_world();
        }

        let machine = game.get::<&Machine>(machine_entity).unwrap();
        assert!(!machine.input_inventory.has(&input_item, input_quantity));
        assert!(machine.output_inventory.has(&output_item, output_quantity));
    }

    #[test]
    fn test_factory_dont_consume_input_if_not_enough_items() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let game_data = game.game_data();
        let recipe = game_data.get_recipe("iron_gear");
        let input_item = game_data.get_item_type("iron_plate");
        let output_item = game_data.get_item_type("iron_gear");
        let input_quantity = recipe.inputs[0].quantity - 1;
        let output_quantity = recipe.outputs[0].quantity;
        let duration = recipe.duration;

        let machine_entity = game.world_mut().spawn((
            Machine {
                recipe: Some(recipe),
                progress: 0,
                is_running: true,
                input_inventory: Inventory::default(),
                output_inventory: Inventory::default(),
            },
            Factory,
        ));

        {
            let mut machine = game.get::<&mut Machine>(machine_entity).unwrap();
            machine
                .input_inventory
                .add(input_item.clone(), input_quantity);
            assert!(machine.input_inventory.has(&input_item, input_quantity));
            assert!(machine.output_inventory.items.is_empty());
        }

        for _ in 0..duration {
            game.step_world();
        }

        let machine = game.get::<&Machine>(machine_entity).unwrap();
        assert!(machine.input_inventory.has(&input_item, input_quantity));
        assert!(!machine.output_inventory.has(&output_item, output_quantity));
    }

    #[test]
    fn test_factory_if_5_in_recipe_duration_it_needs_5_step_world_to_finish() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let game_data = game.game_data();
        let recipe = game_data.get_recipe("iron_gear");
        let input_item = game_data.get_item_type("iron_plate");
        let output_item = game_data.get_item_type("iron_gear");
        let input_quantity = recipe.inputs[0].quantity;
        let output_quantity = recipe.outputs[0].quantity;
        let duration = recipe.duration;

        let machine_entity = game.world_mut().spawn((
            Machine {
                recipe: Some(recipe),
                progress: 0,
                is_running: true,
                input_inventory: Inventory::default(),
                output_inventory: Inventory::default(),
            },
            Factory,
        ));

        {
            let mut machine = game.get::<&mut Machine>(machine_entity).unwrap();
            machine
                .input_inventory
                .add(input_item.clone(), input_quantity);
            assert!(machine.input_inventory.has(&input_item, input_quantity));
            assert!(machine.output_inventory.items.is_empty());
        }

        for _ in 0..duration - 1 {
            game.step_world();

            let machine = game.get::<&Machine>(machine_entity).unwrap();
            assert!(!machine.input_inventory.has(&input_item, input_quantity));
            assert!(!machine.output_inventory.has(&output_item, output_quantity));
        }

        game.step_world();

        let machine = game.get::<&Machine>(machine_entity).unwrap();
        assert!(!machine.input_inventory.has(&input_item, input_quantity));
        assert!(machine.output_inventory.has(&output_item, output_quantity));
    }
}
