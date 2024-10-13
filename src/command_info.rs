use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;

use crate::{log, models::value::Value};

#[derive(Debug)]
pub struct CommandInfo {
    pub name: &'static str,
    pub num_args: usize,
    pub arg_types: Vec<&'static str>,
}

lazy_static! {
    static ref MANY_ARG_COMMANDS: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("SET");
        m.insert("HSET");
        m
    };
}

lazy_static! {
    pub static ref DEPRECATED_COMMANDS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("HMSET", "HSET");
        m
    };
}

impl CommandInfo {
    pub fn new(command: &str, args: Vec<String>) -> Result<(String, Vec<Value>)> {
        if let Some(new_command) = DEPRECATED_COMMANDS.get(command) {
            return Err(anyhow!(
                "Command '{}' is deprecated. Use '{}' instead.",
                command,
                new_command
            ));
        }

        let command_info = COMMANDS
            .get(command)
            .ok_or_else(|| anyhow!("Invalid command: {}", command))?;

        log!("Validating command: {:?}", command);
        let transformed_args = command_info.validate_and_transform_args(args)?;

        Ok((command.to_string(), transformed_args))
    }

    pub fn validate_and_transform_args(&self, args: Vec<String>) -> Result<Vec<Value>> {
        log!("Validating args: {:?}", args);
        if args.len() != self.num_args && !MANY_ARG_COMMANDS.contains(&self.name) {
            log!("Expected {} arguments, got {}", self.num_args, args.len());
            return Err(anyhow!(
                "Expected {} arguments, got {}",
                self.num_args,
                args.len()
            ));
        }

        args.iter()
            .zip(&self.arg_types)
            .map(|(arg, expected_type)| {
                log!(
                    "Validating arg: {} with expected type: {}",
                    arg,
                    expected_type
                );
                match *expected_type {
                    "String" => Ok(Value::BulkString(arg.clone())),
                    "Integer" => arg
                        .parse::<i64>()
                        .map(Value::Integer)
                        .map_err(|_| anyhow!("Expected integer, got {}", arg)),
                    _ => Err(anyhow!("Unknown argument type: {}", expected_type)),
                }
            })
            .collect()
    }
}

lazy_static! {
    pub static ref COMMANDS: HashMap<&'static str, CommandInfo> = {
        let mut m = HashMap::new();
        m.insert("quit", CommandInfo { name: "quit", num_args: 0, arg_types: vec![] });
        m.insert("ECHO", CommandInfo { name: "ECHO", num_args: 1, arg_types: vec!["String"] });
        m.insert("PING", CommandInfo { name: "PING", num_args: 0, arg_types: vec![] }); // or 1 argument
        m.insert("PSYNC", CommandInfo { name: "PSYNC", num_args: 2, arg_types: vec!["Integer", "Integer"] });
        m.insert("FULLRESYNC", CommandInfo { name: "FULLRESYNC", num_args: 2, arg_types: vec!["Integer", "Integer"] });
        m.insert("FLUSHALL", CommandInfo { name: "FLUSHALL", num_args: 0, arg_types: vec![] });
        m.insert("INFO", CommandInfo { name: "INFO", num_args: 0, arg_types: vec![] });
        m.insert("SET", CommandInfo { name: "SET", num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("GET", CommandInfo { name: "GET", num_args: 1, arg_types: vec!["String"] });
        m.insert("KEYS", CommandInfo { name: "KEYS", num_args: 1, arg_types: vec!["String"] });
        m.insert("TYPE", CommandInfo { name: "TYPE", num_args: 1, arg_types: vec!["String"] });
        m.insert("DEL", CommandInfo { name: "DEL", num_args: 1, arg_types: vec!["String"] });
        m.insert("UNLINK", CommandInfo { name: "UNLINK", num_args: 1, arg_types: vec!["String"] });
        m.insert("EXPIRE", CommandInfo { name: "EXPIRE", num_args: 2, arg_types: vec!["String", "Integer"] });
        m.insert("RENAME", CommandInfo {name:"RENAME",num_args:2,arg_types:vec!["String","String"] });
        m.insert("LLEN", CommandInfo { name: "LLEN", num_args: 1, arg_types: vec!["String"] });
        m.insert("LREM", CommandInfo { name: "LREM", num_args: 3, arg_types: vec!["String", "Integer", "String"] });
        m.insert("LINDEX", CommandInfo { name: "LINDEX", num_args: 2, arg_types: vec!["String", "Integer"] });
        m.insert("LPOP", CommandInfo { name: "LPOP", num_args: 1, arg_types: vec!["String"] });
        m.insert("RPOP", CommandInfo { name: "RPOP", num_args: 1, arg_types: vec!["String"] });
        m.insert("LPUSH", CommandInfo { name: "LPUSH", num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("RPUSH", CommandInfo { name: "RPUSH", num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("LSET", CommandInfo { name: "LSET", num_args: 3, arg_types: vec!["String", "Integer", "String"] });
        m.insert("HGET", CommandInfo { name: "HGET", num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("HEXISTS", CommandInfo { name: "HEXISTS", num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("HDEL", CommandInfo { name: "HDEL", num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("HGETALL", CommandInfo { name: "HGETALL", num_args: 1, arg_types: vec!["String"] });
        m.insert("HKEYS", CommandInfo { name: "HKEYS", num_args: 1, arg_types: vec!["String"] });
        m.insert("HLEN", CommandInfo { name: "HLEN", num_args: 1, arg_types: vec!["String"] });
        m.insert("HMSET", CommandInfo { name: "HMSET", num_args: 0, arg_types: vec!["String", "String", "String"] });
        m.insert("HSET", CommandInfo { name: "HSET", num_args: 0, arg_types: vec!["String", "String", "String"] });
        m.insert("HVALS", CommandInfo { name: "HVALS", num_args: 1, arg_types: vec!["String"] });
        m.insert("REPLCONF", CommandInfo { name: "REPLCONF", num_args: 2, arg_types: vec!["String"] });
        m
    };
}
