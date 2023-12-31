use clap::Args;

use lib_enigma::{EnigmaMachine, Letter, MachineState, Message, RotorId};

#[derive(Args)]
pub struct EncipherArgs {
    /// ID of reflector to use, eg `"B"`
    reflector_id: String,

    /// IDs of the rotors to use
    ///
    /// Each rotor should be specified in the format `id` or `id:start`, where
    /// `id` is the rotor ID (in roman numerals), and `start` is the starting
    /// position of said rotor. `start` defaults to 0
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    rotor_ids: Vec<String>,

    /// Sets of plugs to use in the plug board
    ///
    /// Each connection should specify two letters to swap, for example `AB`.
    #[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ')]
    plug_map: Vec<String>,
}

pub fn encipher_main(args: EncipherArgs) {
    // Parse the rotor options
    let rotors: Vec<(RotorId, Letter)> = args
        .rotor_ids
        .into_iter()
        .map(|r| match r.split_once(':') {
            None => (
                r.as_str()
                    .try_into()
                    .unwrap_or_else(|_| panic!("Invalid rotor ID {r:?}")),
                Letter::A,
            ),
            Some((id, start)) => {
                let parsed = start.chars().next().unwrap();
                (
                    id.try_into()
                        .unwrap_or_else(|_| panic!("Invalid rotor ID {r:?}")),
                    Letter::from_char(parsed).unwrap().0,
                )
            }
        })
        .collect();

    let (rotor_ids, rotor_starts): (Vec<_>, Vec<_>) = rotors.into_iter().unzip();

    // And also parse the plug maps
    let plugs: Vec<(Letter, Letter)> = args
        .plug_map
        .into_iter()
        .map(|c| {
            assert_eq!(c.len(), 2);
            (
                Letter::from_char(c.chars().next().unwrap()).unwrap().0,
                Letter::from_char(c.chars().nth(1).unwrap()).unwrap().0,
            )
        })
        .collect();

    // Now configure the machine
    let mut machine = EnigmaMachine::from(MachineState::new(
        plugs,
        rotor_ids,
        rotor_starts,
        args.reflector_id
            .as_str()
            .try_into()
            .expect("Invalid reflector ID"),
    ));

    for line in std::io::stdin().lines() {
        println!(
            "{}",
            machine.consume(&Message::from(line.unwrap())).to_string()
        );
    }
}
