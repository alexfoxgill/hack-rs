pub mod hackword;
pub mod instruction;
pub mod io;
pub mod machine;

#[cfg(test)]
mod tests {
    use super::{io::read_instructions, machine::Machine, hackword::HackWord};

    #[test]
    fn add() {
        let mut machine = Machine::new();
        let instructions = read_instructions("resources/add.hack").unwrap();
        machine.load_instructions(instructions);

        machine.run().unwrap();

        assert_eq!(machine.memory[0], HackWord(5))
    }
}
