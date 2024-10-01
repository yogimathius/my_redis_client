use std::collections::HashMap;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct CommandInfo {
    num_args: usize,
    arg_types: Vec<&'static str>,
}

impl CommandInfo {
    pub fn validate_args(&self, args: Vec<String>) -> Result<()> {
        println!("Validating args: {:?}", args);
        if args.len() != self.num_args {
            println!("Expected {} arguments, got {}", self.num_args, args.len());
            return Err(anyhow!(
                "Expected {} arguments, got {}",
                self.num_args,
                args.len()
            ));
        }
        for (arg, expected_type) in args.iter().zip(&self.arg_types) {
            println!(
                "Validating arg: {} with expected type: {}",
                arg, expected_type
            );
            match *expected_type {
                "String" => {} // All arguments are strings, so no validation needed
                "Integer" => {
                    if arg.parse::<i64>().is_err() {
                        return Err(anyhow!("Expected integer, got {}", arg));
                    }
                }
                _ => return Err(anyhow!("Unknown argument type: {}", expected_type)),
            }
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref COMMANDS: HashMap<&'static str, CommandInfo> = {
        let mut m = HashMap::new();
        m.insert("quit", CommandInfo { num_args: 0, arg_types: vec![] });
        m.insert("ECHO", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("PING", CommandInfo { num_args: 0, arg_types: vec![] }); // or 1 argument
        m.insert("PSYNC", CommandInfo { num_args: 2, arg_types: vec!["Integer", "Integer"] });
        m.insert("FULLRESYNC", CommandInfo { num_args: 2, arg_types: vec!["Integer", "Integer"] });
        m.insert("FLUSHALL", CommandInfo { num_args: 0, arg_types: vec![] });
        m.insert("INFO", CommandInfo { num_args: 0, arg_types: vec![] });
        m.insert("SET", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("GET", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("KEY", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("TYPE", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("DEL", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("UNLINK", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("EXPIRE", CommandInfo { num_args: 2, arg_types: vec!["String", "Integer"] });
        m.insert("RENAME", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("LLEN", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("LREM", CommandInfo { num_args: 3, arg_types: vec!["String", "Integer", "String"] });
        m.insert("LINDEX", CommandInfo { num_args: 2, arg_types: vec!["String", "Integer"] });
        m.insert("LPOP", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("RPOP", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("LPUSH", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("RPUSH", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("LSET", CommandInfo { num_args: 3, arg_types: vec!["String", "Integer", "String"] });
        m.insert("HGET", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("HEXISTS", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("HDEL", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("HGETALL", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("HKEYS", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("HLEN", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("HMSET", CommandInfo { num_args: 3, arg_types: vec!["String", "String", "String"] });
        m.insert("HSET", CommandInfo { num_args: 3, arg_types: vec!["String", "String", "String"] });
        m.insert("HVALS", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m
    };
}
