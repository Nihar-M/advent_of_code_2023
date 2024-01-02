pub const DAY_STR: &str = "day_19";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::collections::HashMap;

    use itertools::Itertools;
    use nom::{
        bytes::complete::tag,
        character::complete::{self, alpha1, anychar, newline},
        combinator::opt,
        error::Error,
        multi::{many1, separated_list1},
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
    };

    struct Rule {
        condition: Box<dyn Fn(Part) -> bool>,
        destination: String,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Part {
        pub x: u32,
        pub m: u32,
        pub a: u32,
        pub s: u32,
    }

    fn parse_workflow(input: &str) -> IResult<&str, (String, Vec<Rule>)> {
        let (input, name) = alpha1(input)?;

        let (input, rules) = delimited(
            tag("{"),
            separated_list1(
                tag(","),
                tuple((
                    opt(terminated(
                        tuple((anychar, anychar, complete::u32)),
                        tag(":"),
                    )),
                    alpha1,
                )),
            ),
            tag("}"),
        )(input)?;

        let name = name.to_string();

        let rules = rules
            .into_iter()
            .map(|(cond, dest)| {
                let condition: Box<dyn Fn(Part) -> bool> =
                    if let Some((field, comparison, value)) = cond {
                        match comparison {
                            '>' => match field {
                                'x' => Box::new(move |part: Part| part.x > value),
                                'm' => Box::new(move |part: Part| part.m > value),
                                'a' => Box::new(move |part: Part| part.a > value),
                                's' => Box::new(move |part: Part| part.s > value),
                                _ => panic!(),
                            },
                            '<' => match field {
                                'x' => Box::new(move |part: Part| part.x < value),
                                'm' => Box::new(move |part: Part| part.m < value),
                                'a' => Box::new(move |part: Part| part.a < value),
                                's' => Box::new(move |part: Part| part.s < value),
                                _ => panic!(),
                            },
                            _ => panic!(),
                        }
                    } else {
                        Box::new(|_part: Part| true)
                    };

                let destination = dest.to_string();

                Rule {
                    condition,
                    destination,
                }
            })
            .collect_vec();

        Ok((input, (name, rules)))
    }

    fn parse_part(input: &str) -> IResult<&str, Part> {
        let (input, _) = tag("{")(input)?;
        let (input, x) = preceded(tag("x="), complete::u32)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, m) = preceded(tag("m="), complete::u32)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, a) = preceded(tag("a="), complete::u32)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, s) = preceded(tag("s="), complete::u32)(input)?;
        let (input, _) = tag("}")(input)?;

        Ok((input, Part { x, m, a, s }))
    }

    pub fn solution(input: String) -> u32 {
        let (input, workflows) = separated_list1(newline, parse_workflow)(&input).unwrap();
        let (input, _) = many1(newline::<&str, Error<&str>>)(input).unwrap();
        let (_input, parts) = separated_list1(newline, parse_part)(input).unwrap();

        let mut workflow_map = HashMap::new();

        workflows.into_iter().for_each(|(name, rules)| {
            workflow_map.insert(name, rules);
        });

        // dbg!(&workflow_map.keys());

        // dbg!(&parts);

        let accepted = parts
            .into_iter()
            .filter(|part| {
                let mut workflow = "in".to_string();

                while &workflow != "A" && &workflow != "R" {
                    // dbg!(&workflow);
                    let rules = workflow_map.get(&workflow).unwrap();
                    workflow = rules
                        .iter()
                        .find_map(|rule| {
                            if (rule.condition)(*part) {
                                Some(rule.destination.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap();
                }

                workflow.as_str() == "A"
                // todo!()
            })
            .collect_vec();

        // dbg!(&accepted);

        accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            19114
        );
    }
}

mod part_2 {

    use super::part_1::Part;

    use std::collections::HashMap;

    use itertools::Itertools;
    use nom::{
        bytes::complete::tag,
        character::complete::{self, alpha1, anychar, newline},
        combinator::opt,
        error::Error,
        multi::{many1, separated_list1},
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Comparison {
        Greater,
        Lesser,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Field {
        X,
        M,
        A,
        S,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Condition {
        field: Field,
        comp: Comparison,
        value: u32,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Rule {
        condition: Option<Condition>,
        destination: String,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct PartRange {
        lower: Part,
        upper: Part,
    }

    impl PartRange {
        fn split_range(self, cond: Condition) -> (Option<PartRange>, Option<PartRange>) {
            match cond.field {
                Field::X => match cond.comp {
                    Comparison::Greater => {
                        let lower = self.lower.x;
                        let upper = self.upper.x;

                        if lower > cond.value {
                            (Some(self), None)
                        } else if upper < cond.value {
                            (None, Some(self))
                        } else {
                            (
                                Some(PartRange {
                                    lower: Part {
                                        x: cond.value + 1,
                                        ..self.lower
                                    },
                                    upper: self.upper,
                                }),
                                Some(PartRange {
                                    lower: self.lower,
                                    upper: Part {
                                        x: cond.value,
                                        ..self.upper
                                    },
                                }),
                            )
                        }
                    }
                    Comparison::Lesser => {
                        let lower = self.lower.x;
                        let upper = self.upper.x;

                        if upper < cond.value {
                            (Some(self), None)
                        } else if lower > cond.value {
                            (None, Some(self))
                        } else {
                            (
                                Some(PartRange {
                                    lower: self.lower,
                                    upper: Part {
                                        x: cond.value - 1,
                                        ..self.upper
                                    },
                                }),
                                Some(PartRange {
                                    lower: Part {
                                        x: cond.value,
                                        ..self.lower
                                    },
                                    upper: self.upper,
                                }),
                            )
                        }
                    }
                },
                Field::M => match cond.comp {
                    Comparison::Greater => {
                        let lower = self.lower.m;
                        let upper = self.upper.m;

                        if lower > cond.value {
                            (Some(self), None)
                        } else if upper < cond.value {
                            (None, Some(self))
                        } else {
                            (
                                Some(PartRange {
                                    lower: Part {
                                        m: cond.value + 1,
                                        ..self.lower
                                    },
                                    upper: self.upper,
                                }),
                                Some(PartRange {
                                    lower: self.lower,
                                    upper: Part {
                                        m: cond.value,
                                        ..self.upper
                                    },
                                }),
                            )
                        }
                    }
                    Comparison::Lesser => {
                        let lower = self.lower.m;
                        let upper = self.upper.m;

                        if upper < cond.value {
                            (Some(self), None)
                        } else if lower > cond.value {
                            (None, Some(self))
                        } else {
                            (
                                Some(PartRange {
                                    lower: self.lower,
                                    upper: Part {
                                        m: cond.value - 1,
                                        ..self.upper
                                    },
                                }),
                                Some(PartRange {
                                    lower: Part {
                                        m: cond.value,
                                        ..self.lower
                                    },
                                    upper: self.upper,
                                }),
                            )
                        }
                    }
                },
                Field::A => match cond.comp {
                    Comparison::Greater => {
                        let lower = self.lower.a;
                        let upper = self.upper.a;

                        if lower > cond.value {
                            (Some(self), None)
                        } else if upper < cond.value {
                            (None, Some(self))
                        } else {
                            (
                                Some(PartRange {
                                    lower: Part {
                                        a: cond.value + 1,
                                        ..self.lower
                                    },
                                    upper: self.upper,
                                }),
                                Some(PartRange {
                                    lower: self.lower,
                                    upper: Part {
                                        a: cond.value,
                                        ..self.upper
                                    },
                                }),
                            )
                        }
                    }
                    Comparison::Lesser => {
                        let lower = self.lower.a;
                        let upper = self.upper.a;

                        if upper < cond.value {
                            (Some(self), None)
                        } else if lower > cond.value {
                            (None, Some(self))
                        } else {
                            (
                                Some(PartRange {
                                    lower: self.lower,
                                    upper: Part {
                                        a: cond.value - 1,
                                        ..self.upper
                                    },
                                }),
                                Some(PartRange {
                                    lower: Part {
                                        a: cond.value,
                                        ..self.lower
                                    },
                                    upper: self.upper,
                                }),
                            )
                        }
                    }
                },
                Field::S => match cond.comp {
                    Comparison::Greater => {
                        let lower = self.lower.s;
                        let upper = self.upper.s;

                        if lower > cond.value {
                            (Some(self), None)
                        } else if upper < cond.value {
                            (None, Some(self))
                        } else {
                            (
                                Some(PartRange {
                                    lower: Part {
                                        s: cond.value + 1,
                                        ..self.lower
                                    },
                                    upper: self.upper,
                                }),
                                Some(PartRange {
                                    lower: self.lower,
                                    upper: Part {
                                        s: cond.value,
                                        ..self.upper
                                    },
                                }),
                            )
                        }
                    }
                    Comparison::Lesser => {
                        let lower = self.lower.s;
                        let upper = self.upper.s;

                        if upper < cond.value {
                            (Some(self), None)
                        } else if lower > cond.value {
                            (None, Some(self))
                        } else {
                            (
                                Some(PartRange {
                                    lower: self.lower,
                                    upper: Part {
                                        s: cond.value - 1,
                                        ..self.upper
                                    },
                                }),
                                Some(PartRange {
                                    lower: Part {
                                        s: cond.value,
                                        ..self.lower
                                    },
                                    upper: self.upper,
                                }),
                            )
                        }
                    }
                },
            }
        }

        fn get_area(&self) -> u64 {
            let dx = 1 + (self.upper.x - self.lower.x) as u64;
            let dm = 1 + (self.upper.m - self.lower.m) as u64;
            let da = 1 + (self.upper.a - self.lower.a) as u64;
            let ds = 1 + (self.upper.s - self.lower.s) as u64;
            dx * dm * da * ds
        }
    }

    fn parse_workflow(input: &str) -> IResult<&str, (String, Vec<Rule>)> {
        let (input, name) = alpha1(input)?;

        let (input, rules) = delimited(
            tag("{"),
            separated_list1(
                tag(","),
                tuple((
                    opt(terminated(
                        tuple((anychar, anychar, complete::u32)),
                        tag(":"),
                    )),
                    alpha1,
                )),
            ),
            tag("}"),
        )(input)?;

        let name = name.to_string();

        let rules = rules
            .into_iter()
            .map(|(cond, dest)| {
                let condition = if let Some((field, comp, value)) = cond {
                    Some(Condition {
                        field: match field {
                            'x' => Field::X,
                            'm' => Field::M,
                            'a' => Field::A,
                            's' => Field::S,
                            _ => panic!(),
                        },
                        comp: match comp {
                            '>' => Comparison::Greater,
                            '<' => Comparison::Lesser,
                            _ => panic!(),
                        },
                        value,
                    })
                } else {
                    None
                };

                let destination = dest.to_string();

                Rule {
                    condition,
                    destination,
                }
            })
            .collect_vec();

        Ok((input, (name, rules)))
    }

    fn parse_part(input: &str) -> IResult<&str, Part> {
        let (input, _) = tag("{")(input)?;
        let (input, x) = preceded(tag("x="), complete::u32)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, m) = preceded(tag("m="), complete::u32)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, a) = preceded(tag("a="), complete::u32)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, s) = preceded(tag("s="), complete::u32)(input)?;
        let (input, _) = tag("}")(input)?;

        Ok((input, Part { x, m, a, s }))
    }

    pub fn solution(input: String) -> u64 {
        let (input, workflows) = separated_list1(newline, parse_workflow)(&input).unwrap();
        let (input, _) = many1(newline::<&str, Error<&str>>)(input).unwrap();
        let (_input, _parts) = separated_list1(newline, parse_part)(input).unwrap();

        let mut workflow_map = HashMap::new();

        workflows.into_iter().for_each(|(name, rules)| {
            workflow_map.insert(name, rules);
        });

        // dbg!(&workflow_map);

        let workflow = "in".to_string();

        let mut part_ranges = vec![(
            PartRange {
                lower: Part {
                    x: 1,
                    m: 1,
                    a: 1,
                    s: 1,
                },
                upper: Part {
                    x: 4000,
                    m: 4000,
                    a: 4000,
                    s: 4000,
                },
            },
            workflow.clone(),
        )];

        let mut accepted_prs = vec![];

        while let Some((part_range, name)) = part_ranges.pop() {
            // dbg!(&name);

            if &name == "A" {
                accepted_prs.push(part_range);
                continue;
            } else if &name == "R" {
                continue;
            }

            let rules = workflow_map.get(&name).unwrap();
            // dbg!(rules);

            let mut remaining_prs = vec![part_range];

            for rule in rules {
                remaining_prs = remaining_prs
                    .into_iter()
                    .filter_map(|pr| {
                        if let Some(cond) = rule.condition {
                            let (acc, rej) = pr.split_range(cond);

                            if let Some(acc) = acc {
                                part_ranges.push((acc, rule.destination.clone()));
                            }
                            rej
                        } else {
                            part_ranges.push((pr, rule.destination.clone()));
                            None
                        }
                    })
                    .collect_vec();
                // dbg!(&remaining_prs);
            }

            // dbg!(&part_ranges);

            // todo!();
        }

        accepted_prs.iter().map(|pr| pr.get_area()).sum()

        // dbg!(part_ranges);
        // dbg!(accepted_prs);

        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            167409079868000
        );
    }
}
