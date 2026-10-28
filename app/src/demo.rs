//! Demo profiles for `--demo NAME`: open the app already scored, e.g. to take
//! the screenshots in the README and the blog post.

use crate::model::{
    Build, Education, Gender, Goal, IncomeBand, Looks, Profile, RedFlags, Region, Size, Smarts,
};

/// Every demo, by the name given after `--demo`.
pub const DEMOS: [(&str, Profile); 4] = [
    (
        "top",
        Profile {
            age: 31,
            gender: Gender::Man,
            education: Education::Master,
            income: IncomeBand::High,
            height_cm: Some(190),
            looks: Looks::AboveAverage,
            build: Build::GymRegular,
            smarts: Smarts::Smart,
            size: Size::PreferNotToSay,
            red_flags: RedFlags::None,
            region: Region::TurkeyMiddleEast,
            goal: Goal::LongTerm,
        },
    ),
    (
        "niche",
        Profile {
            age: 40,
            gender: Gender::Woman,
            education: Education::Master,
            income: IncomeBand::Middle,
            height_cm: None,
            looks: Looks::Average,
            build: Build::Average,
            smarts: Smarts::VerySmart,
            size: Size::PreferNotToSay,
            red_flags: RedFlags::None,
            region: Region::SouthernEurope,
            goal: Goal::Casual,
        },
    ),
    (
        "its-over",
        Profile {
            age: 64,
            gender: Gender::Man,
            education: Education::HighSchool,
            income: IncomeBand::Low,
            height_cm: Some(158),
            looks: Looks::BelowAverage,
            build: Build::CouchPotato,
            smarts: Smarts::Average,
            size: Size::BelowAverage,
            red_flags: RedFlags::None,
            region: Region::EastAsia,
            goal: Goal::Undecided,
        },
    ),
    (
        "single-mother-arc",
        Profile {
            age: 58,
            gender: Gender::Woman,
            education: Education::Doctorate,
            income: IncomeBand::Low,
            height_cm: Some(185),
            looks: Looks::BelowAverage,
            build: Build::CouchPotato,
            smarts: Smarts::GalaxyBrain,
            size: Size::PreferNotToSay,
            red_flags: RedFlags::None,
            region: Region::EastAsia,
            goal: Goal::Undecided,
        },
    ),
];

/// The demo profile called `name`, if there is one.
pub fn find(name: &str) -> Option<Profile> {
    DEMOS.iter().find(|(n, _)| *n == name).map(|(_, p)| *p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Tier, evaluate};

    /// Each demo must land in the tier its name promises, so the screenshots
    /// stay true when the weights change.
    #[test]
    fn demos_land_in_their_tiers() {
        let tier = |name| evaluate(&find(name).expect("demo exists")).map(|v| v.tier);
        assert_eq!(tier("top"), Ok(Tier::TopOfTheMarket));
        assert_eq!(tier("niche"), Ok(Tier::NicheButLoved));
        assert_eq!(tier("its-over"), Ok(Tier::LifeIsOver));
        assert_eq!(tier("single-mother-arc"), Ok(Tier::LifeIsOver));
    }

    #[test]
    fn unknown_demo_is_none() {
        assert!(find("nope").is_none());
    }
}
