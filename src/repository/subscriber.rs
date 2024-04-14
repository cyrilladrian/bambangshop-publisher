use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Singleton of Database
lazy_static! {
    static ref SUBSCRIBERS: DashMap<usize, Subscriber> = DashMap::new();
}

pub struct SubscriberRepository;
impl SubscriberRepository{
}