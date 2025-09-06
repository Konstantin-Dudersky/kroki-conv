pub enum InternalMessage<T> {
    Value(T),
    EndExecution,
}
