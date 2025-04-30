use console::{Emoji, style};

pub(crate) fn write_welcome_message() {
    let intro_logo = r#"
        __    __    ___  ____       ___    __ __  ___ ___  ____       ____    _____
        |  |__|  |  /  _]|    \     |   \  |  |  ||   |   ||    \     |    \  / ___/
        |  |  |  | /  [_ |  o  )    |    \ |  |  || _   _ ||  o  )    |  D  )(   \_
        |  |  |  ||    _]|     |    |  D  ||  |  ||  \_/  ||   _/     |    /  \__  |
        |  `  '  ||   [_ |  O  |    |     ||  :  ||   |   ||  |       |    \  /  \ |
        \      / |     ||     |    |     ||     ||   |   ||  |       |  .  \ \    |
        \_/\_/  |_____||_____|    |_____| \__,_||___|___||__|       |__|\_|  \___|
        
        "#;
    log::info!(
        "{}\n\t\t{}{}\n\n",
        style(intro_logo).green(),
        style("Trans rights are human rights!").magenta().bright(),
        Emoji("⚧️ 💜", "")
    );
}
