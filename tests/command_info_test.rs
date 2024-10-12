#[cfg(test)]
mod tests {
    use my_redis_client::command_info::COMMANDS;

    #[test]
    fn test_validate_args_valid_set() {
        let command_info = COMMANDS.get("SET").unwrap();
        let args = vec!["key".to_string(), "value".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_set() {
        let command_info = COMMANDS.get("SET").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_unknown_command() {
        let command_info = COMMANDS.get("unknown");
        assert!(command_info.is_none(), "Expected command_info to be None");
    }
    #[test]
    fn test_validate_args_valid_ping_with_argument() {
        let command_info = COMMANDS.get("PING").unwrap();
        let args = vec!["hello".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_ping_without_argument() {
        let command_info = COMMANDS.get("PING").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_ping() {
        let command_info = COMMANDS.get("PING").unwrap();
        let args = vec!["123".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_flushall() {
        let command_info = COMMANDS.get("FLUSHALL").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_flushall() {
        let command_info = COMMANDS.get("FLUSHALL").unwrap();
        let args = vec!["extra_arg".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_info() {
        let command_info = COMMANDS.get("INFO").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_info() {
        let command_info = COMMANDS.get("INFO").unwrap();
        let args = vec!["extra_arg".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_quit() {
        let command_info = COMMANDS.get("quit").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_quit() {
        let command_info = COMMANDS.get("quit").unwrap();
        let args = vec!["extra_arg".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_echo() {
        let command_info = COMMANDS.get("ECHO").unwrap();
        let args = vec!["hello".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_echo() {
        let command_info = COMMANDS.get("ECHO").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_get() {
        let command_info = COMMANDS.get("GET").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_get() {
        let command_info = COMMANDS.get("GET").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_key() {
        let command_info = COMMANDS.get("KEYS").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_keys() {
        let command_info = COMMANDS.get("KEYS").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_type() {
        let command_info = COMMANDS.get("TYPE").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_type() {
        let command_info = COMMANDS.get("TYPE").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_del() {
        let command_info = COMMANDS.get("DEL").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_del() {
        let command_info = COMMANDS.get("DEL").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_unlink() {
        let command_info = COMMANDS.get("UNLINK").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_unlink() {
        let command_info = COMMANDS.get("UNLINK").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_expire() {
        let command_info = COMMANDS.get("EXPIRE").unwrap();
        let args = vec!["key".to_string(), "10".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_expire() {
        let command_info = COMMANDS.get("EXPIRE").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_expire() {
        let command_info = COMMANDS.get("EXPIRE").unwrap();
        let args = vec!["key".to_string(), "not_an_integer".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_rename() {
        let command_info = COMMANDS.get("RENAME").unwrap();
        let args = vec!["old_key".to_string(), "new_key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_rename() {
        let command_info = COMMANDS.get("RENAME").unwrap();
        let args = vec!["old_key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_llen() {
        let command_info = COMMANDS.get("LLEN").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_llen() {
        let command_info = COMMANDS.get("LLEN").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lrem() {
        let command_info = COMMANDS.get("LREM").unwrap();
        let args = vec!["key".to_string(), "0".to_string(), "value".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lrem() {
        let command_info = COMMANDS.get("LREM").unwrap();
        let args = vec!["key".to_string(), "0".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_lrem() {
        let command_info = COMMANDS.get("LREM").unwrap();
        let args = vec![
            "key".to_string(),
            "not_an_integer".to_string(),
            "value".to_string(),
        ];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lindex() {
        let command_info = COMMANDS.get("LINDEX").unwrap();
        let args = vec!["key".to_string(), "0".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lindex() {
        let command_info = COMMANDS.get("LINDEX").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_lindex() {
        let command_info = COMMANDS.get("LINDEX").unwrap();
        let args = vec!["key".to_string(), "not_an_integer".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lpop() {
        let command_info = COMMANDS.get("LPOP").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lpop() {
        let command_info = COMMANDS.get("LPOP").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_rpop() {
        let command_info = COMMANDS.get("RPOP").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_rpop() {
        let command_info = COMMANDS.get("RPOP").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lpush() {
        let command_info = COMMANDS.get("LPUSH").unwrap();
        let args = vec!["key".to_string(), "value".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lpush() {
        let command_info = COMMANDS.get("LPUSH").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_rpush() {
        let command_info = COMMANDS.get("RPUSH").unwrap();
        let args = vec!["key".to_string(), "value".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_rpush() {
        let command_info = COMMANDS.get("RPUSH").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_lset() {
        let command_info = COMMANDS.get("LSET").unwrap();
        let args = vec!["key".to_string(), "0".to_string(), "value".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_lset() {
        let command_info = COMMANDS.get("LSET").unwrap();
        let args = vec!["key".to_string(), "0".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_invalid_arg_type_lset() {
        let command_info = COMMANDS.get("LSET").unwrap();
        let args = vec![
            "key".to_string(),
            "not_an_integer".to_string(),
            "value".to_string(),
        ];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hget() {
        let command_info = COMMANDS.get("HGET").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hget() {
        let command_info = COMMANDS.get("HGET").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hexists() {
        let command_info = COMMANDS.get("HEXISTS").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hexists() {
        let command_info = COMMANDS.get("HEXISTS").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hdel() {
        let command_info = COMMANDS.get("HDEL").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hdel() {
        let command_info = COMMANDS.get("HDEL").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hgetall() {
        let command_info = COMMANDS.get("HGETALL").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hgetall() {
        let command_info = COMMANDS.get("HGETALL").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hkeys() {
        let command_info = COMMANDS.get("HKEYS").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hkeys() {
        let command_info = COMMANDS.get("HKEYS").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hlen() {
        let command_info = COMMANDS.get("HLEN").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hlen() {
        let command_info = COMMANDS.get("HLEN").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hmset() {
        let command_info = COMMANDS.get("HMSET").unwrap();
        let args = vec!["key".to_string(), "field".to_string(), "value".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hmset() {
        let command_info = COMMANDS.get("HMSET").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hset() {
        let command_info = COMMANDS.get("HSET").unwrap();
        let args = vec!["key".to_string(), "field".to_string(), "value".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hset() {
        let command_info = COMMANDS.get("HSET").unwrap();
        let args = vec!["key".to_string(), "field".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }

    #[test]
    fn test_validate_args_valid_hvals() {
        let command_info = COMMANDS.get("HVALS").unwrap();
        let args = vec!["key".to_string()];
        assert!(command_info.validate_and_transform_args(args).is_ok());
    }

    #[test]
    fn test_validate_args_invalid_num_args_hvals() {
        let command_info = COMMANDS.get("HVALS").unwrap();
        let args = vec![];
        assert!(command_info.validate_and_transform_args(args).is_err());
    }
}
