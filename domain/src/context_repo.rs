use crate::context::Context;
use crate::message::Message;
use mockall::automock;
use std::future::Future;

#[automock]
pub trait ContextRepo {
    fn detect_context_change(
        &self,
        previous_messages: &[Message],
        new_message: &Message,
    ) -> impl Future<Output = Result<bool, &'static str>>;

    fn generate_by_message(
        &self,
        message: &Message,
    ) -> impl Future<Output = Result<Context, &'static str>>;
}

pub trait ContextRepoProvider {
    type Repo: ContextRepo;

    fn provide(&self) -> &Self::Repo;
}
