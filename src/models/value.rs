use anyhow::{anyhow, Result};
use bytes::BytesMut;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    Array(Vec<Value>),
    Integer(i64),
    NullBulkString,
    Error(String),
}

impl Value {
    pub fn serialize(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        match self {
            Value::SimpleString(s) => buf.extend_from_slice(format!("+{}\r\n", s).as_bytes()),
            Value::BulkString(s) => {
                buf.extend_from_slice(format!("${}\r\n{}\r\n", s.len(), s).as_bytes())
            }
            Value::Array(values) => {
                buf.extend_from_slice(format!("*{}\r\n", values.len()).as_bytes());
                for value in values {
                    buf.extend_from_slice(&value.serialize());
                }
            }
            Value::Integer(i) => buf.extend_from_slice(format!(":{}\r\n", i).as_bytes()),
            Value::NullBulkString => buf.extend_from_slice("$-1\r\n".as_bytes()),
            Value::Error(e) => buf.extend_from_slice(format!("-{}\r\n", e).as_bytes()),
        }
        buf
    }

    pub fn deserialize(input: &str) -> Result<Self> {
        match input.chars().next() {
            Some('+') => Ok(Value::SimpleString(input[1..].trim().to_string())),
            Some('$') => {
                if input.starts_with("$-1") {
                    Ok(Value::NullBulkString)
                } else {
                    let parts: Vec<&str> = input.split("\r\n").collect();
                    if parts.len() >= 2 {
                        Ok(Value::BulkString(parts[1].to_string()))
                    } else {
                        Err(anyhow!("Invalid bulk string format"))
                    }
                }
            }
            Some('*') => {
                let mut values = Vec::new();
                let lines: Vec<&str> = input.lines().skip(1).collect();
                for line in lines {
                    values.push(Value::deserialize(line)?);
                }
                Ok(Value::Array(values))
            }
            Some(':') => {
                let num = input[1..].trim().parse::<i64>()?;
                Ok(Value::Integer(num))
            }
            Some('-') => Ok(Value::Error(input[1..].trim().to_string())),
            _ => Err(anyhow!("Unknown Redis value type")),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::SimpleString(s) | Value::BulkString(s) => s.clone(),
            Value::Array(values) => values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            Value::Integer(i) => i.to_string(),
            Value::NullBulkString => "(nil)".to_string(),
            Value::Error(e) => format!("Error: {}", e),
        }
    }
}
