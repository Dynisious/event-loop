//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-07-28

use super::*;
use alloc::{
  vec::Vec,
  collections::BinaryHeap,
};

/// Types which execute a queue of events.
pub trait EventLoop: Sized {
  /// The event type of the `EventLoop`.
  /// 
  /// Ording is a the maximum ordered event first.
  type Event: super::Event + Ord;
  /// The context carried across each event execution.
  type Context;
  /// The final output once all events have been executed.
  type Output;

  /// Generates a collection of initial events to populate the event queue in an
  /// `execute` call.
  /// 
  /// # Params
  /// 
  /// context --- The context of the `EventLoop`.  
  fn setup(&mut self, context: &mut Self::Context,) -> Vec<Self::Event>;
  /// Converts the `EventLoop`s context into an `Event`s context for execution.
  /// 
  /// Called before event event is executed.
  /// 
  /// # Params
  /// 
  /// context --- The context of the `EventLoop`.  
  fn event_context(&mut self, context: &mut Self::Context,) -> <Self::Event as Event>::Context;
  /// Converts an `Event`s output into more events to be queued for execution.
  /// 
  /// Called after every event is executed.
  /// 
  /// # Params
  /// 
  /// context --- The context of the `EventLoop`.  
  /// output --- The event output to be converted.  
  fn finalise_event(&mut self, context: &mut Self::Context, output: <Self::Event as Event>::Output,) -> Vec<Self::Event>;
  /// Converts the `EventLoop`s context into an `EventLoop`s output.
  /// 
  /// Called once all events have been executed.
  /// 
  /// # Params
  /// 
  /// context --- The context of the `EventLoop`.  
  fn finalise(self, context: Self::Context,) -> Self::Output;
  /// The driver of an `EventLoop`.
  /// 
  /// 1. Calls `setup` to populate the event queue.  
  /// 2. Calls `event_context` before executing an `Event` from the queue.  
  /// 3. Executes an `Event` from the queue.  
  /// 4. Calls `finalise_event` after executing an `Event` from the queue.  
  /// 5. Calls `finalise` after all `Event`s have been executed.  
  fn execute(mut self, mut context: Self::Context,) -> Self::Output {
    use core::iter::FromIterator;

    //Populate the event queue.
    let mut event_queue = BinaryHeap::from_iter(self.setup(&mut context,),);

    //Execute all events.
    while let Some(event) = event_queue.pop() {
      //The output of executing an event.
      let output = event.execute(self.event_context(&mut context,),);

      //Queue any additional events.
      for event in self.finalise_event(&mut context, output,) {
        event_queue.push(event,);
      }
    }

    //Finalise the event loop.
    self.finalise(context,)
  }
}
