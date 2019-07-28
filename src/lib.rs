//! Provides higher order functions defining an update loop and an event loop.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-07-28

#![deny(missing_docs,)]
#![no_std]

extern crate alloc;
#[cfg(any(test, doc,),)]
extern crate std;

mod events;
mod event_loop;

pub use self::{events::*, event_loop::*,};

#[cfg(test,)]
mod tests {
  use super::*;
  use std::{
    vec::Vec,
    cmp::{PartialOrd, Ord, Ordering,},
  };

  #[derive(PartialEq, Eq, Clone, Copy,)]
  enum MathEnt {
    Div(i32,),
    Mul(i32,),
    Add(i32,),
    Sub(i32,),
  }

  impl PartialOrd for MathEnt {
    #[inline]
    fn partial_cmp(&self, rhs: &Self,) -> Option<Ordering> { Some(self.cmp(rhs,)) }
  }

  impl Ord for MathEnt {
    #[inline]
    fn cmp(&self, rhs: &Self,) -> Ordering {
      use MathEnt::*;

      match self {
        Div(_) => match rhs {
          Div(_) => Ordering::Equal,
          _ => Ordering::Greater,
        },
        Mul(_) => match rhs {
          Div(_) => Ordering::Less,
          Mul(_) => Ordering::Equal,
          _ => Ordering::Greater,
        },
        Add(_) => match rhs {
          Sub(_) => Ordering::Greater,
          Add(_) => Ordering::Equal,
          _ => Ordering::Less,
        },
        Sub(_) => match rhs {
          Sub(_) => Ordering::Equal,
          _ => Ordering::Less,
        },
      }
    }
  }

  impl Event for MathEnt {
    type Output = i32;
    type Context = i32;

    fn execute(self, context: Self::Context,) -> Self::Output {
      use MathEnt::*;

      match self {
        Add(f,) => context + f,
        Sub(f,) => context - f,
        Mul(f,) => context * f,
        Div(f,) => context / f,
      }
    }
  }

  struct MathLoop(Vec<MathEnt>,);

  impl EventLoop for MathLoop {
    type Event = MathEnt;
    type Context = i32;
    type Output = i32;

    fn setup(&mut self, _: &mut Self::Context,) -> Vec<Self::Event> { self.0.clone() }
    fn event_context(&mut self, context: &mut Self::Context,) -> <Self::Event as Event>::Context { *context }
    fn finalise_event(&mut self, context: &mut Self::Context, output: <Self::Event as Event>::Output,) -> Vec<Self::Event> {
      use std::dbg;

      *context = dbg!(output,); Vec::new()
    }
    fn finalise(self, context: Self::Context,) -> Self::Output { context }
  }

  #[test]
  fn test_event_loop() {
    use MathEnt::*;
    use std::vec;

    let math = MathLoop(vec![Add(2,), Sub(3,), Mul(6,), Div(2,),],);

    assert_eq!(math.execute(8,), 23,);
  }
}
