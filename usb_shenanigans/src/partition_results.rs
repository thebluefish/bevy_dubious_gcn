/// Like `partition` but splits Results
pub trait PartitionResults<T, E> {
    fn partition_results(self) -> (Vec<T>, Vec<E>);
}

impl<I, T, E> PartitionResults<T, E> for I
    where
        I: IntoIterator<Item = Result<T, E>> + Iterator,
{
    fn partition_results(self) -> (Vec<T>, Vec<E>) {
        let mut ts = Vec::new();
        let mut es = Vec::with_capacity(self.size_hint().1.unwrap_or(self.size_hint().0));

        for item in self.into_iter() {
            match item {
                Ok(t) => ts.push(t),
                Err(e) => es.push(e),
            }
        }

        (ts, es)
    }
}