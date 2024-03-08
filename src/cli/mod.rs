
use dialoguer::{Select, MultiSelect};

pub enum Command 
{
    UseGUI,
    ModifyIODevice,
    ModifyAudioEffects,
    Exit
}

pub struct CommandOptions 
{
    pub commands: Vec<(Command, String)>,
}

impl CommandOptions 
{

    // Constructor
    pub fn new() -> Self 
    {
        Self 
        {
            commands: vec![
                (Command::UseGUI, String::from("Use Graphical User Interface (GUI)")),
                (Command::ModifyIODevice, String::from("Modify I/O Device settings")),
                (Command::ModifyAudioEffects, String::from("Modify Audio Effects")),
                (Command::Exit, String::from("Exit")),
            ],
        }
    }

    // Function to execute selected command
    pub fn execute_command(&self, command: &Command) -> bool 
    {
        match command 
        {
            Command::UseGUI => 
            {
                true
            },
            Command::ModifyAudioEffects => 
            {
                true
            },
            Command::ModifyIODevice => 
            {
                true
            },
            Command::Exit => 
            {
                false
            },
        }
    }

}

pub fn initialize_cli() 
{

    // Create new CommandOptions instance
    let command_options = CommandOptions::new();

    // Get command descriptions
    let descriptions: Vec<&str> = command_options.commands.iter().map(|(_, desc)| desc.as_str()).collect();

    loop 
    {

        // Prompt user to select a command
        let selection = Select::new()
            .with_prompt("Please select a command:")
            .items(&descriptions)
            .default(0)
            .interact()
            .unwrap();

        // Get selected command
        let (command, _) = &command_options.commands[selection];

        // Execute selected command
        if !command_options.execute_command(command) { break }

    }

}