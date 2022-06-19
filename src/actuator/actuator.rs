
pub trait Actuator{
    fn notify_query_status(query :String, status: bool);
}