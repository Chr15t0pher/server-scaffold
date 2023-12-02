pub use crate::domain::SubscriberEmail;
pub use crate::domain::SubscriberName;

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
