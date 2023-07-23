use crate::cli::CLI;

impl CLI {
    pub(in crate::cli) fn print_welcome() {
        let text = "Welcome to RustyCleaner!

RustyCleaner is a powerful tool that will help you remove old Discord messages effortlessly.
Are you tired of reading your cringe messages from the past? Then RustyCleaner is just what you need!

Features:
1. user-friendly login: simply log in with your Discord User Access Token and get access to your servers.
2. thorough cleaning: RustyCleaner scans the discord-package folder and removes all the old messages you specified.

How to use RustyCleaner:
1. get your Discord user access token.
2. start RustyCleaner and follow the simple login instructions.
3. sit back and relax while RustyCleaner does the rest!

Please note that RustyCleaner only deletes messages that it has access to. Your privacy and security come first.";
        println!("{}", text);
        println!("Discord doesn't allow self bots. This is kinda a bot. \
        There is a risk of being banned from Discord!");

        let mut confirm = String::new();

        while !confirm.eq("confirm") {
            print!("Pls type confirm to enter:");
            confirm = Self::ask_for_input();
            print!("\n");
        }
    }
}