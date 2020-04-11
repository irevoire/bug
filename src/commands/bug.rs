use serenity::framework::standard::{macros::command, Args, CommandResult};

use serenity::{model::channel::Message, prelude::Context};

#[command]
pub fn bug(_ctx: &mut Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}
