pub mod hackword;
pub mod instruction;
pub mod io;
pub mod machine;

#[cfg(test)]
mod tests {
    use super::{io::read_instructions, machine::Machine, hackword::HackWord};



    #[test]
    fn add() {
        let instructions = read_instructions("resources/add.hack").unwrap();
        let mut machine = Machine::from_instructions(instructions);

        machine.run().unwrap();

        assert_eq!(machine.memory[0], HackWord(5))
    }

    #[test]
    fn max() {
        let instructions = read_instructions("resources/max.hack").unwrap();
        let mut machine = Machine::from_instructions(instructions);
        machine.memory[0] = HackWord(5);
        machine.memory[1] = HackWord(4);

        machine.run().unwrap();

        assert_eq!(machine.memory[2], HackWord(5))
    }
}
