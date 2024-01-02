pub const DAY_STR: &str = "day_20";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    use std::{
        collections::{HashMap, VecDeque},
        fmt::Debug,
    };

    use nom::{
        bytes::complete::{tag, take_until1},
        character::complete::{alpha1, newline},
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    pub fn parse_modules(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
        separated_list1(
            newline,
            separated_pair(
                take_until1(" "),
                tag(" -> "),
                separated_list1(tag(", "), alpha1),
            ),
        )(input)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Pulse {
        Low,
        High,
    }

    #[derive(Debug, Clone)]
    pub struct Base<'a> {
        pub name: &'a str,
        pub outputs: Vec<&'a str>,
    }

    #[derive(Debug, Clone)]
    pub enum Kind<'a> {
        FlipFlop { on: bool },
        Conjunction { mem: HashMap<&'a str, Pulse> },
    }

    impl<'a> Kind<'a> {
        pub fn process_pulse(&mut self, pulse: Pulse, source: &'a str) -> Option<Pulse> {
            match self {
                Kind::FlipFlop { on } => match pulse {
                    Pulse::Low => {
                        if *on {
                            *on = false;
                            Some(Pulse::Low)
                        } else {
                            *on = true;
                            Some(Pulse::High)
                        }
                    }
                    Pulse::High => None,
                },
                Kind::Conjunction { mem } => {
                    mem.insert(source, pulse);

                    if mem.values().all(|p| p == &Pulse::High) {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    }
                }
            }
        }
    }

    #[test]
    fn test_process_pulse() {
        let mut flipflop = Module {
            base: Base {
                name: "hi",
                outputs: vec!["bye", "see ya"],
            },
            kind: Kind::FlipFlop { on: false },
        };

        dbg!(&flipflop);
        assert_eq!(flipflop.kind.process_pulse(Pulse::High, "hi"), None);
        dbg!(&flipflop);
        assert_eq!(
            flipflop.kind.process_pulse(Pulse::Low, "hi"),
            Some(Pulse::High)
        );
        dbg!(&flipflop);
        assert_eq!(
            flipflop.kind.process_pulse(Pulse::Low, "hi"),
            Some(Pulse::Low)
        );
        dbg!(&flipflop);

        let mut conjunction = Module {
            base: Base {
                name: "hi",
                outputs: vec!["bye", "see ya"],
            },
            kind: Kind::Conjunction {
                mem: {
                    let mut map = HashMap::new();
                    map.insert("a", Pulse::Low);
                    map.insert("b", Pulse::Low);
                    map
                },
            },
        };

        dbg!(&conjunction);
        assert_eq!(
            conjunction.kind.process_pulse(Pulse::High, "a"),
            Some(Pulse::High)
        );
        dbg!(&conjunction);
        assert_eq!(
            conjunction.kind.process_pulse(Pulse::High, "b"),
            Some(Pulse::Low)
        );
        dbg!(&conjunction);
        assert_eq!(
            conjunction.kind.process_pulse(Pulse::Low, "a"),
            Some(Pulse::High)
        );
        dbg!(&conjunction);
    }

    #[derive(Debug, Clone)]
    pub struct Module<'a> {
        pub base: Base<'a>,
        pub kind: Kind<'a>,
    }

    #[derive(Clone)]
    pub struct Signal<'a> {
        pub source: &'a str,
        pub destination: &'a str,
        pub pulse: Pulse,
    }

    impl<'a> Debug for Signal<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "{} -{:?}-> {}",
                self.source, self.pulse, self.destination
            ))
        }
        //
    }

    pub fn solution(input: String) -> usize {
        let (_input, parsed) = parse_modules(&input).unwrap();

        // only a single broadcaster module
        let mut broadcaster = vec![];

        let mut computer = HashMap::new();

        parsed.into_iter().for_each(|(name, outputs)| {
            if name == "broadcaster" {
                broadcaster = outputs;
            } else {
                let (kind, name) = name.split_at(1);

                computer.insert(
                    name,
                    Module {
                        base: Base {
                            name,
                            outputs: outputs.clone(),
                        },
                        kind: match kind {
                            "&" => Kind::Conjunction {
                                mem: HashMap::new(), // has to be initialized later
                            },
                            "%" => Kind::FlipFlop { on: false },
                            other => panic!("Unexpected Module Kind {other}"),
                        },
                    },
                );
            }
        });

        let mut computer: HashMap<&str, Module<'_>> =
            HashMap::from_iter(computer.clone().into_iter().map(|(name, mut module)| {
                match &mut module.kind {
                    Kind::FlipFlop { on: _ } => (),
                    Kind::Conjunction { mem } => {
                        for (x_name, x_module) in &computer {
                            if x_module.base.outputs.contains(&name) {
                                mem.insert(x_name, Pulse::Low);
                            }
                        }
                    }
                }
                (name, module)
            }));

        // dbg!(&computer);

        // dbg!(&broadcaster);

        let mut num_low_pulses = 0;
        let mut num_high_pulses = 0;

        for _ in 0..1000 {
            let mut signals = broadcaster
                .iter()
                .map(|&name| Signal {
                    source: "broadcaster",
                    destination: name,
                    pulse: Pulse::Low,
                })
                .collect::<VecDeque<_>>();

            num_low_pulses += 1; // for button -Low-> broadcaster

            // dbg!(&signals);

            let mut processed_signals = vec![];

            while let Some(signal) = signals.pop_front() {
                // dbg!(&signals);
                processed_signals.push(signal.clone());

                match signal.pulse {
                    Pulse::Low => num_low_pulses += 1,
                    Pulse::High => num_high_pulses += 1,
                }

                let dest = computer.get_mut(signal.destination);

                if dest.is_none() {
                    continue;
                }
                let dest = dest.unwrap();

                if let Some(output_pulse) = dest.kind.process_pulse(signal.pulse, signal.source) {
                    dest.base.outputs.iter().for_each(|&dest_name| {
                        signals.push_back(Signal {
                            source: dest.base.name,
                            destination: dest_name,
                            pulse: output_pulse,
                        })
                    })
                }
            }

            // dbg!(&computer);
            // dbg!(processed_signals);
        }

        // dbg!(num_high_pulses);
        // dbg!(num_low_pulses);

        num_high_pulses * num_low_pulses
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            32000000
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample2.txt", super::DAY_STR))
                    .unwrap()
            ),
            11687500
        );
    }
}

mod part_2 {

    use std::collections::{HashMap, VecDeque};

    use itertools::Itertools;

    use super::part_1::*;

    pub fn solution(input: String) -> usize {
        let (_input, parsed) = parse_modules(&input).unwrap();

        // only a single broadcaster module
        let mut broadcaster = vec![];

        let mut computer = HashMap::new();

        parsed.into_iter().for_each(|(name, outputs)| {
            if name == "broadcaster" {
                broadcaster = outputs;
            } else {
                let (kind, name) = name.split_at(1);

                computer.insert(
                    name,
                    Module {
                        base: Base {
                            name,
                            outputs: outputs.clone(),
                        },
                        kind: match kind {
                            "&" => Kind::Conjunction {
                                mem: HashMap::new(), // has to be initialized later
                            },
                            "%" => Kind::FlipFlop { on: false },
                            other => panic!("Unexpected Module Kind {other}"),
                        },
                    },
                );
            }
        });

        let mut computer: HashMap<&str, Module<'_>> =
            HashMap::from_iter(computer.clone().into_iter().map(|(name, mut module)| {
                match &mut module.kind {
                    Kind::FlipFlop { on: _ } => (),
                    Kind::Conjunction { mem } => {
                        for (x_name, x_module) in &computer {
                            if x_module.base.outputs.contains(&name) {
                                mem.insert(x_name, Pulse::Low);
                            }
                        }
                    }
                }
                (name, module)
            }));

        // dbg!(&computer);

        // dbg!(&broadcaster);

        let (_name, special_mod) = &computer
            .iter()
            .find(|(_name, module)| module.base.outputs.contains(&"rx"))
            .unwrap();

        // dbg!(&special_mod);

        let key_modules = match &special_mod.kind {
            Kind::FlipFlop { on: _ } => panic!(),
            Kind::Conjunction { mem } => mem.keys().copied().collect_vec(),
        };

        let mut key_modules: HashMap<&str, usize> =
            HashMap::from_iter(key_modules.iter().map(|&name| (name, 0)));

        // dbg!(&key_modules);

        // todo!();

        let mut button_presses: usize = 0;
        'outer: loop {
            let mut signals = broadcaster
                .iter()
                .map(|&name| Signal {
                    source: "broadcaster",
                    destination: name,
                    pulse: Pulse::Low,
                })
                .collect::<VecDeque<_>>();

            button_presses += 1;

            let mut processed_signals = vec![];

            while let Some(signal) = signals.pop_front() {
                // dbg!(&signals);
                processed_signals.push(signal.clone());

                if let Some(key_mod) = key_modules.get_mut(signal.destination) {
                    if signal.pulse == Pulse::Low {
                        *key_mod = button_presses;
                        if key_modules.values().all(|x| *x != 0) {
                            break 'outer;
                        }
                    }
                }

                let dest = computer.get_mut(signal.destination);

                if dest.is_none() {
                    continue;
                }
                let dest = dest.unwrap();

                if let Some(output_pulse) = dest.kind.process_pulse(signal.pulse, signal.source) {
                    dest.base.outputs.iter().for_each(|&dest_name| {
                        signals.push_back(Signal {
                            source: dest.base.name,
                            destination: dest_name,
                            pulse: output_pulse,
                        })
                    })
                }
            }

            // dbg!(processed_signals);
            if button_presses > 10000 {
                break 'outer;
            }

            // dbg!(&computer);
        }

        // dbg!(&key_modules);

        // this assumes the following
        // - that each key_module creates a LOW(or HIGH?) pulse on at cycle length
        //   equal to the first time it created a LOW(or HIGH?) pulse
        //   and that this cycle length is always the same
        // - that each cycle length is relatively prime
        //   so we can just multiply them together to get the LCM
        // - that the "rx" register is activated by a conjunction
        //   module that has inputs of these "key_modules"
        key_modules.values().product()
    }

    #[test]
    fn run_on_real_input() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/input.txt", super::DAY_STR))
                    .unwrap()
            ),
            217317393039529
        );
    }
}
