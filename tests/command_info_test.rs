#[cfg(test)]
mod tests {
    use my_redis_client::command_info::COMMANDS;

    #[test]
    fn test_validate_args_valid_set() {
        let command_info = COMMANDS.get("SET").unwrap();
        let args = vec!["key".to_string(), "value".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_set() {
        let command_info = COMMANDS.get("SET").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_unknown_command() {
        let command_info = COMMANDS.get("unknown");
        assert!(command_info.is_none(), "Expected command_info to be None");
    }
    #[test]
    fn test_validate_args_valid_ping_with_argument() {
        let command_info = COMMANDS.get("ping").unwrap();
        let args = vec!["hello".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_ping_without_argument() {
        let command_info = COMMANDS.get("ping").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_ping() {
        let command_info = COMMANDS.get("ping").unwrap();
        let args = vec!["123".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_flushall() {
        let command_info = COMMANDS.get("flushall").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_flushall() {
        let command_info = COMMANDS.get("flushall").unwrap();
        let args = vec!["extra_arg".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_info() {
        let command_info = COMMANDS.get("info").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_info() {
        let command_info = COMMANDS.get("info").unwrap();
        let args = vec!["extra_arg".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_quit() {
        let command_info = COMMANDS.get("quit").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_quit() {
        let command_info = COMMANDS.get("quit").unwrap();
        let args = vec!["extra_arg".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_echo() {
        let command_info = COMMANDS.get("echo").unwrap();
        let args = vec!["hello".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_echo() {
        let command_info = COMMANDS.get("echo").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_get() {
        let command_info = COMMANDS.get("GET").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_get() {
        let command_info = COMMANDS.get("GET").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_key() {
        let command_info = COMMANDS.get("key").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_key() {
        let command_info = COMMANDS.get("key").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_type() {
        let command_info = COMMANDS.get("type").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_type() {
        let command_info = COMMANDS.get("type").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_del() {
        let command_info = COMMANDS.get("del").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_del() {
        let command_info = COMMANDS.get("del").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_unlink() {
        let command_info = COMMANDS.get("unlink").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_unlink() {
        let command_info = COMMANDS.get("unlink").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_expire() {
        let command_info = COMMANDS.get("expire").unwrap();
        let args = vec!["key".to_string(), "10".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_expire() {
        let command_info = COMMANDS.get("expire").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_expire() {
        let command_info = COMMANDS.get("expire").unwrap();
        let args = vec!["key".to_string(), "not_an_integer".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_rename() {
        let command_info = COMMANDS.get("rename").unwrap();
        let args = vec!["old_key".to_string(), "new_key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_rename() {
        let command_info = COMMANDS.get("rename").unwrap();
        let args = vec!["old_key".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_llen() {
        let command_info = COMMANDS.get("llen").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_llen() {
        let command_info = COMMANDS.get("llen").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lrem() {
        let command_info = COMMANDS.get("lrem").unwrap();
        let args = vec!["key".to_string(), "0".to_string(), "value".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lrem() {
        let command_info = COMMANDS.get("lrem").unwrap();
        let args = vec!["key".to_string(), "0".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_lrem() {
        let command_info = COMMANDS.get("lrem").unwrap();
        let args = vec![
            "key".to_string(),
            "not_an_integer".to_string(),
            "value".to_string(),
        ];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lindex() {
        let command_info = COMMANDS.get("lindex").unwrap();
        let args = vec!["key".to_string(), "0".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lindex() {
        let command_info = COMMANDS.get("lindex").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_lindex() {
        let command_info = COMMANDS.get("lindex").unwrap();
        let args = vec!["key".to_string(), "not_an_integer".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lpop() {
        let command_info = COMMANDS.get("lpop").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lpop() {
        let command_info = COMMANDS.get("lpop").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_rpop() {
        let command_info = COMMANDS.get("rpop").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_rpop() {
        let command_info = COMMANDS.get("rpop").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lpush() {
        let command_info = COMMANDS.get("lpush").unwrap();
        let args = vec!["key".to_string(), "value".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lpush() {
        let command_info = COMMANDS.get("lpush").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_rpush() {
        let command_info = COMMANDS.get("rpush").unwrap();
        let args = vec!["key".to_string(), "value".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_rpush() {
        let command_info = COMMANDS.get("rpush").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lset() {
        let command_info = COMMANDS.get("lset").unwrap();
        let args = vec!["key".to_string(), "0".to_string(), "value".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lset() {
        let command_info = COMMANDS.get("lset").unwrap();
        let args = vec!["key".to_string(), "0".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_lset() {
        let command_info = COMMANDS.get("lset").unwrap();
        let args = vec![
            "key".to_string(),
            "not_an_integer".to_string(),
            "value".to_string(),
        ];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hget() {
        let command_info = COMMANDS.get("hget").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hget() {
        let command_info = COMMANDS.get("hget").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hexists() {
        let command_info = COMMANDS.get("hexists").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hexists() {
        let command_info = COMMANDS.get("hexists").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hdel() {
        let command_info = COMMANDS.get("hdel").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hdel() {
        let command_info = COMMANDS.get("hdel").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hgetall() {
        let command_info = COMMANDS.get("hgetall").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hgetall() {
        let command_info = COMMANDS.get("hgetall").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hkeys() {
        let command_info = COMMANDS.get("hkeys").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hkeys() {
        let command_info = COMMANDS.get("hkeys").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hlen() {
        let command_info = COMMANDS.get("hlen").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hlen() {
        let command_info = COMMANDS.get("hlen").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hmset() {
        let command_info = COMMANDS.get("hmset").unwrap();
        let args = vec!["key".to_string(), "field".to_string(), "value".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hmset() {
        let command_info = COMMANDS.get("hmset").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hset() {
        let command_info = COMMANDS.get("hset").unwrap();
        let args = vec!["key".to_string(), "field".to_string(), "value".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hset() {
        let command_info = COMMANDS.get("hset").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hvals() {
        let command_info = COMMANDS.get("hvals").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hvals() {
        let command_info = COMMANDS.get("hvals").unwrap();
        let args = vec![];
        assert!(command_info.validate_args(args).is_err());
    }
}
