
use dialoguer::{Select, MultiSelect, Input, Confirm};
use std::fmt;

enum MainMenuCommand
{
    ModifyInputOutput { description: String, command: Box<dyn Fn()> },
    ModifyAudioEffects { description: String, command: Box<dyn Fn()> },
    Exit { description: String, command: Box<dyn Fn()> },
}

// Implement Display trait for MainMenuCommand
impl fmt::Display for MainMenuCommand 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match self 
        {
            MainMenuCommand::ModifyInputOutput { description, command: _ } => write!(f, "{description}"),
            MainMenuCommand::ModifyAudioEffects { description, command: _ } => write!(f, "{description}"),
            MainMenuCommand::Exit { description, command: _ } => write!(f, "{description}"),
        }
    }
}

enum InputOutputMenuCommand
{
    SelectInputDevice { description: String, command: Box<dyn Fn()> },
    SelectOutputDevice { description: String, command: Box<dyn Fn()> },
    Back { description: String, command: Box<dyn Fn()> },
}

// Implement Display trait for InputOutputMenuCommand
impl fmt::Display for InputOutputMenuCommand 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match self 
        {
            InputOutputMenuCommand::SelectInputDevice { description, command: _ } => write!(f, "{description}"),
            InputOutputMenuCommand::SelectOutputDevice { description, command: _ } => write!(f, "{description}"),
            InputOutputMenuCommand::Back { description, command: _ } => write!(f, "{description}"),
        }
    }
}

#[derive(Clone)]
struct AudioEffectArgument
{
    name: String,
    value: u32,
    unit: String,
}

enum AudioEffectsMenuCommand
{
    // arguments: (name, value, unit)
    ModifyVolume { description: String, command: Box<dyn Fn(&mut Vec<AudioEffectArgument>)>, arguments: Vec<AudioEffectArgument>},
    ModifyOscillator { description: String, command: Box<dyn Fn(&mut Vec<AudioEffectArgument>)>, arguments: Vec<AudioEffectArgument>},
    ModifyCompressor { description: String, command: Box<dyn Fn(&mut Vec<AudioEffectArgument>)>, arguments: Vec<AudioEffectArgument>},
    ModifyDelay { description: String, command: Box<dyn Fn(&mut Vec<AudioEffectArgument>)>, arguments: Vec<AudioEffectArgument>},
    ModifyFlanger { description: String, command: Box<dyn Fn(&mut Vec<AudioEffectArgument>)>, arguments: Vec<AudioEffectArgument>},
    Back { description: String, command: Box<dyn Fn()> },
}

// Implement Display trait for AudioEffectsMenuCommand
impl fmt::Display for AudioEffectsMenuCommand
{   
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {   
        match self
        {
            AudioEffectsMenuCommand::ModifyVolume { description, command: _, arguments: _ } => write!(f, "{description}"),
            AudioEffectsMenuCommand::ModifyOscillator { description, command: _, arguments: _ } => write!(f, "{description}"),
            AudioEffectsMenuCommand::ModifyCompressor { description, command: _, arguments: _ } => write!(f, "{description}"),
            AudioEffectsMenuCommand::ModifyDelay { description, command: _, arguments: _ } => write!(f, "{description}"),
            AudioEffectsMenuCommand::ModifyFlanger { description, command: _, arguments: _ } => write!(f, "{description}"),
            AudioEffectsMenuCommand::Back { description, command: _ } => write!(f, "{description}"),
        }
    }
}

pub fn cli_main_menu() 
{

    // Print welcome message
    println!("\nWelcome to the Audio Effects CLI!");

    // Populate main menu commands
    let main_menu_commands = 
    [
        MainMenuCommand::ModifyInputOutput 
        {
            description: String::from("Modify Input/Output"),
            command: Box::new(cli_modify_input_output),
        },
        MainMenuCommand::ModifyAudioEffects 
        {
            description: String::from("Modify Audio Effects"),
            command: Box::new(cli_modify_audio_effects),
        },
        MainMenuCommand::Exit 
        {
            description: String::from("Exit"),
            command: Box::new(|| {}),
        },
    ];

    'menu: loop 
    {

        // Spacer
        println!("");

        // Use select to allow user to select main menu command
        let selection = Select::new()
            .with_prompt("Main Menu")
            .items(&main_menu_commands)
            .default(0)
            .interact()
            .unwrap();

        // Execute command based on user selection
        match &main_menu_commands[selection] 
        {
            MainMenuCommand::ModifyInputOutput{description: _, command} => command(),
            MainMenuCommand::ModifyAudioEffects{description: _, command} => command(),
            MainMenuCommand::Exit{description: _, command: _} => break 'menu,
        }

    }

}

fn cli_modify_input_output() 
{

    // Populate input/output menu commands
    let input_output_menu_commands = 
    [
        InputOutputMenuCommand::SelectInputDevice 
        {
            description: String::from("Select Input Device"),
            command: Box::new(|| {}),
        },
        InputOutputMenuCommand::SelectOutputDevice 
        {
            description: String::from("Select Output Device"),
            command: Box::new(|| {}),
        },
        InputOutputMenuCommand::Back 
        {
            description: String::from("Back"),
            command: Box::new(|| {}),
        },
    ];

    'menu: loop 
    {

        // Spacer
        println!("");

        // Use select to allow user to select input/output device to modify
        let selection = Select::new()
            .with_prompt("Modify Input/Output")
            .items(&input_output_menu_commands)
            .default(0)
            .interact()
            .unwrap();

        // Execute command based on user selection
        match &input_output_menu_commands[selection] 
        {
            InputOutputMenuCommand::SelectInputDevice{description: _, command} => command(),
            InputOutputMenuCommand::SelectOutputDevice{description: _, command} => command(),
            InputOutputMenuCommand::Back{description: _, command: _} => break 'menu,
        }

    }

}

fn cli_modify_audio_effects() 
{

    // Populate audio effects menu commands (get current/live parameter values)
    let audio_effects_menu_commands = 
    [
        &mut AudioEffectsMenuCommand::ModifyVolume 
        {
            description: String::from("Modify Volume"),
            command: Box::new(|arguments| cli_input_and_modify_audio_effect_arguments(arguments)),
            arguments: vec!
            [
                AudioEffectArgument 
                { 
                    name: String::from("Volume"),
                    value: 0,
                    unit: String::from("dB")
                },
            ],
        },
        &mut AudioEffectsMenuCommand::ModifyOscillator 
        {
            description: String::from("Modify Oscillator"),
            command: Box::new(|arguments| cli_input_and_modify_audio_effect_arguments(arguments)),
            arguments: vec!
            [
                AudioEffectArgument 
                { 
                    name: String::from("Frequency"),
                    value: 0,
                    unit: String::from("Hz")
                },
            ],
        },
        &mut AudioEffectsMenuCommand::ModifyCompressor 
        {
            description: String::from("Modify Compressor"),
            command: Box::new(|arguments| cli_input_and_modify_audio_effect_arguments(arguments)),
            arguments: vec!
            [
                AudioEffectArgument 
                { 
                    name: String::from("Threshold"),
                    value: 0,
                    unit: String::from("dB")
                },
                AudioEffectArgument 
                { 
                    name: String::from("Ratio"),
                    value: 0,
                    unit: String::from("dB")
                },
            ],
        },
        &mut AudioEffectsMenuCommand::ModifyDelay 
        {
            description: String::from("Modify Delay"),
            command: Box::new(|arguments| cli_input_and_modify_audio_effect_arguments(arguments)),
            arguments: vec!
            [
                AudioEffectArgument 
                { 
                    name: String::from("Time"),
                    value: 0,
                    unit: String::from("ms")
                },
            ],
        },
        &mut AudioEffectsMenuCommand::ModifyFlanger 
        {
            description: String::from("Modify Flanger"),
            command: Box::new(|arguments| cli_input_and_modify_audio_effect_arguments(arguments)),
            arguments: vec!
            [
                AudioEffectArgument 
                { 
                    name: String::from("Depth"),
                    value: 0,
                    unit: String::from("ms")
                },
                AudioEffectArgument 
                { 
                    name: String::from("Rate"),
                    value: 0,
                    unit: String::from("Hz")
                },
            ],
        },
        &mut AudioEffectsMenuCommand::Back 
        {
            description: String::from("Back"),
            command: Box::new(|| {}),
        },
    ];

    'menu: loop 
    {

        // Spacer
        println!("");

        // Use multi-select to allow user to select multiple audio effects to modify
        let selection = MultiSelect::new()
            .with_prompt("Modify Audio Effects")
            .items(&audio_effects_menu_commands)
            .interact()
            .unwrap();

        // Iterate through each selected audio effect and modify its parameters
        for i in selection
        {

            // Execute command based on user selection
            match audio_effects_menu_commands[i]
            {
                AudioEffectsMenuCommand::ModifyVolume{description, command, arguments} => 
                {
                    println!("\n{description}:");
                    command(arguments);
                },
                AudioEffectsMenuCommand::ModifyOscillator{description, command, arguments} =>
                {
                    println!("\n{description}:");
                    command(arguments);
                },
                AudioEffectsMenuCommand::ModifyCompressor{description, command, arguments} =>
                {
                    println!("\n{description}:");
                    command(arguments);
                },
                AudioEffectsMenuCommand::ModifyDelay{description, command, arguments} =>
                {
                    println!("\n{description}:");
                    command(arguments);
                },
                AudioEffectsMenuCommand::ModifyFlanger{description, command, arguments} =>
                {
                    println!("\n{description}:");
                    command(arguments);
                },
                AudioEffectsMenuCommand::Back{description: _, command: _} => break 'menu,
            }
        }

    }

}

fn cli_input_and_modify_audio_effect_arguments(arguments: &mut Vec<AudioEffectArgument>) 
{

    // Create copy of arguments to pass to Input::new()
    let mut arguments_temp = arguments.clone();

    // Iterate through each argument and prompt user for input
    arguments_temp.iter_mut().for_each(|argument| 
    {
        argument.value = Input::new()
            .with_prompt(&format!("Enter {} (current value: {} {})", argument.name, argument.value, argument.unit))
            .interact()
            .unwrap();
    });

    // Confirm with user to apply changes
    if Confirm::new()
        .with_prompt("Apply changes?")
        .interact()
        .unwrap()
    {

        // Apply changes to original arguments
        for i in 0..arguments.len() 
        {
            arguments[i].value = arguments_temp[i].value;
        }
    
    }

}