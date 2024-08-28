use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub struct Rule(u32);

#[derive(Deserialize)]
struct RuleHolder {
    rule: Rule,
}

impl Rule {
    pub fn survive(&self, count: u8) -> bool {
        (self.0 >> count) & 1 == 1
    }

    pub fn born(&self, count: u8) -> bool {
        (self.0 >> (count + 8)) & 1 == 1
    }

    pub fn from_str(rulestring: &str) -> Self {
        let rule_holder: RuleHolder =
            toml::from_str(&format!("rule = \"{}\"", rulestring)).unwrap();

        rule_holder.rule
    }

    pub fn serialize(&self) -> String {
        let mut survive_str = String::new();
        let mut born_str = String::new();

        for i in 0..8 {
            if self.survive(i) {
                survive_str.push_str(&i.to_string());
            }
        }

        for i in 0..8 {
            if self.born(i) {
                born_str.push_str(&i.to_string());
            }
        }

        format!("{}/{}", survive_str, born_str)
    }
}

impl Serialize for Rule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.serialize())
    }
}

impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RuleVisitor;

        impl<'de> Visitor<'de> for RuleVisitor {
            type Value = Rule;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a cellular automata rulestring, like '23/3'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Rule, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = value.split('/').collect();
                if parts.len() != 2 {
                    return Err(de::Error::custom("Invalid rulestring format"));
                }

                let survive_part = parts[0];
                let born_part = parts[1];

                let mut rule_value: u32 = 0;

                for ch in survive_part.chars() {
                    if let Some(digit) = ch.to_digit(10) {
                        if digit < 8 {
                            rule_value |= 1 << digit;
                        } else {
                            return Err(de::Error::custom("Survival count out of range"));
                        }
                    } else {
                        return Err(de::Error::custom("Invalid character in survival string"));
                    }
                }

                for ch in born_part.chars() {
                    if let Some(digit) = ch.to_digit(10) {
                        if digit < 8 {
                            rule_value |= 1 << (digit + 8);
                        } else {
                            return Err(de::Error::custom("Birth count out of range"));
                        }
                    } else {
                        return Err(de::Error::custom("Invalid character in birth string"));
                    }
                }

                Ok(Rule(rule_value))
            }
        }

        deserializer.deserialize_str(RuleVisitor)
    }
}
