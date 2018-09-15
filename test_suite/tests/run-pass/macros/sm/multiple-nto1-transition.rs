extern crate sm_macro;
use sm_macro::sm;

sm! {
    TurnStile {
        States { Locked, Unlocked }

        Coin {
            Locked, Unlocked => Unlocked
        }

        Push {
            Locked,
            Unlocked => Locked
        }
    }
}

fn main() {}
