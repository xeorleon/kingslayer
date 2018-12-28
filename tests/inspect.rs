#[cfg(test)]
mod tests {
    use kingslayer::get_world;

    #[test]
    fn cli_inspect() {
        let cli = get_world("data/test_world.json");

        assert!(cli.ask("i").contains("leaf") && !cli.ask("l").contains("leaf"));
        assert_eq!(cli.ask("inspect leaf"), "It's small, brown, and dry.");
        cli.ask("drop leaf");
        assert!(cli.ask("l").contains("leaf") && !cli.ask("i").contains("leaf"));
        assert_eq!(cli.ask("inspect leaf"), "It's small, brown, and dry.");
    }
}
