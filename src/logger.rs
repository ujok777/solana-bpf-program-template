use solana_program::msg;

/// Utility logger module for Solana BPF programs.
/// Prints a short message during runtime for easier debugging.
pub fn log_init_message() {
    msg!("Program initialized successfully 🛰️");
}
