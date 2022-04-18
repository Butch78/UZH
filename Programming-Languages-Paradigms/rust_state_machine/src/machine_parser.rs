pub mod parser {

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

        pub fn execute_transition(&mut self, input: &str) -> Result<(), &'static str> {
            // If input is not in the current transition's input set, return error
            if !&self
                .current_state
                .transitions
                .iter()
                .any(|transition| transition.action_name.to_lowercase() == input.to_lowercase())
            {
                return Err("Invalid Input!");
            } else {
                self.current_transition = self
                    .transitions
                    .iter()
                    .find(|transition| {
                        transition.action_name.to_lowercase() == input.to_lowercase()
                    })
                    .unwrap()
                    .clone();
                self.current_state = self
                    .states
                    .iter()
                    .find(|state| state.state_name == self.current_transition.target_state_name)
                    .unwrap()
                    .clone();

                Ok(())
            }
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

    #[derive(Debug, PartialEq, Clone)]
    pub enum StateKind {
        Start,
        End,
        Normal,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Action {
        pub action_name: String,
        pub action_description: String,
    }

    impl Action {
        pub fn new(input: String) -> Action {
            if input.contains('(') {
                let parts: Vec<_> = input.split(&['(', ')'][..]).collect();
                Action {
                    action_name: parts[1].trim().to_string(),
                    action_description: parts[2].trim_end_matches('}').to_string(),
                }
            } else {
                let parts: Vec<_> = input.split(&['[', ']'][..]).collect();
                Action {
                    action_name: parts[1].trim().to_string(),
                    action_description: parts[2].trim_end_matches('}').to_string(),
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct State {
        pub state: StateKind,
        pub state_name: String,
        pub state_description: String,
        pub transitions: Vec<Action>,
    }

    impl State {
        pub fn new(input: Vec<&str>) -> Result<State, &'static str> {
            // if input[0] contains '@*' then it is a start state
            // if input[0] contains '@+' then it is an end state
            // if input[0] contains '@' then it is a normal state

            let mut transitions = Vec::new();

            let mut _state_description = String::new();

            // Split state input by '\n'
            let state_input_parts: Vec<_> = input[1].split(&['\n'][..]).collect();
            if state_input_parts.len() == 1 {
                return Err("No State transitions found");
            } else if !state_input_parts[1].contains('[') || !state_input_parts[1].contains('[') {
                let split = state_input_parts[1].split('}').collect::<Vec<&str>>();
                _state_description = split[0].trim().to_string();
            } else {
                let split = state_input_parts[0].split('}').collect::<Vec<&str>>();
                _state_description = split[0].trim().to_string();
            }

            for line in state_input_parts {
                if line.contains("[")
                    || line.contains("]")
                    || line.contains("(")
                    || line.contains(")")
                {
                    transitions.push(Action::new(line.to_string()));
                }
            }

            if input[0].contains("@*") {
                Ok(State {
                    state: StateKind::Start,
                    state_name: input[0].trim().trim_start_matches(&['@', '*']).to_string(),
                    state_description: _state_description,
                    transitions: transitions,
                })
            } else if input[0].contains("@+") {
                Ok(State {
                    state: StateKind::End,
                    state_name: input[0].trim().trim_start_matches(&['@', '+']).to_string(),
                    state_description: _state_description,
                    transitions: transitions,
                })
            } else {
                Ok(State {
                    state: StateKind::Normal,
                    state_name: input[0].trim().trim_start_matches('@').to_string(),
                    state_description: _state_description,
                    transitions: transitions,
                })
            }
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

    pub fn create_state(input: &str, start: usize, end: usize) -> Result<State, &'static str> {
        let mut lines: Vec<String> = Vec::new();

        let next_line = end + 1;

        for i in start..next_line {
            lines.push(input.lines().nth(i).unwrap().to_string());
        }

        let one_string = lines.join("\n");
        let split_string: Vec<&str> = one_string.split("{").collect();

        return State::new(split_string);
    }

    pub fn create_states(input: &str) -> Result<Vec<State>, &'static str> {
        // start with "@"" and end at "}"

        let mut states: Vec<State> = Vec::new();
        let mut start = 0;
        let mut end = 0;

        // Enumarate lines
        for (i, line) in input.lines().enumerate() {
            let line = line.trim();
            if line.starts_with("@") {
                start = i;
            }
            if line.contains("}") && start != 0 {
                end = i;
            }

            if start != 0 && end != 0 {
                states.push(create_state(input, start, end)?);
                start = 0;
                end = 0;
            }
        }

        match states.len() {
            0 => Err("No states found"),
            _ => Ok(states),
        }
    }

    pub fn create_transition(input: &str) -> Result<Transition, &'static str> {
        let result = Transition::new(input);
        match result {
            Ok(transition) => Ok(transition),
            Err(_) => Err("Transition not found"),
        }
    }

    pub fn create_transitions(input: &str) -> Result<Vec<Transition>, &'static str> {
        let mut transitions: Vec<Transition> = Vec::new();

        if input.matches('>').count() < 1 {
            return Err("No transitions found");
        } else {
            for line in input.lines() {
                if line.trim().starts_with(">") {
                    let transition = create_transition(line.trim())?;
                    transitions.push(transition);
                }
            }
        }

        Ok(transitions)
    }

    pub fn parser(input: String) -> Result<Machine, &'static str> {
        if input.matches("@*").count() > 1 {
            return Err("Too many Start states");
        } else if input.matches("@*").count() == 0 {
            return Err("No Start state found");
        } else if !input.matches("@+").count() > 0 || !input.matches("@*").count() > 0 {
            let states = create_states(&input)?;
            let transitions = create_transitions(&input)?;
            Ok(Machine::new(states, transitions))
        } else {
            Err("Too many End states")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine_parser::parser::create_states;
    use crate::machine_parser::parser::State;
    use crate::machine_parser::parser::StateKind;

    #[test]
    fn create_state_vending_test() {
        let string_vec = vec![
            "@*Ready{ You are standing in front of the vending machine",
            "[Pay] Put some money into the machine",
            "[Exit] Leave the machine}",
        ];

        let state = State::new(string_vec);

        let unwraped_state = state.unwrap();

        assert_eq!(unwraped_state.state, StateKind::Start);
        assert_eq!(unwraped_state.state_name, "Ready");
        assert_eq!(
            unwraped_state.state_description,
            "You are standing in front of the vending machine"
        );
        // assert_eq!(state.transitions[0], "[Pay] Put some money into the machine");
        // assert_eq!(state.transitions[1], "[Exit] Leave the machine");
        assert_eq!(unwraped_state.transitions.len(), 2);
    }

    #[test]
    fn create_state_car_test() {
        let string: String = String::from(
            r#""
        @Drive{
            The transmission is in "drive".
            (Roll) Start driving
            (Park) Put the transmission into "park"
            (Honk angrily) Toot the horn!
          }
          ""#,
        );

        let states = create_states(&string);

        println!("{:?}", states);
    }

    #[test]
    fn create_states_vending_test() {
        let string: String = String::from(
            r#""
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
        because it is outside of any state or transition definition.""#,
        );

        let _states = create_states(&string);
    }

    #[test]
    fn create_states__car_test() {
        let string: String = String::from(
            r#""
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
          > Park (Leave) Leave: You leave the car "Good bye!"
          > Drive (Roll) Driving: You start driving
          > Drive (Park) Park: You select "park" (P)
          >   Driving   (  Crash  )   Crash  : You crash the car! >.<
          >Driving(Stop)Drive: You stop the car
          > Drive (Honk angrily) Drive: Toot!
          > Driving (  Honk angrily   ) Driving: Toot!""#,
        );

        let result = create_states(&string);

        let _state = State::new(vec![
            "@*Ready{ You are standing in front of the vending machine",
            "[Pay] Put some money into the machine",
            "[Exit] Leave the machine}",
        ]);

        assert_eq!(result.unwrap()[0].state, StateKind::Start);
    }

    #[test]
    fn create_transition_test() {
        let string = "> Ready (Pay) Select: You put some money into the machine";

        let transition = crate::machine_parser::parser::Transition::new(&string);

        assert_eq!(&transition.unwrap().source_state_name, "Ready");
    }

    #[test]
    fn create_transitions_test() {
        let string = "> Ready (Pay) Select: You put some money into the machine";

        let _transition = crate::machine_parser::parser::Transition::new(string);

        let result = crate::machine_parser::parser::create_transitions(string);

        assert_eq!(result.unwrap()[0].source_state_name, "Ready");
    }

    #[test]
    fn create_car_transitions_test() {
        let string = String::from(r#""> Drive (Park) Park: You select "park" (P)""#);

        let _transition = crate::machine_parser::parser::Transition::new(&string);

        let result = crate::machine_parser::parser::create_transitions(&string);

        assert_eq!(result.unwrap()[0].source_state_name, "Drive");
    }

    #[test]
    fn create_transitions_test_car() {
        let string: String = String::from(
            r#""
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
          > Park (Leave) Leave: You leave the car "Good bye!"
          > Drive (Roll) Driving: You start driving
          > Drive (Park) Park: You select "park" (P)
          >   Driving   (  Crash  )   Crash  : You crash the car! >.<
          >Driving(Stop)Drive: You stop the car
          > Drive (Honk angrily) Drive: Toot!
          > Driving (  Honk angrily   ) Driving: Toot!""#,
        );

        let result = crate::machine_parser::parser::create_transitions(string.as_str());

        assert_eq!(result.unwrap().len(), 8);
    }

    #[test]
    fn create_transition_test_vending() {
        let string: String = String::from(
            r#""
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
        because it is outside of any state or transition definition.""#,
        );

        let transition = crate::machine_parser::parser::create_transitions(string.as_str());

        assert_eq!(transition.unwrap().len(), 5);
    }

    #[test]
    fn create_car_machine_test() {
        let string: String = String::from(
            r#""@*Park{
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
          > Park (Leave) Leave: You leave the car "Good bye!"
          > Drive (Roll) Driving: You start driving
          > Drive (Park) Park: You select "park" (P)
          >   Driving   (  Crash  )   Crash  : You crash the car! >.<
          >Driving(Stop)Drive: You stop the car
          > Drive (Honk angrily) Drive: Toot!
          > Driving (  Honk angrily   ) Driving: Toot!""#,
        );

        let result = crate::machine_parser::parser::parser(string);

        println!("{:?}", result);
    }

    #[test]
    fn create_vending_machine_test() {
        let string: String = String::from(
            r#""
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
        because it is outside of any state or transition definition.""#,
        );

        let _result = crate::machine_parser::parser::parser(string);
    }

    #[test]
    fn parser_no_entry() {
        let string = String::from(
            r#""
        # This file is missing both start (*) and end (+) states

        @Start{Initial state. Type 'end' to quit}
        @End{Bye bye}
        
        > Start (end) End: end""#,
        );

        let result = crate::machine_parser::parser::parser(string);

        // Print Error
        println!("{:?}", result);
        // Check it is an error
        assert!(result.is_err());
    }
}
