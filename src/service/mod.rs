use crate::*;

mod command_service;

/// abstract layer for Command
pub trait CommandService {
    /// handle the Command, and return Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
