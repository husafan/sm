extern crate sm;

sm!{
    Lock {
        InitialStates { Locked, Unlocked }

        TurnKey {
            Locked => Unlocked
        }
    }
}

fn main() {
    use Lock::*;
    let sm = Machine::new(Unlocked);

    sm.transition(TurnKey);
    //~^ ERROR no method named `transition` found for type `Lock::Machine<Lock::Unlocked, sm::NoneEvent>` in the current scope
}
