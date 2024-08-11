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
        if args.len() != self.num_args {
            return Err(anyhow!(
                "Expected {} arguments, got {}",
                self.num_args,
                args.len()
            ));
        }
        for (arg, expected_type) in args.iter().zip(&self.arg_types) {
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
        m.insert("echo", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("ping", CommandInfo { num_args: 0, arg_types: vec![] }); // or 1 argument
        m.insert("flushall", CommandInfo { num_args: 0, arg_types: vec![] });
        m.insert("info", CommandInfo { num_args: 0, arg_types: vec![] });
        m.insert("set", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("get", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("key", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("type", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("del", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("unlink", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("expire", CommandInfo { num_args: 2, arg_types: vec!["String", "Integer"] });
        m.insert("rename", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("llen", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("lrem", CommandInfo { num_args: 3, arg_types: vec!["String", "Integer", "String"] });
        m.insert("lindex", CommandInfo { num_args: 2, arg_types: vec!["String", "Integer"] });
        m.insert("lpop", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("rpop", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("lpush", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("rpush", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("lset", CommandInfo { num_args: 3, arg_types: vec!["String", "Integer", "String"] });
        m.insert("hget", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("hexists", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("hdel", CommandInfo { num_args: 2, arg_types: vec!["String", "String"] });
        m.insert("hgetall", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("hkeys", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("hlen", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m.insert("hmset", CommandInfo { num_args: 3, arg_types: vec!["String", "String", "String"] });
        m.insert("hset", CommandInfo { num_args: 3, arg_types: vec!["String", "String", "String"] });
        m.insert("hvals", CommandInfo { num_args: 1, arg_types: vec!["String"] });
        m
    };
}
