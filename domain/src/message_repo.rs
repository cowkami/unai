use crate::message::Message;
use mockall::automock;
use std::future::Future;

#[automock]
pub trait MessageRepo {
    fn save(&self, messages: Vec<Message>) -> impl Future<Output = Result<(), &'static str>>;
    fn list_by_user_id(
        &self,
        user_id: String,
        limit: u32,
        order_direction: OrderDirection,
    ) -> impl Future<Output = Result<Vec<Message>, &'static str>>;
}

pub enum OrderDirection {
    Ascending,
    Descending,
}

pub trait ProvideMessageRepo {
    type Repo: MessageRepo;

    fn provide(&self) -> &Self::Repo;
}
