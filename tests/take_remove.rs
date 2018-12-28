#[cfg(test)]
mod tests {
    use kingslayer::get_world;

    #[test]
    fn cli_take_remove() {
        let cli = get_world("data/test_world.json");

        cli.ask("remove leaf");
        assert_eq!(cli.ask("i"), "You are empty-handed.");
        assert!(
            cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
                && !cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
        );

        cli.ask("take iron sword");
        assert!(
            cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
        );

        cli.ask("take leaf");
        assert!(
            cli.ask("i").contains("iron sword")
                && cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("iron sword")
                && !cli.ask("l").contains("leaf")
        );

        cli.ask("drop iron sword");
        assert!(
            !cli.ask("i").contains("iron sword")
                && !cli.ask("l").contains("leaf")
                && cli.ask("i").contains("leaf")
                && cli.ask("l").contains("iron sword")
        );

        cli.ask("drop leaf");
        assert_eq!(cli.ask("i"), "You are empty-handed.");
        assert!(
            cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
                && !cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
        );
    }

    #[test]
    fn cli_take_all() {
        let cli = get_world("data/test_world.json");

        cli.ask("drop leaf");
        assert_eq!(cli.ask("i"), "You are empty-handed.");
        cli.ask("take all");
        assert!(
            cli.ask("i").contains("iron sword")
                && !cli.ask("l").contains("iron sword")
                && cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("leaf")
        );
    }
}
