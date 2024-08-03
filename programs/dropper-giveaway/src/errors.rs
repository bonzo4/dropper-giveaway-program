use anchor_lang::error_code;

#[error_code]
pub enum DropperError {
    Error,
    #[msg("You are not a chosen winner")]
    NotAWinner,
    #[msg("There are no more giveaways")]
    NoPrizesLeft,
}
