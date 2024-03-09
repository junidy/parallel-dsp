
use dialoguer::{Select, MultiSelect, Input, Confirm};
use std::fmt;

enum Command 
{
    MainMenu        (MainMenuCommand),
    AudioEffectsMenu(AudioEffectsCommand),
}

// fmt::Display trait for Command
impl fmt::Display for Command 
{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {

        match self 
        {
            Command::MainMenu        (command) => write!(f, "{}", command),
            Command::AudioEffectsMenu(command) => write!(f, "{}", command),
        }

    }

}
enum MainMenuCommand 
{
    UseGUI            (String),
    ModifyIODevice    (String),
    ModifyAudioEffects(String),
    Exit              (String)
}

impl fmt::Display for MainMenuCommand {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match self {
            MainMenuCommand::UseGUI            (s) => write!(f, "{}", s),
            MainMenuCommand::ModifyIODevice    (s) => write!(f, "{}", s),
            MainMenuCommand::ModifyAudioEffects(s) => write!(f, "{}", s),
            MainMenuCommand::Exit              (s) => write!(f, "{}", s),
        }
    }
}

enum AudioEffectsCommand {
    ModifyOscillator(String, ((String, u32), (String, u32), (String, u32))),
    ModifyDelay(String, ((String, u32), (String, u32), (String, u32))),
    ModifyFlanger(String, ((String, u32), (String, u32), (String, u32))),
    ModifyCompressor(String, ((String, u32), (String, u32), (String, u32))),
    ModifyVolume(String, ((String, u32), (String, u32), (String, u32))),
}

impl fmt::Display for AudioEffectsCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match self 
        {
            AudioEffectsCommand::ModifyOscillator(s, _) => write!(f, "{}", s),
            AudioEffectsCommand::ModifyDelay     (s, _) => write!(f, "{}", s),
            AudioEffectsCommand::ModifyFlanger   (s, _) => write!(f, "{}", s),
            AudioEffectsCommand::ModifyCompressor(s, _) => write!(f, "{}", s),
            AudioEffectsCommand::ModifyVolume    (s, _) => write!(f, "{}", s),
        }
    }
}

pub fn cli_main_menu()
{

    let main_menu_commands = vec!
    [
        Command::MainMenu(MainMenuCommand::UseGUI            (String::from("Use Graphical User Interface (GUI)"))),
        Command::MainMenu(MainMenuCommand::ModifyIODevice    (String::from("Modify Input/Output Devices"))),
        Command::MainMenu(MainMenuCommand::ModifyAudioEffects(String::from("Modify Audio Effects"))),
        Command::MainMenu(MainMenuCommand::Exit              (String::from("Exit")))
    ];

    loop
    {

        // Prompt user to select a command
        let selected_command = Select::new()
            .with_prompt("Main Menu")
            .items(&main_menu_commands)
            .default(0)
            .interact()
            .unwrap();

        match main_menu_commands[selected_command]
        {

            Command::MainMenu(MainMenuCommand::UseGUI(_))             => {},
            Command::MainMenu(MainMenuCommand::ModifyIODevice(_))     => {},
            Command::MainMenu(MainMenuCommand::ModifyAudioEffects(_)) => cli_audio_effects_menu(),
            Command::MainMenu(MainMenuCommand::Exit(_))               => break,
            _                                                         => {}
        }

    }

}

fn cli_audio_effects_menu()
{

    let audio_effects_commands = vec!
    [
        Command::AudioEffectsMenu(AudioEffectsCommand::ModifyOscillator(
            String::from("Modify Oscillator"), // Title
            ((String::from("Frequency"), 0), (String::from("Amplitude"), 0), (String::from("Phase"), 0)) // Parameters
        )),
        Command::AudioEffectsMenu(AudioEffectsCommand::ModifyDelay(
            String::from("Modify Delay"),
            ((String::from("Delay Time"), 0), (String::from("Feedback"), 0), (String::from("Wet/Dry Mix"), 0))
        )),
        Command::AudioEffectsMenu(AudioEffectsCommand::ModifyFlanger(
            String::from("Modify Flanger"),
            ((String::from("Frequency"), 0), (String::from("Depth"), 0), (String::from("Wet/Dry Mix"), 0))
        )),
        Command::AudioEffectsMenu(AudioEffectsCommand::ModifyCompressor(
            String::from("Modify Compressor"),
            ((String::from("Threshold"), 0), (String::from("Ratio"), 0), (String::from("Gain"), 0))
        )),
        Command::AudioEffectsMenu(AudioEffectsCommand::ModifyVolume(
            String::from("Modify Volume"),
            ((String::from("Volume"), 0), (String::from("Pan"), 0), (String::from("Mute"), 0))
        )),
    ];

    let selected_commands = MultiSelect::new()
        .with_prompt("Audio Effects Menu")
        .items(&audio_effects_commands)
        .interact()
        .unwrap();

    for command in selected_commands 
    {
        match &audio_effects_commands[command]
        {
            Command::AudioEffectsMenu(audio_effect_command) => 
            {
                match audio_effect_command 
                {
                    AudioEffectsCommand::ModifyOscillator(parameter_text, parameters) |
                    AudioEffectsCommand::ModifyDelay(parameter_text, parameters) |
                    AudioEffectsCommand::ModifyFlanger(parameter_text, parameters) |
                    AudioEffectsCommand::ModifyCompressor(parameter_text, parameters) |
                    AudioEffectsCommand::ModifyVolume(parameter_text, parameters) => 
                    {

                        // Print the parameter name currently being modified
                        println!("{parameter_text} parameters:");

                        // Prompt user to modify parameters
                        for (parameter, value) in vec![parameters.0.clone(), parameters.1.clone(), parameters.2.clone()]
                        {
                            
                            // Let user input a value for the parameter then confirm
                            let new_value: u32 = Input::<u32>::new()
                                .with_prompt(&format!("{parameter} (current value: {value})"))
                                .interact()
                                .unwrap();

                        }

                        let confirm = Confirm::new()
                        .with_prompt(&format!("{parameter_text} parameters to {:?}?", parameters))
                        .interact()
                        .unwrap();

                        if confirm 
                        {

                            // Set the parameter to the new value
                            


                        }
                    }
                }
            }
            Command::MainMenu(_) => {},
        }
    }

}
