pub mod parser {

    use regex::Regex;
    use std::process;
    use std::thread;
    use std::time::Duration;

    #[derive(Debug)]
    pub struct Machine {
        pub current_state: State,
        pub current_transition: Transition,
        pub states: Vec<State>,
        pub transitions: Vec<Transition>,
    }

    impl Machine {
        pub fn new(states: Vec<State>, transitions: Vec<Transition>) -> Machine {
            Machine {
                current_state: states
                    .iter()
                    .find(|state| state.state == StateKind::Start)
                    .unwrap()
                    .clone(),
                current_transition: transitions[0].clone(),
                states: states,
                transitions: transitions,
            }
        }

        pub fn get_current_state(&self) -> Result<&State, &'static str> {
            // Get the current state return error if not found
            Ok(&self.current_state)
        }

        pub fn print_transition(&self) -> Result<(), &'static str> {
            // Print the current transition
            return self.current_transition.print_transition();
        }

        pub fn get_current_transition(&self) -> &Transition {
            &self.current_transition
        }

        pub fn set_transition(&mut self, transition_name: &str) -> Result<(), &'static str> {
            match self.transitions.iter().find(|transition| {
                transition.action_name.to_lowercase() == transition_name.to_lowercase()
            }) {
                Some(transition) => {
                    self.current_transition = transition.clone();
                    return Ok(());
                }
                None => {
                    return Err("Transition Not Found");
                }
            }
        }

        pub fn set_state(&mut self) -> Result<(), &'static str> {
            // Set the current state to the state with the given name
            // Return error if not found

            match self.states.iter().find(|state| {
                state.state_name.to_lowercase()
                    == self
                        .current_transition
                        .target_state_name
                        .to_lowercase()
                        .as_str()
                        .to_lowercase()
            }) {
                Some(state) => {
                    self.current_state = state.clone();
                    return Ok(());
                }
                None => {
                    return Err("State Not Found");
                }
            }
        }

        pub fn execute_transition(&mut self, input: &str) -> Result<(), &'static str> {
            // Wildcard function
            let re_choose = Regex::new(r"choose").unwrap();
            let re_smash = Regex::new(r"smash").unwrap();

            if re_smash.captures(input.to_lowercase().as_str()).is_some() {
                eprint!("You Broke it!!!");
                thread::sleep(Duration::from_secs(5));
                process::exit(1)
            }

            if re_choose.captures(input.to_lowercase().as_str()).is_some() {
                match self.set_transition(input.to_lowercase().as_str()) {
                    Ok(_) => (),
                    Err(_e) => match self.set_transition("Choose *") {
                        Ok(_) => (),
                        Err(_e) => {
                            return Err("Transition Not Found");
                        }
                    },
                }
            }

            // Handle Incorrect keypad input
            let re = Regex::new(r"Enter \d{4}").unwrap();
            if re.is_match(&input) && input != "Enter 5729" {
                match self.set_transition("Enter *".to_lowercase().as_str()) {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            };

            match self.set_transition(input.to_lowercase().as_str()) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(e);
                }
            }

            match self.set_state() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(e);
                }
            }

            return Ok(());
        }

        pub fn execute_autoforwarding(&mut self) -> Result<(), &'static str> {
            // Execute the current transition
            // Loop Through AutoForwarding States
            while &self.current_state.get_state_kind() == &StateKind::AutoForwarding {
                //  Transition to the next state
                thread::sleep(Duration::from_millis(
                    self.get_current_state().unwrap().transition_length,
                ));

                self.current_transition = self
                    .transitions
                    .iter()
                    .find(|transition| {
                        transition.source_state_name.to_lowercase()
                            == self.current_state.state_name.to_lowercase()
                    })
                    .unwrap()
                    .clone();

                self.print_transition()?;

                self.current_state = self
                    .states
                    .iter()
                    .find(|state| {
                        state.state_name.to_lowercase()
                            == self.current_transition.target_state_name.to_lowercase()
                    })
                    .unwrap()
                    .clone();

                print!("{esc}c", esc = 27 as char);

                match &self.current_state.print_state() {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }

            return Ok(());
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Transition {
        pub source_state_name: String,
        pub action_name: String,
        pub target_state_name: String,
        pub transition_action: String,
    }

    //  A Transition is always in the same format:
    impl Transition {
        pub fn new(input: &str) -> Result<Transition, &'static str> {
            if input.matches('(').count() > 1 || input.matches('>').count() > 1 {
                let parts: Vec<_> = input.split(&['>', '(', ')', ':'][..]).collect();
                let split = input.split(':').collect::<Vec<&str>>();

                Ok(Transition {
                    source_state_name: parts[1].trim().to_string(),
                    action_name: parts[2].trim().to_string(),
                    target_state_name: parts[3].trim().to_string(),
                    transition_action: split[1].trim().to_string(),
                })
            } else {
                let parts: Vec<_> = input.split(&['>', '(', ')', ':'][..]).collect();

                Ok(Transition {
                    source_state_name: parts[1].trim().to_string(),
                    action_name: parts[2].trim().to_string(),
                    target_state_name: parts[3].trim().to_string(),
                    transition_action: parts[4].trim().to_string(),
                })
            }
        }

        pub fn print_transition(&self) -> Result<(), &'static str> {
            Ok(println!("{}", self.transition_action))
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum StateKind {
        Start,
        End,
        Normal,
        AutoForwarding,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Action {
        pub action_name: String,
        pub action_description: String,
    }

    impl Action {
        pub fn new(input: String) -> Action {
            if input.contains('[') {
                let parts: Vec<_> = input.split(&['[', ']'][..]).collect();
                Action {
                    action_name: parts[1].trim().to_string(),
                    action_description: parts[2].trim_end_matches('}').to_string(),
                }
            } else {
                let parts: Vec<_> = input.split(&['(', ')'][..]).collect();
                Action {
                    action_name: parts[1].trim().to_string(),
                    action_description: parts[2].trim_end_matches('}').to_string(),
                }
            }
        }
    }

    pub fn state_type(input: &str) -> StateKind {
        let char: char = input.split('@').collect::<Vec<&str>>()[1]
            .chars()
            .next()
            .unwrap();
        match char {
            '*' => StateKind::Start,
            '+' => StateKind::End,
            '!' => StateKind::AutoForwarding,
            _ => StateKind::Normal,
        }
    }

    pub fn state_description(input: &str) -> String {
        // Split on regex

        let re_brackets = Regex::new(r"\([a-zA-Z]+\)").unwrap();

        if re_brackets.find(input).is_some() {
            let parts = re_brackets.split(input).collect::<Vec<&str>>();
            return parts[0].to_string();
        } else {
            let parts = input.split(&['[', '}']).collect::<Vec<&str>>();
            return parts[0].to_string();
        }
    }

    pub fn create_actions(input: &str) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        for (_i, line) in input.lines().enumerate() {
            let re_brackets = Regex::new(r"\([a-zA-Z]+\)").unwrap();

            if line.contains('[') {
                actions.push(Action::new(line.to_string()));
            }
            if re_brackets.find(line).is_some() {
                let parts = re_brackets.split(line).collect::<Vec<&str>>();
                actions.push(Action::new(parts[0].to_string()));
            }
        }
        actions
    }

    pub fn create_transition_length(input: &str) -> u64 {
        // Check if input contains a digit
        if input.contains(|c: char| c.is_digit(10)) {
            return input
                .chars()
                .skip_while(|c| !c.is_digit(10))
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
        } else {
            return 0;
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct State {
        pub state: StateKind,
        pub state_name: String,
        pub state_description: String,
        pub transitions: Vec<Action>,
        pub transition_length: u64,
    }

    impl State {
        pub fn new(input: &str) -> Result<State, &'static str> {
            // if input[0] contains '@*' then it is a start state
            // if input[0] contains '@+' then it is an end state
            // if input[0] contains '@' then it is a normal state
            // if input[0] contains '@!' then it is an auto forwarding state

            let parts: Vec<_> = input.split('{').collect();

            let state = State {
                state: state_type(parts[0]),
                state_name: parts[0]
                    .chars()
                    .skip_while(|c| !c.is_alphanumeric())
                    .take_while(|c| c.is_alphabetic())
                    .collect::<String>(),
                transition_length: create_transition_length(parts[0]),
                state_description: state_description(&parts[1]).to_string(),
                transitions: create_actions(&parts[1]),
            };

            Ok(state)
        }

        pub fn get_state_kind(&self) -> StateKind {
            self.state
        }

        pub fn print_state(&self) -> Result<(), &'static str> {
            println!("{}", self.state_description);
            self.print_actions()?;

            Ok(())
        }

        pub fn print_actions(&self) -> Result<(), &'static str> {
            for action in &self.transitions {
                println!("[{}] {}", action.action_name, action.action_description);
            }
            Ok(())
        }
    }

    pub fn valid_string(input: &str) -> Result<bool, &'static str> {
        if input.matches("@*").count() > 1 {
            return Err("Too many Start states");
        } else if input.matches("@*").count() == 0 {
            return Err("No Start state found");
        } else if !input.matches("@+").count() > 0
            || !input.matches("@*").count() > 0
            || !input.matches("@!").count() > 0
        {
            Ok(true)
        } else {
            Err("Too many End states")
        }
    }

    pub fn parser(input: String) -> Result<Machine, &'static str> {
        if valid_string(&input)? {
            let mut states: Vec<State> = Vec::new();
            let mut transitions: Vec<Transition> = Vec::new();
            let mut start = 0;
            let mut end = 0;

            for (i, line) in input.lines().enumerate() {
                let line = line.trim();
                if line.starts_with("@") {
                    start = i;
                }
                if line.contains("}") && start != 0 {
                    end = i;
                }

                if start != 0 && end != 0 {
                    // Create a new str of the input
                    let next_line = end + 1;
                    let mut lines = vec![];

                    for i in start..next_line {
                        lines.push(input.lines().nth(i).unwrap().to_string());
                    }

                    // Turn lines in &str
                    let state_str = lines.join("\n");

                    match State::new(&state_str) {
                        Ok(state) => {
                            states.push(state);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                    start = 0;
                    end = 0;
                }

                if line.trim().starts_with(">") {
                    transitions.push(Transition::new(line.trim()).unwrap());
                }
            }

            Ok(Machine::new(states, transitions))
        } else {
            Err("Invalid input")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine_parser::parser::parser;
    use crate::machine_parser::parser::state_description;
    use crate::machine_parser::parser::state_type;
    use crate::machine_parser::parser::Machine;
    use crate::machine_parser::parser::State;
    use crate::machine_parser::parser::StateKind;
    use crate::machine_parser::parser::Transition;

    use regex::Regex;

    #[test]
    fn test_parser_rolling() {
        let input = r#""

        @*Peak{You're at the top of the hill. [slide] start sliding!}
        @!Rolling1000{It's a steep slope!}
        @!StillRolling1000{You can't stop!}
        @+Bottom{You're at the bottom of the hill}
        >Peak (slide) Rolling: You start sliding down the hill
        >Rolling (slide) StillRolling: You're still sliding down the hill
        >StillRolling (slide) Bottom: You reached the bottom!

        ""#
        .to_string();

        let machine = parser(input).unwrap();
    }

    #[test]
    fn test_parser_hotel() {
        let input = r#""

        

@*Lobby{
    You find yourself in a hotel lobby. There is nobody at the reception and you don't see any other guests. You can see people walking by on the street behind the glass outside.
   
    ┌──┬──┬──────────────────┬────┐
    │  └──┘                  │┼┼┼┼│
    │                    │   └────┴──┬──┬───────┐
    ├────────────────────┘           └──┘       │
    │                                           │
    │                                           │
    ├─┐                         ( ┌┐ )          │
    │ │                           ││            │
    ├─┘                         ( └┘ )          │
    │     ( ┌┐ )                         ( ┌┐ ) │
    │       ││                             ││   │
    │     ( └┘ )                         ( └┘ ) │
    │                   ┌──────┐                │
    └───────────────────┴──────┴────────────────┘
    [Access security panel]: There's a keypad next to the hotel entrance
    [Go to restrooms]: There's a doorway marked "Restrooms" leading to a corridor
   }
   
   >Lobby (Access security panel) SecurityPanel: You look at the keypad
   >Lobby (Go to restrooms) RestroomCorridor: You go through the doorway maked "Restrooms"
   
   @SecurityPanel{
    Maybe entering the right combination will open the door?
   
    ┌───────┐
    │ 1 2 3 │
    │ 4 5 6 │
    │ 7 8 9 │
    │   0   │
    └───────┘
   
    [Enter ****]: Enter four digits
    [Walk away]: Leave the security panel
   }
   
   >SecurityPanel (Enter 5729) Outside: You enter the code and the door opens!
   >SecurityPanel (Enter *) SecurityPanel: The code you entered was rejected. Nothing happens...
   >SecurityPanel (Walk away) Lobby: You walk away from the security panel.
   
   @+Outside{
    You're free! [The End]
   
                                   .-~ | ~-.
                                   |   |   |
                                   |  _:_  |                    .-:~--.._
                                 .-"~~ | ~~"-.                .~  |      |
                _.-~:.           |     |     |                |   |      |
               |    | `.         |     |     |                |   |      |
      _..--~:-.|    |  |         |     |     |                |   |      |
     |      |  ~.   |  |         |  __.:.__  |                |   |      |
     |      |   |   |  |       .-"~~   |   ~~"-.              |   |      |
     |      |   |  _|.--~:-.   |       |       |         .:~-.|   |      |
     |      A   | |      |  ~. |       |   _.-:~--._   .' |   |   |      |
     |      M   | |      |   | |       |  |   |     |  |  |   |   |      |
     |      C   | |      |   | |       |  |   |     |  |  |   |   |      |
     |      |   | |      |   | |       |  |   |     |  |  |   |   |      |
     |      9   | |      |   | |       |  |   |     |  |  |   |   |      |
     |      9   | |      |   | |       |  |   |     |  |  |   |   |      |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   }
   
   @RestroomCorridor{
    Not much here except an unecessarily large mirror on the far side of the corridor. There are four doors.
   
    ┌────────────────────────────────────────────────┐
    │            ┌──────────────────────┐            │
    │            │                      │            │
    │        /│  │    ┌────────────┐    │  │\        │
    │       / │  │    │            │    │  │ \       │
    │      │  │  │    │            │    │  │  │      │
    │   /│ │  │  │    │    ┌──┐    │    │  │  │ │\   │
    │  / │ │  │  │    │    │oo│    │    │  │  │ │ \  │
    │ │  │ │  │  │    │    └┬┬┘    │    │  │  │ │  │ │
    │ │  │ │  │  │    │    /││\    │    │  │  │ │  │ │
    │ │  │ │  │  │    │   / └┘ \   │    │  │  │ │  │ │
    │ │  │ │  │  │    │     /\     │    │  │  │ │  │ │
    │ │  │ │  │  │    │    /  \    │    │  │  │ │  │ │
    │ │  │ │  │  │    │   /    \   │    │  │  │ │  │ │
    │ │  │ │  │  │    └────────────┘    │  │  │ │  │ │
    │ │  │ │  │  │                      │  │  │ │  │ │
    │ │  │ │  │  └──────────────────────┘  │  │ │  │ │
    │ │  │ │  │ /                        \ │  │ │  │ │
    │ │  │ │  │/                          \│  │ │  │ │
    │ │  │ │  /                            \  │ │  │ │
    │ │  │ │ /                              \ │ │  │ │
    │ │  │ │/                                \│ │  │ │
    │ │  │ /                                  \ │  │ │
    │ │  │/                                    \│  │ │
    │ │  /                                      \  │ │
    │ │ /                                        \ │ │
    │ │/                                          \│ │
    │ /                                            \ │
    │/                                              \│
    │                                                │
    └────────────────────────────────────────────────┘
    [Go to male restrooms]: First door on the left
    [Go to female restrooms]: Second door on the left
    [Go to mixed restrooms]: First door on the right
    [Go to supply closet]: It says "Supplies" on the door
    [Go back]: Back to the lobby
   }
   
   >RestroomCorridor (Go to male restrooms) MaleRestrooms: You go to the male restrooms
   >RestroomCorridor (Go to female restrooms) FemaleRestrooms: You go to the female restrooms
   >RestroomCorridor (Go to mixed restrooms) MixedRestrooms: You go to the mixed restrooms
   >RestroomCorridor (Go to supply closet) RestroomCorridor: The door is locked
   >RestroomCorridor (Go back) Lobby: You go back to the lobby
   
   @MaleRestrooms{
    Nobody and nothing's here. You hear only the buzz of the fluorescent lights.
   
          (,.)   ,.   (,.)   ,.   (,.)   ,.
           ||    ||    ||    ||    ||    ||
           ||    ||    ||    ||    ||    ||
         ,.||..  ||  ,.||..  ||  ,.||..  ||
        //""""\\ || //""""\\ || //""""\\ ||
        ||    || || ||    || || ||    || ||
        ||    || || ||    || || ||    || ||
        ||____|| || ||____|| || ||____|| ||
        `.____.' || `.____.' || `.____.' ||
                 ||          ||          ||
                 ||          ||          ||
     ____________||__________||__________||___
   
    [Go back]: Back to the corridor
   }
   
   >MaleRestrooms (Go back) RestroomCorridor: You go back to the corridor
   
   @FemaleRestrooms{
    Nobody and nothing's here. One of the lights is flickering annoyingly.
    ___   _______________   ___   _______________   ___
       I I               I I   I I               I I
       I I               I I   I I               I I
       IHI               I I   IHI               I I
       I I               I I   I I               I I
       I I               I I   I I               I I
       I I               I I   I I               I I
       I I            O  I-I   I I            O  I-I
       I I               I I   I I               I I
       I I               I I   I I               I I
       I I               I I   I I               I I
       IHI               I I   IHI               I I
       I I               I I   I I               I I
    ___I I_______________I I___I I_______________I I___
     I                       I                       I
     I                       I                       I
     I                       I                       I
    _H_                     _H_                     _H_
   
    [Go back]: Back to the corridor
   }
   
   >FemaleRestrooms (Go back) RestroomCorridor: You go back to the corridor
   
   @MixedRestrooms{
    Nobody's here, but while looking around, you find a crumpled up piece of paper on the floor:
   
                                                                /────────────┐
                                  /─────────────────────────────             │
      ┌───────────────────────────                            /              │
      │                                                     /                │
      │       .----------.    _________                    /     .----.      │
      │      /          /    /         |        .-''-.         .   _   \     │
      │     /   ______.'    '-----.   .'      .' .-.  )       /  .' )   |   │
     │     /   /_               .'  .'       / .'  / /       |   (_.    /   │
     │    /      '''--.       .'  .'        (_/   / /         \     ,  /    │
     │   '___          `.   .'  .'               / /           `'-'/  /     │
     │       `'.         | '---'                / /        .-.    /  /      │
     │          )        |                     . '         \  '--'  /       │
     │  ......-'        /                     / /    _.-')  '-....-'       │
    │   \          _..'`                    .' '  _.'.-''                  │
    │    '------'''            /           /  /.-'_.'                      │
    │                     \   /           /    _.'         \               │
    │                      /   \         ( _.-'             /              │
    │                     /     \                          /               │
    │                     /                                     /──────────┘
    │                    /        /────────────────────────────
    └─────────────────────────────
    [Go back]: Back to the corridor
   }
   
   >MixedRestrooms (Go back) RestroomCorridor: You go back to the corridor
   
   

        ""#
        .to_string();

        let machine = parser(input).unwrap();
    }

    #[test]
    fn test_parser_doggo() {
        let input = r#""
        @*Neutral{

            ▄▀▄▀▀▀▀▄▀▄
            █        ▀▄      ▄
           █  ▀  ▀     ▀▄▄  █ █
           █ ▄ █▀ ▄       ▀▀  █
           █  ▀▀▀▀            █
           █                  █
           █                  █
            █  ▄▄  ▄▄▄▄  ▄▄  █
            █ ▄▀█ ▄▀  █ ▄▀█ ▄▀
             ▀   ▀     ▀   ▀
         [Pet]: He's a good boy.
         [Order talk]: "Who's a good boy?"
         [Order ?]: What other tricks might Doggo know?
         [Abandon]: You wouldn't! How could you?!
       }
       
       > Neutral (Pet) Petting: Doggo is excited!
       > Neutral (Order sit) Neutral: Doggo sits down and looks at you in anticipation!
       > Neutral (Order talk) Neutral: Doggo tells a joke. Nobody laughs. Doggo is happy!
       > Neutral (Order turn) Neutral: Doggo spins around, trying to catch its tail!
       # custom error message for invalid "Order" actions:
       > Neutral (Order *) Neutral: Doggo tilts its head in confusion... 
       > Neutral (Order sleep) Sleeping: Doggo plays dead...
       > Neutral (Order backflip) BackflipA: Watch this...
       > Neutral (Abandon) GameOver: Doggo is sad to see you go :(
       
       @!BackflipA1500{
       
            ▄▀▄▀▀▀▀▄▀▄
            █        ▀▄      ▄
           █  ▀  ▀     ▀▄▄  █ █
           █ ▄ █▀ ▄       ▀▀  █
           █  ▀▀▀▀            █
           █                  █
           █                  █
            █ ▄▀█ ▄▀▀▀█ ▄▀█ ▄▀
             ▀   ▀     ▀   ▀
       }
       
       @!BackflipB250{
       
             ▄▄▄▄▀▀▀▀▀▀▀▀▀▀▄▄▄
            ▀▄        ▄▀  ▄  ▄▀
             ▄██      █▀█     █
            ▀▄        ▀▄  ▀   █
              ▀█             ▀▄
             ▄▄█           ▄▀▀
            ▀▄           ▄▀
             ▄██        ▄▀
            ▀▄          ▀▄▄
              ▀▀▀▄▄▄▄▄▄▄▄▄▄▀
       }
       
       @!BackflipC250{
       
              ▄   ▄     ▄   ▄
            ▄▀ █▄▀ █  ▄▀ █▄▀ █
            █  ▀▀  ▀▀▀▀  ▀▀  █
           █                  █
           █                  █
           █            ▄▄▄▄  █
           █  ▄▄       ▀ ▄█ ▀ █
           █ █  ▀▀▄     ▄  ▄  █
            ▀      ▀▄        █
                    ▀▄▀▄▄▄▄▀▄▀
       }
       
       @!BackflipD250{
       
               ▄▀▀▀▀▀▀▀▀▀▀▄▄▄
                ▀▀▄          ▀▄
                 ▄▀        ██▀
                ▄▀           ▀▄
             ▄▄▀           █▀▀
            ▀▄             █▄
            █   ▄  ▀▄        ▀▄
            █     █▀█      ██▀
            ▄▀  ▀  ▄▀        ▀▄
             ▀▀▀▄▄▄▄▄▄▄▄▄▄▀▀▀▀
       }
       
       > BackflipA (continue) BackflipB: !!!
       > BackflipB (continue) BackflipC: !!!
       > BackflipC (continue) BackflipD: !!!
       > BackflipD (continue) Neutral: Tadaaa!!!
       
       @Petting{
       
            ▄▀▄▀▀▀▀▄▀▄
            █        ▀▄      ▄
           █  ^  ^     ▀▄▄  █ █
           █   o          ▀▀  █
           █                  █
           █                  █
           █                  █
            █  ▄▄  ▄▄▄▄  ▄▄  █
            █ ▄▀█ ▄▀  █ ▄▀█ ▄▀
             ▀   ▀     ▀   ▀
       
         [Pet]: He's still a good boy.
         [Stop petting]: Even if it's never enough.
       }
       
       > Petting (Pet) Petting: Doggo is still excited!
       > Petting (Stop petting) Neutral: Doggo excited anyway!
       
       @Sleeping{
       
       
       
            ▄▀▄▀▀▀▀▄▀▄
            █        ▀▄      ▄
           █  v  v     ▀▄▄  █ █
           █   -          ▀▀  █
           █                  █
           █                  █
            █▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█
       
       
         [Say "treat!"]: Let's play!
       }
       
       > Sleeping (Say "treat!") Neutral: Doggo heard "treat" and woke up
       > Sleeping (Say *) Sleeping: Doggo still playing dead
       
       @+GameOver{
       
            ▄▀▄▀▀▀▀▄▀▄  ..-- woof woof!!!
            █        ▀▄      ▄
           █  ▀  ▀     ▀▄▄  █ █
           █   █▀         ▀▀  █
           █  ▀▀▀▀            █
           █                  █
           █                  █
            █  ▄▄  ▄▄▄▄  ▄▄  █
            █ ▄▀█ ▄▀  █ ▄▀█ ▄▀
             ▀   ▀     ▀   ▀
       }
       
        
        ""#
        .to_string();

        let result = parser(input);
    }

    #[test]
    fn test_parser_car() {
        let input = r#""
        @*Park{
            The transmission is in "park".
            (Drive) Put the transmission into "drive"
            (Leave) Leave the car (quit)
          }
          
          @+Leave{
            Bye bye!
          }
          
          # Note that this state is not mentioned in any state description
          # but it's still valid and it would be reached if someone typed
          # 'Crash' while in the driving state, because there exists a
          # corresponding transition below.
          @+Crash{
            Get well soon!
          }
          
          @Drive{
            The transmission is in "drive".
            (Roll) Start driving
            (Park) Put the transmission into "park"
            (Honk angrily) Toot the horn!
          }
          
          @Driving{
            You are driving.
            (Stop) Stop the car.
            (Honk angrily) Toot the horn!
          }
          
          > Park (Drive) Drive: You select "drive" (D)
          > Park (Leave) Leave: You leave the car: "Good bye!"
          > Drive (Roll) Driving: You start driving
          > Drive (Park) Park: You select "park" (P)
          >   Driving   (  Crash  )   Crash  : You crash the car! >.<
          >Driving(Stop)Drive: You stop the car
          > Drive (Honk angrily) Drive: Toot!
          > Driving (  Honk angrily   ) Driving: Toot!
          
        
        ""#
        .to_string();

        let result = parser(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().states.len(), 5);
    }

    #[test]
    fn test_parser_vending() {
        let input = r#""
        # For stylistic reasons, we use octothorpes here to
        # indicate comments, but it's not necessary.
        
        Anything outside valid definitions is ignored!
        
        # The machine file consists of state and transition
        # definitions. These definitions don't need to appear
        # in any particular order.
        
        # A state definition starts with an @ symbol.
        # For exactly one state, the @ must be followed by
        # a * symbol, indicating that this is the starting state.
        # For one or more states, the @ must be followed by
        # a + symbol, indicating that this is an end state.
        # The state definition itself is just an arbitrary character
        # sequence enclosed in { and }. Anything between these two
        # characters (including newlines) shall be printed on
        # screen when the state is entered. You can assume
        # that { and } will not appear inside the state text.
        
        
        @*Ready{ You are standing in front of the vending machine
         [Pay] Put some money into the machine
         [Exit] Leave the machine}
        
        
        
        
        > Ready (Pay) Select: You put some money into the machine
        # Transition syntax doesn't care about whitespace before the colon:
        >Ready  (Exit)Exit   : You're not thirsty right now
        
        
        @Select{ The machine is ready to accept your choice
         [Cancel] Hit the reset button
         [Choose beverage] Select a beverage }
        > Select (Cancel) Ready: You cancel the transaction
        
        # Note that parameters themselves can contain whitespace
        > Select (Choose beverage) Dispense: You select a beverage
        @Dispense{ Your choice has been dropped into the chute
         [Take] Take the beverage from the chute }
        > Dispense (Take) Ready: You remove the beverage from the chute
        
        # Reaching this state will end the program
        @+Exit{
         Good bye! } Note that this sentence here is also just another comment
        because it is outside of any state or transition definition.
        
        ""#
        .to_string();

        let result = parser(input);
    }

    #[test]
    fn test_execution() {
        let input = r#"""Enter 1234""#;

        let states = vec![State {
            state: StateKind::Start,
            state_name: "SecurityPanel".to_string(),
            state_description: "You are standing at the start of the game".to_string(),
            transitions: vec![],
            transition_length: 0,
        }];

        let transitions = vec![Transition {
            source_state_name: "SecurityPanel".to_string(),
            action_name: "Enter *".to_string(),
            target_state_name: "SecurityPanel".to_string(),
            transition_action: "The code you entered was rejected. Nothing happens...".to_string(),
        }];

        let mut test_machine = Machine::new(states, transitions);

        test_machine.execute_transition(input);
    }

    #[test]
    fn test_state_description_brackets() {
        let input = r#""
        
        The transmission is in drive.
        (Roll) Start driving
        (Park) Put the transmission into "park"
        (Honk angrily) Toot the horn!
      }""#;

        let state_description = state_description(input);

        assert_eq!(state_description, "The transmission is in drive. ");
    }

    #[test]
    fn test_state_description_square_brackets() {
        let input = r#""

        ▄▀▄▀▀▀▀▄▀▄
        █        ▀▄      ▄
       █  ▀  ▀     ▀▄▄  █ █
       █ ▄ █▀ ▄       ▀▀  █
       █  ▀▀▀▀            █
       █                  █
       █                  █
        █  ▄▄  ▄▄▄▄  ▄▄  █
        █ ▄▀█ ▄▀  █ ▄▀█ ▄▀
         ▀   ▀     ▀   ▀
     [Pet]: He's a good boy.
     [Order talk]: "Who's a good boy?"
     [Order ?]: What other tricks might Doggo know?
     [Abandon]: You wouldn't! How could you?!""#;

        let state_description = state_description(input);

        assert_eq!(
            state_description,
            "

        ▄▀▄▀▀▀▀▄▀▄
        █        ▀▄      ▄
       █  ▀  ▀     ▀▄▄  █ █
       █ ▄ █▀ ▄       ▀▀  █
       █  ▀▀▀▀            █
       █                  █
       █                  █
        █  ▄▄  ▄▄▄▄  ▄▄  █
        █ ▄▀█ ▄▀  █ ▄▀█ ▄▀
         ▀   ▀     ▀   ▀
      "
        );
    }

    #[test]
    fn test_state_new() {
        let state = State::new(
            r#""@!Drive1000{
            The transmission is in drive.
            (Roll) Start driving
            (Park) Put the transmission into "park"
            (Honk angrily) Toot the horn!
          }""#,
        )
        .unwrap();
        assert_eq!(state.state, StateKind::AutoForwarding);
        assert_eq!(state.state_name, "Drive");
        assert_eq!(state.state_description, "The transmission is in drive.");
        assert_eq!(state.transitions.len(), 3);
        assert_eq!(state.transition_length, 1000);
    }

    #[test]
    fn test_trim_alphebetic() {
        let input = "@!Drive12345";
        let output = input
            .chars()
            .skip_while(|c| !c.is_digit(10))
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        assert_eq!(output, 12345);
    }

    #[test]
    fn test_state_type_autoforwarding() {
        let input = "@!Drive12345";
        let output = state_type(input);

        assert_eq!(output, StateKind::AutoForwarding)
    }
    #[test]
    fn test_state_type_start() {
        let input = "@*Drive12345";
        let output = state_type(input);

        assert_eq!(output, StateKind::Start)
    }
    #[test]
    fn test_state_type_end() {
        let input = "@+Drive12345";
        let output = state_type(input);

        assert_eq!(output, StateKind::End)
    }

    #[test]
    fn test_regex_circular_brackets() {
        let re = Regex::new(r"^\([a-zA-Z]+\)$").unwrap();
        assert!(re.is_match(r"(Pay)"));
    }

    #[test]
    fn test_regex_square_brackets() {
        let re = Regex::new(r"^\[[a-zA-Z]+\]$").unwrap();
        assert!(re.is_match(r"[Pet]"));
    }

    #[test]
    fn test_brackets_regex_find() {
        let text = "Hello (Pay) World";
        let mat = Regex::new(r"\([a-zA-Z]+\)").unwrap().find(text);

        assert!(!mat.is_none());

        // assert_eq!(mat.start(), 6);
        // assert_eq!(mat.end(), 11);
    }

    #[test]
    fn test_square_brackets_regex_find() {
        let text = "Hello [Pay] World";
        let mat = Regex::new(r"\[{1}\w\]{1}").unwrap().find(text);

        assert!(!mat.is_none());

        // assert_eq!(mat.start(), 6);
        // assert_eq!(mat.end(), 11);
    }

    #[test]
    fn test_brackets() {
        let re =
            Regex::new(r"^(?:\({1}|\[{1}){1}[a-zA-Z](?:\){1}|\]{1}){1}(?:\w+|\s+)*\n{1}$").unwrap();
        assert!(re.is_match(r"(Pay) Put some money into the machine"));
        assert!(re.is_match(r"[Pay] Put some money into the machine"));
    }

    #[test]
    fn test_regex_state_name() {
        let re = Regex::new(r"^@{1}(?:\+{1}|\*{1}|!{1}){1}[a-zA-Z]+\{{1}$").unwrap();
        assert!(re.is_match(r"@*Drive{"));
        assert!(re.is_match(r"@+Drive{"));
        assert!(re.is_match(r"@!Drive{"));
    }

    #[test]
    fn test_split_string_with_regex() {
        let text = "Hello (Pay) world!";
        let re = Regex::new(r"\([a-zA-Z]+\)").unwrap();
        let mut iter = re.split(text);
        assert_eq!(iter.next(), Some("Hello "));
        assert_eq!(iter.next(), Some(" world!"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_regex_number() {
        let text = "Enter 1234";
        let re = Regex::new(r"Enter \d{4}").unwrap();

        assert!(re.is_match(text));
    }

    #[test]
    fn test_choose_number() {
        let text = "Let's try choose Pepsi";
        let re = Regex::new(r"choose").unwrap();

        let mat = re.captures(text).unwrap();
        assert_eq!(mat.get(0).unwrap().as_str(), "choose");
    }

    #[test]
    fn test_smash_regex() {
        let text = "smash with hand";
        let re = Regex::new(r"smash with").unwrap();

        let mat = re.captures(text).unwrap();
        assert_eq!(mat.get(0).unwrap().as_str(), "smash with");
    }
}
