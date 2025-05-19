# at://retrospective

Experiment to have a retrospective every day that feels like a status update and gives you a set of records you can run
weekly, monthly, yearly, etc to see what you been up to.


1. Clone this repo
2. Copy and paste [.env.template](.env.template) and re name [.env](.env)
3. Fill out info. Handle is your atproto handle. App password is a [generated app password](https://blueskyfeeds.com/en/faq-app-password). Lexicon collection base is how it shows up in your atproto repo
4. run `cargo run --bin retrospective_cli -- new-day` to generate a empty example lexicon
5. Write till your hearts content
6. run `cargo run --bin retrospective_cli -- ketchup` to catch up your PDS with what's been happening
