use std::{collections::VecDeque, sync::mpsc::Sender};

pub trait Pipe<T>
{
    fn send(&mut self, x: T);
}

impl<T> Pipe<T> for VecDeque<T>
{
    fn send(&mut self, x: T)
    {
        self.push_back(x);
    }
}

impl<T> Pipe<T> for Sender<T>
{
    fn send(&mut self, x: T)
    {
        let _ = Sender::send(&self, x);
    }
}

impl<T, F> Pipe<T> for F where F: FnMut(T) -> ()
{
    fn send(&mut self, x: T)
    {
        self(x)
    }
}
