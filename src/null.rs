#![allow(unused_variables)]

use std::sync::mpsc::Sender;
use std::path::Path;
use super::{Event, Result, Watcher};

pub struct NullWatcher;

impl Watcher for NullWatcher {
    fn new(tx: Sender<Event>) -> Result<NullWatcher> {
        Ok(NullWatcher)
    }

    fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        Ok(())
    }

    fn unwatch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        Ok(())
    }
}
